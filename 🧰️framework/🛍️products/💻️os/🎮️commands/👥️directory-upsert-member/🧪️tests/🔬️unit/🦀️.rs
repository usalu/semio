
use super::*;

#[test]
fn directory_upsert_member_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.upsert-member");
    assert_eq!(LABEL_EN, "Add or Update Member");
    assert_eq!(LABEL_DE, "Mitglied hinzufügen oder aktualisieren");
}
