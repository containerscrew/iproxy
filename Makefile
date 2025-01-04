SHELL:=/bin/sh
.PHONY: all

VERSION = $(patsubst "%",%, $(word 3, $(shell grep version Cargo.toml)))
BUILD_TIME = $(shell date +"%Y/%m/%d %H:%M:%S")
GIT_REVISION = $(shell git log -1 --format="%h")

BINARY_NAME = iproxy

help: ## this help
	@awk 'BEGIN {FS = ":.*?## ";  printf "Usage:\n  make \033[36m<target> \033[0m\n\nTargets:\n"} /^[a-zA-Z0-9_-]+:.*?## / {gsub("\\\\n",sprintf("\n%22c",""), $$2);printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

mtoc: ## Create table of contents with mtoc
	mtoc

pre-commit: ## Run pre-commit
	pre-commit run -a

package: ## Package binary with zip
	zip -j ${BINARY_NAME}-$(ARCH).zip target/$(TARGET)/release/${BINARY_NAME}

git-cliff: ## Run git cliff
	git cliff --output CHANGELOG.md

run-with-autoreload: ## Run the API with autoreload. Run: $ cargo install cargo-watch systemfd
	systemfd --no-pid -s http::3000 -- cargo watch -w src -x run

create-iproxy-network: ## Create iproxy network
	podman network create iproxy

local-dev: ## Start local development
	podman run -itd --rm --name mongo -p 27017:27017 -e MONGO_INITDB_ROOT_USERNAME=admin -e MONGO_INITDB_ROOT_PASSWORD=admin -e MONGO_INITDB_DATABASE=iproxy docker.io/mongo:latest

compose-up: ## Run podman-compose
	podman-compose -f compose.yml up -d

compose-down: ## Run podman-compose down
	podman-compose -f compose.yml down
