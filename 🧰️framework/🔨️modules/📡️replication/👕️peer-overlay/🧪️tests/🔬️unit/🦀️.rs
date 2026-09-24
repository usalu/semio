//! 🧪️ Peer-overlay derivation against `🧫️fixtures/👕️peer-overlay-v1/🔣️.json`.

use super::*;
use crate::{PresenceDomain, PresenceInteraction, PresencePeer, PresenceViewKind, PresenceWindowView};

fn load_fixture() -> serde_json::Value {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/👕️peer-overlay-v1/🔣️.json");
    let text = std::fs::read_to_string(path).expect("peer-overlay fixture");
    serde_json::from_str(&text).expect("peer-overlay fixture json")
}

fn peer_from_json(value: &serde_json::Value) -> PresencePeer {
    let views = value["views"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .map(|view| {
            let kind = &view["kind"];
            let kind = match kind["kind"].as_str().unwrap() {
                "canvas" => PresenceViewKind::Canvas { x: kind["x"].as_f64().unwrap(), y: kind["y"].as_f64().unwrap(), zoom: kind["zoom"].as_f64().unwrap() },
                "orbit" => PresenceViewKind::Orbit {
                    position: json_triple(&kind["position"]),
                    target: json_triple(&kind["target"]),
                    up: json_triple(&kind["up"]),
                    fov: kind["fov"].as_f64().unwrap(),
                },
                "geo" => PresenceViewKind::Geo {
                    lng: kind["lng"].as_f64().unwrap(),
                    lat: kind["lat"].as_f64().unwrap(),
                    zoom: kind["zoom"].as_f64().unwrap(),
                    bearing: kind["bearing"].as_f64().unwrap(),
                    pitch: kind["pitch"].as_f64().unwrap(),
                },
                other => panic!("unknown kind {other}"),
            };
            let size = json_pair(&view["size"]);
            let pointer = view.get("pointer").and_then(|p| if p.is_null() { None } else { Some(json_triple(p)) });
            let ray_origin = view.get("rayOrigin").and_then(|p| if p.is_null() { None } else { Some(json_triple(p)) });
            PresenceWindowView { window_id: view["windowId"].as_str().unwrap().into(), space: view["space"].as_str().unwrap().into(), kind, size, pointer, ray_origin }
        })
        .collect();
    let interaction = value.get("interaction").map(|interaction| PresenceInteraction {
        app_id: interaction["appId"].as_str().unwrap().into(),
        domains: interaction["domains"]
            .as_array()
            .unwrap()
            .iter()
            .map(|domain| PresenceDomain {
                domain: domain["domain"].as_str().unwrap().into(),
                granularity: domain["granularity"].as_str().unwrap().into(),
                selected: domain["selected"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().into()).collect(),
                hovered: domain["hovered"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().into()).collect(),
            })
            .collect(),
    });
    PresencePeer {
        actor: value["actor"].as_str().unwrap().into(),
        connected_at_ms: value["connectedAtMs"].as_i64().unwrap(),
        label: value.get("label").and_then(|v| v.as_str()).map(str::to_string),
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction,
        color: value.get("color").and_then(|v| v.as_u64()).map(|n| n as u8),
        surface: value.get("surface").and_then(|v| v.as_str()).map(str::to_string),
        views,
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: value.get("activeTool").and_then(|v| v.as_str()).map(str::to_string),
    }
}

fn json_triple(value: &serde_json::Value) -> [f64; 3] {
    let arr = value.as_array().unwrap();
    [arr[0].as_f64().unwrap(), arr[1].as_f64().unwrap(), arr[2].as_f64().unwrap()]
}

fn json_pair(value: &serde_json::Value) -> [f64; 2] {
    let arr = value.as_array().unwrap();
    [arr[0].as_f64().unwrap(), arr[1].as_f64().unwrap()]
}

#[test]
fn peer_overlay_v1_fixture() {
    let fixture = load_fixture();
    for row in fixture["cases"].as_array().unwrap() {
        let roster: Vec<PresencePeer> = row["roster"].as_array().unwrap().iter().map(peer_from_json).collect();
        let spec = peers_for_window(
            &roster,
            row["windowId"].as_str().unwrap(),
            row["space"].as_str().unwrap(),
            row["mySurface"].as_str(),
            row["myActor"].as_str().unwrap(),
            row["localColor"].as_u64().unwrap() as u8,
        );
        let expected_actors: Vec<&str> = row["expected"]["artifactPeerActors"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let got: Vec<&str> = spec.artifact_peers.iter().map(|p| p.actor.as_str()).collect();
        assert_eq!(got, expected_actors, "{}", row["id"]);
        if let Some(cursor) = row["expected"].get("cursorScreen") {
            let peer = &spec.artifact_peers[0];
            let local = &row["localView"];
            let local_size = json_pair(&row["localSize"]);
            let pointer = peer.pointer.unwrap();
            let scene = row["scenePath"].as_str().unwrap();
            match &peer.view {
                PresenceViewKind::Canvas { .. } => {
                    let local_view = (local["x"].as_f64().unwrap(), local["y"].as_f64().unwrap(), local["zoom"].as_f64().unwrap());
                    let screen = canvas_point_to_screen(local_view, local_size, [pointer[0], pointer[1]]);
                    assert!((screen[0] as f64 - cursor[0].as_f64().unwrap()).abs() < 1e-6);
                    assert!((screen[1] as f64 - cursor[1].as_f64().unwrap()).abs() < 1e-6);
                    let PresenceViewKind::Canvas { x, y, zoom } = peer.view else { unreachable!() };
                    let rect = canvas_peer_viewport_rect((x, y, zoom), peer.size, local_view, local_size);
                    let expected = row["expected"]["viewportRect"].as_array().unwrap();
                    for i in 0..4 {
                        assert!((rect[i] as f64 - expected[i].as_f64().unwrap()).abs() < 1e-3, "rect[{i}]");
                    }
                    assert_eq!(peer_overlay_path(scene, PeerOverlayKind::Cursor, 0, &peer.actor), row["expected"]["paths"]["cursor"].as_str().unwrap());
                    assert_eq!(peer_overlay_path(scene, PeerOverlayKind::Camera, 0, &peer.actor), row["expected"]["paths"]["viewport"].as_str().unwrap());
                }
                PresenceViewKind::Orbit { .. } => {
                    let expected_ray = row["expected"]["rayOrigin"].as_array().unwrap();
                    let ray = peer.ray_origin.expect("orbit ray origin");
                    for i in 0..3 {
                        assert!((ray[i] - expected_ray[i].as_f64().unwrap()).abs() < 1e-9);
                    }
                    let screen = orbit_point_to_screen(
                        json_triple(&local["position"]),
                        json_triple(&local["target"]),
                        json_triple(&local["up"]),
                        local["fov"].as_f64().unwrap(),
                        local_size,
                        pointer,
                    )
                    .expect("orbit point projects");
                    assert!((screen[0] as f64 - cursor[0].as_f64().unwrap()).abs() < 1e-5);
                    assert!((screen[1] as f64 - cursor[1].as_f64().unwrap()).abs() < 1e-5);
                    assert_eq!(peer_overlay_path(scene, PeerOverlayKind::Cursor, 0, &peer.actor), row["expected"]["paths"]["cursor"].as_str().unwrap());
                }
                _ => panic!("unexpected view kind for cursorScreen"),
            }
        }
        if let Some(mark_keys) = row["expected"].get("markKeys") {
            let mut keys: Vec<String> = spec.marks.iter().flat_map(|(domain, by_id)| by_id.keys().map(move |id| format!("{domain}:{id}"))).collect();
            keys.sort();
            let mut expected: Vec<String> = mark_keys.as_array().unwrap().iter().map(|v| v.as_str().unwrap().to_string()).collect();
            expected.sort();
            assert_eq!(keys, expected, "{}", row["id"]);
        }
        if let Some(tools) = row["expected"].get("activeTools") {
            let got: Vec<Option<&str>> = spec.artifact_peers.iter().map(|p| p.active_tool.as_deref()).collect();
            let expected: Vec<Option<&str>> = tools.as_array().unwrap().iter().map(|v| v.as_str()).collect();
            assert_eq!(got, expected, "{}", row["id"]);
        }
        if let Some(count) = row["expected"].get("frustumCornerCount") {
            let peer = &spec.artifact_peers[0];
            let PresenceViewKind::Orbit { position, target, up, fov } = peer.view else { panic!("expected orbit") };
            let corners = orbit_frustum_corners(position, target, up, fov, peer.size[0] / peer.size[1], 4.0);
            assert_eq!(corners.len(), count.as_u64().unwrap() as usize);
            assert_eq!(orbit_frustum_segments(corners).len(), row["expected"]["frustumSegmentCount"].as_u64().unwrap() as usize);
        }
    }
}
