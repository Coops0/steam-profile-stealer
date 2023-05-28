FROM alpine:latest

COPY target/x86_64-unknown-linux-musl/release/steam-profile-stealer /usr/local/bin/

EXPOSE 8000

# Run the application
ENV RUST_BACKTRACE 1
CMD ["/usr/local/bin/steam-profile-stealer"]