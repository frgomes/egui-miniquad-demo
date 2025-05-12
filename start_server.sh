#!/bin/bash
set -eu

# Starts a local web-server that servs the contents of the `doc/` folder,
# which is the folder to where the web version is compiled.

cargo install basic-http-server

# kills existing running server, if any
pid=$(ss -plnt | grep basic-http-serv | grep -P -o '(?<=pid=)[0-9]+')
[[ -z "${pid}" ]] || kill "${pid}"

(cd docs && basic-http-server --addr 127.0.0.1:8080 .)
# (cd docs && python3 -m http.server 8080 --bind 127.0.0.1)
