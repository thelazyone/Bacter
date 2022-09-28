FROM gitpod/workspace-full

    # Set up wasm-pack and wasm32-unknown-unknown for rustpython_wasm
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh && \
    rustup target add wasm32-unknown-unknown