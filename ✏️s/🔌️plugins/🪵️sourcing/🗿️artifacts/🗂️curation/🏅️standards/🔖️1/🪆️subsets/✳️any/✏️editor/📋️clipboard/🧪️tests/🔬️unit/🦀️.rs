//! 🧪️ Laws of the sourcing editing surface: clipboard copy, cut and paste over curated counts, the `rows` selection topology, the row "More" menu, the painted table selection and the grid instances that stand for their kind.

use crate::editor::sourcing::clipboard::{copy, cut, decode, paste, SourcingClipboard};
use crate::editor::sourcing::commands::curation_set_count;
use crate::editor::sourcing::config::SourcingCurationConfig;
use crate::editor::sourcing::editor_vocabulary::{sourcing_row_menu, SOURCING_OBJECT_GRANULARITY, SOURCING_ROWS_DOMAIN};
use crate::editor::sourcing::modes::edit::windows::{curated, pool};
use crate::editor::sourcing::selection::rows_topology;
use crate::editor::sourcing::terminology::SourcingLabels;
use crate::editor_unit_tests::context::{arena, table_scene_of};
use crate::{CuratedItem, CurationSnapshot};
use semio_plugin::kernel::{ClipboardError, PasteAnchor, PastePlacement};
use semio_plugin::TableCell;

const KVH: &str = "beam-kvh-c24";
const IPE: &str = "beam-steel-ipe200";
const HEA: &str = "beam-steel-hea160";
const GLULAM: &str = "beam-glulam-gl24h";

fn document() -> CurationSnapshot {
    let mut document = crate::standards::v1::subsets::any::io::text::snapshot::default_document();
    document.curated = vec![CuratedItem { object_id: KVH.into(), count: 3 }, CuratedItem { object_id: HEA.into(), count: 8 }];
    document
}

fn ids(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| value.to_string()).collect()
}

fn placement(anchor: PasteAnchor) -> PastePlacement {
    PastePlacement { anchor, position: None }
}

/// ⚖️ LAW: a copy carries the selected kinds in stock order with their curated counts (one unit for a kind that is not curated yet), and an empty selection is refused.
#[test]
fn copy_carries_the_selected_kinds_with_their_curated_counts() {
    let document = document();
    let fragment = copy(&document, &ids(&[IPE, KVH])).expect("copy");
    let packet: SourcingClipboard = decode(&fragment).expect("decode");
    assert_eq!(packet.items, vec![CuratedItem { object_id: KVH.into(), count: 3 }, CuratedItem { object_id: IPE.into(), count: 1 }]);
    assert!(matches!(copy(&document, &[]), Err(ClipboardError::EmptySelection)));
    assert!(matches!(copy(&document, &ids(&["unknown"])), Err(ClipboardError::EmptySelection)));
}

/// ⚖️ LAW: a cut removes exactly the selected kinds that are curated.
#[test]
fn cut_removes_the_selected_curated_kinds() {
    let document = document();
    assert_eq!(cut(&document, &ids(&[KVH, GLULAM])).len(), 1);
    assert_eq!(cut(&document, &ids(&[GLULAM])).len(), 0);
    assert_eq!(cut(&document, &ids(&[KVH, HEA])).len(), 2);
}

/// ⚖️ LAW: a paste adds the copied counts to the curation bounded by availability — a full kind is a no-op, an unstocked kind is skipped — and every anchor pastes alike.
#[test]
fn paste_adds_counts_bounded_by_availability_whatever_the_anchor() {
    let document = document();
    let fragment = copy(&document, &ids(&[KVH, HEA, IPE])).expect("copy");
    for anchor in [PasteAnchor::Original, PasteAnchor::Middle, PasteAnchor::Centroid, PasteAnchor::BottomLeft, PasteAnchor::BottomRight, PasteAnchor::TopLeft, PasteAnchor::TopRight] {
        let mutations = paste(&document, &fragment, &placement(anchor)).expect("paste");
        assert_eq!(mutations.len(), 2, "KVH grows, IPE is created, HEA is already at its availability of eight ({anchor:?})");
    }
    let mut stranger = fragment.clone();
    stranger.dsl_text = stranger.dsl_text.replace(KVH, "unstocked");
    assert_eq!(paste(&document, &stranger, &placement(PasteAnchor::Original)).expect("paste").len(), 1, "the unstocked kind is skipped");
}

/// ⚖️ LAW: a fragment of another media type, schema or shape is refused typed, never applied.
#[test]
fn paste_refuses_foreign_and_malformed_fragments() {
    let document = document();
    let fragment = copy(&document, &ids(&[KVH])).expect("copy");
    let mut foreign = fragment.clone();
    foreign.media_type = semio_plugin::MediaType { class: semio_plugin::MediaClass::TwoD, form: semio_plugin::MediaForm::Design };
    assert!(matches!(paste(&document, &foreign, &placement(PasteAnchor::Original)), Err(ClipboardError::IncompatibleMediaType(_))));
    let mut malformed = fragment.clone();
    malformed.dsl_text = "{\"schema\":\"sourcing.clipboard.v1\",\"items\":[]}".into();
    assert!(matches!(paste(&document, &malformed, &placement(PasteAnchor::Original)), Err(ClipboardError::ParseFailed(_))));
    malformed.dsl_text = "not json".into();
    assert!(matches!(paste(&document, &malformed, &placement(PasteAnchor::Original)), Err(ClipboardError::ParseFailed(_))));
}

/// ⚖️ LAW: the `rows` topology is the stock catalogue in document order under the `object` granularity, so Select All reaches every kind and a range follows the pool order.
#[test]
fn the_rows_topology_lists_every_stock_kind_in_document_order() {
    let document = document();
    let topology = rows_topology(&document);
    let ordered = &topology.domains.get(SOURCING_ROWS_DOMAIN).expect("rows domain").ordered;
    let stock: Vec<String> = crate::stock_of(&document).into_iter().map(|kind| kind.id).collect();
    assert_eq!(ordered.iter().map(|node| node.id.clone()).collect::<Vec<_>>(), stock);
    assert!(ordered.len() >= 12 && ordered.iter().all(|node| node.granularity == SOURCING_OBJECT_GRANULARITY && node.parent.is_none()));
}

/// ⚖️ LAW: the row "More" menu offers curate while stock lasts and curate-less and remove once the kind is curated, every entry placed in the menu and addressed to the row's kind.
#[test]
fn the_row_menu_offers_only_what_the_row_can_do() {
    let document = document();
    let kinds = crate::stock_of(&document);
    let kind = kinds.iter().find(|kind| kind.id == KVH).expect("kvh");
    let labels = &SourcingLabels::NATIVE_EN;
    let sizes = |count: u32| match sourcing_row_menu(kind, count, labels) {
        TableCell::Buttons { buttons } => {
            assert!(buttons.iter().all(|button| button.placement == Some(semio_plugin::UiTreeActionPlacement::Menu)));
            buttons.len()
        }
        other => panic!("the row menu is a buttons cell: {other:?}"),
    };
    assert_eq!((sizes(0), sizes(3), sizes(kind.availability)), (1, 3, 2));
}

/// ⚖️ LAW: both tables paint the rows of the `rows` selection and carry the row menu in every row.
#[test]
fn the_tables_paint_the_selection_and_carry_the_row_menu() {
    let _arena = arena();
    let document = document();
    let labels = &SourcingLabels::NATIVE_EN;
    let cfg = SourcingCurationConfig::default();
    let selected = ids(&[KVH]);
    for built in [pool::render(&document, &cfg, labels, &selected, semio_plugin::Locale::En).expect("pool"), curated::render(&document, &cfg, labels, &selected, semio_plugin::Locale::En).expect("curated")] {
        let scene = table_scene_of(&built).expect("table scene");
        let painted: serde_json::Value = serde_json::from_str(scene.selection_json.as_deref().expect("selection json")).expect("selection json parses");
        assert_eq!(painted["selectedIds"], serde_json::json!([KVH]));
        let rows: Vec<serde_json::Value> = serde_json::from_str(&scene.rows_json).expect("rows");
        assert!(!rows.is_empty() && rows.iter().all(|row| row.get("menu").is_some()), "every row carries its menu cell");
    }
}

/// ⚖️ LAW: a negative typed count is refused rather than read as zero, so a mistyped sign never removes a curated kind; zero still removes it.
#[test]
fn a_negative_typed_count_is_refused_and_zero_removes() {
    let document = document();
    let history = semio_plugin::HistoryView::empty();
    let cfg = SourcingCurationConfig::default();
    let run = |value: f64| curation_set_count::handle(&curation_set_count::CurationSetCount { object_id: KVH.into(), delta: None, value: Some(value) }, &semio_plugin::ArtifactView::new(&document, &history), &semio_plugin::ConfigView { snapshot: &cfg, window: None }).expect("handle").artifact_mutations.len();
    assert_eq!((run(-3.0), run(f64::NAN), run(0.0), run(5.0), run(3.0)), (0, 0, 1, 1, 0));
}
