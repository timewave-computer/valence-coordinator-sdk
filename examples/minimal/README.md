# Minimal Valence Coordinator example: Ticker Bot

A tiny coordinator that periodically ticks the configured Valence Processor on Neutron.

## Functionality overview

1. Read the `.env` config
2. Build a `NeutronClient` to make gRPC requests
3. Sets up the ticker bot
4. Starts periodically executing tick messages on
   the specified processor

## Setup

From the repository root, create your env file from the example:

```bash
cp examples/.env.example examples/.env
```

Only thing you need to fill in is the mnemonic; the rest can be kept as defaults.

## Run

```bash
RUST_LOG=info cargo run --example minimal
```

After building, you should start seeing the logs:

```bash
[2025-08-01T11:37:05Z INFO  COORDINATOR] starting coordinator: ticker_bot
[2025-08-01T11:37:05Z INFO  COORDINATOR] ticker_bot: worker started in new runtime
[2025-08-01T11:37:05Z INFO  minimal] ticker_bot cycle about to tick...
[2025-08-01T11:37:05Z WARN  minimal] ticking the processor failed: status: Unknown, message: "failed to execute message; message index: 0: There is currently nothing to process: execute wasm contract failed [CosmWasm/wasmd@v0.53.2/x/wasm/keeper/keeper.go:439] with gas used: '142025'", details: [], metadata: MetadataMap { headers: {"content-type": "application/grpc", "x-cosmos-block-height": "31286491"} }
[2025-08-01T11:37:05Z INFO  minimal] sleeping for 10sec...
```
