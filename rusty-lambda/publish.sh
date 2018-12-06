
rm rust.zip
cargo build --release --target x86_64-unknown-linux-musl
zip -j rust.zip ./target/x86_64-unknown-linux-musl/release/bootstrap
aws lambda update-function-code --function-name RustyLambda --zip-file fileb://rust.zip
