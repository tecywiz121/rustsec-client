# rustsec client [![Latest Version][crate-image]][crate-link] [![Docs][docs-image]][docs-link] [![Build Status][build-image]][build-link] ![MIT/Apache 2 licensed][license-image]

[crate-image]: https://img.shields.io/crates/v/rustsec.svg
[crate-link]: https://crates.io/crates/rustsec
[docs-image]: https://docs.rs/rustsec/badge.svg
[docs-link]: https://docs.rs/rustsec/
[build-image]: https://travis-ci.org/RustSec/rustsec-client.svg?branch=master
[build-link]: https://travis-ci.org/RustSec/rustsec-client
[license-image]: https://img.shields.io/badge/license-MIT%2FApache2-blue.svg

Client library for accessing the RustSec security advisory database

## About

[Documentation]

The `rustsec` crate is primarily intended to be used by the [cargo-audit] crate for
the purposes of identifying vulnerable crates in Cargo.lock files.

However, it may be useful if you would like to consume the RustSec advisory
database in other capacities.

[Documentation]: https://docs.rs/rustsec/
[cargo-audit]: https://github.com/rustsec/cargo-audit

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE] or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or https://opensource.org/licenses/MIT)

at your option.

[LICENSE-APACHE]: https://github.com/RustSec/rustsec-client/blob/master/LICENSE-APACHE
[LICENSE-MIT]: https://github.com/RustSec/rustsec-client/blob/master/LICENSE-MIT

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.
