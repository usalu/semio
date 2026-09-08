
use super::*;

#[test]
fn set_default_viewer_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.set-default-viewer");
    assert_eq!(LABEL_EN, "Set Default Viewer");
    assert_eq!(LABEL_DE, "Standard-Betrachter festlegen");
}
