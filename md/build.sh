#!/bin/sh

set -e

MD="comrak -e strikethrough,table,superscript,footnotes --unsafe --syntax-highlighting none"
DIR="$(cd -P -- "$(dirname -- "$0")" && pwd -P)"

echo "dir: $DIR"

find "$DIR" -name "*.md" -not -name "_*.md" | while read -r fname; do
  fdir="$(dirname "$fname")"
  odir="$DIR/..${fdir#"$DIR"}"
  ofname="$odir/$(basename "$fname" md)html"
  printf "transforming %s => %s ... " "$fname" "$ofname"
  mkdir -p "$odir"
  eval "$MD $DIR/_head.md" > "$ofname"
  eval "$MD $fname" >> "$ofname"
  eval "$MD $DIR/_tail.md" >> "$ofname"
  echo "ok"
done

