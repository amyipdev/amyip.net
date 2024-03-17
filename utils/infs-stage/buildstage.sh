cd "$(dirname "$0")"

if ! which infsprogs; then
	cargo install infsprogs
fi

infsprogs build -i 64 -b 512 -n 64 -o ../../svelte/wasm/pkg/i.iar stage/
