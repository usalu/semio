//! 🌉️ Puzzle 5d engine — the semio-compose Design importer: maps one already-exported
//! `*.design.semio_compose_rs.json` document's hashed `pieces`/`connections` collections onto a
//! `Puzzle5dProjection`'s `parts`/`fasteners`, including the plane-frame → quaternion conversion.

use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dProjection};
use serde_json::Value;

//#region 🔖️Frames
/// 🧩️ Reads a semio_compose_rs "hashed collection" (`{ hash, items: [...] }`) or a bare array (test-friendly)
/// — mirrors semio_compose_rs's own `__itemsOf`/`fixtureItemsOf` duality (`semio_compose_rs/dev/algorithm/js/
/// index.ts:94`).
fn compose_collection_items(value: &Value) -> &[Value] {
    if let Some(array) = value.as_array() {
        return array;
    }
    value.get("items").and_then(Value::as_array).map_or(&[], Vec::as_slice)
}

fn read_vec3(value: &Value) -> Option<[f64; 3]> {
    Some([value.get("x")?.as_f64()?, value.get("y")?.as_f64()?, value.get("z")?.as_f64()?])
}

fn vec3_cross(a: [f64; 3], b: [f64; 3]) -> [f64; 3] {
    [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]
}

/// 🔄️ Rotation matrix (columns = x/y/z axes of the target frame) → quaternion `[x, y, z, w]`, via the
/// standard matrix-trace method (Shepperd's method, branching on the largest diagonal term for
/// numerical stability).
fn quaternion_from_axes(x_axis: [f64; 3], y_axis: [f64; 3], z_axis: [f64; 3]) -> [f64; 4] {
    let m00 = x_axis[0];
    let m10 = x_axis[1];
    let m20 = x_axis[2];
    let m01 = y_axis[0];
    let m11 = y_axis[1];
    let m21 = y_axis[2];
    let m02 = z_axis[0];
    let m12 = z_axis[1];
    let m22 = z_axis[2];
    let trace = m00 + m11 + m22;
    if trace > 0.0 {
        let s = (trace + 1.0).sqrt() * 2.0;
        [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4.0]
    } else if m00 > m11 && m00 > m22 {
        let s = (1.0 + m00 - m11 - m22).sqrt() * 2.0;
        [s / 4.0, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]
    } else if m11 > m22 {
        let s = (1.0 + m11 - m00 - m22).sqrt() * 2.0;
        [(m01 + m10) / s, s / 4.0, (m12 + m21) / s, (m02 - m20) / s]
    } else {
        let s = (1.0 + m22 - m00 - m11).sqrt() * 2.0;
        [(m02 + m20) / s, (m12 + m21) / s, s / 4.0, (m10 - m01) / s]
    }
}
//#endregion 🔖️Frames

//#region 🔖️ComposeImport
fn compose_piece_to_part(piece: &Value) -> Option<Puzzle5dPart> {
    let id = piece.get("id")?.as_str()?.to_string();
    let part_kind = piece.get("type").and_then(|value| value.get("id")).and_then(Value::as_str).map(str::to_string);
    let pose = piece.get("pose");
    let center = pose.and_then(|pose| pose.get("center"));
    let x = center.and_then(|center| center.get("u")).and_then(Value::as_f64).unwrap_or(0.0);
    let y = center.and_then(|center| center.get("v")).and_then(Value::as_f64).unwrap_or(0.0);
    let plane = pose.and_then(|pose| pose.get("plane"));
    let origin = plane.and_then(|plane| plane.get("origin")).and_then(read_vec3).unwrap_or([0.0, 0.0, 0.0]);
    let orientation = match (plane.and_then(|plane| plane.get("xAxis")).and_then(read_vec3), plane.and_then(|plane| plane.get("yAxis")).and_then(read_vec3)) {
        (Some(x_axis), Some(y_axis)) => Some(quaternion_from_axes(x_axis, y_axis, vec3_cross(x_axis, y_axis))),
        _ => None,
    };
    Some(Puzzle5dPart { id, part_kind, part_2d: Puzzle5dPart2d { x, y, ..Default::default() }, part_3d: Puzzle5dPart3d { origin, orientation, ..Default::default() }, grips: Vec::new() })
}

fn compose_connection_to_fastener(connection: &Value) -> Option<Puzzle5dFastener> {
    let id = connection.get("id")?.as_str()?.to_string();
    let side = |key: &str| -> Option<String> {
        let side = connection.get(key)?;
        let piece_id = side.get("piece")?.get("id")?.as_str()?;
        let connector_id = side.get("connector")?.get("id")?.as_str()?;
        Some(format!("{piece_id}:{connector_id}"))
    };
    let source = side("parent")?;
    let target = side("child")?;
    let number = |key: &str| connection.get(key).and_then(Value::as_f64).unwrap_or(0.0);
    Some(Puzzle5dFastener { id, source, target, fastener_kind: None, gap: number("gap"), shift: number("shift"), rise: number("rise"), rotation: number("rotation"), turn: number("turn"), tilt: number("tilt") })
}

/// 🌉️ Imports a semio_compose_rs Design document (the `*.design.semio_compose_rs.json` shape: top-level `pieces`/
/// `connections` hashed collections) into a `Puzzle5dProjection`'s `parts`/`fasteners` — pieces map to
/// parts (2D position from `pose.center`, 3D pose from `pose.plane`, kind from `piece.type.id` as a
/// free-form string key), connections map to fasteners (`gap`/`shift`/`rise`/`rotation`/`turn`/`tilt`
/// copy verbatim onto the fields `Puzzle5dFastener` gained to unify with `Puzzle3dAttraction`).
/// Scope: this converts ONE already-exported design document, not a full multi-file kit bundle —
/// resolving a piece's type name/representations/grip catalog (which live in separate,
/// content-addressed `type/*.type.semio_compose_rs.json` files in a real kit) is out of scope here; parts
/// import with an empty `grips` list and `kind_catalogs`/`kind_compatibility` untouched, left for the
/// caller to merge in separately (e.g. via a block 3d document's `puzzle3d_catalog_fragment`).
pub fn import_compose_design_json(design_json: &Value) -> Puzzle5dProjection {
    let mut projection = Puzzle5dProjection::default();
    if let Some(label) = design_json.get("name").and_then(Value::as_str) {
        projection.label = Some(label.to_string());
    }
    let pieces = design_json.get("pieces").map_or(&[][..], |value| compose_collection_items(value));
    projection.parts = pieces.iter().filter_map(compose_piece_to_part).collect();
    let connections = design_json.get("connections").map_or(&[][..], |value| compose_collection_items(value));
    projection.fasteners = connections.iter().filter_map(compose_connection_to_fastener).collect();
    projection
}
//#endregion 🔖️ComposeImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 📄️ A minimal semio_compose_rs Design document matching the real `*.design.semio_compose_rs.json` shape (see
    /// `semio_compose_rs/fixture/kit/dev/metabolism/wip/initialKit/design/nakagin-capsule-tower.design.semio_compose_rs.json`):
    /// hashed `{ hash, items }` collections, `pose.center`/`pose.plane`, and a `parent`/`child`
    /// connection with `gap`/`shift`/`rise`/`rotation`/`turn`/`tilt` fields.
    fn compose_design_fixture() -> Value {
        serde_json::json!({
            "id": "design-1",
            "name": "Test Tower",
            "pieces": {
                "hash": "h",
                "items": [
                    {
                        "id": "piece-a",
                        "type": { "id": "type-capsule", "hash": "h" },
                        "pose": {
                            "plane": { "origin": { "x": 0.0, "y": 0.0, "z": 0.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } },
                            "center": { "u": 5.0, "v": 10.0 }
                        }
                    },
                    {
                        "id": "piece-b",
                        "type": { "id": "type-capsule", "hash": "h" },
                        "pose": {
                            "plane": { "origin": { "x": 3.0, "y": 4.0, "z": 5.0 }, "xAxis": { "x": 1.0, "y": 0.0, "z": 0.0 }, "yAxis": { "x": 0.0, "y": 1.0, "z": 0.0 } },
                            "center": { "u": 15.0, "v": 10.0 }
                        }
                    }
                ]
            },
            "connections": {
                "hash": "h",
                "items": [
                    {
                        "id": "conn-1",
                        "parent": { "piece": { "id": "piece-a", "hash": "h" }, "connector": { "id": "conn-a", "hash": "h" } },
                        "child": { "piece": { "id": "piece-b", "hash": "h" }, "connector": { "id": "conn-b", "hash": "h" } },
                        "gap": 0.1,
                        "shift": 0.2,
                        "rise": 0.3,
                        "rotation": 270.0,
                        "turn": 0.0,
                        "tilt": 0.0
                    }
                ]
            }
        })
    }

    #[test]
    fn import_compose_design_json_maps_pieces_to_parts() {
        let projection = import_compose_design_json(&compose_design_fixture());
        assert_eq!(projection.label.as_deref(), Some("Test Tower"));
        assert_eq!(projection.parts.len(), 2);
        let part_a = projection.parts.iter().find(|part| part.id == "piece-a").expect("piece-a imported");
        assert_eq!(part_a.part_kind.as_deref(), Some("type-capsule"));
        assert_eq!(part_a.part_2d.x, 5.0);
        assert_eq!(part_a.part_2d.y, 10.0);
        assert_eq!(part_a.part_3d.origin, [0.0, 0.0, 0.0]);
        // Identity xAxis/yAxis -> identity quaternion [0,0,0,1].
        assert_eq!(part_a.part_3d.orientation, Some([0.0, 0.0, 0.0, 1.0]));
        let part_b = projection.parts.iter().find(|part| part.id == "piece-b").expect("piece-b imported");
        assert_eq!(part_b.part_3d.origin, [3.0, 4.0, 5.0]);
    }

    #[test]
    fn import_compose_design_json_maps_connections_to_fasteners_with_transform_fields() {
        let projection = import_compose_design_json(&compose_design_fixture());
        assert_eq!(projection.fasteners.len(), 1);
        let fastener = &projection.fasteners[0];
        assert_eq!(fastener.id, "conn-1");
        assert_eq!(fastener.source, "piece-a:conn-a");
        assert_eq!(fastener.target, "piece-b:conn-b");
        assert_eq!(fastener.gap, 0.1);
        assert_eq!(fastener.shift, 0.2);
        assert_eq!(fastener.rise, 0.3);
        assert_eq!(fastener.rotation, 270.0);
    }

    #[test]
    fn import_compose_design_json_tolerates_bare_arrays_not_just_hashed_collections() {
        let bare = serde_json::json!({
            "id": "design-2",
            "pieces": [{ "id": "p1", "type": { "id": "k" } }],
            "connections": []
        });
        let projection = import_compose_design_json(&bare);
        assert_eq!(projection.parts.len(), 1);
        assert_eq!(projection.parts[0].id, "p1");
    }

    #[test]
    fn quaternion_from_axes_reports_identity_for_the_identity_frame() {
        let q = quaternion_from_axes([1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]);
        for (actual, expected) in q.iter().zip([0.0, 0.0, 0.0, 1.0].iter()) {
            assert!((actual - expected).abs() < 1e-9, "expected identity quaternion, got {q:?}");
        }
    }
}
//#endregion 🧪️Tests
