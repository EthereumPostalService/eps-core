import 'mocha';
import { expect } from 'chai';
import { ChildProcess, exec, spawn } from 'child_process';
import express from "express";
import { encrypt, encryptAddress, genPrivateKey } from '../lib/src/enc';
import { Wallet, JsonRpcProvider, Contract, WebSocketProvider } from "ethers";
import { abi as CONTRACT_ABI } from "../contracts/out/EthereumPostalService.sol/EthereumPostalService.json";
import bodyParser from 'body-parser';

describe('Integration Test', async () => {
    let liveProcesses: ChildProcess[] = [];

    let rpc_url = "http://127.0.0.1:8545";
    let ws_url = "ws://127.0.0.1:8545";
    let contract_addr = "0x9fE46736679d2D9a65F0992F2272dE9f3c7fa6e0";

    let api_port = 3000;
    let api_key = "APIKEYAPIKEY";

    it('plain', async () => {
        // Spin up anvil
        let anvilProcess = runCmdAsync("anvil");
        liveProcesses.push(anvilProcess);
        await delay(2_000);

        // Mock mail API
        let fakeApi = express();
        let fakeApiUrl = `http://localhost:${api_port}`;
        let server = fakeApi.listen(api_port, () => { });

        let contactPostReqs: any[] = [];
        let contactGetReqs: any[] = [];
        let letterPostReqs: any[] = [];
        fakeApi.post("/print-mail/v1/contacts*", (req, res) => {
            contactPostReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.get("/print-mail/v1/contacts*", (req, res) => {
            contactGetReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.post("/print-mail/v1/letters", (req, res) => {
            letterPostReqs.push(req.body);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeLetter());
        })

        // Spin up backend
        let pk = genPrivateKey();
        await runCmd('cd ../backend && cargo build')
        let backendProcess = runCmdAsync(
            `export RPC_ENDPOINT="${ws_url}" \
                && export CONTRACT="${contract_addr}" \
                && export MAIL_API_KEY="${api_key}" \
                && export MAIL_API_URL="${fakeApiUrl}" \
                && export DEFAULT_SENDER="12" \
                && export PK="${pk.toHex().substring(2)}" \
                && cd ../backend \
                && ./target/debug/mail`);
        await delay(1_000);

        // Deploy some contracts
        await runCmd(`cd ../contracts && forge script script/DeployLocal.s.sol:DeployLocal --rpc-url ${rpc_url} --broadcast`);
        await runCmd(`cd ../contracts && forge script script/CallLocal.s.sol:CallLocal --rpc-url ${rpc_url} --broadcast`);

        // Wait for indexer to attempt
        await delay(2_000);
        backendProcess.kill();
        server.close();
        await delay(1_000);

        expect(contactPostReqs.length).to.be.eq(1);
        expect(contactGetReqs.length).to.be.eq(1);
        expect(letterPostReqs.length).to.be.eq(1);
    });

    it('encrypted', async () => {
        // Spin up anvil
        let anvilProcess = runCmdAsync("anvil");
        liveProcesses.push(anvilProcess);
        await delay(1_000);

        let pk = genPrivateKey();
        let msg = "Hello from EPS";
        let msgEncrypted = encrypt(msg, pk.publicKey.toHex());
        let address = {
            addressLine1: "1 School Rd",
            addressLine2: "",
            city: "Princeton",
            countryCode: "US",
            postalOrZip: "08540",
            name: "John Nash"
        };
        let addressEncrypted = encryptAddress(address, pk.publicKey.toHex());

        // Mock mail API
        let fakeApi = express();
        let fakeApiUrl = `http://localhost:${api_port}`;
        let server = fakeApi.listen(api_port, () => { });
        fakeApi.use(bodyParser.urlencoded({ extended: false }))
        fakeApi.use(bodyParser.json())

        let contactPostReqs: any[] = [];
        let contactGetReqs: any[] = [];
        let letterPostReqs: any[] = [];
        fakeApi.post("/print-mail/v1/contacts*", (req, res) => {
            expect(req.body.addressLine1).to.be.equal(address.addressLine1);
            expect(req.body.addressLine2).to.be.equal(address.addressLine2);
            expect(req.body.countryCode).to.be.equal(address.countryCode);
            expect(req.body.postalOrZip).to.be.equal(address.postalOrZip);
            expect(req.body.firstName).to.be.equal(address.name);

            contactPostReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.get("/print-mail/v1/contacts*", (req, res) => {
            contactGetReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.post("/print-mail/v1/letters", (req, res) => {
            expect(req.body.html).to.be.equal(msg);

            letterPostReqs.push(req.body);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeLetter());
        })

        // Spin up backend
        await runCmd('cd ../backend && cargo build');
        let backendProcess = runCmdAsync(
            `export RPC_ENDPOINT="${ws_url}" \
                && export CONTRACT="${contract_addr}" \
                && export MAIL_API_KEY="${api_key}" \
                && export MAIL_API_URL="${fakeApiUrl}" \
                && export DEFAULT_SENDER="12" \
                && export PK="${pk.toHex().substring(2)}" \
                && cd ../backend \
                && ./target/debug/mail`);
        await delay(1_000);

        // Deploy contracts
        await runCmd(`cd ../contracts && forge script script/DeployLocal.s.sol:DeployLocal --rpc-url ${rpc_url} --broadcast`);
        await delay(2_000);

        // Call contract
        let provider = new JsonRpcProvider(rpc_url);
        let wallet = new Wallet("0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80", provider); // 0th Anvil default PK
        let contract = new Contract(contract_addr, CONTRACT_ABI, wallet);
        let postage = await contract.getPostageWei();
        await contract.sendEncryptedMail(addressEncrypted, msgEncrypted, true, true, { value: postage });

        // Wait for indexer to attempt
        await delay(2_000);
        backendProcess.kill();
        server.close();
        await delay(1_000);

        expect(contactPostReqs.length).to.be.eq(1);
        expect(contactGetReqs.length).to.be.eq(1);
        expect(letterPostReqs.length).to.be.eq(1);
    });

    it('client started post deployment', async () => {
        // Spin up anvil
        let anvilProcess = runCmdAsync("anvil");
        liveProcesses.push(anvilProcess);
        await delay(1_000);

        // Mock mail API
        let fakeApi = express();
        let fakeApiUrl = `http://localhost:${api_port}`;
        let server = fakeApi.listen(api_port, () => { });

        let contactPostReqs: any[] = [];
        let contactGetReqs: any[] = [];
        let letterPostReqs: any[] = [];
        fakeApi.post("/print-mail/v1/contacts*", (req, res) => {
            contactPostReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.get("/print-mail/v1/contacts*", (req, res) => {
            contactGetReqs.push(req);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeContact());
        });
        fakeApi.post("/print-mail/v1/letters", (req, res) => {
            letterPostReqs.push(req.body);
            expect(req.header("x-api-key")).to.be.equal(api_key);
            res.json(fakeLetter());
        })

        // Deploy some contracts
        await runCmd(`cd ../contracts && forge script script/DeployLocal.s.sol:DeployLocal --rpc-url ${rpc_url} --broadcast`);
        await runCmd(`cd ../contracts && forge script script/CallLocal.s.sol:CallLocal --rpc-url ${rpc_url} --broadcast`);
        await delay(1_000);

        // Spin up backend
        let pk = genPrivateKey();
        await runCmd('cd ../backend && cargo build');
        let backendProcess = runCmdAsync(
            `export RPC_ENDPOINT="${ws_url}" \
                && export CONTRACT="${contract_addr}" \
                && export MAIL_API_KEY="${api_key}" \
                && export MAIL_API_URL="${fakeApiUrl}" \
                && export DEFAULT_SENDER="12" \
                && export PK="${pk.toHex().substring(2)}" \
                && cd ../backend \
                && ./target/debug/mail`);
        await delay(1_000);


        backendProcess.kill();
        server.close();

        expect(contactPostReqs.length).to.be.eq(1);
        expect(contactGetReqs.length).to.be.eq(1);
        expect(letterPostReqs.length).to.be.eq(1);
    });

    // Kill all open processes (notably anvil)
    afterEach(() => {
        while (liveProcesses.length > 0) {
            let process = liveProcesses.pop()!;
            expect(process.kill()).to.be.true;
        }
    })
});

// Runs a CLI command, resolves promise when command exits
function runCmd(cmd: string, debug = false, expectNoErrors = true): Promise<number> {
    let cmds = cmd.split(" ");
    let childProcess = spawn(cmds[0], cmds.slice(1), { shell: true });

    if (debug) {
        childProcess.on('error', (err) => {
            console.error(`runCmd failed: ${err}`);
        });
        childProcess.stdout.on('data', (data) => {
            console.log(`runCmd stdout: ${data}`);
        })
        childProcess.stderr.on('data', (data) => {
            console.warn(`runCmd stderr: ${data}`);
        })
    }

    return new Promise((resolve, reject) => {
        childProcess.on('close', (code) => {
            if (expectNoErrors) {
                expect(code, `Cmd "${cmd}" exited with non-zero exit code. Try running runCmd(debug = true).`).to.be.eq(0);
            }
            resolve(code);
        })
    })
}

// Runs a CLI command, resolves promise immedietly, returning process
function runCmdAsync(cmd: string, debug = false): ChildProcess {
    let cmds = cmd.split(" ");
    let childProcess = spawn(cmds[0], cmds.slice(1), { shell: true });

    if (debug) {
        childProcess.on('error', (err) => {
            console.error(`runCmd failed: ${err}`);
        });
        childProcess.stdout.on('data', (data) => {
            console.log(`runCmd stdout: ${data}`);
        })
        childProcess.stderr.on('data', (data) => {
            console.warn(`runCmd stderr: ${data}`);
        })
    }

    return childProcess;
}

function fakeContact() {
    return {
        addressLine1: "addressLine1",
        countryCode: "US",
        id: "1234",
        firstName: "Bill",
        lastName: "Johnson",
        addressLine2: "",
        city: "city",
        country: "country",
        postalOrZip: "12345",
        provinceState: "ST",
    }
}

function fakeLetter() {
    return {
        from: fakeContact(),
        to: fakeContact(),
        html: "html",
        id: "1337"
    }
}

function delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
}