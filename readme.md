# Render handlebar templates with Rust compiled to WASM

A simple demo of rendering handlebar templates with Rust.  Compiling that code to web assembly and executing it in the browser.


## Running Locally

1. Compile to WASM:
    ```bash
    wasm-pack build --target web
    ```

1. Open index.html in the browser.  I used IntelliJ's built in http server. 

## Notes
* The file size for the WASM file is huge (500kb) which, I think, is probably attributed to serde_json.

