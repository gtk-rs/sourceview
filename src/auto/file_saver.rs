// This file was generated by gir (d121f7e) from gir-files (2e2a9ca)
// DO NOT EDIT

#[cfg(feature = "v3_14")]
use Buffer;
#[cfg(feature = "v3_14")]
use CompressionType;
#[cfg(feature = "v3_14")]
use Encoding;
#[cfg(feature = "v3_14")]
use File;
#[cfg(feature = "v3_14")]
use FileSaverFlags;
#[cfg(feature = "v3_14")]
use NewlineType;
use ffi;
#[cfg(feature = "v3_14")]
use gio;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct FileSaver(Object<ffi::GtkSourceFileSaver>);

    match fn {
        get_type => || ffi::gtk_source_file_saver_get_type(),
    }
}

impl FileSaver {
    #[cfg(feature = "v3_14")]
    pub fn new(buffer: &Buffer, file: &File) -> FileSaver {
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new(buffer.to_glib_none().0, file.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    pub fn new_with_target<P: IsA<gio::File>>(buffer: &Buffer, file: &File, target_location: &P) -> FileSaver {
        unsafe {
            from_glib_full(ffi::gtk_source_file_saver_new_with_target(buffer.to_glib_none().0, file.to_glib_none().0, target_location.to_glib_none().0))
        }
    }
}

pub trait FileSaverExt {
    #[cfg(feature = "v3_14")]
    fn get_buffer(&self) -> Option<Buffer>;

    #[cfg(feature = "v3_14")]
    fn get_compression_type(&self) -> CompressionType;

    #[cfg(feature = "v3_14")]
    fn get_encoding(&self) -> Option<Encoding>;

    #[cfg(feature = "v3_14")]
    fn get_file(&self) -> Option<File>;

    #[cfg(feature = "v3_14")]
    fn get_flags(&self) -> FileSaverFlags;

    #[cfg(feature = "v3_14")]
    fn get_location(&self) -> Option<gio::File>;

    #[cfg(feature = "v3_14")]
    fn get_newline_type(&self) -> NewlineType;

    //#[cfg(feature = "v3_14")]
    //fn save_async<'a, 'b, 'c, 'd, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, S: Into<Option<&'d /*Ignored*/gio::AsyncReadyCallback>>>(&self, io_priority: i32, cancellable: P, progress_callback: Q, progress_callback_data: /*Unimplemented*/Fundamental: Pointer, progress_callback_notify: R, callback: S, user_data: /*Unimplemented*/Fundamental: Pointer);

    //#[cfg(feature = "v3_14")]
    //fn save_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error>;

    #[cfg(feature = "v3_14")]
    fn set_compression_type(&self, compression_type: CompressionType);

    #[cfg(feature = "v3_14")]
    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P);

    #[cfg(feature = "v3_14")]
    fn set_flags(&self, flags: FileSaverFlags);

    #[cfg(feature = "v3_14")]
    fn set_newline_type(&self, newline_type: NewlineType);
}

impl<O: IsA<FileSaver>> FileSaverExt for O {
    #[cfg(feature = "v3_14")]
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_compression_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_encoding(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_file(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_flags(&self) -> FileSaverFlags {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_flags(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_source_file_saver_get_location(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::gtk_source_file_saver_get_newline_type(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_14")]
    //fn save_async<'a, 'b, 'c, 'd, P: Into<Option<&'a gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option<&'c /*Ignored*/glib::DestroyNotify>>, S: Into<Option<&'d /*Ignored*/gio::AsyncReadyCallback>>>(&self, io_priority: i32, cancellable: P, progress_callback: Q, progress_callback_data: /*Unimplemented*/Fundamental: Pointer, progress_callback_notify: R, callback: S, user_data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_source_file_saver_save_async() }
    //}

    //#[cfg(feature = "v3_14")]
    //fn save_finish<P: IsA</*Ignored*/gio::AsyncResult>>(&self, result: &P) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_source_file_saver_save_finish() }
    //}

    #[cfg(feature = "v3_14")]
    fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            ffi::gtk_source_file_saver_set_compression_type(self.to_glib_none().0, compression_type.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P) {
        let encoding = encoding.into();
        let encoding = encoding.to_glib_none();
        unsafe {
            ffi::gtk_source_file_saver_set_encoding(self.to_glib_none().0, encoding.0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_flags(&self, flags: FileSaverFlags) {
        unsafe {
            ffi::gtk_source_file_saver_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            ffi::gtk_source_file_saver_set_newline_type(self.to_glib_none().0, newline_type.to_glib());
        }
    }
}
