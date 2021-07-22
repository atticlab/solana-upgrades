
# Overview

Example of data contract migration from one account to new one with versioning.

Contract can update data after its own upgrade.

## Example

1. Account 1 is created with program version 1 with 100 bytes of space.
2. Migration instruction runs, reads Account 1 data, changes it state and version.
3. Account 1 data is stored in bigger account of 123 bytes.

## Dev
### Build and test for program compiled natively
```shell
cargo build
cargo test
```

### Build and test the program compiled for BPF
```shell
cargo build-bpf
cargo test-bpf
```