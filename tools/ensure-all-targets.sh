#!/bin/bash
set -ex
test -f index.darwin-x64.node
test -f index.win32-x64-msvc.node
test -f index.linux-x64-gnu.node
