// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

#[cfg(feature = "v2_16")]
use CookieManager;
use ffi;
use glib;
#[cfg(feature = "v2_16")]
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct WebsiteDataManager(Object<ffi::WebKitWebsiteDataManager>);

    match fn {
        get_type => || ffi::webkit_website_data_manager_get_type(),
    }
}

impl WebsiteDataManager {
    //#[cfg(feature = "v2_10")]
    //pub fn new(first_option_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> WebsiteDataManager {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_new() }
    //}

    #[cfg(feature = "v2_16")]
    pub fn new_ephemeral() -> WebsiteDataManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_website_data_manager_new_ephemeral())
        }
    }
}

pub trait WebsiteDataManagerExt {
    //#[cfg(feature = "v2_16")]
    //fn clear<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, timespan: /*Ignored*/glib::TimeSpan, cancellable: P, callback: Q, user_data: R);

    //#[cfg(feature = "v2_16")]
    //fn clear_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error>;

    //#[cfg(feature = "v2_16")]
    //fn fetch<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, cancellable: P, callback: Q, user_data: R);

    //#[cfg(feature = "v2_16")]
    //fn fetch_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result</*Ignored*/Vec<WebsiteData>, Error>;

    #[cfg(feature = "v2_10")]
    fn get_base_cache_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_10")]
    fn get_base_data_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_16")]
    fn get_cookie_manager(&self) -> Option<CookieManager>;

    #[cfg(feature = "v2_10")]
    fn get_disk_cache_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_10")]
    fn get_indexeddb_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_10")]
    fn get_local_storage_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_10")]
    fn get_offline_application_cache_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_10")]
    fn get_websql_directory(&self) -> Option<String>;

    #[cfg(feature = "v2_16")]
    fn is_ephemeral(&self) -> bool;

    //#[cfg(feature = "v2_16")]
    //fn remove<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, website_data: /*Ignored*/&[&WebsiteData], cancellable: P, callback: Q, user_data: R);

    //#[cfg(feature = "v2_16")]
    //fn remove_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error>;

    #[cfg(feature = "v2_16")]
    fn get_property_is_ephemeral(&self) -> bool;
}

impl<O: IsA<WebsiteDataManager> + IsA<glib::object::Object>> WebsiteDataManagerExt for O {
    //#[cfg(feature = "v2_16")]
    //fn clear<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, timespan: /*Ignored*/glib::TimeSpan, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_clear() }
    //}

    //#[cfg(feature = "v2_16")]
    //fn clear_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_clear_finish() }
    //}

    //#[cfg(feature = "v2_16")]
    //fn fetch<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_fetch() }
    //}

    //#[cfg(feature = "v2_16")]
    //fn fetch_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result</*Ignored*/Vec<WebsiteData>, Error> {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_fetch_finish() }
    //}

    #[cfg(feature = "v2_10")]
    fn get_base_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_base_data_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_base_data_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    fn get_cookie_manager(&self) -> Option<CookieManager> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_cookie_manager(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_disk_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_disk_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_indexeddb_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_indexeddb_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_local_storage_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_local_storage_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_offline_application_cache_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_offline_application_cache_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_10")]
    fn get_websql_directory(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_website_data_manager_get_websql_directory(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_16")]
    fn is_ephemeral(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_website_data_manager_is_ephemeral(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v2_16")]
    //fn remove<'a, 'b, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::AsyncReadyCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, types: /*Ignored*/WebsiteDataTypes, website_data: /*Ignored*/&[&WebsiteData], cancellable: P, callback: Q, user_data: R) {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_remove() }
    //}

    //#[cfg(feature = "v2_16")]
    //fn remove_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::webkit_website_data_manager_remove_finish() }
    //}

    #[cfg(feature = "v2_16")]
    fn get_property_is_ephemeral(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "is-ephemeral".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }
}
