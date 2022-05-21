RUSTFLAGS='-C target-feature=+crt-static' cargo build --target=x86_64-unknown-linux-musl --release
cp target/x86_64-unknown-linux-musl/release/tencent pkg/tencent
cd pkg
chmod 777 ./*
zip pkg.zip bootstrap tencent