// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

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
extern crate once_cell;

extern crate libc;
#[macro_use]
extern crate bitflags;

pub use auto::*;
pub use completion::*;
pub use completion_info::*;
pub use completion_provider::*;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use file_loader::*;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use file_saver::*;
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
    #[doc(hidden)]
    pub use gtk::prelude::*;

    pub use auto::traits::*;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub use file_loader::FileLoaderExtManual;
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub use file_saver::FileSaverExtManual;
}

#[macro_use]
mod rt;

mod auto;
mod completion;
mod completion_info;
mod completion_provider;
#[cfg(any(feature = "v3_14", feature = "dox"))]
mod file_loader;
#[cfg(any(feature = "v3_14", feature = "dox"))]
mod file_saver;
mod gutter;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod mark_attributes;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod region_iter;
#[cfg(any(feature = "v3_22", feature = "dox"))]
mod style;
#[cfg(any(feature = "v2_2", feature = "dox"))]
mod view;
