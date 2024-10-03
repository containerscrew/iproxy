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

autoreload: ## Run the API with autoreload
	systemfd --no-pid -s http::3000 -- cargo watch -x run

container-build: ## Build the container
	docker build -t ${BINARY_NAME}:latest .

compose-up-build: ## Run docker-compose up and build
	docker-compose -f compose.yml up -d --build --force-recreate

compose-up: ## Run docker-compose up
	docker-compose -f compose.yml up -d --force-recreate

compose-down: ## Run docker-compose down
	docker-compose -f compose.yml down
