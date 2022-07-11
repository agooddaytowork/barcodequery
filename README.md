# barcodequery

Demo of using Axum, Hyper, Tokio to build asynchronous application

- The application receives barcode from Barcode Scanner through serial or HDI
- The application queries barcode in persistent storage
  - If error, log to error persistent storage to look at later
  - If barcode is duplicated, 
  - If barcode exists in storage, send an Ok notification to inform user
- The application sends query result to web client through web socket

- Number of clients connecting via websockets: >=2

**Requirements**
- Load?
- What to do when barcode is duplicated?
- Requirement for websocket connection interface with client? Max client at a time?
- Other storage needed other than file?

## Structure
```
src -> source code + unit tests
tests -> integration tests
```

## Pre-requisites

You'll need to install:

- [Rust](https://www.rust-lang.org/tools/install)


## Getting Started
```bash
# build
cargo build

# test
cargo test
```