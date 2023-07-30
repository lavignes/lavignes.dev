# Helo

This website is written in markdown. You can see the sources
[right here](https://github.com/lavignes/lavignes.dev). The beauty of it all is the fact that
my static site generator is [comrak](https://github.com/kivikakk/comrak) and a simple shell script:

```sh
#!/bin/sh

set -e

MD="comrak -e strikethrough,table,superscript,footnotes --unsafe --syntax-highlighting none"
DIR="$(cd -P -- "$(dirname -- "$0")" && pwd -P)"

find "$DIR" -name "*.md" -not -name "_*.md" | while read -r fname; do
  fdir="$(dirname "$fname")"
  odir="$DIR/..${fdir#"$DIR"}"
  ofname="$odir/$(basename "$fname" md)html"
  printf "transforming %s => %s ... " "$fname" "$ofname"
  mkdir -p "$odir"
  title=$(head -n 1 "$fname" | sed -e "s/^\\W*#*\\W*//")
  eval "$MD $DIR/_head.md" > "$ofname"
  echo "<title>$title</title>" >> "$ofname"
  eval "$MD $DIR/_prebody.md" >> "$ofname"
  eval "$MD $fname" >> "$ofname"
  eval "$MD $DIR/_tail.md" >> "$ofname"
  echo "ok"
done
```
