#!/usr/bin/env zsh

set -e

cwd=$(pwd)

inventory=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID)

WASMCLOUD_ACTOR_ID__BUY=$(wash -o json claims inspect ./buy/build/buy_s.wasm | jq '.module' | sed 's/\"//g')
# WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER="VDUPINNDEUVSMFRKQTOWBUCPSVOHK26TJ4LM7WPXVSZ74HIA24U3ZMLN"
WASMCLOUD_CONTRACT_ID__PAYMENTS="wasmcloud:examples:payments"
WASMCLOUD_CONTRACT_ID__ORDERS="orders"
# fp_provider_json=$(echo $inventory | jq '.inventory.providers[] | select(.name == "fakepay-provider")')
# echo $fp_provider_json

echo "Host <id=$WASMCLOUD_HOST_ID>"
echo "Buy actor <id=$WASMCLOUD_ACTOR_ID__BUY>"

wash ctl link del $WASMCLOUD_ACTOR_ID__BUY wasmcloud:examples:payments > /dev/null

BUY_ACTOR_RUNNING_ID=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID | jq '.inventory.actors[] | select(.name == "buy").id')

if [ -n "$BUY_ACTOR_RUNNING_ID" ]; then
    wash ctl stop actor $WASMCLOUD_HOST_ID $WASMCLOUD_ACTOR_ID__BUY --count 0
fi

WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID | jq '.inventory.providers[] | select(.name == "fakepay-provider").id' | sed 's/\"//g')

echo $WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER

if [ -n "$WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER" ]; then
    wash ctl stop provider $WASMCLOUD_HOST_ID $WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER default $WASMCLOUD_CONTRACT_ID__PAYMENTS
fi

wash drain oci

cd $cwd/fakepay-provider
make push
make start

WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER=$(wash -o json ctl get inventory $WASMCLOUD_HOST_ID | jq '.inventory.providers[] | select(.name == "fakepay-provider").id' | sed 's/\"//g')

cd $cwd/buy
make push
make start

wash ctl link put ${WASMCLOUD_ACTOR_ID__BUY} ${WASMCLOUD_PROVIDER_ID__FAKEPAY_PROVIDER} wasmcloud:examples:payments
