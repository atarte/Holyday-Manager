#!/bin/bash

echo "#######################################"
echo "#                                     #"
echo "# Building script for Holyday Manager #"
echo "#                                     #"
echo "#######################################"
echo ""

# Compiling rust code
$CARGO_HOME/bin/cargo build --release

# Add icon to the release forlder
cp ./icon.png ./target/release/icon.png
