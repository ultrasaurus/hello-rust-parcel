## Dev Setup

Node/npm versions (`nvm install 12.14.1`):
````
$ npm --version
6.13.4
$ node --version
v12.14.1
````

Created using [rust-parcel-template]

```sh
cargo install wasm-pack --force  ## force option upgrades if already installed
```


```sh
npm init rust-parcel hello-rust-parcel
# npx: installed 1 in 1.721s
# 🦀Rust + 🕸 WebAssembly + 📦Parcel = ❤️
# Installed dependencies ✅
cd hello-rust-parcel
npm run start
# > create-rust-parcel@0.0.2 start /Users/sallen/src/rust/hello-rust-parcel
# > parcel index.html
# Server running at http://localhost:1234 
# ⠋ Building Cargo.toml...
```

The first time you run this it is **very slow** because it needs to update all
the dependencies.

Browser window should show "Hello from Rust, WebAssembly, and Parcel!"
where innerHTML is set from Rust/wasm.

