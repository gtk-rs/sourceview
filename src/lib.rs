// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gtk_source_sys as ffi;
extern crate gobject_sys as gobject_ffi;
extern crate gio_sys as gio_ffi;
extern crate gdk_sys as gdk_ffi;
extern crate gtk_sys as gtk_ffi;
extern crate gio;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate pango;
extern crate gtk;
extern crate cairo;

extern crate libc;
#[macro_use]
extern crate bitflags;

#[cfg(feature = "futures")]
extern crate send_cell;
#[cfg(feature = "futures")]
extern crate futures_core;

pub use glib::{
    Error,
    Object,
};

pub use auto::*;
pub use completion::*;
pub use completion_info::*;
pub use gutter::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use mark_attributes::*;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use style::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use view::*;

pub mod signal {
    pub use glib::signal::Inhibit;
}

pub mod prelude {
    pub use auto::traits::*;
}

macro_rules! callback_guard {
    () => (
        let _guard = ::glib::CallbackGuard::new();
    )
}

mod auto;
mod completion;
mod completion_info;
mod gutter;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod mark_attributes;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod style;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod view;
