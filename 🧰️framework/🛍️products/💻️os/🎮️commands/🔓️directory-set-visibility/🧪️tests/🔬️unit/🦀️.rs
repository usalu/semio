
use super::*;

#[test]
fn directory_set_visibility_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.set-visibility");
    assert_eq!(LABEL_EN, "Set Visibility");
    assert_eq!(LABEL_DE, "Sichtbarkeit festlegen");
}
