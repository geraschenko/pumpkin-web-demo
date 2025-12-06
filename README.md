# pumpkin-web-demo

Minimal demo of using [pumpkin](https://github.com/ConSol-Lab/Pumpkin) constraint solver in WebAssembly. Uses the same example as [lmmx/pumpkin-web](https://github.com/lmmx/pumpkin-web).

[Try it live](https://geraschenko.github.io/pumpkin-web-demo)

## How it works

The current `pumpkin-solver` crate doesn't run in WASM because:

1. `pumpkin-solver` depends on `signal-hook` (Unix-only) to gracefully handle Ctrl+c in the CLI. **Solution:** use `pumpkin-core` instead.
2. `pumpkin-core` uses `std::time::Instant` which isn't available in WASM. **Solution:** patch to use `web-time` when building for wasm (done in patched `pumpkin-core`).
3. `getrandom` needs configuration for WASM. **Solution:** add `js` feature (done in patched `pumpkin-core`) and rustflag (in `.cargo/config.toml` here). If you're using Dioxus's `dx build`, the `.cargo/config.toml` is not needed — `dx` automatically injects this rustflag for wasm32 targets.

The [patched pumpkin-core](https://github.com/geraschenko/Pumpkin) is [Pumpkin/pull/327](https://github.com/ConSol-Lab/Pumpkin/pull/327).

## Building and running yourself.

1. Install wasm-pack (if you don't have it):
   ```bash
   cargo install wasm-pack
   ```

2. Build the WASM package:
   ```bash
   wasm-pack build --target web
   ```

3. Serve the files:
   ```bash
   python3 -m http.server 8000
   ```

4. Open http://localhost:8000 in your browser

## API

The demo exposes a single function:

```javascript
import init, { solve_sum } from './pkg/pumpkin_web_demo.js';

await init();

// Find x + y = target where x in [min_x, max_x] and y in [min_y, max_y]
const result = solve_sum(1, 10, 1, 10, 12);
// Returns: '{"x": 2, "y": 10}' or '{"error": "No solution exists"}'
```