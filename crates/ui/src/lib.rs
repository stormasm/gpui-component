mod colors;
mod event;
mod root;
mod styled;

pub mod animation;
pub mod button;
pub mod divider;
pub mod dock;
pub mod drawer;
pub mod history;
pub mod icon;
pub mod indicator;
pub mod input;
pub mod label;
pub mod list;
pub mod modal;
pub mod notification;
pub mod popover;
pub mod popup_menu;
pub mod resizable;
pub mod scroll;
pub mod tab;
pub mod theme;
pub mod tooltip;

pub use colors::*;
pub use icon::*;
pub use styled::*;

pub use root::{ContextModal, Root};

/// Initialize the UI module.
pub fn init(cx: &mut gpui::AppContext) {
    dock::init(cx);
    drawer::init(cx);
    modal::init(cx);
}

rust_i18n::i18n!("locales", fallback = "en");
use std::ops::Deref;
pub fn locale() -> impl Deref<Target = str> {
    rust_i18n::locale()
}

pub fn set_locale(locale: &str) {
    rust_i18n::set_locale(locale)
}
