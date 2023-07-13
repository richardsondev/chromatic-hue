# 1. Build Stage
FROM rust:1.70 as builder

WORKDIR /usr/src/chromatic-hue
COPY . .
RUN cargo build --release

# 2. Test Stage
FROM builder as tester
RUN cargo test --release

# 3. Distroless Stage
FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/src/chromatic-hue/target/release/chromatic-hue /usr/local/bin/chromatic-hue

CMD ["chromatic-hue"]
