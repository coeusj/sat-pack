# CCSDS Packet: Generator and Reader

A simple CCSDS packet generator and parser

## Compile proto files

`build.rs` uses `tonic-prost-build` to compile proto files.

Compile:

```bash
cargo build
```

## Run gRPC server

```bash
cargo run --bin grpc-server
```

## Simulation configuration

Configuration file: `Settings.toml`

Section `[ccsds_conf]`: you can modify the values of the CCSDS packet.
Section `[sim_conf]`: you can modify the delay in the update/send loop.
