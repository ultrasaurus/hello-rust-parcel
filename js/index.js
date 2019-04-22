import wasm from '../crate/Cargo.toml'

var greeting = wasm.hello("world");
console.log('hello:', greeting);

try {
  var arr = wasm.hello_array(4);
  console.log("hello_array:", arr);

  var h = wasm.hello_hash(3);
  console.log("hello_hash:", h);

  console.log("wasm.hello('world'):", wasm.hello('world'));

  greeting = wasm.hello();    // failure expected
}
catch(error) {
  console.error(error);
}
