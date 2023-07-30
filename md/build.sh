#!/bin/sh

set -e

MD="comrak -e strikethrough,table,superscript,footnotes --unsafe --syntax-highlighting none"
DIR="$(cd -P -- "$(dirname -- "$0")" && pwd -P)"

find "$DIR" -name "*.md" -not -name "_*.md" | while read -r fname; do
  ofname="$DIR/../$(basename $fname md)html"
  printf "transforming %s => %s ... " "$fname" "$ofname"
  #mkdir -p "$DIR/../$(dirname $fname)"
  eval "$MD $DIR/_head.md" > "$ofname"
  eval "$MD $fname" >> "$ofname"
  eval "$MD $DIR/_tail.md" >> "$ofname"
  echo "ok"
done

