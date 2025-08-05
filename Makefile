.PHONY: build test test-native test-sbf clean deploy

# Build the program for SBF
build:
	cargo build-sbf

# Run tests with SBF target (crucial for Solana float behavior)
test:
	cargo test-sbf

# Run native tests (for development/debugging)
test-native:
	cargo test

# Run SBF tests with verbose output
test-verbose:
	cargo test-sbf -- --nocapture

# Run specific SBF test categories
test-precision:
	cargo test-sbf precision_edge_cases

test-financial:
	cargo test-sbf financial_precision_tests

test-f64:
	cargo test-sbf f64_precision_tests

# Clean build artifacts
clean:
	cargo clean

# Build and test with SBF
all: build test

# Deploy to local validator (requires solana-test-validator running)
deploy: build
	solana program deploy target/deploy/solana_floats.so

# Show program logs
logs:
	solana logs
