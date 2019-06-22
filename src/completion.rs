// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk;
use gtk_source_sys;
use Completion;
use CompletionContext;

impl Completion {
    pub fn create_context(
        &self,
        mut position: Option<&mut gtk::TextIter>,
    ) -> Option<CompletionContext> {
        let position = position.to_glib_none_mut();
        unsafe {
            from_glib_none(gtk_source_sys::gtk_source_completion_create_context(
                self.to_glib_none().0,
                position.0,
            ))
        }
    }
}
