#!/bin/sh
# https://github.com/messense/homebrew-macos-cross-toolchains

#brew tap messense/macos-cross-toolchains
#brew install messense/macos-cross-toolchains/x86_64-unknown-linux-musl
# originally x86_64-unknown-linux-gnu
#            x86_64-unknown-linux-musl
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-musl-gcc
export CC_x86_64_unknown_linux_musl=x86_64-linux-musl-gcc
export CXX_x86_64_unknown_linux_musl=x86_64-linux-musl-g++
export AR_x86_64_unknown_linux_musl=x86_64-linux-musl-ar

#rustup target add x86_64-unknown-linux-musl

cargo build --release --target x86_64-unknown-linux-musl

container=$(docker container ls --all --filter=ancestor=steam-profile-stealer --format "{{.ID}}")

docker build -t steam-profile-stealer .

echo killing $container
docker kill $container
docker rm $container

# new=$(docker run -d --restart unless-stopped -p 3853:8000 steam-profile-stealer)
new=$(docker run -d -p 3853:8000 steam-profile-stealer)
docker logs -f $new