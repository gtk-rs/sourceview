// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::ToGlibPtr;
use glib::translate::from_glib_none;

use MarkAttributes;
use View;

impl View {
    #[cfg(feature = "v2_2")]
    pub fn get_mark_attributes(&self, category: &str, priority: &mut i32) -> Option<MarkAttributes> {
        unsafe {
            from_glib_none(ffi::gtk_source_view_get_mark_attributes(self.to_glib_none().0,
                                                                    category.to_glib_none().0,
                                                                    priority))
        }
    }
}
