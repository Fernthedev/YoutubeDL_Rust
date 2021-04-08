use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::os::raw::c_char;

use crate::drop_extern;
use crate::vec_extern;

use crate::youtubedl::data::{YoutubeDlFormatJson, YoutubeDlVideoJson};
use std::ffi::{CStr, CString};
use std::ptr;

#[derive(Eq)]
#[repr(C)]
pub struct RustCStringWrapper {
    pub string_data: *mut c_char,
}

impl Clone for RustCStringWrapper {
    fn clone(&self) -> Self {
        RustCStringWrapper::new(self.to_string().into())
    }
}

impl PartialEq for RustCStringWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}

impl PartialOrd for RustCStringWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.to_string().partial_cmp(&other.to_string())
    }
}

impl Hash for RustCStringWrapper {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state)
    }
}

unsafe impl Send for RustCStringWrapper {}
unsafe impl Sync for RustCStringWrapper {}

impl ToString for RustCStringWrapper {
    fn to_string(&self) -> String {
        if self.string_data.is_null() {
            return String::new();
        }

        let raw = unsafe { CStr::from_ptr(self.string_data) };

        let hash_str = match raw.to_str() {
            Ok(s) => s,
            Err(_) => return String::new(),
        };

        hash_str.to_string()
    }
}

impl Drop for RustCStringWrapper {
    fn drop(&mut self) {
        if self.string_data.is_null() {
            return;
        }
        unsafe {
            CString::from_raw(self.string_data);
        }
    }
}

impl RustCStringWrapper {
    pub fn new(str_data: Vec<u8>) -> RustCStringWrapper {
        let c_string = CString::new(str_data).expect("RustCStringWrapper::new failed");
        let ptr = c_string.into_raw();
        RustCStringWrapper { string_data: ptr }
    }
}

#[no_mangle]
pub extern "C" fn RustCStringWrapper_c_new(c_str: *mut c_char) -> RustCStringWrapper {
    unsafe {
        RustCStringWrapper {
            string_data: CString::from_raw(c_str).into_raw(),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct YoutubeDlVideo {
    pub id: RustCStringWrapper,
    pub title: RustCStringWrapper,
    pub formats: *const Vec<YoutubeDlFormat>,
}

drop_extern!(YoutubeDlVideo, YoutubeDlVideo_drop);

vec_extern!(
    YoutubeDlVideo,
    formats,
    YoutubeDlFormat,
    YoutubeDlVideo_FormatsGet,
    YoutubeDlVideo_FormatsLen
);

impl YoutubeDlVideo {
    pub fn convert(og: YoutubeDlVideoJson) -> YoutubeDlVideo {
        let mut formats_converted: Vec<YoutubeDlFormat> = vec![];

        for format in og.formats {
            formats_converted.push(YoutubeDlFormat::convert(format));
        }

        YoutubeDlVideo {
            id: RustCStringWrapper::new(og.id.into()),
            title: RustCStringWrapper::new(og.title.into()),
            formats: Box::into_raw(Box::new(formats_converted)),
        }
    }
}

#[repr(C)]
pub struct YoutubeDlFormat {
    pub height: *mut u32,
    pub width: *mut u32,
    pub acodec: *mut RustCStringWrapper,
    pub format_note: *mut RustCStringWrapper,
    pub url: RustCStringWrapper,
    pub ext: RustCStringWrapper,
}

impl YoutubeDlFormat {
    pub fn convert(og: YoutubeDlFormatJson) -> YoutubeDlFormat {
        let mut acodec: *mut RustCStringWrapper = ptr::null_mut();

        if og.acodec.is_some() {
            acodec = Box::into_raw(Box::new(RustCStringWrapper::new(og.acodec.unwrap().into())));
        }

        let mut format_note: *mut RustCStringWrapper = ptr::null_mut();

        if og.format_note.is_some() {
            format_note = Box::into_raw(Box::new(RustCStringWrapper::new(
                og.format_note.unwrap().into(),
            )));
        }

        let mut height: *mut u32 = ptr::null_mut();

        if og.height.is_some() {
            height = Box::into_raw(Box::new(og.height.unwrap()));
        }

        let mut width: *mut u32 = ptr::null_mut();

        if og.width.is_some() {
            width = Box::into_raw(Box::new(og.width.unwrap()));
        }

        YoutubeDlFormat {
            height,
            width,
            acodec,
            format_note,
            url: RustCStringWrapper::new(og.url.into()),
            ext: RustCStringWrapper::new(og.ext.into()),
        }
    }
}

impl Drop for YoutubeDlFormat {
    fn drop(&mut self) {
        unsafe {
            self.acodec.drop_in_place();
            self.format_note.drop_in_place();
            self.width.drop_in_place();
            self.height.drop_in_place();
        }
    }
}


