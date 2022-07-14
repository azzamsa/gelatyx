help: # https://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

setup: ## Setup the repository.
	cargo watch --version || cargo install cargo-watch
	git-cliff --version || cargo install git-cliff

dev: ## Develop the app.
	cargo watch -x fmt -x clippy -x run

fmt: ## Format the codebase.
	cargo +nightly fmt

fmt_check: ## Check is the codebase properly formatted.
	cargo +nightly fmt --check

lint: ## Lint the codebase.
	cargo clippy

test: ## Lint the codebase.
	cargo test

comply: fmt lint test ## Tasks to make the code-base comply with the rules. Mostly used locally or in git hooks.

check:  fmt_check lint test ## Check if the repository comply with the rules. Mostly used in CI.


release:  ## Create a release
	bash scripts/release.sh $(version)

##
## Misc

update_dependencies: ## Check outdated dependencies.
	cargo update
	cargo outdated --root-deps-only
