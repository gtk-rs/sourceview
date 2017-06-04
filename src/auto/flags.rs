// This file was generated by gir (fdeaa47) from gir-files (2e2a9ca)
// DO NOT EDIT

use ffi;
use glib::translate::*;

bitflags! {
    pub flags CompletionActivation: u32 {
        const COMPLETION_ACTIVATION_NONE = 0,
        const COMPLETION_ACTIVATION_INTERACTIVE = 1,
        const COMPLETION_ACTIVATION_USER_REQUESTED = 2,
    }
}

#[doc(hidden)]
impl ToGlib for CompletionActivation {
    type GlibType = ffi::GtkSourceCompletionActivation;

    fn to_glib(&self) -> ffi::GtkSourceCompletionActivation {
        ffi::GtkSourceCompletionActivation::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceCompletionActivation> for CompletionActivation {
    fn from_glib(value: ffi::GtkSourceCompletionActivation) -> CompletionActivation {
        CompletionActivation::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags DrawSpacesFlags: u32 {
        const DRAW_SPACES_SPACE = 1,
        const DRAW_SPACES_TAB = 2,
        const DRAW_SPACES_NEWLINE = 4,
        const DRAW_SPACES_NBSP = 8,
        const DRAW_SPACES_LEADING = 16,
        const DRAW_SPACES_TEXT = 32,
        const DRAW_SPACES_TRAILING = 64,
        const DRAW_SPACES_ALL = 127,
    }
}

#[doc(hidden)]
impl ToGlib for DrawSpacesFlags {
    type GlibType = ffi::GtkSourceDrawSpacesFlags;

    fn to_glib(&self) -> ffi::GtkSourceDrawSpacesFlags {
        ffi::GtkSourceDrawSpacesFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceDrawSpacesFlags> for DrawSpacesFlags {
    fn from_glib(value: ffi::GtkSourceDrawSpacesFlags) -> DrawSpacesFlags {
        DrawSpacesFlags::from_bits_truncate(value.bits())
    }
}

#[cfg(feature = "v3_14")]
bitflags! {
    pub flags FileSaverFlags: u32 {
        const FILE_SAVER_FLAGS_NONE = 0,
        const FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS = 1,
        const FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME = 2,
        const FILE_SAVER_FLAGS_CREATE_BACKUP = 4,
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl ToGlib for FileSaverFlags {
    type GlibType = ffi::GtkSourceFileSaverFlags;

    fn to_glib(&self) -> ffi::GtkSourceFileSaverFlags {
        ffi::GtkSourceFileSaverFlags::from_bits_truncate(self.bits())
    }
}

#[cfg(feature = "v3_14")]
#[doc(hidden)]
impl FromGlib<ffi::GtkSourceFileSaverFlags> for FileSaverFlags {
    fn from_glib(value: ffi::GtkSourceFileSaverFlags) -> FileSaverFlags {
        FileSaverFlags::from_bits_truncate(value.bits())
    }
}

bitflags! {
    pub flags GutterRendererState: u32 {
        const GUTTER_RENDERER_STATE_NORMAL = 0,
        const GUTTER_RENDERER_STATE_CURSOR = 1,
        const GUTTER_RENDERER_STATE_PRELIT = 2,
        const GUTTER_RENDERER_STATE_SELECTED = 4,
    }
}

#[doc(hidden)]
impl ToGlib for GutterRendererState {
    type GlibType = ffi::GtkSourceGutterRendererState;

    fn to_glib(&self) -> ffi::GtkSourceGutterRendererState {
        ffi::GtkSourceGutterRendererState::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GtkSourceGutterRendererState> for GutterRendererState {
    fn from_glib(value: ffi::GtkSourceGutterRendererState) -> GutterRendererState {
        GutterRendererState::from_bits_truncate(value.bits())
    }
}

