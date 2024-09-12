#########################################################################################
## docker will invoke this file from ../.. dir in order to access code
#########################################################################################
FROM rust:1.79-buster AS builder

# RUN apt-get update && apt-get -y install openssl libssl-dev

ARG BUILD_VERSION=0.1

WORKDIR /src/

COPY . /src

RUN --mount=type=cache,target=/usr/local/cargo/registry \
      CARGO_APP_VERSION=$BUILD_VERSION cargo build --release --bin rs-ws-client

#########################################################################################
FROM gcr.io/distroless/cc-debian11 AS ssl
#########################################################################################
# Debian 12 does not include ssl libs
FROM gcr.io/distroless/cc-debian12 AS runner
#########################################################################################
COPY LICENSE /licenses/LICENSE-bsd-3
ENV RUST_LOG=info

WORKDIR /app
### /ssl
COPY --from=ssl /usr/lib/x86_64-linux-gnu/libssl.so.* /lib/x86_64-linux-gnu/
COPY --from=ssl /usr/lib/x86_64-linux-gnu/libcrypto.so.* /lib/x86_64-linux-gnu/
#########################################################################################

COPY --from=builder /src/target/release/rs-ws-client /app/

ENTRYPOINT ["./rs-ws-client"]
