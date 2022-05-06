#!/bin/sh

cargo lambda build --release --target x86_64-unknown-linux-gnu --output-format zip
