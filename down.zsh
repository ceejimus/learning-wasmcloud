#!/usr/bin/env zsh

set -e

RUST_LOG=DEBUG,hyper=INFO

wash down

docker kill registry
