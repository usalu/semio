
use super::*;

#[test]
fn open_artifact_id_and_labels_are_frozen() {
    assert_eq!(ID, "os.open-artifact");
    assert_eq!(LABEL_EN, "Open Artifact");
    assert_eq!(LABEL_DE, "Artefakt öffnen");
}
