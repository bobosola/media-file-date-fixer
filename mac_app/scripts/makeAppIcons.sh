#!/bin/bash

# Generate macOS app icons from source image
# Handles grayscale+alpha PNGs properly

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SRC="$SCRIPT_DIR/calendar-icon-png-1024.png"
DEST="$SCRIPT_DIR/../mfdf/mfdf/Assets.xcassets/AppIcon.appiconset"

# Verify source exists
if [ ! -f "$SRC" ]; then
    echo "Error: Source file not found: $SRC"
    exit 1
fi

echo "Source: $SRC"
echo "Destination: $DEST"

# Check file type
echo "File type: $(file "$SRC")"

# Create destination directory if needed
mkdir -p "$DEST"

# Generate all required macOS icon sizes
# Using -s format png to ensure RGB output (avoids grayscale issues)

echo "Generating icons..."

# 16x16 (1x and 2x)
sips -z 16 16 "$SRC" --out "$DEST/AppIcon-16.png" 2>/dev/null || \
    sips "$SRC" -z 16 16 -s format png --out "$DEST/AppIcon-16.png"
    
sips -z 32 32 "$SRC" --out "$DEST/AppIcon-16@2x.png" 2>/dev/null || \
    sips "$SRC" -z 32 32 -s format png --out "$DEST/AppIcon-16@2x.png"

# 32x32 (1x and 2x)
sips -z 32 32 "$SRC" --out "$DEST/AppIcon-32.png" 2>/dev/null || \
    sips "$SRC" -z 32 32 -s format png --out "$DEST/AppIcon-32.png"
    
sips -z 64 64 "$SRC" --out "$DEST/AppIcon-32@2x.png" 2>/dev/null || \
    sips "$SRC" -z 64 64 -s format png --out "$DEST/AppIcon-32@2x.png"

# 128x128 (1x and 2x)
sips -z 128 128 "$SRC" --out "$DEST/AppIcon-128.png" 2>/dev/null || \
    sips "$SRC" -z 128 128 -s format png --out "$DEST/AppIcon-128.png"
    
sips -z 256 256 "$SRC" --out "$DEST/AppIcon-128@2x.png" 2>/dev/null || \
    sips "$SRC" -z 256 256 -s format png --out "$DEST/AppIcon-128@2x.png"

# 256x256 (1x and 2x)
sips -z 256 256 "$SRC" --out "$DEST/AppIcon-256.png" 2>/dev/null || \
    sips "$SRC" -z 256 256 -s format png --out "$DEST/AppIcon-256.png"
    
sips -z 512 512 "$SRC" --out "$DEST/AppIcon-256@2x.png" 2>/dev/null || \
    sips "$SRC" -z 512 512 -s format png --out "$DEST/AppIcon-256@2x.png"

# 512x512 (1x and 2x)
sips -z 512 512 "$SRC" --out "$DEST/AppIcon-512.png" 2>/dev/null || \
    sips "$SRC" -z 512 512 -s format png --out "$DEST/AppIcon-512.png"
    
cp "$SRC" "$DEST/AppIcon-512@2x.png"

echo ""
echo "Icons generated in: $DEST"
ls -la "$DEST"/AppIcon-*.png

echo ""
echo "✅ Done! Restart Xcode to see the icons."
