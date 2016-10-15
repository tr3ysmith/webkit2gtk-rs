// This file was generated by gir (78b1141) from gir-files (???)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct InstallMissingMediaPluginsPermissionRequest(Object<ffi::WebKitInstallMissingMediaPluginsPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_install_missing_media_plugins_permission_request_get_type(),
    }
}

impl InstallMissingMediaPluginsPermissionRequest {
    #[cfg(feature = "v2_10")]
    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_install_missing_media_plugins_permission_request_get_description(self.to_glib_none().0))
        }
    }
}
