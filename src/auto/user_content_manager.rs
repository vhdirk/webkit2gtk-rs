// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_8")]
use JavascriptResult;
#[cfg(feature = "v2_6")]
use UserScript;
#[cfg(feature = "v2_6")]
use UserStyleSheet;
use ffi;
#[cfg(feature = "v2_8")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_8")]
use glib_ffi;
#[cfg(feature = "v2_8")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_8")]
use std::mem::transmute;

glib_wrapper! {
    pub struct UserContentManager(Object<ffi::WebKitUserContentManager>);

    match fn {
        get_type => || ffi::webkit_user_content_manager_get_type(),
    }
}

impl UserContentManager {
    #[cfg(feature = "v2_6")]
    pub fn new() -> UserContentManager {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_content_manager_new())
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn add_script(&self, script: &UserScript) {
        unsafe {
            ffi::webkit_user_content_manager_add_script(self.to_glib_none().0, script.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn add_style_sheet(&self, stylesheet: &UserStyleSheet) {
        unsafe {
            ffi::webkit_user_content_manager_add_style_sheet(self.to_glib_none().0, stylesheet.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn register_script_message_handler(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_user_content_manager_register_script_message_handler(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn remove_all_scripts(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_scripts(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_6")]
    pub fn remove_all_style_sheets(&self) {
        unsafe {
            ffi::webkit_user_content_manager_remove_all_style_sheets(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn unregister_script_message_handler(&self, name: &str) {
        unsafe {
            ffi::webkit_user_content_manager_unregister_script_message_handler(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v2_8")]
    pub fn connect_script_message_received<F: Fn(&UserContentManager, &JavascriptResult) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&UserContentManager, &JavascriptResult) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "script-message-received",
                transmute(script_message_received_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_8")]
unsafe extern "C" fn script_message_received_trampoline(this: *mut ffi::WebKitUserContentManager, js_result: *mut ffi::WebKitJavascriptResult, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&UserContentManager, &JavascriptResult) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(js_result))
}
