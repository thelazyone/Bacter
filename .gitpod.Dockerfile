FrOM gitpod/workspace-full

RUN bash -cl "rustup toolchain install stable && rustup target add wasm32-unknown-unknown"