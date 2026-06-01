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