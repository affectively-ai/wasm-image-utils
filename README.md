# @affectively/wasm-image-utils

`@affectively/wasm-image-utils` is a Rust/WebAssembly module for lightweight image utility work.

The fair brag is its simplicity. This package gives you a few useful image-adjacent operations without asking you to bring in a much larger image-processing dependency.

## What It Helps You Do

- encode and decode base64
- detect image formats from bytes
- validate image payloads
- read dimensions from headers

## Installation

```bash
npm install @affectively/wasm-image-utils
```

## Quick Start

```ts
import init, {
  encode_base64,
  decode_base64,
  detect_format,
} from '@affectively/wasm-image-utils';

await init();

const base64 = encode_base64(imageBytes);
const bytes = decode_base64(base64String);
const format = detect_format(imageBytes);
```

## Why This README Is Grounded

Image Utils does not need to be more than a helper package. The strongest fair brag is that it already gives you a compact WASM layer for a few common image operations.
