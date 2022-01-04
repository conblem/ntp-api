FROM rust:1.57 as builder
WORKDIR /ntp-api

COPY . .
RUN cargo install --path .

FROM debian:buster-slim
WORKDIR /ntp-api

COPY --from=builder /usr/local/cargo/bin/ntp-api /ntp-api/
CMD ["/ntp-api/ntp-api"]