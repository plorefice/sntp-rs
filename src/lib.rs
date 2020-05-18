/*! A Simple Network Time Protocol (SNTP) client implementation using [`smoltcp`].

This crate has been developed with `#![no_std]` use in mind: it doesn't perform
heap allocations, and does not rely on the `alloc` feature of `smoltcp`.

For convenience, this crate re-exports `smoltcp` under the `net` name.

# Examples

An example on how to use this crate can be found in the source repository.

# Features

The following features can be enabled at the crate level:

## `log`

Enable logging for network activity. Useful to debug the client operation.

Disabled by default

[`smoltcp`]: https://github.com/smoltcp-rs/smoltcp
*/

#![deny(warnings)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![no_std]

#[cfg(any(test, feature = "std"))]
#[macro_use]
extern crate std;

#[cfg(feature = "log")]
#[macro_use(trace, debug)]
extern crate log;

// Re-export smoltcp
pub use smoltcp as net;

#[macro_use]
mod macros;
mod client;
mod wire;

// Export public types
pub use client::Client;
