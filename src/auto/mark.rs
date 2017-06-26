// This file was generated by gir (3c73dd9+) from gir-files (2e2a9ca)
// DO NOT EDIT

use ffi;
use glib;
use glib::Value;
use glib::object::IsA;
use glib::translate::*;
use gobject_ffi;
use gtk;
use gtk_ffi;

glib_wrapper! {
    pub struct Mark(Object<ffi::GtkSourceMark>): [
        gtk::TextMark => gtk_ffi::GtkTextMark,
    ];

    match fn {
        get_type => || ffi::gtk_source_mark_get_type(),
    }
}

impl Mark {
    pub fn new(name: &str, category: &str) -> Mark {
        unsafe {
            from_glib_full(ffi::gtk_source_mark_new(name.to_glib_none().0, category.to_glib_none().0))
        }
    }
}

pub trait MarkExt {
    fn get_category(&self) -> Option<String>;

    fn next<'a, P: Into<Option<&'a str>>>(&self, category: P) -> Option<Mark>;

    fn prev(&self, category: &str) -> Option<Mark>;

    fn get_property_category(&self) -> Option<String>;
}

impl<O: IsA<Mark> + IsA<glib::object::Object>> MarkExt for O {
    fn get_category(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_get_category(self.to_glib_none().0))
        }
    }

    fn next<'a, P: Into<Option<&'a str>>>(&self, category: P) -> Option<Mark> {
        let category = category.into();
        let category = category.to_glib_none();
        unsafe {
            from_glib_none(ffi::gtk_source_mark_next(self.to_glib_none().0, category.0))
        }
    }

    fn prev(&self, category: &str) -> Option<Mark> {
        unsafe {
            from_glib_none(ffi::gtk_source_mark_prev(self.to_glib_none().0, category.to_glib_none().0))
        }
    }

    fn get_property_category(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "category".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }
}
