#!/usr/bin/env zsh

# set -e

docker run -d --rm -p 5000:5000 --name registry registry:2.7 \
    && \
    RUST_LOG=DEBUG,hyper=INFO wash up \
    --detached \
    --output=json \
    --enable-structured-logging \
    --structured-log-level=debug \
    && \
    . ./setenv
