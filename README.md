## Dev 

* `npm run start` -- Serve the project locally for
  development at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)

##

npm run build

slimmed down w/o web_sys and console.log
```
dist/js.3f076ec7.js.map               7.8 KB     12ms
dist/js.3f076ec7.js                   4.4 KB    3.60s
dist/Cargo.ab096783.toml             1.19 KB    3.01s
dist/rust_parcel_bg.c48a31f5.wasm      770 B     11ms
dist/index.html                        228 B      9ms
```

size of initial template
```
dist/rust_parcel_bg.39e6da81.wasm    50.37 KB     16ms
dist/js.b57570e6.js.map              14.49 KB      7ms
dist/js.b57570e6.js                   7.99 KB    3.12s
dist/Cargo.ae71bc13.toml              1.17 KB    1.81s
dist/index.html                         228 B    1.66s
```

## Dev Setup

```sh
cargo install wasm-pack
```

## About this

Created using `rust-parcel-template`

```sh
cargo install wasm-pack
```

```sh
npm init rust-parcel hello-rust-parcel
cd hello-rust-parcel
```


## References

https://rustwasm.github.io/docs/wasm-bindgen/reference/iterating-over-js-values.html