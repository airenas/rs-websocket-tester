-include Makefile.options
log?=INFO
###############################################################################
run/client:
	RUST_LOG=$(log) cargo run --bin rs-ws-client -- --url ws://localhost:8002
.PHONY: run/worker
###############################################################################
run/server:
	RUST_LOG=$(log) cargo run --bin rs-ws-server -- --port=8002
.PHONY: run/server
###############################################################################
build/local: 
	cargo build --release
.PHONY: build/local
###############################################################################
test/unit:
	RUST_LOG=DEBUG cargo test --no-fail-fast
.PHONY: test/unit
test/coverage:
	cargo tarpaulin --ignore-tests
.PHONY: test/coverage
.PHONY: test/unit	
test/lint:
	@cargo clippy -V
	cargo clippy --all-targets --all-features -- -D warnings
.PHONY: test/lint	
test/format:
	cargo fmt -- --check
.PHONY: test/format
audit:
	cargo audit
.PHONY: audit
install/checks:
	cargo install cargo-audit
	cargo install cargo-tarpaulin
.PHONY: install/checks
###############################################################################

.EXPORT_ALL_VARIABLES:

