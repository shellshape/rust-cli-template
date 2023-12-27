#!/bin/bash

NAME="$1"

if [ -z "$NAME" ]; then
    echo "usage: $(basename "$0") NAME"
    exit 1
fi

DIR=$(dirname "$0")

sed 's/name = "rust-cli-template"/name = "'"$NAME"'"/' "$DIR/Cargo.toml" > .tmp
mv -v .tmp "$DIR/Cargo.toml"

shift

if [ "$1" != "--debug" ]; then
    echo -e "# $NAME\n\n" > "$DIR/README.md"
    rm -vrf "$DIR/LICENSE"
    if [[ "$(git -C "$DIR" remote get-url origin)" == *"shellshape/rust-cli-template"* ]]; then
        rm -vrf "$DIR/.git"
        git -C "$DIR" init
    fi
    rm -vrf "$0"
fi

echo -e "\n---------------------------------------------------------"
echo "| Setup finished!"
echo "| Please create a new LICENSE file and update the README.md."
echo "| Happy building and thank you for using this template! ^-^"