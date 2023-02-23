# The reverse oracle/event listener

Listens to some on chain events. If certain event is emitted, then we trigger
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
# fill these out with relevant data
RPC_ENDPOINT=
CONTRACT=
MAIL_API_URL=
MAIL_API_KEY=
DEFAULT_SENDER=
PK=
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

## Running in systemd with dpkg (debian)

```
cargo deb (might need to install this with `cargo install cargo-deb`)
```

```
sudo dpkg -i target/debian/<this name depends on ur system, just tab complete it>
```

start the service:

```
sudo systemctl start mail
```

check status

```
sudo systemctl status mail
```

check logs:

```
sudo journalctl -u mail.service
```
