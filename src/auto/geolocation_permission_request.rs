// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct GeolocationPermissionRequest(Object<ffi::WebKitGeolocationPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_geolocation_permission_request_get_type(),
    }
}

impl GeolocationPermissionRequest {}
