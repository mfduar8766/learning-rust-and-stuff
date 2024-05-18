FROM rust:1.78.0-alpine

COPY ./ ./

RUN cargo build

CMD ["./target/debug/postgres-seed"]
