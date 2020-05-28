use crate::CompletionActivation;
use crate::CompletionContext;
use crate::CompletionInfo;
use crate::CompletionProposal;
use crate::CompletionProvider;
use glib::prelude::*;
use glib::subclass::prelude::*;
use glib::translate::*;
use once_cell::sync::Lazy;

pub trait CompletionProviderImpl: ObjectImpl + Send + 'static {
    fn get_name(&self, obj: &CompletionProvider) -> glib::GString;
    fn get_icon(&self, obj: &CompletionProvider) -> Option<gdk_pixbuf::Pixbuf>;
    fn get_icon_name(&self, obj: &CompletionProvider) -> Option<glib::GString>;
    fn get_gicon(&self, obg: &CompletionProvider) -> Option<gio::Icon>;
    fn populate(&self, obg: &CompletionProvider, context: &CompletionContext);
    fn get_activation(&self, obg: &CompletionProvider) -> CompletionActivation;
    fn provide_match(&self, obg: &CompletionProvider, context: &CompletionContext) -> bool;
    fn get_info_widget(
        &self,
        obg: &CompletionProvider,
        proposal: &CompletionProposal,
    ) -> Option<gtk::Widget>;
    fn update_info(
        &self,
        obg: &CompletionProvider,
        proposal: &CompletionProposal,
        info: &CompletionInfo,
    );
    fn get_start_iter(
        &self,
        obg: &CompletionProvider,
        context: &CompletionContext,
        proposal: &CompletionProposal,
        iter: &gtk::TextIter,
    ) -> bool;
    fn activate_proposal(
        &self,
        obg: &CompletionProvider,
        proposal: &CompletionProposal,
        iter: &gtk::TextIter,
    ) -> bool;
    fn get_interactive_delay(&self, obj: &CompletionProvider) -> i32;
    fn get_priority(&self, obg: &CompletionProvider) -> i32;
}

unsafe impl<T: ObjectSubclass + CompletionProviderImpl> IsImplementable<T> for CompletionProvider {
    unsafe extern "C" fn interface_init(
        iface: glib::glib_sys::gpointer,
        _iface_data: glib::glib_sys::gpointer,
    ) {
        let iface = &mut *(iface as *mut gtk_source_sys::GtkSourceCompletionProviderIface);
        iface.get_name = Some(completion_provider_get_name::<T>);
        iface.get_icon = Some(completion_provider_get_icon::<T>);
        iface.get_icon_name = Some(completion_provider_get_icon_name::<T>);
        iface.get_gicon = Some(completion_provider_get_gicon::<T>);
        iface.populate = Some(completion_provider_populate::<T>);
        iface.get_activation = Some(completion_provider_get_activation::<T>);
        iface.match_ = Some(completion_provider_provide_match::<T>);
        iface.get_info_widget = Some(completion_provider_get_info_widget::<T>);
        iface.update_info = Some(completion_provider_update_info::<T>);
        iface.get_start_iter = Some(completion_provider_get_start_iter::<T>);
        iface.activate_proposal = Some(completion_provider_activate_proposal::<T>);
        iface.get_interactive_delay = Some(completion_provider_get_interactive_delay::<T>);
        iface.get_priority = Some(completion_provider_get_priority::<T>);
    }
}

unsafe extern "C" fn completion_provider_get_name<T: ObjectSubclass + CompletionProviderImpl>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> *const libc::c_char {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_name(&from_glib_borrow(completion_provider))
        .to_glib_full()
}

static COMPLETION_PROVIDER_ICON_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-sourceview-subclass-icon"));

unsafe extern "C" fn completion_provider_get_icon<T: ObjectSubclass + CompletionProviderImpl>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> *mut gdk_pixbuf_sys::GdkPixbuf {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.get_icon(&from_glib_borrow(completion_provider));
    let ret_ptr = match ret {
        Some(ref ret) => ret.as_ptr(),
        None => std::ptr::null_mut(),
    };

    let old_ptr = gobject_sys::g_object_get_qdata(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_ICON_QUARK.to_glib(),
    );
    if !old_ptr.is_null() {
        assert_eq!(old_ptr as *mut _, ret_ptr, "Did not return same icon again");
    }

    gobject_sys::g_object_set_qdata_full(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_ICON_QUARK.to_glib(),
        ret_ptr as *mut libc::c_void,
        Some(unref),
    );

    ret.to_glib_none().0
}

static COMPLETION_PROVIDER_ICON_NAME_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-sourceview-subclass-icon-name"));

unsafe extern "C" fn completion_provider_get_icon_name<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> *const libc::c_char {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.get_icon_name(&from_glib_borrow(completion_provider));
    let ret_ptr = match ret {
        Some(ref ret) => ret.as_ptr(),
        None => std::ptr::null(),
    };

    let old_ptr = gobject_sys::g_object_get_qdata(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_ICON_NAME_QUARK.to_glib(),
    );
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *const _, ret_ptr,
            "Did not return same icon name again"
        );
    }

    gobject_sys::g_object_set_qdata_full(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_ICON_NAME_QUARK.to_glib(),
        ret_ptr as *mut libc::c_void,
        Some(unref),
    );

    ret.to_glib_none().0
}

static COMPLETION_PROVIDER_GICON_QUARK: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-sourceview-subclass-gicon"));

unsafe extern "C" fn completion_provider_get_gicon<T: ObjectSubclass + CompletionProviderImpl>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> *mut gio_sys::GIcon {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.get_gicon(&from_glib_borrow(completion_provider));
    let ret_ptr = match ret {
        Some(ref ret) => ret.as_ptr(),
        None => std::ptr::null_mut(),
    };

    let old_ptr = gobject_sys::g_object_get_qdata(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_GICON_QUARK.to_glib(),
    );
    if !old_ptr.is_null() {
        assert_eq!(
            old_ptr as *mut _, ret_ptr,
            "Did not return same gicon again"
        );
    }

    gobject_sys::g_object_set_qdata_full(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_GICON_QUARK.to_glib(),
        ret_ptr as *mut libc::c_void,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn completion_provider_populate<T: ObjectSubclass + CompletionProviderImpl>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    context: *mut gtk_source_sys::GtkSourceCompletionContext,
) {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.populate(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(context),
    )
}

unsafe extern "C" fn completion_provider_get_activation<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> gtk_source_sys::GtkSourceCompletionActivation {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_activation(&from_glib_borrow(completion_provider))
        .to_glib()
}

unsafe extern "C" fn completion_provider_provide_match<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    context: *mut gtk_source_sys::GtkSourceCompletionContext,
) -> glib_sys::gboolean {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.provide_match(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(context),
    )
    .to_glib()
}

static COMPLETION_PROVIDER_INFO_WIDGET: Lazy<glib::Quark> =
    Lazy::new(|| glib::Quark::from_string("gtk-sourceview-subclass-info-widget"));

unsafe extern "C" fn completion_provider_get_info_widget<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    proposal: *mut gtk_source_sys::GtkSourceCompletionProposal,
) -> *mut gtk_sys::GtkWidget {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();

    let ret = imp.get_info_widget(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(proposal),
    );
    let ret_ptr = match ret {
        Some(ref ret) => ret.as_ptr(),
        None => std::ptr::null_mut(),
    };

    let old_ptr = gobject_sys::g_object_get_qdata(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_INFO_WIDGET.to_glib(),
    );
    if !old_ptr.is_null() {
        //assert_eq!(old_ptr as *mut _, ret_ptr, "Did not return same info widget again");
    }

    gobject_sys::g_object_set_qdata_full(
        completion_provider as *mut _,
        COMPLETION_PROVIDER_INFO_WIDGET.to_glib(),
        ret_ptr as *mut libc::c_void,
        Some(unref),
    );

    ret.to_glib_none().0
}

unsafe extern "C" fn completion_provider_update_info<T: ObjectSubclass + CompletionProviderImpl>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    proposal: *mut gtk_source_sys::GtkSourceCompletionProposal,
    completion_info: *mut gtk_source_sys::GtkSourceCompletionInfo,
) {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.update_info(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(proposal),
        &from_glib_borrow(completion_info),
    )
}

unsafe extern "C" fn completion_provider_get_start_iter<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    context: *mut gtk_source_sys::GtkSourceCompletionContext,
    proposal: *mut gtk_source_sys::GtkSourceCompletionProposal,
    iter: *mut gtk_sys::GtkTextIter,
) -> glib_sys::gboolean {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_start_iter(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(context),
        &from_glib_borrow(proposal),
        &from_glib_borrow(iter),
    )
    .to_glib()
}

unsafe extern "C" fn completion_provider_activate_proposal<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
    proposal: *mut gtk_source_sys::GtkSourceCompletionProposal,
    iter: *mut gtk_sys::GtkTextIter,
) -> glib_sys::gboolean {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.activate_proposal(
        &from_glib_borrow(completion_provider),
        &from_glib_borrow(proposal),
        &from_glib_borrow(iter),
    )
    .to_glib()
}

unsafe extern "C" fn completion_provider_get_interactive_delay<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> libc::c_int {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_interactive_delay(&from_glib_borrow(completion_provider))
}

unsafe extern "C" fn completion_provider_get_priority<
    T: ObjectSubclass + CompletionProviderImpl,
>(
    completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider,
) -> libc::c_int {
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_priority(&from_glib_borrow(completion_provider))
}

unsafe extern "C" fn unref(ptr: glib_sys::gpointer) {
    gobject_sys::g_object_unref(ptr as *mut _);
}
