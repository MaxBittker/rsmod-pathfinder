# rsmod-pathfinder

A breadth-first search pathfinder.
(The fastest in all the land)

Ported from Kotlin: 🔗 https://github.com/rsmod/rsmod/tree/main/engine/pathfinder

----

## ✨ Features

- **`findPath`** - Standard pathfinding with 128x128 search grid (up to 64 tiles in any direction)
- **`findLongPath`** - Long-distance pathfinding with 512x512 search grid (up to 256 tiles in any direction)

Both functions share the same API. Use `findLongPath` when your destination may be more than 64 tiles away from the source.

```typescript
import * as rsmod from '@2004scape/rsmod-pathfinder';

// Standard pathfinding (max ~64 tiles)
const path = rsmod.findPath(
    level,           // y/level (0-3)
    srcX, srcZ,      // source coordinates
    destX, destZ,    // destination coordinates
    srcSize,         // source entity size (1, 2, 3+)
    destWidth, destHeight,  // destination dimensions
    angle,           // object angle (0-3)
    shape,           // object shape (-1 for none)
    moveNear,        // allow partial path to nearest point
    blockAccessFlags,// block access restrictions
    maxWaypoints,    // max waypoints to return
    CollisionType.NORMAL
);

// Long-distance pathfinding (max ~256 tiles)
const longPath = rsmod.findLongPath(
    level, srcX, srcZ, destX, destZ,
    srcSize, destWidth, destHeight,
    angle, shape, moveNear, blockAccessFlags,
    maxWaypoints, CollisionType.NORMAL
);
```

----

## ✨ Installing

> 🔗 https://www.npmjs.com/package/@2004scape/rsmod-pathfinder
>
> `npm i @2004scape/rsmod-pathfinder`

> 🔗 https://crates.io/crates/rsmod-pathfinder
>
> `cargo add rsmod-pathfinder`

----

## 💪 Benchmarks

I have created `main.js` and `main.rs` for synthetic examples of performance.
They load in a full reconstruction of the Lumbridge mapsquare (64x64) tiles
with full clipping flags set. Then in a loop it runs through 100k pathfinder
requests to a destination +10 tiles North with a single access point to the destination.

--

AMD Ryzen 9 3900X 12-Core Processor 3.80 GHz

- All tests below were done on the same machine with the same hardware.

### Rust (Native):

Ran in Release mode on Windows x64 OS.

```
100k paths took: 279.3573ms; time per call: 2.793µs
100k paths took: 280.2238ms; time per call: 2.802µs
100k paths took: 281.206ms; time per call: 2.812µs
100k paths took: 278.8223ms; time per call: 2.788µs
100k paths took: 277.9747ms; time per call: 2.779µs
100k paths took: 279.5909ms; time per call: 2.795µs
100k paths took: 277.8473ms; time per call: 2.778µs
100k paths took: 281.0326ms; time per call: 2.81µs
```

### Rust (WebAssembly):

Ran in Release mode on Windows x64 OS with NodeJS.

This typically takes about 850ms on average in a real world scenario tested
on a server also using TypeScript.

```
100k paths took: 703ms; time per call: 7.03µs
100k paths took: 702ms; time per call: 7.02µs
100k paths took: 701ms; time per call: 7.01µs
100k paths took: 702ms; time per call: 7.02µs
100k paths took: 699ms; time per call: 6.99µs
100k paths took: 705ms; time per call: 7.05µs
100k paths took: 701ms; time per call: 7.01µs
100k paths took: 702ms; time per call: 7.02µs
```

### AssemblyScript (WebAssembly):

```
100k paths took: 1534ms; time per call: 15.34µs
100k paths took: 1540ms; time per call: 15.40µs
100k paths took: 1544ms; time per call: 15.44µs
100k paths took: 1519ms; time per call: 15.19µs
100k paths took: 1533ms; time per call: 15.33µs
100k paths took: 1522ms; time per call: 15.22µs
100k paths took: 1527ms; time per call: 15.27µs
```

### TypeScript:

```
100k paths took: 3858ms; time per call: 38.58µs
100k paths took: 3855ms; time per call: 38.55µs
100k paths took: 3828ms; time per call: 38.28µs
100k paths took: 3820ms; time per call: 38.20µs
100k paths took: 3833ms; time per call: 38.33µs
100k paths took: 3852ms; time per call: 38.52µs
100k paths took: 3844ms; time per call: 38.44µs
```

--

`cargo bench`

```
pathfinder/find_path_short_128x128
                        time:   [4.58 µs 4.66 µs 4.73 µs]
                        thrpt:  [211K elem/s 214K elem/s 218K elem/s]

long_pathfinder/find_path_short_512x512
                        time:   [29.5 µs 30.1 µs 30.8 µs]
                        thrpt:  [32.5K elem/s 33.2K elem/s 33.9K elem/s]

long_pathfinder/find_path_medium_512x512
                        time:   [60.4 µs 62.2 µs 64.6 µs]
                        thrpt:  [15.5K elem/s 16.1K elem/s 16.6K elem/s]

long_pathfinder/find_path_long_512x512
                        time:   [56.2 µs 56.9 µs 57.6 µs]
                        thrpt:  [17.4K elem/s 17.6K elem/s 17.8K elem/s]
```

### Performance Summary

| Function | Grid Size | Path Distance | Time per Call | Memory |
|----------|-----------|---------------|---------------|--------|
| `findPath` | 128x128 | 10 tiles | ~4.7 µs | ~200 KB |
| `findLongPath` | 512x512 | 10 tiles | ~30 µs | ~3.2 MB |
| `findLongPath` | 512x512 | 40 tiles | ~62 µs | ~3.2 MB |
| `findLongPath` | 512x512 | 90 tiles | ~57 µs | ~3.2 MB |

For bot applications with 100-500 concurrent pathfinding requests, `findLongPath` adds only 6-30ms per tick.

----

## Contributor Notes

```shell
wasm-pack build --target nodejs --out-dir dist --out-name rsmod-pathfinder # Builds the wasm bundle.
~/IdeaProjects/wabt/bin/wasm2wat  dist/rsmod-pathfinder_bg.wasm -o src/rsmod.wat # Generates the wat file.
npm publish --access public # Publishes the version of this to npm.
```