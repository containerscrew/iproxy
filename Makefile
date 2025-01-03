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

run: ## Run the code locally with cargo
	cargo run

package: ## Package binary with zip
	zip -j ${BINARY_NAME}-$(ARCH).zip target/$(TARGET)/release/${BINARY_NAME}

git-cliff: ## Run git cliff
	git cliff --output CHANGELOG.md

autoreload: ## Run the API with autoreload. Run cargo install cargo install cargo-watch systemfd
	systemfd --no-pid -s http::3000 -- cargo watch -w src -x run

container-build: ## Build the container
	docker build -t ${BINARY_NAME}:latest .

create-iproxy-network: ## Create iproxy network
	docker network create iproxy

compose-up-build: ## Run docker-compose up and build
	docker-compose -f compose.yml up -d --build --force-recreate

compose-up: ## Run docker-compose up
	docker-compose -f compose.yml up -d --force-recreate

compose-down: ## Run docker-compose down
	docker-compose -f compose.yml down

local-development: ## Run compose for local development
	docker-compose -f local.compose.yml up -d --force-recreate ;\
	CONFIG_FILE_PATH=./local.config.toml systemfd --no-pid -s http::3000 -- cargo watch -w src -x run

local-development-down: ## Run compose for local development
	docker-compose -f local.compose.yml down

podman-up: ## Run podman-compose
	podman-compose -f compose.yml up -d

podman-down: ## Run podman-compose down
	podman-compose -f compose.yml down
