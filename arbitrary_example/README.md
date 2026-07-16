# arbitrary tutorial example

This example shows a minimal `arbitrary` setup. Unlike the fuzzing examples,
`arbitrary` is not a fuzzer by itself. It turns raw bytes into structured Rust
values that a fuzzer or test can use.

The generated input contains an IPv4 address and a port. The target renders
them as an endpoint, parses only the port, and intentionally panics on port
`0`.

## Files

```text
src/lib.rs
src/main.rs
```

## Setup

```bash
cd arbitrary_example
```

## Run

```bash
cargo run
cargo test
```

Expected normal output:

```text
Generated input: EndpointInput { ip: [192, 168, 1, 10], port: 8080 }
Rendered endpoint: 192.168.1.10:8080
Parsed port: 8080
```

## What it demonstrates

- `#[derive(Arbitrary)]` on a structured input type
- `arbitrary::Unstructured` turning a byte slice into a Rust value
- an IPv4 address represented as four generated octets
- a helper that feeds the rendered input into the target function
- target code that extracts only the port from the generated endpoint

## Key idea

This example does not fuzz by itself. Instead, it shows how to convert raw
bytes into a structured `EndpointInput`. That same pattern is commonly
combined with fuzzers such as `cargo-fuzz`, AFL, or Honggfuzz.
