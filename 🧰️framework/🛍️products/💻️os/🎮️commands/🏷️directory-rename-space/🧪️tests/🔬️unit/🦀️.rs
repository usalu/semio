
use super::*;

#[test]
fn directory_rename_space_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.rename-space");
    assert_eq!(LABEL_EN, "Rename Space");
    assert_eq!(LABEL_DE, "Space umbenennen");
}
