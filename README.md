## Dev 

* `npm run start` -- Serve the project locally for
  development at `http://localhost:1234`.

* `npm run build` -- Bundle the project (in production mode)

##

npm run build

with hello_array  (with console log)
```
dist/rust_parcel_bg.47803ff0.wasm    50.65 KB     13ms
dist/js.dc2c2f8f.js.map              14.14 KB     15ms
dist/js.dc2c2f8f.js                   7.17 KB    4.67s
dist/Cargo.0218d37d.toml              1.08 KB    3.85s
dist/index.html                         228 B      8ms
```

with hello_array  (no console log)
```
dist/rust_parcel_bg.f002340b.wasm    23.12 KB     12ms
dist/js.f066ea3f.js.map              13.31 KB      6ms
dist/js.f066ea3f.js                   6.56 KB    1.80s
dist/Cargo.97e81dee.toml              1.08 KB    1.05s
dist/index.html                         228 B      6ms
```

hello a4063ae2d4c5a137a676568c3825af353e2b98b3
```
dist/rust_parcel_bg.8b1b2b6e.wasm    47.14 KB     16ms
dist/js.c290f29f.js.map               13.2 KB      6ms
dist/js.c290f29f.js                   6.44 KB    2.74s
dist/Cargo.bb9fabd3.toml              1.06 KB    1.52s
dist/index.html                         228 B      6ms
```



``` hello (a4063ae2d4c5a137a676568c3825af353e2b98b3 w/ log off)
dist/rust_parcel_bg.e0abe008.wasm     19.7 KB     10ms
dist/js.e729c419.js.map              12.37 KB      7ms
dist/js.e729c419.js                   5.83 KB    3.95s
dist/Cargo.677f4c65.toml              1.07 KB    3.25s
dist/index.html                         228 B      6ms
``


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