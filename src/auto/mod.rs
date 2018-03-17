// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

mod buffer;
pub use self::buffer::Buffer;
pub use self::buffer::BufferExt;

mod completion;
pub use self::completion::Completion;
pub use self::completion::CompletionExt;

mod completion_context;
pub use self::completion_context::CompletionContext;
pub use self::completion_context::CompletionContextExt;

mod completion_info;
pub use self::completion_info::CompletionInfo;
pub use self::completion_info::CompletionInfoExt;

mod completion_item;
pub use self::completion_item::CompletionItem;
pub use self::completion_item::CompletionItemExt;

mod completion_proposal;
pub use self::completion_proposal::CompletionProposal;
pub use self::completion_proposal::CompletionProposalExt;

mod completion_provider;
pub use self::completion_provider::CompletionProvider;
pub use self::completion_provider::CompletionProviderExt;

mod completion_words;
pub use self::completion_words::CompletionWords;
pub use self::completion_words::CompletionWordsExt;

#[cfg(any(feature = "v3_14", feature = "dox"))]
mod file;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file::File;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file::FileExt;

#[cfg(any(feature = "v3_14", feature = "dox"))]
mod file_loader;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file_loader::FileLoader;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file_loader::FileLoaderExt;

#[cfg(any(feature = "v3_14", feature = "dox"))]
mod file_saver;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file_saver::FileSaver;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::file_saver::FileSaverExt;

mod gutter;
pub use self::gutter::Gutter;
pub use self::gutter::GutterExt;

mod gutter_renderer;
pub use self::gutter_renderer::GutterRenderer;
pub use self::gutter_renderer::GutterRendererExt;

mod gutter_renderer_pixbuf;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbuf;
pub use self::gutter_renderer_pixbuf::GutterRendererPixbufExt;

mod gutter_renderer_text;
pub use self::gutter_renderer_text::GutterRendererText;
pub use self::gutter_renderer_text::GutterRendererTextExt;

mod language;
pub use self::language::Language;
pub use self::language::LanguageExt;

mod language_manager;
pub use self::language_manager::LanguageManager;
pub use self::language_manager::LanguageManagerExt;

#[cfg(any(feature = "v3_18", feature = "dox"))]
mod map;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use self::map::Map;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use self::map::MapExt;

mod mark;
pub use self::mark::Mark;
pub use self::mark::MarkExt;

mod mark_attributes;
pub use self::mark_attributes::MarkAttributes;
pub use self::mark_attributes::MarkAttributesExt;

mod print_compositor;
pub use self::print_compositor::PrintCompositor;
pub use self::print_compositor::PrintCompositorExt;

#[cfg(any(feature = "v3_22", feature = "dox"))]
mod region;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use self::region::Region;
#[cfg(any(feature = "v3_22", feature = "dox"))]
pub use self::region::RegionExt;

#[cfg(any(feature = "v3_10", feature = "dox"))]
mod search_context;
#[cfg(any(feature = "v3_10", feature = "dox"))]
pub use self::search_context::SearchContext;
#[cfg(any(feature = "v3_10", feature = "dox"))]
pub use self::search_context::SearchContextExt;

#[cfg(any(feature = "v3_10", feature = "dox"))]
mod search_settings;
#[cfg(any(feature = "v3_10", feature = "dox"))]
pub use self::search_settings::SearchSettings;
#[cfg(any(feature = "v3_10", feature = "dox"))]
pub use self::search_settings::SearchSettingsExt;

#[cfg(any(feature = "v3_24", feature = "dox"))]
mod space_drawer;
#[cfg(any(feature = "v3_24", feature = "dox"))]
pub use self::space_drawer::SpaceDrawer;
#[cfg(any(feature = "v3_24", feature = "dox"))]
pub use self::space_drawer::SpaceDrawerExt;

mod style;
pub use self::style::Style;
pub use self::style::StyleExt;

mod style_scheme;
pub use self::style_scheme::StyleScheme;
pub use self::style_scheme::StyleSchemeExt;

#[cfg(any(feature = "v3_16", feature = "dox"))]
mod style_scheme_chooser;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use self::style_scheme_chooser::StyleSchemeChooser;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use self::style_scheme_chooser::StyleSchemeChooserExt;

#[cfg(any(feature = "v3_16", feature = "dox"))]
mod style_scheme_chooser_button;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use self::style_scheme_chooser_button::StyleSchemeChooserButton;

#[cfg(any(feature = "v3_16", feature = "dox"))]
mod style_scheme_chooser_widget;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use self::style_scheme_chooser_widget::StyleSchemeChooserWidget;

mod style_scheme_manager;
pub use self::style_scheme_manager::StyleSchemeManager;
pub use self::style_scheme_manager::StyleSchemeManagerExt;

#[cfg(any(feature = "v3_20", feature = "dox"))]
mod tag;
#[cfg(any(feature = "v3_20", feature = "dox"))]
pub use self::tag::Tag;
#[cfg(any(feature = "v3_20", feature = "dox"))]
pub use self::tag::TagExt;

mod undo_manager;
pub use self::undo_manager::UndoManager;
pub use self::undo_manager::UndoManagerExt;

mod view;
pub use self::view::View;
pub use self::view::ViewExt;

#[cfg(any(feature = "v3_14", feature = "dox"))]
mod encoding;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::encoding::Encoding;

mod enums;
#[cfg(any(feature = "v3_16", feature = "dox"))]
pub use self::enums::BackgroundPatternType;
#[cfg(any(feature = "v3_12", feature = "dox"))]
pub use self::enums::ChangeCaseType;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::enums::CompressionType;
pub use self::enums::GutterRendererAlignmentMode;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::enums::NewlineType;
pub use self::enums::SmartHomeEndType;

mod flags;
pub use self::flags::CompletionActivation;
pub use self::flags::DrawSpacesFlags;
#[cfg(any(feature = "v3_14", feature = "dox"))]
pub use self::flags::FileSaverFlags;
pub use self::flags::GutterRendererState;
#[cfg(any(feature = "v3_18", feature = "dox"))]
pub use self::flags::SortFlags;
#[cfg(any(feature = "v3_24", feature = "dox"))]
pub use self::flags::SpaceLocationFlags;
#[cfg(any(feature = "v3_24", feature = "dox"))]
pub use self::flags::SpaceTypeFlags;

#[doc(hidden)]
pub mod traits {
    pub use super::BufferExt;
    pub use super::CompletionExt;
    pub use super::CompletionContextExt;
    pub use super::CompletionInfoExt;
    pub use super::CompletionItemExt;
    pub use super::CompletionProposalExt;
    pub use super::CompletionProviderExt;
    pub use super::CompletionWordsExt;
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub use super::FileExt;
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub use super::FileLoaderExt;
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub use super::FileSaverExt;
    pub use super::GutterExt;
    pub use super::GutterRendererExt;
    pub use super::GutterRendererPixbufExt;
    pub use super::GutterRendererTextExt;
    pub use super::LanguageExt;
    pub use super::LanguageManagerExt;
    #[cfg(any(feature = "v3_18", feature = "dox"))]
    pub use super::MapExt;
    pub use super::MarkExt;
    pub use super::MarkAttributesExt;
    pub use super::PrintCompositorExt;
    #[cfg(any(feature = "v3_22", feature = "dox"))]
    pub use super::RegionExt;
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub use super::SearchContextExt;
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub use super::SearchSettingsExt;
    #[cfg(any(feature = "v3_24", feature = "dox"))]
    pub use super::SpaceDrawerExt;
    pub use super::StyleExt;
    pub use super::StyleSchemeExt;
    #[cfg(any(feature = "v3_16", feature = "dox"))]
    pub use super::StyleSchemeChooserExt;
    pub use super::StyleSchemeManagerExt;
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    pub use super::TagExt;
    pub use super::UndoManagerExt;
    pub use super::ViewExt;
}
