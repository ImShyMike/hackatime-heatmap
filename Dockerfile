FROM rust:1.93-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app
COPY Cargo.toml Cargo.lock build.rs ./
COPY src/ src/

RUN cargo build --release

FROM alpine:3.21

RUN apk add --no-cache ca-certificates font-dejavu

COPY --from=builder /app/target/release/hackatime-heatmap /usr/local/bin/

EXPOSE 8282 9292

CMD ["hackatime-heatmap"]
