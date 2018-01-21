pub use core_foundation_sys::filedescriptor::*;

use core_foundation_sys::base::{Boolean, CFIndex, CFRelease};
use core_foundation_sys::base::{kCFAllocatorDefault, CFOptionFlags};

use base::TCFType;
use runloop::CFRunLoopSource;

use std::mem;
use std::os::unix::io::{AsRawFd, RawFd};
use std::ptr;

pub struct CFFileDescriptor(CFFileDescriptorRef);

impl Drop for CFFileDescriptor {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_CFTypeRef())
        }
    }
}

impl_TCFType!(CFFileDescriptor, CFFileDescriptorRef, CFFileDescriptorGetTypeID);

impl CFFileDescriptor {
    pub fn new(fd: RawFd,
               closeOnInvalidate: bool,
               callout: CFFileDescriptorCallBack,
               context: Option<&CFFileDescriptorContext>) -> Option<CFFileDescriptor> {
        let context = context.map_or(ptr::null(), |c| c as *const _);
        unsafe {
            let fd_ref = CFFileDescriptorCreate(kCFAllocatorDefault,
                                                fd,
                                                closeOnInvalidate as Boolean,
                                                callout,
                                                context);
            if fd_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(fd_ref))
            }
        }
    }

    pub fn context(&self) -> CFFileDescriptorContext {
        unsafe {
            let mut context: CFFileDescriptorContext = mem::uninitialized();
            CFFileDescriptorGetContext(self.0, &mut context);
            context
        }
    }

    pub fn enable_callbacks(&self, callback_types: CFOptionFlags) {
        unsafe {
            CFFileDescriptorEnableCallBacks(self.0, callback_types)
        }
    }

    pub fn disable_callbacks(&self, callback_types: CFOptionFlags) {
        unsafe {
            CFFileDescriptorDisableCallBacks(self.0, callback_types)
        }
    }

    pub fn valid(&self) -> bool {
        unsafe {
            CFFileDescriptorIsValid(self.0) != 0
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            CFFileDescriptorInvalidate(self.0)
        }
    }

    pub fn to_run_loop_source(&self, order: CFIndex) -> Option<CFRunLoopSource> {
        unsafe {
            let source_ref = CFFileDescriptorCreateRunLoopSource(
                kCFAllocatorDefault,
                self.0,
                order
            );
            if source_ref.is_null() {
                None
            } else {
                Some(TCFType::wrap_under_create_rule(source_ref))
            }
        }
    }
}

impl AsRawFd for CFFileDescriptor {
    fn as_raw_fd(&self) -> RawFd {
        unsafe {
            CFFileDescriptorGetNativeDescriptor(self.0)
        }
    }
}


#[cfg(test)]
mod test {
    extern crate libc;

    use super::*;
    use std::ffi::CString;
    use libc::{c_void, O_RDWR};
    use core_foundation_sys::base::{CFOptionFlags};
    use core_foundation_sys::runloop::{kCFRunLoopDefaultMode};
    use runloop::{CFRunLoop};

    #[test]
    fn test_consumed() {
        let path = CString::new("/dev/null").unwrap();
        let raw_fd = unsafe { libc::open(path.as_ptr(), O_RDWR, 0) };
        let cf_fd = CFFileDescriptor::new(raw_fd, true, never_callback, None);
        assert!(cf_fd.is_some());
        let cf_fd = cf_fd.unwrap();

        assert!(cf_fd.valid());
        cf_fd.invalidate();
        assert!(!cf_fd.valid());

        // close() should fail
        assert_eq!(unsafe { libc::close(raw_fd) }, -1);
    }

    #[test]
    fn test_unconsumed() {
        let path = CString::new("/dev/null").unwrap();
        let raw_fd = unsafe { libc::open(path.as_ptr(), O_RDWR, 0) };
        let cf_fd = CFFileDescriptor::new(raw_fd, false, never_callback, None);
        assert!(cf_fd.is_some());
        let cf_fd = cf_fd.unwrap();

        assert!(cf_fd.valid());
        cf_fd.invalidate();
        assert!(!cf_fd.valid());

        // close() should succeed
        assert_eq!(unsafe { libc::close(raw_fd) }, 0);
    }

    extern "C" fn never_callback(_f: CFFileDescriptorRef,
                                 _callback_types: CFOptionFlags,
                                 _info_ptr: *mut c_void) {
        unreachable!();
    }

    struct TestInfo {
        value: CFOptionFlags
    }

    #[test]
    fn test_callback() {
        let mut info = TestInfo { value: 0 };
        let context = CFFileDescriptorContext {
            version: 0,
            info: &mut info as *mut _ as *mut c_void,
            retain: None,
            release: None,
            copyDescription: None
        };

        let path = CString::new("/dev/null").unwrap();
        let raw_fd = unsafe { libc::open(path.as_ptr(), O_RDWR, 0) };
        let cf_fd = CFFileDescriptor::new(raw_fd, true, callback, Some(&context));
        assert!(cf_fd.is_some());
        let cf_fd = cf_fd.unwrap();

        assert!(cf_fd.valid());

        let runloop = CFRunLoop::get_current();
        let source = CFRunLoopSource::from_file_descriptor(&cf_fd, 0);
        assert!(source.is_some());
        unsafe {
            runloop.add_source(&source.unwrap(), kCFRunLoopDefaultMode);
        }

        info.value = 0;
        cf_fd.enable_callbacks(kCFFileDescriptorReadCallBack);
        CFRunLoop::run_current();
        assert_eq!(info.value, kCFFileDescriptorReadCallBack);

        info.value = 0;
        cf_fd.enable_callbacks(kCFFileDescriptorWriteCallBack);
        CFRunLoop::run_current();
        assert_eq!(info.value, kCFFileDescriptorWriteCallBack);

        info.value = 0;
        cf_fd.disable_callbacks(kCFFileDescriptorReadCallBack | kCFFileDescriptorWriteCallBack);

        cf_fd.invalidate();
        assert!(!cf_fd.valid());
    }

    extern "C" fn callback(_f: CFFileDescriptorRef, callback_types: CFOptionFlags, info_ptr: *mut c_void) {
        assert!(!info_ptr.is_null());

        let info: *mut TestInfo = info_ptr as *mut TestInfo;

        unsafe { (*info).value = callback_types };

        CFRunLoop::get_current().stop();
    }
}
