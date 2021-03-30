#!/bin/bash -eu

function build_wasm {
    local self=$(readlink -f "${BASH_SOURCE[0]}")
    local dir=$(dirname $self)
    local app=$(basename $dir)
   
    if [[ "$@" =~ '--release' ]] ;then folder="release" ;else folder="debug" ;fi

    rustup target add wasm32-unknown-unknown && cargo install wasm-bindgen-cli && \
    cargo build --target wasm32-unknown-unknown $@ && \
        find target/wasm32-unknown-unknown/${folder}/examples -type f | fgrep .wasm | \
            while read wasm ;do
                wasm-strip ${wasm}
                cp ${wasm} docs
            done
}

function build_serve {
    cargo install basic-http-server && \
        basic-http-server --addr 127.0.0.1:8080 $@
}

function build_android {
    ## miniglue cargo apk build $@
    curl https://raw.githubusercontent.com/frgomes/bash-scripts/master/bin/miniglue -sSf | \
        sh -s -- cargo apk build $@
}

function build {
    case "$1" in
        linux)   shift; cargo build   $@ ;;
        android) shift; build_android $@ ;;
        web)     shift; build_wasm    $@ ;;
        serve)   shift; build_serve   $@ ;;
        *)              cargo build   $@ ;;
    esac
}

build $@
