.PHONY: help build test clean format lint deploy-local deploy-devnet install validator pre-commit

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  %-20s %s\n", $$1, $$2}'

install: ## Install dependencies
	npm install
	cargo build --manifest-path programs/cfuk/Cargo.toml

build: ## Build the Anchor program
	anchor build

test: ## Run Anchor tests (spins up local validator automatically)
	anchor test

test-skip: ## Run tests against existing validator
	anchor test --skip-local-validator

clean: ## Clean build artifacts
	anchor clean
	cargo clean --manifest-path programs/cfuk/Cargo.toml
	rm -rf target/
	rm -rf node_modules/

format: ## Format all code (Rust + TypeScript)
	cargo fmt --manifest-path programs/cfuk/Cargo.toml
	npm run lint:fix

lint: ## Check code formatting
	cargo fmt --manifest-path programs/cfuk/Cargo.toml -- --check
	cargo clippy --manifest-path programs/cfuk/Cargo.toml -- -D warnings
	npm run lint

check: ## Run cargo check on Rust code
	cargo check --manifest-path programs/cfuk/Cargo.toml

deploy-local: ## Deploy to local validator
	anchor deploy

deploy-devnet: ## Deploy to devnet
	npm run dev

validator: ## Start a local Solana validator
	solana-test-validator

pre-commit: ## Run pre-commit hooks on all files
	pre-commit run --all-files

verify: format lint test ## Run all checks (format, lint, test)
