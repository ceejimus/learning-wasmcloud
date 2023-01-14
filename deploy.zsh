#!/usr/bin/env zsh

set -e

inventory=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID)


WASMCLOUD_ACTOR_ID__BUY=$(wash -o json claims inspect ./buy/build/buy_s.wasm | jq '.module' | sed 's/\"//g')
WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER="VDUPINNDEUVSMFRKQTOWBUCPSVOHK26TJ4LM7WPXVSZ74HIA24U3ZMLN"
WASMCLOUD_CONTRACT_ID__PAYMENTS="wasmcloud:examples:payments"
WASMCLOUD_CONTRACT_ID__ORDERS="orders"
# fp_provider_json=$(echo $inventory | jq '.inventory.providers[] | select(.name == "fakepay-provider")')
# echo $fp_provider_json

wash ctl link del $WASMCLOUD_ACTOR_ID__BUY wasmcloud:examples:payments

BUY_ACTOR_RUNNING_ID=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID | jq '.inventory.actors[] | select(.name == "buy").id')

if [ -n "$BUY_ACTOR_RUNNING_ID" ]; then
    wash ctl stop actor $WASMCLOUD_HOST_ID $WASMCLOUD_ACTOR_ID__BUY
fi

FAKE_PAY_PROVIDER_RUNNING_ID=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID | jq '.inventory.providers[] | select(.name == "fakepay-provider").id')

if [ -n "$FAKE_PAY_PROVIDER_RUNNING_ID" ]; then
    wash ctl stop provider $WASMCLOUD_HOST_ID $WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER default $WASMCLOUD_CONTRACT_ID__PAYMENTS
fi

wash drain oci

CURRENT_DIR=$(pwd)
cd ./buy
make push
make start

cd ../fakepay-provider
make push
make start

sleep 1

wash ctl link put ${WASMCLOUD_ACTOR_ID__BUY} ${WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER} wasmcloud:examples:payments
