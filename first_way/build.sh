RUSTFLAGS='-C target-feature=+crt-static' cargo build --target=x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/tencent pkg/bootstrap
cd pkg
chmod 777 ./bootstrap
zip pkg.zip bootstrap