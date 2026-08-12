//! 📦️ Playbook artifact — binary document surface + laws (constitutional: pack).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifacts::playbook::PlaybookSnapshot;
use store::PackError;

/// 📦️ Encodes a `PlaybookSnapshot` to its binary pack form.
pub fn encode(document: &PlaybookSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `PlaybookSnapshot` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<PlaybookSnapshot, PackError> {
    <PlaybookSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::playbook::empty_playbook_snapshot;
    use crate::artifacts::playbook::{dsl, PLAYBOOK_DOCUMENT_SCHEMA};

    #[test]
    fn pack_round_trips_the_empty_snapshot() {
        let document = empty_playbook_snapshot();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn facade_generator_example_pack_round_trips() {
        let document = empty_playbook_snapshot();
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn facade_generator_example_pack_agrees_with_dsl() {
        let document = empty_playbook_snapshot();
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

  #[test]
    fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::playbook::op::{change_title_operation, PlaybookMutation};
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let mut store: ArtifactStore<PlaybookSnapshot, PlaybookMutation> = ArtifactStore::new(create_document_envelope(PLAYBOOK_DOCUMENT_SCHEMA, "playbook-demo", empty_playbook_snapshot(), None));
        store.dispatch(ArtifactCommand::Apply { mutations: vec![change_title_operation(Some("Recipe".into()))], description: None }).expect("apply");
        let edit: &Edit<PlaybookMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<PlaybookSnapshot, PlaybookMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
}
//#endregion 🧪️Tests
