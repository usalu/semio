use super::*;
use crate::tui::cell::{diff, Cell, CellBuffer, DiffRun};
use crate::tui::geometry::{Pos, Rect, Size};
use semio_framework_pack_json::{parse, JsonMemberPolicy, Value};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

struct Case {
    name: String,
    text: String,
    clusters: Vec<String>,
    cells: u16,
}

fn cases() -> Vec<Case> {
    let root = parse(include_str!("../../🧫️fixtures/📏️text-width/🔣️.json"), JsonMemberPolicy::Reject).expect("the width fixture is valid JSON");
    let list = root.as_object().and_then(|object| object.get("cases")).and_then(Value::as_array).expect("cases array");
    list.iter()
        .map(|entry| {
            let object = entry.as_object().expect("case object");
            let text = |key: &str| object.get(key).and_then(Value::as_str).expect("case string").to_string();
            let clusters = object.get("clusters").and_then(Value::as_array).expect("clusters").iter().map(|cluster| cluster.as_str().expect("cluster string").to_string()).collect();
            Case { name: text("name"), text: text("text"), clusters, cells: object.get("cells").and_then(Value::as_u64).expect("cells") as u16 }
        })
        .collect()
}

fn oracle_scalar_cells(c: char) -> u8 {
    c.width().map_or(0, |width| width.min(2) as u8)
}

fn oracle_ranges(predicate: impl Fn(char) -> bool) -> Vec<(u32, u32)> {
    let mut out: Vec<(u32, u32)> = Vec::new();
    for codepoint in 0..=0x10ffffu32 {
        let Some(c) = char::from_u32(codepoint) else { continue };
        if predicate(c) {
            match out.last_mut() {
                Some(last) if last.1 + 1 == codepoint => last.1 = codepoint,
                _ => out.push((codepoint, codepoint)),
            }
        }
    }
    out
}

fn model_clusters(s: &str) -> Vec<String> {
    clusters(s).map(str::to_string).collect()
}

fn oracle_clusters(s: &str) -> Vec<String> {
    s.graphemes(true).map(str::to_string).collect()
}

//#region 📏️Fixture
#[test]
fn fixture_is_large_enough_to_cover_the_repository_taxonomy() {
    let list = cases();
    assert!(list.len() >= 90, "{} cases", list.len());
    for needle in ["repo path toolbox framework", "repo path engine artist zwj", "family zwj", "keycap one", "flag switzerland", "desktop computer vs16"] {
        assert!(list.iter().any(|case| case.name == needle), "fixture misses {needle}");
    }
}

#[test]
fn model_matches_the_fixture_clusters_and_cell_widths() {
    for case in cases() {
        assert_eq!(model_clusters(&case.text), case.clusters, "clusters of {:?}", case.name);
        assert_eq!(display_width(&case.text), case.cells, "cells of {:?}", case.name);
        let summed: u16 = case.clusters.iter().map(|cluster| u16::from(cluster_cells(cluster))).sum();
        assert_eq!(summed, case.cells, "per-cluster sum of {:?}", case.name);
    }
}

#[test]
fn oracles_produce_the_fixture_clusters_and_cell_widths() {
    for case in cases() {
        assert_eq!(oracle_clusters(&case.text), case.clusters, "oracle clusters of {:?}", case.name);
        assert_eq!(case.text.as_str().width() as u16, case.cells, "oracle cells of {:?}", case.name);
    }
}
//#endregion 📏️Fixture

//#region 🔤️Tables
#[test]
fn scalar_width_tables_equal_the_oracle_ranges() {
    assert_eq!(oracle_ranges(|c| matches!(c.width(), Some(2 | 3))), WIDE);
    assert_eq!(oracle_ranges(|c| c.width() == Some(0)), ZERO);
    assert_eq!(oracle_ranges(|c| c.width() == Some(1) && format!("{c}\u{fe0f}").as_str().width() == 2), VS16_BASE);
    assert_eq!(oracle_ranges(|c| c.width() == Some(2) && format!("{c}\u{fe0e}").as_str().width() == 1), VS15_NARROW);
    assert_eq!(oracle_ranges(|c| c.width() == Some(2) && format!("{c}\u{200d}\u{1f525}").as_str().width() == 2), EMOJI_ZWJ);
    assert_eq!(
        oracle_ranges(|c| c.width().is_some_and(|width| width > 0) && !('\u{1f3fb}'..='\u{1f3ff}').contains(&c) && format!("{c}\u{1f3fd}").as_str().width() == 2),
        EMOJI_MODIFIER_BASE
    );
}

#[test]
fn break_table_is_sorted_disjoint_and_confined_to_assigned_planes() {
    let mut previous_last = None::<u32>;
    for &(first, last, _) in BREAKS {
        assert!(first <= last);
        assert!(previous_last.is_none_or(|before| before < first), "BREAKS overlaps at U+{first:04X}");
        assert!(last < 0x40000 || first >= 0xe0000, "BREAKS entry U+{first:04X}..U+{last:04X} reaches an unswept plane");
        previous_last = Some(last);
    }
    for table in [WIDE, ZERO, VS16_BASE, VS15_NARROW, EMOJI_ZWJ, EMOJI_MODIFIER_BASE, INCB_EXTEND] {
        assert!(table.windows(2).all(|pair| pair[0].1 < pair[1].0), "range table not sorted and disjoint");
    }
    assert!(INCB_LINKER.windows(2).all(|pair| pair[0] < pair[1]));
}

#[test]
fn every_scalar_has_the_oracle_width() {
    for codepoint in 0..=0x10ffffu32 {
        let Some(c) = char::from_u32(codepoint) else { continue };
        assert_eq!(char_cells(c), oracle_scalar_cells(c), "U+{codepoint:04X}");
    }
}

#[test]
fn every_scalar_segments_like_the_oracle_in_context() {
    let plain: &[(&str, &str)] = &[("a", ""), ("", "a"), ("\u{1f468}\u{200d}", ""), ("\u{1f1e6}", "")];
    let special: &[(&str, &str)] = &[
        ("", ""),
        ("\u{1f1e6}\u{1f1e7}", ""),
        ("\u{1100}", ""),
        ("\u{ac00}", ""),
        ("\u{ac01}", ""),
        ("\u{915}\u{94d}", ""),
        ("\u{915}\u{94d}\u{93c}", ""),
        ("", "\u{200d}\u{1f469}"),
        ("\r", ""),
        ("", "\n"),
    ];
    let compare = |text: &str, label: &str| -> bool {
        let theirs: Vec<&str> = text.graphemes(true).collect();
        let mine: Vec<&str> = clusters(text).collect();
        assert_eq!(mine, theirs, "{label}");
        theirs.len() != text.chars().count()
    };
    let swept = (0..0x20000u32).chain((0x20000..0x32000u32).step_by(37)).chain(0xe0000..0xe1000).filter_map(char::from_u32);
    let mut specials = 0usize;
    for c in swept {
        let mut interesting = break_class(c) != Break::Any;
        for (before, after) in plain {
            interesting |= compare(&format!("{before}{c}{after}"), &format!("U+{:04X} between {before:?} and {after:?}", c as u32));
        }
        if interesting {
            specials += 1;
            for (before, after) in special {
                compare(&format!("{before}{c}{after}"), &format!("U+{:04X} between {before:?} and {after:?}", c as u32));
            }
        }
    }
    assert!(specials > 2500, "{specials} non-trivial scalars swept");
}
//#endregion 🔤️Tables

//#region 🧪️Sequences
const ATOMS: &[&str] = &[
    "a", "界", "e", "\u{301}", "\u{200d}", "\u{fe0f}", "\u{fe0e}", "\u{20e3}", "1", "#", "🙂", "🧰", "🖥", "⌨", "❤", "☝", "👍", "🏽", "🇨", "🇭", "\u{1100}", "\u{1161}", "\u{11a8}", "\u{ac00}", "क", "\u{94d}", "ष", "ा", "\u{e0067}", "\u{e007f}", "🏴", "🌈", "🔥", "⌚", "\u{200b}",
];

#[test]
fn short_sequences_of_emoji_marks_and_jamo_measure_like_the_oracle() {
    let mut checked = 0usize;
    let mut mismatches: Vec<String> = Vec::new();
    for first in ATOMS {
        for second in ATOMS {
            for third in ATOMS {
                let text = format!("{first}{second}{third}");
                let (mine, theirs) = (model_clusters(&text), oracle_clusters(&text));
                assert_eq!(mine, theirs, "clusters of {text:?}");
                let modifier_joiner = text.starts_with(|c: char| ('\u{1f3fb}'..='\u{1f3ff}').contains(&c)) && text.contains('\u{200d}');
                if display_width(&text) != text.as_str().width() as u16 && !modifier_joiner {
                    mismatches.push(format!("{text:?}: model {} oracle {}", display_width(&text), text.as_str().width()));
                }
                checked += 1;
            }
        }
    }
    assert!(checked > 40_000);
    assert!(mismatches.is_empty(), "{} width mismatches, first: {:?}", mismatches.len(), &mismatches[..mismatches.len().min(12)]);
}

#[test]
fn a_skin_tone_modifier_that_starts_a_zwj_sequence_is_a_known_deviation_from_the_string_oracle() {
    let text = "\u{1f3fd}\u{200d}\u{1f642}";
    assert_eq!(model_clusters(text), vec!["\u{1f3fd}\u{200d}".to_string(), "\u{1f642}".to_string()], "UAX #29 breaks before the pictograph, so the cluster model measures two clusters");
    assert_eq!(display_width(text), 4);
    assert_eq!(text.width(), 2, "the string oracle joins across the cluster boundary");
}
//#endregion 🧪️Sequences

//#region ✂️Truncation
#[test]
fn truncate_never_splits_a_cluster_or_a_wide_glyph() {
    assert_eq!(truncate_to("a世b", 2), ("a", 1));
    assert_eq!(truncate_to("世", 2), ("世", 2));
    assert_eq!(truncate_to("🧰\u{fe0f}framework", 3), ("🧰\u{fe0f}f", 3));
    assert_eq!(truncate_to("🖥\u{fe0f}x", 1), ("", 0), "a two-cell cluster does not fit one cell");
    assert_eq!(truncate_to("e\u{301}x", 1), ("e\u{301}", 1));
}

#[test]
fn elide_end_marks_the_cut_and_stays_within_the_budget() {
    assert_eq!(elide_end("short", 10), "short");
    assert_eq!(elide_end("abcdefghij", 6), "abcde…");
    assert_eq!(elide_end("abcdefghij", 1), "…");
    assert_eq!(elide_end("abcdefghij", 0), "");
    assert_eq!(elide_end("世界世界", 4), "世…", "a wide cluster that does not fit leaves a gap, never overflows");
    assert_eq!(elide_end("🧰\u{fe0f}framework", 5), "🧰\u{fe0f}fr…");
}

#[test]
fn elide_start_keeps_the_end() {
    assert_eq!(elide_start("abcdefghij", 6), "…fghij");
    assert_eq!(elide_start("🧰\u{fe0f}framework", 5), "…work");
}

#[test]
fn elide_middle_keeps_the_discriminating_tail() {
    assert_eq!(elide_middle("puzzle3d-react-wide", 12, 6), "puzzl…t-wide");
    assert_eq!(elide_middle("short", 12, 6), "short");
    let elided = elide_middle("framework-os-dev:dev-puzzle3d-react-dev", 24, 6);
    assert_eq!(display_width(&elided), 24);
    assert_eq!(elided, "framework-os-dev:…ct-dev");
    assert_eq!(elide_middle("abcdefghij", 3, 6), "a…j");
}

#[test]
fn elide_with_a_wider_ellipsis_honours_the_budget() {
    assert_eq!(elide("abcdefghij", 6, Elision::End, "..."), "abc...");
    assert_eq!(elide("abcdefghij", 2, Elision::End, "..."), "..");
    assert_eq!(display_width(&elide("🧰\u{fe0f}framework/🔨\u{fe0f}modules", 10, Elision::Middle { tail: 4 }, "...")), 10);
}

#[test]
fn elision_never_exceeds_the_budget_for_any_fixture_text() {
    for case in cases() {
        for budget in 0..=12u16 {
            for elision in [Elision::End, Elision::Start, Elision::Middle { tail: 6 }, Elision::Middle { tail: 2 }] {
                let result = elide(&case.text, budget, elision, ELLIPSIS);
                assert!(display_width(&result) <= budget, "{:?} at {budget} with {elision:?}: {result:?}", case.name);
                assert_eq!(model_clusters(&result).concat(), result.to_string());
            }
        }
    }
}
//#endregion ✂️Truncation

//#region 🎚️Width Modes
fn oracle_scalar_width(text: &str) -> u16 {
    text.chars().map(|c| c.width().unwrap_or(0) as u16).sum()
}

#[test]
fn scalar_mode_counts_every_fixture_case_like_the_oracle_char_sum() {
    for case in cases() {
        assert_eq!(display_width_in(&case.text, WidthMode::Scalar), oracle_scalar_width(&case.text), "scalar cells of {:?}", case.name);
        assert_eq!(display_width_in(&case.text, WidthMode::Cluster), case.cells, "cluster cells of {:?}", case.name);
    }
}

#[test]
fn the_two_modes_disagree_exactly_on_text_default_pictographs_with_a_variation_selector() {
    let platform = "\u{1f5a5}\u{fe0f}platform";
    let toolbox = "\u{1f9f0}\u{fe0f}framework";
    assert_eq!((display_width_in(platform, WidthMode::Cluster), display_width_in(platform, WidthMode::Scalar)), (10, 9));
    assert_eq!((display_width_in(toolbox, WidthMode::Cluster), display_width_in(toolbox, WidthMode::Scalar)), (11, 11), "a wide emoji is two cells either way");
    let path = format!("{toolbox}/\u{1f528}\u{fe0f}modules/{platform}/\u{2328}\u{fe0f}tui");
    assert_eq!(display_width_in(&path, WidthMode::Cluster), 38, "keyboard and desktop widen to two cells in cluster mode");
    assert_eq!(display_width_in(&path, WidthMode::Scalar), 36);
}

#[test]
fn the_active_mode_follows_the_configured_buffer_and_every_width_function_obeys_it() {
    let platform = "\u{1f5a5}\u{fe0f}platform";
    let mut buf = buffer(24);
    assert_eq!(active_width_mode(), WidthMode::Cluster);
    assert_eq!(display_width(platform), 10);
    buf.set_width_mode(WidthMode::Scalar);
    assert_eq!(active_width_mode(), WidthMode::Scalar);
    assert_eq!(display_width(platform), 9);
    assert_eq!(truncate_to(platform, 3), ("\u{1f5a5}\u{fe0f}pl", 3), "one cell for the pictograph, then p and l");
    assert_eq!(elide_end(platform, 6), "\u{1f5a5}\u{fe0f}plat\u{2026}");
    assert_eq!(boundary_at_cell(platform, 1), "\u{1f5a5}\u{fe0f}".len(), "column 1 is the p");
    assert_eq!(window_cells(platform, 1, 4), ("\u{fe0f}plat", 0), "the zero-width selector rides along at the left edge");
    let consumed = write(&mut buf, 0, platform);
    assert_eq!(consumed, display_width(platform), "the buffer paints as many cells as the active mode counts");
    buf.set_width_mode(WidthMode::Cluster);
    assert_eq!(display_width(platform), 10);
}

#[test]
fn every_cut_fits_its_budget_in_both_modes_for_all_fixture_texts() {
    for mode in [WidthMode::Cluster, WidthMode::Scalar] {
        for case in cases() {
            for budget in 0..=12u16 {
                let (cut, used) = truncate_to_in(&case.text, budget, mode);
                assert!(used <= budget && display_width_in(cut, mode) == used, "{mode:?} {:?} at {budget}", case.name);
                for elision in [Elision::End, Elision::Start, Elision::Middle { tail: 6 }] {
                    let result = elide_in(&case.text, budget, elision, ELLIPSIS, mode);
                    assert!(display_width_in(&result, mode) <= budget, "{mode:?} {:?} at {budget} with {elision:?}: {result:?}", case.name);
                }
                let (visible, lead) = window_cells_in(&case.text, budget, 6, mode);
                assert!(display_width_in(visible, mode) + lead <= 6, "{mode:?} window of {:?} at {budget}", case.name);
            }
        }
    }
}

#[test]
fn scalar_mode_buffers_paint_the_cells_the_scalar_width_predicts_for_repo_paths() {
    for case in cases().into_iter().filter(|case| case.name.starts_with("repo ")) {
        let mut buf = CellBuffer::new(Size { width: 80, height: 1 }, blank());
        buf.set_width_mode(WidthMode::Scalar);
        let consumed = write(&mut buf, 0, &case.text);
        assert_eq!(consumed, display_width_in(&case.text, WidthMode::Scalar), "{:?}", case.name);
        assert_paired(&buf, &case.name);
    }
    set_active_width_mode(WidthMode::Cluster);
}
//#endregion 🎚️Width Modes

//#region 🪟️Windows
#[test]
fn window_cells_scrolls_text_under_a_view_without_splitting_wide_clusters() {
    assert_eq!(window_cells("abcdef", 0, 3), ("abc", 0));
    assert_eq!(window_cells("abcdef", 2, 3), ("cde", 0));
    assert_eq!(window_cells("abcdef", 9, 3), ("", 0), "scrolled past the end shows nothing");
    assert_eq!(window_cells("a世界b", 1, 4), ("世界", 0));
    assert_eq!(window_cells("a世界b", 2, 4), ("界b", 1), "the second cell of 世 shows as one blank, then 界 and b");
    assert_eq!(window_cells("a世界b", 0, 4), ("a世", 0), "a view that cannot hold 界 whole stops before it");
    assert_eq!(window_cells("🧰\u{fe0f}x", 1, 3), ("x", 1));
    let (visible, lead) = window_cells("a世界b", 2, 3);
    assert_eq!((visible, lead), ("界", 1));
    assert!(display_width(visible) + lead <= 3);
}
//#endregion 🪟️Windows

//#region 🧵️Boundaries
#[test]
fn cursor_boundaries_step_over_whole_clusters() {
    let text = "a🧰\u{fe0f}e\u{301}b";
    let first = next_boundary(text, 0);
    let second = next_boundary(text, first);
    let third = next_boundary(text, second);
    assert_eq!((first, second, third), (1, 1 + "🧰\u{fe0f}".len(), 1 + "🧰\u{fe0f}".len() + "e\u{301}".len()));
    assert_eq!(next_boundary(text, text.len()), text.len());
    assert_eq!(previous_boundary(text, text.len()), third);
    assert_eq!(previous_boundary(text, third), second);
    assert_eq!(previous_boundary(text, 1), 0);
    assert_eq!(previous_boundary(text, 0), 0);
    assert_eq!(boundary_at_or_before(text, second + 2), second);
    assert_eq!(boundary_at_or_before(text, text.len() + 5), text.len());
}

#[test]
fn boundary_at_cell_maps_columns_to_clusters() {
    let text = "a🧰\u{fe0f}b";
    assert_eq!(boundary_at_cell(text, 0), 0);
    assert_eq!(boundary_at_cell(text, 1), 1);
    assert_eq!(boundary_at_cell(text, 2), 1, "the second cell of a wide cluster belongs to it");
    assert_eq!(boundary_at_cell(text, 3), 1 + "🧰\u{fe0f}".len());
    assert_eq!(boundary_at_cell(text, 4), text.len());
}
//#endregion 🧵️Boundaries

//#region 📏️Lengths
#[test]
fn long_ascii_and_emoji_strings_sum_their_clusters() {
    let ascii = "x".repeat(4000);
    let emoji = "🧰\u{fe0f}framework/".repeat(400);
    assert_eq!(display_width(&ascii), 4000);
    assert_eq!(display_width(&emoji), 400 * 12);
}
//#endregion 📏️Lengths

//#region 🔲️Cells
fn blank() -> Cell {
    Cell::blank([1, 1, 1], [2, 2, 2])
}

fn buffer(width: u16) -> CellBuffer {
    CellBuffer::new(Size { width, height: 1 }, blank())
}

fn write(buf: &mut CellBuffer, x: u16, text: &str) -> u16 {
    buf.put_str(Pos { x, y: 0 }, text, [9, 9, 9], [8, 8, 8], 0, Rect::new(0, 0, buf.size.width, 1))
}

fn widths(buf: &CellBuffer) -> Vec<u8> {
    (0..buf.size.width).map(|x| buf.get(x, 0).unwrap().width).collect()
}

fn assert_paired(buf: &CellBuffer, context: &str) {
    let mut x = 0u16;
    while x < buf.size.width {
        let width = u16::from(buf.get(x, 0).unwrap().width);
        assert_ne!(width, 0, "orphaned continuation at {x} after {context}: {:?}", widths(buf));
        for covered in x + 1..x + width {
            assert!(covered < buf.size.width, "wide lead at {x} overflows after {context}: {:?}", widths(buf));
            assert_eq!(buf.get(covered, 0).unwrap().width, 0, "lead at {x} misses its continuation at {covered} after {context}: {:?}", widths(buf));
        }
        x += width.max(1);
    }
}

#[test]
fn wide_and_multi_scalar_clusters_occupy_a_lead_and_continuation_cells() {
    let mut buf = buffer(14);
    let text = "a🧰\u{fe0f}b🧑\u{200d}💻c🇨🇭";
    let consumed = write(&mut buf, 0, text);
    assert_eq!(consumed, display_width(text));
    assert_eq!(usize::from(consumed), text.width());
    assert_eq!(widths(&buf)[..consumed as usize], [1, 2, 0, 1, 2, 0, 1, 2, 0]);
    assert_eq!(buf.row_text(0).trim_end(), text);
    assert_ne!(buf.tail_id(1, 0), 0, "the VS16 of the toolbox travels with its lead cell");
    assert_ne!(buf.tail_id(4, 0), 0, "the ZWJ sequence travels with its lead cell");
    assert_eq!(buf.get(1, 0).unwrap().ch, '🧰');
    assert_paired(&buf, "initial write");
}

#[test]
fn a_narrow_glyph_over_either_half_of_a_wide_cluster_blanks_the_other_half() {
    let mut over_continuation = buffer(6);
    write(&mut over_continuation, 0, "a世b");
    over_continuation.put(2, 0, Cell { ch: 'x', ..blank() });
    assert_eq!(over_continuation.row_text(0), "a xb  ", "the lead half is blanked, not left as a two-cell glyph over a narrow one");
    assert_paired(&over_continuation, "narrow over continuation");

    let mut over_lead = buffer(6);
    write(&mut over_lead, 0, "a世b");
    over_lead.put(1, 0, Cell { ch: 'y', ..blank() });
    assert_eq!(over_lead.row_text(0), "ay b  ");
    assert_paired(&over_lead, "narrow over lead");
}

#[test]
fn a_wide_cluster_over_the_middle_of_two_wide_clusters_blanks_both_remainders() {
    let mut buf = buffer(8);
    write(&mut buf, 0, "世界");
    write(&mut buf, 1, "🙂");
    assert_eq!(buf.row_text(0), " 🙂   ".to_string() + "  ", "the cut halves of both neighbours are blanks");
    assert_paired(&buf, "wide across two wide");
}

#[test]
fn a_wide_cluster_that_does_not_fit_the_last_column_becomes_a_blank() {
    let mut buf = buffer(3);
    let consumed = write(&mut buf, 2, "世");
    assert_eq!(consumed, 0, "the clip refuses a cluster that does not fit");
    buf.put(2, 0, Cell { ch: '世', width: 2, ..blank() });
    assert_eq!((buf.get(2, 0).unwrap().ch, buf.get(2, 0).unwrap().width), (' ', 1));
    assert_paired(&buf, "wide at the last column");
}

#[test]
fn an_orphan_continuation_write_becomes_a_blank() {
    let mut buf = buffer(3);
    buf.put(0, 0, Cell { ch: '\0', width: 0, ..blank() });
    assert_eq!((buf.get(0, 0).unwrap().ch, buf.get(0, 0).unwrap().width), (' ', 1));
    buf.put(1, 0, Cell { ch: 'a', ..blank() });
    buf.put(2, 0, Cell { ch: '\0', width: 0, ..blank() });
    assert_eq!(buf.get(2, 0).unwrap().width, 1, "a continuation next to a narrow cell has no lead and is blanked");
}

#[test]
fn random_writes_never_leave_an_orphaned_half() {
    let samples = ["a", "世", "🧰\u{fe0f}", "e\u{301}", "🧑\u{200d}💻", "🇨🇭", "z"];
    let mut state = 0x2545_f491_u32;
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state
    };
    let mut buf = buffer(11);
    for step in 0..4000 {
        let text = samples[(next() % samples.len() as u32) as usize];
        let x = (next() % 12) as u16;
        match next() % 3 {
            0 => {
                write(&mut buf, x, text);
            }
            1 => buf.put(x, 0, Cell { ch: text.chars().next().unwrap(), width: (next() % 3) as u8, ..blank() }),
            _ => buf.fill_rect(Rect::new(x, 0, (next() % 4) as u16, 1), blank()),
        }
        assert_paired(&buf, &format!("step {step}"));
    }
}

#[test]
fn diff_sees_a_changed_trailing_scalar_and_repaints_resized_buffers_row_by_row() {
    let mut before = buffer(6);
    let mut after = buffer(6);
    write(&mut before, 0, "🖥");
    write(&mut after, 0, "🖥\u{fe0f}");
    assert_eq!(before.get(0, 0).unwrap().ch, after.get(0, 0).unwrap().ch);
    assert_eq!(diff(&before, &after), vec![DiffRun { y: 0, x: 0, len: 2 }], "the VS16 widens the glyph, so lead and continuation are repainted");
    let mut same_width = buffer(6);
    write(&mut same_width, 0, "e");
    let mut with_accent = buffer(6);
    write(&mut with_accent, 0, "e\u{301}");
    assert_eq!(diff(&same_width, &with_accent), vec![DiffRun { y: 0, x: 0, len: 1 }], "a combining mark changes only the tail of a narrow cell");

    let small = CellBuffer::new(Size { width: 3, height: 2 }, blank());
    let large = CellBuffer::new(Size { width: 4, height: 3 }, blank());
    assert_eq!(diff(&small, &large), (0..3).map(|y| DiffRun { y, x: 0, len: 4 }).collect::<Vec<_>>());
}

#[test]
fn scalar_mode_counts_like_wcwidth_and_drops_joiners_and_selectors() {
    let mut buf = buffer(8);
    buf.set_width_mode(WidthMode::Scalar);
    let consumed = write(&mut buf, 0, "🖥\u{fe0f}x");
    assert_eq!(consumed, 2, "the desktop computer is one cell without its variation selector");
    assert_eq!(buf.row_text(0).trim_end(), "🖥x");
    buf.resize(Size { width: 8, height: 1 }, blank());
    assert_eq!(buf.width_mode(), WidthMode::Scalar, "a resize keeps the width mode");
}

#[test]
fn put_str_clips_at_the_rect_and_returns_the_cells_it_consumed() {
    let mut buf = buffer(8);
    let clip = Rect::new(1, 0, 5, 1);
    let consumed = buf.put_str(Pos { x: 1, y: 0 }, "🧰\u{fe0f}framework", [1, 1, 1], [2, 2, 2], 0, clip);
    assert_eq!(consumed, 5);
    assert_eq!(buf.row_text(0), " 🧰\u{fe0f}fra  ");
}
//#endregion 🔲️Cells
