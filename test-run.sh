#!/usr/bin/bash

cargo run --profile dev-rel -- \
    --from-source \
    --xray-version v26.1.18 \
    --verbose
