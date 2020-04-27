// Copyright 2020, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use glib::object::IsA;
use glib::translate::*;
use gtk_source_sys;
use std::boxed::Box;
use std::ptr;
use FileLoader;

pub trait FileLoaderExtManual {
    fn load_async<
        P: IsA<gio::Cancellable>,
        Q: 'static + Fn(i64, i64) + Send,
        R: 'static + FnOnce(Result<(), glib::Error>) + Send,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        progress_callback: Q,
        ready_callback: R,
    );
}

impl<O: IsA<FileLoader>> FileLoaderExtManual for O {
    fn load_async<
        P: IsA<gio::Cancellable>,
        Q: 'static + Fn(i64, i64) + Send,
        R: 'static + FnOnce(Result<(), glib::Error>) + Send,
    >(
        &self,
        io_priority: glib::Priority,
        cancellable: Option<&P>,
        progress_callback: Q,
        ready_callback: R,
    ) {
        unsafe extern "C" fn load_async_progress_trampoline<Q: 'static + Fn(i64, i64) + Send>(
            current_num_bytes: i64,
            total_num_bytes: i64,
            user_data: glib_sys::gpointer,
        ) {
            let callback = &*(user_data as *const Q);
            callback(current_num_bytes, total_num_bytes);
        }

        unsafe extern "C" fn load_async_ready_trampoline<
            R: 'static + FnOnce(Result<(), glib::Error>) + Send,
        >(
            source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = gtk_source_sys::gtk_source_file_loader_load_finish(
                source_object as *mut _,
                res,
                &mut error,
            );
            let result = if !error.is_null() {
                Err(from_glib_full(error))
            } else if ret == glib_sys::GFALSE {
                Err(glib::Error::new(
                    glib::FileError::Failed,
                    "Unexpected failure",
                ))
            } else {
                Ok(())
            };
            let callback = Box::from_raw(user_data as *mut R);
            callback(result);
        }

        unsafe extern "C" fn load_async_notify_trampoline<Q: 'static + Fn(i64, i64) + Send>(
            user_data: glib_sys::gpointer,
        ) {
            Box::from_raw(user_data as *mut Q);
        }

        unsafe {
            gtk_source_sys::gtk_source_file_loader_load_async(
                self.as_ref().to_glib_none().0,
                io_priority.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(load_async_progress_trampoline::<Q>),
                Box::into_raw(Box::new(progress_callback)) as *mut _,
                Some(load_async_notify_trampoline::<Q>),
                Some(load_async_ready_trampoline::<R>),
                Box::into_raw(Box::new(ready_callback)) as *mut _,
            );
        }
    }
}
