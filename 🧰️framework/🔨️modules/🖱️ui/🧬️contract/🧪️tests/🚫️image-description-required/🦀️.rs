//! 🚫️ An image without an accessible description must not compile.
use semio_framework_ui_contract_builder::*;

fn main() {
    let _ = image(UiText::try_from_str("atlas://logo").expect("valid image source")).try_build();
}
