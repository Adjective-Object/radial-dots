FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
# binaryen supports wasm-opt for shrinking the
# size of the generated wasm file
RUN apt-get update
RUN apt-get -y install binaryen

# allow the user to party inside the cargo registry.
# this is typically pretty unsafe, but this container only exists in
# development, so go wild
RUN chmod -R +777 /usr/local/cargo/registry 

WORKDIR /app
COPY . .

RUN rustup component add clippy rustfmt
RUN rustup target add wasm32-unknown-unknown
