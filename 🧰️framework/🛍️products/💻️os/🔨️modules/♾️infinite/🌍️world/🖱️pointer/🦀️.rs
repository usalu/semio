//! 🖱️ World pointer, wheel and gesture entry points with action, pick and relocate helpers.

use super::*;

//#region 🧭️WorldOrbitViewGizmo
/** 🧭️ The pure placement/tip-geometry/paint logic relocated to `ui_wgpu::wgpu::widgets::gizmo` (see
`.🧬semio/🦑️repo/🎫️tickets/26/08/05/FRAMEWORK-BUILDER-PASSTHROUGHS-APP-COMMANDS-MACRO-WIDGET-EXTRACTION`) — this
region now only keeps the `World3dState`-specific hover-state plumbing (app config, not paint), calling
through to `gizmo::orbit_view_gizmo_placement`/`gizmo::orbit_view_gizmo_tips`/`gizmo::orbit_view_gizmo_hit_test`. */
#[cfg(test)]
fn update_world_orbit_view_gizmo_hover(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let (margin_x, margin_y) = gizmo::orbit_view_gizmo_placement(inner);
    let origin_x = inner.x + inner.w - margin_x;
    let origin_y = inner.y + inner.h - margin_y;
    let zone_radius = 56.0;
    if ((x - origin_x).powi(2) + (y - origin_y).powi(2)).sqrt() > zone_radius {
        state.gizmo_hovered_tip = None;
        return;
    }
    let camera = state.orbit.to_camera();
    let tips = gizmo::orbit_view_gizmo_tips(&camera, inner);
    state.gizmo_hovered_tip = gizmo::orbit_view_gizmo_hit_test(x, y, &tips);
}
//#endregion 🧭️WorldOrbitViewGizmo

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<ActionDescriptor> {
    HitTarget { rect: bounds, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None }
}

#[cfg(test)]
pub(super) fn handle_world3d_pointer_move(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        state.gizmo_hovered_tip = None;
        return None;
    }
    if down && button == 0 {
        if state.marquee_active {
            state.marquee_points.push([x, y]);
            update_marquee_preview(state, render_pick_viewport(state));
        } else if state.paint_stroke_active && state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.controller_id.clone(),
                    action: "paintAt".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": state.surface_id,
                        "objectId": object_id,
                        "u": u,
                        "v": v,
                        "phase": "stream",
                    })),
                });
            }
        } else if button == 2 || button == 1 {
            return None;
        }
    }
    if !down {
        update_world_orbit_view_gizmo_hover(state, x, y, inner);
        return pick_hover_action(state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

#[cfg(test)]
pub(super) fn handle_world3d_paint_actions(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16) -> Vec<ActionDescriptor> {
    if state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(state, x, y, inner) else {
            return Vec::new();
        };
        return vec![ActionDescriptor {
            controller_id: state.controller_id.clone(),
            action: "paintAt".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": state.surface_id,
                "objectId": object_id,
                "u": u,
                "v": v,
                "phase": "stream",
            })),
        }];
    }
    Vec::new()
}

/// 🖱️📋️ A right RELEASE owes the unconditional camera report and NOTHING else: the menu is
/// the shell's, opened on the press (see the authority's own right-release arm).
#[cfg(test)]
pub(super) fn handle_world3d_pointer_button(state: &mut World3dState, x: f32, y: f32, down: bool, button: i16, modifiers: &PointerModifiers) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(state);
    if !inner.contains(x, y) {
        return None;
    }
    let shift = modifiers.shift;
    let ctrl = modifiers.ctrl;
    if down {
        if button == 0 {
            if state.active_utility == "surfaceBrush" {
                return None;
            }
            if state.interaction_mode == "paint" {
                state.paint_stroke_active = true;
                let (object_id, u, v) = pick_paint_hit(state, x, y, inner)?;
                return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.surface_id, "objectId": object_id, "u": u, "v": v, "phase": "stream" })) });
            }
            if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
                if let Some(full_id) = pick_vortex_at(state, x, y, inner) {
                    let merge = world_merge_mode(shift, ctrl).wire_label();
                    return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexSelect".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.surface_id, "fullId": full_id, "merge": merge })) });
                }
            } else if state.active_utility == "select" {
                if !state.selected_ids.is_empty() {
                    if let Some(handle) = pick_gumball_handle_at(state, x, y, inner) {
                        start_gumball_drag(state, handle, x, y, inner);
                        return None;
                    }
                }
                if state.gumball_handle.is_none() && !component_mode_active(state) {
                    state.press_object_id = pick_instance_at(state, x, y, inner);
                } else {
                    state.press_object_id = None;
                }
            }
            state.marquee_active = true;
            state.marquee_points = vec![[x, y]];
            return None;
        }
        if button == 2 {
            state.marquee_active = false;
            state.marquee_points.clear();
            state.right_press_point = Some([x, y]);
        }
        return None;
    }
    if button == 0 && state.marquee_active {
        state.marquee_active = false;
        if is_click_gesture(state, x, y) {
            state.marquee_points.clear();
            state.marquee_preview_ids.clear();
            state.press_object_id = None;
            state.drag_object_id = None;
            state.drag_last_position = None;
            if state.active_utility == "surfaceBrush" {
                if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "worldSurfacePlace".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.surface_id,
                            "pane": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                            "normal": normal,
                        })),
                    });
                }
                return None;
            }
            if state.gumball_handle.is_none() {
                return pick_select_action(state, x, y, inner, shift, ctrl);
            }
        } else {
            return marquee_select_action(state, render_pick_viewport(state), shift, ctrl);
        }
    }
    if button == 0 && state.interaction_mode == "paint" && state.paint_stroke_active {
        state.paint_stroke_active = false;
        let mut actions = Vec::new();
        actions.push(ActionDescriptor { controller_id: state.controller_id.clone(), action: "paintAt".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.surface_id, "phase": "commit" })) });
        return actions.first().cloned();
    }
    if button == 0 {
        if state.active_utility == "surfaceBrush" {
            if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: state.controller_id.clone(),
                    action: "worldSurfacePlace".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": state.surface_id,
                        "pane": state.surface_id,
                        "objectId": object_id,
                        "position": position,
                        "normal": normal,
                    })),
                });
            }
            return None;
        }
        if let Some(action) = gumball_commit_action(state) {
            state.gumball_handle = None;
            reset_gumball_preview(state);
            return Some(action);
        }
        state.gumball_handle = None;
        reset_gumball_preview(state);
        state.press_object_id = None;
        if let Some(object_id) = state.drag_object_id.take() {
            if let Some(position) = state.drag_last_position {
                if !is_click_gesture(state, x, y) {
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "worldRelocate".into(),
                        args: Some(semio_framework::dsl_value!({
                            "surfaceId": state.surface_id,
                            "objectId": object_id,
                            "position": position,
                        })),
                    });
                }
            }
        }
        if state.active_utility == "brush" {
            if let Some(preview) = state.brush_preview.clone() {
                if let (Some(target), Some(kind), Some(index)) = (preview.target_vortex_full_id, preview.object_kind_id, preview.source_vortex_index) {
                    let origin = preview.origin.unwrap_or([0.0, 0.0, 0.0]);
                    return Some(ActionDescriptor {
                        controller_id: state.controller_id.clone(),
                        action: "addBrushObject".into(),
                        args: Some(semio_framework::dsl_value!({
                            "targetVortexFullId": target,
                            "objectKindId": kind,
                            "sourceVortexIndex": index,
                            "origin": origin,
                            "orientation": preview.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]),
                            "scale": preview.scale.as_ref().map(semio_framework::DslValue::from),
                        })),
                    });
                }
            }
        }
        return pick_select_action(state, x, y, inner, shift, ctrl);
    }
    if button == 2 {
        state.right_press_point = None;
        return Some(orbit_camera_action(state));
    }
    if button == 1 {
        return Some(orbit_camera_action(state));
    }
    None
}

/// 🖱️📋️ Resolves which entity a right-click context menu targets, in the React reference's exact
/// priority order — `resolveWorldContextMenuTarget` in `🧱️elements/🌐️World3dHost/🟦️.tsx:1893`: a
/// hovered vortex wins first, then a hovered mesh component (vertex/edge/face — reported as kind
/// `"object"`, the component's owning instance, matching the React source's own naming), then a
/// hovered reference image plane, then any other plain hover as `"object"`; `None` (no target) if
/// nothing is currently hovered.
pub fn resolve_world_context_menu_target(state: &World3dState) -> Option<(&'static str, String)> {
    if let Some(vortex_id) = state.hovered_vortex_id.clone() {
        return Some(("vortex", vortex_id));
    }
    if state.hovered_component_mode.is_some() {
        if let Some(object_id) = state.hovered_component_object_id.clone() {
            return Some(("object", object_id));
        }
    }
    let hovered = state.local_hover_id.as_deref()?;
    if let Some(reference_id) = hovered.strip_prefix("reference:") {
        return Some(("reference", reference_id.to_string()));
    }
    Some(("object", hovered.to_string()))
}

/// 🪧️ The `surface` half of one world right-click's context-menu request: the entity under the
/// pointer as the single `hits` row, and the painted selection as per-domain `selection` groups —
/// the Rust twin of `world3dContextMenuSurfaceV1`
/// (`🧱️elements/🌐️World3dHost/🟦️.tsx:1909`). The hit target rides THIS request; no separate
/// target-recording dispatch precedes the menu.
pub fn world3d_context_menu_surface(state: &World3dState) -> (Vec<ui_wgpu::wgpu::ContextMenuHit>, Vec<ui_wgpu::wgpu::ContextMenuSelectionGroup>) {
    let hits = resolve_world_context_menu_target(state).map(|(domain, id)| ui_wgpu::wgpu::ContextMenuHit { domain: domain.to_string(), id, label: None }).into_iter().collect();
    let mut selection = Vec::new();
    if !state.selected_ids.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "object".into(), ids: state.selected_ids.clone() });
    }
    if !state.component_ids.is_empty() {
        selection.push(ui_wgpu::wgpu::ContextMenuSelectionGroup { domain: "feature".into(), ids: state.component_ids.clone() });
    }
    (hits, selection)
}

#[cfg(test)]
pub(super) fn handle_world3d_pointer_drag(state: &mut World3dState, x: f32, y: f32, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) {
    let inner = world_pick_rect(state);
    if button == 0 {
        if state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(state, x, y, inner);
            return;
        }
        if state.drag_object_id.is_none() && state.gumball_handle.is_none() && !component_mode_active(state) && pointer_drag_distance(state, x, y) > CLICK_DRAG_THRESHOLD_PX {
            if let Some(object_id) = state.press_object_id.take().or_else(|| pick_instance_at(state, x, y, inner)) {
                state.drag_object_id = Some(object_id.clone());
                state.marquee_active = false;
                state.marquee_points.clear();
                state.marquee_preview_ids.clear();
                state.drag_last_position = object_world_position(state, &object_id);
                state.drag_object_z = state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
            }
        }
        if state.drag_object_id.is_some() && inner.contains(x, y) {
            if let Some(position) = world3d_ground_plane_pick(state, x, y, state.drag_object_z) {
                state.drag_last_position = Some(position);
                if let Some(object_id) = state.drag_object_id.clone() {
                    update_dragged_instance_position(state, &object_id, position);
                }
            }
            return;
        }
    }
    if button == 1 || (button == 2 && modifiers.shift) {
        state.orbit.pan(-dx, -dy);
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        state.orbit.orbit(dx, dy);
    }
}

#[cfg(test)]
pub(super) fn handle_world3d_wheel(state: &mut World3dState, delta: f32) {
    state.orbit.zoom(delta);
}

/// 🎯️ Optimistic local set algebra over the ONE framework merge vocabulary — the five
/// `MergeMode` words of `🕹️interaction/🧬️schema/🔣️.json` (fixture `🎯️merge-modes.json`). It takes a
/// decoded mode, never a word: ticket 26/09/09/PROCEDURAL-3D-END-TO-END deleted this file's private
/// `add`/`remove`/`toggle` spelling, which made `worldPick` and the domain-bound `interactionSelect`
/// disagree about what a shift-click means. `Range` needs an ordered topology this local preview does
/// not carry, so it replaces — the documented degradation, identical to `next_selection`'s.
fn merge_string_ids(existing: &[String], incoming: &[String], merge: MergeMode) -> Vec<String> {
    match merge {
        MergeMode::Additive => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if !merged.contains(id) {
                    merged.push(id.clone());
                }
            }
            merged
        }
        MergeMode::Invertive => {
            let mut merged = existing.to_vec();
            for id in incoming {
                if let Some(index) = merged.iter().position(|entry| entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id.clone());
                }
            }
            merged
        }
        MergeMode::Subtractive => existing.iter().filter(|id| !incoming.contains(id)).cloned().collect(),
        MergeMode::Replace | MergeMode::Range => incoming.to_vec(),
    }
}

//#region 🔖️WorldInteractionDomain
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the OS's own bare-board `world`
/// domain — plain (non-component) 3D-world object picking/hover for a world3d window that binds NO
/// app `InteractionDefinition`. Granularity `"item"` targets are `"{surfaceId}/{objectId}"`, split by
/// [`HierarchyProvider::PathDelimited`] — the cross-surface addressing a shared, multi-board OS domain
/// needs (`"surface"` covers a future whole-surface selection, not emitted by pointer picking today).
///
/// Most world3d apps bind their OWN domain instead (`WindowKindDefinition.interactions` /
/// `.window_kind_interactions(...)`) — e.g. CAD's `"cad"` domain with `HierarchyProvider::Flat`. That
/// binding reaches this file only through `World3dScene.domain_id`/`domain_granularity_id` (set by the
/// app's own `render()`, the only party that knows both its `window_kind_id` and which domain it bound
/// there — mirrors `UiTreeNode.interaction_domain`/`PanelTreeBuilder::interaction_domain`, extended to
/// `Scene` surfaces), captured onto `World3dState.bound_domain_id`/`bound_domain_granularity_id` by
/// `sync_world3d_state`. `resolved_domain_id`/`resolved_domain_granularity_id`/`resolved_item_id` below
/// are what every plain pick/hover emit site actually reads — never `WORLD_INTERACTION_DOMAIN_ID`
/// directly, so a bound app domain and this OS domain never both light up for the same click (the
/// two-selection-universe bug this indirection exists to prevent). A bound domain is inherently
/// single-surface-scoped (one app document per window), so its ids are always bare — no `surfaceId/`
/// prefix, unlike `world`'s own multi-surface `PathDelimited` shape.
///
/// Component-level targets retain their original render instance, analytic source and exact label
/// in the same interaction domain as ordinary object targets.
pub const WORLD_INTERACTION_DOMAIN_ID: &str = "world";
pub(super) const WORLD_ITEM_GRANULARITY_ID: &str = "item";
pub(super) const WORLD_ITEM_PATH_DELIMITER: &str = "/";

pub fn world_interaction_definition() -> InteractionDefinition {
    InteractionDefinition {
        id: WORLD_INTERACTION_DOMAIN_ID.into(),
        label: LocalizedLabel::native("World", "Welt"),
        granularities: vec![
            GranularityDefinition { id: "surface".into(), label: LocalizedLabel::native("Surface", "Fläche"), icon_id: "layers".into() },
            GranularityDefinition { id: WORLD_ITEM_GRANULARITY_ID.into(), label: LocalizedLabel::native("Item", "Objekt"), icon_id: "box".into() },
            GranularityDefinition { id: "vertex".into(), label: LocalizedLabel::native("Vertex", "Ecke"), icon_id: "circle-dot".into() },
            GranularityDefinition { id: "edge".into(), label: LocalizedLabel::native("Edge", "Kante"), icon_id: "minus".into() },
            GranularityDefinition { id: "face".into(), label: LocalizedLabel::native("Face", "Fläche"), icon_id: "square".into() },
        ],
        hierarchy: HierarchyProvider::PathDelimited { delimiter: WORLD_ITEM_PATH_DELIMITER.into() },
        hover: HoverSpec::default(),
        selection: SelectionSpec {
            modes: vec![SelectionMode::Multiple, SelectionMode::Single],
            methods: vec![SelectionMethod::Pick, SelectionMethod::Rectangle, SelectionMethod::Lasso],
            merges: vec![MergeMode::Replace, MergeMode::Additive, MergeMode::Subtractive, MergeMode::Invertive],
            transitive: false,
            broadcast: true,
        },
    }
}

/// 🪟️ The interaction domain a plain pick/hover on this surface targets: the app-bound domain from
/// `World3dScene.domain_id`, falling back to the OS's own `world` board domain when unbound.
pub(super) fn resolved_domain_id(state: &World3dState) -> &str {
    state.bound_domain_id.as_deref().unwrap_or(WORLD_INTERACTION_DOMAIN_ID)
}

/// 🎯️ The scene's default granularity for a plain instance target — the app-declared
/// granularity when a domain is bound, else the `world` domain's own `"item"` granularity.
pub(super) fn resolved_domain_granularity_id(state: &World3dState) -> &str {
    state.bound_domain_granularity_id.as_deref().unwrap_or(WORLD_ITEM_GRANULARITY_ID)
}

/// 🎯️ The TOPOLOGY target a rendered instance stands for.
///
/// ⚖️ A surface renders one channel as many instances (`extrude@solid#0`, `extrude@solid#1`), and the
/// guest's selection/hover is over the CHANNEL (`extrude@solid`). Publishing the render id makes the
/// guest silently drop the target: it is not in its topology. Measured on 6118 —
/// `interactionSelect targets=[{"id":"extrude@solid#0"}]` left `selectedIds` empty across hover,
/// click and marquee (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md`). React resolves the same way
/// (`world3dInstanceInteractionTarget`), and the fixture
/// `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` is the shared oracle.
pub(super) fn instance_interaction_id<'a>(state: &'a World3dState, render_id: &'a str) -> &'a str {
    state.instance_interaction_ids.get(render_id).map_or(render_id, String::as_str)
}

/// 🎯️ The granularity of one rendered instance's interaction target. An explicit record
/// override wins; otherwise pick, hover and marquee all use the scene default, matching React's
/// `world3dInstanceInteractionTarget`.
pub(super) fn instance_interaction_granularity_id<'a>(state: &'a World3dState, render_id: &str) -> &'a str {
    state.instance_interaction_granularity_ids.get(render_id).map_or_else(|| resolved_domain_granularity_id(state), String::as_str)
}

/// 🧮️ Upper bound used by marquee action reservation. Every retained override is already
/// capped by [`WORLD_INTERACTION_ID_BYTE_CAPACITY`] at bridge admission.
pub(super) fn maximum_instance_interaction_granularity_bytes(state: &World3dState) -> usize {
    state.instance_interaction_granularity_ids.values().map(String::len).max().unwrap_or(0).max(resolved_domain_granularity_id(state).len())
}

#[cfg(test)]
fn world_item_target_id(surface_id: &str, object_id: &str) -> String {
    format!("{surface_id}{WORLD_ITEM_PATH_DELIMITER}{object_id}")
}

/// 🔤️ Strips this state's own `surfaceId/` prefix from a `world`-domain item target id — the inverse
/// of [`world_item_target_id`]. Targets for a different surface (cross-surface batch, not produced by
/// this file's single-surface pointer handlers) are dropped rather than mis-parsed.
fn world_item_id_for_surface<'a>(state: &World3dState, target_id: &'a str) -> Option<&'a str> {
    target_id.strip_prefix(&state.surface_id).and_then(|rest| rest.strip_prefix(WORLD_ITEM_PATH_DELIMITER))
}

/// 🧮️ Builds the wire target id for a plain pick/hover hit: a bare object id when a real app domain is
/// bound (`HierarchyProvider::Flat`-style, single-surface-scoped), else `world_item_target_id`'s
/// `"surfaceId/id"` `PathDelimited` shape for the shared `world` domain.
#[cfg(test)]
pub(super) fn resolved_item_id(state: &World3dState, object_id: &str) -> String {
    if state.bound_domain_id.is_some() {
        object_id.to_string()
    } else {
        world_item_target_id(&state.surface_id, object_id)
    }
}

/// 🔤️ Inverse of [`resolved_item_id`], for parsing ids back out of an incoming action's targets during
/// optimistic local preview.
fn parse_resolved_item_id<'a>(state: &World3dState, target_id: &'a str) -> Option<&'a str> {
    if state.bound_domain_id.is_some() {
        Some(target_id)
    } else {
        world_item_id_for_surface(state, target_id)
    }
}

/// 🕹️ The ONE modifier → merge rule this surface has, in the ONE vocabulary of
/// `🕹️interaction/🧬️schema/🔣️.json`, put on the wire verbatim.
///
/// ⚖️ `shift+ctrl` = invertive, `shift` = additive, `ctrl` (and its platform twin `meta`) =
/// subtractive, nothing = replace — identical to React's `resolveWorldMergeMode`
/// (`🌐️World3dHost/🟦️.tsx`) and pinned by `🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json`. Six
/// call sites in this file each open-coded `shift → additive, ctrl → INVERTIVE, else replace`, which
/// has no `subtractive` at all and spells ctrl's mode wrong; they now all read this.
pub(super) fn world_merge_mode(shift: bool, ctrl_or_meta: bool) -> MergeMode {
    match (shift, ctrl_or_meta) {
        (true, true) => MergeMode::Invertive,
        (true, false) => MergeMode::Additive,
        (false, true) => MergeMode::Subtractive,
        (false, false) => MergeMode::Replace,
    }
}

/// 🕹️ [`world_merge_mode`] as the fixed `u8` the bounded pick cursors carry through a retained
/// gesture — the cursors hold no `String`, so the mode travels as its ordinal and is spelled once,
/// at publication, by [`world_merge_wire_label`].
pub(super) fn world_merge_code(shift: bool, ctrl: bool, meta: bool) -> u8 {
    match world_merge_mode(shift, ctrl || meta) {
        MergeMode::Additive => 1,
        MergeMode::Subtractive => 2,
        MergeMode::Invertive => 3,
        _ => 0,
    }
}

pub(super) fn world_merge_wire_label(code: u8) -> &'static str {
    match code {
        1 => MergeMode::Additive.wire_label(),
        2 => MergeMode::Subtractive.wire_label(),
        3 => MergeMode::Invertive.wire_label(),
        _ => MergeMode::Replace.wire_label(),
    }
}

pub(super) fn selection_method_wire_str(method: SelectionMethod) -> &'static str {
    match method {
        SelectionMethod::Pick => "pick",
        SelectionMethod::Rectangle => "rectangle",
        SelectionMethod::Lasso => "lasso",
    }
}
//#endregion 🔖️WorldInteractionDomain

/// Applies hover/selection action payloads to renderer-local world state before the plugin round-trip.
///
/// 🕹️ An undecodable merge leaves this OPTIMISTIC LOCAL PREVIEW untouched — the word is
/// outside `🕹️interaction`'s schema enum, so the authoritative path faults on it and the
/// preview must not invent a meaning for it (see `MergeMode::from_wire_label`).
///
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: replaces the deleted ad-hoc
/// `worldSelect`/`worldHover` command strings with the framework interaction verbs
/// (`domainId`/`targets`/`merge`/`method`, `domainId`/`channel`/`targets`) — see
/// `resolved_domain_id` (the domain this surface actually targets: its bound app domain, or the
/// OS `world` fallback). This is still the OPTIMISTIC LOCAL PREVIEW only; the framework's
/// `next_selection`/`next_hover` machine (not this file) is the source of truth once the
/// round-trip settles.
///
/// 🎯️ Validates a component target against the original retained instance and mesh label.
pub(super) fn world_component_preview_target<'a>(state: &World3dState, id: &'a str, granularity: &str) -> Option<(&'a str, u32)> {
    let address = WorldComponentAddress::parse(id, granularity)?;
    let (draw, instance) = state.draws.iter().find_map(|draw| draw.instances.iter().find(|instance| instance.id == address.object).map(|instance| (draw, instance)))?;
    address.matches_source(state, instance.component_source, &draw.mesh_key).then_some((address.object, address.group))
}

/// 🖍️ Resolves source targets per instance while retaining the ordinary active-object projection.
pub(super) fn world_instance_selected_components(state:&World3dState,instance:&Instance3d,kind:&str)->HashSet<String> {
    let mut selected: HashSet<String> = state.gumball_selection_ids.iter().filter_map(|id|world_component_preview_target(state,id,kind)).filter(|(object,_)|*object==instance.id).map(|(_,group)|group.to_string()).collect();
    if instance.component_source.is_none() && state.active_object_id.as_deref()==Some(&instance.id) { selected.extend(state.component_ids.iter().cloned()); }
    selected
}

/// 🕹️ Projects the original encoded interaction targets into the existing local preview fields.
pub fn apply_world_action_preview(state: &mut World3dState, action: &ActionDescriptor) {
    let Some(args) = action.args.as_ref() else { return; };
    if args.get("domainId").and_then(|value| value.as_str()) != Some(resolved_domain_id(state)) || !matches!(action.action.as_str(), "interactionSelect" | "interactionHover") { return; }
    let Some(text) = args.get("targets").and_then(|value| value.as_str()).filter(|text| text.len() <= ui_wgpu::wgpu::ACTION_STRING_BYTE_CAPACITY) else { return; };
    let Ok(targets) = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return; };
    let Some(targets) = targets.as_array() else { return; };
    let mut ids = Vec::new();
    let mut components = Vec::new();
    let mut component_object = None;
    let mut component_mode = None;
    for target in targets {
        let (Some(granularity), Some(id)) = (target.get("granularity").and_then(|value| value.as_str()), target.get("id").and_then(|value| value.as_str()).and_then(|id| parse_resolved_item_id(state, id))) else { return; };
        if matches!(granularity, "vertex" | "edge" | "face") {
            let Some((object, group)) = world_component_preview_target(state, id, granularity) else { return; };
            components.push(group.to_string());
            component_object.get_or_insert_with(|| object.to_string());
            component_mode.get_or_insert_with(|| granularity.to_string());
        }
        ids.push(id.to_string());
    }
    if action.action == "interactionSelect" {
        let Some(merge) = args.get("merge").and_then(|value| value.as_str()).and_then(MergeMode::from_wire_label) else { return; };
        if let Some(mode) = component_mode {
            state.gumball_selection_ids = merge_string_ids(&state.gumball_selection_ids, &ids, merge);
            let selected: Vec<(String,u32)> = state.gumball_selection_ids.iter().filter_map(|id| world_component_preview_target(state,id,&mode).map(|(object,group)|(object.to_string(),group))).collect();
            state.selected_ids = selected.iter().map(|(object,_)|object.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            let retained = |object:&str|selected.iter().any(|(selected,_)|selected==object);
            state.active_object_id = component_object.filter(|object|retained(object)).or_else(||state.active_object_id.take().filter(|object|retained(object))).or_else(||selected.first().map(|(object,_)|object.clone()));
            state.component_ids = selected.iter().filter(|(object,_)|Some(object)==state.active_object_id.as_ref()).map(|(_,group)|group.to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            state.granularity = mode;
        } else {
            state.selected_ids = merge_string_ids(&state.selected_ids, &ids, merge);
            if merge == MergeMode::Replace { state.gumball_selection_ids.clear();state.component_ids.clear();state.active_object_id=None; }
        }
    } else if component_mode.is_some() {
        state.hovered_component_id = components.into_iter().next();
        state.hovered_component_object_id = component_object;
        state.hovered_component_mode = component_mode;
        state.local_hover_id = None;
        state.local_hover_granularity_id = None;
    } else {
        state.hovered_component_id = None;
        state.hovered_component_object_id = None;
        state.hovered_component_mode = None;
        state.local_hover_id = ids.into_iter().next();
        state.local_hover_granularity_id = targets.first().and_then(|target| target.get("granularity")).and_then(|value| value.as_str()).map(str::to_string);
    }
}

/// 🖼️ Falls back to reference-image plane hit-testing when no mesh instance is under the
/// cursor — mirrors React's `hoveredId` covering both mesh instances and `"reference:"`-prefixed
/// reference planes (see `resolveWorldContextMenuTarget`'s reference tier).
///
/// 🕹️ `interactionHover` — empty `targets` clears the channel (see `HoverInput`/`next_hover`);
/// `domainId`/target id shape follow this surface's resolved (app-bound or `world`-fallback) domain.
#[cfg(test)]
pub(super) fn pick_hover_action(state: &mut World3dState, x: f32, y: f32, inner: Rect) -> Option<ActionDescriptor> {
    if state.active_utility == "surfaceBrush" {
        if let Some((object_id, position, normal)) = pick_surface_at(state, x, y, inner) {
            return Some(ActionDescriptor {
                controller_id: state.controller_id.clone(),
                action: "worldSurfaceHover".into(),
                args: Some(semio_framework::dsl_value!({
                    "surfaceId": state.surface_id,
                    "pane": state.surface_id,
                    "objectId": object_id,
                    "position": position,
                    "normal": normal,
                })),
            });
        }
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldSurfaceLeave".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.surface_id, "pane": state.surface_id })) });
    }
    if state.active_utility == "brush" || (state.active_utility == "select" && state.granularity == "vertex") {
        let hit = pick_vortex_at(state, x, y, inner);
        if state.hovered_vortex_id == hit {
            return None;
        }
        state.hovered_vortex_id = hit.clone();
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "worldVortexHover".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": state.surface_id, "fullId": hit })) });
    }
    if component_mode_active(state) {
        let hit = pick_component_at(state, x, y, inner);
        if state.hovered_component_id.as_deref() == hit.as_ref().map(|(_, id, _)| id.as_str()) && state.hovered_component_object_id.as_deref() == hit.as_ref().map(|(_, _, object)| object.as_str()) && state.hovered_component_mode.as_deref() == hit.as_ref().map(|(kind, _, _)| kind.as_str()) { return None; }
        state.hovered_component_id = hit.as_ref().map(|(_, id, _)| id.clone());
        state.hovered_component_object_id = hit.as_ref().map(|(_, _, object)| object.clone());
        state.hovered_component_mode = hit.as_ref().map(|(kind, _, _)| kind.clone());
        state.local_hover_id = None; state.local_hover_granularity_id = None;
        let id = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&state.surface_id), id.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(state), "channel": "pointer", "targets": targets.as_str() })) });
    }
    let hit = pick_instance_at(state, x, y, inner).or_else(|| pick_reference_at(state, x, y, inner).map(|url| format!("reference:{url}")));
    let target = hit.as_deref().map(|id| (instance_interaction_id(state, id).to_string(), instance_interaction_granularity_id(state, id).to_string()));
    if state.local_hover_id.as_deref() == target.as_ref().map(|(id, _)| id.as_str()) && state.local_hover_granularity_id.as_deref() == target.as_ref().map(|(_, granularity)| granularity.as_str()) {
        return None;
    }
    state.local_hover_id = target.as_ref().map(|(id, _)| id.clone());
    state.local_hover_granularity_id = target.as_ref().map(|(_, granularity)| granularity.clone());
    let targets = world_interaction_targets(Some(&state.surface_id), target.as_ref().map(|(id, _)| id.as_str()), target.as_ref().map(|(_, kind)| kind.as_str()), state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(state), "channel": "pointer", "targets": targets.as_str() })) })
}

/// 🎯️ Geometry oracle publishes the same shared interaction command and target text.
#[cfg(test)]
pub(super) fn pick_select_action(state: &World3dState, x: f32, y: f32, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    let merge = world_merge_mode(shift, ctrl).wire_label();
    if state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(state) {
        let hit = pick_component_at(state, x, y, inner);
        let target = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&state.surface_id), target.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor {
            controller_id: state.controller_id.clone(), action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(state), "targets": targets.as_str(), "merge": merge, "method": "pick" })),
        });
    }
    let hit = pick_instance_at(state, x, y, inner);
    let targets = world_interaction_targets(Some(&state.surface_id), hit.as_deref().map(|id| instance_interaction_id(state, id)), hit.as_deref().map(|id| instance_interaction_granularity_id(state, id)), state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(state),
            "targets": targets.as_str(),
            "merge": merge,
            "method": selection_method_wire_str(SelectionMethod::Pick),
        })),
    })
}

#[cfg(test)]
pub(super) fn merge_u32_ids(existing: &[String], incoming: &[String], merge: MergeMode) -> Vec<u32> {
    let parse = |ids: &[String]| -> Vec<u32> { ids.iter().filter_map(|id| id.parse().ok()).collect() };
    let existing_ids = parse(existing);
    let incoming_ids = parse(incoming);
    match merge {
        MergeMode::Additive => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if !merged.contains(&id) {
                    merged.push(id);
                }
            }
            merged
        }
        MergeMode::Invertive => {
            let mut merged = existing_ids;
            for id in incoming_ids {
                if let Some(index) = merged.iter().position(|entry| *entry == id) {
                    merged.remove(index);
                } else {
                    merged.push(id);
                }
            }
            merged
        }
        MergeMode::Subtractive => existing_ids.into_iter().filter(|id| !incoming_ids.contains(id)).collect(),
        MergeMode::Replace | MergeMode::Range => incoming_ids,
    }
}

/// 🕹️ ONE decoded `MergeMode` drives both the local merge and the wire word — see
/// `pick_select_action`'s identical rationale.
///
/// 🕹️ Marquee/lasso stays GEOMETRIC here — `screen_select_instances` above is the surface's own
/// hit-test, this just batches its raw hits into ONE `interactionSelect`; the merge/mode algebra is
/// the os-kernel `next_selection` machine's job, not this file's.
#[cfg(test)]
pub(super) fn marquee_select_action(state: &mut World3dState, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    if state.marquee_points.len() < 2 {
        return None;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    let (meshes, draws) = scene_geometry(state);
    let ids = if component_mode_active(state) {
        screen_select_components(&meshes, &draws, view_proj, state.projection_spec, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
    state.marquee_points.clear();
    state.marquee_preview_ids.clear();
    let merge = world_merge_mode(shift, ctrl);
    if component_mode_active(state) {
        let object = state.active_object_id.as_deref().or_else(|| state.draws.iter().flat_map(|draw| &draw.instances).next().map(|instance| instance.id.as_str()))?;
        let targets: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": state.granularity, "id": resolved_item_id(state, &format!("{object}.{}.{id}", state.granularity)) })).collect();
        return Some(ActionDescriptor { controller_id: state.controller_id.clone(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(state), "targets": serde_json::to_string(&targets).expect("test targets encode"), "merge": merge.wire_label(), "method": "rectangle" })) });
    }
    let mut seen = HashSet::new();
    let targets: Vec<serde_json::Value> = ids
        .iter()
        .filter_map(|id| {
            let granularity = instance_interaction_granularity_id(state, id);
            let target_id = instance_interaction_id(state, id);
            seen.insert((granularity.to_string(), target_id.to_string())).then(|| serde_json::json!({ "granularity": granularity, "id": resolved_item_id(state, target_id) }))
        })
        .collect();
    Some(ActionDescriptor {
        controller_id: state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(state),
            "targets": serde_json::to_string(&targets).expect("test targets encode"),
            "merge": merge.wire_label(),
            "method": selection_method_wire_str(SelectionMethod::Rectangle),
        })),
    })
}

#[cfg(test)]
fn gumball_drag_update(state: &mut World3dState, x: f32, y: f32, inner: Rect) {
    let Some(handle) = state.gumball_handle else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, inner.w, inner.h);
    let pivot = state.gumball_pivot;
    let eye = gumball_eye(&camera, pivot);
    reset_gumball_preview(state);
    if handle.is_translate() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let delta = current - state.gumball_drag_anchor;
                state.gumball_preview_translate = axis.normalize().scale(delta);
            }
        } else if let Some(normal) = handle.plane_normal() {
            if let Some(current) = ray_plane_point(origin, dir, pivot, normal) {
                state.gumball_preview_translate = current.sub(state.gumball_drag_start_vec);
            }
        }
    } else if handle.is_rotate() {
        if let Some(normal) = handle.plane_normal() {
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let current = hit.sub(pivot);
                if current.length() > 1e-4 && state.gumball_drag_start_vec.length() > 1e-4 {
                    state.gumball_preview_angle = axis_rotate_angle(state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if state.gumball_drag_anchor.abs() > 1e-4 { current / state.gumball_drag_anchor } else { 1.0 };
                let factor = factor.clamp(0.05, 20.0);
                match handle {
                    GumballHandle::ScaleX => state.gumball_preview_scale.x = factor,
                    GumballHandle::ScaleY => state.gumball_preview_scale.y = factor,
                    GumballHandle::ScaleZ => state.gumball_preview_scale.z = factor,
                    _ => {}
                }
            }
        }
    }
    apply_gumball_preview(state);
}

#[cfg(test)]
fn start_gumball_drag(state: &mut World3dState, handle: GumballHandle, x: f32, y: f32, inner: Rect) {
    let Some(pivot) = selection_centroid(state) else {
        return;
    };
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, inner.w, inner.h);
    let eye = gumball_eye(&camera, pivot);
    state.gumball_handle = Some(handle);
    state.gumball_pivot = pivot;
    reset_gumball_preview(state);
    if let Some(axis) = handle.axis_dir() {
        state.gumball_drag_anchor = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        state.gumball_drag_start_vec = ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            state.gumball_drag_start_vec = state.gumball_drag_start_vec.sub(pivot);
        }
        state.gumball_drag_anchor = 0.0;
    }
}

#[cfg(test)]
pub(super) fn pick_component_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, String, String)> {
    let (local_x, local_y, rect) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (rect.w / rect.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.projection_spec, rect.w, rect.h);
    let granularity = state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for vertex_index in 0..schema.vertices {
                        let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                        let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                        let Some(screen) = ui_wgpu::wgpu::projection_spec_project_point(view_proj, state.projection_spec, world, rect.w, rect.h) else {
                            continue;
                        };
                        let dx = screen[0] - local_x;
                        let dy = screen[1] - local_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist <= PICK_VERTEX_SCREEN_PX && best.as_ref().is_none_or(|(best_dist, _, _)| dist < *best_dist) {
                            let group = world_mesh_component_id(mesh, Mesh3dField::VertexIds, vertex_index);
                    if group == u32::MAX { continue; }
                    let id = group.to_string();
                            best = Some((dist, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "edge" => {
            let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                if schema.edges == 0 || schema.edge_ids != schema.edges {
                    continue;
                }
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    for edge_index in 0..schema.edges {
                        let Ok(edge) = mesh.edge(edge_index) else { continue };
                        let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                        let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                        let Some(segment) = ui_wgpu::wgpu::projection_spec_project_segment(view_proj, state.projection_spec, a, b, rect.w, rect.h) else { continue; };
                        let [screen_a, screen_b] = segment.screen;
                        let screen_dist = ui_wgpu::wgpu::screen_segment_distance(local_x, local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                        if screen_dist > PICK_EDGE_SCREEN_PX {
                            continue;
                        }
                        let Some(closest) = ui_wgpu::wgpu::ray_segment_closest(origin, dir, segment.points[0], segment.points[1]) else { continue };
                        let ray_dist = closest.distance;
                        let depth = closest.segment_point.sub(origin).dot(dir);
                        if !depth.is_finite() || depth < 0.0 { continue; }
                        let better = match &best {
                            None => true,
                            Some((best_ray, best_depth, _, _)) => depth < *best_depth - 1e-4 || ((depth - *best_depth).abs() <= 1e-4 && ray_dist < *best_ray),
                        };
                        if better {
                            let id = world_mesh_component_id(mesh, Mesh3dField::EdgeIds, edge_index).to_string();
                            best = Some((ray_dist, depth, id, instance.id.clone()));
                        }
                    }
                }
            }
            return best.map(|(_, _, id, object_id)| (granularity.to_string(), id, object_id));
        }
        "face" => {
            let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &state.draws {
                let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(state, &instance.id) {
                        continue;
                    }
                    let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") else {
                        continue;
                    };
                    if best.as_ref().is_none_or(|(best_depth, _, _)| hit.distance < *best_depth) {
                        let id = mesh_face_id(mesh, u32::try_from(hit.triangle_index).ok()?);
                        best = Some((hit.distance, id, instance.id.clone()));
                    }
                }
            }
            return best.map(|(_, id, object_id)| (granularity.to_string(), id, object_id));
        }
        _ => {}
    }
    None
}

#[cfg(test)]
fn pick_paint_hit(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, f32, f32)> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") {
                if let Some((u, v)) = world_test_mesh_query(||interpolate_mesh_uv(mesh, hit.triangle_index, hit.bary_u, hit.bary_v)).expect("finite caller mesh query admission") {
                    if best.as_ref().is_none_or(|(best_dist, _, _, _)| hit.distance < *best_dist) {
                        best = Some((hit.distance, instance.id.clone(), u, v));
                    }
                }
            }
        }
    }
    best.map(|(_, object_id, u, v)| (object_id, u, v))
}

/// 🖱️ Screen-space marquee path for the selection rectangle overlay. Production pointer routing
/// retains the gesture on `WorldInteractionAuthority::marquee`; the legacy `marquee_points` /
/// `marquee_active` fields remain for the oracle tests only.
pub(super) fn world_marquee_overlay_points(state: &World3dState) -> Option<Vec<[f32; 2]>> {
    if let Some(marquee) = state.interaction_authority.as_ref()?.marquee.as_ref() {
        if !marquee.retiring && marquee.len >= 2 {
            let mut points = Vec::with_capacity(usize::from(marquee.len));
            for index in 0..usize::from(marquee.len) {
                points.push(marquee.points[index].expect("marquee gesture point"));
            }
            return Some(points);
        }
    }
    if state.marquee_active && state.marquee_points.len() >= 2 {
        return Some(state.marquee_points.clone());
    }
    None
}

#[cfg(test)]
fn marquee_local_polygon(state: &World3dState, rect: Rect) -> (Vec<[f32; 2]>, bool, bool) {
    let rectangle = state.selection_method != "lasso";
    let crossing = marquee_is_crossing_from_path(&state.marquee_points, !rectangle);
    let global: Vec<[f32; 2]> = if rectangle {
        let start = state.marquee_points[0];
        let end = state.marquee_points[state.marquee_points.len() - 1];
        vec![start, end]
    } else {
        state.marquee_points.clone()
    };
    let local = global.iter().map(|point| [point[0] - rect.x, point[1] - rect.y]).collect();
    (local, rectangle, crossing)
}

#[cfg(test)]
pub(super) fn update_marquee_preview(state: &mut World3dState, inner: Rect) {
    if state.marquee_points.len() < 2 {
        state.marquee_preview_ids.clear();
        return;
    }
    let camera = state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_wgpu::wgpu::projection_spec_view_proj(&camera, state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(state, inner);
    let (meshes, draws) = scene_geometry(state);
    state.marquee_preview_ids = if component_mode_active(state) {
        screen_select_components(&meshes, &draws, view_proj, state.projection_spec, inner.w, inner.h, &polygon, rectangle, state.granularity.as_str(), state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
}

#[cfg(test)]
pub(super) fn scene_geometry(state: &World3dState) -> (HashMap<String, Mesh3dLease>, Vec<SceneDraw3d>) {
    (state.meshes.iter().map(|(id, mesh)| (id.clone(), *mesh)).collect(), state.draws.iter().cloned().collect())
}

#[cfg(test)]
pub(super) fn pick_instance_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            if let Some(distance) = world_test_mesh_query(||ray_pick_instance(origin, dir, mesh, instance)).expect("finite caller mesh query admission") {
                if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                    best = Some((distance, instance.id.clone()));
                }
            }
        }
    }
    best.map(|(_, id)| id)
}

#[cfg(test)]
fn pick_surface_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<(String, [f64; 3], [f64; 3])> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, Vec3, Vec3)> = None;
    for draw in &state.draws {
        let Some(&mesh) = state.meshes.get(&draw.mesh_key) else {
            continue;
        };
        for instance in &draw.instances {
            let Some(hit) = world_test_mesh_query(||ray_pick_mesh_detail(origin, dir, mesh, instance)).expect("finite caller mesh query admission") else {
                continue;
            };
            if best.as_ref().is_none_or(|(best_distance, _, _, _)| hit.distance < *best_distance) {
                best = Some((hit.distance, instance.id.clone(), hit.point, hit.normal));
            }
        }
    }
    best.map(|(_, id, point, normal)| (id, [point.x as f64, point.y as f64, point.z as f64], [normal.x as f64, normal.y as f64, normal.z as f64]))
}

//#region VortexArrow
const VORTEX_ARROW_SHAFT_MESH: &str = "cylinder";
const VORTEX_ARROW_HEAD_MESH: &str = "cone";

struct VortexArrowLayout {
    point_radius: f32,
    shaft_radius: f32,
    shaft_length: f32,
    head_length: f32,
    shaft_center: [f32; 3],
    head_base: [f32; 3],
    pub(super) rotation: [f32; 4],
}

fn quat_normalize_f32(quat: [f32; 4]) -> [f32; 4] {
    let len = (quat[0] * quat[0] + quat[1] * quat[1] + quat[2] * quat[2] + quat[3] * quat[3]).sqrt();
    if len < 1e-9 {
        [0.0, 0.0, 0.0, 1.0]
    } else {
        [quat[0] / len, quat[1] / len, quat[2] / len, quat[3] / len]
    }
}

fn quat_from_unit_vectors(from: Vec3, to: Vec3) -> [f32; 4] {
    let from = from.normalize();
    let to = to.normalize();
    let r = from.dot(to) + 1.0;
    let quat = if r < 0.000_001 {
        if from.x.abs() > from.z.abs() {
            [-from.y, from.x, 0.0, r]
        } else {
            [0.0, -from.z, from.y, r]
        }
    } else {
        let cross = from.cross(to);
        [cross.x, cross.y, cross.z, r]
    };
    quat_normalize_f32(quat)
}

fn vortex_unit_direction(direction: Option<[f64; 3]>) -> Vec3 {
    let dir = direction.map_or(Vec3::new(0.0, 0.0, -1.0), |value| Vec3::new(value[0] as f32, value[1] as f32, value[2] as f32));
    if dir.dot(dir) < 1e-12 {
        Vec3::new(0.0, 0.0, -1.0)
    } else {
        dir.normalize()
    }
}

fn vortex_arrow_layout(position: [f64; 3], direction: Option<[f64; 3]>, radius: f32, display_direction: Option<&str>) -> VortexArrowLayout {
    let dir = vortex_unit_direction(direction);
    let pos = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
    let arrow_length = radius;
    let head_length = radius * 0.28;
    let shaft_length = (arrow_length - head_length).max(radius * 0.2);
    let shaft_radius = radius * 0.055;
    let point_radius = radius * 0.18;
    let outward = !display_direction.is_some_and(|mode| mode == "inwards");
    let rotation = quat_from_unit_vectors(Vec3::new(0.0, 1.0, 0.0), dir);
    let (shaft_center, head_base) = if outward { (pos.add(dir.scale(shaft_length * 0.5)), pos.add(dir.scale(shaft_length))) } else { (pos.sub(dir.scale(head_length + shaft_length * 0.5)), pos.sub(dir.scale(head_length))) };
    VortexArrowLayout { point_radius, shaft_radius, shaft_length, head_length, shaft_center: shaft_center.to_array(), head_base: head_base.to_array(), rotation }
}

fn ensure_primitive_mesh(state: &mut World3dState, mesh_key: &str) {
    if state.meshes.contains_key(mesh_key) {
        return;
    }
    begin_world_placeholder_mesh(state, mesh_key, WorldPlaceholderKind::resolve(mesh_key));
}

pub(super) fn append_vortex_arrow_draws(state: &mut World3dState, gpu: &mut World3dBuildContext, extra_draws: &mut Vec<SceneDraw3d>) {
    if state.vortices.is_empty() {
        return;
    }
    let mut point_instances = Vec::new();
    let mut shaft_instances = Vec::new();
    let mut head_instances = Vec::new();
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let layout = vortex_arrow_layout(position, vortex.direction, radius, vortex.display_direction.as_deref());
        let color = parse_color(vortex.color.as_deref().unwrap_or("#38bdf8"));
        let hovered = state.hovered_vortex_id.as_deref() == Some(vortex.full_id.as_str());
        let id = vortex.full_id.clone();
        point_instances.push(Instance3d { component_source: None,
            id: format!("{id}:point"),
            model: Instance3d::model_from_trs([position[0] as f32, position[1] as f32, position[2] as f32], [0.0, 0.0, 0.0, 1.0], [layout.point_radius, layout.point_radius, layout.point_radius]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        shaft_instances.push(Instance3d { component_source: None,
            id: format!("{id}:shaft"),
            model: Instance3d::model_from_trs(layout.shaft_center, layout.rotation, [layout.shaft_radius * 2.0, layout.shaft_length, layout.shaft_radius * 2.0]),
            color,
            selected: false,
            hovered,
         material: Default::default() });
        head_instances.push(Instance3d { component_source: None, id: format!("{id}:head"), model: Instance3d::model_from_trs(layout.head_base, layout.rotation, [layout.shaft_radius * 3.6, layout.head_length, layout.shaft_radius * 3.6]), color, selected: false, hovered , material: Default::default() });
    }
    ensure_primitive_mesh(state, "vortex-marker");
    ensure_primitive_mesh(state, VORTEX_ARROW_SHAFT_MESH);
    ensure_primitive_mesh(state, VORTEX_ARROW_HEAD_MESH);
    for (mesh_key, instances) in [("vortex-marker", point_instances), (VORTEX_ARROW_SHAFT_MESH, shaft_instances), (VORTEX_ARROW_HEAD_MESH, head_instances)] {
        let mesh_version = live_world3d_mesh_version(state, mesh_key);
        if let Some(mesh) = state.meshes.get(mesh_key).copied() {
            gpu.ensure_mesh(mesh_key, mesh_version, mesh);
            extra_draws.push(SceneDraw3d { mesh_key: mesh_key.into(), mesh_version, instances, shadow_role: Default::default() });
        }
    }
}
//#endregion VortexArrow

#[cfg(test)]
fn pick_vortex_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for vortex in &state.vortices {
        let position = vortex.position.unwrap_or([0.0, 0.0, 0.0]);
        let radius = vortex.radius.unwrap_or(0.36) as f32;
        let center = Vec3::new(position[0] as f32, position[1] as f32, position[2] as f32);
        let min = [center.x - radius, center.y - radius, center.z - radius];
        let max = [center.x + radius, center.y + radius, center.z + radius];
        if let Some(distance) = ray_aabb_slab(origin, dir, min, max) {
            if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
                best = Some((distance, vortex.full_id.clone()));
            }
        }
    }
    best.map(|(_, id)| id)
}

/// 🖼️ Ray-vs-quad hit test against visible reference-image planes, returning the closest hit
/// reference's authored `id`. References are flat rectangles lying in the local XY plane (normal +Z,
/// width along X, height along Y) centered at `origin` — matching this renderer's Z-up convention.
#[cfg(test)]
fn pick_reference_at(state: &World3dState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let plane_normal = Vec3::new(0.0, 0.0, 1.0);
    let mut best: Option<(f32, String)> = None;
    for reference in &state.references {
        if reference.hidden.unwrap_or(false) || reference.locked.unwrap_or(false) {
            continue;
        }
        let Some(url) = reference.url.as_deref() else {
            continue;
        };
        let plane_position = reference.origin.unwrap_or([0.0, 0.0, 0.0]);
        let plane_origin = Vec3::new(plane_position[0] as f32, plane_position[1] as f32, plane_position[2] as f32);
        let width = world_reference_width(reference);
        let image_aspect = reference_image_aspect(state, url);
        let height = width / image_aspect.max(0.01);
        let Some(hit) = ray_plane_point(origin, dir, plane_origin, plane_normal) else {
            continue;
        };
        let offset = hit.sub(plane_origin);
        if offset.x.abs() > width * 0.5 || offset.y.abs() > height * 0.5 {
            continue;
        }
        let distance = origin.sub(hit).length();
        if best.as_ref().is_none_or(|(best_distance, _)| distance < *best_distance) {
            best = Some((distance, reference.id.as_deref().unwrap_or(url).to_string()));
        }
    }
    best.map(|(_, id)| id)
}

/// 📍️ One instance's own world ORIGIN — the translation column of its model matrix. Production since
/// the relocate gesture landed: `world3d_begin_relocate_drag` needs the grabbed object's origin to
/// apply React's `origin + (to - from)` travel (`world3dRelocateDispatchArgsV1`).
fn object_world_position(state: &World3dState, object_id: &str) -> Option<[f32; 3]> {
    for draw in &state.draws {
        for instance in &draw.instances {
            if instance.id == object_id {
                let translation = instance.model.cols[3];
                return Some([translation[0], translation[1], translation[2]]);
            }
        }
    }
    None
}

#[cfg(test)]
fn update_dragged_instance_position(state: &mut World3dState, object_id: &str, position: [f32; 3]) {
    for draw in &mut state.draws {
        for instance in &mut draw.instances {
            if instance.id == object_id {
                instance.model.cols[3] = [position[0], position[1], position[2], 1.0];
            }
        }
    }
}

/// 🎯️ Intersects the camera ray through a shell client point with the world Z=0 ground plane.
pub fn world3d_ground_plane_pick(state: &World3dState, x: f32, y: f32, plane_z: f32) -> Option<[f32; 3]> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(state, x, y)?;
    let camera = state.orbit.to_camera();
    let (origin, dir) = ui_wgpu::wgpu::projection_spec_ray_from_screen(&camera, state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    if dir.z.abs() < 1e-5 {
        return None;
    }
    let t = (plane_z - origin.z) / dir.z;
    if t < 0.0 {
        return None;
    }
    let hit = origin.add(dir.scale(t));
    Some([hit.x, hit.y, hit.z])
}

//#region WorldRelocateGesture
/// 🚚️ One live relocate drag. `origin` is where the grabbed object sat when the press landed and
/// `from` is the ground point under that press; the commit applies the ground-plane travel
/// `from → to` to `origin`, never an incremental pose delta — React's `World3dRelocateSession`
/// (`🌐️World3dHost/🟦️.tsx:4752`).
#[derive(Clone, Debug, PartialEq)]
pub struct World3dRelocateSession {
    pub object_id: String,
    pub origin: [f32; 3],
    pub from: [f32; 3],
}

/// 📏️ `GUMBALL_TRANSFORM_EPSILON` (`🌐️World3dHost/🟦️.tsx:2496`) — a drag that lands within this of
/// where it started writes no document edit.
const WORLD_RELOCATE_EPSILON: f64 = 1e-6;

/// 🚚️ Which object a relocate press grabs — React `world3dRelocateDragTargetV1`
/// (`🌐️World3dHost/🟦️.tsx:4726`). A press on empty canvas relocates the first selected object; a press
/// with no selection relocates what it landed on; a press INSIDE a selection relocates that object,
/// and a press on something outside the selection relocates nothing (it is a selection gesture, and
/// `clearSelection` stays the way to drop a selection).
pub fn world3d_relocate_drag_target(pressed_id: Option<&str>, selected_ids: &[String]) -> Option<String> {
    let Some(pressed_id) = pressed_id.filter(|id| !id.is_empty()) else {
        return selected_ids.first().cloned();
    };
    if selected_ids.is_empty() {
        return Some(pressed_id.to_string());
    }
    selected_ids.iter().any(|id| id == pressed_id).then(|| pressed_id.to_string())
}

/// 🚚️ The ONE `worldRelocate` payload a finished relocate drag commits — React
/// `world3dRelocateDispatchArgsV1` (`🌐️World3dHost/🟦️.tsx:4738`). The ground travel `from → to` is
/// added to the grabbed object's own origin and snapped exactly like a catalogue drop; a drag that
/// lands within [`WORLD_RELOCATE_EPSILON`] of where it started answers `None`, so a click without
/// travel never writes a document edit.
pub fn world3d_relocate_dispatch_args(session: &World3dRelocateSession, to: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> Option<[f64; 3]> {
    let moved = [session.origin[0] + (to[0] - session.from[0]), session.origin[1] + (to[1] - session.from[1]), session.origin[2] + (to[2] - session.from[2])];
    let position = snap_world_point_to_grid(moved, grid_snap_enabled, grid_factor);
    let unchanged = (0..3).all(|axis| (position[axis] - f64::from(session.origin[axis])).abs() < WORLD_RELOCATE_EPSILON);
    (!unchanged).then_some(position)
}

/// 🚚️ Whether this surface is in React's `relocateMode` — `activeUtility === "worldRelocate"`
/// (`🌐️World3dHost/🟦️.tsx:5656`).
pub fn world3d_relocate_mode(state: &World3dState) -> bool {
    state.active_utility == "worldRelocate"
}

/// 🚚️ Arms a relocate drag at a primary press. The pressed object is the one this surface's own hover
/// authority last resolved (`local_hover_id`) — the wgpu stand-in for the r3f pointer event's own
/// `object` target, which is the same object by construction since the hover cursor resolves every
/// move before the press lands. Answers `false` when the press grabs nothing, which leaves the press
/// to the ordinary selection route.
pub fn world3d_begin_relocate_drag(state: &mut World3dState, x: f32, y: f32) -> bool {
    if !world3d_relocate_mode(state) {
        return false;
    }
    let Some(object_id) = world3d_relocate_drag_target(state.local_hover_id.as_deref(), &state.selected_ids) else {
        return false;
    };
    let Some(origin) = object_world_position(state, &object_id) else {
        return false;
    };
    let Some(from) = world3d_ground_plane_pick(state, x, y, origin[2]) else {
        return false;
    };
    state.relocate = Some(World3dRelocateSession { object_id, origin, from });
    true
}

/// 🚚️ Moves the relocate ghost. React previews the travel with the SAME catalogue-drop ghost a drop
/// from the catalogue uses (`setWorldCatalogueDropPreview`, `🌐️World3dHost/🟦️.tsx:6851`) and
/// dispatches nothing until the release.
pub fn world3d_update_relocate_drag(state: &mut World3dState, x: f32, y: f32) -> bool {
    let Some(session) = state.relocate.clone() else {
        return false;
    };
    let origin = world3d_ground_plane_pick(state, x, y, session.origin[2]).and_then(|to| world3d_relocate_dispatch_args(&session, to, state.lod.grid_snap_enabled, state.lod.grid_factor)).unwrap_or([
        f64::from(session.origin[0]),
        f64::from(session.origin[1]),
        f64::from(session.origin[2]),
    ]);
    let object_kind = state.relocate.as_ref().map(|session| session.object_id.clone()).unwrap_or_default();
    state.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind, mesh_url: world3d_mesh_url_for_object(state, &session.object_id), origin });
    true
}

/// 🚚️ Closes the relocate drag and answers the position to commit, or `None` for an abort or a drag
/// that never travelled. `to` is `None` for Escape and for a cancelled pointer — React's
/// `endRelocateDrag(null)`, which drops the ghost and dispatches nothing.
pub fn world3d_end_relocate_drag(state: &mut World3dState, to: Option<(f32, f32)>) -> Option<(String, [f64; 3])> {
    let session = state.relocate.take()?;
    state.catalogue_drop_preview = None;
    let (x, y) = to?;
    let point = world3d_ground_plane_pick(state, x, y, session.origin[2])?;
    let position = world3d_relocate_dispatch_args(&session, point, state.lod.grid_snap_enabled, state.lod.grid_factor)?;
    Some((session.object_id, position))
}

/// ⎋️ Escape drops a live relocate drag and dispatches nothing — React binds it as a CAPTURE-phase
/// `keydown` while `relocateMode` is on (`🌐️World3dHost/🟦️.tsx:6875`). Answers whether a drag was
/// actually cancelled, so the caller can stop the key there exactly as React calls
/// `event.stopPropagation()` only when `endRelocateDrag` returns true.
pub fn world3d_cancel_relocate_drag(state: &mut World3dState) -> bool {
    if state.relocate.is_none() {
        return false;
    }
    world3d_end_relocate_drag(state, None);
    true
}

/// 🚚️ Whether a relocate drag is live on this surface.
pub fn world3d_relocate_active(state: &World3dState) -> bool {
    state.relocate.is_some()
}

/// 🥽️ The mesh url the relocate ghost should draw with — the grabbed instance's own mesh, so the
/// preview is the object being moved rather than a stand-in box.
fn world3d_mesh_url_for_object(state: &World3dState, object_id: &str) -> Option<String> {
    for draw in &state.draws {
        if draw.instances.iter().any(|instance| instance.id == object_id) {
            return state.mesh_source_urls.get(&draw.mesh_key).cloned();
        }
    }
    None
}

/// 🚚️ The `worldRelocate` plan a finished relocate drag publishes.
pub(super) fn plan_world3d_relocate(state: &World3dState, generation: u64, object_id: &str, position: [f64; 3]) -> Option<WorldInteractionPlan> {
    let mut plan = WorldInteractionPlan::new(state.interaction_revision, generation);
    let controller = plan.push_string(&state.controller_id)?;
    let surface = plan.push_string(&state.surface_id)?;
    let object = plan.push_string(object_id)?;
    let action = WorldFlatAction { kind: WorldFlatActionKind::Relocate, strings: [Some(controller), Some(surface), Some(object), None, None, None, None, None], numbers: [position[0], position[1], position[2], 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0] };
    plan.push_action(action).then_some(plan)
}
//#endregion WorldRelocateGesture

fn snap_world_point_to_grid(point: [f32; 3], grid_snap_enabled: bool, grid_factor: f64) -> [f64; 3] {
    if !grid_snap_enabled || grid_factor <= 0.0 {
        return [point[0] as f64, point[1] as f64, point[2] as f64];
    }
    let snap = |value: f32| (value as f64 / grid_factor).round() * grid_factor;
    [snap(point[0]), snap(point[1]), snap(point[2])]
}

/// 👻️ Updates the live catalogue-drop ghost for a wgpu `World3d` pane under the pointer.
pub fn world3d_update_catalogue_drop_preview(state: &mut World3dState, client_x: f32, client_y: f32, object_kind: &str, mesh_url: Option<&str>) {
    let Some(hit) = world3d_ground_plane_pick(state, client_x, client_y, 0.0) else {
        state.catalogue_drop_preview = None;
        return;
    };
    let origin = snap_world_point_to_grid(hit, state.lod.grid_snap_enabled, state.lod.grid_factor);
    state.catalogue_drop_preview = Some(WorldCatalogueDropPreviewRecord { object_kind: object_kind.to_string(), mesh_url: mesh_url.filter(|url| !url.is_empty()).map(str::to_string), origin });
}

/// 👻️ Clears the catalogue-drop ghost on one `World3d` pane.
pub fn world3d_clear_catalogue_drop_preview(state: &mut World3dState) {
    state.catalogue_drop_preview = None;
}

/// 🎯️ Grid-snapped ground origin for committing a catalogue drop on a wgpu `World3d` pane.
pub fn world3d_catalogue_drop_origin(state: &World3dState, client_x: f32, client_y: f32) -> Option<[f64; 3]> {
    world3d_ground_plane_pick(state, client_x, client_y, 0.0).map(|hit| snap_world_point_to_grid(hit, state.lod.grid_snap_enabled, state.lod.grid_factor))
}

pub(super) fn preview_scale(scale: Option<&serde_json::Value>) -> [f32; 3] {
    match scale {
        Some(serde_json::Value::Array(values)) if values.len() >= 3 => [values[0].as_f64().unwrap_or(1.0) as f32, values[1].as_f64().unwrap_or(1.0) as f32, values[2].as_f64().unwrap_or(1.0) as f32],
        _ => [1.0, 1.0, 1.0],
    }
}

pub(super) fn reference_image_aspect(state: &World3dState, url: &str) -> f32 {
    state
        .reference_pixels
        .get(url)
        .map_or(1.0, |raster| {
            let descriptor = raster.identity.descriptor();
            descriptor.width as f32 / descriptor.height.max(1) as f32
        })
        .max(0.01)
}

pub(super) fn world_reference_plane_size(state: &World3dState, url: &str, width: f32) -> [f32; 2] {
    [width, width / reference_image_aspect(state, url)]
}

/// 👻️ Mesh key for `BrushPreviewGhost`: the real GLB's resolved id when a `meshUrl` is given
/// (loaded lazily, same as any other mesh), else the shared "box" primitive fallback ghost.
pub(super) fn brush_preview_mesh_id(mesh_url: Option<&str>) -> String {
    mesh_url.map_or_else(|| "box".to_string(), mesh_id_from_url)
}

/// 👻️ The key a GHOST actually draws through. React's `BrushPreviewGhost` renders a translucent unit
/// box until its GLB resolves and the real mesh after — but React's ghost owns a separate three.js
/// object, while here every draw shares ONE mesh table keyed by mesh id. Minting the ghost's box
/// under a `mesh:…` id the scene is still waiting on would make that id resident, and
/// `offer_missing_mesh_fetches` offers a url only while its key is NOT resident — the ghost would
/// permanently suppress the fetch for the mesh it is standing in for. So a ghost over an unresolved
/// url draws through the shared `box` primitive instead, and switches to the real id the frame its
/// mesh lands.
pub(super) fn ghost_mesh_id(state: &World3dState, mesh_url: Option<&str>) -> String {
    let id = brush_preview_mesh_id(mesh_url);
    if state.meshes.contains_key(&id) || !scene_mesh_awaits_its_asset(state, &id) {
        return id;
    }
    "box".to_string()
}

pub(super) fn append_box_wireframe(lines: &mut Vec<LineVertex3d>, origin: [f64; 3], orientation: [f64; 4], scale: [f64; 3], color: [f32; 4]) {
    let corners = [[-0.5, -0.5, -0.5], [0.5, -0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, 0.5, 0.5]];
    let model = Instance3d::model_from_trs([origin[0] as f32, origin[1] as f32, origin[2] as f32], [orientation[0] as f32, orientation[1] as f32, orientation[2] as f32, orientation[3] as f32], [scale[0] as f32, scale[1] as f32, scale[2] as f32]);
    let world_corners: Vec<[f32; 3]> = corners.iter().map(|corner| model.transform_point(Vec3::new(corner[0], corner[1], corner[2])).to_array()).collect();
    let edges = [(0, 1), (1, 2), (2, 3), (3, 0), (4, 5), (5, 6), (6, 7), (7, 4), (0, 4), (1, 5), (2, 6), (3, 7)];
    for (a, b) in edges {
        lines.push(LineVertex3d { position: world_corners[a], color });
        lines.push(LineVertex3d { position: world_corners[b], color });
    }
}

pub(super) fn parse_color(value: &str) -> [f32; 4] {
    let trimmed = value.trim();
    if let Some(hex) = trimmed.strip_prefix('#') {
        let expanded = match hex.len() {
            3 => hex.chars().map(|ch| format!("{ch}{ch}")).collect::<String>(),
            _ => hex.to_string(),
        };
        if expanded.len() >= 6 {
            let r = u8::from_str_radix(&expanded[0..2], 16).unwrap_or(148) as f32 / 255.0;
            let g = u8::from_str_radix(&expanded[2..4], 16).unwrap_or(163) as f32 / 255.0;
            let b = u8::from_str_radix(&expanded[4..6], 16).unwrap_or(184) as f32 / 255.0;
            return [srgb_to_linear(r), srgb_to_linear(g), srgb_to_linear(b), 1.0];
        }
    }
    [srgb_to_linear(0.58), srgb_to_linear(0.64), srgb_to_linear(0.72), 1.0]
}

/// 🎨️ One sRGB channel in the LINEAR working space every world pipeline shades in.
///
/// 🩸️ `parse_color` used to hand raw sRGB bytes straight to the GPU. The world pass renders into an
/// sRGB VIEW (`🧊️gpu/🦀️.rs`'s `color_target_format`), so those bytes were encoded a second time on
/// write and every wire colour came out washed out against React, whose `new Color(hex)` linearizes
/// through `ColorManagement` exactly like this. `Theme` was always linear; only this reader was not.
pub(super) fn srgb_to_linear(channel: f32) -> f32 {
    if channel <= 0.04045 {
        channel / 12.92
    } else {
        ((channel + 0.055) / 1.055).powf(2.4)
    }
}
