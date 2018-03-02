// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct NetworkProxySettings(Boxed<ffi::WebKitNetworkProxySettings>);

    match fn {
        copy => |ptr| ffi::webkit_network_proxy_settings_copy(mut_override(ptr)),
        free => |ptr| ffi::webkit_network_proxy_settings_free(ptr),
        get_type => || ffi::webkit_network_proxy_settings_get_type(),
    }
}

impl NetworkProxySettings {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn add_proxy_for_scheme(&mut self, scheme: &str, proxy_uri: &str) {
        unsafe {
            ffi::webkit_network_proxy_settings_add_proxy_for_scheme(self.to_glib_none_mut().0, scheme.to_glib_none().0, proxy_uri.to_glib_none().0);
        }
    }
}
