
use super::*;

#[test]
fn directory_remove_member_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.remove-member");
    assert_eq!(LABEL_EN, "Remove Member");
    assert_eq!(LABEL_DE, "Mitglied entfernen");
}
