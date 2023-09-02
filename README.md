# Tauri and Linux
<a id="markdown-tauri-and-linux" name="tauri-and-linux"></a>

<!-- TOC -->

- [Prerequisites for Linux](#prerequisites-for-linux)
  - [Basics](#basics)
  - [WebKit driver](#webkit-driver)
  - [libsoup](#libsoup)
  - [gtk-rs](#gtk-rs)
  - [Tauri CLI](#tauri-cli)
- [Run](#run)

<!-- /TOC -->

## Prerequisites for Linux
<a id="markdown-prerequisites-for-linux" name="prerequisites-for-linux"></a>

### 1. Basics
<a id="markdown-basics" name="basics"></a>

Install the following from
[docs](https://tauri.app/v1/guides/getting-started/prerequisites):

```shell
sudo apt update
sudo apt install libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

### 2. gtk-rs
<a id="markdown-gtk-rs" name="gtk-rs"></a>

Also need `gtk-rs` from
[docs](https://gtk-rs.org/gtk4-rs/stable/latest/book/installation_linux.html) to remove
this warning `  error: could not find system library 'webkit2gtk-4.1' required by the
'webkit2gtk-sys' crate`.

```shell
sudo apt-get install -y libwebkit2gtk-4.1-dev build-essential
```

More info here: [blog post](https://tauri.app/blog/2023/02/03/tauri-2-0-0-alpha-3/#__docusaurus_skipToContent_fallback).

### 3. xapp

Also need `xapp` from [docs](
https://askubuntu.com/questions/1396739/failed-to-load-module-xapp-gtk3-module) to remove
this warning `Failed to load module "xapp-gtk3-module"`.

```shell
sudo apt install xapp
```

### 4. WebKit driver
<a id="markdown-webkit-driver" name="webkit-driver"></a>

Also need `WebKitWebDriver` binary from
[docs](https://tauri.app/v1/guides/testing/webdriver/introduction/#linux).

```shell
sudo apt install webkit2gtk-driver
```

### 5. libsoup
<a id="markdown-libsoup" name="libsoup"></a>

Also need `libsoup` from
[here](https://installati.one/install-libsoup-3.0-common-ubuntu-22-04/?expand_article=1)
to remove this warning `error: could not find system library 'libsoup-3.0' required by
the 'soup3-sys' crate`.

```shell
sudo apt -y install libsoup-3.0-common
```

More info here: [blog post](https://tauri.app/blog/2023/02/03/tauri-2-0-0-alpha-3/#__docusaurus_skipToContent_fallback).

### 6. Tauri CLI
<a id="markdown-tauri-cli" name="tauri-cli"></a>

Also install the Tauri CLI dev tools

- Via `npm`:
  ```shell
  npm install --save-dev @tauri-apps/cli
  ```

- Via `cargo`:
  ```shell
  cargo install tauri-cli --locked --version 2.0.0-alpha.11
  ```

## Run
<a id="markdown-run" name="run"></a>

Run with `npx`:

```shell
cd src-tauri
npx tauri dev
```

Run with `cargo`.

```shell
cd src-tauri
cargo tauri dev
```

## Docs and References

- [System Tray](https://tauri.app/v1/guides/features/system-tray/#updating-tray-icon).
- [HTML, CSS, JS](https://tauri.app/v1/guides/getting-started/setup/html-css-js/)
- [Next.js](https://tauri.app/v1/guides/getting-started/setup/next-js/)
- [Calling Rust from frontend](https://tauri.app/v1/guides/features/command)
