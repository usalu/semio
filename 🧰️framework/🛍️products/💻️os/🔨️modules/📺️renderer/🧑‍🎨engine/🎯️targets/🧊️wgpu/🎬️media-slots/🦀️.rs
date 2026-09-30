//! 🎬️ Bounded media descriptors from GPU-accepted retained geometry.

use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::{Rect, UiNode};

pub const PRESENTED_MEDIA_SLOT_CAPACITY: usize = 32;
pub const PRESENTED_MEDIA_DESCRIPTOR_BYTES: usize = 65_536;

fn serialize_coordinate<S: serde::Serializer>(value: &f32, serializer: S) -> Result<S::Ok, S::Error> {
    if replication::value::json_integer(f64::from(*value)).is_some() {
        ui_contract::UiValue::Number(f64::from(*value)).serialize(serializer)
    } else {
        serializer.serialize_f32(*value)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresentedMediaRect {
    #[serde(serialize_with = "serialize_coordinate")]
    pub x: f32,
    #[serde(serialize_with = "serialize_coordinate")]
    pub y: f32,
    #[serde(serialize_with = "serialize_coordinate")]
    pub width: f32,
    #[serde(serialize_with = "serialize_coordinate")]
    pub height: f32,
}

impl From<Rect> for PresentedMediaRect {
    fn from(rect: Rect) -> Self {
        Self { x: rect.x, y: rect.y, width: rect.w, height: rect.h }
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PresentedMediaSlot {
    pub token: String,
    pub window_id: String,
    pub node_id: String,
    pub node_key: String,
    pub rect: PresentedMediaRect,
    pub clip: PresentedMediaRect,
    pub paint_order: u32,
    pub occluded: bool,
    pub plugin_id: String,
    pub controller_id: String,
    pub app_instance_id: u32,
    pub parent_document_id: String,
    pub props: serde_json::Value,
}

#[derive(Clone, Debug)]
pub(crate) struct PresentedMediaOwner {
    pub plugin_id: String,
    pub controller_id: String,
    pub app_instance_id: u32,
    pub parent_document_id: String,
}


#[derive(Default)]
pub(crate) struct DocumentIdentityRegistry {
    next_request: u64,
    states: std::collections::HashMap<(String, u32), (u64, Option<String>)>,
}

impl DocumentIdentityRegistry {
    pub(crate) fn retain(&mut self, mut keep: impl FnMut(&(String, u32)) -> bool) {
        self.states.retain(|key, _| keep(key));
    }

    pub(crate) fn remove(&mut self, key: &(String, u32)) {
        self.states.remove(key);
    }

    pub(crate) fn begin(&mut self, key: (String, u32)) -> Option<u64> {
        if !self.states.contains_key(&key) && self.states.len() == 128 { return None; }
        let request = self.next_request.checked_add(1)?;
        self.next_request = request;
        self.states.insert(key, (request, None));
        Some(request)
    }

    pub(crate) fn accept(&mut self, key: &(String, u32), request: u64, instance_id: u32, document_id: Option<String>) -> bool {
        if key.1 != instance_id || document_id.as_ref().is_some_and(|id| !(1..=512).contains(&id.chars().count())) { return false; }
        let Some(state) = self.states.get_mut(key) else { return false };
        if state.0 != request { return false; }
        state.1 = document_id;
        true
    }

    pub(crate) fn document(&self, key: &(String, u32)) -> Option<&str> {
        self.states.get(key)?.1.as_deref()
    }
}

fn intersect(left: Rect, right: Rect) -> Rect {
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    Rect::new(x, y, (left.x + left.w).min(right.x + right.w).max(x) - x, (left.y + left.h).min(right.y + right.h).max(y) - y)
}

fn valid_bounds(rect: Rect) -> bool {
    [rect.x, rect.y, rect.w, rect.h].iter().all(|value| value.is_finite()) && rect.w >= 0.0 && rect.h >= 0.0
}

fn valid_rect(rect: Rect) -> bool {
    [rect.x, rect.y, rect.w, rect.h].iter().all(|value| value.is_finite()) && rect.w > 0.0 && rect.h > 0.0
}

pub(crate) fn rects_overlap(left: Rect, right: Rect) -> bool {
    left.x < right.x + right.w && left.x + left.w > right.x && left.y < right.y + right.h && left.y + left.h > right.y
}

pub(crate) fn body_occluder(body: Rect, frames: impl Iterator<Item = Rect>) -> Rect {
    frames.filter(|frame| body.x >= frame.x && body.y >= frame.y && body.x + body.w <= frame.x + frame.w && body.y + body.h <= frame.y + frame.h)
        .min_by(|left, right| (left.w * left.h).total_cmp(&(right.w * right.h))).unwrap_or(body)
}

/// 🪪️ Refuses any resource whose authority differs from the accepted shell owner.
pub(crate) fn resource_matches_owner(props: &serde_json::Value, owner: &PresentedMediaOwner) -> bool {
    props.get("resource").is_some_and(|resource| {
        resource.is_null()
            || (resource.get("controllerId").and_then(serde_json::Value::as_str) == Some(owner.controller_id.as_str())
                && resource.get("appInstanceId").and_then(serde_json::Value::as_u64) == Some(owner.app_instance_id as u64)
                && resource.get("parentDocumentId").and_then(serde_json::Value::as_str) == Some(owner.parent_document_id.as_str()))
    })
}

/// 📐️ Walks one strictly presented tree and composes its leaves with the accepted shell body.
pub(crate) fn collect_tree_slots(tree: &ui_wgpu::wgpu::tree::UiTree, window_id: &str, body: Rect, owner: &PresentedMediaOwner, slots: &mut Vec<PresentedMediaSlot>) -> bool {
    let Some(root) = tree.root else { return true };
    if !valid_bounds(body) || !(1..=512).contains(&window_id.chars().count()) || !(1..=256).contains(&owner.plugin_id.chars().count()) || !(1..=256).contains(&owner.controller_id.chars().count()) || !(1..=512).contains(&owner.parent_document_id.chars().count()) {
        return true;
    }
    let first = slots.len();
    let mut stack = vec![(root, body.x, body.y, body, 0usize, false)];
    let mut visited = 0usize;
    while let Some((id, origin_x, origin_y, inherited_clip, depth, ancestor_occluded)) = stack.pop() {
        visited += 1;
        if visited > ui_contract::UI_DOCUMENT_NODES || depth >= 64 {
            return false;
        }
        let Some(node) = tree.node(id) else { continue };
        let Some((x, y, width, height)) = tree.mounted_layout(id) else { continue };
        let occluded = ancestor_occluded || !node.spec.0.presence().visible() || tree.overlay_walk_origin(id).is_some();
        let rect = Rect::new(origin_x + x, origin_y + y, width, height);
        let painted_clip = intersect(rect, inherited_clip);
        if valid_rect(painted_clip) {
            let mut index = first;
            while index < slots.len() {
                let clip = &slots[index].clip;
                if !occluded && rects_overlap(Rect::new(clip.x, clip.y, clip.width, clip.height), painted_clip) { slots[index].occluded = true; }
                index += 1;
            }
        }
        if let Some(document_id) = tree.document_id(id) {
            if let UiNode::ExternalSlot(slot) = &node.spec.0 {
                if slot.body_key == ui_wgpu::wgpu::reconcile::MEDIA_TRANSPORT_EXTENSION_ID && slot.params_json.len() <= PRESENTED_MEDIA_DESCRIPTOR_BYTES && slot.app_id == owner.controller_id {
                    if let Ok(props) = serde_json::from_str::<serde_json::Value>(&slot.params_json) {
                        let clip = intersect(rect, inherited_clip);
                        if valid_bounds(rect) && valid_bounds(clip) && ui_wgpu::wgpu::reconcile::media_transport_contract_valid(&props) && resource_matches_owner(&props, owner) {
                            if slots.len() == PRESENTED_MEDIA_SLOT_CAPACITY {
                                return false;
                            }
                            let node_key = tree.document().and_then(|document| document.record(document_id)).map(|record| record.key.as_str().to_string()).unwrap_or_default();
                            if !(1..=512).contains(&node_key.chars().count()) {
                                continue;
                            }
                            let resource_identity = props.get("resource").filter(|resource| !resource.is_null()).map(|resource| ["kind", "controllerId", "appInstanceId", "parentDocumentId", "outputPort", "revision", "generation"].map(|field| resource.get(field)));
                            let identity = (window_id, document_id.0.to_string(), &node_key, &owner.plugin_id, &owner.controller_id, owner.app_instance_id, &owner.parent_document_id, resource_identity);
                            let token = semio_framework_hash::sha256_hex(&serde_json::to_vec(&identity).expect("media identity serialization"));
                            slots.push(PresentedMediaSlot { token, window_id: window_id.to_string(), node_id: document_id.0.to_string(), node_key, rect: rect.into(), clip: clip.into(), paint_order: slots.len() as u32, occluded: occluded || !valid_rect(rect) || !valid_rect(clip), plugin_id: owner.plugin_id.clone(), controller_id: owner.controller_id.clone(), app_instance_id: owner.app_instance_id, parent_document_id: owner.parent_document_id.clone(), props });
                        }
                    }
                }
            }
        }
        {
            let children = tree.children(id).collect::<Vec<_>>();
            if stack.len().saturating_add(children.len()) > ui_contract::UI_DOCUMENT_NODES {
                return false;
            }
            let clip = if node.flags.contains(ui_wgpu::wgpu::tree::NodeFlags::CLIPS_CHILDREN) { intersect(inherited_clip, rect) } else { inherited_clip };
            let Some(origin) = tree.child_walk_origin(id, (origin_x, origin_y)) else { return false };
            stack.extend(children.into_iter().rev().map(|child| (child, origin.0, origin.1, clip, depth + 1, occluded || tree.disclosure_open(id) == Some(false))));
        }
    }
    serde_json::to_vec(slots).is_ok_and(|bytes| bytes.len() <= PRESENTED_MEDIA_DESCRIPTOR_BYTES)
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️wgpu-media-slots-unit/🦀️.rs"]
mod tests;
