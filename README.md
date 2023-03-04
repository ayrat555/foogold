foogold (Fool's Gold)
------------

Fool's gold is a tool for trying your luck with bitcoin mnemonics


### Description

Bitcoin user addresses are generated using 2048 words defined in [BIP-39](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki). If words are selected randomly, it's impossible to figure them out. For example, the number of 12 words mnemonics is  2048¹² (2¹³²). But if a user decided to use a fixed number of words, it may be possible to learn them.

Foogold is a tool to try different mnemonics hoping one of them will generate a used address on the bitcoin chain.

It has the following features:

- index all bitcoin user addresses (segwit, nested segwit and legacy). This data is used to check if an address generated by a mnemonic was used in a bitcoin transaction on the chain.
- check a single mnemonic using different derivation paths
- check combinations of 1 and 2 words
- run foogold forever on your server, checking random mnemonics hoping one of them unlocks something

### Indexing the chain

Setup database by running:

```
diesel database setup
```

For this step, you'll need access to a bitcoin node.

```rust
cargo run --release -- index --node-url "https://mynode.com" --sync-start-block 0 --sync-end-block 700000
```

The number of addresses is more than 1 billion and it may take from a couple of days to a couple of weeks for the whole chain to sync. Ideally, several instances of this command should be run indexing different block ranges

![index](demo/demo2.gif)

##### Docs

```bash
Usage: foogold index [OPTIONS] --database-url <DATABASE_URL> --node-url <NODE_URL> --sync-start-block <SYNC_START_BLOCK> --sync-end-block <SYNC_END_BLOCK>

Options:
      --database-url <DATABASE_URL>
          [env: DATABASE_URL=postgres://postgres:postgres@localhost:5432/foogold]
      --database-pool-size <DATABASE_POOL_SIZE>
          [env: DATABASE_POOL_SIZE=] [default: 10]
      --node-url <NODE_URL>
          [env: NODE_URL=]
      --node-request-header-name <NODE_REQUEST_HEADER_NAME>
          [env: NODE_REQUEST_HEADER_NAME=]
      --node-request-header-value <NODE_REQUEST_HEADER_VALUE>
          [env: NODE_REQUEST_HEADER_VALUE=]
      --sync-start-block <SYNC_START_BLOCK>
          [env: SYNC_START_BLOCK=]
      --sync-end-block <SYNC_END_BLOCK>
          [env: SYNC_END_BLOCK=]
  -h, --help
          Print help
```

### Checking a single mnemonic

```rust
cargo run --release -- mnemonic-checker --derivation-paths "m/44'/0'/0'/0/0" "m/44'/0'/0'/0/1" "m/49'/0'/0'/0/0" "m/49'/0'/0'/0/1" "m/84'/0'/0'/0/0" "m/84'/0'/0'/0/1" --mnemonic "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon"
```

![single](demo/demo1.gif)

##### Docs

```bash
Usage: foogold mnemonic-checker [OPTIONS] --database-url <DATABASE_URL> --mnemonic <MNEMONIC>

Options:
      --telegram-token <TELEGRAM_TOKEN>
          [env: TELEGRAM_API_TOKEN=]
      --telegram-chat-id <TELEGRAM_CHAT_ID>
          [env: TELEGRAM_CHAT_ID=]
      --database-url <DATABASE_URL>
          [env: DATABASE_URL=postgres://postgres:postgres@localhost:5432/foogold]
      --database-pool-size <DATABASE_POOL_SIZE>
          [env: DATABASE_POOL_SIZE=] [default: 10]
      --derivation-paths <DERIVATION_PATHS>...
          [env: DERIVATION_PATHS=]
      --mnemonic <MNEMONIC>
          [env: MNEMONIC=]
  -h, --help
          Print help
```

### Checking combinations

```rust
RUST_LOG=info RUST_BACKTRACE=1 cargo run --release -- combination-checker --mnemonic-size 12 --combination-size 2 --derivation-paths "m/44'/0'/0'/0/0" "m/44'/0'/0'/0/1" "m/49'/0'/0'/0/0" "m/49'/0'/0'/0/1" "m/84'/0'/0'/0/0" "m/84'/0'/0'/0/1"
```

![combination](demo/demo3.gif)

##### Docs

```bash
Usage: foogold combination-checker [OPTIONS] --database-url <DATABASE_URL> --mnemonic-size <MNEMONIC_SIZE> --combination-size <COMBINATION_SIZE>

Options:
      --telegram-token <TELEGRAM_TOKEN>
          [env: TELEGRAM_API_TOKEN=]
      --telegram-chat-id <TELEGRAM_CHAT_ID>
          [env: TELEGRAM_CHAT_ID=]
      --database-url <DATABASE_URL>
          [env: DATABASE_URL=postgres://postgres:postgres@localhost:5432/foogold]
      --database-pool-size <DATABASE_POOL_SIZE>
          [env: DATABASE_POOL_SIZE=] [default: 10]
      --derivation-paths <DERIVATION_PATHS>...
          [env: DERIVATION_PATHS=]
      --mnemonic-size <MNEMONIC_SIZE>
          [env: MNEMONIC_SIZE=]
      --combination-size <COMBINATION_SIZE>
          [env: COMBINATION_SIZE=]
  -h, --help
          Print help
```

### Checking random mnemonics

```rust
RUST_LOG=info RUST_BACKTRACE=1 cargo run --release -- random-checker --mnemonic-size 12 --derivation-paths "m/44'/0'/0'/0/0" "m/44'/0'/0'/0/1" "m/49'/0'/0'/0/0" "m/49'/0'/0'/0/1" "m/84'/0'/0'/0/0" "m/84'/0'/0'/0/1"
```

![random](demo/demo4.gif)

### Docs

```bash
Usage: foogold random-checker [OPTIONS] --database-url <DATABASE_URL> --mnemonic-size <MNEMONIC_SIZE>

Options:
      --telegram-token <TELEGRAM_TOKEN>
          [env: TELEGRAM_API_TOKEN=]
      --telegram-chat-id <TELEGRAM_CHAT_ID>
          [env: TELEGRAM_CHAT_ID=]
      --database-url <DATABASE_URL>
          [env: DATABASE_URL=postgres://postgres:postgres@localhost:5432/foogold]
      --database-pool-size <DATABASE_POOL_SIZE>
          [env: DATABASE_POOL_SIZE=] [default: 10]
      --derivation-paths <DERIVATION_PATHS>...
          [env: DERIVATION_PATHS=]
      --mnemonic-size <MNEMONIC_SIZE>
          [env: MNEMONIC_SIZE=]
  -h, --help
          Print help
```
