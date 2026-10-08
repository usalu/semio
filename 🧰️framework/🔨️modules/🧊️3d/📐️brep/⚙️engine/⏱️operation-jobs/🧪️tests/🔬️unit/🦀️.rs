use super::super::*;
use super::*;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

const FIXTURE: &str = include_str!("../../../🧫️fixtures/⏱️operation-jobs/🔣️.json");

/// 🧭️ Reads a fixture vector.
fn vec3(value: &Value) -> [f64; 3] {
    std::array::from_fn(|axis| value[axis].as_f64().unwrap())
}

/// 🧭️ Reads a fixture number by key.
fn number(value: &Value, key: &str) -> f64 {
    match value[key].as_str() {
        Some("infinity") => f64::INFINITY,
        Some("nan") => f64::NAN,
        _ => value[key].as_f64().unwrap_or_else(|| panic!("missing number {key} in {value}")),
    }
}

/// 🧭️ The fixture, parsed once per test.
fn fixture() -> Value {
    serde_json::from_str(FIXTURE).unwrap()
}

/// 🧭️ Case ids selected by `SEMIO_OPERATION_JOBS_CASE` (comma-separated substrings, any match; every
/// case when unset) minus those named by `SEMIO_OPERATION_JOBS_SKIP` (same syntax).
fn selected(id: &str) -> bool {
    let matches = |name: &str| std::env::var(name).ok().filter(|list| !list.is_empty()).map(|list| list.split(',').any(|part| id.contains(part)));
    matches("SEMIO_OPERATION_JOBS_CASE").unwrap_or(true) && !matches("SEMIO_OPERATION_JOBS_SKIP").unwrap_or(false)
}

/// 🧱️ Builds one fixture shape inside `kernel`.
fn build(kernel: &mut Brep, spec: &Value) -> GeometryHandle {
    let handle = if let Some(a) = spec.get("box") {
        kernel.box_prim_sync(a[0].as_f64().unwrap(), a[1].as_f64().unwrap(), a[2].as_f64().unwrap()).unwrap()
    } else if let Some(a) = spec.get("cylinder") {
        kernel.cylinder_prim_sync(a[0].as_f64().unwrap(), a[1].as_f64().unwrap()).unwrap()
    } else if let Some(points) = spec.get("convexHull") {
        let points: Vec<[f64; 3]> = points.as_array().unwrap().iter().map(vec3).collect();
        kernel.convex_hull_sync(&points).unwrap()
    } else if let Some(points) = spec.get("polygonFace") {
        let points: Vec<[f64; 3]> = points.as_array().unwrap().iter().map(vec3).collect();
        kernel.planar_face_from_points_sync(&points).unwrap()
    } else if let Some(a) = spec.get("rectangleFace") {
        let wire = kernel.rectangle_wire_sync(a[0].as_f64().unwrap(), a[1].as_f64().unwrap()).unwrap();
        kernel.planar_face_from_wire_sync(&wire).unwrap()
    } else if let Some(a) = spec.get("rectangleWire") {
        kernel.rectangle_wire_sync(a[0].as_f64().unwrap(), a[1].as_f64().unwrap()).unwrap()
    } else if let Some(points) = spec.get("polylineWire") {
        let points: Vec<[f64; 3]> = points.as_array().unwrap().iter().map(vec3).collect();
        kernel.polyline_wire_sync(&points).unwrap()
    } else {
        panic!("unknown fixture shape {spec}")
    };
    match spec.get("translate") {
        Some(offset) => kernel.translate_sync(&handle, vec3(offset)).unwrap(),
        None => handle,
    }
}

/// 📸️ Everything observable about a session: the whole body and the live handle set. Two equal
/// snapshots mean the session is byte-identical.
fn snapshot(kernel: &Brep) -> String {
    format!("{:?}|{:?}|{}", kernel.body, kernel.live.keys().collect::<Vec<_>>(), kernel.pending_mutations)
}

/// 🧱️ One built case: input handles by fixture key.
struct Scene {
    kernel: Brep,
    inputs: BTreeMap<String, Vec<GeometryHandle>>,
}

impl Scene {
    /// 🧱️ Builds every input of `case` into a fresh kernel.
    fn new(case: &Value) -> Self {
        let mut kernel = Brep::new();
        let mut inputs = BTreeMap::new();
        for (key, spec) in case["inputs"].as_object().unwrap() {
            let handles = match spec.as_array() {
                Some(list) => list.iter().map(|item| build(&mut kernel, item)).collect(),
                None => vec![build(&mut kernel, spec)],
            };
            inputs.insert(key.clone(), handles);
        }
        Self { kernel, inputs }
    }

    /// 🎯️ The single handle stored under `key`.
    fn one(&self, key: &str) -> GeometryHandle {
        self.inputs[key][0].clone()
    }

    /// 🎯️ Face handles of the case's `shape` chosen by index list or the `smallest` selector.
    fn faces(&mut self, selector: &Value) -> Vec<GeometryHandle> {
        let shape = self.one("shape");
        let topology = self.kernel.deconstruct_sync(&shape).unwrap();
        match selector.as_str() {
            Some("smallest") => {
                let mut best = topology.faces[0].clone();
                for face in &topology.faces {
                    if self.kernel.area_sync(face).unwrap() < self.kernel.area_sync(&best).unwrap() {
                        best = face.clone();
                    }
                }
                vec![best]
            }
            _ => selector.as_array().unwrap().iter().map(|index| topology.faces[index.as_u64().unwrap() as usize].clone()).collect(),
        }
    }

    /// 🎯️ Edge handles of the case's `shape` chosen by index list.
    fn edges(&mut self, selector: &Value) -> Vec<GeometryHandle> {
        let shape = self.one("shape");
        let topology = self.kernel.deconstruct_sync(&shape).unwrap();
        selector.as_array().unwrap().iter().map(|index| topology.edges[index.as_u64().unwrap() as usize].clone()).collect()
    }

    /// 🧭️ The case's operation as a descriptor.
    fn operation(&mut self, case: &Value) -> BrepOperation {
        let op = &case["operation"];
        match op["kind"].as_str().unwrap() {
            "fillet" => BrepOperation::Fillet { shape: self.one("shape"), radius: number(op, "radius") },
            "filletVariable" => BrepOperation::FilletVariable { shape: self.one("shape"), radius_start: number(op, "radiusStart"), radius_end: number(op, "radiusEnd") },
            "filletEdges" => BrepOperation::FilletEdges { shape: self.one("shape"), edges: self.edges(&op["edges"]), radius: number(op, "radius") },
            "chamfer" => BrepOperation::Chamfer { shape: self.one("shape"), distance: number(op, "distance") },
            "chamferAsymmetric" => BrepOperation::ChamferAsymmetric { shape: self.one("shape"), first: number(op, "first"), second: number(op, "second") },
            "chamferEdges" => BrepOperation::ChamferEdges { shape: self.one("shape"), edges: self.edges(&op["edges"]), distance: number(op, "distance") },
            "shell" => BrepOperation::Shell { shape: self.one("shape"), thickness: number(op, "thickness"), open_faces: self.faces(&op["openFaces"]) },
            "draft" => BrepOperation::Draft { shape: self.one("shape"), faces: self.faces(&op["faces"]), pull_direction: vec3(&op["pullDirection"]), neutral_point: vec3(&op["neutralPoint"]), angle: number(op, "angle") },
            "offsetSolid" => BrepOperation::OffsetSolid { shape: self.one("shape"), distance: number(op, "distance") },
            "offsetFace" => BrepOperation::OffsetFace { face: self.faces(&serde_json::json!([op["face"]])).remove(0), distance: number(op, "distance") },
            "thickenFace" => BrepOperation::ThickenFace { face: self.one("profile"), thickness: number(op, "thickness") },
            "defeature" => BrepOperation::Defeature { shape: self.one("shape"), faces: self.faces(&op["faces"]) },
            "extrude" => BrepOperation::Extrude { face: self.one("profile"), direction: vec3(&op["direction"]), distance: number(op, "distance") },
            "extrudeWire" => BrepOperation::ExtrudeWire { wire: self.one("wire"), vector: vec3(&op["vector"]) },
            "revolve" => BrepOperation::Revolve { face: self.one("profile"), axis_origin: vec3(&op["axisOrigin"]), axis_direction: vec3(&op["axisDirection"]), angle: number(op, "angle") },
            "sweep" => BrepOperation::Sweep { profile: self.one("profile"), path: self.one("path") },
            "pipe" => BrepOperation::Pipe { profile: self.one("profile"), path: self.one("path"), guide: self.inputs.get("guide").map(|guide| guide[0].clone()) },
            "helicalSweep" => BrepOperation::HelicalSweep { profile: self.one("profile"), axis_origin: vec3(&op["axisOrigin"]), axis_direction: vec3(&op["axisDirection"]), radius: number(op, "radius"), pitch: number(op, "pitch"), turns: number(op, "turns") },
            "loft" => BrepOperation::Loft { profiles: self.inputs["profiles"].clone(), smooth: op["smooth"].as_bool().unwrap() },
            "section" => BrepOperation::Section { solid: self.one("solid"), plane_origin: vec3(&op["planeOrigin"]), plane_normal: vec3(&op["planeNormal"]) },
            "split" => BrepOperation::Split { solid: self.one("solid"), plane_origin: vec3(&op["planeOrigin"]), plane_normal: vec3(&op["planeNormal"]) },
            "compoundCut" => BrepOperation::CompoundCut { target: self.one("target"), tools: self.inputs["tools"].clone() },
            "linearPattern" => BrepOperation::LinearPattern { shape: self.one("shape"), direction: vec3(&op["direction"]), spacing: number(op, "spacing"), count: op["count"].as_u64().unwrap() as usize },
            "circularPattern" => BrepOperation::CircularPattern { shape: self.one("shape"), axis: vec3(&op["axis"]), count: op["count"].as_u64().unwrap() as usize },
            "gridPattern" => BrepOperation::GridPattern { shape: self.one("shape"), dir_x: vec3(&op["dirX"]), dir_y: vec3(&op["dirY"]), spacing_x: number(op, "spacingX"), spacing_y: number(op, "spacingY"), count_x: op["countX"].as_u64().unwrap() as usize, count_y: op["countY"].as_u64().unwrap() as usize },
            "convexHull" => BrepOperation::ConvexHull { points: op["points"].as_array().unwrap().iter().map(vec3).collect() },
            other => panic!("unknown fixture operation {other}"),
        }
    }

    /// 🔁️ Runs the case through the one-shot synchronous facade.
    fn oneshot(&mut self, case: &Value) -> Result<Vec<GeometryHandle>, BrepError> {
        let op = &case["operation"];
        let kind = op["kind"].as_str().unwrap();
        let single = |handle: Result<GeometryHandle, BrepError>| handle.map(|handle| vec![handle]);
        match kind {
            "fillet" => single(self.kernel.fillet_sync(&self.one("shape"), number(op, "radius"))),
            "filletVariable" => single(self.kernel.fillet_variable_sync(&self.one("shape"), number(op, "radiusStart"), number(op, "radiusEnd"))),
            "filletEdges" => {
                let edges = self.edges(&op["edges"]);
                single(self.kernel.fillet_edges_sync(&self.one("shape"), &edges, number(op, "radius")))
            }
            "chamfer" => single(self.kernel.chamfer_sync(&self.one("shape"), number(op, "distance"))),
            "chamferAsymmetric" => single(self.kernel.chamfer_asymmetric_sync(&self.one("shape"), number(op, "first"), number(op, "second"))),
            "chamferEdges" => {
                let edges = self.edges(&op["edges"]);
                single(self.kernel.chamfer_edges_sync(&self.one("shape"), &edges, number(op, "distance")))
            }
            "shell" => {
                let open = self.faces(&op["openFaces"]);
                single(self.kernel.shell_sync(&self.one("shape"), number(op, "thickness"), &open))
            }
            "draft" => {
                let faces = self.faces(&op["faces"]);
                single(self.kernel.draft_sync(&self.one("shape"), &faces, vec3(&op["pullDirection"]), vec3(&op["neutralPoint"]), number(op, "angle")))
            }
            "offsetSolid" => single(self.kernel.offset_solid_sync(&self.one("shape"), number(op, "distance"))),
            "offsetFace" => {
                let face = self.faces(&serde_json::json!([op["face"]]));
                single(self.kernel.offset_face_sync(&face[0], number(op, "distance")))
            }
            "thickenFace" => single(self.kernel.thicken_face_sync(&self.one("profile"), number(op, "thickness"))),
            "defeature" => {
                let faces = self.faces(&op["faces"]);
                single(self.kernel.defeature_sync(&self.one("shape"), &faces))
            }
            "extrude" => single(self.kernel.extrude_sync(&self.one("profile"), vec3(&op["direction"]), number(op, "distance"))),
            "extrudeWire" => single(self.kernel.extrude_wire_sync(&self.one("wire"), vec3(&op["vector"]))),
            "revolve" => single(self.kernel.revolve_sync(&self.one("profile"), vec3(&op["axisOrigin"]), vec3(&op["axisDirection"]), number(op, "angle"))),
            "sweep" => single(self.kernel.sweep_sync(&self.one("profile"), &self.one("path"))),
            "pipe" => single(self.kernel.pipe_sync(&self.one("profile"), &self.one("path"), self.inputs.get("guide").map(|guide| &guide[0]))),
            "helicalSweep" => single(self.kernel.helical_sweep_sync(&self.one("profile"), vec3(&op["axisOrigin"]), vec3(&op["axisDirection"]), number(op, "radius"), number(op, "pitch"), number(op, "turns"))),
            "loft" => single(self.kernel.loft_sync(&self.inputs["profiles"].clone(), op["smooth"].as_bool().unwrap())),
            "section" => self.kernel.section_sync(&self.one("solid"), vec3(&op["planeOrigin"]), vec3(&op["planeNormal"])),
            "split" => self.kernel.split_sync(&self.one("solid"), vec3(&op["planeOrigin"]), vec3(&op["planeNormal"])).map(|(a, b)| vec![a, b]),
            "compoundCut" => single(self.kernel.compound_cut_sync(&self.one("target"), &self.inputs["tools"].clone())),
            "linearPattern" => single(self.kernel.linear_pattern_sync(&self.one("shape"), vec3(&op["direction"]), number(op, "spacing"), op["count"].as_u64().unwrap() as usize)),
            "circularPattern" => single(self.kernel.circular_pattern_sync(&self.one("shape"), vec3(&op["axis"]), op["count"].as_u64().unwrap() as usize)),
            "gridPattern" => single(self.kernel.grid_pattern_sync(&self.one("shape"), vec3(&op["dirX"]), vec3(&op["dirY"]), (number(op, "spacingX"), number(op, "spacingY")), op["countX"].as_u64().unwrap() as usize, op["countY"].as_u64().unwrap() as usize)),
            "convexHull" => {
                let points: Vec<[f64; 3]> = op["points"].as_array().unwrap().iter().map(vec3).collect();
                single(self.kernel.convex_hull_sync(&points))
            }
            other => panic!("unknown fixture operation {other}"),
        }
    }

    /// 📏️ Kind, volume, area and face count of every output. `mesh` measures the tessellation instead of
    /// integrating the exact surfaces — the cheap, deterministic yardstick for curved and free-form
    /// results, whose exact adaptive integration costs seconds per solid in a debug build.
    fn measure(&mut self, outputs: &[GeometryHandle], mesh: bool) -> Value {
        let mut volume = 0.0;
        let mut area = 0.0;
        let mut faces = 0usize;
        let mut kinds = Vec::new();
        for handle in outputs {
            let kind = self.kernel.kind_sync(handle).unwrap();
            kinds.push(format!("{kind:?}").to_lowercase());
            let (v, a) = if mesh { self.mesh_volume_and_area(handle) } else { (if kind == GeometryKind::Solid { self.kernel.volume_sync(handle).unwrap() } else { 0.0 }, self.kernel.area_sync(handle).unwrap()) };
            volume += v;
            area += a;
            faces += match kind {
                GeometryKind::Solid => self.kernel.deconstruct_sync(handle).unwrap().faces.len(),
                GeometryKind::Face => 1,
                other => panic!("unexpected output kind {other:?}"),
            };
        }
        serde_json::json!({"outputs": outputs.len(), "kinds": kinds, "volume": volume, "area": area, "faces": faces})
    }

    /// 🔺️ Enclosed volume and surface area of the tessellation of `handle` (deflection 0.01).
    fn mesh_volume_and_area(&mut self, handle: &GeometryHandle) -> (f64, f64) {
        let mesh = self.kernel.tessellate_sync(handle, 0.01).unwrap();
        let point = |index: u32| -> [f64; 3] { std::array::from_fn(|axis| f64::from(mesh.position[index as usize * 3 + axis])) };
        let (mut volume, mut area) = (0.0, 0.0);
        for triangle in mesh.index.chunks_exact(3) {
            let (a, b, c) = (point(triangle[0]), point(triangle[1]), point(triangle[2]));
            let cross = [(b[1] - a[1]) * (c[2] - a[2]) - (b[2] - a[2]) * (c[1] - a[1]), (b[2] - a[2]) * (c[0] - a[0]) - (b[0] - a[0]) * (c[2] - a[2]), (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])];
            volume += (a[0] * (b[1] * c[2] - b[2] * c[1]) - a[1] * (b[0] * c[2] - b[2] * c[0]) + a[2] * (b[0] * c[1] - b[1] * c[0])) / 6.0;
            area += 0.5 * (cross[0] * cross[0] + cross[1] * cross[1] + cross[2] * cross[2]).sqrt();
        }
        (volume.abs(), area)
    }
}

/// 📏️ Whether `case` is measured on its tessellation (curved and free-form results) rather than exactly.
fn mesh_measure(case: &Value) -> bool {
    case["measure"].as_str() == Some("mesh")
}

/// ✅️ Asserts `measured` matches the case's expectation (volume and area within `tolerance`).
fn assert_matches(id: &str, expect: &Value, measured: &Value) {
    assert_eq!(measured["outputs"], expect["outputs"], "{id}: output count");
    assert_eq!(measured["kinds"], expect["kinds"], "{id}: output kinds");
    assert_eq!(measured["faces"], expect["faces"], "{id}: face count");
    let tolerance = expect["tolerance"].as_f64().unwrap_or(1e-6);
    for key in ["volume", "area"] {
        let (actual, wanted) = (measured[key].as_f64().unwrap(), expect[key].as_f64().unwrap());
        assert!((actual - wanted).abs() <= tolerance * wanted.abs().max(1.0), "{id}: {key} {actual} vs fixture {wanted}");
    }
}

/// 🔬️ Independent third-party volume of an oracle description: signed fractions of parry3d cuboids
/// and cylinders plus parry3d convex hulls of point sets (`volume = Σ sign · mass`).
fn parry_volume(oracle: &Value) -> f64 {
    use parry3d::shape::Shape;
    let mut volume = 0.0;
    for cuboid in oracle["cuboids"].as_array().into_iter().flatten() {
        let size = vec3(&cuboid["size"]);
        let shape = parry3d::shape::Cuboid::new(parry3d::math::Vector::new(size[0] as f32 / 2.0, size[1] as f32 / 2.0, size[2] as f32 / 2.0));
        volume += cuboid["sign"].as_f64().unwrap() * f64::from(shape.mass_properties(1.0).mass());
    }
    for cylinder in oracle["cylinders"].as_array().into_iter().flatten() {
        let shape = parry3d::shape::Cylinder::new(number(cylinder, "height") as f32 / 2.0, number(cylinder, "radius") as f32);
        volume += cylinder["sign"].as_f64().unwrap() * f64::from(shape.mass_properties(1.0).mass());
    }
    for hull in oracle["hulls"].as_array().into_iter().flatten() {
        let points: Vec<parry3d::math::Point<f32>> = hull["points"].as_array().unwrap().iter().map(|point| parry3d::math::Point::new(point[0].as_f64().unwrap() as f32, point[1].as_f64().unwrap() as f32, point[2].as_f64().unwrap() as f32)).collect();
        let shape = parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("oracle hull");
        volume += hull["sign"].as_f64().unwrap() * f64::from(shape.mass_properties(1.0).mass());
    }
    volume
}

/// 📈️ The observations of one job run.
struct Trace {
    handles: Vec<GeometryHandle>,
    calls: usize,
    progress: Vec<BrepOperationProgress>,
}

/// ⏱️ Drives `operation` with `budget`, asserting after every step that the session is byte-identical
/// to `before` and that progress is monotone, until the job is `Ready`.
fn drive(scene: &mut Scene, operation: BrepOperation, budget: usize, before: &str) -> Result<Trace, BrepError> {
    let mut job = match scene.kernel.operation_job_sync(operation)? {
        BrepOperationAdmission::Answered(handles) => return Ok(Trace { handles, calls: 0, progress: Vec::new() }),
        BrepOperationAdmission::Job(job) => job,
    };
    assert_eq!(snapshot(&scene.kernel), before, "admission must not touch the session");
    let mut progress = vec![job.progress()];
    let mut calls = 0;
    loop {
        calls += 1;
        let step = scene.kernel.step_operation_job_sync(&mut job, budget);
        let step = match step {
            Ok(step) => step,
            Err(error) => {
                assert_eq!(snapshot(&scene.kernel), before, "a failed job must not touch the session");
                assert!(job.is_terminal());
                assert_eq!(scene.kernel.step_operation_job_sync(&mut job, budget).err(), Some(error.clone()), "a failed job keeps failing the same way");
                return Err(error);
            }
        };
        match step {
            BrepOperationStep::Working(now) => {
                assert_eq!(snapshot(&scene.kernel), before, "a working job must not touch the session");
                assert!(!job.is_terminal());
                progress.push(now);
            }
            BrepOperationStep::Ready(handles) => {
                assert!(job.is_terminal());
                progress.push(job.progress());
                job.cancel();
                match scene.kernel.step_operation_job_sync(&mut job, budget).unwrap() {
                    BrepOperationStep::Ready(again) => assert_eq!(again, handles, "a ready job stays ready and cannot be cancelled"),
                    _ => panic!("a finished job must not be cancellable"),
                }
                return Ok(Trace { handles, calls, progress });
            }
            BrepOperationStep::Cancelled(_) => panic!("nothing cancelled this job"),
        }
    }
}

/// 📈️ Asserts progress is monotone, bounded by its total and complete when the job is ready. The plan
/// only grows while the job runs; it may shrink once, on the last observation, when a nested boolean
/// planned pessimistically.
fn assert_progress(id: &str, progress: &[BrepOperationProgress]) {
    for pair in progress.windows(2) {
        assert!(pair[1].done >= pair[0].done, "{id}: done went backwards {:?} -> {:?}", pair[0], pair[1]);
    }
    for now in progress {
        assert!(now.total >= now.done, "{id}: done beyond total {now:?}");
    }
    for pair in progress[..progress.len() - 1].windows(2) {
        assert!(pair[1].total >= pair[0].total, "{id}: the plan shrank before the end {:?} -> {:?}", pair[0], pair[1]);
    }
    let last = progress.last().unwrap();
    assert_eq!(last.done, last.total, "{id}: a ready job has done everything it planned");
}

/// 🧪️ Prints the measurements a fixture expectation is written from (run with `--ignored`).
#[test]
#[ignore = "prints fixture expectations for authoring"]
fn record_fixture_expectations() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) {
            continue;
        }
        let started = std::time::Instant::now();
        let mut scene = Scene::new(case);
        let operation = scene.operation(case);
        let before = snapshot(&scene.kernel);
        match drive(&mut scene, operation, 1, &before) {
            Ok(trace) => {
                let operation_time = started.elapsed();
                let measured = scene.measure(&trace.handles, mesh_measure(case));
                println!("[DEBUG] record {id} {measured} calls={} total={} operation={operation_time:?} measured={:?}", trace.calls, trace.progress.last().unwrap().total, started.elapsed());
            }
            Err(error) => println!("[DEBUG] record {id} ERR {error} {:?}", started.elapsed()),
        }
    }
}

/// 🔁️ The one-shot facade of every operation reproduces the fixture.
#[test]
fn oneshot_facade_matches_fixture() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) {
            continue;
        }
        let mut scene = Scene::new(case);
        let result = scene.oneshot(case);
        if case["expect"]["error"].is_string() {
            assert!(result.is_err(), "{id}: expected a refusal");
            continue;
        }
        let outputs = result.unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_matches(id, &case["expect"], &scene.measure(&outputs, mesh_measure(case)));
    }
}

/// 📐️ The exact surface integrals of the curved and free-form results equal the ones the original
/// one-shot implementation produced (recorded in the fixture before the jobs replaced it). Exact adaptive
/// integration of these shapes costs minutes in a debug build, so this runs on demand.
#[test]
#[ignore = "exact adaptive integration of curved results is slow in debug builds"]
fn exact_integrals_of_curved_results_match_the_original_implementation() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) || case.get("exact").is_none() {
            continue;
        }
        let mut scene = Scene::new(case);
        let operation = scene.operation(case);
        let outputs = scene.kernel.run_operation_sync(operation).unwrap_or_else(|error| panic!("{id}: {error}"));
        let measured = scene.measure(&outputs, false);
        for key in ["volume", "area"] {
            let (actual, wanted) = (measured[key].as_f64().unwrap(), case["exact"][key].as_f64().unwrap());
            assert!((actual - wanted).abs() <= 1e-4 * wanted.abs().max(1.0), "{id}: exact {key} {actual} vs original {wanted}");
        }
    }
}

/// ⏱️ Driving the job one unit at a time reproduces the one-shot result, never touches the session
/// before it is ready, reports monotone progress that completes, and stays inside the unit bounds.
#[test]
fn job_driven_one_unit_at_a_time_matches_oneshot() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) {
            continue;
        }
        let mut scene = Scene::new(case);
        let operation = scene.operation(case);
        let before = snapshot(&scene.kernel);
        let result = drive(&mut scene, operation, 1, &before);
        if let Some(kind) = case["expect"]["error"].as_str() {
            let error = result.err().unwrap_or_else(|| panic!("{id}: expected a refusal"));
            assert!(matches!((kind, &error), ("operation", BrepError::Operation(_)) | ("invalidInput", BrepError::InvalidInput(_))), "{id}: {error}");
            continue;
        }
        let trace = result.unwrap_or_else(|error| panic!("{id}: {error}"));
        assert_progress(id, &trace.progress);
        assert_matches(id, &case["expect"], &scene.measure(&trace.handles, mesh_measure(case)));
        let units = &case["expect"]["units"];
        let total = trace.progress.last().unwrap().total;
        assert!(trace.calls == total || total == 0, "{id}: budget 1 takes exactly one call per unit ({} calls, {total} units)", trace.calls);
        assert!(total as u64 >= units["min"].as_u64().unwrap() && total as u64 <= units["max"].as_u64().unwrap(), "{id}: {total} units outside {units}");
        if let Some(oracle) = case.get("oracle") {
            let measured = scene.measure(&trace.handles, mesh_measure(case))["volume"].as_f64().unwrap();
            let wanted = parry_volume(oracle);
            assert!((measured - wanted).abs() <= oracle["tolerance"].as_f64().unwrap(), "{id}: parry3d volume {wanted} vs job {measured}");
        }
    }
}

/// ⏱️ The budget never changes what a job computes: every budget leaves the session byte-identical to the
/// one-unit-at-a-time run (handles, ids, labels and geometry), so a host may pick any step size.
#[test]
fn job_budget_does_not_change_the_result() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) || case["heavy"].as_bool().unwrap_or(false) || case["expect"]["error"].is_string() {
            continue;
        }
        let mut reference = None;
        for budget in [1, 3, 1000, usize::MAX] {
            let mut scene = Scene::new(case);
            let operation = scene.operation(case);
            let before = snapshot(&scene.kernel);
            let trace = drive(&mut scene, operation, budget, &before).unwrap_or_else(|error| panic!("{id}: {error}"));
            assert_progress(id, &trace.progress);
            let after = snapshot(&scene.kernel);
            assert_ne!(after, before, "{id}: a finished job commits its result");
            assert_eq!(reference.get_or_insert_with(|| (after.clone(), trace.handles.clone())), &(after, trace.handles), "{id}: budget {budget} changed the result");
        }
    }
}

/// 🛑️ Cancelling after any number of units leaves the session byte-identical to before admission, the
/// job terminal and cancelled for good; and a session that was cancelled midway and then ran the operation
/// to completion is byte-identical to one that never started the cancelled attempt.
#[test]
fn cancel_at_every_sampled_step_leaves_the_session_untouched() {
    for case in fixture()["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        if !selected(id) || case["expect"]["error"].is_string() {
            continue;
        }
        let total = case["expect"]["units"]["max"].as_u64().unwrap() as usize;
        let heavy = case["heavy"].as_bool().unwrap_or(false);
        let middle = total / 2;
        let indices: BTreeSet<usize> = if case["exhaustiveCancel"].as_bool().unwrap_or(false) { (0..total).collect() } else if heavy { BTreeSet::from([0, middle]) } else { BTreeSet::from([0, 1, middle, total.saturating_sub(1)]) };
        let reference = {
            let mut scene = Scene::new(case);
            let operation = scene.operation(case);
            scene.kernel.run_operation_sync(operation).unwrap_or_else(|error| panic!("{id}: {error}"));
            snapshot(&scene.kernel)
        };
        for index in indices {
            let mut scene = Scene::new(case);
            let operation = scene.operation(case);
            let before = snapshot(&scene.kernel);
            let handles_before = scene.kernel.live_handles();
            let BrepOperationAdmission::Job(mut job) = scene.kernel.operation_job_sync(operation).unwrap() else { continue };
            let mut ran = 0;
            while ran < index {
                match scene.kernel.step_operation_job_sync(&mut job, 1).unwrap() {
                    BrepOperationStep::Working(_) => ran += 1,
                    BrepOperationStep::Ready(_) => break,
                    BrepOperationStep::Cancelled(_) => panic!("{id}: cancelled itself"),
                }
            }
            if job.is_terminal() {
                continue;
            }
            let progress = job.progress();
            job.cancel();
            assert!(job.is_terminal(), "{id}@{index}");
            match scene.kernel.step_operation_job_sync(&mut job, usize::MAX).unwrap() {
                BrepOperationStep::Cancelled(last) => assert_eq!(last, progress, "{id}@{index}: a cancelled job keeps its last progress"),
                _ => panic!("{id}@{index}: a cancelled job stays cancelled"),
            }
            assert_eq!(snapshot(&scene.kernel), before, "{id}@{index}: cancel must leave the session byte-identical");
            assert_eq!(scene.kernel.live_handles(), handles_before, "{id}@{index}: no handle appeared");
            if index == middle {
                let operation = scene.operation(case);
                scene.kernel.run_operation_sync(operation).unwrap_or_else(|error| panic!("{id}@{index}: rerun {error}"));
                assert_eq!(snapshot(&scene.kernel), reference, "{id}@{index}: a cancelled attempt leaves no trace in a later run");
            }
        }
    }
}

/// 🛡️ Refused requests and failing jobs leave the session untouched, whether they fail at admission
/// or on a later unit.
#[test]
fn refusals_and_failures_precede_any_mutation() {
    for case in fixture()["refusals"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let mut scene = Scene::new(case);
        let operation = scene.operation(case);
        let before = snapshot(&scene.kernel);
        let error = drive(&mut scene, operation, 1, &before).err().unwrap_or_else(|| panic!("{id}: expected a refusal"));
        assert!(matches!((case["error"].as_str().unwrap(), &error), ("operation", BrepError::Operation(_)) | ("invalidInput", BrepError::InvalidInput(_)) | ("missingHandle", BrepError::MissingHandle(_))), "{id}: {error}");
        assert_eq!(snapshot(&scene.kernel), before, "{id}: refusal must precede mutation");
    }
}

/// 🧭️ The fixture names every operation the descriptor can carry, and the descriptor tags agree.
#[test]
fn fixture_covers_every_operation() {
    let fixture = fixture();
    let all = [
        "fillet", "filletVariable", "filletEdges", "chamfer", "chamferAsymmetric", "chamferEdges", "shell", "draft", "offsetSolid", "offsetFace", "thickenFace", "defeature", "extrude", "extrudeWire", "revolve", "sweep", "pipe", "helicalSweep", "loft", "section", "split", "compoundCut", "linearPattern", "circularPattern", "gridPattern", "convexHull",
    ];
    let mut covered = BTreeSet::new();
    for case in fixture["cases"].as_array().unwrap() {
        let kind = case["operation"]["kind"].as_str().unwrap();
        assert_eq!(Scene::new(case).operation(case).tag(), kind);
        covered.insert(kind);
    }
    for kind in all {
        assert!(covered.contains(kind), "fixture lacks {kind}");
    }
}

/// 🏷️ A job's result never reuses a label an existing entity carries: inputs that survive into the
/// result (the profile cap of an extrusion, the solid a defeature edits) are re-labelled on commit.
#[test]
fn committed_results_never_share_a_label_with_existing_entities() {
    for id in ["extrude-rectangle-face", "defeature-hull-smallest-face", "loft-two-squares-ruled", "revolve-offset-face-half-turn", "sweep-square-bent-path"] {
        let fixture = fixture();
        let case = fixture["cases"].as_array().unwrap().iter().find(|case| case["id"] == id).unwrap();
        if !selected(id) {
            continue;
        }
        let mut scene = Scene::new(case);
        let operation = scene.operation(case);
        let outputs = scene.kernel.run_operation_sync(operation).unwrap();
        let body = &scene.kernel.body;
        let labels: Vec<u64> = body.vertices.iter().map(|(_, v)| v.label.0).chain(body.edges.iter().map(|(_, e)| e.label.0)).chain(body.faces.iter().map(|(_, f)| f.label.0)).chain(body.shells.iter().map(|(_, s)| s.label.0)).chain(body.solids.iter().map(|(_, s)| s.label.0)).collect();
        let unique: BTreeSet<u64> = labels.iter().copied().collect();
        assert_eq!(unique.len(), labels.len(), "{id}: two live entities share a label");
        assert!(!outputs.is_empty());
        for input in scene.inputs.values().flatten() {
            assert!(scene.kernel.kind_sync(input).is_ok(), "{id}: the input handle survives");
        }
    }
}

/// ⚡️ A pattern of fewer than two instances is the identity and needs no job.
#[test]
fn single_instance_patterns_answer_immediately() {
    let mut kernel = Brep::new();
    let shape = kernel.box_prim_sync(1.0, 1.0, 1.0).unwrap();
    let before = snapshot(&kernel);
    for operation in [
        BrepOperation::LinearPattern { shape: shape.clone(), direction: [1.0, 0.0, 0.0], spacing: 2.0, count: 1 },
        BrepOperation::CircularPattern { shape: shape.clone(), axis: [0.0, 0.0, 1.0], count: 0 },
        BrepOperation::GridPattern { shape: shape.clone(), dir_x: [1.0, 0.0, 0.0], dir_y: [0.0, 1.0, 0.0], spacing_x: 2.0, spacing_y: 2.0, count_x: 1, count_y: 1 },
    ] {
        match kernel.operation_job_sync(operation).unwrap() {
            BrepOperationAdmission::Answered(handles) => assert_eq!(handles, vec![shape.clone()]),
            BrepOperationAdmission::Job(_) => panic!("an identity pattern needs no job"),
        }
    }
    assert_eq!(snapshot(&kernel), before);
}

/// 🔒️ `Body::extract` copies exactly what the roots reach with every label kept, and `Body::absorb` gives
/// each surviving copy a fresh label while leaving everything already in the body untouched.
#[test]
fn extract_and_absorb_isolate_a_working_copy() {
    let mut kernel = Brep::new();
    let kept = kernel.box_prim_sync(1.0, 2.0, 3.0).unwrap();
    let other = kernel.box_prim_sync(4.0, 4.0, 4.0).unwrap();
    let solid = kernel.solid_id(&kept).unwrap();
    let before = snapshot(&kernel);
    let base = kernel.body.labels.next();
    let (scratch, map) = kernel.body.extract(&[EntityRef::Solid(solid)]);
    assert_eq!(snapshot(&kernel), before, "extracting reads the body only");
    assert_eq!((scratch.solids.len(), scratch.faces.len(), scratch.edges.len(), scratch.vertices.len()), (1, 6, 12, 8), "only what the box reaches is copied");
    assert_eq!(scratch.labels.next(), base, "the copy's label counter continues at the body's");
    for (old, new) in &map.faces {
        assert_eq!(scratch.faces.get(*new).unwrap().label, kernel.body.faces.get(*old).unwrap().label, "labels are preserved in the copy");
    }
    let other_solid = kernel.solid_id(&other).unwrap();
    let other_label = kernel.body.solids.get(other_solid).unwrap().label;
    let absorbed = kernel.body.absorb(&scratch, &[EntityRef::Solid(map.solids[&solid])], base);
    let copy = absorbed.solids[&map.solids[&solid]];
    assert_ne!(kernel.body.solids.get(copy).unwrap().label, kernel.body.solids.get(solid).unwrap().label, "a surviving copy never shares its source's label");
    assert_eq!(kernel.body.solids.get(other_solid).unwrap().label, other_label, "existing entities are untouched");
    let body = &kernel.body;
    let labels: Vec<u64> = body.vertices.iter().map(|(_, v)| v.label.0).chain(body.edges.iter().map(|(_, e)| e.label.0)).chain(body.faces.iter().map(|(_, f)| f.label.0)).chain(body.shells.iter().map(|(_, s)| s.label.0)).chain(body.solids.iter().map(|(_, s)| s.label.0)).collect();
    assert_eq!(labels.iter().collect::<BTreeSet<_>>().len(), labels.len(), "no two live entities share a label");
}
