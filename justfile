default:
	just --list

ci: build test clippy fmt-check

build:
	cargo build

test:
	cargo test

clippy:
  cargo clippy --all-targets --all-features

fmt-check:
  cargo +nightly fmt --all -- --check
  @echo formatting check done

run subcommand *args:
	cargo run {{subcommand}} {{args}}

fmt:
	cargo +nightly fmt

check:
	cargo check --all-features --all-targets

watch +COMMAND='test':
	cargo watch --clear --exec "{{COMMAND}}"

usage:
	cargo run -- --help | pbcopy

bin:
	cp -r ./target/debug/zk /usr/local/bin
