// This file was generated by gir (f00d658) from gir-files (1069259)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

#[cfg(feature = "v2_10")]
bitflags! {
    pub struct EditorTypingAttributes: u32 {
        const EDITOR_TYPING_ATTRIBUTE_NONE = 2;
        const EDITOR_TYPING_ATTRIBUTE_BOLD = 4;
        const EDITOR_TYPING_ATTRIBUTE_ITALIC = 8;
        const EDITOR_TYPING_ATTRIBUTE_UNDERLINE = 16;
        const EDITOR_TYPING_ATTRIBUTE_STRIKETHROUGH = 32;
    }
}

#[cfg(feature = "v2_10")]
#[doc(hidden)]
impl ToGlib for EditorTypingAttributes {
    type GlibType = ffi::WebKitEditorTypingAttributes;

    fn to_glib(&self) -> ffi::WebKitEditorTypingAttributes {
        ffi::WebKitEditorTypingAttributes::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v2_10")]
#[doc(hidden)]
impl FromGlib<ffi::WebKitEditorTypingAttributes> for EditorTypingAttributes {
    fn from_glib(value: ffi::WebKitEditorTypingAttributes) -> EditorTypingAttributes {
        skip_assert_initialized!();
        EditorTypingAttributes::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v2_10")]
impl StaticType for EditorTypingAttributes {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_editor_typing_attributes_get_type()) }
    }
}

#[cfg(feature = "v2_10")]
impl<'a> FromValueOptional<'a> for EditorTypingAttributes {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

#[cfg(feature = "v2_10")]
impl<'a> FromValue<'a> for EditorTypingAttributes {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::WebKitEditorTypingAttributes::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

#[cfg(feature = "v2_10")]
impl SetValue for EditorTypingAttributes {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct FindOptions: u32 {
        const FIND_OPTIONS_NONE = 0;
        const FIND_OPTIONS_CASE_INSENSITIVE = 1;
        const FIND_OPTIONS_AT_WORD_STARTS = 2;
        const FIND_OPTIONS_TREAT_MEDIAL_CAPITAL_AS_WORD_START = 4;
        const FIND_OPTIONS_BACKWARDS = 8;
        const FIND_OPTIONS_WRAP_AROUND = 16;
    }
}

#[doc(hidden)]
impl ToGlib for FindOptions {
    type GlibType = ffi::WebKitFindOptions;

    fn to_glib(&self) -> ffi::WebKitFindOptions {
        ffi::WebKitFindOptions::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitFindOptions> for FindOptions {
    fn from_glib(value: ffi::WebKitFindOptions) -> FindOptions {
        skip_assert_initialized!();
        FindOptions::from_bits_truncate(value.bits())
    }
}

impl StaticType for FindOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_find_options_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FindOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FindOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::WebKitFindOptions::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for FindOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct HitTestResultContext: u32 {
        const HIT_TEST_RESULT_CONTEXT_DOCUMENT = 2;
        const HIT_TEST_RESULT_CONTEXT_LINK = 4;
        const HIT_TEST_RESULT_CONTEXT_IMAGE = 8;
        const HIT_TEST_RESULT_CONTEXT_MEDIA = 16;
        const HIT_TEST_RESULT_CONTEXT_EDITABLE = 32;
        const HIT_TEST_RESULT_CONTEXT_SCROLLBAR = 64;
        const HIT_TEST_RESULT_CONTEXT_SELECTION = 128;
    }
}

#[doc(hidden)]
impl ToGlib for HitTestResultContext {
    type GlibType = ffi::WebKitHitTestResultContext;

    fn to_glib(&self) -> ffi::WebKitHitTestResultContext {
        ffi::WebKitHitTestResultContext::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitHitTestResultContext> for HitTestResultContext {
    fn from_glib(value: ffi::WebKitHitTestResultContext) -> HitTestResultContext {
        skip_assert_initialized!();
        HitTestResultContext::from_bits_truncate(value.bits())
    }
}

impl StaticType for HitTestResultContext {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_hit_test_result_context_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for HitTestResultContext {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for HitTestResultContext {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::WebKitHitTestResultContext::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for HitTestResultContext {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SnapshotOptions: u32 {
        const SNAPSHOT_OPTIONS_NONE = 0;
        const SNAPSHOT_OPTIONS_INCLUDE_SELECTION_HIGHLIGHTING = 1;
        const SNAPSHOT_OPTIONS_TRANSPARENT_BACKGROUND = 2;
    }
}

#[doc(hidden)]
impl ToGlib for SnapshotOptions {
    type GlibType = ffi::WebKitSnapshotOptions;

    fn to_glib(&self) -> ffi::WebKitSnapshotOptions {
        ffi::WebKitSnapshotOptions::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitSnapshotOptions> for SnapshotOptions {
    fn from_glib(value: ffi::WebKitSnapshotOptions) -> SnapshotOptions {
        skip_assert_initialized!();
        SnapshotOptions::from_bits_truncate(value.bits())
    }
}

impl StaticType for SnapshotOptions {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::webkit_snapshot_options_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SnapshotOptions {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SnapshotOptions {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::WebKitSnapshotOptions::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SnapshotOptions {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

