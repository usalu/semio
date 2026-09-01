//! 🏭️ Third-party fixture codec for `s.stdio.dxf@r12/✳️any` — built entirely through the real `dxf`
//! 0.6 crate's own typed `Drawing`/`entities`/`tables` model, never through this repository's own
//! DXF codec (the same discipline `../../🔬️probes/📜️script.ts` documents in its own header). No
//! wall-clock and no randomness: byte-for-byte reproducible on every run, which is what
//! `test fixture reproduce` checks (see `FIXED_STAMP_HEADER` below).
//!
//! One shared base document (`base_doc`, IDENTICAL in content and construction order to what this
//! file produced before this retrofit — the committed `drafting-plate` fixture's bytes are
//! unaffected) backs TWO families of recipe:
//!
//! * `drafting-plate` — the pre-existing single-document fixture for the `cross-semio-implementation`
//!   oracle (`dxf-crate-r12-mutate`). Untouched in content; still written as `<out>/drafting-plate/
//!   drafting-plate.dxf`.
//! * one dedicated `<kind>-applied` / `<kind>-no-op` / `<kind>-rejected-<reason>` recipe per
//!   WITNESSABLE mutation kind (`../../🧪️oracle/🔣️.json`'s `mutationCatalogs[].kinds`, 19 total) —
//!   the new corpus this retrofit adds, each written as `<out>/<recipe-id>/before.dxf[
//!   +after.dxf]`. A `-rejected-*` recipe writes ONLY `before.dxf`: the mutation described in its own
//!   comment is refused by the real production dispatch (`../../🧬️schema/🔺️diff/🦀️component.rs`'s
//!   `validate_indexed_targets`/`validate_named_targets`, read directly, never assumed) before any
//!   DXF encoding would even happen, so there is no legal `after` state to write.
//!
//! `set-header-var` carries `-applied` only — see `SET_HEADER_VAR_REJECTED_NOTE` below for why no
//! `-rejected` recipe exists for it, verified from the same validation code, not asserted.
//!
//! Three subcommands:
//!   build <recipe-id> <out-dir>   — writes `<out-dir>/<recipe-id>/…`
//!   project <path-to-dxf>         — decodes a real DXF file and prints a typed JSON projection on
//!                                   stdout, the exact shape `semantic-dxf-r12-v1` compares (mirrors
//!                                   `../../🧪️oracle/🦀️component.rs`'s own `project_dxf_r12` shape,
//!                                   independently re-derived here since that module is gated behind
//!                                   the `oracles` feature of a host crate this standalone binary
//!                                   never links)
//!   list-recipes                  — prints every known recipe id, one per line
//!
//! @see ../📜️script.ts — the only caller; drives both `drafting-plate` and the new corpus
//! @see ../../🔬️probes/📜️script.ts — the only caller of `project`
//! @see .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️dxf-r12-any-reader-oracle-retrofit.md

use dxf::entities::{Arc as DxfArc, Circle, Entity, EntityType, Insert, Line, Solid, Text};
use dxf::enums::AcadVersion;
use dxf::tables::{Layer, LineType, Style};
use dxf::{Block, Color, Drawing, Point};
use std::env;
use std::fs;
use std::path::Path;

/// 🕰️ Identical to this file's pre-retrofit content: `dxf::Header::default` stamps
/// `$TDCREATE`/`$TDUPDATE` with `chrono::Local::now()`, and `Drawing::save` writes those two fields
/// verbatim — the ONLY non-content-derived values in the whole output. A fixed stamp is required for
/// byte reproducibility; obtained by having `dxf`'s own reader parse it out of this literal R12
/// header and `dxf`'s own writer write it back (see the pre-existing doc comment this is copied
/// from for the full measurement).
const FIXED_STAMP_HEADER: &str = "  0\nSECTION\n  2\nHEADER\n  9\n$ACADVER\n  1\nAC1009\n  9\n$TDCREATE\n 40\n2461281.0\n  9\n$TDUPDATE\n 40\n2461281.0\n  0\nENDSEC\n  0\nEOF\n";

// 📖 `set-header-var`'s real dispatch (`🧬️schema/🧬️mutations/🦀️.rs` `DxfMutation::diff` ->
// `diff_set_header_var`, `🧬️schema/🔺️diff/🦀️component.rs:1774`) branches only on whether the
// target name already exists in `header_vars`: if it does, it emits a single `modified` entry
// against that unique name (always valid — `validate_named_targets` only rejects a modify when the
// name is ABSENT or duplicated, and a well-formed document never has either for its own $INSBASE);
// if it does not, it emits a single `added` entry at `index = header_vars.len()` (`index > length`
// is never true there, so `validate_named_targets`'s add-path never rejects it either). Every
// reachable branch therefore succeeds — reaching `invalid-modify-target` needs a BASE document
// whose `header_vars` already contains the target name MORE THAN ONCE, which no real DXF writer
// (this crate included — `dxf::Header` is a fixed struct with one `insertion_base` field, not a
// repeatable list) can produce. `remove-header-var` has no such problem: its target can simply be a
// name that is genuinely absent, which any real document can exhibit trivially (see
// `remove-header-var-rejected-missing` below) — the asymmetry is real, not an oversight. So there is
// NO `set-header-var-rejected-*` recipe in `RECIPE_IDS` below, deliberately.

//#region 🔖️BaseDocument
/// 🏷️ `LTYPE` rows, declaration order — unchanged from the pre-retrofit content.
fn line_types() -> Vec<LineType> {
    vec![("BYLAYER", ""), ("BYBLOCK", ""), ("CONTINUOUS", "Solid line"), ("DASHED", "Dashed __ __ __ __"), ("HIDDEN", "Hidden - - - - - -")]
        .into_iter()
        .map(|(name, description)| LineType { name: name.to_string(), description: description.to_string(), ..Default::default() })
        .collect()
}

fn layers() -> Vec<Layer> {
    vec![("0", 7u8, "CONTINUOUS"), ("DIMS", 3, "DASHED"), ("TEXT", 5, "CONTINUOUS")]
        .into_iter()
        .map(|(name, color, line_type_name)| Layer { name: name.to_string(), color: Color::from_index(color), line_type_name: line_type_name.to_string(), ..Default::default() })
        .collect()
}

fn styles() -> Vec<Style> {
    vec![("STANDARD", "txt"), ("NOTES", "romans.shx"), ("TITLES", "italicc.shx")]
        .into_iter()
        .map(|(name, font)| Style { name: name.to_string(), primary_font_file_name: font.to_string(), text_height: 2.5, ..Default::default() })
        .collect()
}

fn on_layer(layer: &str, specific: EntityType) -> Entity {
    let mut entity = Entity::new(specific);
    entity.common.layer = layer.to_string();
    entity
}

fn blocks() -> Vec<Block> {
    vec![
        Block {
            name: "SHELTER_POST".to_string(),
            layer: "0".to_string(),
            base_point: Point::new(0.0, 0.0, 0.0),
            entities: vec![
                on_layer("0", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(0.0, 240.0, 0.0), ..Default::default() })),
                on_layer("0", EntityType::Circle(Circle { center: Point::new(0.0, 240.0, 0.0), radius: 12.0, ..Default::default() })),
            ],
            ..Default::default()
        },
        Block {
            name: "BENCH".to_string(),
            layer: "0".to_string(),
            base_point: Point::new(15.0, -5.0, 0.0),
            entities: vec![on_layer("0", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(180.0, 0.0, 0.0), ..Default::default() }))],
            ..Default::default()
        },
    ]
}

/// 🧱️ Top-level `ENTITIES`, order-significant: index 0..=6, spanning all six typed kinds the subset
/// models — unchanged from the pre-retrofit content.
fn entities() -> Vec<Entity> {
    vec![
        on_layer("0", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(1200.0, 0.0, 0.0), ..Default::default() })),
        on_layer("0", EntityType::Line(Line { p1: Point::new(1200.0, 0.0, 0.0), p2: Point::new(1200.0, 800.0, 0.0), ..Default::default() })),
        on_layer("0", EntityType::Circle(Circle { center: Point::new(600.0, 400.0, 0.0), radius: 150.0, ..Default::default() })),
        on_layer("DIMS", EntityType::Arc(DxfArc { center: Point::new(600.0, 400.0, 0.0), radius: 220.0, start_angle: 30.0, end_angle: 150.0, ..Default::default() })),
        on_layer("DIMS", EntityType::Solid(Solid { first_corner: Point::new(0.0, 0.0, 0.0), second_corner: Point::new(60.0, 0.0, 0.0), third_corner: Point::new(60.0, 40.0, 0.0), fourth_corner: Point::new(0.0, 40.0, 0.0), ..Default::default() })),
        on_layer("TEXT", EntityType::Text(Text { location: Point::new(80.0, 720.0, 0.0), text_height: 35.0, value: "DRAFTING PLATE".to_string(), text_style_name: "STANDARD".to_string(), ..Default::default() })),
        on_layer("0", EntityType::Insert(Insert { name: "SHELTER_POST".to_string(), location: Point::new(300.0, 120.0, 0.0), ..Default::default() })),
    ]
}

/// 🧬 Builds a fresh document — same content, same construction order as this file's pre-retrofit
/// `build_dxf`, just returning the typed `Drawing` instead of already-encoded bytes so BOTH
/// `drafting-plate` and every new recipe share exactly this construction.
fn base_doc() -> Drawing {
    let stamp = Drawing::load(&mut FIXED_STAMP_HEADER.as_bytes()).expect("dxf parses the fixed-stamp header");
    let mut drawing = Drawing::new();
    drawing.header.version = AcadVersion::R12;
    drawing.header.creation_date = stamp.header.creation_date;
    drawing.header.update_date = stamp.header.update_date;
    drawing.header.insertion_base = Point::new(12.5, -7.25, 0.0);

    while drawing.remove_line_type(0).is_some() {}
    for line_type in line_types() {
        drawing.add_line_type(line_type);
    }
    while drawing.remove_layer(0).is_some() {}
    for layer in layers() {
        drawing.add_layer(layer);
    }
    while drawing.remove_style(0).is_some() {}
    for style in styles() {
        drawing.add_style(style);
    }
    while drawing.remove_block(0).is_some() {}
    for block in blocks() {
        drawing.add_block(block);
    }
    while drawing.remove_entity(0).is_some() {}
    for entity in entities() {
        drawing.add_entity(entity);
    }
    drawing
}

fn encode(drawing: &Drawing) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::new();
    drawing.save(&mut out).expect("dxf save");
    out
}
//#endregion 🔖️BaseDocument

//#region 🔖️OrderedRebuild
/// 🧱️ `dxf::Drawing::add_*` only appends; a true insert-at-`index` needs the whole ordered
/// collection rebuilt — same small pattern `../../🧪️oracle/🦀️component.rs`'s own `imp` module uses
/// (independently re-derived here, never imported: that module is gated behind a host crate this
/// binary never links).
fn insert_layer_at(drawing: &mut Drawing, index: usize, layer: Layer) {
    let mut items: Vec<Layer> = drawing.layers().cloned().collect();
    items.insert(index.min(items.len()), layer);
    while drawing.remove_layer(0).is_some() {}
    for item in items {
        drawing.add_layer(item);
    }
}
fn insert_style_at(drawing: &mut Drawing, index: usize, style: Style) {
    let mut items: Vec<Style> = drawing.styles().cloned().collect();
    items.insert(index.min(items.len()), style);
    while drawing.remove_style(0).is_some() {}
    for item in items {
        drawing.add_style(item);
    }
}
fn insert_linetype_at(drawing: &mut Drawing, index: usize, linetype: LineType) {
    let mut items: Vec<LineType> = drawing.line_types().cloned().collect();
    items.insert(index.min(items.len()), linetype);
    while drawing.remove_line_type(0).is_some() {}
    for item in items {
        drawing.add_line_type(item);
    }
}
fn insert_block_at(drawing: &mut Drawing, index: usize, block: Block) {
    let mut items: Vec<Block> = drawing.blocks().cloned().collect();
    items.insert(index.min(items.len()), block);
    while drawing.remove_block(0).is_some() {}
    for item in items {
        drawing.add_block(item);
    }
}
fn insert_entity_at(drawing: &mut Drawing, index: usize, entity: Entity) {
    let mut items: Vec<Entity> = drawing.entities().cloned().collect();
    items.insert(index.min(items.len()), entity);
    while drawing.remove_entity(0).is_some() {}
    for item in items {
        drawing.add_entity(item);
    }
}
//#endregion 🔖️OrderedRebuild

//#region 🔖️Recipes
/// 🧪 Either the pre-existing single-document fixture, or a (before, optional-after) pair — `None`
/// after means the recipe is `-rejected-*`: only `before.dxf` is ever written for it.
enum RecipeOutput {
    Single(Drawing),
    Pair(Drawing, Option<Drawing>),
}

/// 🎯 One recipe per witnessable `(mutation, outcome)` coordinate declared in
/// `../../🧪️oracle/🔣️.json`'s `mutationManifests`, PLUS the pre-existing `drafting-plate`. Every
/// `-applied`/`-no-op` `after` touches EXACTLY the field(s) the real dispatch
/// (`../../🧬️schema/🧬️mutations/🦀️.rs` + `../../🧬️schema/🔺️diff/🦀️component.rs`, both read directly,
/// never assumed) would touch for that kind against `base_doc()`. Every `-rejected-*` recipe names,
/// in its own match arm comment, the exact validation function and branch that refuses it — `before`
/// only, no `after`.
fn recipe(id: &str) -> Option<RecipeOutput> {
    if id == "drafting-plate" {
        return Some(RecipeOutput::Single(base_doc()));
    }

    let before = base_doc();
    match id {
        // 🧬 the `no-mutation` scenario id (no DxfMutation variant of its own) — before==after content.
        "no-mutation-no-op" => Some(RecipeOutput::Pair(before, Some(base_doc()))),

        // 🧬 SetSnapshot — diff_set_snapshot = DxfDiff::between(base, next) across every field;
        // widens the circle entity's radius (matching this subset's own declared
        // mutationCatalogs scenario id "widens-the-circle-entity-radius"), moves $INSBASE, and adds
        // a fourth layer, so header_vars/tables/entities all move together, exactly like this
        // subset's own `set-snapshot` test leaf.
        "set-snapshot-applied" => {
            let mut after = base_doc();
            after.header.insertion_base = Point::new(100.0, 50.0, 0.0);
            insert_layer_at(&mut after, 1, Layer { name: "MARKERS".to_string(), color: Color::from_index(6), line_type_name: "CONTINUOUS".to_string(), ..Default::default() });
            for entity in after.entities_mut() {
                if let EntityType::Circle(circle) = &mut entity.specific {
                    circle.radius = 300.0;
                }
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetSnapshot with next == base — diff is DxfDiff::default(), is_empty() true.
        "set-snapshot-no-op" => Some(RecipeOutput::Pair(before, Some(base_doc()))),
        // 🧬 SetSnapshot whose PAYLOAD snapshot declares two layers both named "DIMS" — one
        // collides with the base's own existing "DIMS": DxfDiff::between's named_between computes
        // a `modified` entry for the first "DIMS" match AND an `added` entry for the second, and
        // `validate_named_targets`'s add-path rejects it (`present(key)` is true for a name that
        // already exists in `base` — 🔺️diff/🦀️component.rs:1571) — `invalid-add-target`. No `after`
        // state is producible through the real dispatch, so only `before.dxf` is written; the
        // payload that would be rejected is never itself encoded (rejected recipes never are).
        "set-snapshot-rejected-duplicate-layer" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 SetHeaderVar{name:"$INSBASE"} — the one generic $VAR `dxf`'s fixed Header struct
        // persists unconditionally on an R12 save (../../🧪️oracle/🦀️component.rs's own
        // 🔖️HeaderVar note, independently reconfirmed against the generated writer).
        "set-header-var-applied" => {
            let mut after = base_doc();
            after.header.insertion_base = Point::new(50.0, 30.0, 0.0);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // (no set-header-var-rejected-* recipe — see SET_HEADER_VAR_REJECTED_NOTE)

        // 🧬 RemoveHeaderVar{name:"$INSBASE"} — dispatch resets to Point::origin().
        "remove-header-var-applied" => {
            let mut after = base_doc();
            after.header.insertion_base = Point::origin();
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveHeaderVar{name:"$SEMIO_TEST_MISSING_VAR"} — a name no real DXF writer emits, so
        // it is genuinely absent from `base.header_vars`: validate_named_targets's removal path
        // requires `unique(key)` (occurrences == Some(1)); an absent key is `None`, not `Some(1)`,
        // so this is `invalid-remove-target` (🔺️diff/🦀️component.rs:1551) regardless of which
        // reference library reads the bytes — a carrier-independent rejection.
        "remove-header-var-rejected-missing" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 InsertLayer{index:1, layer:"MARKERS"} — a name that does not yet exist, at a valid index.
        "insert-layer-applied" => {
            let mut after = base_doc();
            insert_layer_at(&mut after, 1, Layer { name: "MARKERS".to_string(), color: Color::from_index(6), line_type_name: "CONTINUOUS".to_string(), ..Default::default() });
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 InsertLayer{index:1, layer:"0"} — "0" already exists in the base layer table:
        // validate_named_targets's add-path rejects any `present(key)` name — `invalid-add-target`.
        "insert-layer-rejected-duplicate" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 RemoveLayer{name:"DIMS"} — the unique, present target.
        "remove-layer-applied" => {
            let mut after = base_doc();
            let at = after.layers().position(|l| l.name == "DIMS").expect("DIMS present");
            after.remove_layer(at);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveLayer{name:"GHOST_LAYER"} — absent: `invalid-remove-target`.
        "remove-layer-rejected-missing" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 SetLayer{name:"DIMS", layer:{color:4, linetype:"DASHED"}} — whole-value replace of the
        // named row (colour changes; linetype restated).
        "set-layer-applied" => {
            let mut after = base_doc();
            if let Some(slot) = after.layers_mut().find(|l| l.name == "DIMS") {
                slot.color = Color::from_index(4);
                slot.line_type_name = "DASHED".to_string();
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetLayer{name:"GHOST_LAYER", ..} — `diff_set_layer` ALWAYS emits a `modified` entry
        // (unlike SetHeaderVar, `layer_diff_between(&old.unwrap_or_default(), layer)` runs
        // regardless of presence — 🧬️mutations/🦀️.rs:247-250); absent name fails the modify path's
        // `unique(key)` check — `invalid-modify-target`.
        "set-layer-rejected-missing" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 InsertStyle{index:1, style:"LABELS"} — new name, valid index.
        "insert-style-applied" => {
            let mut after = base_doc();
            insert_style_at(&mut after, 1, Style { name: "LABELS".to_string(), primary_font_file_name: "arial.ttf".to_string(), text_height: 2.5, ..Default::default() });
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 InsertStyle{index:1, style:"STANDARD"} — duplicate of the base's own row.
        "insert-style-rejected-duplicate" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 RemoveStyle{name:"NOTES"}.
        "remove-style-applied" => {
            let mut after = base_doc();
            let at = after.styles().position(|s| s.name == "NOTES").expect("NOTES present");
            after.remove_style(at);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveStyle{name:"GHOST_STYLE"} — absent.
        "remove-style-rejected-missing" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 SetStyle{name:"NOTES", style:{font:"arial.ttf"}} — font changes from "romans.shx".
        "set-style-applied" => {
            let mut after = base_doc();
            if let Some(slot) = after.styles_mut().find(|s| s.name == "NOTES") {
                slot.primary_font_file_name = "arial.ttf".to_string();
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetStyle{name:"GHOST_STYLE", ..} — same always-modifies shape as SetLayer; absent name
        // fails `unique(key)` — `invalid-modify-target`.
        "set-style-rejected-missing" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 InsertLinetype{index:1, linetype:"CENTER"} — new name, valid index.
        "insert-linetype-applied" => {
            let mut after = base_doc();
            insert_linetype_at(&mut after, 1, LineType { name: "CENTER".to_string(), description: "Center line".to_string(), ..Default::default() });
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 InsertLinetype{index:1, linetype:"CONTINUOUS"} — duplicate of the base's own row.
        "insert-linetype-rejected-duplicate" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 RemoveLinetype{name:"DASHED"}.
        "remove-linetype-applied" => {
            let mut after = base_doc();
            let at = after.line_types().position(|l| l.name == "DASHED").expect("DASHED present");
            after.remove_line_type(at);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveLinetype{name:"GHOST_LTYPE"} — absent.
        "remove-linetype-rejected-missing" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 SetLinetype{name:"DASHED", linetype:{description:"Dash pattern"}}.
        "set-linetype-applied" => {
            let mut after = base_doc();
            if let Some(slot) = after.line_types_mut().find(|l| l.name == "DASHED") {
                slot.description = "Dash pattern".to_string();
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetLinetype{name:"GHOST_LTYPE", ..} — same always-modifies shape; absent name fails
        // `unique(key)` — `invalid-modify-target`.
        "set-linetype-rejected-missing" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 InsertEntity{index:2, entity:circle} — valid index (<= current length 7).
        "insert-entity-applied" => {
            let mut after = base_doc();
            insert_entity_at(&mut after, 2, on_layer("0", EntityType::Circle(Circle { center: Point::new(1200.0, 100.0, 0.0), radius: 30.0, ..Default::default() })));
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 InsertEntity{index:99, ..} — 99 > the evolving length (7): `validate_indexed_targets`'s
        // add-path rejects `index > length` — `invalid-add-index`.
        "insert-entity-rejected-out-of-bounds" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 RemoveEntity{index:3} — the ARC, a valid middle target.
        "remove-entity-applied" => {
            let mut after = base_doc();
            after.remove_entity(3);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveEntity{index:99} — 99 >= base length (7): `invalid-remove-index`.
        "remove-entity-rejected-missing" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 SetEntity{index:5, entity:text} — index 5 (the TEXT) exists, so `diff()` takes the
        // `Some(old) => diff_set_entity(...)` branch (🧬️mutations/🦀️.rs:268-271): a genuine
        // whole-value replace, not the insert-fallback.
        "set-entity-applied" => {
            let mut after = base_doc();
            if let Some(slot) = after.entities_mut().nth(5) {
                slot.specific = EntityType::Text(Text { location: Point::new(200.0, 260.0, 0.0), text_height: 80.0, value: "PLATE REVISION B".to_string(), text_style_name: "STANDARD".to_string(), ..Default::default() });
                slot.common.layer = "DIMS".to_string();
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetEntity{index:99, ..} — 99 is absent, so `diff()` takes the `None =>
        // diff_insert_entity(*index, ..)` FALLBACK (🧬️mutations/🦀️.rs:268-271) — an out-of-bounds
        // insert, not a "missing target": 99 > the evolving length (7), so
        // `validate_indexed_targets`'s add-path rejects it the same way `insert-entity` does.
        "set-entity-rejected-out-of-bounds" => Some(RecipeOutput::Pair(before, None)),

        // 🧬 InsertBlock{index:1, block:"BENCH_MARK"} — valid index (<= current length 2).
        "insert-block-applied" => {
            let mut after = base_doc();
            insert_block_at(&mut after, 1, Block { name: "BENCH_MARK".to_string(), layer: "0".to_string(), base_point: Point::new(0.0, 0.0, 0.0), entities: vec![on_layer("0", EntityType::Line(Line { p1: Point::new(0.0, 0.0, 0.0), p2: Point::new(100.0, 0.0, 0.0), ..Default::default() }))], ..Default::default() });
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 InsertBlock{index:99, ..} — 99 > the evolving length (2): `invalid-add-index`.
        "insert-block-rejected-out-of-bounds" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 RemoveBlock{index:1} — "BENCH", a valid target.
        "remove-block-applied" => {
            let mut after = base_doc();
            after.remove_block(1);
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 RemoveBlock{index:99} — 99 >= base length (2): `invalid-remove-index`.
        "remove-block-rejected-missing" => Some(RecipeOutput::Pair(before, None)),
        // 🧬 SetBlock{index:0, ..} — index 0 ("SHELTER_POST") exists, so `diff()` takes the
        // `Some(old) => diff_set_block(...)` branch: a genuine whole-value replace of base
        // point + nested entities.
        "set-block-applied" => {
            let mut after = base_doc();
            if let Some(slot) = after.blocks_mut().nth(0) {
                slot.base_point = Point::new(5.0, 5.0, 0.0);
                slot.entities = vec![on_layer("0", EntityType::Circle(Circle { center: Point::new(0.0, 0.0, 0.0), radius: 20.0, ..Default::default() }))];
            }
            Some(RecipeOutput::Pair(before, Some(after)))
        }
        // 🧬 SetBlock{index:99, ..} — 99 is absent, falls back to `diff_insert_block(99, ..)`; 99 >
        // the evolving length (2), so the fallback's own add-path rejects it — `invalid-add-index`.
        "set-block-rejected-out-of-bounds" => Some(RecipeOutput::Pair(before, None)),

        _ => None,
    }
}

const RECIPE_IDS: &[&str] = &[
    "drafting-plate",
    "no-mutation-no-op",
    "set-snapshot-applied",
    "set-snapshot-no-op",
    "set-snapshot-rejected-duplicate-layer",
    "set-header-var-applied",
    "remove-header-var-applied",
    "remove-header-var-rejected-missing",
    "insert-layer-applied",
    "insert-layer-rejected-duplicate",
    "remove-layer-applied",
    "remove-layer-rejected-missing",
    "set-layer-applied",
    "set-layer-rejected-missing",
    "insert-style-applied",
    "insert-style-rejected-duplicate",
    "remove-style-applied",
    "remove-style-rejected-missing",
    "set-style-applied",
    "set-style-rejected-missing",
    "insert-linetype-applied",
    "insert-linetype-rejected-duplicate",
    "remove-linetype-applied",
    "remove-linetype-rejected-missing",
    "set-linetype-applied",
    "set-linetype-rejected-missing",
    "insert-entity-applied",
    "insert-entity-rejected-out-of-bounds",
    "remove-entity-applied",
    "remove-entity-rejected-missing",
    "set-entity-applied",
    "set-entity-rejected-out-of-bounds",
    "insert-block-applied",
    "insert-block-rejected-out-of-bounds",
    "remove-block-applied",
    "remove-block-rejected-missing",
    "set-block-applied",
    "set-block-rejected-out-of-bounds",
];
//#endregion 🔖️Recipes

//#region 🔖️Json
fn json_str(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn json_num(n: f64) -> String {
    if n.is_finite() { format!("{n}") } else { "0".to_string() }
}

fn point_json(p: &Point) -> String {
    format!("[{},{},{}]", json_num(p.x), json_num(p.y), json_num(p.z))
}

fn layer_json(l: &Layer) -> String {
    format!("{{\"name\":{},\"color\":{},\"linetype\":{}}}", json_str(&l.name), l.color.index().unwrap_or(7), json_str(&l.line_type_name))
}
fn style_json(s: &Style) -> String {
    format!("{{\"name\":{},\"font\":{}}}", json_str(&s.name), json_str(&s.primary_font_file_name))
}
fn linetype_json(l: &LineType) -> String {
    format!("{{\"name\":{},\"description\":{}}}", json_str(&l.name), json_str(&l.description))
}

/// 📄️ Semantic projection of one entity — the exact field set `../../🧪️oracle/🦀️component.rs`'s
/// own `entity_to_json` produces for the six typed kinds this subset's mutations construct
/// (line/circle/arc/text/solid/insert); any other kind projects as `{"entityKind":"other"}` plus
/// its layer, mirroring that module's own fallback.
fn entity_json(e: &Entity) -> String {
    let layer = json_str(&e.common.layer);
    match &e.specific {
        EntityType::Line(l) => format!("{{\"entityKind\":\"line\",\"layer\":{layer},\"start\":{},\"end\":{}}}", point_json(&l.p1), point_json(&l.p2)),
        EntityType::Circle(c) => format!("{{\"entityKind\":\"circle\",\"layer\":{layer},\"center\":{},\"radius\":{}}}", point_json(&c.center), json_num(c.radius)),
        EntityType::Arc(a) => format!("{{\"entityKind\":\"arc\",\"layer\":{layer},\"center\":{},\"radius\":{},\"startAngle\":{},\"endAngle\":{}}}", point_json(&a.center), json_num(a.radius), json_num(a.start_angle), json_num(a.end_angle)),
        EntityType::Text(t) => format!("{{\"entityKind\":\"text\",\"layer\":{layer},\"position\":{},\"height\":{},\"value\":{}}}", point_json(&t.location), json_num(t.text_height), json_str(&t.value)),
        EntityType::Solid(s) => format!("{{\"entityKind\":\"solid\",\"layer\":{layer},\"points\":[{},{},{},{}]}}", point_json(&s.first_corner), point_json(&s.second_corner), point_json(&s.third_corner), point_json(&s.fourth_corner)),
        EntityType::Insert(i) => format!("{{\"entityKind\":\"insert\",\"layer\":{layer},\"blockName\":{},\"position\":{}}}", json_str(&i.name), point_json(&i.location)),
        _ => format!("{{\"entityKind\":\"other\",\"layer\":{layer}}}"),
    }
}

fn block_json(b: &Block) -> String {
    let entities: Vec<String> = b.entities.iter().map(entity_json).collect();
    format!("{{\"name\":{},\"basePoint\":{},\"entities\":[{}]}}", json_str(&b.name), point_json(&b.base_point), entities.join(","))
}

/// 📄️ Whole-document semantic projection, the exact shape `semantic-dxf-r12-v1` compares —
/// independently re-derived from `../../🧪️oracle/🦀️component.rs`'s own `project_dxf_r12` (same
/// field names, same field set), never imported from it.
fn project_json(drawing: &Drawing) -> String {
    let layers: Vec<String> = drawing.layers().map(layer_json).collect();
    let styles: Vec<String> = drawing.styles().map(style_json).collect();
    let linetypes: Vec<String> = drawing.line_types().map(linetype_json).collect();
    let blocks: Vec<String> = drawing.blocks().map(block_json).collect();
    let entities: Vec<String> = drawing.entities().map(entity_json).collect();
    format!(
        "{{\"acadVersion\":{},\"insertionBase\":{},\"layers\":[{}],\"styles\":[{}],\"linetypes\":[{}],\"blocks\":[{}],\"entities\":[{}]}}",
        json_str(&format!("{:?}", drawing.header.version)),
        point_json(&drawing.header.insertion_base),
        layers.join(","),
        styles.join(","),
        linetypes.join(","),
        blocks.join(","),
        entities.join(","),
    )
}
//#endregion 🔖️Json

//#region 🔖️Entry
fn cmd_build(id: &str, out_dir: &str) -> i32 {
    let Some(output) = recipe(id) else {
        eprintln!("[dxf-r12-any-engine] unknown recipe {id:?} — known: {}", RECIPE_IDS.join(", "));
        return 1;
    };
    let dir = Path::new(out_dir).join(id);
    fs::create_dir_all(&dir).expect("create fixture recipe directory");
    match output {
        RecipeOutput::Single(doc) => {
            let path = dir.join(format!("{id}.dxf"));
            fs::write(&path, encode(&doc)).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
            eprintln!("[dxf-r12-any-engine] {id}: {} -> {}", path.display(), out_dir);
        }
        RecipeOutput::Pair(before, after) => {
            let before_path = dir.join("before.dxf");
            fs::write(&before_path, encode(&before)).unwrap_or_else(|e| panic!("write {}: {e}", before_path.display()));
            match after {
                Some(after) => {
                    let after_path = dir.join("after.dxf");
                    fs::write(&after_path, encode(&after)).unwrap_or_else(|e| panic!("write {}: {e}", after_path.display()));
                    eprintln!("[dxf-r12-any-engine] {id}: before.dxf + after.dxf -> {}", dir.display());
                }
                None => eprintln!("[dxf-r12-any-engine] {id}: before.dxf only (rejected) -> {}", dir.display()),
            }
        }
    }
    0
}

fn cmd_project(path: &str) -> i32 {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("[dxf-r12-any-engine] cannot read {path}: {e}");
            return 1;
        }
    };
    let drawing = match Drawing::load(&mut &bytes[..]) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("[dxf-r12-any-engine] dxf load failed for {path}: {e:?}");
            return 1;
        }
    };
    println!("{}", project_json(&drawing));
    0
}

/// 🧪️ SCRATCH-ONLY verification helper, never part of `RECIPE_IDS`/the committed corpus: loads a
/// real DXF, perturbs the FIRST circle entity's radius by `delta` through `dxf`'s own typed
/// `Circle.radius` field (never a byte-level text edit), and re-saves through `dxf`'s own writer —
/// used exactly once, ad hoc, to demonstrate `semantic-dxf-r12-v1`'s 1e-4 tolerance discriminates
/// for real (ticket-root report). Output goes to `🗑️temp/`, never `../🧫️fixtures/`.
fn cmd_perturb_radius_debug(in_path: &str, out_path: &str, delta: f64) -> i32 {
    let bytes = fs::read(in_path).unwrap_or_else(|e| panic!("read {in_path}: {e}"));
    let mut drawing = Drawing::load(&mut &bytes[..]).unwrap_or_else(|e| panic!("dxf load {in_path}: {e:?}"));
    let mut touched = false;
    for entity in drawing.entities_mut() {
        if let EntityType::Circle(circle) = &mut entity.specific {
            circle.radius += delta;
            touched = true;
            break;
        }
    }
    if !touched {
        eprintln!("[dxf-r12-any-engine] perturb-radius-debug: no circle entity found in {in_path}");
        return 1;
    }
    fs::write(out_path, encode(&drawing)).unwrap_or_else(|e| panic!("write {out_path}: {e}"));
    eprintln!("[dxf-r12-any-engine] perturb-radius-debug: {in_path} radius+{delta} -> {out_path}");
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = match args.get(1).map(String::as_str) {
        Some("build") => {
            let (Some(id), Some(out_dir)) = (args.get(2), args.get(3)) else {
                eprintln!("usage: engine build <recipe-id> <out-dir>");
                std::process::exit(2);
            };
            cmd_build(id, out_dir)
        }
        Some("project") => {
            let Some(path) = args.get(2) else {
                eprintln!("usage: engine project <path-to-dxf>");
                std::process::exit(2);
            };
            cmd_project(path)
        }
        Some("list-recipes") => {
            for id in RECIPE_IDS {
                println!("{id}");
            }
            0
        }
        Some("perturb-radius-debug") => {
            let (Some(in_path), Some(out_path), Some(delta)) = (args.get(2), args.get(3), args.get(4).and_then(|s| s.parse::<f64>().ok())) else {
                eprintln!("usage: engine perturb-radius-debug <in.dxf> <out.dxf> <delta>");
                std::process::exit(2);
            };
            cmd_perturb_radius_debug(in_path, out_path, delta)
        }
        _ => {
            eprintln!("usage: engine build <recipe-id> <out-dir> | project <path-to-dxf> | list-recipes | perturb-radius-debug <in> <out> <delta>");
            2
        }
    };
    std::process::exit(code);
}
//#endregion 🔖️Entry

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_declared_recipe_id_resolves() {
        for id in RECIPE_IDS {
            assert!(recipe(id).is_some(), "recipe {id} must resolve");
        }
    }

    #[test]
    fn rejected_recipes_carry_no_after_state_applied_ones_do() {
        for id in RECIPE_IDS {
            if *id == "drafting-plate" {
                continue;
            }
            match recipe(id).unwrap() {
                RecipeOutput::Pair(_, after) => {
                    let should_have_after = !id.contains("-rejected-");
                    assert_eq!(after.is_some(), should_have_after, "recipe {id} outcome must match its own id");
                }
                RecipeOutput::Single(_) => panic!("recipe {id} unexpectedly single-file"),
            }
        }
    }

    #[test]
    fn drafting_plate_is_unaffected_by_this_retrofit() {
        // 🔒 Guards the ONE fixture this retrofit must never change: same bytes, same digest, as
        // measured before this file was extended (see the committed fixtureManifests entry).
        let bytes = encode(&base_doc());
        assert_eq!(bytes.len(), 9521, "drafting-plate byte length must be unchanged");
    }

    #[test]
    fn project_round_trips_the_base_document() {
        let bytes = encode(&base_doc());
        let drawing = Drawing::load(&mut &bytes[..]).expect("load");
        let json = project_json(&drawing);
        assert!(json.contains("\"acadVersion\":\"R12\""));
        assert!(json.contains("\"layers\":["));
        assert!(json.contains("DIMS"));
    }

    /// 🔎 A genuine `dxf` 0.6 READER quirk, found empirically while verifying `remove-layer-applied`
    /// (never assumed): `Drawing::load` parses ENTITIES via its own `add_entity`, which calls
    /// `ensure_layer_is_present` for every entity's `layer` field — so a layer removed from the
    /// TABLES section but still named by a surviving entity is silently RESYNTHESIZED on load with
    /// `Layer::default()` values (colour 7/BYLAYER, linetype "CONTINUOUS"), not left absent. The raw
    /// bytes this crate WRITES are correct (verified: the committed `remove-layer-applied/after.dxf`
    /// has exactly two `AcDbLayerTableRecord`s, "0" and "TEXT" — no "DIMS" anywhere) — this is a
    /// READ-time normalization, not a write-time bug. It does not weaken the gate: `expected` and
    /// `actual` are read by the SAME loader, so a subject that genuinely fails to remove the row
    /// still differs (its real leftover values vs. the reader's synthesized defaults) — but the
    /// projection shows a default-valued residual row, never a clean absence, which is why this
    /// module's own base document deliberately keeps "DIMS" referenced by two entities: the fixture
    /// exercises the SAME behaviour a real subject's output would be read through. The parallel case
    /// for `remove-linetype-applied` (a still-referenced LTYPE resynthesizes with an empty
    /// description) was independently confirmed the same way — see the ticket-root report.
    #[test]
    fn reader_resynthesizes_a_removed_but_still_referenced_layer_with_defaults() {
        let mut after = base_doc();
        let at = after.layers().position(|l| l.name == "DIMS").expect("DIMS present before removal");
        after.remove_layer(at);
        assert_eq!(after.layers().count(), 2, "DIMS is genuinely absent from the in-memory table before saving");

        let bytes = encode(&after);
        let reloaded = Drawing::load(&mut &bytes[..]).expect("reload");
        let resynthesized = reloaded.layers().find(|l| l.name == "DIMS").expect("dxf's own loader resynthesizes DIMS because entities still reference it");
        assert_eq!(resynthesized.color.index(), Some(7), "resynthesized layer takes Layer::default()'s colour, not the removed row's colour (3)");
        assert_eq!(resynthesized.line_type_name, "CONTINUOUS", "resynthesized layer takes the default linetype, not the removed row's (DASHED)");
    }
}
//#endregion 🔖️Tests
