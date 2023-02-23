import { exit } from "process";
import { genPrivateKey } from "../lib/src/enc";

export async function gen() {
    let privateKey = genPrivateKey();
    console.log(`Encryption Public key: 0x${privateKey.publicKey.toHex()}`);
    console.log(`Private key: ${privateKey.toHex()}`);
}

gen()
    .catch(err => {
        console.error(err);
        exit(-1);
    });