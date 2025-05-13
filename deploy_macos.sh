#!/bin/bash
set -e

# Colors for prettier output
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${BLUE}=== Music Player macOS Deployment Script ===${NC}"

# Check if running on macOS
if [[ "$(uname)" != "Darwin" ]]; then
    echo -e "${RED}Error: This script must be run on macOS.${NC}"
    exit 1
fi

# Get the Cargo target directory
CARGO_TARGET_DIR=$(cargo metadata --format-version 1 | jq -r '.target_directory')

# Configuration
APP_NAME="MusicPlayer"
VERSION=$(grep '^version' Cargo.toml | head -n 1 | cut -d '"' -f 2)
BUILD_TYPE="release"
DMG_NAME="${APP_NAME}-${VERSION}.dmg"

# Ensure clean build
echo -e "${YELLOW}Cleaning previous builds...${NC}"
rm -rf "${APP_NAME}.app" "${DMG_NAME}" $CARGO_TARGET_DIR/${BUILD_TYPE}

# Build the application in release mode
echo -e "${YELLOW}Building ${APP_NAME} in ${BUILD_TYPE} mode...${NC}"
cargo build --${BUILD_TYPE}

# Create app structure
echo -e "${YELLOW}Creating macOS app bundle structure...${NC}"
mkdir -p "${APP_NAME}.app/Contents/MacOS"
mkdir -p "${APP_NAME}.app/Contents/Resources"

# Copy executable
echo -e "${YELLOW}Copying executable to app bundle...${NC}"
cp "$CARGO_TARGET_DIR/${BUILD_TYPE}/musicplayer" "${APP_NAME}.app/Contents/MacOS/"

# Check if the executable has proper permissions
chmod +x "${APP_NAME}.app/Contents/MacOS/musicplayer"

# Create Info.plist
echo -e "${YELLOW}Creating Info.plist...${NC}"
cat > "${APP_NAME}.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleName</key>
    <string>Music Player</string>
    <key>CFBundleDisplayName</key>
    <string>Music Player</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon.icns</string>
    <key>CFBundleIdentifier</key>
    <string>com.musicplayer.app</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleSignature</key>
    <string>????</string>
    <key>CFBundleExecutable</key>
    <string>musicplayer</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>CFBundleDocumentTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeName</key>
            <string>Audio Files</string>
            <key>CFBundleTypeRole</key>
            <string>Viewer</string>
            <key>LSHandlerRank</key>
            <string>Alternate</string>
            <key>LSItemContentTypes</key>
            <array>
                <string>public.mp3</string>
                <string>public.audio</string>
                <string>public.wav</string>
                <string>public.ogg-audio</string>
                <string>public.flac-audio</string>
                <string>public.aac-audio</string>
                <string>public.m4a-audio</string>
                <string>com.microsoft.wma-audio</string>
            </array>
        </dict>
    </array>
    <key>NSAppleEventsUsageDescription</key>
    <string>This app needs access to run AppleScript to handle file events</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © $(date +%Y)</string>
</dict>
</plist>
EOF

# Create the app icon
./macos_create_icon.sh resources/music_note.svg

# Create an icon placeholder (or copy your actual icon if you have one)
echo -e "${YELLOW}Adding app icon...${NC}"
if [ -f "resources/app_icon.icns" ]; then
    cp "resources/app_icon.icns" "${APP_NAME}.app/Contents/Resources/AppIcon.icns"
else
    echo -e "${YELLOW}No app icon found at resources/app_icon.icns. Using default macOS icon.${NC}"
    # The app will use a default document icon
fi

# Create a simple PkgInfo file
echo -e "${YELLOW}Creating PkgInfo...${NC}"
echo "APPL????" > "${APP_NAME}.app/Contents/PkgInfo"

# Fix file permissions
echo -e "${YELLOW}Setting correct file permissions...${NC}"
chmod -R 755 "${APP_NAME}.app"

echo -e "${GREEN}App bundle created successfully at ${APP_NAME}.app${NC}"

# Optionally create a DMG (disk image) for distribution
if command -v create-dmg &> /dev/null; then
    echo -e "${YELLOW}Creating DMG for distribution...${NC}"
    create-dmg \
        --volname "${APP_NAME}" \
        --volicon "${APP_NAME}.app/Contents/Resources/AppIcon.icns" \
        --window-pos 200 120 \
        --window-size 600 400 \
        --icon-size 100 \
        --icon "${APP_NAME}.app" 175 190 \
        --hide-extension "${APP_NAME}.app" \
        --app-drop-link 425 190 \
        "${DMG_NAME}" \
        "${APP_NAME}.app"
    
    echo -e "${GREEN}DMG created successfully at ${DMG_NAME}${NC}"
else
    echo -e "${YELLOW}create-dmg not found. To create a DMG, install it with:${NC}"
    echo -e "    brew install create-dmg"
    echo -e "${YELLOW}and run this script again.${NC}"
    
    # Alternative: Create a simple zip file
    echo -e "${YELLOW}Creating a zip archive instead...${NC}"
    zip -r "${APP_NAME}-${VERSION}.zip" "${APP_NAME}.app"
    echo -e "${GREEN}Zip archive created at ${APP_NAME}-${VERSION}.zip${NC}"
fi

echo -e "${GREEN}Deployment complete!${NC}"
echo -e "${BLUE}To install, copy ${APP_NAME}.app to your Applications folder.${NC}" 