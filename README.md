# Layer Painter

## Introduction

A simple app for drawing images of multiple layers on an html canvas element, built with yew & tauri.

It's an exercise work for myself to learn rust, yew and wasm.

The app maybe will not be upgraded in the future.

### wasm

Using wasm instead of javascript to create a SPA.

### css

Tailwind css.

### desktop app

Smaller, no need to pack a chromium core.

Click to run, no need to download Node.js runtime.

### Dev & Build

#### Set up

First, you need to check the enviroments of [tauri](https://tauri.studio/docs/getting-started/prerequisites) & [yew](https://yew.rs/docs/getting-started/introduction).

Don't forget `tauri-cli`.

#### Dev

In `root` dir,

```
cargo tauri dev
```

For tailwind css, in `wasm` dir,

```
tailwindcss -i index.css -o build.css --watch
```

#### Build Release

In `root` dir,

```
cargo tauri build
```

After building, the installer will be in `./src-tauri/release/bundle`.

### Problems

#### Platforms

I devlopped wasm part on Macbook Pro m1, and it worked well.

However, on MacOS,  `tauri` / `wry` is not surpport html `input` element to upload file yet. Here is the [issue](https://github.com/tauri-apps/wry/issues/305).

I didn't test on Linux, but I think it would work.

### Thanks
