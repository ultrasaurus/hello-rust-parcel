import wasm from '../crate/Cargo.toml'

var greeting = wasm.hello("world");
console.log('hello:', greeting);
