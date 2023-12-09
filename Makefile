BUILD_COMMAND = cargo build --release

clean:
	cargo clean
	rm -rf ./file-normalizer-rs* Cargo.lock
	cargo check
	cargo clean
windows-release:
	$(BUILD_COMMAND) --target x86_64-pc-windows-gnu
	mv target/x86_64-pc-windows-gnu/release/file-normalizer-rs.exe .
	sha256sum file-normalizer-rs.exe > ./file-normalizer-rs.exe.sha256
nix-release:
	$(BUILD_COMMAND)
	mv target/release/file-normalizer-rs .
	sha256sum file-normalizer-rs > ./file-normalizer-rs.sha256
