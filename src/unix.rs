use libc;

/// Type alias to the Unix version of `in_addr`.
#[allow(non_camel_case_types)]
pub type in_addr = libc::in_addr;

/// Wrapper around `in_addr` on which `From` is implemented.
#[derive(Copy, Clone)]
pub struct InAddr(pub(crate) in_addr);

impl From<u32> for InAddr {
  fn from(value: u32) -> Self {
    InAddr(in_addr { s_addr: value })
  }
}

impl From<InAddr> for u32 {
  fn from(InAddr(addr): InAddr) -> u32 {
    addr.s_addr
  }
}
