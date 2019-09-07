// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#![cfg_attr(feature = "cargo-clippy", allow(transmute_ptr_to_ref))]
#![cfg_attr(feature = "cargo-clippy", allow(type_complexity))]

#[macro_use]
extern crate glib;
extern crate cairo;
extern crate gdk;
extern crate gdk_pixbuf;
extern crate gdk_sys;
extern crate gio;
extern crate gio_sys;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gtk;
extern crate gtk_source_sys;
extern crate gtk_sys;
extern crate pango;

extern crate libc;
#[macro_use]
extern crate bitflags;

#[cfg(any(feature = "futures", feature = "dox"))]
extern crate fragile;
#[cfg(any(feature = "futures", feature = "dox"))]
extern crate futures;

pub use glib::{Error, Object};

pub use auto::*;
pub use completion::*;
pub use completion_info::*;
pub use gutter::*;
#[cfg(any(feature = "v2_2", feature = "dox"))]
pub use mark_attributes::*;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use region_iter::*;
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

#[macro_use]
mod rt;

mod auto;
mod completion;
mod completion_info;
mod gutter;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod mark_attributes;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod region_iter;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod style;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod view;
