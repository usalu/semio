//! 🖌️ `add-brush-part` command, and the brush placement `add-part-kind` and board events share with it.

use crate::editor::puzzle5d::precompute::brush::puzzle5d_brush_placement;
use crate::editor::puzzle5d::{
    find_part_by_grip_full_id, grips_from_templates, puzzle5d_grip_full_id, puzzle5d_next_part_label, resolve_part_kind_mesh_url, set_part_2d_position, world_grip_direction, world_grip_position, Puzzle5dActionCtx, Puzzle5dFastener,
    Puzzle5dFreshIds, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, PUZZLE5D_DEFAULT_PART_RADIUS,
};
use dsl::os_pack::json::Value;

fn arg_str<'a>(args: Option<&'a Value>, key: &str) -> Option<&'a str> {
    args.and_then(|value| value.get(key)).and_then(Value::as_str).filter(|text| !text.is_empty())
}

/// 🎯️ The grip a brush placement or a candidate cycle works on: `explicit`, else the grip an open suggestion
/// menu lists candidates for, else the first selected grip, else the grip the brush suggestions run is pointed at.
pub fn puzzle5d_brush_source_grip(ctx: &Puzzle5dActionCtx<'_>, explicit: Option<&str>) -> Option<String> {
    explicit
        .map(str::to_string)
        .or_else(|| ctx.scene.runtime.suggestion_menu.as_ref().map(|menu| menu.vortex_full_id.clone()).filter(|id| !id.is_empty()))
        .or_else(|| ctx.selected_grip_ids().first().cloned())
        .or_else(|| ctx.brush_suggestions(|link| link.target().map(str::to_string)).flatten())
}

/// 🧱️ `addBrushPart`: places `partKind` on the brush target grip.
pub fn add_brush_part(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let part_kind = arg_str(args, "partKind").or_else(|| arg_str(args, "objectKindId")).unwrap_or("Part").to_string();
    let source = arg_str(args, "targetVortexFullId").or_else(|| arg_str(args, "targetGripFullId"));
    let at = [args.and_then(|value| value.get("x")).and_then(Value::as_f64), args.and_then(|value| value.get("y")).and_then(Value::as_f64)];
    puzzle5d_place_brush_part(ctx, &part_kind, source, at, arg_str(args, "nodeId"), arg_str(args, "edgeId"));
}

/// 🖌️ One brush placement of `part_kind` onto the source grip. A free candidate the brush suggestions run resolved
/// for that grip lands posed exactly as the search posed it (the first candidate of `part_kind`, else the cycled
/// candidate index); without a resolved candidate the part lands one grip direction beyond the source grip. Both
/// aspects land in one part, fastened to the source grip, at the board position `at` when given.
pub fn puzzle5d_place_brush_part(ctx: &mut Puzzle5dActionCtx<'_>, part_kind: &str, source: Option<&str>, at: [Option<f64>; 2], part_id: Option<&str>, fastener_id: Option<&str>) {
    let source = puzzle5d_brush_source_grip(ctx, source);
    let mut fresh_ids = Puzzle5dFreshIds::from_document(&ctx.scene.document);
    let part_id = part_id.map_or_else(|| fresh_ids.next_part(), str::to_string);
    let fastener_id = fastener_id.map_or_else(|| fresh_ids.next_fastener(), str::to_string);
    let found = source.as_ref().and_then(|source| ctx.brush_suggestions(|link| link.found(source).cloned()).flatten());
    if let Some(found) = found {
        let index = ctx.scene.runtime.brush_candidate_index;
        let placed = puzzle5d_brush_placement(ctx.snapshot, &ctx.scene.document, &found, Some(part_kind), 0, part_id.clone(), fastener_id.clone()).and_then(|placed| match placed {
            Some(placed) => Ok(Some(placed)),
            None => puzzle5d_brush_placement(ctx.snapshot, &ctx.scene.document, &found, None, index, part_id.clone(), fastener_id.clone()),
        });
        match placed {
            Ok(Some((part, fastener))) => {
                let placed_id = part.id.clone();
                ctx.scene.document.parts.push(part);
                ctx.scene.document.fasteners.push(fastener);
                set_part_2d_position(&mut ctx.scene.document, &placed_id, at[0], at[1]);
                return;
            }
            Ok(None) => {}
            Err(_) => {
                ctx.abort = true;
                return;
            }
        }
    }
    let document = &ctx.scene.document;
    let source_world = source.as_deref().and_then(|full_id| find_part_by_grip_full_id(document, full_id)).map(|(part, grip)| (world_grip_position(part, grip), world_grip_direction(part, grip)));
    let origin = source_world.map_or([0.0; 3], |(position, direction)| [position[0] + direction[0], position[1] + direction[1], position[2] + direction[2]]);
    let part = Puzzle5dPart {
        id: part_id,
        anchor: Default::default(),
        part_kind: part_kind.to_string(),
        part_2d: Puzzle5dPart2d { x: at[0].unwrap_or(120.0), y: at[1].unwrap_or(120.0), shape: "circle".into(), radius: PUZZLE5D_DEFAULT_PART_RADIUS, width: None, height: None, text: part_kind.to_string(), icon_kind: None, hidden: None, locked: None },
        // 🏷️ A brushed part gets its authored, peer-numbered label stamped at creation, so the
        // outliner names it the same way the catalogue row the operator painted from does.
        part_3d: Puzzle5dPart3d {
            origin,
            mesh_url: resolve_part_kind_mesh_url(part_kind, document.kind_catalogs.as_ref()),
            orientation: Some([0.0, 0.0, 0.0, 1.0]),
            scale: None,
            label: Some(puzzle5d_next_part_label(&document.parts, document, part_kind)),
        },
        grips: grips_from_templates(document, part_kind),
    };
    let target = part.grips.first().map(|grip| puzzle5d_grip_full_id(&part.id, &grip.id));
    ctx.scene.document.parts.push(part);
    if let (Some(source), Some(target)) = (source, target) {
        ctx.scene.document.fasteners.push(Puzzle5dFastener { id: fastener_id, source, target, fastener_kind: None, gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 });
    }
}
