# @affectively/wasm-image-utils

High-performance WebAssembly image utilities written in Rust.

[![npm](https://img.shields.io/npm/v/@affectively/wasm-image-utils.svg)](https://www.npmjs.com/package/@affectively/wasm-image-utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Base64 Encoding/Decoding** - Fast Base64 operations for images
- **Format Detection** - Detect image format from magic bytes
- **Validation** - Validate image data integrity
- **Dimension Extraction** - Get width/height from image headers

## Installation

```bash
npm install @affectively/wasm-image-utils
```

## Quick Start

```typescript
import init, { encode_base64, decode_base64, detect_format } from '@affectively/wasm-image-utils';

await init();

const base64 = encode_base64(imageBytes);
const bytes = decode_base64(base64String);
const format = detect_format(imageBytes); // "png", "jpeg", "gif", etc.
```

## License

MIT License

---

Made with ❤️ by [AFFECTIVELY](https://affectively.app)
