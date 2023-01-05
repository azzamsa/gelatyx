#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

TARGET_DIR=gelatyx-"$RELEASE_VERSION"/

mkdir "$TARGET_DIR"

bin="gelatyx"
if [ "$OS" = "windows-2022" ]; then
  bin="gelatyx.exe"
fi
cp "target/$TARGET/release/$bin" "$TARGET_DIR"
