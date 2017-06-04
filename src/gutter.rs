// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use ffi;
use glib::translate::ToGlibPtr;

use Gutter;

impl Gutter {
    pub fn get_padding(&self) -> (i32, i32) {
        let mut xpad = 0;
        let mut ypad = 0;
        unsafe {
            ffi::gtk_source_gutter_get_padding(self.to_glib_none().0, &mut xpad, &mut ypad);
        }
        (xpad, ypad)
    }
}
