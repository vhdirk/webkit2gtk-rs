// This file was generated by gir (d9591be) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_6")]
use UserContentInjectedFrames;
#[cfg(feature = "v2_6")]
use UserStyleLevel;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct UserStyleSheet(Shared<ffi::WebKitUserStyleSheet>);

    match fn {
        ref => |ptr| ffi::webkit_user_style_sheet_ref(ptr),
        unref => |ptr| ffi::webkit_user_style_sheet_unref(ptr),
    }
}

impl UserStyleSheet {

    #[cfg(feature = "v2_6")]
    pub fn new(source: &str, injected_frames: UserContentInjectedFrames, level: UserStyleLevel, whitelist: &[&str], blacklist: &[&str]) -> UserStyleSheet {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_user_style_sheet_new(source.to_glib_none().0, injected_frames.to_glib(), level.to_glib(), whitelist.to_glib_none().0, blacklist.to_glib_none().0))
        }
    }
}
