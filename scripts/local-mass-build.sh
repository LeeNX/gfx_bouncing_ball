#!/usr/bin/env sh
#
#
#

build_opts=${build_opts:-"--release"}

export CROSS_CONTAINER_ENGINE_NO_BUILDKIT=1
export CROSS_CONTAINER_ENGINE=podman
export HTTP_PROXY=http://192.168.101.10:3142

cross build ${build_opts} --target aarch64-unknown-linux-gnu
cross build ${build_opts} --target x86_64-unknown-linux-gnu
cross build ${build_opts} --target riscv64gc-unknown-linux-gnu

cargo build ${build_opts} --target x86_64-apple-darwin
cargo build ${build_opts} --target aarch64-apple-darwin

cross build ${build_opts} --target x86_64-pc-windows-gnu
