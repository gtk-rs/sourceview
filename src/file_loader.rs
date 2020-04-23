// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_source_sys;
use std::boxed::Box;
use std::ptr;
use FileLoader;

impl FileLoader
{
	pub fn load_async<
		P: IsA<gio::Cancellable>,
		Q: 'static + Fn(i64, i64) + Send,
		R: 'static + FnOnce(Result<bool, glib::Error>) + Send,
		S: 'static + FnOnce() + Send,
	>(
		&self,
		io_priority: glib::Priority,
		cancellable: Option<&P>,
		progress_callback: Q,
		ready_callback: R,
		notify_callback: S,
	)
	{
		let progress_user_data: Box<(Q, S)> = Box::new((progress_callback, notify_callback));
		unsafe extern "C" fn load_async_progress_trampoline<
			Q: 'static + Fn(i64, i64) + Send,
			S: 'static + FnOnce() + Send,
		>(
			current_num_bytes: i64,
			total_num_bytes: i64,
			user_data: glib_sys::gpointer,
		)
		{
			let callbacks: Box<(Q, S)> = Box::from_raw(user_data as *mut _);
			callbacks.0(current_num_bytes, total_num_bytes);
			Box::into_raw(callbacks); // Don't drop until 'notify' callback
		}
		let progress_callback = load_async_progress_trampoline::<Q, S>;

		let ready_user_data: Box<R> = Box::new(ready_callback);
		unsafe extern "C" fn load_async_ready_trampoline<
			R: 'static + FnOnce(Result<bool, glib::Error>) + Send,
		>(
			source_object: *mut gobject_sys::GObject,
			res: *mut gio_sys::GAsyncResult,
			user_data: glib_sys::gpointer,
		)
		{
			let mut error = ptr::null_mut();
			let ret = gtk_source_sys::gtk_source_file_loader_load_finish(
				source_object as *mut _,
				res,
				&mut error,
			);
			let result = if error.is_null()
			{
				Ok(ret != 0)
			}
			else
			{
				Err(from_glib_full(error))
			};
			let callback: Box<R> = Box::from_raw(user_data as *mut _);
			callback(result);
		}
		let ready_callback = load_async_ready_trampoline::<R>;

		unsafe extern "C" fn load_async_notify_trampoline<
			Q: 'static + Fn(i64, i64) + Send,
			S: 'static + FnOnce() + Send,
		>(
			user_data: glib_sys::gpointer,
		)
		{
			let callbacks: Box<(Q, S)> = Box::from_raw(user_data as *mut _);
			callbacks.1();
		}
		let notify_callback = load_async_notify_trampoline::<Q, S>;

		unsafe {
			gtk_source_sys::gtk_source_file_loader_load_async(
				self.to_glib_none().0,
				io_priority.to_glib(),
				cancellable.map(|p| p.as_ref()).to_glib_none().0,
				Some(progress_callback),
				Box::into_raw(progress_user_data) as *mut _,
				Some(notify_callback),
				Some(ready_callback),
				Box::into_raw(ready_user_data) as *mut _,
			);
		}
	}
}
