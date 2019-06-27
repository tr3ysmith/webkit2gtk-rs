// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use glib::GString;
use glib::translate::*;
use webkit2_sys;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct MimeInfo(Shared<webkit2_sys::WebKitMimeInfo>);

    match fn {
        ref => |ptr| webkit2_sys::webkit_mime_info_ref(ptr),
        unref => |ptr| webkit2_sys::webkit_mime_info_unref(ptr),
        get_type => || webkit2_sys::webkit_mime_info_get_type(),
    }
}

impl MimeInfo {
    pub fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_mime_info_get_description(self.to_glib_none().0))
        }
    }

    pub fn get_extensions(&self) -> Vec<GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(webkit2_sys::webkit_mime_info_get_extensions(self.to_glib_none().0))
        }
    }

    pub fn get_mime_type(&self) -> Option<GString> {
        unsafe {
            from_glib_none(webkit2_sys::webkit_mime_info_get_mime_type(self.to_glib_none().0))
        }
    }
}