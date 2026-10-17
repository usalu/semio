//! 🌱️ Demo cases: the two test snapshots and the representative presentation diff cases. See [`super`].

use super::*;

//#region 🔖️HandcraftedDiffCodec
//#region 🔖️Demo
/// 🌱 Representative `SemioPresentationDiff` fixtures — promoted to module scope (from the old
/// `mod handcrafted_diff_codec_tests`-local helpers) so `🎹️composer/🦀️.rs`'s conformance
/// laws AND `🧬️mutations/🦀️.rs`'s own test fixtures can reuse them, same promotion model
/// every prior semio wave uses.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_a() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![
            SlideMaster { id: "keep".into(), shapes: vec![SlideShape::Placeholder { frame: SlideFrame { origin: SemioPoint2 { x: 0.0, y: 0.0 }, width: 10.0, height: 10.0 }, kind: PlaceholderKind::Title }] },
            SlideMaster { id: "toRemove".into(), shapes: Vec::new() },
        ],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "toRemove".into(), shapes: Vec::new() }],
        slides: vec![
            Slide { id: "s1".into(), layout_id: None, shapes: vec![SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 1.0 }, width: 5.0, height: 5.0 }, blocks: vec![DocBlock::paragraph("old")] }], notes: Vec::new() },
            Slide { id: "toDrop".into(), layout_id: Some("layout1".into()), shapes: Vec::new(), notes: Vec::new() },
        ],
    }
}

#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn snapshot_b() -> SemioPresentationSnapshot {
    SemioPresentationSnapshot {
        schema: "s.stdio.semio.presentation".into(),
        masters: vec![SlideMaster { id: "keep".into(), shapes: Vec::new() }, SlideMaster { id: "added".into(), shapes: Vec::new() }],
        layouts: vec![SlideLayout { id: "layout1".into(), master_id: "keep".into(), shapes: Vec::new() }],
        slides: vec![Slide {
            id: "s1".into(),
            layout_id: Some("layout1".into()),
            shapes: vec![
                SlideShape::TextBox { frame: SlideFrame { origin: SemioPoint2 { x: 1.0, y: 1.0 }, width: 5.0, height: 5.0 }, blocks: vec![DocBlock::paragraph("new")] },
                SlideShape::Picture { frame: SlideFrame { origin: SemioPoint2::default(), width: 1.0, height: 1.0 }, image: SlidePictureImage { asset_id: "a".into(), mime: "image/png".into(), bytes: vec![9] } },
            ],
            notes: vec![DocBlock::paragraph("noted")],
        }],
    }
}

/// 🌱 Representative `SemioPresentationDiff` cases built declaratively (empty/no-op and an empty-but-present row triple per collection) — single source of truth for `diff_grammar_conformance_law`/`protocol_walk_law` in
/// `🎹️composer/🦀️.rs`.
#[cfg(any(test, feature = "testkit"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_diff_cases() -> Vec<SemioPresentationDiff> {
    vec![SemioPresentationDiff::default(), SemioPresentationDiff { masters: Some(Default::default()), layouts: Some(Default::default()), slides: Some(Default::default()) }]
}
//#endregion 🔖️Demo
//#endregion 🔖️HandcraftedDiffCodec
