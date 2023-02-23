import { Wallet } from "ethers";

let PREFIX = "E950";

let num = 0;
let startTime = new Date().getTime();
while (true) {
    let wallet = Wallet.createRandom();

    if (num % 1000 == 0) {
        let elapsed = new Date().getTime() - startTime;
        console.log(`${num / elapsed * 1000} addr / sec`);
    }


    if (wallet.address.substring(2, PREFIX.length + 2) === PREFIX) {
        console.log(`Address: ${wallet.address}`);
        console.log(`mnemonic: ${wallet.mnemonic.phrase}`);
        console.log(`PK: ${wallet.privateKey}`);
    }
    num += 1;
}