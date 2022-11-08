mod handle;
mod ifacename;
pub mod ifreq;

use crate::Error;
pub use ifacename::InterfaceName;
use libc::{AF_INET, SOCK_DGRAM};

use std::{net, os::unix::prelude::RawFd};

pub(crate) mod ioctls;

pub(crate) fn dummy_socket() -> Result<RawFd, Error> {
    let fd = unsafe { libc::socket(AF_INET, SOCK_DGRAM, 0)};
    if fd < 0 {
        return Err(Error::Io(std::io::Error::last_os_error()));
    }
    Ok(fd)
}

pub(crate) fn list_interfaces() -> Result<Vec<crate::Interface>, Error> {
    Ok(nix::net::if_::if_nameindex()?
        .iter()
        .map(|a| crate::Interface::from_index_unchecked(a.index()))
        .collect())
}
