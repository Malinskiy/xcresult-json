#!/usr/bin/env zsh

set -x

declare -A BUILDS
BUILDS=( ["darwin_amd64_v1"]="x86_64-apple-darwin" ["darwin_arm64"]="aarch64-apple-darwin")

if [ -z "$1" ] || [ -z "$2" ]; then
	echo "Specify goeleaser arch name and binary name"
	exit 1;
fi

# The args come from goreleaser.
GO_ARCH=$1
BIN=$2
RUST_ARCH=${BUILDS[$GO_ARCH]}
GO_PATH=dist/${BIN}_${GO_ARCH}

if [ -z "$RUST_ARCH" ]; then
	echo "${GO_ARCH} not found in the build map"
	exit 1;
fi

echo "building $GO_ARCH => $RUST_ARCH"

rm -rf $GO_PATH
rm -rf target

# Build.
cargo build --release --target=$RUST_ARCH

# Copy all results to goreleaser dist.
mkdir -p $GO_PATH
cp -R target/$RUST_ARCH/release/xcresult-json $GO_PATH
