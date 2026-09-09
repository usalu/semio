//! 🖼️ Described and decorative images can both be built.
use semio_framework_ui_contract::*;

fn main() {
    assert!(image(UiText::try_from_str("atlas://logo").expect("valid image source"))
        .alt(Label::try_from("Company logo").expect("valid image label"))
        .try_build()
        .is_ok());
    assert!(image(UiText::try_from_str("atlas://deco").expect("valid image source"))
        .decorative()
        .try_build()
        .is_ok());
}
