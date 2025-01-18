# bysqr

Open source Pay by Square encoder written in Rust.

## Notice

Work on the project is still in progress. It is not suitable for a production run, until version 1.0, since not
all features are implemented. It is likely that there will be breaking changes in the future before settling on some stable API.

The goal of the project is to provide full encoder and decoder implementation for Pay by Square and Invoice by Square,
without any external dependencies and to be as much portable as possible, to allow compilation for various targets.

## Usage

You can use `bysqr` binary to encode and decode QR codes. Currently only Pay encoding is supported.

### Encoding to QR code

To encode `Pay` or `Invoice` to a QR code, you can run `encode` command with following arguments:

```shell
bysqr encode --src payment.xml --save ~/Desktop/qr.svg

bysqr encode --src '<?xml version="1.0"?><Pay type="Pay">...</Pay>' --save ~/Desktop/qr.svg
```

Provided source (`--src`) must be a valid Pay by Square or Invoice by Square XML structure. You can either pass a path to the XML file
or directly provide an XML content.

To save generated QR code as image, use `--save` option with path where to save the image. Type of the file is
determined by the output file extension. We support generating `svg`, `png` and `jpeg` images.

#### QR code preview

You may also preview generated code instead of saving, by passing a `--preview` option. This will open a window
where the QR code is displayed.

```shell
bysqr encode --src payment.xml --preview
```

#### Output to stdout

If you want to output content of the image directly to the standard output, you may use `--format` option instead of `--save`.
This will print content of the image to the stdout in requested format. If you specify an `svg` format, the XML of the SVG will be printed out.
Other formats such as `png` and `jpeg` are printed out as base64 encoded strings.

```shell
bysqr encode --src payment.xml --format svg # output: <svg xmlns="http://www.w3.org/2000/svg">...</svg>
bysqr encode --src payment.xml --format png # output: data:image/png;base64,...
bysqr encode --src payment.xml --format jpeg # output: data:image/jpeg;base64,...
```

#### Image size

When you request `png` or `jpeg` format, you may use the `--size` option to control the size of the output image. The size
option controls the width of the generated image. Height of the image is automatically calculated, since QR code with required logo outline
is a rectangle. The `svg` format ignores the size setting.

```shell
# This will create a png image with 1024px width
bysqr encode --src payment.xml --format png --size 1024
```

#### Image quality

When saving to a `jpeg` format, you may configure image encoder quality using `--quality` option. It must be a number from **1** to **100**.
The default quality is set to **90**.

```shell
bysqr encode --src payment.xml --format jpeg --quality 95
```

## Build

To build a project, ensure you have latest [Rust](https://www.rust-lang.org/tools/install) installed. Then, run build using `cargo`:

```shell
cargo build --release
```

You can find `bysqr` executable and rust library in `target/release`.

### WASM build

`bysqr` can be built for Web Assembly target, which allows you to run encoder and decoder in the browser, without need for a server.

Before building for `wasm` target, you need to install `wasm-pack`.

```shell
cargo install wasm-pack
```

After installing, you can start build:

```shell
wasm-pack build --target web
```

Built wasm module will be located in `pkg` folder.

#### macOS wasm build

Apple clang is not supported when building for wasm target and you have to instal `llvm` instead.

```shell
# Install llvm
brew install llvm

# Add llvm to $PATH, you may place it to .zshrc
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
export LDFLAGS="-L/opt/homebrew/opt/llvm/lib"
export CPPFLAGS="-I/opt/homebrew/opt/llvm/include"

# Verify installation
llvm-config --version
```

## Roadmap to v1.0

- [x] Pay encoder
- [ ] Pay decoder
- [ ] Invoice encoder
- [ ] Invoice decoder
- [ ] alternative JSON input and output structure
- [ ] theming
- [ ] support for different logo position
