//! 🛍️ The wgpu half of the app-static catalogue surface. The shell fetches
//! `framework.section.catalogue` as an ordinary retained document (`SurfaceVisible` →
//! `AdvanceRetained`, the same protocol every window body rides) and reassembles it with
//! `read_paged_text_document`. That reader's whole contract is: **the payload is the depth-first
//! concatenation of every `Component::Text` leaf under the document's root**, which is what
//! `semio_framework_plugin::app::paged_text_carrier` splits it into — a balanced
//! `UI_BUILT_CHILDREN_MAX`-ary tree of `UI_TEXT_MAX_BYTES` leaves, because a real catalogue is
//! ~100 KB against a 32 KiB fixed surface doc (ticket 26/09/09/PROCEDURAL-3D-END-TO-END §3.1).
//!
//! These lanes publish that exact document SHAPE as `UiNodeRecord`s — one nested intermediate level
//! included, so a reader that only walked the root's direct children would fail — and hold the
//! reader to byte-for-byte recovery. The producer's own chunking is covered where it lives, in the
//! plugin crate; what could break here is the reader, and the reader is what is pinned.

use super::*;
use ui_contract::SurfaceId;

/// 🔒️ The retained-document arena is process-global and fixed, so these lanes publish one catalogue
/// at a time rather than racing each other for its slots.
fn exclusive() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(())).lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// 🛍️ A catalogue payload shaped like the real one `flow_app_catalogue_json` produces (a section of
/// operator descriptors), sized by `operators` so a case can sit either side of one text leaf.
fn catalogue_payload(operators: usize) -> String {
    let entries: Vec<String> = (0..operators)
        .map(|index| format!(r#""brep.solid.op{index}":{{"label":"Operator {index}","module":"brep","inputs":["profile","axis"],"outputs":["solid"],"summary":"Operator {index} — a deliberately chatty description so the payload pages."}}"#))
        .collect();
    format!(r#"{{"operators":{{{}}},"palette":{{"sections":[{{"id":"brep","label":"Brep"}}]}}}}"#, entries.join(","))
}

/// ✂️ The carrier's own split: `UI_TEXT_MAX_BYTES` per leaf, cut on UTF-8 character boundaries so a
/// multi-byte glyph never straddles two leaves (mirrors `app::section_text_chunks`).
fn text_leaves(payload: &str) -> Vec<&str> {
    let mut leaves = Vec::new();
    let mut start = 0;
    while start < payload.len() {
        let mut end = (start + ui_contract::UI_TEXT_MAX_BYTES).min(payload.len());
        while end > start && !payload.is_char_boundary(end) {
            end -= 1;
        }
        leaves.push(&payload[start..end]);
        start = end;
    }
    if leaves.is_empty() {
        leaves.push("");
    }
    leaves
}

fn record(id: u64, key: &str, component: ui_contract::Component, children: &[u64]) -> ui_contract::UiNodeRecord {
    let mut list = ui_contract::UiNodeChildren::default();
    for child in children {
        list.try_push(ui_contract::UiNodeId(*child)).expect("catalogue child id");
    }
    ui_contract::UiNodeRecord {
        id: ui_contract::UiNodeId(id),
        key: ui_contract::UiText::try_from_str(key).expect("catalogue node key"),
        component,
        layout: Default::default(),
        style: Default::default(),
        activity: Default::default(),
        disabled: false,
        transition: None,
        accessibility: Default::default(),
        bindings: Default::default(),
        menu: None,
        children: list,
    }
}

fn text_component(value: &str) -> ui_contract::Component {
    ui_contract::Component::Text(ui_contract::TextProps { value: ui_contract::Label(ui_contract::UiText::try_from_str(value).expect("catalogue leaf label")), emphasize: None, data_attributes: None })
}

fn container_component() -> ui_contract::Component {
    ui_contract::Component::Container(ui_contract::ContainerProps { role: ui_contract::ContainerRole::Plain, label: None, description: None, required: None, error: None, default_open: None, drop_overlay: None })
}

/// 🛍️ Publishes `payload` in the carrier's own shape: a root container over `pages` intermediate
/// containers, each over its share of the text leaves. `pages > 1` is what makes the document a real
/// tree rather than one flat level — the case a reader that only walks the root's direct children
/// would silently truncate.
fn catalogue_document(payload: &str, pages: usize) -> UiDocumentLease {
    let leaves = text_leaves(payload);
    let per_page = leaves.len().div_ceil(pages.max(1)).max(1);
    let mut records = Vec::new();
    let mut page_ids = Vec::new();
    let mut next = 2u64;
    for chunk in leaves.chunks(per_page) {
        let page_id = next;
        next += 1;
        let mut leaf_ids = Vec::new();
        for leaf in chunk {
            records.push(record(next, &format!("c{next}"), text_component(leaf), &[]));
            leaf_ids.push(next);
            next += 1;
        }
        records.push(record(page_id, &format!("p{page_id}"), container_component(), &leaf_ids));
        page_ids.push(page_id);
    }
    records.push(record(1, semio_framework::UiRefreshSection::Catalogue.body_key(), container_component(), &page_ids));
    let surface = SurfaceId::try_from(semio_framework::UiRefreshSection::Catalogue.body_key()).expect("reserved catalogue surface id");
    static GENERATION: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
    let generation = GENERATION.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let mut builder = ui_contract::UiDocumentBuilder::try_new(generation, surface, ui_contract::UiRevision(1), Some(ui_contract::UiNodeId(1)), 0).expect("catalogue document slot");
    for record in records {
        builder.try_push(record).expect("catalogue record");
    }
    builder.finish().expect("catalogue document")
}

/// ♻️ Drains one fetched catalogue document through its own retirement ladder — the fixed document
/// arena has a slot ceiling, so a lane that publishes several must close each one.
fn retire_catalogue_document(mut document: UiDocumentLease) {
    for _ in 0..1_048_576 {
        if document.close_step() && document.terminal_is_empty() {
            return;
        }
    }
    panic!("catalogue document retirement did not converge");
}

#[test]
fn app_catalogue_section_document_round_trips_through_the_wgpu_reader() {
    let _guard = exclusive();
    for (operators, pages) in [(1usize, 1usize), (8, 1), (64, 4)] {
        let payload = catalogue_payload(operators);
        let document = catalogue_document(&payload, pages);
        let read = read_paged_text_document(&document).expect("catalogue reassembles");
        retire_catalogue_document(document);
        assert_eq!(read, payload, "a {operators}-operator catalogue ({} bytes over {pages} carrier pages) must survive byte-for-byte", payload.len());
    }
}

#[test]
fn app_catalogue_reader_walks_a_payload_larger_than_one_text_leaf() {
    let _guard = exclusive();
    let payload = catalogue_payload(64);
    assert!(payload.len() > ui_contract::UI_TEXT_MAX_BYTES, "the paging case needs a payload past one {}-byte text leaf, got {}", ui_contract::UI_TEXT_MAX_BYTES, payload.len());
    let document = catalogue_document(&payload, 4);
    let header = document.header().expect("catalogue header");
    let read = read_paged_text_document(&document).expect("catalogue reassembles");
    retire_catalogue_document(document);
    assert!(header.node_count > text_leaves(&payload).len(), "a paged carrier is a tree — leaves plus their intermediate pages plus the root");
    assert_eq!(read, payload, "every leaf across every intermediate page must be concatenated in depth-first order");
}

#[test]
fn app_catalogue_reader_returns_the_empty_payload_for_an_app_without_a_catalogue() {
    let _guard = exclusive();
    let document = catalogue_document("{}", 1);
    let read = read_paged_text_document(&document).expect("catalogue reassembles");
    retire_catalogue_document(document);
    assert_eq!(read, "{}", "the default `ArtifactApp::app_catalogue_json` is an empty object, and must arrive as one");
}
