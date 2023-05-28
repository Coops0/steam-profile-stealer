FROM zenika/alpine-chrome

USER root
RUN apk add --no-cache tini make gcc g++

USER chrome

COPY --chown=chrome target/x86_64-unknown-linux-musl/release/steam-profile-stealer /usr/local/bin/

EXPOSE 8000

ENV RUST_BACKTRACE 1

ENTRYPOINT ["tini", "--"]
CMD ["/usr/local/bin/steam-profile-stealer"]