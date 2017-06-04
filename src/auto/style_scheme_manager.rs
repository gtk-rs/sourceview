// This file was generated by gir (fdeaa47) from gir-files (2e2a9ca)
// DO NOT EDIT

use StyleScheme;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct StyleSchemeManager(Object<ffi::GtkSourceStyleSchemeManager>);

    match fn {
        get_type => || ffi::gtk_source_style_scheme_manager_get_type(),
    }
}

impl StyleSchemeManager {
    pub fn new() -> StyleSchemeManager {
        unsafe {
            from_glib_full(ffi::gtk_source_style_scheme_manager_new())
        }
    }

    pub fn get_default() -> Option<StyleSchemeManager> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_default())
        }
    }
}

pub trait StyleSchemeManagerExt {
    fn append_search_path(&self, path: &str);

    fn force_rescan(&self);

    fn get_scheme(&self, scheme_id: &str) -> Option<StyleScheme>;

    fn get_scheme_ids(&self) -> Vec<String>;

    fn get_search_path(&self) -> Vec<String>;

    fn prepend_search_path(&self, path: &str);

    fn set_search_path(&self, path: &[&str]);
}

impl<O: IsA<StyleSchemeManager>> StyleSchemeManagerExt for O {
    fn append_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_append_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn force_rescan(&self) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_force_rescan(self.to_glib_none().0);
        }
    }

    fn get_scheme(&self, scheme_id: &str) -> Option<StyleScheme> {
        unsafe {
            from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme(self.to_glib_none().0, scheme_id.to_glib_none().0))
        }
    }

    fn get_scheme_ids(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_manager_get_scheme_ids(self.to_glib_none().0))
        }
    }

    fn get_search_path(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gtk_source_style_scheme_manager_get_search_path(self.to_glib_none().0))
        }
    }

    fn prepend_search_path(&self, path: &str) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_prepend_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }

    fn set_search_path(&self, path: &[&str]) {
        unsafe {
            ffi::gtk_source_style_scheme_manager_set_search_path(self.to_glib_none().0, path.to_glib_none().0);
        }
    }
}
