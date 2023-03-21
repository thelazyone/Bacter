FROM gitpod/workspace-full

    # Set up wasm-pack and wasm32-unknown-unknown for rustpython_wasm
RUN [rustup target add wasm32-unknown-unknown]
RUN cargo install wasm-bindgen-cli 
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh