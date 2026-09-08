
use super::*;

#[test]
fn secondary_pointer_button_uses_context_menu_code() {
    assert_eq!(pointer_button_to_i16(PointerButton::Secondary), 2);
}
