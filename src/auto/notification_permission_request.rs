// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

use PermissionRequest;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct NotificationPermissionRequest(Object<ffi::WebKitNotificationPermissionRequest>): PermissionRequest;

    match fn {
        get_type => || ffi::webkit_notification_permission_request_get_type(),
    }
}

impl NotificationPermissionRequest {}
