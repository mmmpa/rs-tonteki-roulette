# とんてきルーレット

## Build

read: https://github.com/awslabs/aws-lambda-rust-runtime

```shell
rustup target add x86_64-unknown-linux-gnu
cargo install cargo-lambda
cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip
```

## Deploy

```shell
cd terraform
terraform init
terraform apply
```
