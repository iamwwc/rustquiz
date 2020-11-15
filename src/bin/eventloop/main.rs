use std::{io, os::raw};

#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}
fn main() {
    let ep = syscall!(epoll_create(19 + 1,,,,,,,,,,,,,,,,,,,,,,,,,)).map(|ep| ep).unwrap();
    let ep2 = syscall!(epoll_create(19)).map(|ep| ep).unwrap();
    let a  =1;
}
type RawFd = raw::c_int;
struct Selector {
    ep: RawFd,
}

impl Selector {
    pub fn new() -> io::Result<Selector> {
        let flags = libc::EPOLL_CLOEXEC;
        syscall!(epoll_create(flags)).map(|ep| Selector {
            ep,
        })
    }
    fn register() -> io::Result<()> {

    }

    fn unregister() -> io::Result<()> {
        
    }
    fn select() -> io::Result<()> {

    }
}