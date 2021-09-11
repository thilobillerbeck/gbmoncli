#!/bin/bash

cargo build --release
bash ./scripts/install-rules.sh
sudo cp ./target/release/gbmoncli /usr/local/bin/gbmoncli