#!/bin/bash

set -e

cd of

PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release 2>/dev/null || PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release
cd ..

SOURCE_LIB="of/target/release/libof.so"
DEST_LIB="of.so"

# Check if the destination library exists and if it's identical to the source
if ! [ -f "$DEST_LIB" ] || ! cmp -s "$SOURCE_LIB" "$DEST_LIB"; then
  cp "$SOURCE_LIB" "$DEST_LIB"
fi

python3 conv.py
python3 -m unittest test_error_attribute.py
