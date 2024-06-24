```
cd auth
cargo component build --release
cd ../greet
cargo component build --release
cd ../router2
cargo component build --target wasm32-unknown-unknown --release
```

```
cd ..
wac encode compose.wac -d component:auth=./auth/target/wasm32-wasi/release/auth.wasm -d component:greet=./greet/target/wasm32-wasi/release/greet.wasm -d component:router2=./router2/target/wasm32-unknown-unknown/release/router2.wasm -o COMPOSED.wasm --registry wa.dev
spin up -f COMPOSED.wasm
```
