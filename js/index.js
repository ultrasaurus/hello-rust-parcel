import wasm from '../crate/Cargo.toml'

// 1. Async JS => Rust => JS
// 2. HTTP Request from Rust

// fetch('http://localhost:1234/sample.json')
//   .then((response) => {
//     //debugger;
//     // response.blob();
//     return response.json();
//   })
//   .then((myJson) => {
//     console.log(myJson);
//   });

// localhost/:1 Access to fetch at 'http://localhost/' from origin 'http://localhost:1234' has been blocked by CORS policy: No 'Access-Control-Allow-Origin' header is present on the requested resource. If an opaque response serves your needs, set the request's mode to 'no-cors' to fetch the resource with CORS disabled.
// index.js:9 Cross-Origin Read Blocking (CORB) blocked cross-origin response http://localhost/ with MIME type text/html. See https://www.chromestatus.com/feature/5629709824032768 for more details.


try {

  wasm.hello_fetch("foo").then((data) => {
    console.log(data);

    // console.log("The latest commit to the wasm-bindgen %s branch is:", data.name);
    // console.log("%s, authored by %s <%s>", data.commit.sha, data.commit.commit.author.name, data.commit.commit.author.email);
  })

  // var greeting = wasm.hello("world");
  // console.log('hello:', greeting);

  // var arr = wasm.hello_array(4);
  // console.log("hello_array:", arr);

  // var h = wasm.hello_hash(3);
  // console.log("hello_hash:", h);
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
