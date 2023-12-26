# Test Driven Development for Embedded Rust - Example

This repository hosts the project introduced in [this blog post.][link]
It contains a simple application which can be both unit tested and deployed on the target with no configuration changes needed.

## Dependencies

To build this program, you'll need:

- Flash and run/debug tools:
``` console
$ cargo install probe-rs --features cli
```

- `rust-std` components (pre-compiled `core` crate) for the STM32F446RE
  target. Run:
  
``` console
$ rustup target add thumbv7em-none-eabihf
```

## Building the project
To build the repository for the target, run:
``` console
$ cargo build --release --target=thumbv7em-none-eabihf
```

## Unit testing
To unit test the project, just simply run:
``` console
$ cargo test
```


## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Code of Conduct

Contribution to this crate is organized under the terms of the [Rust Code of
Conduct][CoC], the maintainer of this crate, promises
to intervene to uphold that code of conduct.

[link]: https://hackaday.io/page/21907-test-driven-embedded-rust-development-tutorial
[CoC]: https://www.rust-lang.org/policies/code-of-conduct
