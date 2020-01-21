import wasm from '../crate/Cargo.toml'

try {
  var greeting = wasm.hello("world");
  console.log('hello:', greeting);

  var arr = wasm.hello_array(4);
  console.log("hello_array:", arr);

  var h = wasm.hello_hash(3);
  console.log("hello_hash:", h);

}
catch(error) {
  console.error(error);
}
