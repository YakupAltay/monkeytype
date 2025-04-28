# Monkeytype

**Monkeytype** is a Web3-powered typing game that attests your typing sessions to decentralized networks like [Celestia](https://celestia.org) and [Avail](https://www.availproject.org).

After each game, your session statistics — including typing speed, accuracy, and performance — are submitted as a blob to the selected Data Availability (DA) layers.

Each session stores:
- `typed` — number of words typed
- `correct` — number of correct words
- `accuracy` — percentage of correct words
- `wpm` — words per minute

---

## How to Play

### 1. Start the Celestia and/or Avail Light Clients

You must run light clients locally to submit your game results.

#### Start Avail Light Client

```bash
docker compose -f ./src/light-node/avail/docker-compose.yaml up -d
```

#### Stop Avail Light Client

```bash
docker compose -f ./src/light-node/avail/docker-compose.yaml down
```

---

#### Start Celestia Light Node

```bash
docker compose -f ./src/light-node/celestia/docker-compose.yaml up -d
```

#### Stop Celestia Light Node

```bash
docker compose -f ./src/light-node/celestia/docker-compose.yaml down
```

You can run either or both, depending on which DA layers you want to use.

---

### 2. Get Your Wallet Addresses

You can get your embedded wallet addresses directly via the binary — no manual Docker commands needed:

```bash
./target/release/monkeytype address --da avail,celestia
```

This will show your Avail and Celestia addresses.

- **Celestia Mocha Faucet:** [https://mocha-4.celenium.io/faucet](https://mocha-4.celenium.io/faucet)
- **Avail Testnet Faucet:** [https://faucet.avail.tools](https://faucet.avail.tools)

Make sure you fund your addresses with testnet tokens before playing.

---

## Build the Game

First, compile the project:

```bash
cargo build --release
```

---

## Play Monkeytype

You can start a typing session and select which DA layers to submit your session to:

```bash
./target/release/monkeytype start --da avail,celestia
```

- **avail** — Submit to Avail
- **celestia** — Submit to Celestia
- You can select one or both layers.

Example:

```bash
./target/release/monkeytype start --da avail
```

---

## Shut Down the Nodes

When you're done:

```bash
docker compose -f ./src/light-node/avail/docker-compose.yaml down
docker compose -f ./src/light-node/celestia/docker-compose.yaml down
```

---

# Project Structure

| Folder/File | Purpose |
|:------------|:--------|
| `src/api.rs` | Fetch random words for typing |
| `src/game.rs` | Core typing game logic |
| `src/celestia.rs` | Celestia submission functions |
| `src/avail.rs` | Avail submission functions |
| `src/constants.rs` | Constants (like RPC URLs) |
| `src/commands/start.rs` | Handle starting typing sessions |
| `src/commands/address.rs` | Handle printing wallet addresses |
| `src/cli.rs` | CLI argument parsing |
| `src/main.rs` | Main application entry point |

---

## Notes

- Ensure your light nodes are running before starting the game.
- Submissions are only attempted after a session ends.
- If no DA layer is selected, the game will not attempt submission.
