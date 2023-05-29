#!/bin/sh

container=$(docker container ls --all --filter=ancestor=steam-profile-stealer --format "{{.ID}}")

DOCKER_BUILDKIT=1 docker build -t steam-profile-stealer .

echo killing $container
docker kill $container
docker rm $container

# new=$(docker run -d --restart unless-stopped -p 3853:8000 steam-profile-stealer)
new=$(docker run --restart unless-stopped -d -p 3853:8000 steam-profile-stealer)
docker logs -f $new