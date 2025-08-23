FROM rust:1.72 as builder

WORKDIR /usr/src/uomi_starter_agent
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo build --release --target wasm32-unknown-unknown

FROM scratch
COPY --from=builder /usr/src/uomi_starter_agent/target/wasm32-unknown-unknown/release/uomi_starter_agent.wasm /
