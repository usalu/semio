//! 🧩️ Puzzle 5d play app commands — part lifecycle: palette drops, catalogue placements, deletion,
//! duplication and the hidden/locked flags.

use crate::apps::puzzle5d::config::Puzzle5dSelection;
use crate::apps::puzzle5d::{add_palette_part, next_part_id, remove_grips, remove_parts, Puzzle5dActionCtx, Puzzle5dPart};
use semio_framework_plugin::SelectionSet;
use serde_json::{json, Value};

/// 🎨️ Palette drop at a flat point — the volume origin is derived from the nearest peer part.
pub fn add_node(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let x = args.and_then(|value| value.get("x")).and_then(|value| value.as_f64()).unwrap_or(120.0);
    let y = args.and_then(|value| value.get("y")).and_then(|value| value.as_f64()).unwrap_or(120.0);
    add_palette_part(ctx.scene, &part_kind, x, y);
}

/// 🛍️ Catalogue placement — routed through the paired board/engine brush placement so both aspects land at once.
pub fn add_part_kind(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = args.and_then(|value| value.get("partKind")).and_then(|value| value.as_str()).unwrap_or("Part").to_string();
    let payload = json!({ "nodeKind": part_kind, "x": 120.0, "y": 120.0 });
    ctx.app.apply_board_brush_place(ctx.scene, &payload);
}

/// 🗑️ Removes every selected part (and its fasteners), grip and fastener, then clears the selection.
pub fn delete_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let selection = ctx.scene.runtime.selection.clone();
    remove_parts(&mut ctx.scene.document, selection.part_ids.as_slice());
    remove_grips(&mut ctx.scene.document, selection.grip_ids.as_slice());
    ctx.scene.document.fasteners.retain(|fastener| !selection.fastener_ids.contains(&fastener.id));
    ctx.scene.runtime.selection = Puzzle5dSelection::default();
}

/// 📄️ Clones every selected part at a small flat+volume offset. Aborts (emitting nothing at all) when
/// the selection holds no parts — the pre-migration `return Emit::default()`.
pub fn duplicate_selection(ctx: &mut Puzzle5dActionCtx<'_>) {
    let ids = ctx.scene.runtime.selection.part_ids.clone();
    let clones: Vec<Puzzle5dPart> = ctx
        .scene
        .document
        .parts
        .iter()
        .filter(|part| ids.contains(&part.id))
        .map(|part| {
            let mut clone = part.clone();
            clone.id = next_part_id();
            clone.part_3d.origin[0] += 0.5;
            clone.part_3d.origin[1] += 0.5;
            clone.part_2d.x += 48.0;
            clone.part_2d.y += 24.0;
            clone
        })
        .collect();
    if clones.is_empty() {
        ctx.abort = true;
        return;
    }
    let new_ids: Vec<String> = clones.iter().map(|part| part.id.clone()).collect();
    ctx.scene.document.parts.extend(clones);
    ctx.scene.runtime.selection = Puzzle5dSelection { part_ids: SelectionSet::from_ids(new_ids), grip_ids: SelectionSet::default(), fastener_ids: SelectionSet::default() };
}

/// 👁️ Sets `hidden`/`locked` on every selected part.
pub fn set_selection_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    let part_ids = ctx.scene.runtime.selection.part_ids.clone();
    for part in &mut ctx.scene.document.parts {
        if !part_ids.contains(&part.id) {
            continue;
        }
        match flag {
            "hidden" => part.part_2d.hidden = Some(value),
            "locked" => part.part_2d.locked = Some(value),
            _ => {}
        }
    }
}
