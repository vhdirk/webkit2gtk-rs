// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use MimeInfo;
use ffi;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Plugin(Object<ffi::WebKitPlugin, ffi::WebKitPluginClass, PluginClass>);

    match fn {
        get_type => || ffi::webkit_plugin_get_type(),
    }
}

pub const NONE_PLUGIN: Option<&Plugin> = None;

pub trait PluginExt: 'static {
    fn get_description(&self) -> Option<GString>;

    fn get_mime_info_list(&self) -> Vec<MimeInfo>;

    fn get_name(&self) -> Option<GString>;

    fn get_path(&self) -> Option<GString>;
}

impl<O: IsA<Plugin>> PluginExt for O {
    fn get_description(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_description(self.as_ref().to_glib_none().0))
        }
    }

    fn get_mime_info_list(&self) -> Vec<MimeInfo> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::webkit_plugin_get_mime_info_list(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_path(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::webkit_plugin_get_path(self.as_ref().to_glib_none().0))
        }
    }
}

impl fmt::Display for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Plugin")
    }
}
