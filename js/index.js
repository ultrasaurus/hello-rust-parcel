import wasm from '../crate/Cargo.toml'

var greeting, err = wasm.hello("world");
console.log("hello", greeting, err);


greeting, err = wasm.hello()
console.log("hello", greeting, err);

