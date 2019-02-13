# in_addr

This crate exposes a common interface to in_addr between Unix and Windows. Get rid of these `#[cfg]`s!

## Features

- **`no-std`**: Compile this crate with `no_std`. Conversions to and from `std::net::Ipv4Addr` will not be available.

## Documentation

- **`cfg(windows)`**: https://animalsiknow.github.io/doc/in_addr
- **`cfg(unix)`**: https://docs.rs/in_addr/*/in_addr
