# cargo-fuzz tutorial example

This example shows a minimal `cargo-fuzz` setup. The target function is
`parse_port` in `src/lib.rs`. It parses UTF-8 bytes as a port number and
intentionally panics on input `0`.

## Files

```text
src/lib.rs
fuzz/Cargo.toml
fuzz/fuzz_targets/parse_port.rs
```

## Setup

```bash
cd cargo_fuzz_example
cargo install cargo-fuzz
rustup toolchain install nightly
rustup component add llvm-tools-preview --toolchain nightly
```

## Run

```bash
cargo run
cargo +nightly fuzz run parse_port
```

Expected normal output:

```text
Parsed port: 8080
```

## Saved inputs

- `fuzz/corpus/parse_port/` stores interesting non-crashing inputs.
- `fuzz/artifacts/parse_port/` stores crashing inputs.

Inspect a saved file with:

```bash
xxd fuzz/artifacts/parse_port/<file>
cat fuzz/artifacts/parse_port/<file>
```

## Replay

```bash
cargo +nightly fuzz run parse_port fuzz/artifacts/parse_port/<crash-file>
# Minimize the smallest input to reproduce the failure
cargo +nightly fuzz tmin parse_port fuzz/artifacts/parse_port/<crash-file>
```

## Coverage

Coverage helps confirm that the corpus is exercising the interesting paths in `parse_port`, instead of only proving that the fuzzer runs.
In this example it is a quick way to see whether the saved inputs actually reach the panic on `0` or if we still need to grow the corpus.

Generate coverage data from the saved corpus:

```bash
cargo +nightly fuzz coverage --sanitizer none parse_port fuzz/corpus/parse_port
```

This writes the merged profile to:

```text
fuzz/coverage/parse_port/coverage.profdata
```

Set the nightly LLVM tools directory once:

```bash
LLVM_BIN="$(dirname "$(rustc +nightly --print target-libdir)")/bin"
```

Show a summary report with `llvm-cov`:

```bash
"$LLVM_BIN"/llvm-cov report \
  target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/parse_port \
  --instr-profile fuzz/coverage/parse_port/coverage.profdata
```

Show line-by-line coverage for the example library:

```bash
"$LLVM_BIN"/llvm-cov show \
  target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/parse_port \
  --instr-profile fuzz/coverage/parse_port/coverage.profdata \
  src/lib.rs
```

Generate an HTML coverage report:

```bash
"$LLVM_BIN"/llvm-cov show \
  target/x86_64-unknown-linux-gnu/coverage/x86_64-unknown-linux-gnu/release/parse_port \
  --instr-profile fuzz/coverage/parse_port/coverage.profdata \
  --format=html \
  --output-dir fuzz/coverage/parse_port/html \
  src/lib.rs
```

Open `fuzz/coverage/parse_port/html/index.html` in a browser to view the report.

Note: using the default sanitizer for `cargo fuzz coverage` failed in this environment with a LeakSanitizer error during corpus merge, while `--sanitizer none` completed successfully.
