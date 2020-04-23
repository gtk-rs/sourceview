// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::translate::*;
use gtk_source_sys;
use std::mem;

pub struct RegionIter
{
	inner: gtk_source_sys::GtkSourceRegionIter,
}

#[doc(hidden)]
impl Uninitialized for RegionIter
{
	unsafe fn uninitialized() -> Self
	{
		mem::zeroed()
	}
}

#[doc(hidden)]
impl<'a> ToGlibPtr<'a, *const gtk_source_sys::GtkSourceRegionIter> for RegionIter
{
	type Storage = &'a Self;

	#[inline]
	fn to_glib_none(&'a self) -> Stash<'a, *const gtk_source_sys::GtkSourceRegionIter, Self>
	{
		let ptr: *const gtk_source_sys::GtkSourceRegionIter = &self.inner;
		Stash(ptr as *const gtk_source_sys::GtkSourceRegionIter, self)
	}
}

#[doc(hidden)]
impl<'a> ToGlibPtrMut<'a, *mut gtk_source_sys::GtkSourceRegionIter> for RegionIter
{
	type Storage = &'a mut Self;

	#[inline]
	fn to_glib_none_mut(
		&'a mut self,
	) -> StashMut<'a, *mut gtk_source_sys::GtkSourceRegionIter, Self>
	{
		let ptr: *mut gtk_source_sys::GtkSourceRegionIter = &mut self.inner;
		StashMut(ptr as *mut gtk_source_sys::GtkSourceRegionIter, self)
	}
}
