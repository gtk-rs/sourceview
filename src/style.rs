// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk;
use gtk_source_sys;
use Style;

impl Style
{
	#[cfg(any(feature = "v3_22", feature = "dox"))]
	pub fn apply<P: IsA<gtk::TextTag>>(&self, tag: &P)
	{
		unsafe {
			let tmp: *mut gtk_source_sys::GtkSourceStyle = self.to_glib_none().0;
			gtk_source_sys::gtk_source_style_apply(
				tmp as *const gtk_source_sys::GtkSourceStyle,
				tag.as_ref().to_glib_none().0,
			);
		}
	}
}
