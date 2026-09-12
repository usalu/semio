//! 🏷️ Laws over the node-graph caption clipping fixture (`🔣️.json`): a title is the node's NAME
//! clipped by the presentation's own text measure, never a font shrunk until the word disappears.
//!
//! The defect this closes: the generation3d Flow window showed `E` where `extrude` belonged — the
//! draw LOD served `DagNodeSpec::abbreviation` above the `Normal` band and the overlay answered an
//! over-wide caption by binary-searching its font down to 4px.

use super::text;
use serde_json::Value;

const LABEL_FIT_FIXTURE: &str = include_str!("../../🧫️fixtures/🏷️label-fit/🔣️.json");

/// 📏️ The fixture's own synthetic advance — the ONE measure both implementations are driven with,
/// so a row pins the clipping algorithm rather than a font file.
fn synthetic_measure(text: &str, char_width: f64) -> f64 {
    text.chars().count() as f64 * char_width
}

#[test]
fn every_fixture_row_clips_to_the_string_the_law_names() {
    let document: Value = serde_json::from_str(LABEL_FIT_FIXTURE).expect("label fit fixture");
    assert_eq!(document["provenance"]["ellipsis"], Value::from(text::LABEL_ELLIPSIS));
    let rows = document["rows"].as_array().expect("rows");
    assert!(rows.len() >= 8, "the caption law needs more than a happy path");
    for row in rows {
        let name = row["name"].as_str().expect("row name");
        let source = row["text"].as_str().expect("text");
        let max_width = row["maxWidth"].as_f64().expect("maxWidth");
        let char_width = row["charWidth"].as_f64().expect("charWidth");
        let fitted = text::ellipsize_by_measure(source, max_width, |candidate| synthetic_measure(candidate, char_width));
        assert_eq!(fitted, row["expect"].as_str().expect("expect"), "{name}: clipped to the wrong string");
        assert!(synthetic_measure(&fitted, char_width) <= max_width.max(0.0) + 1e-9, "{name}: the clipped caption {fitted:?} still overflows its budget");
        assert!(fitted.matches(text::LABEL_ELLIPSIS).count() <= 1, "{name}: a caption is clipped once, not repeatedly");
    }
}

#[test]
fn a_clipped_caption_keeps_the_longest_prefix_that_fits() {
    let budgets = [12.0_f64, 18.0, 24.0, 36.0, 48.0, 60.0, 120.0];
    for budget in budgets {
        let fitted = text::ellipsize_by_measure("Brep.solid.extrude", budget, |candidate| synthetic_measure(candidate, 6.0));
        if fitted == "Brep.solid.extrude" || fitted == text::LABEL_ELLIPSIS {
            continue;
        }
        let prefix = fitted.trim_end_matches(text::LABEL_ELLIPSIS);
        let longer: String = "Brep.solid.extrude".chars().take(prefix.chars().count() + 1).collect::<String>() + text::LABEL_ELLIPSIS;
        assert!(synthetic_measure(&longer, 6.0) > budget, "at budget {budget} the caption {fitted:?} left room for one more glyph");
    }
}

#[test]
fn the_measure_the_caller_passes_is_the_one_that_decides() {
    let wide = text::ellipsize_by_measure("polygon", 40.0, |candidate| synthetic_measure(candidate, 4.0));
    let narrow = text::ellipsize_by_measure("polygon", 40.0, |candidate| synthetic_measure(candidate, 12.0));
    assert_eq!(wide, "polygon", "a measure that says it fits must leave the caption whole");
    assert_ne!(narrow, "polygon", "a measure that says it does not fit must clip it");
    assert!(narrow.ends_with(text::LABEL_ELLIPSIS));
}

/// 🔡️ The ladder half of the same defect: no draw tier may shorten a caption's CONTENT, so zooming
/// in never replaces a name with a shorter word.
#[test]
fn no_draw_tier_serves_a_shorter_caption_than_the_tier_below_it() {
    use semio_framework_artifact_infinite_dag::{DagDrawLod, DagNodeLabel};
    let ladder = [DagDrawLod::Minimap, DagDrawLod::Overview, DagDrawLod::Compact, DagDrawLod::Normal, DagDrawLod::Detail, DagDrawLod::Micro];
    let mut captioned = false;
    for lod in ladder {
        match lod.node_label() {
            DagNodeLabel::None => assert!(!captioned, "{} dropped a caption a lower tier already showed", lod.label()),
            DagNodeLabel::Name => captioned = true,
        }
    }
    assert!(captioned, "no draw tier captions a node at all");
}
