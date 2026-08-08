# Achtung example agent

A minimal gRPC agent for the Achtung battle. It implements the `achtung.agent`
service (see [`protos/achtung_agent.proto`](protos/achtung_agent.proto)):

- `Initialize` — called once at the start of each game.
- `GetAction` — called every tick; returns the direction to steer.

This sample is deliberately dumb: it ignores the game state and picks a random
direction each tick (heavily weighted toward going straight). It exists to
exercise the pipeline and to serve as a starting point for your own agent —
put your logic in `GetAction`.

## Getting started

- Fork this repository.
- Implement your strategy in `get_action` in [`src/main.rs`](src/main.rs).
- Push to `main`; CI builds and publishes a container image to
  `ghcr.io/<your-user>/achtung-client-example`.
- Register the agent image in the Achtung web UI.

## Run locally

```sh
cargo run            # listens on 0.0.0.0:50052
PORT=50060 cargo run # override the port
```

Requires a Rust toolchain and `protoc` (protobuf compiler) on your `PATH`.
