#!/bin/sh
# Rebuilds the SVGs and renders them to PNG (1200 px wide) for the blog.
# Usage: sh render.sh [output_dir]   (default: this folder)
set -e
cd "$(dirname "$0")"
OUT="${1:-.}"
python3 make_figures.py
for f in fig-*.svg; do
  rsvg-convert -w 1200 "$f" -o "$OUT/${f%.svg}.png"
done
