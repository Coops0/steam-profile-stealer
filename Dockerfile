################
##### Builder
FROM rust:slim as builder

WORKDIR /usr/src

# Create blank project
RUN USER=root cargo new app

# We want dependencies cached, so copy those first.
COPY Cargo.toml Cargo.lock /usr/src/app/

# Set the working directory
WORKDIR /usr/src/app


# cooper's hack to let any program of any name to still compile to 'app' binary
RUN mv /usr/src/app/Cargo.toml /usr/src/app/Cargo.toml2
RUN sed 's/.*name = ".*/name = "app"/' /usr/src/app/Cargo.toml2 > /usr/src/app/Cargo.toml

## Install target platform (Cross-Compilation) --> Needed for Alpine
RUN rustup target add x86_64-unknown-linux-musl

# This is a dummy build to get the dependencies cached.
RUN cargo build --target x86_64-unknown-linux-musl --release

# Now copy in the rest of the sources
COPY src /usr/src/app/src/

## Touch main.rs to prevent cached release build
RUN touch /usr/src/app/src/main.rs

# This is the actual application build.
RUN cargo build --target x86_64-unknown-linux-musl --release

################
##### Runtime
FROM alpine:latest AS runtime

# Copy application binary from builder image
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/app /usr/local/bin

EXPOSE 8000

# Run the application
CMD ["/usr/local/bin/app"]