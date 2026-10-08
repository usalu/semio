use super::*;
use crate::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked};
use semio_framework_value::{
    SnapshotRetirementStep,
    retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep},
    retirement::owned_retirement,
};
use std::sync::Arc;

fn close_cursor(cursor: &mut <RetainedXmlDocument as RetainedClone>::Cursor, grant: RetainedCloneGrant, maximum_turns: usize) {
    assert!(cursor.begin_close());
    for turn in 0..maximum_turns {
        match cursor.close_step(grant.maximum_items, grant.maximum_capacity_bytes).expect("retained XML cursor close") {
            SnapshotRetirementStep::Complete => {
                assert!(cursor.terminal_is_empty());
                return;
            }
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items);
                assert!(released_bytes <= grant.maximum_capacity_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("retained XML cursor blocked at turn {turn}"),
        }
    }
    panic!("retained XML cursor close exceeded fixture turn bound");
}

#[test]
fn flat_retained_xml_copy_materialize_and_retire_preserve_shared_vocabulary() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("retained XML fixture");
    let source = xml_document_from_text(fixture["xml"].as_str().expect("retained XML text")).expect("shared XML parser accepts retained fixture");
    let retained = RetainedXmlDocument::try_from_document(&source).expect("flat retained XML owner");
    assert_eq!(retained.nodes.len(), fixture["expected"]["nodes"].as_u64().unwrap() as usize);
    assert_eq!(retained.attributes.len(), fixture["expected"]["attributes"].as_u64().unwrap() as usize);
    assert_eq!(retained.prolog.len(), fixture["expected"]["prolog"].as_u64().unwrap() as usize);
    assert_eq!(retained.epilog.len(), fixture["expected"]["epilog"].as_u64().unwrap() as usize);
    assert_eq!(retained.doctype.as_ref().expect("retained doctype").declarations.len(), fixture["expected"]["dtdDeclarations"].as_u64().unwrap() as usize);
    let mut progress = |_| true;
    let materialized = retained.materialize(&mut NativeEncodeControl::new(4 * 1024 * 1024, &mut progress)).expect("controlled retained XML materialization");
    assert_eq!(materialized, source);
    let emitted = xml_document_to_text_checked(&materialized).expect("shared XML writer emits retained fixture");

    let mut reader = quick_xml::Reader::from_str(&emitted);
    reader.config_mut().trim_text(false);
    let mut observed = serde_json::json!({"elements":0,"text":0,"cdata":0,"comments":0,"processingInstructions":0,"declarations":0,"doctypes":0});
    loop {
        match reader.read_event().expect("quick-xml independently parses retained emission") {
            quick_xml::events::Event::Start(_) | quick_xml::events::Event::Empty(_) => observed["elements"] = serde_json::json!(observed["elements"].as_u64().unwrap() + 1),
            quick_xml::events::Event::Text(value) if !value.as_ref().iter().all(u8::is_ascii_whitespace) => observed["text"] = serde_json::json!(observed["text"].as_u64().unwrap() + 1),
            quick_xml::events::Event::CData(_) => observed["cdata"] = serde_json::json!(observed["cdata"].as_u64().unwrap() + 1),
            quick_xml::events::Event::Comment(_) => observed["comments"] = serde_json::json!(observed["comments"].as_u64().unwrap() + 1),
            quick_xml::events::Event::PI(_) => observed["processingInstructions"] = serde_json::json!(observed["processingInstructions"].as_u64().unwrap() + 1),
            quick_xml::events::Event::Decl(_) => observed["declarations"] = serde_json::json!(observed["declarations"].as_u64().unwrap() + 1),
            quick_xml::events::Event::DocType(_) => observed["doctypes"] = serde_json::json!(observed["doctypes"].as_u64().unwrap() + 1),
            quick_xml::events::Event::Eof => break,
            _ => {}
        }
    }
    assert_eq!(observed, fixture["expected"]["quickXml"]);

    let grant_value = &fixture["grant"];
    let grant = RetainedCloneGrant {
        maximum_items: grant_value["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: grant_value["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: grant_value["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_depth: grant_value["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: grant_value["maximumReleaseBytes"].as_u64().unwrap() as usize };
    let maximum_turns = grant_value["maximumTurns"].as_u64().unwrap() as usize;
    let source = RetainedCloneSource::from_authority(Arc::new(retained), ());
    let mut cursor = RetainedXmlDocument::retained_clone_cursor();
    let mut copied = None;
    for _ in 0..maximum_turns {
        match cursor.advance(source.borrow(), grant).expect("retained XML copy") {
            RetainedCloneStep::Progress(_) => {}
            RetainedCloneStep::Complete(_) => {
                copied = cursor.take();
                break;
            }
        }
    }
    let copied = copied.expect("retained XML copy terminates within fixture bound");
    close_cursor(&mut cursor, grant, maximum_turns);
    let mut progress = |_| true;
    assert_eq!(copied.materialize(&mut NativeEncodeControl::new(4 * 1024 * 1024, &mut progress)).unwrap(), materialized);

    let mut interrupted = RetainedXmlDocument::retained_clone_cursor();
    assert!(matches!(interrupted.advance(source.borrow(), grant).unwrap(), RetainedCloneStep::Progress(_)));
    close_cursor(&mut interrupted, grant, maximum_turns);

    let mut retirement = owned_retirement(copied);
    for turn in 0..maximum_turns {
        match retirement.close_step(grant.maximum_items, grant.maximum_capacity_bytes).expect("retained XML retirement") {
            SnapshotRetirementStep::Complete => break,
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= grant.maximum_items);
                assert!(released_bytes <= grant.maximum_capacity_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("retained XML retirement blocked at turn {turn}"),
        }
    }
    assert!(retirement.terminal_is_empty());
    drop(source);
}
