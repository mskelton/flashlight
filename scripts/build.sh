#!/bin/bash

outdir="$(./scripts/cargo-out-dir.sh "${{ env.TARGET_DIR }}")"
staging="flashlight-${{ matrix.target }}"
mkdir -p "$staging"/complete

cp {README.md,LICENSE} "$staging/"
cp "$outdir"/{flashlight.bash,flashlight.fish,_flashlight.ps1} "$staging/complete/"
cp complete/_flashlight "$staging/complete/"

if [ "${{ matrix.os }}" = "windows-latest" ]; then
  cp "target/${{ matrix.target }}/release/flashlight.exe" "$staging/"
  7z a "$staging.zip" "$staging"
  echo "ASSET=$staging.zip" >> $GITHUB_ENV
else
  cp "target/${{ matrix.target }}/release/flashlight" "$staging/"
  tar czf "$staging.tar.gz" "$staging"
  echo "ASSET=$staging.tar.gz" >> $GITHUB_ENV
fi
