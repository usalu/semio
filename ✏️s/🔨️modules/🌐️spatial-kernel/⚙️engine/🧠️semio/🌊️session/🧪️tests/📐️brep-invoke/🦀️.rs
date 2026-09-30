//! 🌉️ The `brep_invoke` bridge law — the wire every CAD `SemioBrepKernel` call rides (`✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🟦️.ts`
//! → `flow_core` wasm → [`brep_invoke_json`]), answered from its declarations:
//!
//! 1. The verb catalog `📐️brep-geometry/🔣️.json` (schema `📐️brep-geometry/🧬️schema/🔣️.json`) IS the dispatcher, arm for arm: the
//!    same methods, each reading exactly the declared arguments in the declared order with the declared type and default,
//!    answering the declared result shape and wrapping a real `BrepKernel` operation. A verb the TS kernel calls but the
//!    dispatcher lacks (`translate`/`rotate` until 2026-09-29) fails here instead of in a browser.
//! 2. Every verb dispatches: an empty argument object is refused as a missing argument, never as an unknown method.
//! 3. The affine-transform vectors `🧊️brep/🧫️fixtures/🔁️affine-transforms/🔣️.json` answer over the wire as the kernel law answers
//!    them (the same measurements, taken through the wire's own verbs), and every degenerate step is refused as invalid input.
//!
//! Ticket 26/09/23 END-TO-END-OS-HUB-COLLABORATION-MCP (slice CD1).

// #region 🔖️Imports
use semio_s_spatial_kernel_semio_session::Session;
fn session() -> &'static Session {
    static SESSION: std::sync::OnceLock<Session> = std::sync::OnceLock::new();
    SESSION.get_or_init(Session::new)
}
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::BREP_KERNEL_OPERATIONS;
use serde_json::{json, Value};
use std::collections::BTreeMap;
// #endregion 🔖️Imports

// #region 🔖️Declarations
const CATALOG: &str = include_str!("../../🔣️.json");
const DISPATCHER: &str = include_str!("../../🦀️.rs");
const AFFINE_FIXTURE: &str = include_str!("../../../../../../../🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧫️fixtures/🔁️affine-transforms/🔣️.json");

/// 🧾️ One verb as either side declares it: kernel operation, arguments `(name, type, default)` in read order, result shape.
#[derive(Debug, PartialEq)]
struct Verb {
    kernel_operation: String,
    args: Vec<(String, String, Option<Value>)>,
    result: String,
}

fn catalog() -> BTreeMap<String, Verb> {
    let root: Value = serde_json::from_str(CATALOG).expect("verb catalog parses");
    assert_eq!(root["schema"].as_str(), Some("semio.flow.brep-invoke.verbs/v1"), "verb catalog schema");
    let mut verbs = BTreeMap::new();
    for row in root["verbs"].as_array().expect("verbs") {
        let method = row["method"].as_str().expect("method").to_string();
        let args = row["args"].as_array().expect("args").iter().map(|arg| (arg["name"].as_str().expect("arg name").to_string(), arg["type"].as_str().expect("arg type").to_string(), arg.get("default").cloned())).collect();
        let verb = Verb { kernel_operation: row["kernelOperation"].as_str().expect("kernelOperation").to_string(), args, result: row["result"].as_str().expect("result").to_string() };
        assert!(verbs.insert(method.clone(), verb).is_none(), "verb {method} declared twice");
    }
    assert!(!verbs.is_empty(), "the catalog declares at least one verb");
    verbs
}

/// 🔍️ The dispatcher's arms as its source declares them — method literal, first `guard.<operation>(` call, every
/// `arg_<kind>(&args, "<name>"[, default])` read in order, and the result helper it maps into.
fn dispatcher() -> BTreeMap<String, Verb> {
    let start = DISPATCHER.find("fn brep_invoke_inner(").expect("brep_invoke_inner");
    let end = start + DISPATCHER[start..].find("other => Err(BrepModuleError::UnknownMethod").expect("unknown-method arm");
    let body = &DISPATCHER[start..end];
    let mut arms = BTreeMap::new();
    let mut pieces = body.split("\n        \"").skip(1);
    for piece in pieces.by_ref() {
        let Some((method, arm)) = piece.split_once("\" => {") else { continue };
        let operation = arm.split("guard").skip(1).map(str::trim_start).find_map(|rest| rest.strip_prefix('.')).and_then(|rest| rest.split('(').next()).expect("arm calls a kernel operation").to_string();
        let mut args = Vec::new();
        for read in arm.split("arg_").skip(1) {
            let Some((kind, rest)) = read.split_once("(&args, \"") else { continue };
            let (name, tail) = rest.split_once('"').expect("argument name literal");
            let kind_type = match kind.trim_end_matches("_or") {
                "f64" => "number",
                "usize" => "integer",
                "bool" => "boolean",
                "string" => "string",
                "vec3" => "vec3",
                "points" => "points",
                "handle" => "handle",
                "handles" => "handles",
                other => panic!("{method}: unknown argument reader arg_{other}"),
            };
            let default = kind.ends_with("_or").then(|| {
                let literal = tail.trim_start_matches(", ").split(')').next().expect("default literal").trim();
                match literal {
                    "true" => Value::Bool(true),
                    "false" => Value::Bool(false),
                    number => json!(number.parse::<f64>().expect("numeric default")),
                }
            });
            args.push((name.to_string(), kind_type.to_string(), default));
        }
        let result = [("handles_result", "handles"), ("handle_result", "handle"), ("number_result", "number"), ("vec3_result", "vec3"), ("string_result", "string"), ("mesh_result", "mesh"), ("topology_result", "topology"), ("unit_result", "unit")]
            .into_iter()
            .find(|(helper, _)| arm.contains(helper))
            .map(|(_, shape)| shape.to_string())
            .expect("arm maps into a result helper");
        assert!(arms.insert(method.to_string(), Verb { kernel_operation: operation, args, result }).is_none(), "arm {method} dispatched twice");
    }
    arms
}

fn normalized(verbs: BTreeMap<String, Verb>) -> BTreeMap<String, Verb> {
    verbs.into_iter().map(|(method, verb)| (method, Verb { args: verb.args.into_iter().map(|(name, kind, default)| (name, kind, default.map(|value| value.as_f64().map_or(value.clone(), |number| json!(number))))).collect(), ..verb })).collect()
}
// #endregion 🔖️Declarations

// #region 🔖️Wire
fn invoke(method: &str, args: &Value) -> Result<Value, String> {
    let out: Value = serde_json::from_str(&session().brep_invoke_json(method, &args.to_string())).expect("brep_invoke answers JSON");
    match out.get("error").and_then(Value::as_str) {
        Some(error) => Err(error.to_string()),
        None => Ok(out),
    }
}

fn without_kind(value: &Value) -> serde_json::Map<String, Value> {
    let mut map = value.as_object().expect("fixture object").clone();
    map.remove("kind");
    map
}

fn handle_of(out: Value) -> String {
    out["handle"].as_str().expect("handle result").to_string()
}

fn number(value: &Value, key: &str) -> f64 {
    value[key].as_f64().unwrap_or_else(|| panic!("fixture field {key} is a number"))
}

fn vec3(value: &Value) -> [f64; 3] {
    let items = value.as_array().expect("3-number array");
    assert_eq!(items.len(), 3, "3-number array");
    [items[0].as_f64().expect("x"), items[1].as_f64().expect("y"), items[2].as_f64().expect("z")]
}

/// 📐️ The wire's own measurements of one shape: `volume`, `centerOfMass`, the `tessellate` extent and planar faces' normals,
/// and `deconstruct`'s face/edge/vertex counts.
struct WireMeasure {
    volume: f64,
    center_of_mass: [f64; 3],
    min: [f64; 3],
    max: [f64; 3],
    normals: Vec<[f64; 3]>,
    topology: [usize; 3],
}

fn measure(shape: &str, tessellation: f64) -> WireMeasure {
    let volume = invoke("volume", &json!({ "shape": shape })).expect("volume")["value"].as_f64().expect("volume value");
    let center_of_mass = vec3(&invoke("centerOfMass", &json!({ "shape": shape })).expect("centerOfMass")["value"]);
    let mesh = invoke("tessellate", &json!({ "shape": shape, "tolerance": tessellation })).expect("tessellate");
    let positions: Vec<f64> = mesh["position"].as_array().expect("positions").iter().map(|value| value.as_f64().expect("position")).collect();
    let mut min = [f64::INFINITY; 3];
    let mut max = [f64::NEG_INFINITY; 3];
    for point in positions.chunks_exact(3) {
        for axis in 0..3 {
            min[axis] = min[axis].min(point[axis]);
            max[axis] = max[axis].max(point[axis]);
        }
    }
    let normals = mesh["face_infos"].as_array().expect("face infos").iter().map(|info| vec3(&info["normal"])).collect();
    let topology = invoke("deconstruct", &json!({ "shape": shape })).expect("deconstruct");
    let count = |key: &str| topology[key].as_array().expect("topology list").len();
    WireMeasure { volume, center_of_mass, min, max, normals, topology: [count("faces"), count("edges"), count("vertices")] }
}
// #endregion 🔖️Wire

// #region 🔖️Laws
/// 🧾️ LAW: the verb catalog and the dispatcher declare the same verbs, argument for argument, and every verb wraps a real
/// `BrepKernel` operation.
#[test]
fn the_verb_catalog_is_the_dispatcher_arm_for_arm() {
    let declared = normalized(catalog());
    let dispatched = normalized(dispatcher());
    let missing: Vec<&String> = declared.keys().filter(|method| !dispatched.contains_key(*method)).collect();
    let undeclared: Vec<&String> = dispatched.keys().filter(|method| !declared.contains_key(*method)).collect();
    assert!(missing.is_empty() && undeclared.is_empty(), "catalog-only verbs {missing:?}, dispatcher-only arms {undeclared:?}");
    for (method, verb) in &declared {
        assert_eq!(Some(verb), dispatched.get(method), "{method}: catalog vs dispatcher");
        assert!(BREP_KERNEL_OPERATIONS.contains(&verb.kernel_operation.as_str()), "{method}: {} is not a BrepKernel operation", verb.kernel_operation);
    }
}

/// 🚪️ LAW: every declared verb dispatches — an empty argument object is refused as a missing argument, never as an unknown
/// method — and an undeclared method is refused as unknown.
#[test]
fn every_declared_verb_dispatches_and_refuses_a_missing_argument() {
    for (method, verb) in catalog() {
        if verb.args.iter().all(|(_, _, default)| default.is_some()) {
            continue;
        }
        let refusal = invoke(&method, &json!({})).expect_err("an empty argument object must be refused");
        assert!(refusal.starts_with("invalid brep_invoke args: missing"), "{method}: {refusal}");
    }
    assert_eq!(invoke("frobnicate", &json!({})).expect_err("undeclared"), "unknown brep_invoke method: frobnicate");
}

/// 🔁️ LAW: every affine-transform vector answers over the wire — volume, centre of mass, tessellation extent, planar faces'
/// outward normals and preserved topology — and every degenerate step is refused as invalid input.
#[test]
fn every_affine_transform_vector_answers_over_the_wire() {
    let root: Value = serde_json::from_str(AFFINE_FIXTURE).expect("affine-transforms fixture parses");
    assert_eq!(root["schema"].as_str(), Some("s.stdio.semio.brep.affine-transforms/v1"), "fixture schema");
    let tessellation = number(&root, "tessellationTolerance");
    let volume_tolerance = number(&root, "volumeRelativeTolerance");
    let center_tolerance = number(&root, "centerOfMassTolerance");
    let bounds_tolerance = number(&root, "boundsTolerance");
    let normal_tolerance = number(&root, "normalTolerance");
    let mut failures = Vec::new();
    for case in root["cases"].as_array().expect("cases") {
        let id = case["id"].as_str().expect("case id");
        let expect = &case["expect"];
        let solid = &case["solid"];
        let mut shape = handle_of(invoke(solid["kind"].as_str().expect("solid kind"), &Value::Object(without_kind(solid))).expect("primitive"));
        let before = measure(&shape, tessellation);
        for step in case["steps"].as_array().expect("steps") {
            let mut args = without_kind(step);
            args.insert("shape".into(), Value::String(shape.clone()));
            shape = handle_of(invoke(step["kind"].as_str().expect("step kind"), &Value::Object(args)).unwrap_or_else(|error| panic!("{id}: {error}")));
        }
        let after = measure(&shape, tessellation);
        let volume = number(expect, "volume");
        let mut near = |label: String, got: f64, want: f64, within: f64| {
            if !((got - want).abs() <= within) {
                failures.push(format!("{id}: {label} {got} != {want} (±{within})"));
            }
        };
        near("volume".into(), after.volume, volume, volume_tolerance * volume.abs());
        let center = vec3(&expect["centerOfMass"]);
        let min = vec3(&expect["bounds"]["min"]);
        let max = vec3(&expect["bounds"]["max"]);
        for axis in 0..3 {
            near(format!("centerOfMass[{axis}]"), after.center_of_mass[axis], center[axis], center_tolerance);
            near(format!("bounds.min[{axis}]"), after.min[axis], min[axis], bounds_tolerance);
            near(format!("bounds.max[{axis}]"), after.max[axis], max[axis], bounds_tolerance);
        }
        if before.topology != after.topology {
            failures.push(format!("{id}: faces/edges/vertices {:?} -> {:?}", before.topology, after.topology));
        }
        if let Some(normals) = expect["faceNormals"].as_array() {
            let mut unmatched = after.normals.clone();
            for want in normals.iter().map(vec3) {
                match unmatched.iter().position(|got| (0..3).all(|axis| (got[axis] - want[axis]).abs() <= normal_tolerance)) {
                    Some(index) => {
                        unmatched.remove(index);
                    }
                    None => failures.push(format!("{id}: outward face normal {want:?} missing from {:?}", after.normals)),
                }
            }
            if !unmatched.is_empty() {
                failures.push(format!("{id}: unexpected face normals {unmatched:?}"));
            }
        }
    }
    for refusal in root["refusals"].as_array().expect("refusals") {
        let id = refusal["id"].as_str().expect("refusal id");
        let solid = &refusal["solid"];
        let shape = handle_of(invoke(solid["kind"].as_str().expect("solid kind"), &Value::Object(without_kind(solid))).expect("primitive"));
        let mut args = without_kind(&refusal["step"]);
        args.insert("shape".into(), Value::String(shape));
        match invoke(refusal["step"]["kind"].as_str().expect("step kind"), &Value::Object(args)) {
            Err(error) if error.contains("invalid input") => {}
            other => failures.push(format!("{id}: expected an invalid-input refusal, got {other:?}")),
        }
    }
    assert!(failures.is_empty(), "{failures:#?}");
}
// #endregion 🔖️Laws
