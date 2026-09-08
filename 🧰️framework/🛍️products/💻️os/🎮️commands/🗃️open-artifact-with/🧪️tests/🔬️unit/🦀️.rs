
use super::*;

#[test]
fn open_artifact_with_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.open-artifact-with");
    assert_eq!(LABEL_EN, "Open With…");
    assert_eq!(LABEL_DE, "Öffnen mit…");
}
