#!/bin/bash

set -e

cd rust_of

PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release 2>/dev/null || PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 cargo build --release
cd ..

SOURCE_LIB="rust_of/target/release/librust_of.so"
DEST_LIB="rust_of.so"

# Check if the destination library exists and if it's identical to the source
if [ -f "$DEST_LIB" ] && cmp -s "$SOURCE_LIB" "$DEST_LIB"; then
  echo "librust_of.so is already up-to-date. Skipping copy."
else
  echo "Copying and renaming librust_of.so..."
  cp "$SOURCE_LIB" "$DEST_LIB"
fi

python3 conv.py
