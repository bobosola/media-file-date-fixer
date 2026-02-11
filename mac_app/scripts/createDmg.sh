#!/bin/bash
# 
# https://github.com/sindresorhus/create-dmg

RELEASE_NAME="mfdf-2026-02-10-23-46-08"
APP_NAME="mfdf"
PROJECT_PATH="/Users/bobosola/rust/media-file-date-fixer/mac_app"
APP_PATH="$PROJECT_PATH/Builds/Releases/$RELEASE_NAME/$APP_NAME.app"
DEST_PATH="$PROJECT_PATH/Builds/Releases"

create-dmg $APP_PATH $DEST_PATH