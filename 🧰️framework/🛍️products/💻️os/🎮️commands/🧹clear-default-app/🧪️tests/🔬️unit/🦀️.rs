
use super::*;

#[test]
fn clear_default_app_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.clear-default-app");
    assert_eq!(LABEL_EN, "Clear Default App");
    assert_eq!(LABEL_DE, "Standard-App zurücksetzen");
}
