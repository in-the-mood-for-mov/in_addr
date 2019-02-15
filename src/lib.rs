//! This crate exposes a common interface to in_addr between Unix and Windows.
//!
//! There are two types in this crate.
//! * `in_addr`, a type alias to the platform specific version of this type. Use this in the
//!   signature of `extern` functions.
//! * `InAddr`, a newtype wrapper around `in_addr`. It implements conversions to and from `u32`,
//!   `std::net::Ipv4Addr`, and `in_addr`.
//!
//! # Example
//!
//! ```
//! extern {
//!   fn inet_ntoa(addr: *const in_addr::in_addr) -> *const std::os::raw::c_char;
//! }
//!
//! fn main() {
//!   let addr = in_addr::InAddr::new(std::net::Ipv4Addr::LOCALHOST);
//!   let addr_text = unsafe { std::ffi::CStr::from_ptr(inet_ntoa(&addr.into())) };
//!   println!("The address is {}.", addr_text.to_string_lossy());
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub use crate::unix::*;

#[cfg(windows)]
pub use crate::windows::*;

impl InAddr {
  pub fn new<T: Into<Self>>(t: T) -> Self {
    t.into()
  }
}

impl From<in_addr> for InAddr {
  fn from(in_addr: in_addr) -> Self {
    InAddr(in_addr)
  }
}

impl From<InAddr> for in_addr {
  fn from(InAddr(addr): InAddr) -> Self {
    addr
  }
}

#[cfg(feature = "std")]
impl std::fmt::Debug for InAddr {
  fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
    Into::<std::net::Ipv4Addr>::into(*self).fmt(formatter)
  }
}

#[cfg(feature = "std")]
impl From<std::net::Ipv4Addr> for InAddr {
  fn from(addr: std::net::Ipv4Addr) -> Self {
    Into::<u32>::into(addr).into()
  }
}

#[cfg(feature = "std")]
impl From<InAddr> for std::net::Ipv4Addr {
  fn from(addr: InAddr) -> Self {
    Into::<u32>::into(addr).into()
  }
}
