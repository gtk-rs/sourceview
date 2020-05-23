use crate::CompletionActivation;
use crate::CompletionContext;
use crate::CompletionInfo;
use crate::CompletionProposal;
use crate::CompletionProvider;
use glib::prelude::*;
use glib::translate::*;
use glib::subclass::prelude::*;

pub trait CompletionProviderImpl: ObjectImpl + Send + 'static {
    fn get_name(&self, obj: &CompletionProvider) -> glib::GString;
    fn get_icon(&self, obj: &CompletionProvider) -> Option<gdk_pixbuf::Pixbuf>;
    fn get_icon_name(&self, obj: &CompletionProvider) -> Option<glib::GString>;
    fn get_gicon(&self, obg: &CompletionProvider) -> Option<gio_sys::GIcon>;
    fn populate(&self, obg: &CompletionProvider, context: &CompletionContext);
    fn get_activation(&self, obg: &CompletionProvider) -> CompletionActivation;
    fn provider_match(&self, obg: &CompletionProvider, context: &CompletionContext) -> bool;
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
    unsafe extern "C" fn interface_init(iface: glib::glib_sys::gpointer, _iface_data: glib::glib_sys::gpointer) {
        let iface = &mut *(iface as *mut gtk_source_sys::GtkSourceCompletionProviderIface);
        iface.get_name = Some(completion_provider_get_name::<T>);
    }
}

unsafe extern "C" fn completion_provider_get_name<T: ObjectSubclass + CompletionProviderImpl>(completion_provider: *mut gtk_source_sys::GtkSourceCompletionProvider) -> *const libc::c_char{
    let instance = &*(completion_provider as *mut T::Instance);
    let imp = instance.get_impl();
    imp.get_name(&from_glib_borrow(completion_provider)).to_glib_full()
}