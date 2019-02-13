use core::mem;
use winapi::shared::inaddr;

/// Type alias to the Windows version of `in_addr`.
#[allow(non_camel_case_types)]
pub type in_addr = inaddr::in_addr;

/// Wrapper around `in_addr` on which `From` is implemented.
#[derive(Copy, Clone)]
pub struct InAddr(pub(crate) in_addr);

impl From<u32> for InAddr {
  fn from(value: u32) -> Self {
    let un = unsafe {
      let mut un = mem::zeroed::<inaddr::in_addr_S_un>();
      *un.S_addr_mut() = value;
      un
    };
    InAddr(inaddr::in_addr { S_un: un })
  }
}

impl From<InAddr> for u32 {
  fn from(InAddr(addr): InAddr) -> u32 {
    unsafe { *addr.S_un.S_addr() }
  }
}
