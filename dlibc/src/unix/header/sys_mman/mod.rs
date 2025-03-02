use crate::unix::{
    c_str::{CStr, CString},
    header::fcntl,
};



pub const MADV_NORMAL: ::c_int = 0;
pub const MADV_RANDOM: ::c_int = 1;
pub const MADV_SEQUENTIAL: ::c_int = 2;
pub const MADV_WILLNEED: ::c_int = 3;
pub const MADV_DONTNEED: ::c_int = 4;

pub const MAP_SHARED: ::c_int = 0x0001;
pub const MAP_PRIVATE: ::c_int = 0x0002;
pub const MAP_TYPE: ::c_int = 0x000F;
pub const MAP_ANON: ::c_int = 0x0020;
pub const MAP_ANONYMOUS: ::c_int = MAP_ANON;

pub const MS_ASYNC: ::c_int = 0x0001;
pub const MS_INVALIDATE: ::c_int = 0x0002;
pub const MS_SYNC: ::c_int = 0x0004;

pub const MCL_CURRENT: ::c_int = 1;
pub const MCL_FUTURE: ::c_int = 2;

pub const POSIX_MADV_NORMAL: ::c_int = 0;
pub const POSIX_MADV_RANDOM: ::c_int = 1;
pub const POSIX_MADV_SEQUENTIAL: ::c_int = 2;
pub const POSIX_MADV_WILLNEED: ::c_int = 3;
pub const POSIX_MADV_WONTNEED: ::c_int = 4;

// #[no_mangle]
// pub unsafe extern "C" fn mlock(addr: *const ::c_void, len: usize) -> ::c_int {
//     platform::pal::mlock(addr, len)
// }

// #[no_mangle]
// pub extern "C" fn mlockall(flags: ::c_int) -> ::c_int {
//     platform::pal::mlockall(flags)
// }

// #[no_mangle]
// pub unsafe extern "C" fn mmap(
//     addr: *mut ::c_void,
//     len: ::size_t,
//     prot: ::c_int,
//     flags: ::c_int,
//     fildes: ::c_int,
//     off: ::off_t,
// ) -> *mut ::c_void {
//     platform::pal::mmap(addr, len, prot, flags, fildes, off)
// }

// #[no_mangle]
// pub unsafe extern "C" fn mprotect(addr: *mut ::c_void, len: ::size_t, prot: ::c_int) -> ::c_int {
//     platform::pal::mprotect(addr, len, prot)
// }

// #[no_mangle]
// pub unsafe extern "C" fn msync(addr: *mut ::c_void, len: ::size_t, flags: ::c_int) -> ::c_int {
//     platform::pal::msync(addr, len, flags)
// }

// #[no_mangle]
// pub unsafe extern "C" fn munlock(addr: *const ::c_void, len: usize) -> ::c_int {
//     platform::pal::munlock(addr, len)
// }

// #[no_mangle]
// pub extern "C" fn munlockall() -> ::c_int {
//     platform::pal::munlockall()
// }

// #[no_mangle]
// pub unsafe extern "C" fn munmap(addr: *mut ::c_void, len: ::size_t) -> ::c_int {
//     platform::pal::munmap(addr, len)
// }

#[cfg(target_os = "linux")]
static SHM_PATH: &'static [u8] = b"/dev/shm/";

#[cfg(target_os = "dragonos")]
static SHM_PATH: &'static [u8] = b"/dev/shm/";

#[cfg(target_os = "redox")]
static SHM_PATH: &'static [u8] = b"shm:";

unsafe fn shm_path(name: *const ::c_char) -> CString {
    let name_c = CStr::from_ptr(name);

    let mut path = SHM_PATH.to_vec();

    let mut skip_slash = true;
    for &b in name_c.to_bytes() {
        if skip_slash {
            if b == b'/' {
                continue;
            } else {
                skip_slash = false;
            }
        }
        path.push(b);
    }

    CString::from_vec_unchecked(path)
}

#[no_mangle]
pub unsafe extern "C" fn shm_open(name: *const ::c_char, oflag: ::c_int, mode: ::mode_t) -> ::c_int {
    let path = shm_path(name);
    fcntl::sys_open(path.as_ptr(), oflag, mode)
}

#[no_mangle]
pub unsafe extern "C" fn shm_unlink(name: *const ::c_char) -> ::c_int {
    let path = shm_path(name);
    ::unlink(path.as_ptr())
}
