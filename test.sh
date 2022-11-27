#!/bin/bash

set -ex

cargo fmt
cargo clippy -- -D warnings
cargo build
cargo build --release

USER="theempty"
NAME="ping-server"
TEST_REPO="192.168.7.7:5000"
BUILDX="pensive_albattani"
PLATFORMS="linux/amd64"

docker buildx build --builder ${BUILDX} -t ${TEST_REPO}/${USER}/${NAME} --push --platform=${PLATFORMS} .
kubectl rollout restart deployment/${NAME}
say "deploying" || true
sleep 90
kubectl logs -l app=${NAME}
