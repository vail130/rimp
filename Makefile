default: fix build

.PHONY: fix
fix:
	cargo clippy --fix --allow-dirty

build: target/debug/openai
target/debug/openai:
	cargo build
