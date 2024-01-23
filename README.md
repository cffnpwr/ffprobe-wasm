# FFprobe WASM

## Installation

### Deno

```typescript
import { getInfo, libavformatVersion, libavcodecVersion, libavutilVersion } from "https://deno.land/x/ffprobe_wasm@<version>/mod.ts"; 
```

### npm

```typescript
import { getInfo, libavformatVersion, libavcodecVersion, libavutilVersion } from "@cffnpwr/ffprobe-wasm";
```

## Usage

```typescript
const info = await getInfo("URL");
```