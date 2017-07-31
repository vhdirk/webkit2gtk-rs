// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct FileChooserRequest(Object<ffi::WebKitFileChooserRequest>);

    match fn {
        get_type => || ffi::webkit_file_chooser_request_get_type(),
    }
}

pub trait FileChooserRequestExt {
    fn cancel(&self);

    fn get_mime_types(&self) -> Vec<String>;

    //fn get_mime_types_filter(&self) -> /*Ignored*/Option<gtk::FileFilter>;

    fn get_select_multiple(&self) -> bool;

    fn get_selected_files(&self) -> Vec<String>;

    fn select_files(&self, files: &[&str]);

    //fn get_property_filter(&self) -> /*Ignored*/Option<gtk::FileFilter>;
}

impl<O: IsA<FileChooserRequest> + IsA<glib::object::Object>> FileChooserRequestExt for O {
    fn cancel(&self) {
        unsafe {
            ffi::webkit_file_chooser_request_cancel(self.to_glib_none().0);
        }
    }

    fn get_mime_types(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_mime_types(self.to_glib_none().0))
        }
    }

    //fn get_mime_types_filter(&self) -> /*Ignored*/Option<gtk::FileFilter> {
    //    unsafe { TODO: call ffi::webkit_file_chooser_request_get_mime_types_filter() }
    //}

    fn get_select_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_file_chooser_request_get_select_multiple(self.to_glib_none().0))
        }
    }

    fn get_selected_files(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_file_chooser_request_get_selected_files(self.to_glib_none().0))
        }
    }

    fn select_files(&self, files: &[&str]) {
        unsafe {
            ffi::webkit_file_chooser_request_select_files(self.to_glib_none().0, files.to_glib_none().0);
        }
    }

    //fn get_property_filter(&self) -> /*Ignored*/Option<gtk::FileFilter> {
    //    let mut value = Value::from(None::<&/*Ignored*/gtk::FileFilter>);
    //    unsafe {
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "filter".to_glib_none().0, value.to_glib_none_mut().0);
    //    }
    //    value.get()
    //}
}
