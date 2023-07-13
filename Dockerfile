# 1. Build Stage
FROM rust:1.70 as builder

WORKDIR /usr/src/hello_world
COPY . .
RUN cargo build --release

# 2. Test Stage
FROM builder as tester
RUN cargo test --release

# 3. Distroless Stage
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/src/hello_world/target/release/hello_world /usr/local/bin/hello_world

CMD ["hello_world"]
