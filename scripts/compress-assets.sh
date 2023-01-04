#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "gelatyx-$RELEASE_VERSION-$TARGET.zip" gelatyx-"$RELEASE_VERSION"/
else
  tar -czvf gelatyx-"$RELEASE_VERSION"-"$TARGET".tar.gz gelatyx-"$RELEASE_VERSION"/
  shasum -a 512 gelatyx-"$RELEASE_VERSION"-"$TARGET".tar.gz >gelatyx-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
