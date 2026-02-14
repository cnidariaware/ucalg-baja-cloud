# ----------- Build Stage -----------
FROM rust:alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    pkgconfig \
    musl-dev \
    openssl-dev \
    bash \
    make \
    g++ \
    cmake \
    libffi-dev
    
# Copy source and build
COPY . .
RUN cargo build --release  && strip target/release/ucalg-baja-cloud
    
# ----------- Runtime Stage -----------
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache \
    bash \
    openssl \
    musl \
    libffi
    
WORKDIR  /app
COPY --from=builder  /app/target/release/ucalg-baja-cloud .
    
EXPOSE 6525
CMD ["./ucalg-baja-cloud"]