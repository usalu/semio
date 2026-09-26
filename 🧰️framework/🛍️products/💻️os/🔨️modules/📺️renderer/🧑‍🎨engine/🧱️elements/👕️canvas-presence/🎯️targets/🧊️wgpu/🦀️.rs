//! 👕️ wgpu twin of the canvas-presence element (`👕️canvas-presence/🟦️.ts` + `🟦️.tsx`): what one board
//! window publishes about its camera and pointer on the presence heartbeat (React's `Board2dHost`
//! `publishLocalPresenceWindowViewV1`), and the peer overlays it paints from the verified roster (React's
//! `CanvasPresenceOverlayV1`). Both shells speak the one presence wire (`protocol::PresencePeer`), and both
//! pass the schema-first fixture `🧫️fixtures/👕️canvas-presence` (`🧬️schema/👕️canvas-presence`).

use replication::{canvas_peer_viewport_rect, canvas_point_to_screen, peer_overlay_path, peers_for_window, PeerOverlayKind, PresencePeer, PresenceViewKind, PresenceWindowView};
use ui_wgpu::wgpu::{Locale, Rect};

/// 🪟️ The presence space every board window publishes and paints in — React's `space: "canvas"`.
pub const CANVAS_PRESENCE_SPACE: &str = "canvas";

/// 📐️ Inverse of the canonical board transform `screen = (world - camera) * zoom + size / 2` — React's
/// `puzzle2dScreenToWorld`, the inverse of `protocol::canvas_point_to_screen`.
pub fn canvas_screen_to_point(camera: (f64, f64, f64), size_px: [f64; 2], screen: [f64; 2]) -> [f64; 2] {
    let (x, y, zoom) = camera;
    let zoom = if zoom == 0.0 { 1.0 } else { zoom };
    [x + (screen[0] - size_px[0] / 2.0) / zoom, y + (screen[1] - size_px[1] / 2.0) / zoom]
}

/// 📡️ One board window's presence view: its camera, its size and — while the pointer is over the board —
/// the world point under it. `pointer` is in the shell's logical screen space, the space of `bounds`.
pub fn board_presence_view(window_id: &str, bounds: Rect, camera: (f64, f64, f64), pointer: Option<(f32, f32)>) -> PresenceWindowView {
    let size = [f64::from(bounds.w), f64::from(bounds.h)];
    let pointer = pointer.filter(|(x, y)| bounds.contains(*x, *y)).map(|(x, y)| {
        let world = canvas_screen_to_point(camera, size, [f64::from(x - bounds.x), f64::from(y - bounds.y)]);
        [world[0], world[1], 0.0]
    });
    PresenceWindowView { window_id: window_id.to_string(), space: CANVAS_PRESENCE_SPACE.to_string(), kind: PresenceViewKind::Canvas { x: camera.0, y: camera.1, zoom: camera.2 }, size, pointer, ray_origin: None }
}

/// 🖱️ One peer's cursor on a board window, in the shell's screen space: the dot, the peer's viewport
/// rectangle, the name chip and the two `data-ui-path` twins React stamps on the same nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardPeerCursor {
    pub actor: String,
    pub label: String,
    pub color: u8,
    pub at: [f32; 2],
    pub viewport: [f32; 4],
    pub chip: String,
    pub cursor_path: String,
    pub viewport_path: String,
}

/// 🏷️ One peer's selection or hover of one entity, painted as a corner chip of the peer's initials, the way
/// React's canvas overlay paints marks it cannot place in the world.
#[derive(Clone, Debug, PartialEq)]
pub struct BoardPeerMark {
    pub actor: String,
    pub label: String,
    pub color: u8,
    pub domain: String,
    pub id: String,
    pub selected: bool,
    pub chip: String,
    pub path: String,
    pub row: usize,
}

/// 👥️ Everything one board window paints for the roster: every other actor's cursor on this very window,
/// and every mark of an actor looking at this window, in React's order (`peersForWindow`: artifact peers
/// by actor, marks by domain and id).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BoardPeerOverlays {
    pub cursors: Vec<BoardPeerCursor>,
    pub marks: Vec<BoardPeerMark>,
}

/// 👥️ Derives one board window's peer overlays from the verified roster: `camera` is this window's own
/// camera, `bounds` its screen rectangle, `my_actor` the hub-admitted local actor (never painted).
pub fn board_peer_overlays(roster: &[PresencePeer], window_id: &str, bounds: Rect, camera: (f64, f64, f64), my_actor: &str, local_color: u8, scene_path: &str) -> BoardPeerOverlays {
    let spec = peers_for_window(roster, window_id, CANVAS_PRESENCE_SPACE, None, my_actor, local_color);
    let size = [f64::from(bounds.w), f64::from(bounds.h)];
    let cursors = spec
        .artifact_peers
        .iter()
        .enumerate()
        .filter_map(|(index, peer)| {
            let &PresenceViewKind::Canvas { x, y, zoom } = &peer.view else { return None };
            let pointer = peer.pointer?;
            let at = canvas_point_to_screen(camera, size, [pointer[0], pointer[1]]);
            let rect = canvas_peer_viewport_rect((x, y, zoom), peer.size, camera, size);
            Some(BoardPeerCursor {
                actor: peer.actor.clone(),
                label: peer.label.clone(),
                color: peer.color.unwrap_or(0),
                at: [at[0] + bounds.x, at[1] + bounds.y],
                viewport: [rect[0] + bounds.x, rect[1] + bounds.y, rect[2], rect[3]],
                chip: peer.active_tool.as_deref().map_or_else(|| peer.label.clone(), |tool| peer_tool_chip(&peer.label, tool)),
                cursor_path: peer_overlay_path(scene_path, PeerOverlayKind::Cursor, index, &peer.actor),
                viewport_path: peer_overlay_path(scene_path, PeerOverlayKind::Camera, index, &peer.actor),
            })
        })
        .collect();
    let marks = spec
        .marks
        .iter()
        .flat_map(|(domain, by_id)| {
            by_id.iter().enumerate().flat_map(move |(row, (id, marks))| {
                marks.iter().map(move |mark| BoardPeerMark {
                    actor: mark.actor.clone(),
                    label: mark.label.clone(),
                    color: mark.color.unwrap_or(0),
                    domain: domain.clone(),
                    id: id.clone(),
                    selected: mark.selected,
                    chip: mark.label.chars().take(2).collect::<String>().to_uppercase(),
                    path: peer_overlay_path(scene_path, PeerOverlayKind::Marks, row, &format!("{domain}:{id}")),
                    row,
                })
            })
        })
        .collect();
    BoardPeerOverlays { cursors, marks }
}

/// 🏷️ The name chip of a peer with an active tool — React's `PEER_OVERLAY_LABELS.*.tool`, the same in every locale.
pub fn peer_tool_chip(name: &str, tool: &str) -> String {
    format!("{name}: {tool}")
}

/// 🔤️ Which overlay a peer label names — React's `PEER_OVERLAY_LABELS` keys.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PeerOverlayLabel {
    Cursor,
    Viewport,
    Selection,
    Hover,
}

/// 🔤️ The accessible name of one peer overlay, en + de, term for term with React's `PEER_OVERLAY_LABELS`.
pub fn peer_overlay_label(label: PeerOverlayLabel, name: &str, locale: Locale) -> String {
    match (label, locale) {
        (PeerOverlayLabel::Cursor, Locale::De) => format!("Cursor von {name}"),
        (PeerOverlayLabel::Viewport, Locale::De) => format!("Ansicht von {name}"),
        (PeerOverlayLabel::Selection, Locale::De) => format!("Auswahl von {name}"),
        (PeerOverlayLabel::Hover, Locale::De) => format!("Hover von {name}"),
        (PeerOverlayLabel::Cursor, _) => format!("{name}'s cursor"),
        (PeerOverlayLabel::Viewport, _) => format!("{name}'s viewport"),
        (PeerOverlayLabel::Selection, _) => format!("{name}'s selection"),
        (PeerOverlayLabel::Hover, _) => format!("{name}'s hover"),
    }
}

#[cfg(test)]
#[path = "../../🧪️tests/🔬️wgpu-unit/🦀️.rs"]
pub(crate) mod tests;
