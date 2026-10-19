//! 📌️ Interaction dispatch: the payload digest, the interaction dispatch, the target prefixes, the geometry census and the interaction action writers. See [`super`].

use super::*;

//#region NodeGraph
//#region 📌️GraphInteractionChange
// 📮️ WHAT A POINTER MOVE IS ALLOWED TO COST.
//
// Every pointer move over a node graph published `interactionSelect` + `interactionHover` +
// `nodeGraphViewport` whether or not any of the three had changed. The bounded action queue holds
// `ACTION_QUEUE_ITEM_CAPACITY` = 256 items and is drained one step per host turn, so a hover sweep
// denser than the frame loop filled it: `BoundedActionFault::ItemCredits` at admission, after which
// the session published NOTHING for the rest of its life — measured on 6118 as one
// `graph move fault … fault=ItemCredits` followed by 0 of 440 sweep points and 0 of 6 presses ever
// reaching the host (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-gestures-2026-09-13.md`).
//
// A plain move is not a mutation and must not be priced like one. The surface remembers the
// fingerprint of each action it last dispatched, a gesture publishes only the ones whose payload
// actually changed, and it reserves exactly that many credits — zero of them when nothing changed,
// in which case no reservation is taken at all and the plan is simply committed. React reaches the
// same end structurally: `emitIntent` answers `undefined` for an unchanged intent, and
// `NodeGraphHost`'s hover effect is keyed on the hovered id rather than on the pointer.

/// 🔢️ FNV-1a over a payload's own bytes — the change witness is the dispatched payload itself,
/// never a hand-maintained shadow of the fields that feed it.
pub(super) fn graph_payload_digest(parts: &[&str]) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325_u64;
    for part in parts {
        for byte in part.as_bytes().iter().chain(std::iter::once(&0xff)) {
            digest ^= u64::from(*byte);
            digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
        }
    }
    digest
}

/// 📮️ One gesture's interaction payloads, with the three flags that say which of them this surface
/// has not already told the guest.
pub struct GraphInteractionDispatch {
    pub(in super::super) domain_id: Option<String>,
    select_targets: String,
    hover_targets: String,
    pub(in super::super) viewport: semio_framework_os_kernel::Viewport2d,
    pub(super) digest: PublishedGraphInteraction,
    pub publish_select: bool,
    pub publish_hover: bool,
    publish_viewport: bool,
}

impl GraphInteractionDispatch {
    pub fn item_count(&self) -> usize {
        usize::from(self.publish_select) + usize::from(self.publish_hover) + usize::from(self.publish_viewport)
    }
}

pub(super) fn node_graph_interaction_domain(surface_id: &str) -> Option<Arc<semio_framework_ui_declaration::ui::NodeGraphInteractionDomain>> {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).and_then(|entry| entry.sync_cache.interaction_domain.clone()))
}

fn node_graph_target_prefix<'a>(domain: &'a semio_framework_ui_declaration::ui::NodeGraphInteractionDomain, granularity: &str) -> Option<&'a str> {
    match granularity {
        "node" => Some(&domain.node_target_prefix),
        "edge" => Some(&domain.edge_target_prefix),
        "handle" => Some(&domain.handle_target_prefix),
        _ => None,
    }
}

pub fn node_graph_domain_targets_json(domain: &semio_framework_ui_declaration::ui::NodeGraphInteractionDomain, granularity: &str, ids: &[String]) -> Result<String, semio_framework_ui_action::BoundedActionFault> {
    let prefix = node_graph_target_prefix(domain, granularity).ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
    serde_json::to_string(&ids.iter().map(|id| json!({ "granularity": granularity, "id": format!("{prefix}{id}") })).collect::<Vec<_>>()).map_err(|_| semio_framework_ui_action::BoundedActionFault::Structure)
}

pub fn graph_interaction_dispatch(published: PublishedGraphInteraction, snapshot: GraphInteractionSnapshot, domain: Option<&semio_framework_ui_declaration::ui::NodeGraphInteractionDomain>) -> Result<GraphInteractionDispatch, semio_framework_ui_action::BoundedActionFault> {
    let select_targets = match domain {
        Some(domain) => node_graph_domain_targets_json(domain, "node", &snapshot.node_ids)?,
        None => "[]".into(),
    };
    let hover_targets = match (domain, snapshot.hovered_handle.as_ref(), snapshot.hovered_id.as_ref()) {
        (Some(domain), Some(handle), _) => node_graph_domain_targets_json(domain, "handle", std::slice::from_ref(handle))?,
        (Some(domain), None, Some(node)) => node_graph_domain_targets_json(domain, "node", std::slice::from_ref(node))?,
        _ => "[]".into(),
    };
    let viewport_text = format!("{}|{}|{}", snapshot.viewport.x, snapshot.viewport.y, snapshot.viewport.zoom);
    let domain_id = domain.map(|domain| domain.id.clone());
    let digest = PublishedGraphInteraction {
        select: graph_payload_digest(&[domain_id.as_deref().unwrap_or_default(), select_targets.as_str()]),
        hover: graph_payload_digest(&[domain_id.as_deref().unwrap_or_default(), hover_targets.as_str()]),
        viewport: graph_payload_digest(&[viewport_text.as_str()]),
    };
    Ok(GraphInteractionDispatch {
        domain_id,
        select_targets,
        hover_targets,
        viewport: snapshot.viewport,
        digest,
        publish_select: domain.is_some() && digest.select != published.select,
        publish_hover: domain.is_some() && digest.hover != published.hover,
        publish_viewport: digest.viewport != published.viewport,
    })
}

/// 🗺️ One line naming where every node and port of a graph surface IS, emitted once per CHANGE of
/// that geometry rather than once per frame — the census a browser probe needs so that finding the
/// graph costs nothing, instead of costing the host's whole pointer budget in a hover sweep.
pub fn log_graph_geometry_census(surface_id: &str) {
    ENGINE_SURFACES.with(|cell| {
        let mut map = cell.borrow_mut();
        let Some(entry) = map.get_mut(surface_id) else { return };
        let Some(engine) = entry.node_graph.as_ref() else { return };
        let census = match engine {
            NodeGraphEngine::Flow(host) => screen_geometry_census_json(&host.dag.viewport, &host.dag.graph, &host.dag.minimap),
            NodeGraphEngine::Dag(host) => screen_geometry_census_json(&host.dag.viewport, &host.dag.graph, &host.dag.minimap),
        };
        let digest = graph_payload_digest(&[census.as_str()]);
        if entry.published_graph_geometry == digest {
            return;
        }
        entry.published_graph_geometry = digest;
        engine_canvas_debug_log(&format!("[TRACE] wgpu node-graph geometry surface={surface_id} entities={census}"));
    });
}

pub(super) fn record_graph_interaction(surface_id: &str, digest: PublishedGraphInteraction) {
    ENGINE_SURFACES.with(|cell| {
        if let Some(entry) = cell.borrow_mut().get_mut(surface_id) {
            entry.published_graph_interaction = digest;
        }
    });
}
//#endregion 📌️GraphInteractionChange

pub(super) fn write_graph_interaction_actions(batch: &mut semio_framework_ui_action::BoundedActionBatchReservation<'_>, surface_id: &str, controller_id: &str, dispatch: &GraphInteractionDispatch) -> Result<(), semio_framework_ui_action::BoundedActionFault> {
    if dispatch.publish_select {
        let domain_id = dispatch.domain_id.as_deref().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let select_action = "interactionSelect";
        let select_targets = dispatch.select_targets.as_str();
        let select_bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, select_action, "domainId", domain_id, "targets", select_targets, "merge", "replace", "method", "pick"])?;
        batch.action(controller_id, select_action, select_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain_id)?;
            builder.string(Some("targets"), select_targets)?;
            builder.string(Some("merge"), "replace")?;
            builder.string(Some("method"), "pick")?;
            builder.end_container()
        })?;
    }
    if dispatch.publish_hover {
        let domain_id = dispatch.domain_id.as_deref().ok_or(semio_framework_ui_action::BoundedActionFault::Structure)?;
        let hover_action = "interactionHover";
        let hover_targets = dispatch.hover_targets.as_str();
        let hover_bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, hover_action, "domainId", domain_id, "channel", "pointer", "targets", hover_targets])?;
        batch.action(controller_id, hover_action, hover_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("domainId"), domain_id)?;
            builder.string(Some("channel"), "pointer")?;
            builder.string(Some("targets"), hover_targets)?;
            builder.end_container()
        })?;
    }
    if dispatch.publish_viewport {
        let viewport_action = "nodeGraphViewport";
        let viewport = dispatch.viewport;
        let viewport_bytes = semio_framework_ui_action::checked_action_string_bytes(&[controller_id, viewport_action, "surfaceId", surface_id, "viewport", "x", "y", "zoom"])?;
        batch.action(controller_id, viewport_action, viewport_bytes, |builder| {
            builder.begin_object(None)?;
            builder.string(Some("surfaceId"), surface_id)?;
            builder.begin_object(Some("viewport"))?;
            builder.number(Some("x"), viewport.x)?;
            builder.number(Some("y"), viewport.y)?;
            builder.number(Some("zoom"), viewport.zoom)?;
            builder.end_container()?;
            builder.end_container()
        })?;
    }
    Ok(())
}

pub(super) fn world_to_screen_inner(inner: Rect, cam_x: f64, cam_y: f64, zoom: f64, wx: f64, wy: f64) -> (f32, f32) {
    let zoom = zoom.max(0.05) as f32;
    let cx = inner.w * 0.5;
    let cy = inner.h * 0.5;
    let sx = inner.x + (wx - cam_x) as f32 * zoom + cx;
    let sy = inner.y + (wy - cam_y) as f32 * zoom + cy;
    (sx, sy)
}
//#endregion NodeGraph
