// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use gtk;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use gtk_ffi;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_16", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct PrintCustomWidget(Object<ffi::WebKitPrintCustomWidget, ffi::WebKitPrintCustomWidgetClass>);

    match fn {
        get_type => || ffi::webkit_print_custom_widget_get_type(),
    }
}

impl PrintCustomWidget {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    pub fn new<P: IsA<gtk::Widget>>(widget: &P, title: &str) -> PrintCustomWidget {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_print_custom_widget_new(widget.to_glib_none().0, title.to_glib_none().0))
        }
    }
}

pub trait PrintCustomWidgetExt {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_title(&self) -> Option<String>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_update<F: Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<PrintCustomWidget> + IsA<glib::object::Object>> PrintCustomWidgetExt for O {
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::webkit_print_custom_widget_get_title(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_widget(&self) -> Option<gtk::Widget> {
        unsafe {
            from_glib_none(ffi::webkit_print_custom_widget_get_widget(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_apply<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "apply",
                transmute(apply_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_update<F: Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gtk::PageSetup, &gtk::PrintSettings) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "update",
                transmute(update_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::title",
                transmute(notify_title_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::widget",
                transmute(notify_widget_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn apply_trampoline<P>(this: *mut ffi::WebKitPrintCustomWidget, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintCustomWidget::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn update_trampoline<P>(this: *mut ffi::WebKitPrintCustomWidget, page_setup: *mut gtk_ffi::GtkPageSetup, print_settings: *mut gtk_ffi::GtkPrintSettings, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    callback_guard!();
    let f: &&(Fn(&P, &gtk::PageSetup, &gtk::PrintSettings) + 'static) = transmute(f);
    f(&PrintCustomWidget::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(page_setup), &from_glib_borrow(print_settings))
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn notify_title_trampoline<P>(this: *mut ffi::WebKitPrintCustomWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintCustomWidget::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn notify_widget_trampoline<P>(this: *mut ffi::WebKitPrintCustomWidget, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<PrintCustomWidget> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&PrintCustomWidget::from_glib_borrow(this).downcast_unchecked())
}
