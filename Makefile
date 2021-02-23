run-tmp:
	cargo run -- --dev --tmp -lruntime=debug

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

.PHONY: init
init:
	./scripts/init.sh

.PHONY: check
check:
	SKIP_WASM_BUILD=1 cargo check --release

.PHONY: test
test:
	SKIP_WASM_BUILD=1 cargo test --release --all

run-release:
	cargo run --release -- --dev --tmp

run:
	SKIP_WASM_BUILD=1 cargo run -- --dev --tmp -lruntime=debug

build-release:
	cargo build --release

build:
	SKIP_WASM_BUILD=1 cargo build

run-node1:
	SKIP_WASM_BUILD=1 cargo run -- --base-path data/node1 --chain local --alice --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --name validator-alice --validator
	
run-node2:
	SKIP_WASM_BUILD=1 cargo run -- --base-path data/node2 --chain local --bob --port 30334 --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --name validator-bob --validator

cargo-expand-runtime:
	BUILD_DUMMY_WASM_BINARY=1 cargo expand -p node-template-runtime > runtime.rs

cargo-expand-kitties:
	BUILD_DUMMY_WASM_BINARY=1 cargo expand -p pallet-kitties > pallet_kitties.rs