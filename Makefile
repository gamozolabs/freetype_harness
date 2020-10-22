all:
	RUSTFLAGS="-Zlink-native-libraries=off" cargo build --release --target riscv64-unknown-linux-gnu.json -Z build-std=core,std,panic_abort

run: all
	qemu-riscv64 target/riscv64-unknown-linux-gnu/release/ft_harness

clean:
	cargo clean

