#!/usr/bin/make -f
VERSION := $(shell echo $(shell git describe --tags) | sed 's/^v//')
COMMIT := $(shell git log -1 --format='%H')

CURRENT_DIR := $(shell pwd)
BASE_DIR := $(shell basename $(CURRENT_DIR))

compile:
	@echo "Compiling Contract $(COMMIT)..."	
	@docker run --rm -v "$(CURRENT_DIR)":/code \
	--mount type=volume,source="$(BASE_DIR)_cache",target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/rust-optimizer:0.12.13

run-test:
	cargo run --package interchaintest-e2e --bin interchaintest-e2e

all:
	cargo schema	
	cargo fmt
	cargo test
	cargo clippy -- -D warnings