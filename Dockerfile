# Base image
FROM rust:1.77-bookworm AS base
WORKDIR /app
RUN update-ca-certificates
RUN apt update -y && apt clean
RUN apt install build-essential -y
RUN apt install lld -y
RUN apt install clang -y
RUN rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

FROM base AS builder
ENV USER=app
ENV UID=10001
RUN adduser --disabled-password --gecos "" --home "/nonexistent" --shell "/sbin/nologin" --no-create-home --uid "${UID}" "${USER}"
WORKDIR /app
COPY . .
RUN cargo build --release
RUN strip -s target/release/kawalhaji

# final outcome
FROM gcr.io/distroless/cc-debian12
WORKDIR /app
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /app/target/release/kawalhaji ./
COPY --from=builder /lib/x86_64-linux-gnu/libz.so.1 /lib/x86_64-linux-gnu/
EXPOSE 8080
CMD ["./kawalhaji"]