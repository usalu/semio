use super::*;
use crate::brep::operations::primitives::{make_cone, make_cylinder, make_planar_face_from_points, make_sphere, make_torus};
use crate::brep::operations::sweep::{extrude_face, revolve_face};
use crate::brep::queries::analysis::{manifold_report, mass_properties, topology_counts, validity, ShapeScope, Watertightness};
use crate::brep::queries::tessellation::tessellate_solid;
use crate::brep::representation::vector::matrix::Affine3;
use serde_json::Value;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

fn number(value: &Value) -> f64 {
    value.as_f64().expect("fixture number")
}

fn triple(value: &Value) -> [f64; 3] {
    [number(&value[0]), number(&value[1]), number(&value[2])]
}

fn build_operand(body: &mut Body, rec: &mut OpRecorder, spec: &Value) -> Result<SolidId, KernelError> {
    let solid = match spec["kind"].as_str().expect("fixture kind") {
        "box" => {
            let [w, d, h] = triple(&spec["size"]);
            make_box(body, w, d, h, rec)?
        }
        "sphere" => make_sphere(body, number(&spec["radius"]), rec)?,
        "cylinder" => make_cylinder(body, number(&spec["radius"]), number(&spec["height"]), rec)?,
        "cone" => make_cone(body, number(&spec["radius"]), number(&spec["height"]), rec)?,
        "torus" => make_torus(body, number(&spec["major"]), number(&spec["minor"]), rec)?,
        "prism" => {
            let points: Vec<Pnt3> = spec["polygon"].as_array().expect("polygon").iter().map(|p| Pnt3::new(number(&p[0]), number(&p[1]), 0.0)).collect();
            let face = make_planar_face_from_points(body, &points, rec)?;
            extrude_face(body, face, Vec3::Z, number(&spec["height"]), rec)?
        }
        "revolved" => {
            let points: Vec<Pnt3> = spec["profile"].as_array().expect("profile").iter().map(|p| Pnt3::new(number(&p[0]), 0.0, number(&p[1]))).collect();
            let face = make_planar_face_from_points(body, &points, rec)?;
            revolve_face(body, face, Pnt3::new(0.0, 0.0, 0.0), Vec3::Z, std::f64::consts::TAU, rec)?
        }
        other => return Err(KernelError::InvalidInput(format!("fixture kind {other}"))),
    };
    let mut solid = solid;
    if !spec["rotate"].is_null() {
        for (axis, degrees) in [Vec3::X, Vec3::Y, Vec3::Z].into_iter().zip(triple(&spec["rotate"])) {
            if degrees != 0.0 {
                solid = transform_solid(body, solid, &Affine3::rotation_axis_angle(axis, degrees.to_radians()), rec)?;
            }
        }
    }
    if !spec["translate"].is_null() {
        let [x, y, z] = triple(&spec["translate"]);
        solid = transform_solid(body, solid, &Affine3::translation(Vec3::new(x, y, z)), rec)?;
    }
    Ok(solid)
}

fn mesh_measures(mesh: &MeshTransfer) -> (f64, f64) {
    let point = |i: u32| {
        let k = i as usize * 3;
        [f64::from(mesh.position[k]), f64::from(mesh.position[k + 1]), f64::from(mesh.position[k + 2])]
    };
    let (mut volume, mut area) = (0.0, 0.0);
    for tri in mesh.index.chunks_exact(3) {
        let (a, b, c) = (point(tri[0]), point(tri[1]), point(tri[2]));
        let cross = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])];
        area += 0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
    }
    (volume, area)
}

fn run_case(entry: &Value, tolerance: &Value) -> Result<String, String> {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let a = build_operand(&mut body, &mut rec, &entry["a"]).map_err(|e| format!("operand a: {e:?}"))?;
    let b = build_operand(&mut body, &mut rec, &entry["b"]).map_err(|e| format!("operand b: {e:?}"))?;
    let op = match entry["operation"].as_str().expect("operation") {
        "fuse" => BooleanOp::Unite,
        "cut" => BooleanOp::Cut,
        _ => BooleanOp::Intersect,
    };
    let expected = &entry["expected"];
    let outcome = boolean_solid(&mut body, a, b, op, 1e-6, &mut rec);
    if expected["empty"].as_bool().expect("empty") {
        return match outcome {
            Err(KernelError::Boolean(BooleanError::InvalidResult(message))) if message.contains("empty") => Ok("empty".into()),
            other => Err(format!("expected an empty result, got {:?}", other.map(|_| "a solid"))),
        };
    }
    let result = outcome.map_err(|e| format!("boolean failed: {e:?}"))?;
    let scope = ShapeScope::solid(result);
    if std::env::var("KB1_DUMP").is_ok() {
        for face in body.solid_faces(result) {
            debug_dump_face(&body, face);
        }
    }
    let mut failures = Vec::new();
    let mut note = |ok: bool, text: String| {
        if !ok {
            failures.push(text);
        }
    };
    let rel = |got: f64, want: f64, tol: f64| (got - want).abs() <= tol * want.abs().max(1.0);
    let mass = mass_properties(&body, &scope, 1e-4).map_err(|e| format!("mass: {e:?}"))?;
    let volume = mass.volumetric.map_or(f64::NAN, |v| v.volume);
    note(rel(volume, number(&expected["volume"]), number(&tolerance["volume"])), format!("volume {volume} vs {}", expected["volume"]));
    note(rel(mass.area, number(&expected["area"]), number(&tolerance["area"])), format!("area {} vs {}", mass.area, expected["area"]));
    let report = validity(&body, &scope).map_err(|e| format!("validity: {e:?}"))?;
    note(report.ok, format!("validity {:?}", report.issues.iter().map(|i| format!("{}:{}", i.code, i.message)).collect::<Vec<_>>()));
    let closed = manifold_report(&body, &scope).map_err(|e| format!("manifold: {e:?}"))?;
    note(closed.verdict == Watertightness::Watertight, format!("manifold {closed:?}"));
    let counts = topology_counts(&body, &scope).map_err(|e| format!("topology: {e:?}"))?;
    if entry["compareTopology"].as_bool().expect("compareTopology") {
        note(counts.euler_characteristic == number(&expected["euler"]) as i64, format!("euler {} vs {}", counts.euler_characteristic, expected["euler"]));
        note(counts.shells == expected["shells"].as_u64().expect("shells") as usize, format!("shells {} vs {}", counts.shells, expected["shells"]));
    }
    match tessellate_solid(&body, result, 0.01) {
        Ok(mesh) => {
            let (mesh_volume, mesh_area) = mesh_measures(&mesh);
            note(rel(mesh_volume, number(&expected["volume"]), 3.0 * number(&tolerance["volume"]).max(5e-3)), format!("tessellated volume {mesh_volume} vs {}", expected["volume"]));
            note(rel(mesh_area, number(&expected["area"]), 3.0 * number(&tolerance["area"]).max(5e-3)), format!("tessellated area {mesh_area} vs {}", expected["area"]));
        }
        Err(e) => note(false, format!("tessellation: {e:?}")),
    }
    if failures.is_empty() { Ok(format!("volume={volume:.5} area={:.5}", mass.area)) } else { Err(failures.join("; ")) }
}

/// 🧪 Every case of the language-agnostic differential fixture, run in parallel: `manifold-3d` measured the expected volume,
/// area, Euler characteristic and shell count of each `a op b`, and the exact kernel must reproduce them with a valid,
/// watertight result whose tessellation agrees too. `KB1_ONLY` narrows the run to ids containing that text.
#[test]
fn boolean_differential_matrix_matches_manifold_oracle() {
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/differential-matrix/🔣️.json")).unwrap();
    let only = std::env::var("KB1_ONLY").ok();
    let cases: Vec<&Value> = fixture["cases"].as_array().unwrap().iter().filter(|c| only.as_ref().is_none_or(|text| c["id"].as_str().unwrap().contains(text.as_str()))).collect();
    let threads = std::env::var("KB1_THREADS").ok().and_then(|t| t.parse().ok()).unwrap_or(4usize);
    let limit = std::env::var("KB1_LIMIT").ok().and_then(|t| t.parse().ok()).unwrap_or(120u64);
    let next = AtomicUsize::new(0);
    let results: Mutex<Vec<(String, Result<String, String>)>> = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..threads {
            scope.spawn(|| loop {
                let index = next.fetch_add(1, Ordering::SeqCst);
                let Some(entry) = cases.get(index) else { break };
                let id = entry["id"].as_str().unwrap().to_string();
                let started = std::time::Instant::now();
                let (sender, receiver) = std::sync::mpsc::channel();
                std::thread::Builder::new()
                    .stack_size(64 << 20)
                    .spawn({
                        let entry = (*entry).clone();
                        let tolerance = fixture["tolerance"].clone();
                        move || {
                            let outcome = std::panic::catch_unwind(|| run_case(&entry, &tolerance)).unwrap_or_else(|_| Err("panicked".into()));
                            let _ = sender.send(outcome);
                        }
                    })
                    .unwrap();
                let outcome = receiver.recv_timeout(std::time::Duration::from_secs(limit)).unwrap_or_else(|_| Err(format!("did not finish within {limit}s")));
                eprintln!("[DEBUG] case {id} {} in {}ms {}", if outcome.is_ok() { "OK" } else { "FAIL" }, started.elapsed().as_millis(), match &outcome { Ok(text) | Err(text) => text.as_str() });
                results.lock().unwrap().push((id, outcome));
            });
        }
    });
    let results = results.into_inner().unwrap();
    let failed: Vec<String> = results.iter().filter_map(|(id, outcome)| outcome.as_ref().err().map(|e| format!("{id}: {e}"))).collect();
    eprintln!("[DEBUG] matrix {} cases, {} failed", results.len(), failed.len());
    assert!(failed.is_empty(), "{} of {} cases failed:\n{}", failed.len(), results.len(), failed.join("\n"));
}

/// [DEBUG] temporary: tessellates every face of one case separately with timings.
#[test]
fn debug_face_tessellation() {
    let Ok(only) = std::env::var("KB1_FACES") else { return };
    let fixture: Value = serde_json::from_str(include_str!("../../🧫️fixtures/differential-matrix/🔣️.json")).unwrap();
    let entry = fixture["cases"].as_array().unwrap().iter().find(|c| c["id"] == only.as_str()).unwrap().clone();
    std::thread::Builder::new().stack_size(64 << 20).spawn(move || {
        let mut body = Body::new();
        let mut rec = OpRecorder::new();
        let a = build_operand(&mut body, &mut rec, &entry["a"]).unwrap();
        let b = build_operand(&mut body, &mut rec, &entry["b"]).unwrap();
        let op = match entry["operation"].as_str().unwrap() { "fuse" => BooleanOp::Unite, "cut" => BooleanOp::Cut, _ => BooleanOp::Intersect };
        let result = boolean_solid(&mut body, a, b, op, 1e-6, &mut rec).unwrap();
        let body = std::sync::Arc::new(body);
        for face in body.solid_faces(result) {
            let started = std::time::Instant::now();
            let (sender, receiver) = std::sync::mpsc::channel();
            let shared = body.clone();
            std::thread::spawn(move || { let _ = sender.send(crate::brep::queries::tessellation::tessellate_face(&shared, face, 0.01).map(|m| m.index.len() / 3)); });
            let outcome = receiver.recv_timeout(std::time::Duration::from_secs(10));
            eprintln!("[DEBUG] face {face} tessellation {:?} in {}ms", outcome, started.elapsed().as_millis());
        }
    }).unwrap().join().unwrap();
}
