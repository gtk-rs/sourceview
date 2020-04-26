// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_source_sys;
use std::boxed::Box;
use std::ptr;
use FileSaver;

pub trait FileSaverExtManual {
    fn save_async<
        P: IsA<gio::Cancellable>,
        Q: 'static + Fn(i64, i64) + Send,
        R: 'static + Fn(Result<bool, glib::Error>) + Send,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        progress_callback: Q,
        ready_callback: R,
    );
}

impl<O: IsA<FileSaver>> FileSaverExtManual for O {
    fn save_async<
        P: IsA<gio::Cancellable>,
        Q: 'static + Fn(i64, i64) + Send,
        R: 'static + Fn(Result<bool, glib::Error>) + Send,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        progress_callback: Q,
        ready_callback: R,
    ) {
        unsafe extern "C" fn save_async_progress_trampoline<
            Q: 'static + Fn(i64, i64) + Send,
            R: 'static + Fn(Result<bool, glib::Error>) + Send,
        >(
            current_num_bytes: i64,
            total_num_bytes: i64,
            user_data: glib_sys::gpointer,
        ) {
            let callbacks = &*(user_data as *mut (Q, R));
            callbacks.0(current_num_bytes, total_num_bytes);
        }

        unsafe extern "C" fn save_async_ready_trampoline<
            Q: 'static + Fn(i64, i64) + Send,
            R: 'static + Fn(Result<bool, glib::Error>) + Send,
        >(
            source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gtk_source_sys::gtk_source_file_saver_save_finish(
                source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(ret != 0)
            } else {
                Err(from_glib_full(error))
            };
            let callbacks = &*(user_data as *mut (Q, R));
            callbacks.1(result);
        }

        unsafe extern "C" fn save_async_notify_trampoline<
            Q: 'static + Fn(i64, i64) + Send,
            R: 'static + Fn(Result<bool, glib::Error>) + Send,
        >(
            user_data: glib_sys::gpointer,
        ) {
            Box::from_raw(user_data as *mut _);
        }

        let callbacks = Box::new((progress_callback, ready_callback));
        let user_data = Box::into_raw(callbacks) as *mut _;
        unsafe {
            gtk_source_sys::gtk_source_file_saver_save_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(save_async_progress_trampoline::<Q, R>),
                user_data,
                Some(save_async_notify_trampoline::<Q, R>),
                Some(save_async_ready_trampoline::<Q, R>),
                user_data,
            );
        }
    }
}
