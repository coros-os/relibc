//! termios implementation, following http://pubs.opengroup.org/onlinepubs/7908799/xsh/termios.h.html

use crate::{
    header::{errno, sys_ioctl},
    platform::{self, types::*},
};

pub type cc_t = u8;
pub type speed_t = u32;
pub type tcflag_t = u32;

pub const NCCS: usize = 32;

pub const VINTR: usize = 0;
pub const VQUIT: usize = 1;
pub const VERASE: usize = 2;
pub const VKILL: usize = 3;
pub const VEOF: usize = 4;
pub const VTIME: usize = 5;
pub const VMIN: usize = 6;
pub const VSWTC: usize = 7;
pub const VSTART: usize = 8;
pub const VSTOP: usize = 9;
pub const VSUSP: usize = 10;
pub const VEOL: usize = 11;
pub const VREPRINT: usize = 12;
pub const VDISCARD: usize = 13;
pub const VWERASE: usize = 14;
pub const VLNEXT: usize = 15;
pub const VEOL2: usize = 16;

pub const IGNBRK: usize = 0o000_001;
pub const BRKINT: usize = 0o000_002;
pub const IGNPAR: usize = 0o000_004;
pub const PARMRK: usize = 0o000_010;
pub const INPCK: usize = 0o000_020;
pub const ISTRIP: usize = 0o000_040;
pub const INLCR: usize = 0o000_100;
pub const IGNCR: usize = 0o000_200;
pub const ICRNL: usize = 0o000_400;
pub const IUCLC: usize = 0o001_000;
pub const IXON: usize = 0o002_000;
pub const IXANY: usize = 0o004_000;
pub const IXOFF: usize = 0o010_000;
pub const IMAXBEL: usize = 0o020_000;
pub const IUTF8: usize = 0o040_000;

pub const OPOST: usize = 0o000_001;
pub const OLCUC: usize = 0o000_002;
pub const ONLCR: usize = 0o000_004;
pub const OCRNL: usize = 0o000_010;
pub const ONOCR: usize = 0o000_020;
pub const ONLRET: usize = 0o00_0040;
pub const OFILL: usize = 0o000_100;
pub const OFDEL: usize = 0o000_200;

pub const VTDLY: usize = 0o040_000;
pub const VT0: usize = 0o000_000;
pub const VT1: usize = 0o040_000;

pub const B0: usize = 0o000_000;
pub const B50: usize = 0o000_001;
pub const B75: usize = 0o000_002;
pub const B110: usize = 0o000_003;
pub const B134: usize = 0o000_004;
pub const B150: usize = 0o000_005;
pub const B200: usize = 0o000_006;
pub const B300: usize = 0o000_007;
pub const B600: usize = 0o000_010;
pub const B1200: usize = 0o000_011;
pub const B1800: usize = 0o000_012;
pub const B2400: usize = 0o000_013;
pub const B4800: usize = 0o000_014;
pub const B9600: usize = 0o000_015;
pub const B19200: usize = 0o000_016;
pub const B38400: usize = 0o000_017;

pub const B57600: usize = 0o010_001;
pub const B115200: usize = 0o010_002;
pub const B230400: usize = 0o010_003;
pub const B460800: usize = 0o010_004;
pub const B500000: usize = 0o010_005;
pub const B576000: usize = 0o010_006;
pub const B921600: usize = 0o010_007;
pub const B1000000: usize = 0o010_010;
pub const B1152000: usize = 0o010_011;
pub const B1500000: usize = 0o010_012;
pub const B2000000: usize = 0o010_013;
pub const B2500000: usize = 0o010_014;
pub const B3000000: usize = 0o010_015;
pub const B3500000: usize = 0o010_016;
pub const B4000000: usize = 0o010_017;

pub const CSIZE: usize = 0o000_060;
pub const CS5: usize = 0o000_000;
pub const CS6: usize = 0o000_020;
pub const CS7: usize = 0o000_040;
pub const CS8: usize = 0o000_060;
pub const CSTOPB: usize = 0o000_100;
pub const CREAD: usize = 0o000_200;
pub const PARENB: usize = 0o000_400;
pub const PARODD: usize = 0o001_000;
pub const HUPCL: usize = 0o002_000;
pub const CLOCAL: usize = 0o004_000;

pub const ISIG: usize = 0o000_001;
pub const ICANON: usize = 0o000_002;
pub const ECHO: usize = 0o000_010;
pub const ECHOE: usize = 0o000_020;
pub const ECHOK: usize = 0o000_040;
pub const ECHONL: usize = 0o000_100;
pub const NOFLSH: usize = 0o000_200;
pub const TOSTOP: usize = 0o000_400;
pub const IEXTEN: usize = 0o100_000;

pub const TCOOFF: usize = 0;
pub const TCOON: usize = 1;
pub const TCIOFF: usize = 2;
pub const TCION: usize = 3;

pub const TCIFLUSH: usize = 0;
pub const TCOFLUSH: usize = 1;
pub const TCIOFLUSH: usize = 2;

pub const TCSANOW: usize = 0;
pub const TCSADRAIN: usize = 1;
pub const TCSAFLUSH: usize = 2;

#[repr(C)]
#[derive(Default)]
pub struct termios {
    c_iflag: tcflag_t,
    c_oflag: tcflag_t,
    c_cflag: tcflag_t,
    c_lflag: tcflag_t,
    c_line: cc_t,
    c_cc: [cc_t; NCCS],
    __c_ispeed: speed_t,
    __c_ospeed: speed_t,
}

#[no_mangle]
pub unsafe extern "C" fn tcgetattr(fd: c_int, out: *mut termios) -> c_int {
    sys_ioctl::ioctl(fd, sys_ioctl::TCGETS, out as *mut c_void)
}

#[no_mangle]
pub unsafe extern "C" fn tcsetattr(fd: c_int, act: c_int, value: *mut termios) -> c_int {
    if act < 0 || act > 2 {
        platform::errno = errno::EINVAL;
        return -1;
    }
    // This is safe because ioctl shouldn't modify the value
    sys_ioctl::ioctl(fd, sys_ioctl::TCSETS + act as c_ulong, value as *mut c_void)
}

#[no_mangle]
pub unsafe extern "C" fn cfgetispeed(termios_p: *const termios) -> speed_t {
    (*termios_p).__c_ispeed
}

#[no_mangle]
pub unsafe extern "C" fn cfgetospeed(termios_p: *const termios) -> speed_t {
    (*termios_p).__c_ospeed
}

#[no_mangle]
pub unsafe extern "C" fn cfsetispeed(termios_p: *mut termios, speed: speed_t) -> c_int {
    match speed as usize {
        B0..=B38400 | B57600..=B4000000 => {
            (*termios_p).__c_ispeed = speed;
            0
        }
        _ => {
            platform::errno = errno::EINVAL;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn cfsetospeed(termios_p: *mut termios, speed: speed_t) -> c_int {
    match speed as usize {
        B0..=B38400 | B57600..=B4000000 => {
            (*termios_p).__c_ospeed = speed;
            0
        }
        _ => {
            platform::errno = errno::EINVAL;
            -1
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn tcflush(fd: c_int, queue: c_int) -> c_int {
    sys_ioctl::ioctl(fd, sys_ioctl::TCFLSH, queue as *mut c_void)
}

#[no_mangle]
pub unsafe extern "C" fn tcdrain(fd: c_int) -> c_int {
    sys_ioctl::ioctl(fd, sys_ioctl::TCSBRK, 1 as *mut _)
}

#[no_mangle]
pub unsafe extern "C" fn tcsendbreak(fd: c_int, _dur: c_int) -> c_int {
    // non-zero duration is ignored by musl due to it being
    // implementation-defined. we do the same.
    sys_ioctl::ioctl(fd, sys_ioctl::TCSBRK, 0 as *mut _)
}

#[no_mangle]
pub unsafe extern "C" fn tcflow(fd: c_int, action: c_int) -> c_int {
    // non-zero duration is ignored by musl due to it being
    // implementation-defined. we do the same.
    sys_ioctl::ioctl(fd, sys_ioctl::TCXONC, action as *mut _)
}
