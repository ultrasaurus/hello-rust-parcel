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

try {
  var greeting = wasm.hello();
  console.log('hello:', greeting);
}
catch(error) {
  console.log("hello requires a parameter (error caught successfully)\n", error);
}

wasm.async_hello().then(result => {
  console.log("async_hello:", result)
});