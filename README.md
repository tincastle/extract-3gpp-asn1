# extract-3gpp-asn1

Extract 3GPP ASN.1 from a plain text file and print to stdout

## Usage

```sh
Usage: extract-3gpp-asn1 <FILE>

Arguments:
  <FILE>  The file to process

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Cross-compilation

## Windows on Linux/macOS

```sh
# Add target
rustup target add x86_64-pc-windows-gnu
# Install linker
apt install -y mingw-w64 # Linux
brew install mingw-w64 # macOS
```

`Cargo.toml`

```toml
[target.x86_64-pc-windows-gnu]
linker = "x86_64-w64-mingw32-gcc"
ar = "x86_64-w64-mingw32-gcc-ar"
```

```sh
cargo build --release --target x86_64-pc-windows-gnu
```
