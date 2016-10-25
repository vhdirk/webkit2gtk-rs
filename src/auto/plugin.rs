// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct Plugin(Object<ffi::WebKitPlugin>);

    match fn {
        get_type => || ffi::webkit_plugin_get_type(),
    }
}

impl Plugin {
    pub fn get_description(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_description(self.to_glib_none().0))
        }
    }

    //pub fn get_mime_info_list(&self) -> /*Ignored*/Vec<MimeInfo> {
    //    unsafe { TODO: call ffi::webkit_plugin_get_mime_info_list() }
    //}

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_path(self.to_glib_none().0))
        }
    }
}
