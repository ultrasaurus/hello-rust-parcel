

## Production build

Building for production also provides output about size


### without web-sys

For this small sample web-sys adds ~12KB

```
$ npm run build

> create-rust-parcel@0.0.2 build /Users/sallen/src/rust/hello-rust-parcel
> parcel build index.html

✨  Built in 1.64s.

dist/hello_rust_parcel_bg.ee35688a.wasm       43 KB     10ms
dist/js.be34be9e.js.map                    12.46 KB      5ms
dist/js.be34be9e.js                         6.09 KB    1.67s
dist/Cargo.856619e7.toml                    1.21 KB    1.07s
dist/index.html                              228  B      5ms
```


### initial template
```
$ npm run build

> create-rust-parcel@0.0.2 build /Users/sallen/src/rust/hello-rust-parcel
> parcel build index.html

✨  Built in 2.25s.

dist/hello_rust_parcel_bg.fcada054.wasm    48.18 KB     11ms
dist/js.c5d8a472.js.map                    16.29 KB      6ms
dist/js.c5d8a472.js                         9.22 KB    1.79s
dist/Cargo.430570ab.toml                    1.19 KB    636ms
dist/index.html                              228  B    899ms
```



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

Browser window is blank

