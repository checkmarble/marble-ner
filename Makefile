.PHONY: prepare run

OS_NAME := $(shell uname -s)

prepare:
	@if [ -z "$(TARGET) ]; then $(error "TARGET variable is required to be either 'cpu' or 'gpu'"); fi

ifeq ($(TARGET), gpu)
	@ln -sf pyproject.gpu.toml pyproject.toml
	@ln -sf poetry.gpu.lock poetry.lock
else ifeq ($(TARGET), cpu)
	@if [ "$(OS_NAME)" == 'Darwin' ]; then echo "WARN: pytorch CPU-builds are not provided for macOS."; fi

	@ln -sf pyproject.cpu.toml pyproject.toml
	@ln -sf poetry.cpu.lock poetry.lock
else
	@echo "ERROR: unsupported target, should be 'cpu' or 'gpu'."
endif

run:
	poetry run uvicorn --workers=1 --port 9000 main:app
