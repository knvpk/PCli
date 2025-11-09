help:
	@echo "Welcome to Vikalp"

login otp:
    nix run .#cli aws sts-mfa {{otp}}

clean:
	cargo clean

test:
	cargo test

build:
	cargo build

[doc('Build cli app using nix.')]
build_cli:
	nix build .#cli

release:
	cargo build --release --all-targets

release_musl:
	cargo build --target x86_64-unknown-linux-musl --release

lint:
	cargo clippy

format:
	cargo fmt

set_husky:
	chmod -R +x .cargo-husky/hooks

check_options:
	cargo run --bin cli -- aws sts-mfa --help

# BUILD_DATE := `date -u +'%Y-%m-%dT%H:%M:%SZ'`
# VERSION := `yq '.package.version' cli/Cargo.toml`
# AUTHOR := `yq '.package.authors.0' cli/Cargo.toml`
# DESCRIPTION := `yq '.package.description' cli/Cargo.toml`
# NAME := `yq '.package.metadata.default.name' cli/Cargo.toml`
# VENDOR := `yq '.package.metadata.default.vendor' cli/Cargo.toml`

# build_cli:
# 	docker build \
# 		--build-arg="BUILD_DATE=$(BUILD_DATE)" \
# 		--build-arg="VERSION=$(VERSION)" \
# 		--build-arg="AUTHOR=$(AUTHOR)" \
# 		--build-arg="DESCRIPTION=$(DESCRIPTION)" \
# 		--build-arg="NAME=$(NAME)" \
# 		--build-arg="VENDOR=$(VENDOR)"  \
# 		-f=Dockerfile.cli \
# 		-t=cli:latest . \
# 		--no-cache