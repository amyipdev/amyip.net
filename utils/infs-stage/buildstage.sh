cd "$(dirname "$0")"

if ! which infsprogs; then
	cargo install infsprogs
fi

infsprogs build -o ../../svelte/wasm/pkg/i.iar stage/