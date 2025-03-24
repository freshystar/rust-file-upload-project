FROM rust:alpine AS build-env

WORKDIR /app

COPY . /app

# Install necessary build dependencies
RUN apk add --no-cache musl-dev upx

RUN cargo build --release

# Compress the binary using upx
RUN upx --best target/release/file-upload

# Use a minimal base image for the final image
FROM scratch

COPY --from=build-env /app/target/release/file-upload .

ENTRYPOINT ["./file-upload"]