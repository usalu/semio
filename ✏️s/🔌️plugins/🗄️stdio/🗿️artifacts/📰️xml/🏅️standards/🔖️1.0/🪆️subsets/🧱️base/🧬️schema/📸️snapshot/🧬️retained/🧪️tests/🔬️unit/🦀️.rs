use super::*;
use crate::standards::v1_0::subsets::base::io::text::snapshot::{xml_document_from_text, xml_document_to_text_checked};
use semio_framework_value::{
    retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep},
    retirement::{admit_owned_retirement, owned_retirement_birth_bytes},
};
use std::sync::Arc;

fn close_cursor(cursor: &mut <RetainedXmlDocument as RetainedClone>::Cursor, _grant: RetainedCloneGrant, maximum_turns: usize) {
    assert!(cursor.begin_close());
    for turn in 0..maximum_turns {
        let copy = cursor.next_close_copy_byte_demand().expect("retained XML close copy demand");
        let funded = RetainedCloneGrant {
            maximum_items: 1,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: cursor.next_close_capacity_byte_demand(copy).expect("retained XML close capacity demand"),
            maximum_release_bytes: cursor.next_close_release_byte_demand().expect("retained XML close release demand"),
            maximum_depth: cursor.next_close_depth_demand().expect("retained XML close depth demand"),
        };
        let step = cursor.close_step(funded).unwrap_or_else(|error| panic!("retained XML cursor close at turn {turn}: {error:?}"));
        assert!(step.progress().fits(funded));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(cursor.terminal_is_empty());
            return;
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

    let birth = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 0, maximum_capacity_bytes: owned_retirement_birth_bytes::<RetainedXmlDocument>(), maximum_release_bytes: 0, maximum_depth: 2 };
    let (mut retirement, receipt) = admit_owned_retirement(copied, birth).unwrap_or_else(|(error, _)| panic!("retained XML retirement birth: {error:?}"));
    assert!(receipt.fits(birth));
    for turn in 0..maximum_turns {
        if retirement.terminal_is_empty() {
            break;
        }
        let demand = retirement.next_demand(0).expect("retained XML retirement demand");
        let funded = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth.max(1) };
        let step = retirement.close_step(funded).unwrap_or_else(|error| panic!("retained XML retirement at turn {turn}: {error:?}"));
        assert!(step.progress().fits(funded));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break;
        }
    }
    assert!(retirement.terminal_is_empty());
    drop(source);
}
