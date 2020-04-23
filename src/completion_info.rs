// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk;
use gtk_source_sys;

use CompletionInfo;

impl CompletionInfo
{
	pub fn move_to_iter<'a, P: IsA<gtk::TextView>>(
		&self,
		view: &P,
		mut iter: Option<&mut gtk::TextIter>,
	)
	{
		let iter = iter.to_glib_none_mut();
		unsafe {
			gtk_source_sys::gtk_source_completion_info_move_to_iter(
				self.to_glib_none().0,
				view.as_ref().to_glib_none().0,
				iter.0,
			);
		}
	}
}
