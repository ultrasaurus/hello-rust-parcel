import wasm from '../crate/Cargo.toml'

try {

  var greeting = wasm.hello("world");
  console.log('hello:', greeting);

  // var arr = wasm.hello_array(4);
  // console.log("hello_array:", arr);

  // var h = wasm.hello_hash(3);
  // console.log("hello_hash:", h);

  wasm.hello_fetch("foo").then((data) => {
    console.log(data);

    console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
    console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
  })

}
catch(error) {
  console.error(error);
}

// try {
//   var greeting = wasm.hello();
//   console.log('hello:', greeting);
// }
// catch(error) {
//   console.log("hello requires a parameter (error caught successfully)\n", error);
// }
