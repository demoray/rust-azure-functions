# Examples on building Azure Functions in Rust

## Build & Deploy
```bash
cd hello-world
cargo build --release
cp ../target/release/hello-world .
func azure functionapp publish ${FUNCTION_NAME} --custom | strings
```

## Test Locally
In shell oen:
```bash
func start | strings
```

In shell two:
```bash
curl https://bmc-test.azurewebsites.net/api/hello
```
