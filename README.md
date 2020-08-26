# deno_plugin_num_cpus

[![Build Status](https://github.com/justjavac/deno_plugin_num_cpus/workflows/ci/badge.svg?branch=master)](https://github.com/justjavac/deno_plugin_num_cpus/actions)
[![tag](https://img.shields.io/github/release/justjavac/deno_plugin_num_cpus)](https://github.com/justjavac/deno_plugin_num_cpus/releases)
[![Crates.io](https://img.shields.io/crates/v/deno_plguin_starter)](https://crates.io/crates/deno_plguin_starter)
[![Docs.rs](https://docs.rs/deno_plguin_starter/badge.svg)](https://docs.rs/deno_plguin_starter)
[![license](https://img.shields.io/github/license/justjavac/deno_plugin_num_cpus)](https://github.com/justjavac/deno_plugin_num_cpus/blob/master/LICENSE)

Rust [num_cpus](https://crates.io/crates/num_cpus) binding for Deno.

Get the number of CPUs available on the current system.

Sometimes the CPU will exaggerate the number of CPUs it contains, because it can use [processor tricks](https://en.wikipedia.org/wiki/Simultaneous_multithreading) to deliver increased performance when there are more threads. This crate provides methods to get both the logical and physical numbers of cores.

This information can be used as a guide to how many tasks can be run in parallel. There are many properties of the system architecture that will affect parallelism, for example memory access speeds (for all the caches and RAM) and the physical architecture of the processor, so the number of CPUs should be used as a rough guide only.

## Ops

### `op_num_cpus`

Get the number of CPUs available on the current system.

Use in Deno:

```ts
const { op_num_cpus } = Deno.core.ops();
const response: Uint8Array = Deno.core.dispatch(op_num_cpus)!;
```

Returned Binary Layout:

```
+----------------+----------------+----------------+----------------+
|   NUM_CPUS (8) |                |                |                |
+----------------+----------------+----------------+----------------+
```

The number of cpu on each machine will not be greater than 256(2^8),
so we use 1 byte to pass the return value.

### License

[deno_plugin_num_cpus](https://github.com/justjavac/deno_plugin_num_cpus) is released under the MIT License. See the bundled [LICENSE](./LICENSE) file for details.
