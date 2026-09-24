//! @emoji 👕️ Pure peer-overlay derivation from `PresencePeer` roster + local window
//! (contract-freeze §C7.8). Ephemeral shared only — never persisted. Twin of `🟦️.ts`.

use crate::{PresencePeer, PresenceViewKind};
use std::collections::BTreeMap;

//#region 🔖️PeerOverlay

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerOverlayKind {
    Camera,
    Cursor,
    Marks,
    Caret,
    Playhead,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiPeerMark {
    pub actor: String,
    pub color: Option<u8>,
    pub hovered: bool,
    pub selected: bool,
    pub label: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeerView {
    pub actor: String,
    pub label: String,
    pub color: Option<u8>,
    pub view: PresenceViewKind,
    pub pointer: Option<[f64; 3]>,
    pub ray_origin: Option<[f64; 3]>,
    pub size: [f64; 2],
    pub window_id: String,
    pub space: String,
    pub active_tool: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PeerOverlaySpec {
    pub window_id: String,
    pub local_color: u8,
    pub artifact_peers: Vec<PeerView>,
    pub marks: BTreeMap<String, BTreeMap<String, Vec<UiPeerMark>>>,
}

pub fn peer_overlay_path(scene_path: &str, kind: PeerOverlayKind, index: usize, key: &str) -> String {
    let segment = match kind {
        PeerOverlayKind::Camera => "peerCamera",
        PeerOverlayKind::Cursor => "peerCursor",
        PeerOverlayKind::Marks => "peerMarks",
        PeerOverlayKind::Caret => "peerCaret",
        PeerOverlayKind::Playhead => "peerPlayhead",
    };
    format!("{scene_path}/{segment}[{index}]#{key}")
}

pub fn peers_for_window(roster: &[PresencePeer], window_id: &str, space: &str, _my_surface: Option<&str>, my_actor: &str, local_color: u8) -> PeerOverlaySpec {
    let mut artifact_peers = Vec::new();
    let mut marks: BTreeMap<String, BTreeMap<String, Vec<UiPeerMark>>> = BTreeMap::new();
    for peer in roster {
        if peer.actor == my_actor {
            continue;
        }
        let mut matched = false;
        for view in &peer.views {
            if view.window_id != window_id || view.space != space {
                continue;
            }
            matched = true;
            artifact_peers.push(PeerView {
                actor: peer.actor.clone(),
                label: peer.label.clone().unwrap_or_else(|| peer.actor.clone()),
                color: peer.color,
                view: view.kind.clone(),
                pointer: view.pointer,
                ray_origin: view.ray_origin,
                size: view.size,
                window_id: view.window_id.clone(),
                space: view.space.clone(),
                active_tool: peer.active_tool.clone(),
            });
        }
        if !matched {
            continue;
        }
        if let Some(interaction) = &peer.interaction {
            for domain in &interaction.domains {
                let by_id = marks.entry(domain.domain.clone()).or_default();
                for id in &domain.selected {
                    let list = by_id.entry(id.clone()).or_default();
                    if let Some(existing) = list.iter_mut().find(|mark| mark.actor == peer.actor) {
                        existing.selected = true;
                    } else {
                        list.push(UiPeerMark {
                            actor: peer.actor.clone(),
                            color: peer.color,
                            hovered: false,
                            selected: true,
                            label: peer.label.clone().unwrap_or_else(|| peer.actor.clone()),
                        });
                    }
                }
                for id in &domain.hovered {
                    let list = by_id.entry(id.clone()).or_default();
                    if let Some(existing) = list.iter_mut().find(|mark| mark.actor == peer.actor) {
                        existing.hovered = true;
                    } else {
                        list.push(UiPeerMark {
                            actor: peer.actor.clone(),
                            color: peer.color,
                            hovered: true,
                            selected: false,
                            label: peer.label.clone().unwrap_or_else(|| peer.actor.clone()),
                        });
                    }
                }
            }
        }
    }
    artifact_peers.sort_by(|a, b| a.actor.cmp(&b.actor).then(a.window_id.cmp(&b.window_id)));
    PeerOverlaySpec { window_id: window_id.to_string(), local_color, artifact_peers, marks }
}

pub fn peer_marks_for<'a>(spec: &'a PeerOverlaySpec, domain: &str, id: &str) -> &'a [UiPeerMark] {
    spec.marks.get(domain).and_then(|by_id| by_id.get(id)).map(|list| list.as_slice()).unwrap_or(&[])
}

pub fn canvas_point_to_screen(local_view: (f64, f64, f64), local_size_px: [f64; 2], world: [f64; 2]) -> [f32; 2] {
    let (x, y, zoom) = local_view;
    let zoom = if zoom == 0.0 { 1.0 } else { zoom };
    [((world[0] - x) * zoom + local_size_px[0] / 2.0) as f32, ((world[1] - y) * zoom + local_size_px[1] / 2.0) as f32]
}

pub fn canvas_peer_viewport_rect(peer_view: (f64, f64, f64), peer_size: [f64; 2], local_view: (f64, f64, f64), local_size_px: [f64; 2]) -> [f32; 4] {
    let (px, py, pzoom) = peer_view;
    let pzoom = if pzoom == 0.0 { 1.0 } else { pzoom };
    let half_w = peer_size[0] / (2.0 * pzoom);
    let half_h = peer_size[1] / (2.0 * pzoom);
    let corners = [[px - half_w, py - half_h], [px + half_w, py - half_h], [px + half_w, py + half_h], [px - half_w, py + half_h]];
    let screens: Vec<[f32; 2]> = corners.iter().map(|c| canvas_point_to_screen(local_view, local_size_px, *c)).collect();
    let min_x = screens.iter().map(|s| s[0]).fold(f32::INFINITY, f32::min);
    let min_y = screens.iter().map(|s| s[1]).fold(f32::INFINITY, f32::min);
    let max_x = screens.iter().map(|s| s[0]).fold(f32::NEG_INFINITY, f32::max);
    let max_y = screens.iter().map(|s| s[1]).fold(f32::NEG_INFINITY, f32::max);
    [min_x, min_y, max_x - min_x, max_y - min_y]
}


pub fn orbit_point_to_screen(position: [f64; 3], target: [f64; 3], up: [f64; 3], fov_deg: f64, local_size_px: [f64; 2], world: [f64; 3]) -> Option<[f32; 2]> {
    let forward = normalize([target[0] - position[0], target[1] - position[1], target[2] - position[2]]);
    let right = normalize(cross(forward, up));
    let true_up = cross(right, forward);
    let rel = [world[0] - position[0], world[1] - position[1], world[2] - position[2]];
    let cam_z = rel[0] * forward[0] + rel[1] * forward[1] + rel[2] * forward[2];
    if cam_z <= 1e-9 {
        return None;
    }
    let cam_x = rel[0] * right[0] + rel[1] * right[1] + rel[2] * right[2];
    let cam_y = rel[0] * true_up[0] + rel[1] * true_up[1] + rel[2] * true_up[2];
    let aspect = local_size_px[0] / if local_size_px[1] == 0.0 { 1.0 } else { local_size_px[1] };
    let half_v = (fov_deg.to_radians() / 2.0).tan();
    let half_h = half_v * aspect;
    let ndc_x = cam_x / (cam_z * half_h);
    let ndc_y = cam_y / (cam_z * half_v);
    Some([((ndc_x + 1.0) * 0.5 * local_size_px[0]) as f32, ((1.0 - ndc_y) * 0.5 * local_size_px[1]) as f32])
}

pub fn orbit_frustum_corners(position: [f64; 3], target: [f64; 3], up: [f64; 3], fov_deg: f64, aspect: f64, depth: f64) -> [[f32; 3]; 5] {
    let forward = normalize([target[0] - position[0], target[1] - position[1], target[2] - position[2]]);
    let right = normalize(cross(forward, up));
    let true_up = cross(right, forward);
    let half_v = (fov_deg.to_radians() / 2.0).tan() * depth;
    let half_h = half_v * aspect;
    let far = [position[0] + forward[0] * depth, position[1] + forward[1] * depth, position[2] + forward[2] * depth];
    let c = |u: f64, r: f64| -> [f32; 3] {
        [
            (far[0] + true_up[0] * u + right[0] * r) as f32,
            (far[1] + true_up[1] * u + right[1] * r) as f32,
            (far[2] + true_up[2] * u + right[2] * r) as f32,
        ]
    };
    [[position[0] as f32, position[1] as f32, position[2] as f32], c(half_v, -half_h), c(half_v, half_h), c(-half_v, half_h), c(-half_v, -half_h)]
}

pub fn orbit_frustum_segments(corners: [[f32; 3]; 5]) -> [([f32; 3], [f32; 3]); 8] {
    let [apex, a, b, c, d] = corners;
    [(apex, a), (apex, b), (apex, c), (apex, d), (a, b), (b, c), (c, d), (d, a)]
}

fn cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

fn normalize(v: [f64; 3]) -> [f64; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    let len = if len == 0.0 { 1.0 } else { len };
    [v[0] / len, v[1] / len, v[2] / len]
}

//#endregion 🔖️PeerOverlay

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
