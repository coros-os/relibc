use crate::{header::sys_socket::sa_family_t, platform::types::*};

#[repr(C)]
pub struct sockaddr_un {
    sun_family: sa_family_t,
    sun_path: [c_char; 108],
}
