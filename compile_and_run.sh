#!/usr/bin/env sh

if [[ -z $(type -t rustc) ]]; then
  echo Please first install rust!
  exit 1
fi

if [[ -r ./src/main.rs ]]; then
  rustc --out-dir build src/main.rs
else
  echo Didn’t find any readable src/main.rs file.
  exit 2
fi


if [[ -x ./build/main ]]; then
  exec build/main
  rm build/main
else
  echo Didn’t find any executable build/main file.
  exit 3
fi
