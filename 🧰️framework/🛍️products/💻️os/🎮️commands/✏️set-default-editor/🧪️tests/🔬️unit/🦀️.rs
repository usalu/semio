
use super::*;

#[test]
fn set_default_editor_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.set-default-editor");
    assert_eq!(LABEL_EN, "Set Default Editor");
    assert_eq!(LABEL_DE, "Standard-Editor festlegen");
}
