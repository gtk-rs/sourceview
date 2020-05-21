use crate::CompletionActivation;
use crate::CompletionContext;
use crate::CompletionInfo;
use crate::CompletionProposal;
use crate::CompletionProvider;
use glib::subclass::prelude::*;
use gtk_source_sys::gtk_source_completion_provider_get_name;

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

