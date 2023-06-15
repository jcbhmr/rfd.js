#!/bin/bash
set -ex
if ! test -f index.*.node || [[ ! -f index.js || ! -f index.d.ts ]]; then
  if [[ $NODE_ENV == production ]]; then
    napi build --platform --release
  else
    napi build --platform
  fi
fi
