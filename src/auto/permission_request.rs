// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct PermissionRequest(Interface<ffi::WebKitPermissionRequest>);

    match fn {
        get_type => || ffi::webkit_permission_request_get_type(),
    }
}

pub const NONE_PERMISSION_REQUEST: Option<&PermissionRequest> = None;

pub trait PermissionRequestExt: 'static {
    fn allow(&self);

    fn deny(&self);
}

impl<O: IsA<PermissionRequest>> PermissionRequestExt for O {
    fn allow(&self) {
        unsafe {
            ffi::webkit_permission_request_allow(self.as_ref().to_glib_none().0);
        }
    }

    fn deny(&self) {
        unsafe {
            ffi::webkit_permission_request_deny(self.as_ref().to_glib_none().0);
        }
    }
}

impl fmt::Display for PermissionRequest {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PermissionRequest")
    }
}
