
cargo build --release --example circ_c
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/examples/circ_c jess/add.c


RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/examples/circ ./examples/ZoKrates/mpc/unit_tests/arithmetic_tests/2pc_add.zok r1cs --action count


