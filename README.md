# 🖼️ rav1e AVIF encoder

An [AVIF](https://netflixtechblog.com/avif-for-next-generation-image-coding-b1d75675fe4) encoder using [rav1e](https://github.com/xiph/rav1e) written in WebAssembly.

> 💁 This is an experimental repository. My other WebAssembly-related projects are collected in the [saschazar21/webassembly](https://github.com/saschazar21/webassembly) repository.

## Prerequisites

To build the WebAssembly package, the following prerequisites are needed (either as a native install, or using the included Dockerfile):

### Native

- A Rust setup, preferably using [rustup](https://rustup.rs/)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) for compiling and creating a WebAssembly package

### Docker

Everything listed above is already included in the [Dockerfile](./Dockerfile). To build a Docker image using the Dockerfile, run the following command:

```bash
$ docker build -t <yourname>:rust .
```

To compile the WebAssembly package, mount the repository in the Docker container:

```bash
$ docker run --rm -it --name rust -v `pwd`:/app <yourname>:rust bash
```

This will create a new Docker container, mount the current working directory at the `/app` path and start a `bash` session.

## Build

To build this project as a WebAssembly package, execute the following command:

```bash
$ wasm-pack build
```

> Hint: append `--target nodejs`, if you want to compile it for the Node.js environment, instead of the browser. More information is available in the [wasm-pack docs](https://rustwasm.github.io/docs/wasm-pack/).

## Usage

The package exposes only one function – `encode()` for encoding raw RGB(A) pixel data in a `Uint8Array` to AVIF:

```javascript
// Node.js example

// import encode function from './pkg'
const { encode } = require('./pkg');

// default options
const options = {
  width: 640,         // mandatory property
  height: 480,        // mandatory property
  quality: 75,        // 0 (worst) - 100 (best)
  alpha_quality: 25,  // 0 (worst) - 100 (best)
  speed: 6,           // 0 (slowest) - 10 (fastest)
  premultiplied_alpha: false,
  color_space: 0,     // 0 (YCbCr), 1 (RGB)
  chroma: 0,          // 0 (4:4:4) - only 4:4:4 works atm...
};

const raw_pixels = new Uint8Array([...]);

const encoded = encode(raw_pixels, options);
```

## Issues

1. **Speed**: the encoding process is single-threaded and dead slooow (I mean, really, really slow) - encoding a single 3072\*2048 pixel image using the default options above takes ~4 min!

1. **Chroma Subsampling**: Other Chroma subsamplings than `4:4:4` currently return a corrupted image. Haven't found any clue yet.

## License

Licensed under the MIT license.

Copyright ©️ 2021 Sascha Zarhuber
