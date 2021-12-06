
cargo build --release --example circ_c
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/examples/circ_c jess/add.c

