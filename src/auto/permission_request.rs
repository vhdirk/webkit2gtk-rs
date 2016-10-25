// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct PermissionRequest(Object<ffi::WebKitPermissionRequest>);

    match fn {
        get_type => || ffi::webkit_permission_request_get_type(),
    }
}

pub trait PermissionRequestExt {
    fn allow(&self);

    fn deny(&self);
}

impl<O: IsA<PermissionRequest>> PermissionRequestExt for O {
    fn allow(&self) {
        unsafe {
            ffi::webkit_permission_request_allow(self.to_glib_none().0);
        }
    }

    fn deny(&self) {
        unsafe {
            ffi::webkit_permission_request_deny(self.to_glib_none().0);
        }
    }
}
