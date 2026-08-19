//! 📦️ Note artifact — binary document surface + laws (constitutional: pack).

use crate::artifacts::note::NoteSnapshot;
use store::PackError;

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️component.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️component.protocol.semio");
//#endregion 📡️SemioProtocol


/// 📦️ Encodes a `NoteSnapshot` to its binary pack form.
pub async fn encode(document: &NoteSnapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(document)
}

/// 📖️ Decodes a `NoteSnapshot` from its binary pack form.
pub async fn decode(bytes: &[u8]) -> Result<NoteSnapshot, PackError> {
    <NoteSnapshot as store::ArtifactPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::note::{NoteBlockNode, NoteImageAsset, NoteTableCell, NoteTextParagraph, NoteTextRun, NOTE_DOCUMENT_SCHEMA};
    use std::collections::BTreeMap;

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_and_agrees_with_dsl() {
        let document = crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[semio_framework_async_macros::async_test]
    async fn pack_round_trips_representative_document() {
        let mut assets = BTreeMap::new();
        assets.insert("asset-1".into(), NoteImageAsset { mime: "image/png".into(), data: "data:image/png;base64,abc==".into(), width: Some(10.0), height: Some(20.0) });
        let document = NoteSnapshot {
            schema: NOTE_DOCUMENT_SCHEMA.into(),
            id: "doc-1".into(),
            title: Some("Representative \"Doc\"".into()),
            grid_visible: Some(true),
            grid_spacing: Some(32.0),
            grid_subdivisions: None,
            grid_opacity: Some(0.35),
            snap_enabled: None,
            snap_grid_spacing: Some(8.0),
            pencil_width: Some(3.0),
            eraser_radius: None,
            assets,
            linked_artifact: None,
            blocks: vec![
                NoteBlockNode::Text {
                    content: crate::artifacts::note::note_text_child_handle_and_cache(
                        "text-1",
                        &[NoteTextParagraph { runs: vec![NoteTextRun { text: "plain".into(), bold: None, italic: None, underline: None, link: None }] }],
                    ),
                    id: "text-1".into(),
                    name: "Text".into(),
                    x: 0.0,
                    y: 0.0,
                    width: 200.0,
                    height: 80.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    font_size: 16.0,
                    font_weight: "bold".into(),
                    align: "center".into(),
                },
                NoteBlockNode::Table {
                    id: "table-1".into(),
                    name: "Table".into(),
                    x: 20.0,
                    y: 20.0,
                    width: 320.0,
                    height: 120.0,
                    rotation: 0.0,
                    visible: true,
                    locked: false,
                    columns: vec!["A".into(), "B".into()],
                    rows: vec![vec![NoteTableCell { content: "a1".into() }, NoteTableCell { content: "b1".into() }]],
                },
            ],
        };
        store::os_store::test_support::assert_dsl_pack_equivalence(&document);
    }

    //#region 🔖️CommandEnvelopeTests
    /// 🎫️ CW7 command-envelope law (`POLICY_COMMAND_ENVELOPE_COMPLETENESS_ALLOWLIST`): proves
    /// `NoteMutation`'s `Edit` round-trips through `protocol::MutationEnvelope`s beside this file's
    /// existing pack round-trip laws (same pattern as `mathematical_pack`'s own
    /// `command_envelope_round_trip_holds_for_an_applied_operation`).
    #[semio_framework_async_macros::async_test]
    async fn command_envelope_round_trip_holds_for_an_applied_operation() {
        use crate::artifacts::note::standards::v1::subsets::any::io::mutations::text::NoteMutation;
        use protocol::{ArtifactId, Edit, SchemaId};
        use store::{create_document_envelope, ArtifactCommand, ArtifactStore};

        let initial = crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse semio example");
        let envelope = create_document_envelope::<NoteSnapshot, NoteMutation>(NOTE_DOCUMENT_SCHEMA, "note-command-envelope-demo", initial, None);
        let mut store = ArtifactStore::new(envelope).expect("valid artifact store fixture");
        store.dispatch(ArtifactCommand::Apply { mutations: vec![crate::artifacts::note::schema::mutations::change_grid_visible(Some(false))], description: None }).expect("apply");
        let edit: &Edit<NoteMutation> = store.envelope().vcs.edits.last().expect("dispatch must have recorded an edit");
        store::os_store::test_support::assert_command_envelope_round_trip::<NoteSnapshot, NoteMutation>(edit, &ArtifactId(store.envelope().id.clone()), &SchemaId(store.envelope().schema.clone()));
    }
    //#endregion 🔖️CommandEnvelopeTests
}
//#endregion 🧪️Tests

#[cfg(test)]
mod semio_protocol_conformance {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn component_protocol_semio_is_protocol_dialect() {
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol.semio");
        assert_eq!(g.dialect, ::dsl::SemioDialect::Protocol);
        assert!(!COMPONENT_PROTOCOL_SEMIO.is_empty());
        let _ = COMPONENT_PROTOCOL_PATH;
    }

    #[semio_framework_async_macros::async_test]
    async fn verify_protocol_bytes_against_encoded_pack() {
        let document = crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::parse_dsl(crate::artifacts::note::standards::v1::subsets::any::io::snapshot::text::SEMIO_NOTE_EXAMPLE_TEXT).expect("parse fixture");
        let bytes = encode(&document);
        let g = ::dsl::parse_grammar(COMPONENT_PROTOCOL_SEMIO).expect("parse protocol");
        ::dsl::verify_protocol_bytes(&g, &bytes).expect("protocol recognizes pack bytes");
    }
}
