BUILD_COMMAND = cargo build --release
OUTPUT_FILE = file-normalizer

clean:
	rm -rf ./$(OUTPUT_FILE)* Cargo.lock
	cargo clean
windows-release:
	$(BUILD_COMMAND) --target x86_64-pc-windows-gnu
	mv target/x86_64-pc-windows-gnu/release/file-normalizer-rs.exe ./$(OUTPUT_FILE).exe
	sha256sum $(OUTPUT_FILE).exe > ./$(OUTPUT_FILE).exe.sha256
nix-release:
	$(BUILD_COMMAND)
	mv target/release/file-normalizer-rs ./$(OUTPUT_FILE)
	sha256sum $(OUTPUT_FILE) > ./$(OUTPUT_FILE).sha256
