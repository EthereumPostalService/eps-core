![logo](imgs/EPS-opt-1-dark@1920x.png)

# EthMail

Send physical letters with an on-chain smart contract call.

Is it trustless? No.

Is it atomic? No.

Is it e2e encrypted? No.

Why would you do this? For fun.

# Usage

Call the contract directly

```solidity
EthMail ethMail = EthMail(0xETHMAILADDRESS);
uint256 postageCost = ethMail.getPostageWei();

EthMail.PostalAddress memory addr =
    EthMail.PostalAddress(
        "2 Lincoln Memorial Cir NW", // Address Line 1
        "",                          // Address Line 2
        "Washington DC",             // City
        "US",                        // State
        "20002",                     // Zip
        "Abe Lincoln");              // Recipient
ethMail.sendMail{value: postageCost}(addr, "Thanks for your work out there.");
```

Alternatively use the `sendsMail` modifier from [`contracts/src/MailSender.sol`](contracts/src/MailSender.sol) as in [`contracts/src/test/MailingNFT.sol`](contracts/src/MailSender.sol).

The EthMail contract also exposes a `sendEncryptedMail()` function which allows the caller to toggle encryption for both the address and the underlying message. The address / message fields should be ECIES encrypted to the public key stored on the `EthMail.encryptionPubKey` field. A library to assist with encryption in javascript can be found in [`lib/src/enc.ts`](lib/src/enc.ts).

# Addresses

| Chain    | `EthMail.sol ` | `ChainlinkPostagePriceModule` | Chainlink ETH / USD                          |
| -------- | -------------- | ----------------------------- | -------------------------------------------- |
| Mainnet  | TBD            | TBD                           | `0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419` |
| Optimism | TBD            | TBD                           | `0x13e3Ee699D1909E989722E753853AE30b17e08c5` |

# Cmds

- `yarn install`
- `yarn build`
- `yarn test`
- `yarn integration test`

* Foundry and Anvil must be installed to run unit / integration tests. \*

# Future

- [x] Encrypted version
- [x] Integration tests wait for specific stdout phrases
- [ ] UniV3 PostagePriceModule
- [ ] Distributed sending
