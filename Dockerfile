FROM rust:1.73 AS builder
WORKDIR /opt
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /opt/target/release/app /opt/app
CMD ["/opt/app"]