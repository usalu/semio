use super::*;
use crate::brep::representation::arena::{ArenaId, Curve3Id, SolidId};
use semio_framework_pack_json::{from_json_str, to_json_string, JsonMemberPolicy};
use serde_json::Value;

const FIXTURE: &str = include_str!("../../../🧫️fixtures/🪬️shape-values/🔣️.json");

const KINDS: [(&str, GeometryKind); 9] = [
    ("vertex", GeometryKind::Vertex),
    ("edge", GeometryKind::Edge),
    ("wire", GeometryKind::Wire),
    ("face", GeometryKind::Face),
    ("shell", GeometryKind::Shell),
    ("solid", GeometryKind::Solid),
    ("compound", GeometryKind::Compound),
    ("curve", GeometryKind::Curve),
    ("surface", GeometryKind::Surface),
];

fn fixture() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}

fn vector(value: &Value) -> [f64; 3] {
    std::array::from_fn(|axis| value[axis].as_f64().unwrap())
}

fn build(kernel: &mut Brep, recipe: &Value) -> GeometryHandle {
    let operand = |kernel: &mut Brep, key: &str| build(kernel, &recipe[key]);
    match recipe["op"].as_str().unwrap() {
        "box" => {
            let size = vector(&recipe["size"]);
            kernel.box_prim_sync(size[0], size[1], size[2]).unwrap()
        }
        "cylinder" => kernel.cylinder_prim_sync(recipe["radius"].as_f64().unwrap(), recipe["height"].as_f64().unwrap()).unwrap(),
        "translate" => {
            let shape = operand(kernel, "of");
            kernel.translate_sync(&shape, vector(&recipe["offset"])).unwrap()
        }
        "fuse" | "cut" | "intersect" => {
            let (a, b) = (operand(kernel, "a"), operand(kernel, "b"));
            match recipe["op"].as_str().unwrap() {
                "fuse" => kernel.fuse_sync(&a, &b),
                "cut" => kernel.cut_sync(&a, &b),
                _ => kernel.intersect_sync(&a, &b),
            }
            .unwrap()
        }
        "vertex" => kernel.vertex_sync(vector(&recipe["point"])).unwrap(),
        "edgeOf" | "faceOf" => {
            let shape = operand(kernel, "of");
            let topology = kernel.deconstruct_sync(&shape).unwrap();
            let index = recipe["index"].as_u64().unwrap() as usize;
            if recipe["op"] == "edgeOf" { topology.edges[index].clone() } else { topology.faces[index].clone() }
        }
        "shellOf" => {
            let shape = operand(kernel, "of");
            kernel.solid_shells_sync(&shape).unwrap()[0].clone()
        }
        "planarFace" => {
            let points: Vec<_> = recipe["points"].as_array().unwrap().iter().map(vector).collect();
            kernel.planar_face_from_points_sync(&points).unwrap()
        }
        "rectangleWire" => {
            let size = recipe["size"].as_array().unwrap();
            kernel.rectangle_wire_sync(size[0].as_f64().unwrap(), size[1].as_f64().unwrap()).unwrap()
        }
        "compound" => {
            let solids: Vec<_> = recipe["of"].as_array().unwrap().iter().map(|member| build(kernel, member)).collect();
            kernel.compound_sync(&solids).unwrap()
        }
        "line" => kernel.line_curve_sync(vector(&recipe["start"]), vector(&recipe["end"])).unwrap(),
        "plane" => kernel.plane_surface_sync(vector(&recipe["origin"]), vector(&recipe["normal"])).unwrap(),
        other => panic!("unknown recipe op {other}"),
    }
}

fn measure(kernel: &Brep, handle: &GeometryHandle, name: &str) -> f64 {
    match name {
        "volume" => kernel.volume_sync(handle),
        "area" => kernel.area_sync(handle),
        "length" => kernel.length_sync(handle),
        other => panic!("unknown measure {other}"),
    }
    .unwrap()
}

fn kind_of(name: &str) -> GeometryKind {
    KINDS.iter().find(|(tag, _)| *tag == name).unwrap().1
}

fn encode(shape: &ShapeValue) -> String {
    to_json_string(shape)
}

fn parry_box_volume(size: [f64; 3]) -> f64 {
    use parry3d::shape::Shape;
    f64::from(parry3d::shape::Cuboid::new(parry3d::math::Vector::new(size[0] as f32 / 2.0, size[1] as f32 / 2.0, size[2] as f32 / 2.0)).mass_properties(1.0).mass())
}

/// 🧪️ Every geometry kind survives export, JSON codec and import with kind, labels, handle, counts and measures intact.
#[test]
fn shape_value_round_trips_every_kind_with_fixture_counts_and_measures() {
    let fixture = fixture();
    let mut seen = std::collections::BTreeSet::new();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let tolerance = case["tolerance"].as_f64().unwrap_or(fixture["tolerance"].as_f64().unwrap());
        let mut source = Brep::new();
        let handle = build(&mut source, &case["recipe"]);
        let shape = source.export_shape(&handle).unwrap();
        shape.check().unwrap_or_else(|error| panic!("{name}: {error}"));
        let kind = kind_of(case["kind"].as_str().unwrap());
        seen.insert(format!("{kind:?}"));
        assert_eq!(shape.kind(), kind, "{name}");
        assert_eq!(shape.label(), source.label_of(&handle), "{name}: root label");
        for (tag, component_kind) in KINDS.iter().take(6) {
            if let Some(expected) = case["components"][*tag].as_u64() {
                assert_eq!(shape.components(*component_kind).len() as u64, expected, "{name}: {tag} components");
            }
        }
        let decoded: ShapeValue = from_json_str(&encode(&shape), JsonMemberPolicy::Reject).unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(decoded, shape, "{name}: codec");
        let mut target = Brep::new();
        let imported = target.import_shape(&decoded).unwrap();
        assert_eq!(imported, handle, "{name}: fresh-session handle");
        assert_eq!(target.kind_sync(&imported).unwrap(), kind, "{name}");
        assert_eq!(target.label_of(&imported), shape.label(), "{name}: imported label");
        if let Some(measures) = case["measures"].as_object() {
            for (measure_name, expected) in measures {
                let expected = expected.as_f64().unwrap();
                let original = measure(&source, &handle, measure_name);
                let reborn = measure(&target, &imported, measure_name);
                assert!((original - expected).abs() < tolerance, "{name}: {measure_name} oracle {expected} vs source {original}");
                assert_eq!(original, reborn, "{name}: {measure_name} after import");
            }
        }
        assert_eq!(target.export_shape(&imported).unwrap(), shape, "{name}: re-export");
    }
    assert_eq!(seen.len(), 9, "{seen:?}");
}

/// 🧮️ Fuse, cut and intersect on imported values equal the session-native result and the parry cuboid algebra.
#[test]
fn shape_value_booleans_on_imported_shapes_equal_session_native_results() {
    let size = [2.0, 2.0, 2.0];
    let overlap = parry_box_volume([1.0, 1.0, 1.0]);
    let cube = parry_box_volume(size);
    let mut native = Brep::new();
    let a = native.box_prim_sync(size[0], size[1], size[2]).unwrap();
    let moved = native.box_prim_sync(size[0], size[1], size[2]).unwrap();
    let b = native.translate_sync(&moved, [1.0, 1.0, 1.0]).unwrap();
    let (shape_a, shape_b) = (native.export_shape(&a).unwrap(), native.export_shape(&b).unwrap());
    for (operation, oracle) in [("fuse", 2.0 * cube - overlap), ("cut", cube - overlap), ("intersect", overlap)] {
        let mut session = Brep::new();
        let (imported_a, imported_b) = (session.import_shape(&shape_a).unwrap(), session.import_shape(&shape_b).unwrap());
        let (expected, result) = match operation {
            "fuse" => (native.fuse_sync(&a, &b), session.fuse_sync(&imported_a, &imported_b)),
            "cut" => (native.cut_sync(&a, &b), session.cut_sync(&imported_a, &imported_b)),
            _ => (native.intersect_sync(&a, &b), session.intersect_sync(&imported_a, &imported_b)),
        };
        let (expected, result) = (expected.unwrap(), result.unwrap());
        let (native_volume, value_volume) = (native.volume_sync(&expected).unwrap(), session.volume_sync(&result).unwrap());
        assert!((native_volume - value_volume).abs() < 1e-9, "{operation}: native {native_volume} vs value {value_volume}");
        assert!((value_volume - oracle).abs() < 1e-6, "{operation}: parry oracle {oracle} vs {value_volume}");
        let exported = session.export_shape(&result).unwrap();
        let mut again = Brep::new();
        let reborn = again.import_shape(&exported).unwrap();
        assert!((again.volume_sync(&reborn).unwrap() - value_volume).abs() < 1e-12, "{operation}: boolean result survives export");
    }
}

/// 🏷️ Sub-element labels resolve through `handle_for_label` in a fresh session and, shifted, in a populated one.
#[test]
fn shape_value_labels_resolve_after_import_in_fresh_and_populated_sessions() {
    let fixture = fixture();
    let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == "cut-through-hole").unwrap();
    let mut source = Brep::new();
    let handle = build(&mut source, &case["recipe"]);
    let shape = source.export_shape(&handle).unwrap();
    let components: Vec<_> = [GeometryKind::Vertex, GeometryKind::Edge, GeometryKind::Face, GeometryKind::Shell, GeometryKind::Solid].into_iter().flat_map(|kind| shape.components(kind)).collect();
    assert!(components.len() > 30, "{}", components.len());
    let mut fresh = Brep::new();
    fresh.import_shape(&shape).unwrap();
    let mut populated = Brep::new();
    let neighbour = populated.box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let neighbour_label = populated.label_of(&neighbour);
    let imported = populated.import_shape_mapped(&shape).unwrap();
    assert!(imported.label_offset > 0);
    assert_eq!(populated.label_of(&neighbour), neighbour_label);
    for component in &components {
        let in_source = source.handle_for_label(component.label).unwrap_or_else(|| panic!("source {component:?}"));
        let in_fresh = fresh.handle_for_label(component.label).unwrap_or_else(|| panic!("fresh {component:?}"));
        assert_eq!(in_fresh, in_source, "{component:?}: fresh handle");
        assert_eq!(fresh.kind_sync(&in_fresh).unwrap(), component.kind);
        let shifted = imported.session_label(component.label);
        let in_populated = populated.handle_for_label(shifted).unwrap_or_else(|| panic!("populated {component:?}"));
        assert_eq!(populated.label_of(&in_populated), Some(shifted));
        assert_eq!(populated.kind_sync(&in_populated).unwrap(), component.kind);
    }
    let faces = fresh.deconstruct_sync(&handle).unwrap().faces;
    let labels: Vec<_> = faces.iter().map(|face| fresh.label_of(face).unwrap()).collect();
    assert_eq!(labels, shape.components(GeometryKind::Face).iter().map(|component| component.label).collect::<Vec<_>>());
}

/// 🎲️ Export, import and re-export are byte-identical, and equal values mint equal handles in fresh sessions.
#[test]
fn shape_value_encoding_and_handles_are_deterministic() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut first = Brep::new();
        let handle = build(&mut first, &case["recipe"]);
        let shape = first.export_shape(&handle).unwrap();
        assert_eq!(encode(&first.export_shape(&handle).unwrap()), encode(&shape), "{name}: repeated export");
        let (mut left, mut right) = (Brep::new(), Brep::new());
        let (left_handle, right_handle) = (left.import_shape(&shape).unwrap(), right.import_shape(&shape).unwrap());
        assert_eq!(left_handle, right_handle, "{name}: fresh handles");
        let (left_bytes, right_bytes) = (encode(&left.export_shape(&left_handle).unwrap()), encode(&right.export_shape(&right_handle).unwrap()));
        assert_eq!(left_bytes, right_bytes, "{name}: sessions");
        assert_eq!(left_bytes, encode(&shape), "{name}: export-import-export");
        assert_eq!(shape.content_hash(), left.export_shape(&left_handle).unwrap().content_hash(), "{name}: hash");
        let twice = left.import_shape(&shape).unwrap();
        assert_ne!(twice, left_handle, "{name}: same-session import must not alias");
    }
}

/// 🔺️ Tessellating a value equals tessellating the session handle, in one shot and in budgeted steps, and cancels cleanly.
#[test]
fn shape_value_tessellation_equals_session_tessellation_and_is_resumable() {
    let fixture = fixture();
    let mut tessellated = 0;
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut kernel = Brep::new();
        let handle = build(&mut kernel, &case["recipe"]);
        let shape = kernel.export_shape(&handle).unwrap();
        match (kernel.tessellate_sync(&handle, 0.1), shape.tessellate(0.1)) {
            (Ok(expected), Ok(actual)) => {
                assert_eq!(actual, expected, "{name}");
                tessellated += 1;
                let mut job = shape.tessellate_job(0.1).unwrap();
                let (mut steps, mut done) = (0, 0);
                while !job.is_terminal() {
                    let step = job.step(1).unwrap();
                    steps += 1;
                    let progress = job.progress();
                    assert!(progress.units_done >= done && progress.units_done <= progress.units_total, "{name}: monotone");
                    done = progress.units_done;
                    assert!(matches!(step, TessellationStep::Working(_) | TessellationStep::Done(_)));
                }
                assert_eq!(job.into_mesh().unwrap().0, expected, "{name}: stepped");
                assert!(steps >= 1, "{name}");
            }
            (Err(_), Err(_)) => {}
            (expected, actual) => panic!("{name}: session {expected:?} vs value {actual:?}"),
        }
    }
    assert!(tessellated >= 10, "{tessellated}");
    let mut kernel = Brep::new();
    let solid = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let mut job = kernel.export_shape(&solid).unwrap().tessellate_job(0.1).unwrap();
    assert!(matches!(job.step(2).unwrap(), TessellationStep::Working(_)));
    job.cancel();
    assert!(job.is_terminal());
    assert!(matches!(job.step(8).unwrap(), TessellationStep::Cancelled(_)));
    assert!(job.into_mesh().is_none());
}

/// 🛡️ A decoded value with dangling references is refused instead of panicking, and the session stays untouched.
#[test]
fn shape_value_import_refuses_dangling_references_without_mutation() {
    let mut source = Brep::new();
    let solid = source.box_prim_sync(1.0, 2.0, 3.0).unwrap();
    let shape = source.export_shape(&solid).unwrap();
    let mut missing_root = shape.clone();
    missing_root.root = ShapeRoot::Solid(SolidId::from_raw(99, 0));
    let mut missing_curve = shape.clone();
    let edge = missing_curve.body.edges.ids().next().unwrap();
    missing_curve.body.edges.get_mut(edge).unwrap().curve = Curve3Id::from_raw(77, 0);
    let mut stale_labels = shape.clone();
    stale_labels.body.labels = LabelSource::from_next(0);
    let mut target = Brep::new();
    let before = (target.body.clone(), target.live.len());
    for (name, broken) in [("root", missing_root), ("curve", missing_curve), ("labels", stale_labels)] {
        assert!(matches!(target.import_shape(&broken), Err(BrepError::InvalidInput(_))), "{name}");
        assert!(matches!(broken.tessellate_job(0.1), Err(BrepError::InvalidInput(_))), "{name}");
    }
    assert_eq!((target.body.clone(), target.live.len()), before);
}

/// 📐️ The language-agnostic fixture meshes equal a fresh value tessellation and enclose the oracle volume and area.
#[test]
fn shape_value_fixture_tessellations_match_fresh_value_tessellation_and_oracle_measures() {
    let fixture = fixture();
    for entry in fixture["tessellations"].as_array().unwrap() {
        let name = entry["case"].as_str().unwrap();
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["name"] == name).unwrap();
        let mut kernel = Brep::new();
        let handle = build(&mut kernel, &case["recipe"]);
        let mesh = kernel.export_shape(&handle).unwrap().tessellate(entry["deflection"].as_f64().unwrap()).unwrap();
        let positions: Vec<f32> = entry["positions"].as_array().unwrap().iter().map(|value| value.as_f64().unwrap() as f32).collect();
        let indices: Vec<u32> = entry["indices"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
        assert_eq!(mesh.position, positions, "{name}: positions");
        assert_eq!(mesh.index, indices, "{name}: indices");
        let faces: std::collections::BTreeSet<_> = mesh.face_groups.iter().map(|group| group.entity_id.clone()).collect();
        let expected_faces: std::collections::BTreeSet<_> = entry["faceEntityIds"].as_array().unwrap().iter().map(|id| id.as_str().unwrap().to_string()).collect();
        assert_eq!(faces, expected_faces, "{name}: face labels");
        let corner = |index: u32| -> [f64; 3] { std::array::from_fn(|axis| f64::from(positions[index as usize * 3 + axis])) };
        let (mut volume, mut area) = (0.0, 0.0);
        for triangle in indices.chunks(3) {
            let (a, b, c) = (corner(triangle[0]), corner(triangle[1]), corner(triangle[2]));
            volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) + a[1] * (b[2] * c[0] - b[0] * c[2]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
            let (u, v) = (std::array::from_fn::<f64, 3, _>(|axis| b[axis] - a[axis]), std::array::from_fn::<f64, 3, _>(|axis| c[axis] - a[axis]));
            let cross = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
            area += (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt() / 2.0;
        }
        let relative = entry["relativeTolerance"].as_f64().unwrap_or(1e-6);
        let (expected_volume, expected_area) = (case["measures"]["volume"].as_f64().unwrap(), case["measures"]["area"].as_f64());
        assert!((volume - expected_volume).abs() / expected_volume <= relative, "{name}: mesh volume {volume} vs {expected_volume}");
        if let Some(expected_area) = expected_area {
            assert!((area - expected_area).abs() / expected_area <= relative, "{name}: mesh area {area} vs {expected_area}");
        }
    }
}

/// 🎲️ The same recipe builds a byte-identical value in every fresh session, booleans included.
#[test]
fn shape_value_recipes_build_identical_values_in_every_fresh_session() {
    let fixture = fixture();
    for case in fixture["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let hashes: std::collections::BTreeSet<_> = (0..6)
            .map(|_| {
                let mut kernel = Brep::new();
                let handle = build(&mut kernel, &case["recipe"]);
                kernel.export_shape(&handle).unwrap().content_hash()
            })
            .collect();
        assert_eq!(hashes.len(), 1, "{name}: recipe is not reproducible");
    }
}
