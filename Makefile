CARGO ?= cargo

.PHONY: build test fmt clippy clean

build:
	$(CARGO) build

test:
	$(CARGO) test

fmt:
	$(CARGO) fmt --all

clippy:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

clean:
	$(CARGO) clean
