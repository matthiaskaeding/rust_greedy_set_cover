################################################################################
# Targets
################################################################################

PACKAGE_NAME := set_cover

## Cargo duild
.PHONY: build
build:
	cargo build

## Run ./main
.PHONY: debug
debug:
	./target/debug/${PACKAGE_NAME}

## Run tests
.PHONY: test
test: 
	cargo test

## Build and debug
run: build debug

build-test: build test


################################################################################
# Self Documenting Commands
################################################################################
.DEFAULT_GOAL := help

define PRINT_HELP_PYSCRIPT
import re, sys; \
lines = '\n'.join([line for line in sys.stdin]); \
matches = re.findall(r'\n## (.*)\n[\s\S]+?\n([a-zA-Z_-]+):', lines); \
print('Available rules:\n'); \
print('\n'.join(['{:25}{}'.format(*reversed(match)) for match in matches]))
endef
export PRINT_HELP_PYSCRIPT

help:
	@python -c "$$PRINT_HELP_PYSCRIPT" < $(MAKEFILE_LIST)
