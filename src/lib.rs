//! Get the number of CPUs available on the current system.
//!
//! Sometimes the CPU will exaggerate the number of CPUs it contains, because it
//! can use [processor tricks](https://en.wikipedia.org/wiki/Simultaneous_multithreading)
//! to deliver increased performance when there are more threads.
//! This crate provides methods to get both the logical and physical numbers of cores.
//!
//! This information can be used as a guide to how many tasks can be run in parallel.
//! There are many properties of the system architecture that will affect parallelism,
//! for example memory access speeds (for all the caches and RAM) and the physical
//! architecture of the processor, so the number of CPUs should be used as a rough guide only.

extern crate deno_core;
extern crate num_cpus;

use deno_core::plugin_api::{Buf, Interface, Op, ZeroCopyBuf};

/// Initializing the Deno plugin.
#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
  interface.register_op("op_num_cpus", op_num_cpus);
}

/// Get the number of CPUs available on the current system.
///
/// Use in Deno:
///
/// ```ts
/// const { op_num_cpus } = Deno.core.ops();
/// const response: Uint8Array = Deno.core.dispatch(op_num_cpus)!;
/// ```
///
/// Returned Binary Layout:
///
/// ```
/// +----------------+----------------+----------------+----------------+
/// |   NUM_CPUS (8) |                |                |                |
/// +----------------+----------------+----------------+----------------+
/// ```
/// 
/// The number of cpu on each machine will not be greater than 256(2^8),
/// so we use 1 byte to pass the return value.
pub fn op_num_cpus(
  _interface: &mut dyn Interface,
  _zero_copy: &mut [ZeroCopyBuf],
) -> Op {
  let nums = num_cpus::get() as u8;
  let result: Buf = Box::new([nums]);
  Op::Sync(result)
}
