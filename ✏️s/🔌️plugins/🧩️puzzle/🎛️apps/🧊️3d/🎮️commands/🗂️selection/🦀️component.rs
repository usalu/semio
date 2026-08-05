//! 🗂️ Puzzle 3d play app commands — selection: the panel/renderer-driven `setSelection`, the world
//! marquee and pick paths (including the locked/hidden-pick-clears-like-background rule), the
//! vortex pick with its merge-mode vocabulary, select-all / clear / same-kind, the right-click
//! select-then-open-menu shortcut, and the selection method / merge mode / selectable-kind settings.

use crate::apps::puzzle3d::{
    drive_precompute, puzzle3d_clear_non_object_selection, puzzle3d_clear_non_vortex_selection, puzzle3d_clear_selection, Puzzle3dActionCtx,
};
use crate::apps::puzzle3d::config::Puzzle3dSelection;
use semio_framework_plugin::{merge_world_selection_ids, SelectionSet};
use serde_json::Value;

pub fn set_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(selection) = args.and_then(|value| value.get("selection")) {
        if let Ok(parsed) = serde_json::from_value(selection.clone()) {
            ctx.scene.runtime.selection = parsed;
        }
    }
}

pub fn world_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    let ids: Vec<String> = args.and_then(|value| value.get("ids")).and_then(|value| serde_json::from_value(value.clone()).ok()).unwrap_or_default();
    ctx.scene.runtime.selection.object_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.object_ids, &ids, merge);
}

pub fn world_pick(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or("replace");
    if args.and_then(|value| value.get("id")).is_none_or(Value::is_null) {
        if merge == "replace" {
            puzzle3d_clear_selection(&mut ctx.scene.runtime.selection);
        }
    } else if ctx.scene.runtime.selectable_kinds.objects {
        let index = args.and_then(|value| value.get("id")).and_then(|value| value.as_u64()).unwrap_or(0) as usize;
        // 🔓️ Locked/hidden picks are equivalent to background: clear on replace instead of
        // no-opping while the mesh still absorbs the click ahead of `onPointerMissed`.
        match ctx.scene.fixture.objects.get(index).filter(|object| !object.locked && !object.hidden) {
            Some(object) => {
                let id = object.id.clone();
                if merge == "replace" {
                    puzzle3d_clear_non_object_selection(&mut ctx.scene.runtime.selection);
                }
                ctx.scene.runtime.selection.object_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.object_ids, &[id], merge);
            }
            None if merge == "replace" => {
                puzzle3d_clear_selection(&mut ctx.scene.runtime.selection);
            }
            None => {}
        }
    }
}

pub fn world_vortex_select(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if !ctx.scene.runtime.selectable_kinds.vortices {
        return;
    }
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) else {
        return;
    };
    let merge = args.and_then(|value| value.get("merge")).and_then(|value| value.as_str()).unwrap_or(&ctx.scene.runtime.selection_mode_default);
    let merge_mode = match merge {
        "additive" => "add",
        "subtractive" => "remove",
        "invertive" => "toggle",
        "default" => "replace",
        other => other,
    };
    if merge_mode == "replace" {
        puzzle3d_clear_non_vortex_selection(&mut ctx.scene.runtime.selection);
    }
    ctx.scene.runtime.selection.vortex_ids = merge_world_selection_ids(&ctx.scene.runtime.selection.vortex_ids, &[full_id.to_string()], merge_mode);
    drive_precompute(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
}

pub fn select_all(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.selection.object_ids = if ctx.scene.runtime.selectable_kinds.objects {
        ctx.scene.fixture.objects.iter().filter(|object| !object.hidden && !object.locked).map(|object| object.id.clone()).collect::<SelectionSet>()
    } else {
        SelectionSet::default()
    };
    ctx.scene.runtime.selection.vortex_ids.clear();
    ctx.scene.runtime.selection.attraction_ids.clear();
    ctx.scene.runtime.selection.target_volume_ids.clear();
    ctx.scene.runtime.selection.reference_ids.clear();
}

pub fn clear_selection(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.selection = Puzzle3dSelection::default();
}

/// 🎯️ Replaces the object selection with every object sharing the first selected object's kind.
/// Aborts the whole action (no config snapshot, no window save) when there is nothing to widen from,
/// exactly as the pre-migration early `return` did.
pub fn select_same_kind(ctx: &mut Puzzle3dActionCtx<'_>) {
    let Some(first_id) = ctx.scene.runtime.selection.object_ids.first().map(str::to_string) else {
        ctx.abort = true;
        return;
    };
    let Some(kind) = ctx.scene.fixture.objects.iter().find(|object| object.id == first_id).and_then(|object| object.object_kind.clone()).filter(|kind| !kind.is_empty()) else {
        ctx.abort = true;
        return;
    };
    ctx.scene.runtime.selection.object_ids = ctx.scene.fixture.objects.iter().filter(|object| object.object_kind.as_deref() == Some(kind.as_str())).map(|object| object.id.clone()).collect::<SelectionSet>();
}

/// 🖱️ Right-click on an unselected entity selects it and opens its menu in one round trip, instead of
/// requiring a separate pick action before the menu items become available.
pub fn context_menu_at(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
    ctx.scene.runtime.selection = Puzzle3dSelection::default();
    match kind {
        "object" => ctx.scene.runtime.selection.object_ids = SelectionSet::from(vec![id.to_string()]),
        "vortex" => ctx.scene.runtime.selection.vortex_ids = SelectionSet::from(vec![id.to_string()]),
        "attraction" => ctx.scene.runtime.selection.attraction_ids = SelectionSet::from(vec![id.to_string()]),
        "targetVolume" => ctx.scene.runtime.selection.target_volume_ids = SelectionSet::from(vec![id.to_string()]),
        "reference" => ctx.scene.runtime.selection.reference_ids = SelectionSet::from(vec![id.to_string()]),
        _ => {}
    }
}

pub fn set_selection_method(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let method = args.and_then(|value| value.get("method")).and_then(|value| value.as_str()).unwrap_or("rectangle");
    ctx.scene.runtime.selection_method = method.into();
}

pub fn set_selection_mode_default(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("mode").or_else(|| value.get("value"))).and_then(|value| value.as_str()) {
        ctx.scene.runtime.selection_mode_default = mode.into();
    }
}

pub fn set_selectable_kind(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool());
    let kinds = &mut ctx.scene.runtime.selectable_kinds;
    match kind {
        "objects" => kinds.objects = pressed.unwrap_or(!kinds.objects),
        "vortices" => kinds.vortices = pressed.unwrap_or(!kinds.vortices),
        "attractions" => kinds.attractions = pressed.unwrap_or(!kinds.attractions),
        _ => {}
    }
}
