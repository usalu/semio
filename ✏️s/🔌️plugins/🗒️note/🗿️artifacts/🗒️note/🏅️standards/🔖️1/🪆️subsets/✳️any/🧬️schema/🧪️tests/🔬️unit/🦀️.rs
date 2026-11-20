#[allow(unused_imports)]
use crate::{NoteTableCell, NoteTextParagraph, NoteTextRun};
#[allow(unused_imports)]
use framework_schema::ArtifactSchema;
#[allow(unused_imports)]
use semio_framework_value_derive::{FromValue, ToValue};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::collections::BTreeMap;
use crate::standards::v1::subsets::any::schema::*;

/// 🧪️ Relocated from the deleted `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES).
#[semio_framework_async_macros::async_test]
async fn clone_block_reids_group_children() {
    let mut ids = NoteIdOwner::new("schema-clone-test", 0);
    let child = create_block_by_kind(&mut ids, "text", 0.0, 0.0);
    let child_id = block_id(&child).to_string();
    let group = NoteBlockNode::Group { id: "group-1".into(), name: "Group".into(), x: 0.0, y: 0.0, width: 100.0, height: 100.0, rotation: 0.0, visible: true, locked: false, children: vec![child] };
    let cloned = clone_block(&mut ids, &group);
    if let NoteBlockNode::Group { children, .. } = &cloned {
        assert_ne!(block_id(&children[0]), child_id);
    } else {
        panic!("expected group block");
    }
}

#[test]
fn note_snapshot_asset_map_preserves_the_committed_wire_object() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document/🔣️.json")).expect("note document fixture");
    let document = &fixture["document"];
    let expected: BTreeMap<String, NoteImageAsset> = serde_json::from_value(document["assets"].clone()).expect("serde asset oracle");
    let snapshot = <NoteSnapshot as semio_framework_value::FromValue>::from_value(serde_json::from_value(document.clone()).expect("note value")).expect("note snapshot");
    assert_eq!(snapshot.assets.iter().collect::<BTreeMap<_, _>>(), expected.iter().collect());
    assert_eq!(serde_json::to_value(&snapshot.assets).expect("retained assets"), serde_json::to_value(&expected).expect("serde assets"));
}
