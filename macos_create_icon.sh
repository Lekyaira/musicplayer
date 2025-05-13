#!/bin/bash

# Check dependencies
command -v inkscape >/dev/null 2>&1 || { echo >&2 "Inkscape is required but not installed. Aborting."; exit 1; }
command -v iconutil >/dev/null 2>&1 || { echo >&2 "iconutil (macOS only) is required but not found. Aborting."; exit 1; }

# Check input
if [ $# -ne 1 ]; then
  echo "Usage: $0 path/to/icon.svg"
  exit 1
fi

SVG_FILE="$1"
ICNS_FILE="./resources/app_icon.icns"

# Create iconset directory
mkdir -p ./app_icon.iconset

# Create the icon set png images from the .svg
echo "Generating PNGs from SVG..."
for size in 16 32 64 128 256 512 1024; do
  echo "  - App Icon ($size x $size)"
  inkscape -o ./app_icon.iconset/icon_${size}x${size}.png -w $size -h $size $SVG_FILE
done

# Convert to .icns
echo "Creating .icns file..."
iconutil -c icns ./app_icon.iconset -o "$ICNS_FILE"

# Cleanup
echo "Cleaning up..."
rm -r ./app_icon.iconset

echo "✅ Done: $ICNS_FILE created."

