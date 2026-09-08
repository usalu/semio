
use super::*;

#[test]
fn directory_delete_space_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.delete-space");
    assert_eq!(LABEL_EN, "Delete Space");
    assert_eq!(LABEL_DE, "Space löschen");
}
