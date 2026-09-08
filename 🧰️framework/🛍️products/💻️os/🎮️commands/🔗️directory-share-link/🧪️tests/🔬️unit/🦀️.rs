
use super::*;

#[test]
fn directory_share_link_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.directory.share-link");
    assert_eq!(LABEL_EN, "Share Link");
    assert_eq!(LABEL_DE, "Link teilen");
}
