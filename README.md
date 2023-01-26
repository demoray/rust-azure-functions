# Examples on building Azure Functions in Rust

## Build & Deploy
```bash
cd hello-world
cargo build --release
cp ../target/release/hello-world .
func azure functionapp publish ${FUNCTION_NAME} --custom | strings
```

## Test Locally
In shell one:
```bash
func start | strings
```

In shell two:
```bash
curl https://${FUNCTION_NAME}.azurewebsites.net/api/hello
```
