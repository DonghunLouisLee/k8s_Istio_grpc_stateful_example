FROM rustlang/rust:nightly-stretch as builder

# muslc is required in order to build the rust image.
RUN apt-get update && apt-get -y install ca-certificates cmake musl-tools libssl-dev && rm -rf /var/lib/apt/lists/*

COPY . app
WORKDIR app
#RUN rustup nightly
RUN rustup toolchain remove nightly && rustup toolchain install nightly
RUN rustup component add rustfmt
RUN rustup target add x86_64-unknown-linux-musl
# Sets the environment variable for the cargo build command that fsollows.
ENV PKG_CONFIG_ALLOW_CROSS=1
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.8
# ENV variable
RUN apk --no-cache add ca-certificates
# ADD static static
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/poc-server .

CMD ["/poc-server"]