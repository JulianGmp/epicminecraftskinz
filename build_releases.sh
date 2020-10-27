#!/bin/sh

cd "$(dirname "$0")" || exit
rootdir=$(pwd)

make_release() {
	arch="$1"
	version="$2"
	fextension="$3"
	echo "Building v$version for $arch"
	rustup target add "$arch" && \
	cargo build --release --target="$arch" && \
	cd "./target/$arch/release" && \
	zip "$rootdir/epicminecraftskinz_v${version}_${arch}.zip" "./epicminecraftskinz${fextension}"
	cd "$rootdir" || return
}

vers="1.0.0"

make_release "x86_64-unknown-linux-gnu" "$vers" ""
make_release "x86_64-pc-windows-gnu" "$vers" ".exe"
