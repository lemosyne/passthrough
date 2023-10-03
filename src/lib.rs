use anyhow::Result;
use core::ffi::*;
use fuse_sys::*;
use std::ffi::CString;
use xmp::*;

trait AsPtr<T> {
    fn as_ptr(&self) -> *const T;
}

impl<'a, T> AsPtr<T> for Option<&'a T> {
    fn as_ptr(&self) -> *const T {
        match self {
            Some(v) => *v as *const _,
            None => std::ptr::null(),
        }
    }
}

trait AsMutPtr<T> {
    fn as_mut_ptr(&self) -> *mut T;
}

impl<'a, T> AsMutPtr<T> for Option<&'a mut T> {
    fn as_mut_ptr(&self) -> *mut T {
        match self {
            Some(v) => *v as *const _ as *mut _,
            None => std::ptr::null_mut(),
        }
    }
}

pub struct Passthru;

impl Passthru {
    pub fn new() -> Self {
        Self {}
    }

    fn canonicalize(&self, path: &str) -> CString {
        CString::new(format!("/tmp/fsdata{path}")).unwrap()
    }
}

impl UnthreadedFileSystem for Passthru {
    fn getattr(
        &mut self,
        path: &str,
        stbuf: Option<&mut fuse_sys::stat>,
        fi: Option<&mut fuse_sys::fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_getattr(
                self.canonicalize(path).as_ptr(),
                stbuf.unwrap(),
                fi.as_mut_ptr(),
            )
        })
    }

    fn readlink(&mut self, path: &str, buf: &mut [u8]) -> Result<i32> {
        Ok(unsafe {
            xmp_readlink(
                self.canonicalize(path).as_ptr(),
                buf.as_mut_ptr(),
                buf.len(),
            )
        })
    }

    fn mknod(&mut self, path: &str, mode: mode_t, rdev: dev_t) -> Result<i32> {
        Ok(unsafe { xmp_mknod(self.canonicalize(path).as_ptr(), mode, rdev) })
    }

    fn mkdir(&mut self, path: &str, mode: mode_t) -> Result<i32> {
        Ok(unsafe { xmp_mkdir(self.canonicalize(path).as_ptr(), mode) })
    }

    fn unlink(&mut self, path: &str) -> Result<i32> {
        Ok(unsafe { xmp_unlink(self.canonicalize(path).as_ptr()) })
    }

    fn rmdir(&mut self, path: &str) -> Result<i32> {
        Ok(unsafe { xmp_rmdir(self.canonicalize(path).as_ptr()) })
    }

    fn symlink(&mut self, from: &str, to: &str) -> Result<i32> {
        Ok(unsafe {
            xmp_symlink(
                self.canonicalize(from).as_ptr(),
                self.canonicalize(to).as_ptr(),
            )
        })
    }

    fn rename(&mut self, from: &str, to: &str, flags: c_uint) -> Result<i32> {
        Ok(unsafe {
            xmp_rename(
                self.canonicalize(from).as_ptr(),
                self.canonicalize(to).as_ptr(),
                flags,
            )
        })
    }

    fn link(&mut self, from: &str, to: &str) -> Result<i32> {
        Ok(unsafe {
            xmp_link(
                self.canonicalize(from).as_ptr(),
                self.canonicalize(to).as_ptr(),
            )
        })
    }

    fn chmod(&mut self, path: &str, mode: mode_t, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_chmod(self.canonicalize(path).as_ptr(), mode, fi.as_mut_ptr()) })
    }

    fn chown(
        &mut self,
        path: &str,
        uid: uid_t,
        gid: gid_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe { xmp_chown(self.canonicalize(path).as_ptr(), uid, gid, fi.as_mut_ptr()) })
    }

    fn truncate(
        &mut self,
        path: &str,
        size: off_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe { xmp_truncate(self.canonicalize(path).as_ptr(), size, fi.as_mut_ptr()) })
    }

    fn open(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_open(self.canonicalize(path).as_ptr(), fi.as_mut_ptr()) })
    }

    fn read(
        &mut self,
        path: &str,
        buf: &mut [u8],
        offset: off_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_read(
                self.canonicalize(path).as_ptr(),
                buf.as_mut_ptr(),
                buf.len(),
                offset,
                fi.as_mut_ptr(),
            )
        })
    }

    fn write(
        &mut self,
        path: &str,
        buf: &[u8],
        offset: off_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_write(
                self.canonicalize(path).as_ptr(),
                buf.as_ptr(),
                buf.len(),
                offset,
                fi.as_mut_ptr(),
            )
        })
    }

    fn statfs(&mut self, path: &str, stbuf: Option<&mut statvfs>) -> Result<i32> {
        Ok(unsafe { xmp_statfs(self.canonicalize(path).as_ptr(), stbuf.as_mut_ptr()) })
    }

    fn flush(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_flush(self.canonicalize(path).as_ptr(), fi.as_mut_ptr()) })
    }

    fn release(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_release(self.canonicalize(path).as_ptr(), fi.as_mut_ptr()) })
    }

    fn fsync(
        &mut self,
        path: &str,
        isdatasync: c_int,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_fsync(
                self.canonicalize(path).as_ptr(),
                isdatasync,
                fi.as_mut_ptr(),
            )
        })
    }

    fn opendir(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_opendir(self.canonicalize(path).as_ptr(), fi.as_mut_ptr()) })
    }

    fn readdir(
        &mut self,
        path: &str,
        buf: Option<&mut c_void>,
        filler: fuse_fill_dir_t,
        offset: off_t,
        fi: Option<&mut fuse_file_info>,
        flags: fuse_readdir_flags,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_readdir(
                self.canonicalize(path).as_ptr(),
                buf.unwrap(),
                filler,
                offset,
                fi.as_mut_ptr(),
                flags,
            )
        })
    }

    fn releasedir(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_release(self.canonicalize(path).as_ptr(), fi.as_mut_ptr()) })
    }

    fn access(&mut self, path: &str, mask: c_int) -> Result<i32> {
        Ok(unsafe { xmp_access(self.canonicalize(path).as_ptr(), mask) })
    }

    fn create(&mut self, path: &str, mode: mode_t, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_create(self.canonicalize(path).as_ptr(), mode, fi.as_mut_ptr()) })
    }

    fn write_buf(
        &mut self,
        path: &str,
        buf: Option<&mut fuse_bufvec>,
        offset: off_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        Ok(unsafe {
            xmp_write_buf(
                self.canonicalize(path).as_ptr(),
                buf.as_mut_ptr(),
                offset,
                fi.as_mut_ptr(),
            )
        })
    }

    fn read_buf(
        &mut self,
        path: &str,
        bufp: &mut [&mut fuse_bufvec],
        offset: off_t,
        fi: Option<&mut fuse_file_info>,
    ) -> Result<i32> {
        let mut bufp_raw: *mut fuse_bufvec = std::ptr::null_mut();

        let res = unsafe {
            xmp_read_buf(
                self.canonicalize(path).as_ptr(),
                &mut bufp_raw as *mut _,
                bufp.len(),
                offset,
                fi.as_mut_ptr(),
            )
        };

        bufp[0] = unsafe { bufp_raw.as_mut().unwrap() };

        Ok(res)
    }

    fn flock(&mut self, path: &str, fi: Option<&mut fuse_file_info>, op: c_int) -> Result<i32> {
        Ok(unsafe { xmp_flock(self.canonicalize(path).as_ptr(), fi.as_mut_ptr(), op) })
    }

    fn lock(
        &mut self,
        _arg1: &str,
        _arg2: Option<&mut fuse_file_info>,
        _cmd: c_int,
        _arg3: Option<&mut flock>,
    ) -> Result<i32> {
        // Dummy lock function for now.
        Ok(0)
    }
}
