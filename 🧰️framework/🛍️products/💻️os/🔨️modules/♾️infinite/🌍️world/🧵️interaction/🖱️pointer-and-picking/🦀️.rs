//! 🖱️ World3d pointer handlers, context menu, action previews, gumball drag and the pick entry points.

use super::*;

pub fn world3d_hit_target(scene: &UiComponentSceneNode, bounds: Rect) -> HitTarget<ActionDescriptor> {
    HitTarget { rect: bounds, event: None, control_id: Some(scene.surface_id.clone()), kind: HitKind::World3d, drag_axis: None, drag_data: None }
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_pointer_move(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, x: f32, y: f32, down: bool, button: i16) -> Option<ActionDescriptor> {
    let inner = world_pick_rect(session_state);
    if !inner.contains(x, y) {
        camera_state.gizmo_hovered_tip = None;
        return None;
    }
    if down && button == 0 {
        if selection_state.marquee_active {
            selection_state.marquee_points.push([x, y]);
            update_marquee_preview(camera_state, selection_state, scene_state, render_pick_viewport(session_state));
        } else if selection_state.paint_stroke_active && selection_state.interaction_mode == "paint" {
            if let Some((object_id, u, v)) = pick_paint_hit(session_state, camera_state, scene_state, x, y, inner) {
                return Some(ActionDescriptor {
                    controller_id: session_state.controller_id.clone(),
                    action: "paintAt".into(),
                    args: Some(semio_framework::dsl_value!({
                        "surfaceId": session_state.surface_id,
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
        update_world_orbit_view_gizmo_hover(camera_state, x, y, inner);
        return pick_hover_action(session_state, camera_state, selection_state, interaction_state, scene_state, x, y, inner);
    }
    if button == 2 {
        return None;
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_paint_actions(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, x: f32, y: f32, down: bool, button: i16) -> Vec<ActionDescriptor> {
    if selection_state.interaction_mode != "paint" || button != 0 {
        return Vec::new();
    }
    let inner = world_pick_rect(session_state);
    if !inner.contains(x, y) {
        return Vec::new();
    }
    if down && selection_state.paint_stroke_active {
        let Some((object_id, u, v)) = pick_paint_hit(session_state, camera_state, scene_state, x, y, inner) else {
            return Vec::new();
        };
        return vec![ActionDescriptor {
            controller_id: session_state.controller_id.clone(),
            action: "paintAt".into(),
            args: Some(semio_framework::dsl_value!({
                "surfaceId": session_state.surface_id,
                "objectId": object_id,
                "u": u,
                "v": v,
                "phase": "stream",
            })),
        }];
    }
    Vec::new()
}

/// 🖱️📋️ Resolves which entity a right-click context menu targets, in the React reference's exact
/// priority order — `resolveWorldContextMenuTarget` in `🧱️elements/🌐️World3dHost/🟦️.tsx:1893`: a
/// hovered vortex wins first, then a hovered mesh component (vertex/edge/face — reported as kind
/// `"object"`, the component's owning instance, matching the React source's own naming), then a
/// hovered reference image plane, then any other plain hover as `"object"`; `None` (no target) if
/// nothing is currently hovered.
pub fn resolve_world_context_menu_target(selection_state: &WorldSelectionState) -> Option<(&'static str, String)> {
    if let Some(vortex_id) = selection_state.hovered_vortex_id.clone() {
        return Some(("vortex", vortex_id));
    }
    if selection_state.hovered_component_mode.is_some() {
        if let Some(object_id) = selection_state.hovered_component_object_id.clone() {
            return Some(("object", object_id));
        }
    }
    let hovered = selection_state.local_hover_id.as_deref()?;
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
pub fn world3d_context_menu_surface(selection_state: &WorldSelectionState) -> (Vec<semio_framework_ui_declaration::ui::ContextMenuHit>, Vec<semio_framework_ui_declaration::ui::ContextMenuSelectionGroup>) {
    let hits = resolve_world_context_menu_target(selection_state).map(|(domain, id)| semio_framework_ui_declaration::ui::ContextMenuHit { domain: domain.to_string(), id, label: None }).into_iter().collect();
    let mut selection = Vec::new();
    if !selection_state.selected_ids.is_empty() {
        selection.push(semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: "object".into(), ids: selection_state.selected_ids.clone() });
    }
    if !selection_state.component_ids.is_empty() {
        selection.push(semio_framework_ui_declaration::ui::ContextMenuSelectionGroup { domain: "feature".into(), ids: selection_state.component_ids.clone() });
    }
    (hits, selection)
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_pointer_drag(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32, dx: f32, dy: f32, button: i16, modifiers: &PointerModifiers) {
    let inner = world_pick_rect(session_state);
    if button == 0 {
        if gumball_state.gumball_handle.is_some() && inner.contains(x, y) {
            gumball_drag_update(camera_state, selection_state, gumball_state, scene_state, x, y, inner);
            return;
        }
        if selection_state.drag_object_id.is_none() && gumball_state.gumball_handle.is_none() && !component_mode_active(selection_state) && pointer_drag_distance(selection_state, x, y) > CLICK_DRAG_THRESHOLD_PX {
            if let Some(object_id) = selection_state.press_object_id.take().or_else(|| pick_instance_at(session_state, camera_state, scene_state, x, y, inner)) {
                selection_state.drag_object_id = Some(object_id.clone());
                selection_state.marquee_active = false;
                selection_state.marquee_points.clear();
                selection_state.marquee_preview_ids.clear();
                selection_state.drag_last_position = object_world_position(scene_state, &object_id);
                selection_state.drag_object_z = selection_state.drag_last_position.map(|p| p[2]).unwrap_or(0.0);
            }
        }
        if selection_state.drag_object_id.is_some() && inner.contains(x, y) {
            if let Some(position) = world3d_ground_plane_pick(session_state, camera_state, x, y, selection_state.drag_object_z) {
                selection_state.drag_last_position = Some(position);
                if let Some(object_id) = selection_state.drag_object_id.clone() {
                    update_dragged_instance_position(scene_state, &object_id, position);
                }
            }
            return;
        }
    }
    if button == 1 || (button == 2 && modifiers.shift) {
        camera_state.orbit.pan(-dx, -dy);
    } else if button == 2 && (modifiers.alt || modifiers.meta) {
        camera_state.orbit.orbit(dx, dy);
    }
}

#[cfg(any(test, feature = "testkit"))]
pub(crate) fn handle_world3d_wheel(camera_state: &mut WorldCameraState, delta: f32) {
    camera_state.orbit.zoom(delta);
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

#[cfg(any(test, feature = "testkit"))]
fn world_item_target_id(surface_id: &str, object_id: &str) -> String {
    format!("{surface_id}{WORLD_ITEM_PATH_DELIMITER}{object_id}")
}

/// 🔤️ Strips this state's own `surfaceId/` prefix from a `world`-domain item target id — the inverse
/// of [`world_item_target_id`]. Targets for a different surface (cross-surface batch, not produced by
/// this file's single-surface pointer handlers) are dropped rather than mis-parsed.
fn world_item_id_for_surface<'a>(session_state: &WorldSessionState, target_id: &'a str) -> Option<&'a str> {
    target_id.strip_prefix(&session_state.surface_id).and_then(|rest| rest.strip_prefix(WORLD_ITEM_PATH_DELIMITER))
}

/// 🧮️ Builds the wire target id for a plain pick/hover hit: a bare object id when a real app domain is
/// bound (`HierarchyProvider::Flat`-style, single-surface-scoped), else `world_item_target_id`'s
/// `"surfaceId/id"` `PathDelimited` shape for the shared `world` domain.
#[cfg(any(test, feature = "testkit"))]
pub fn resolved_item_id(session_state: &WorldSessionState, object_id: &str) -> String {
    if session_state.bound_domain_id.is_some() {
        object_id.to_string()
    } else {
        world_item_target_id(&session_state.surface_id, object_id)
    }
}

/// 🔤️ Inverse of [`resolved_item_id`], for parsing ids back out of an incoming action's targets during
/// optimistic local preview.
fn parse_resolved_item_id<'a>(session_state: &WorldSessionState, target_id: &'a str) -> Option<&'a str> {
    if session_state.bound_domain_id.is_some() {
        Some(target_id)
    } else {
        world_item_id_for_surface(session_state, target_id)
    }
}

/// 🕹️ [`world_merge_mode`] as the fixed `u8` the bounded pick cursors carry through a retained
/// gesture — the cursors hold no `String`, so the mode travels as its ordinal and is spelled once,
/// at publication, by [`world_merge_wire_label`].
pub fn world_merge_code(shift: bool, ctrl: bool, meta: bool) -> u8 {
    match world_merge_mode(shift, ctrl || meta) {
        MergeMode::Additive => 1,
        MergeMode::Subtractive => 2,
        MergeMode::Invertive => 3,
        _ => 0,
    }
}

/// 🕹️ Projects the original encoded interaction targets into the existing local preview fields.
pub fn apply_world_action_preview(session_state: &mut WorldSessionState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, action: &ActionDescriptor) {
    let Some(args) = action.args.as_ref() else { return; };
    if args.get("domainId").and_then(|value| value.as_str()) != Some(resolved_domain_id(session_state)) || !matches!(action.action.as_str(), "interactionSelect" | "interactionHover") { return; }
    let Some(text) = args.get("targets").and_then(|value| value.as_str()).filter(|text| text.len() <= semio_framework_ui_action::ACTION_STRING_BYTE_CAPACITY) else { return; };
    let Ok(targets) = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject) else { return; };
    let Some(targets) = targets.as_array() else { return; };
    let mut ids = Vec::new();
    let mut components = Vec::new();
    let mut component_object = None;
    let mut component_mode = None;
    for target in targets {
        let (Some(granularity), Some(id)) = (target.get("granularity").and_then(|value| value.as_str()), target.get("id").and_then(|value| value.as_str()).and_then(|id| parse_resolved_item_id(session_state, id))) else { return; };
        if matches!(granularity, "vertex" | "edge" | "face") {
            let Some((object, group)) = world_component_preview_target(scene_state, id, granularity) else { return; };
            components.push(group.to_string());
            component_object.get_or_insert_with(|| object.to_string());
            component_mode.get_or_insert_with(|| granularity.to_string());
        }
        ids.push(id.to_string());
    }
    if action.action == "interactionSelect" {
        let Some(merge) = args.get("merge").and_then(|value| value.as_str()).and_then(MergeMode::from_wire_label) else { return; };
        if let Some(mode) = component_mode {
            gumball_state.gumball_selection_ids = merge_string_ids(&gumball_state.gumball_selection_ids, &ids, merge);
            let selected: Vec<(String,u32)> = gumball_state.gumball_selection_ids.iter().filter_map(|id| world_component_preview_target(scene_state,id,&mode).map(|(object,group)|(object.to_string(),group))).collect();
            selection_state.selected_ids = selected.iter().map(|(object,_)|object.clone()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            let retained = |object:&str|selected.iter().any(|(selected,_)|selected==object);
            selection_state.active_object_id = component_object.filter(|object|retained(object)).or_else(||selection_state.active_object_id.take().filter(|object|retained(object))).or_else(||selected.first().map(|(object,_)|object.clone()));
            selection_state.component_ids = selected.iter().filter(|(object,_)|Some(object)==selection_state.active_object_id.as_ref()).map(|(_,group)|group.to_string()).collect::<std::collections::BTreeSet<_>>().into_iter().collect();
            selection_state.granularity = mode;
        } else {
            selection_state.selected_ids = merge_string_ids(&selection_state.selected_ids, &ids, merge);
            if merge == MergeMode::Replace { gumball_state.gumball_selection_ids.clear();selection_state.component_ids.clear();selection_state.active_object_id=None; }
        }
    } else if component_mode.is_some() {
        selection_state.hovered_component_id = components.into_iter().next();
        selection_state.hovered_component_object_id = component_object;
        selection_state.hovered_component_mode = component_mode;
        selection_state.local_hover_id = None;
        selection_state.local_hover_granularity_id = None;
    } else {
        selection_state.hovered_component_id = None;
        selection_state.hovered_component_object_id = None;
        selection_state.hovered_component_mode = None;
        selection_state.local_hover_id = ids.into_iter().next();
        selection_state.local_hover_granularity_id = targets.first().and_then(|target| target.get("granularity")).and_then(|value| value.as_str()).map(str::to_string);
    }
}

/// 🖼️ Falls back to reference-image plane hit-testing when no mesh instance is under the
/// cursor — mirrors React's `hoveredId` covering both mesh instances and `"reference:"`-prefixed
/// reference planes (see `resolveWorldContextMenuTarget`'s reference tier).
///
/// 🕹️ `interactionHover` — empty `targets` clears the channel (see `HoverInput`/`next_hover`);
/// `domainId`/target id shape follow this surface's resolved (app-bound or `world`-fallback) domain.
#[cfg(any(test, feature = "testkit"))]
pub fn pick_hover_action(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, x: f32, y: f32, inner: Rect) -> Option<ActionDescriptor> {
    if session_state.active_utility == "surfaceBrush" {
        if let Some((object_id, position, normal)) = pick_surface_at(session_state, camera_state, scene_state, x, y, inner) {
            return Some(ActionDescriptor {
                controller_id: session_state.controller_id.clone(),
                action: "worldSurfaceHover".into(),
                args: Some(semio_framework::dsl_value!({
                    "surfaceId": session_state.surface_id,
                    "pane": session_state.surface_id,
                    "objectId": object_id,
                    "position": position,
                    "normal": normal,
                })),
            });
        }
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "worldSurfaceLeave".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": session_state.surface_id, "pane": session_state.surface_id })) });
    }
    if session_state.active_utility == "brush" || (session_state.active_utility == "select" && selection_state.granularity == "vertex") {
        let hit = pick_vortex_at(session_state, camera_state, scene_state, x, y, inner);
        if selection_state.hovered_vortex_id == hit {
            return None;
        }
        selection_state.hovered_vortex_id = hit.clone();
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "worldVortexHover".into(), args: Some(semio_framework::dsl_value!({ "surfaceId": session_state.surface_id, "fullId": hit })) });
    }
    if component_mode_active(selection_state) {
        let hit = pick_component_at(session_state, camera_state, selection_state, scene_state, x, y, inner);
        if selection_state.hovered_component_id.as_deref() == hit.as_ref().map(|(_, id, _)| id.as_str()) && selection_state.hovered_component_object_id.as_deref() == hit.as_ref().map(|(_, _, object)| object.as_str()) && selection_state.hovered_component_mode.as_deref() == hit.as_ref().map(|(kind, _, _)| kind.as_str()) { return None; }
        selection_state.hovered_component_id = hit.as_ref().map(|(_, id, _)| id.clone());
        selection_state.hovered_component_object_id = hit.as_ref().map(|(_, _, object)| object.clone());
        selection_state.hovered_component_mode = hit.as_ref().map(|(kind, _, _)| kind.clone());
        selection_state.local_hover_id = None; selection_state.local_hover_granularity_id = None;
        let id = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&session_state.surface_id), id.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "channel": "pointer", "targets": targets.as_str() })) });
    }
    let hit = pick_instance_at(session_state, camera_state, scene_state, x, y, inner).or_else(|| pick_reference_at(session_state, camera_state, scene_state, x, y, inner).map(|url| format!("reference:{url}")));
    let target = hit.as_deref().map(|id| (instance_interaction_id(interaction_state, id).to_string(), instance_interaction_granularity_id(session_state, interaction_state, id).to_string()));
    if selection_state.local_hover_id.as_deref() == target.as_ref().map(|(id, _)| id.as_str()) && selection_state.local_hover_granularity_id.as_deref() == target.as_ref().map(|(_, granularity)| granularity.as_str()) {
        return None;
    }
    selection_state.local_hover_id = target.as_ref().map(|(id, _)| id.clone());
    selection_state.local_hover_granularity_id = target.as_ref().map(|(_, granularity)| granularity.clone());
    let targets = world_interaction_targets(Some(&session_state.surface_id), target.as_ref().map(|(id, _)| id.as_str()), target.as_ref().map(|(_, kind)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionHover".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "channel": "pointer", "targets": targets.as_str() })) })
}

/// 🎯️ Geometry oracle publishes the same shared interaction command and target text.
#[cfg(any(test, feature = "testkit"))]
pub fn pick_select_action(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, interaction_state: &WorldInteractionState, scene_state: &WorldSceneState, x: f32, y: f32, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    let merge = world_merge_mode(shift, ctrl).wire_label();
    if selection_state.interaction_mode == "paint" {
        return None;
    }
    if component_mode_active(selection_state) {
        let hit = pick_component_at(session_state, camera_state, selection_state, scene_state, x, y, inner);
        let target = hit.as_ref().map(|(kind, group, object)| format!("{object}.{kind}.{group}"));
        let targets = world_interaction_targets(Some(&session_state.surface_id), target.as_deref(), hit.as_ref().map(|(kind, _, _)| kind.as_str()), session_state.bound_domain_id.is_some()).ok()?;
        return Some(ActionDescriptor {
            controller_id: session_state.controller_id.clone(), action: "interactionSelect".into(),
            args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "targets": targets.as_str(), "merge": merge, "method": "pick" })),
        });
    }
    let hit = pick_instance_at(session_state, camera_state, scene_state, x, y, inner);
    let targets = world_interaction_targets(Some(&session_state.surface_id), hit.as_deref().map(|id| instance_interaction_id(interaction_state, id)), hit.as_deref().map(|id| instance_interaction_granularity_id(session_state, interaction_state, id)), session_state.bound_domain_id.is_some()).ok()?;
    Some(ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(session_state),
            "targets": targets.as_str(),
            "merge": merge,
            "method": selection_method_wire_str(SelectionMethod::Pick),
        })),
    })
}

#[cfg(any(test, feature = "testkit"))]
pub fn merge_u32_ids(existing: &[String], incoming: &[String], merge: MergeMode) -> Vec<u32> {
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
#[cfg(any(test, feature = "testkit"))]
pub fn marquee_select_action(session_state: &mut WorldSessionState, camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, interaction_state: &mut WorldInteractionState, scene_state: &mut WorldSceneState, inner: Rect, shift: bool, ctrl: bool) -> Option<ActionDescriptor> {
    if selection_state.marquee_points.len() < 2 {
        return None;
    }
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_scene::math::projection_spec_view_proj(&camera, camera_state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(selection_state, inner);
    let (meshes, draws) = scene_geometry(scene_state);
    let ids = if component_mode_active(selection_state) {
        screen_select_components(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, selection_state.granularity.as_str(), selection_state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
    selection_state.marquee_points.clear();
    selection_state.marquee_preview_ids.clear();
    let merge = world_merge_mode(shift, ctrl);
    if component_mode_active(selection_state) {
        let object = selection_state.active_object_id.as_deref().or_else(|| scene_state.draws.iter().flat_map(|draw| &draw.instances).next().map(|instance| instance.id.as_str()))?;
        let targets: Vec<serde_json::Value> = ids.iter().map(|id| serde_json::json!({ "granularity": selection_state.granularity, "id": resolved_item_id(session_state, &format!("{object}.{}.{id}", selection_state.granularity)) })).collect();
        return Some(ActionDescriptor { controller_id: session_state.controller_id.clone(), action: "interactionSelect".into(), args: Some(semio_framework::dsl_value!({ "domainId": resolved_domain_id(session_state), "targets": serde_json::to_string(&targets).expect("test targets encode"), "merge": merge.wire_label(), "method": "rectangle" })) });
    }
    let mut seen = HashSet::new();
    let targets: Vec<serde_json::Value> = ids
        .iter()
        .filter_map(|id| {
            let granularity = instance_interaction_granularity_id(session_state, interaction_state, id);
            let target_id = instance_interaction_id(interaction_state, id);
            seen.insert((granularity.to_string(), target_id.to_string())).then(|| serde_json::json!({ "granularity": granularity, "id": resolved_item_id(session_state, target_id) }))
        })
        .collect();
    Some(ActionDescriptor {
        controller_id: session_state.controller_id.clone(),
        action: "interactionSelect".into(),
        args: Some(semio_framework::dsl_value!({
            "domainId": resolved_domain_id(session_state),
            "targets": serde_json::to_string(&targets).expect("test targets encode"),
            "merge": merge.wire_label(),
            "method": selection_method_wire_str(SelectionMethod::Rectangle),
        })),
    })
}

#[cfg(any(test, feature = "testkit"))]
fn gumball_drag_update(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, x: f32, y: f32, inner: Rect) {
    let Some(handle) = gumball_state.gumball_handle else {
        return;
    };
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, inner.w, inner.h);
    let pivot = gumball_state.gumball_pivot;
    let eye = gumball_eye(&camera, pivot);
    reset_gumball_preview(gumball_state);
    if handle.is_translate() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let delta = current - gumball_state.gumball_drag_anchor;
                gumball_state.gumball_preview_translate = axis.normalize().scale(delta);
            }
        } else if let Some(normal) = handle.plane_normal() {
            if let Some(current) = ray_plane_point(origin, dir, pivot, normal) {
                gumball_state.gumball_preview_translate = current.sub(gumball_state.gumball_drag_start_vec);
            }
        }
    } else if handle.is_rotate() {
        if let Some(normal) = handle.plane_normal() {
            if let Some(hit) = ray_plane_point(origin, dir, pivot, normal) {
                let current = hit.sub(pivot);
                if current.length() > 1e-4 && gumball_state.gumball_drag_start_vec.length() > 1e-4 {
                    gumball_state.gumball_preview_angle = axis_rotate_angle(gumball_state.gumball_drag_start_vec, current, normal);
                }
            }
        }
    } else if handle.is_scale() {
        if let Some(axis) = handle.axis_dir() {
            if let Some(current) = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye) {
                let factor = if gumball_state.gumball_drag_anchor.abs() > 1e-4 { current / gumball_state.gumball_drag_anchor } else { 1.0 };
                let factor = factor.clamp(0.05, 20.0);
                match handle {
                    GumballHandle::ScaleX => gumball_state.gumball_preview_scale.x = factor,
                    GumballHandle::ScaleY => gumball_state.gumball_preview_scale.y = factor,
                    GumballHandle::ScaleZ => gumball_state.gumball_preview_scale.z = factor,
                    _ => {}
                }
            }
        }
    }
    apply_gumball_preview(selection_state, gumball_state, scene_state);
}

#[cfg(any(test, feature = "testkit"))]
pub fn start_gumball_drag(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, gumball_state: &mut WorldGumballState, scene_state: &mut WorldSceneState, handle: GumballHandle, x: f32, y: f32, inner: Rect) {
    let Some(pivot) = selection_centroid(selection_state, gumball_state, scene_state) else {
        return;
    };
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let local_x = x - inner.x;
    let local_y = y - inner.y;
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, inner.w, inner.h);
    let eye = gumball_eye(&camera, pivot);
    gumball_state.gumball_handle = Some(handle);
    gumball_state.gumball_pivot = pivot;
    reset_gumball_preview(gumball_state);
    if let Some(axis) = handle.axis_dir() {
        gumball_state.gumball_drag_anchor = gumball_project_ray_onto_axis(origin, dir, pivot, axis, eye).unwrap_or(0.0);
    } else if let Some(normal) = handle.plane_normal() {
        gumball_state.gumball_drag_start_vec = ray_plane_point(origin, dir, pivot, normal).unwrap_or(pivot);
        if handle.is_rotate() {
            gumball_state.gumball_drag_start_vec = gumball_state.gumball_drag_start_vec.sub(pivot);
        }
        gumball_state.gumball_drag_anchor = 0.0;
    }
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_component_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, selection_state: &WorldSelectionState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, String, String)> {
    let (local_x, local_y, rect) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (rect.w / rect.h.max(1.0)).max(0.1);
    let view_proj = ui_scene::math::projection_spec_view_proj(&camera, camera_state.projection_spec, rect.w, rect.h);
    let granularity = selection_state.granularity.as_str();
    match granularity {
        "vertex" => {
            let mut best: Option<(f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
                        continue;
                    }
                    for vertex_index in 0..schema.vertices {
                        let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                        let world = instance.model.transform_point(Vec3::new(point[0], point[1], point[2]));
                        let Some(screen) = ui_scene::math::projection_spec_project_point(view_proj, camera_state.projection_spec, world, rect.w, rect.h) else {
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
            let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                let Ok(schema) = mesh.schema() else { continue };
                if schema.edges == 0 || schema.edge_ids != schema.edges {
                    continue;
                }
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
                        continue;
                    }
                    for edge_index in 0..schema.edges {
                        let Ok(edge) = mesh.edge(edge_index) else { continue };
                        let a = instance.model.transform_point(Vec3::new(edge[0][0], edge[0][1], edge[0][2]));
                        let b = instance.model.transform_point(Vec3::new(edge[1][0], edge[1][1], edge[1][2]));
                        let Some(segment) = ui_scene::math::projection_spec_project_segment(view_proj, camera_state.projection_spec, a, b, rect.w, rect.h) else { continue; };
                        let [screen_a, screen_b] = segment.screen;
                        let screen_dist = ui_scene::math::screen_segment_distance(local_x, local_y, screen_a[0], screen_a[1], screen_b[0], screen_b[1]);
                        if screen_dist > PICK_EDGE_SCREEN_PX {
                            continue;
                        }
                        let Some(closest) = ui_scene::math::ray_segment_closest(origin, dir, segment.points[0], segment.points[1]) else { continue };
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
            let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, rect.w, rect.h);
            let mut best: Option<(f32, String, String)> = None;
            for draw in &scene_state.draws {
                let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
                    continue;
                };
                for instance in &draw.instances {
                    if !pick_targets_instance(selection_state, &instance.id) {
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

#[cfg(any(test, feature = "testkit"))]
pub fn pick_paint_hit(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, f32, f32)> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, f32, f32)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
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
pub fn world_marquee_overlay_points(selection_state: &WorldSelectionState, authority_state: &WorldAuthorityState) -> Option<Vec<[f32; 2]>> {
    if let Some(marquee) = authority_state.interaction_authority.as_ref()?.marquee.as_ref() {
        if !marquee.retiring && marquee.len >= 2 {
            let mut points = Vec::with_capacity(usize::from(marquee.len));
            for index in 0..usize::from(marquee.len) {
                points.push(marquee.points[index].expect("marquee gesture point"));
            }
            return Some(points);
        }
    }
    if selection_state.marquee_active && selection_state.marquee_points.len() >= 2 {
        return Some(selection_state.marquee_points.clone());
    }
    None
}

#[cfg(any(test, feature = "testkit"))]
fn marquee_local_polygon(selection_state: &WorldSelectionState, rect: Rect) -> (Vec<[f32; 2]>, bool, bool) {
    let rectangle = selection_state.selection_method != "lasso";
    let crossing = marquee_is_crossing_from_path(&selection_state.marquee_points, !rectangle);
    let global: Vec<[f32; 2]> = if rectangle {
        let start = selection_state.marquee_points[0];
        let end = selection_state.marquee_points[selection_state.marquee_points.len() - 1];
        vec![start, end]
    } else {
        selection_state.marquee_points.clone()
    };
    let local = global.iter().map(|point| [point[0] - rect.x, point[1] - rect.y]).collect();
    (local, rectangle, crossing)
}

#[cfg(any(test, feature = "testkit"))]
pub fn update_marquee_preview(camera_state: &mut WorldCameraState, selection_state: &mut WorldSelectionState, scene_state: &mut WorldSceneState, inner: Rect) {
    if selection_state.marquee_points.len() < 2 {
        selection_state.marquee_preview_ids.clear();
        return;
    }
    let camera = camera_state.orbit.to_camera();
    let aspect = (inner.w / inner.h.max(1.0)).max(0.1);
    let view_proj = ui_scene::math::projection_spec_view_proj(&camera, camera_state.projection_spec, inner.w, inner.h);
    let (polygon, rectangle, crossing) = marquee_local_polygon(selection_state, inner);
    let (meshes, draws) = scene_geometry(scene_state);
    selection_state.marquee_preview_ids = if component_mode_active(selection_state) {
        screen_select_components(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, selection_state.granularity.as_str(), selection_state.active_object_id.as_deref(), crossing)
    } else {
        screen_select_instances(&meshes, &draws, view_proj, camera_state.projection_spec, inner.w, inner.h, &polygon, rectangle, crossing)
    };
}

#[cfg(any(test, feature = "testkit"))]
pub fn scene_geometry(scene_state: &WorldSceneState) -> (HashMap<String, Mesh3dLease>, Vec<SceneDraw3d>) {
    (scene_state.meshes.iter().map(|(id, mesh)| (id.clone(), *mesh)).collect(), scene_state.draws.iter().cloned().collect())
}

#[cfg(any(test, feature = "testkit"))]
pub fn pick_instance_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<String> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
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

#[cfg(any(test, feature = "testkit"))]
pub fn pick_surface_at(session_state: &WorldSessionState, camera_state: &WorldCameraState, scene_state: &WorldSceneState, x: f32, y: f32, _inner: Rect) -> Option<(String, [f64; 3], [f64; 3])> {
    let (local_x, local_y, viewport) = pointer_in_pick_rect(session_state, x, y)?;
    let camera = camera_state.orbit.to_camera();
    let aspect = (viewport.w / viewport.h.max(1.0)).max(0.1);
    let (origin, dir) = ui_scene::math::projection_spec_ray_from_screen(&camera, camera_state.projection_spec, local_x, local_y, viewport.w, viewport.h);
    let mut best: Option<(f32, String, Vec3, Vec3)> = None;
    for draw in &scene_state.draws {
        let Some(&mesh) = scene_state.meshes.get(&draw.mesh_key) else {
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
