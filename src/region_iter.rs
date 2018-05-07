// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use std::mem;

use ffi;
use glib::translate::*;

pub struct RegionIter {
    inner: ffi::GtkSourceRegionIter,
}

#[doc(hidden)]
impl Uninitialized for RegionIter {
    unsafe fn uninitialized() -> Self {
        mem::uninitialized()
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const ffi::GtkSourceRegionIter> for RegionIter {
    type Storage = &'a Self;

    #[inline]
    fn to_glib_none(&'a self) -> Stash<'a, *const ffi::GtkSourceRegionIter, Self> {
        let ptr: *const ffi::GtkSourceRegionIter = &self.inner;
        Stash(ptr as *const ffi::GtkSourceRegionIter, self)
    }
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut ffi::GtkSourceRegionIter> for RegionIter {
    type Storage = &'a mut Self;

    #[inline]
    fn to_glib_none_mut(&'a mut self) -> StashMut<'a, *mut ffi::GtkSourceRegionIter, Self> {
        let ptr: *mut ffi::GtkSourceRegionIter = &mut self.inner;
        StashMut(ptr as *mut ffi::GtkSourceRegionIter, self)
    }
}
