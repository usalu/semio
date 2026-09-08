
use super::*;

#[test]
fn directory_create_space_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.create-space");
    assert_eq!(LABEL_EN, "Create Space");
    assert_eq!(LABEL_DE, "Space erstellen");
}
