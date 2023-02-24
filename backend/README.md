# The reverse oracle/event listener

Fetches on chain events. If certain event is emitted, then we trigger
off-chain action.

---

Clone the repo:

```
git clone git@github.com:ethereumpostalservice/eps-core.git
cd backend
```

Fill out the `.env`

```
cp .env.example .env
```

```bash
# fill these out with relevant data, 
RPC_ENDPOINT_ETH=wss://eth-mainnet.g.alchemy.com/v2/<key>
RPC_ENDPOINT_OP=wss://opt-mainnet.g.alchemy.com/v2/<key>
CONTRACT=0x2156fcCff55637317D211B62318007309378fB95
MAIL_API_URL=https://api.postgrid.com
MAIL_API_KEY=
PK=
DEFAULT_SENDER=contact_gV5mranPJKMiRmyMeY9hdz
TEMPLATE_ID=template_nXrkht6QJTZvhyg6wWH8E7 
```

---

## Running locally

Build or run locally:

```bash
cargo run
# or
cargo build --release
target/release/mail
```

---

## Production

Fortunately, the mail system is slow, so there is no real rush to process these events.
This means we can be super lazy about when we check for new logs and drive the events through
to the postage api. In production, we run a cron job that executes this rust program every hour.

## Future

more chains are easy to support but need to update the `config.rs` file.
