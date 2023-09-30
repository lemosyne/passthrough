use anyhow::Result;
use core::ffi::*;
use fuse_sys::*;
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

trait AsPtrMut<T> {
    fn as_ptr_mut(&self) -> *mut T;
}

impl<'a, T> AsPtrMut<T> for Option<&'a mut T> {
    fn as_ptr_mut(&self) -> *mut T {
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

    fn canonicalize(&self, path: &str) -> String {
        format!("/tmp/fsdata{path}")
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
                dbg!(self.canonicalize(path)).as_ptr(),
                stbuf.unwrap(),
                fi.as_ptr_mut(),
            )
        })
    }

    fn access(&mut self, path: &str, amode: c_int) -> Result<i32> {
        Ok(unsafe { xmp_access(self.canonicalize(path).as_ptr(), amode) })
    }

    fn opendir(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_opendir(dbg!(self.canonicalize(path)).as_ptr(), fi.as_ptr_mut()) })
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
                fi.as_ptr_mut(),
                flags,
            )
        })
    }

    fn releasedir(&mut self, path: &str, fi: Option<&mut fuse_file_info>) -> Result<i32> {
        Ok(unsafe { xmp_releasedir(self.canonicalize(path).as_ptr(), fi.as_ptr_mut()) })
    }
}
