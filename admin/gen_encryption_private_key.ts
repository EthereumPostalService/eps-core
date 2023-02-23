import { exit } from "process";
import { genPrivateKey } from "../lib/src/enc";

export async function gen() {
    let privateKey = genPrivateKey();
    console.log(`Encryption public key: ${privateKey.publicKey.toHex()}\nPrivate key: ${privateKey.toHex()}`);
}

gen()
    .catch(err => {
        console.error(err);
        exit(-1);
    });