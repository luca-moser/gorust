#!/bin/bash
sh ./build.sh || exit
SCRIPTPATH="$(
  cd "$(dirname "$0")" >/dev/null 2>&1 || exit
  pwd -P
)"
cd "$SCRIPTPATH" || exit
go run -ldflags="-r ${SCRIPTPATH}/lib" main.go
