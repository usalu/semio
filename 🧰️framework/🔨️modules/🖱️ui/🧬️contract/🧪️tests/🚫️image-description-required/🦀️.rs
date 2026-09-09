//! 🚫️ An image without an accessible description must not compile.
use semio_framework_ui_contract::*;

fn main() {
    let _ = image(UiText::try_from_str("atlas://logo").expect("valid image source")).try_build();
}
