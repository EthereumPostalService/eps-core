![logo](imgs/EPS-opt-1-dark@1920x.png)

# Ethereum Postal Service

Send physical letters with an Ethereum smart contract call. An interface can be found at [EthereumPostalService.com](https://ethereumpostalservice.com/) or the contract can be called directly on Ethereum and Optimism.

Send letters to your friends. Send letters to your foes. Send letters as an autonomous and immutable agent to inform the world of your existence.

Is it trustless? No.

Is it atomic? No.

Is it e2e encrypted? No.

Why would you do this? Maybe the world is too focused on getting data on-chain, when really we should focus on getting data off-chain.

# Usage

Call the contract directly

```solidity
EthereumPostalService eps = EthereumPostalService(0x2156fcCff55637317D211B62318007309378fB95);
uint256 postageCost = eps.getPostageWei();

EthereumPostalService.PostalAddress memory addr =
    EthereumPostalService.PostalAddress(
        "2 Lincoln Memorial Cir NW", // Address Line 1
        "",                          // Address Line 2
        "Washington DC",             // City
        "US",                        // State
        "20002",                     // Zip
        "Abe Lincoln");              // Recipient
eps.sendMail{value: postageCost}(addr, "Thanks for your work out there.");
```

Alternatively use the `sendsMail` modifier from [`contracts/src/MailSender.sol`](contracts/src/MailSender.sol) as in [`contracts/src/test/MailingNFT.sol`](contracts/src/MailSender.sol).

The EthereumPostalService contract also exposes a `sendEncryptedMail()` function which allows the caller to toggle encryption for both the address and the underlying message. The address / message fields should be ECIES encrypted to the public key stored on the `EthereumPostalService.encryptionPubKey` field. A library to assist with encryption in javascript can be found in [`lib/src/enc.ts`](lib/src/enc.ts).

# Addresses

| Chain    | `EthereumPostalService.sol `                 | `ChainlinkPostagePriceModule`                | Chainlink ETH / USD                          |
| -------- | -------------------------------------------- | -------------------------------------------- | -------------------------------------------- |
| Mainnet  | `0x2156fcCff55637317D211B62318007309378fB95` | `0x526153e7996b5CcD6971E41b38E75C924b3f7044` | `0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419` |
| Optimism | `0x2156fcCff55637317D211B62318007309378fB95` | `0x526153e7996b5CcD6971E41b38E75C924b3f7044` | `0x13e3Ee699D1909E989722E753853AE30b17e08c5` |

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
