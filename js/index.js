import wasm from '../crate/Cargo.toml'

var greeting = wasm.hello("world");
console.log('hello:', greeting);

try {
  var result = wasm.hello_array(4);
  console.log("hello_array:", result);

  console.log("wasm.hello('world'):", wasm.hello('world'));

  greeting = wasm.hello();    // failure expected
}
catch(error) {
  console.error(error);
}
