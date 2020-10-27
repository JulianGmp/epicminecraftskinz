#!/bin/sh

make_release() {
	arch="$1"
	version="$2"
	fextension="$3"
	echo "Building v$version for $arch"
	rustup target add "$arch" && \
	cargo build --release --target="$arch" && \
	zip "epicminecraftskinz_v${version}_${arch}.zip" "./target/$arch/release/epicminecraftskinz${fextension}"
}

vers="0.2.0"

make_release "x86_64-unknown-linux-gnu" "$vers" ""
make_release "x86_64-pc-windows-gnu" "$vers" ".exe"
