// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <https://opensource.org/licenses/MIT>

use gdk_pixbuf;
use glib::object::IsA;
use glib::translate::*;
use gtk;
use gtk_source_sys;
use MarkAttributes;

impl MarkAttributes {
    pub fn get_pixbuf(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let ptr = gtk_source_sys::gtk_source_mark_attributes_get_pixbuf(self.to_glib_none().0);
            if ptr.is_null() {
                None
            } else {
                Some(from_glib_full(mut_override(ptr)))
            }
        }
    }

    pub fn render_icon<P: IsA<gtk::Widget>>(
        &self,
        widget: &P,
        size: i32,
    ) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            let ptr = gtk_source_sys::gtk_source_mark_attributes_render_icon(
                self.to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                size,
            );
            if ptr.is_null() {
                None
            } else {
                Some(from_glib_full(mut_override(ptr)))
            }
        }
    }
}
