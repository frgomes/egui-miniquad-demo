# egui-miniquad-demo

A demo application based on [egui-miniquad](https://github.com/not-fl3/egui-miniquad), for Linux, MacOS, Windows, WASM, Android and iOS.

See a demo online here: [https://frgomes.github.io/egui-miniquad-demo/docs/](https://frgomes.github.io/egui-miniquad-demo/docs/)

## Building for Linux

The main program `demo` contains the executable code for Linux and other platforms.

``` bash
#!/bin/bash
cargo run --example demo
```

## Building for the web

The main program `demo` contains the executable code for [WASM](https://en.wikipedia.org/wiki/WebAssembly) and other platforms.

> You can compile your app to [WASM](https://en.wikipedia.org/wiki/WebAssembly) and publish it as a web page.

``` bash
#!/bin/bash
./setup_web.sh && ./build_web.sh && ./start_server.sh &
firefox http://127.0.0.1:8080/
```
The finished web app is found in the `docs/` folder. You can easily share it with [GitHub Pages](https://docs.github.com/en/free-pro-team@latest/github/working-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site).


## Building for Android

The main program `android` contains the activity to be executed by Android.

### Starting the emulator

Run the emulation on a separate terminal.

``` bash
#!/bin/bash
emulator -avd Nexus6P_android-29
```

### Running the application in the emulator.

``` bash
#!/bin/bash
cargo apk run --example android
```
