//! 🍳️ The 16 witnessable-mutation recipes. Every BEFORE and AFTER `NoteDoc` below is independently
//! authored by hand — never derived by executing note's own mutation semantics — exactly the "author
//! both states directly" shape `…💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/✳️any/🏭️generator/📜️script.ts`
//! uses. `carriers` names exactly the files `generate` writes for that recipe, taken verbatim from
//! `../../🧪️oracle/🔣️.json`'s `mutationManifests`/the carrier table in
//! `📓️note-layout-carrier-oracle-findings.md` — never all three blindly: a mutation whose carrier
//! list is `svg` only (e.g. `move-block`) gets no `.dxf`/`.pdf` pair, because DXF/PDF would be
//! byte-identical before/after for it (DXF/PDF never read `x`/`y`/`rotation` at all) and a fixture
//! pair with nothing to witness would prove nothing.

use crate::{Asset, Block, NoteDoc, TINY_PNG, TINY_SVG};
use std::collections::BTreeMap;

pub struct Recipe {
    pub id: &'static str,
    pub mutation: &'static str,
    pub carriers: &'static [&'static str],
    pub before: NoteDoc,
    pub after: NoteDoc,
}

/// 🧱️ The shared building blocks every recipe starts from: one block of each of the six kinds, plus
/// a nested `Group` (so `flatten` exercises parent-then-child ordering), plus one referenced asset.
/// Each recipe below clones this and edits exactly the field its mutation changes — the "before" and
/// "after" are two full, independently-legible documents, never a patch.
fn base_document() -> NoteDoc {
    let mut assets = BTreeMap::new();
    assets.insert("logo", Asset { mime: "image/png", bytes: TINY_PNG.to_vec() });
    NoteDoc {
        title: Some("Untitled Note".to_string()),
        assets,
        blocks: vec![
            Block::Text { id: "intro-text", x: 20.0, y: 20.0, rotation: 0.0, width: 200.0, height: 60.0, visible: true, font_size: 14.0, text: "Welcome to the note.".to_string() },
            Block::Ink { id: "sketch-ink", x: 0.0, y: 0.0, rotation: 0.0, width: 100.0, height: 100.0, visible: true, points: vec![[10.0, 10.0], [40.0, 60.0], [80.0, 20.0]], stroke_width: 2.0 },
            Block::Image { id: "logo-image", x: 250.0, y: 20.0, rotation: 0.0, width: 64.0, height: 64.0, visible: true, image_key: "logo" },
            Block::Image { id: "diagram-image", x: 250.0, y: 100.0, rotation: 0.0, width: 64.0, height: 64.0, visible: true, image_key: "diagram" },
            Block::Table { id: "data-table", x: 20.0, y: 300.0, rotation: 0.0, width: 150.0, height: 80.0, visible: true },
            Block::Math { id: "pythagoras-math", x: 200.0, y: 300.0, rotation: 0.0, width: 120.0, height: 40.0, visible: true },
            Block::Group {
                id: "callout-group",
                x: 20.0,
                y: 420.0,
                rotation: 0.0,
                width: 120.0,
                height: 80.0,
                visible: true,
                children: vec![Block::Ink { id: "callout-ink", x: 30.0, y: 430.0, rotation: 0.0, width: 40.0, height: 40.0, visible: true, points: vec![[30.0, 430.0], [50.0, 450.0], [70.0, 430.0]], stroke_width: 1.5 }],
            },
        ],
    }
}

/// 🔎️ Replaces the block matching `id` (searched recursively into `Group` children) with the result
/// of `edit`. Never a diff — a full new document, built by a full recursive walk-and-rebuild.
fn with_block(doc: &NoteDoc, id: &str, edit: impl Fn(Block) -> Block) -> NoteDoc {
    fn walk(blocks: &[Block], id: &str, edit: &dyn Fn(Block) -> Block) -> Vec<Block> {
        blocks
            .iter()
            .map(|block| {
                let matches = match block {
                    Block::Text { id: bid, .. } | Block::Ink { id: bid, .. } | Block::Image { id: bid, .. } | Block::Table { id: bid, .. } | Block::Math { id: bid, .. } | Block::Group { id: bid, .. } => *bid == id,
                };
                if matches {
                    edit(block.clone())
                } else if let Block::Group { id: gid, x, y, rotation, width, height, visible, children } = block {
                    Block::Group { id: gid, x: *x, y: *y, rotation: *rotation, width: *width, height: *height, visible: *visible, children: walk(children, id, edit) }
                } else {
                    block.clone()
                }
            })
            .collect()
    }
    NoteDoc { title: doc.title.clone(), assets: doc.assets.clone(), blocks: walk(&doc.blocks, id, &edit) }
}

fn without_blocks(doc: &NoteDoc, ids: &[&str]) -> NoteDoc {
    fn walk(blocks: &[Block], ids: &[&str]) -> Vec<Block> {
        blocks
            .iter()
            .filter(|block| {
                let bid = match block {
                    Block::Text { id, .. } | Block::Ink { id, .. } | Block::Image { id, .. } | Block::Table { id, .. } | Block::Math { id, .. } | Block::Group { id, .. } => *id,
                };
                !ids.contains(&bid)
            })
            .map(|block| match block {
                Block::Group { id, x, y, rotation, width, height, visible, children } => Block::Group { id, x: *x, y: *y, rotation: *rotation, width: *width, height: *height, visible: *visible, children: walk(children, ids) },
                other => other.clone(),
            })
            .collect()
    }
    NoteDoc { title: doc.title.clone(), assets: doc.assets.clone(), blocks: walk(&doc.blocks, ids) }
}

fn with_appended(doc: &NoteDoc, block: Block) -> NoteDoc {
    let mut blocks = doc.blocks.clone();
    blocks.push(block);
    NoteDoc { title: doc.title.clone(), assets: doc.assets.clone(), blocks }
}

pub fn recipes() -> Vec<Recipe> {
    let base = base_document();
    vec![
        // 🏷️ rename-note — pdf only (title is the first thing NoteIntoPdf concatenates; DXF/SVG never read it).
        Recipe { id: "retitles-the-document", mutation: "rename-note", carriers: &["pdf"], before: base.clone(), after: NoteDoc { title: Some("Project Kickoff Notes".to_string()), ..base.clone() } },
        // 🆕 create-asset — svg only. `diagram-image` already references key "diagram" in BOTH states;
        // before, that key is undefined so the block falls back to an outline rect; after, the newly
        // created asset resolves it into a real <image>. This is the only shape that actually witnesses
        // "creating an asset" through a carrier that reads REFERENCED, RESOLVED image bytes.
        Recipe {
            id: "adds-the-diagram-asset",
            mutation: "create-asset",
            carriers: &["svg"],
            before: base.clone(),
            after: {
                let mut assets = base.assets.clone();
                assets.insert("diagram", Asset { mime: "image/svg+xml", bytes: TINY_SVG.to_vec() });
                NoteDoc { title: base.title.clone(), blocks: base.blocks.clone(), assets }
            },
        },
        // 🔁 replace-asset-payload — svg only.
        Recipe {
            id: "swaps-the-logo-payload",
            mutation: "replace-asset-payload",
            carriers: &["svg"],
            before: base.clone(),
            after: {
                let mut assets = base.assets.clone();
                assets.insert("logo", Asset { mime: "image/svg+xml", bytes: TINY_SVG.to_vec() });
                NoteDoc { title: base.title.clone(), blocks: base.blocks.clone(), assets }
            },
        },
        // 🗑️ delete-asset — svg only. Removing "logo" drops `logo-image` back to the fallback outline.
        Recipe {
            id: "removes-the-logo-asset",
            mutation: "delete-asset",
            carriers: &["svg"],
            before: base.clone(),
            after: { let mut assets = base.assets.clone(); assets.remove("logo"); NoteDoc { title: base.title.clone(), blocks: base.blocks.clone(), assets } },
        },
        // ➕ create-block (Ink) — dxf+svg. An Ink block is the one kind that reaches BOTH carriers at
        // once; PDF is untouched by this scenario (Ink never reaches PDF) — see `deletes-the-ink-and-text-blocks`
        // below for a scenario that exercises all three of this verb-family's declared carriers.
        Recipe {
            id: "creates-an-ink-block",
            mutation: "create-block",
            carriers: &["dxf", "svg"],
            before: base.clone(),
            after: with_appended(&base, Block::Ink { id: "new-ink", x: 300.0, y: 300.0, rotation: 0.0, width: 60.0, height: 60.0, visible: true, points: vec![[300.0, 300.0], [330.0, 330.0], [360.0, 300.0]], stroke_width: 2.0 }),
        },
        // ❌ delete-block (Text) — svg+pdf. Text reaches PDF+SVG, never DXF.
        Recipe { id: "deletes-the-intro-text-block", mutation: "delete-block", carriers: &["svg", "pdf"], before: base.clone(), after: without_blocks(&base, &["intro-text"]) },
        // 🧺 delete-blocks (Ink + Text together) — dxf+svg+pdf, all three: this plural verb removes two
        // blocks in one mutation, so a single recipe can legitimately witness every declared carrier.
        Recipe { id: "deletes-the-ink-and-text-blocks", mutation: "delete-blocks", carriers: &["dxf", "svg", "pdf"], before: base.clone(), after: without_blocks(&base, &["sketch-ink", "intro-text"]) },
        // 🎯 duplicate-block (Ink) — dxf+svg.
        Recipe {
            id: "duplicates-the-ink-block",
            mutation: "duplicate-block",
            carriers: &["dxf", "svg"],
            before: base.clone(),
            after: with_appended(&base, Block::Ink { id: "sketch-ink-copy", x: 0.0, y: 0.0, rotation: 0.0, width: 100.0, height: 100.0, visible: true, points: vec![[10.0, 10.0], [40.0, 60.0], [80.0, 20.0]], stroke_width: 2.0 }),
        },
        // 👥 duplicate-blocks (Ink + Text together) — dxf+svg+pdf, all three.
        Recipe {
            id: "duplicates-the-ink-and-text-blocks",
            mutation: "duplicate-blocks",
            carriers: &["dxf", "svg", "pdf"],
            before: base.clone(),
            after: with_appended(
                &with_appended(&base, Block::Ink { id: "sketch-ink-copy", x: 0.0, y: 0.0, rotation: 0.0, width: 100.0, height: 100.0, visible: true, points: vec![[10.0, 10.0], [40.0, 60.0], [80.0, 20.0]], stroke_width: 2.0 }),
                Block::Text { id: "intro-text-copy", x: 20.0, y: 90.0, rotation: 0.0, width: 200.0, height: 60.0, visible: true, font_size: 14.0, text: "Welcome to the note.".to_string() },
            ),
        },
        // 🤏 drag-blocks — svg only. Drags `callout-group`'s WHOLE subtree (the group node itself and
        // its child ink) by the same (+20,+20) offset — `note_block_transform` reads each node's own
        // x/y independently, so both the group's rect and the child's path move.
        Recipe {
            id: "drags-the-callout-group-subtree",
            mutation: "drag-blocks",
            carriers: &["svg"],
            before: base.clone(),
            after: with_block(
                &with_block(&base, "callout-group", |b| if let Block::Group { id, x, y, rotation, width, height, visible, children } = b { Block::Group { id, x: x + 20.0, y: y + 20.0, rotation, width, height, visible, children } } else { b }),
                "callout-ink",
                |b| if let Block::Ink { id, x, y, rotation, width, height, visible, points, stroke_width } = b { Block::Ink { id, x: x + 20.0, y: y + 20.0, rotation, width, height, visible, points: points.iter().map(|p| [p[0] + 20.0, p[1] + 20.0]).collect(), stroke_width } } else { b },
            ),
        },
        // 📍 move-block — svg only.
        Recipe {
            id: "moves-the-math-block",
            mutation: "move-block",
            carriers: &["svg"],
            before: base.clone(),
            after: with_block(&base, "pythagoras-math", |b| if let Block::Math { id, x: _, y: _, rotation, width, height, visible } = b { Block::Math { id, x: 260.0, y: 340.0, rotation, width, height, visible } } else { b }),
        },
        // ↔️ resize-block — svg only.
        Recipe {
            id: "resizes-the-image-block",
            mutation: "resize-block",
            carriers: &["svg"],
            before: base.clone(),
            after: with_block(&base, "logo-image", |b| if let Block::Image { id, x, y, rotation, width: _, height: _, visible, image_key } = b { Block::Image { id, x, y, rotation, width: 96.0, height: 96.0, visible, image_key } } else { b }),
        },
        // 👀 change-block-visible — svg ONLY (the confirmed DXF/PDF cross-carrier bug: neither ever
        // filters by visibility, so a Text block chosen here would read IDENTICAL in a PDF probe before
        // and after — reported, not worked around, hence no pdf pair for this mutation).
        Recipe {
            id: "hides-the-intro-text-block",
            mutation: "change-block-visible",
            carriers: &["svg"],
            before: base.clone(),
            after: with_block(&base, "intro-text", |b| if let Block::Text { id, x, y, rotation, width, height, visible: _, font_size, text } = b { Block::Text { id, x, y, rotation, width, height, visible: false, font_size, text } } else { b }),
        },
        // 📝 edit-block-text — pdf+svg.
        Recipe {
            id: "edits-the-intro-paragraph",
            mutation: "edit-block-text",
            carriers: &["pdf", "svg"],
            before: base.clone(),
            after: with_block(&base, "intro-text", |b| if let Block::Text { id, x, y, rotation, width, height, visible, font_size, text: _ } = b { Block::Text { id, x, y, rotation, width, height, visible, font_size, text: "Project kickoff scheduled for Monday.".to_string() } } else { b }),
        },
        // 🖊️ change-block-ink-width — svg ONLY (DXF's `Line` entity carries no width field at all).
        Recipe {
            id: "thickens-the-sketch-stroke",
            mutation: "change-block-ink-width",
            carriers: &["svg"],
            before: base.clone(),
            after: with_block(&base, "sketch-ink", |b| if let Block::Ink { id, x, y, rotation, width, height, visible, points, stroke_width: _ } = b { Block::Ink { id, x, y, rotation, width, height, visible, points, stroke_width: 5.0 } } else { b }),
        },
        // 🎨 edit-block-ink-stroke — dxf+svg (rewrites `points` itself, so both carriers see it).
        Recipe {
            id: "redraws-the-sketch-polyline",
            mutation: "edit-block-ink-stroke",
            carriers: &["dxf", "svg"],
            before: base.clone(),
            after: with_block(&base, "sketch-ink", |b| if let Block::Ink { id, x, y, rotation, width, height, visible, points: _, stroke_width } = b { Block::Ink { id, x, y, rotation, width, height, visible, points: vec![[10.0, 10.0], [50.0, 70.0], [90.0, 30.0]], stroke_width } } else { b }),
        },
    ]
}
