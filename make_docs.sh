#!/bin/sh

cargo doc --package aoc_utils --no-deps
rm -rf ./docs
echo "<meta http-equiv=\"refresh\" content=\"0; url=aoc_utils\">" > target/doc/index.html
cp -r target/doc ./docs
