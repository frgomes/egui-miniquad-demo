# egui-miniquad-demo

A demo application based on [egui-miniquad](https://github.com/not-fl3/egui-miniquad), for Linux, MacOS, Windows, WASM, Android and iOS.

See a demo online here: [https://frgomes.github.io/egui-miniquad-demo/docs/](https://frgomes.github.io/egui-miniquad-demo/docs/)

## Building for Linux

The main program `demo` contains the executable code for Linux and other platforms.

``` bash
./cargo.sh linux --example demo
```

or just the usual:

``` bash
cargo run --example demo
```

## Building for the web

The main program `demo` contains the executable code for [WASM](https://en.wikipedia.org/wiki/WebAssembly) and other platforms.

> You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.


``` bash
#!/bin/bash
./build.sh web --example demo
./build.sh serve docs
firefox http://127.0.0.1:8080/
```

The finished web app is found in the `docs/` folder. You can easily share it with [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).


## Building for Android

The main program `android` contains the activity to be executed by Android.

> NOTE: The Android build is done in a container which is automagically created for you, if necessary. For more information, please visit: https://github.com/frgomes/bash-scripts/blob/master/bin/miniglue

``` bash
#!/bin/bash
./build.sh android --target aarch64-linux-android --example android
adb install ./target/debug/apk/examples/demo.apk
```
