# sntp-rs

> A Simple Network Time Protocol (SNTP) client implementation using [`smoltcp`].

This crate has been developed with `#![no_std]` use in mind: it doesn't perform
heap allocations, and does not rely on the `alloc` feature of `smoltcp`.

[`smoltcp`]: https://github.com/smoltcp-rs/smoltcp

## Requirements

- Rust 1.43+

## Examples

See the [examples] directory for an example on how to use this crate in a hosted Linux environment. For bare-metal examples, refer to the documentation and the [loopback example] of smoltcp.

[examples]: examples/
[loopback example]: https://github.com/smoltcp-rs/smoltcp/blob/master/examples/loopback.rs

## Features

The following features can be enabled at the crate level:

### `log`

Enable logging for network activity. Useful to debug the client operation. Disabled by default.

## License

Copyright © 2020 Pietro Lorefice

Dual licensed under your choice of either of:

- Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
