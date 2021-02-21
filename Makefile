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

.PHONY: run
run:
	cargo run --release -- --dev --tmp

.PHONY: build
build:
	cargo build --release

run-node1:
	SKIP_WASM_BUILD=1 cargo run -- --base-path data/node1 --chain local --alice --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --name validator-alice --validator
	
run-node2:
	SKIP_WASM_BUILD=1 cargo run -- --base-path data/node2 --chain local --bob --port 30334 --telemetry-url 'wss://telemetry.polkadot.io/submit/ 0' --name validator-bob --validator