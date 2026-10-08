//! ⚡️ Semio object artifact — hand-rolled `OpText` for `SemioObjectMutation`. `#[derive(dsl::
//! Mutations)]` only generates `Mutation`/`SemanticMutation` — the wire-text codec stays
//! handcrafted here, one keyword per semantic verb, grammar `keyword:arg1,arg2,...`.

use crate::standards::v1::subsets::object::schema::mutations::SemioObjectMutation;

use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint3, SemioQuaternion};
use crate::standards::v1::subsets::object::schema::mutations::{
    create_brep::CreateBrep, create_mesh::CreateMesh, create_properties::CreateProperties, delete_brep::DeleteBrep, delete_mesh::DeleteMesh, delete_properties::DeleteProperties, move_object::MoveObject, rotate_object::RotateObject,
    scale_object::ScaleObject,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Primitives
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if !s.len().is_multiple_of(2) {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_f64(s: &str) -> Result<f64, String> {
    s.trim().parse::<f64>().map_err(|e| e.to_string())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn enc_ref(r: &semio_framework_artifact_reference::ArtifactRef) -> String {
    crate::standards::v1::subsets::object::io::text::snapshot::enc_ref(r)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn dec_ref(s: &str) -> Result<semio_framework_artifact_reference::ArtifactRef, String> {
    crate::standards::v1::subsets::object::io::text::snapshot::dec_ref(s)
}
//#endregion 🔖️Primitives

//#region 🔖️OpText
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn print_object_mutation(m: &SemioObjectMutation) -> String {
    match m {
        SemioObjectMutation::MoveObject(p) => format!("moveObject:{},{},{}", p.translation.x, p.translation.y, p.translation.z),
        SemioObjectMutation::RotateObject(p) => format!("rotateObject:{},{},{},{}", p.rotation.x, p.rotation.y, p.rotation.z, p.rotation.w),
        SemioObjectMutation::ScaleObject(p) => format!("scaleObject:{},{},{}", p.scale.x, p.scale.y, p.scale.z),
        SemioObjectMutation::CreateBrep(p) => format!("createBrep:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioObjectMutation::DeleteBrep(_) => "deleteBrep".to_string(),
        SemioObjectMutation::CreateMesh(p) => format!("createMesh:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioObjectMutation::DeleteMesh(_) => "deleteMesh".to_string(),
        SemioObjectMutation::CreateProperties(p) => format!("createProperties:{},{}", enc_str(&p.child_id), enc_ref(&p.target)),
        SemioObjectMutation::DeleteProperties(_) => "deleteProperties".to_string(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_object_mutation(line: &str) -> Result<SemioObjectMutation, String> {
    if let Some(payload) = line.strip_prefix("setSnapshot:") {
        let bytes = hex_decode(payload)?;
        let json = String::from_utf8(bytes).map_err(|error| error.to_string())?;
        let parsed = semio_framework_pack_json::parse(&json, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        let snapshot = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(&parsed)).map_err(|error| error.to_string())?;
    }
    if line == "deleteBrep" {
        return Ok(SemioObjectMutation::DeleteBrep(DeleteBrep {}));
    }
    if line == "deleteMesh" {
        return Ok(SemioObjectMutation::DeleteMesh(DeleteMesh {}));
    }
    if line == "deleteProperties" {
        return Ok(SemioObjectMutation::DeleteProperties(DeleteProperties {}));
    }
    let (tag, rest) = line.split_once(':').ok_or_else(|| format!("object mutation: missing ':' in {line:?}"))?;
    match tag {
        "moveObject" => {
            let parts: Vec<&str> = rest.split(',').collect();
            let [x, y, z] = parts.as_slice() else { return Err("moveObject: expected 3 fields".to_string()) };
            Ok(SemioObjectMutation::MoveObject(MoveObject { translation: SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? } }))
        }
        "rotateObject" => {
            let parts: Vec<&str> = rest.split(',').collect();
            let [x, y, z, w] = parts.as_slice() else { return Err("rotateObject: expected 4 fields".to_string()) };
            Ok(SemioObjectMutation::RotateObject(RotateObject { rotation: SemioQuaternion { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)?, w: parse_f64(w)? } }))
        }
        "scaleObject" => {
            let parts: Vec<&str> = rest.split(',').collect();
            let [x, y, z] = parts.as_slice() else { return Err("scaleObject: expected 3 fields".to_string()) };
            Ok(SemioObjectMutation::ScaleObject(ScaleObject { scale: SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? } }))
        }
        "createBrep" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createBrep: missing comma".to_string())?;
            Ok(SemioObjectMutation::CreateBrep(CreateBrep { child_id: dec_str(child_id)?, target: dec_ref(target)? }))
        }
        "createMesh" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createMesh: missing comma".to_string())?;
            Ok(SemioObjectMutation::CreateMesh(CreateMesh { child_id: dec_str(child_id)?, target: dec_ref(target)? }))
        }
        "createProperties" => {
            let (child_id, target) = rest.split_once(',').ok_or_else(|| "createProperties: missing comma".to_string())?;
            Ok(SemioObjectMutation::CreateProperties(CreateProperties { child_id: dec_str(child_id)?, target: dec_ref(target)? }))
        }
        other => Err(format!("object mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for SemioObjectMutation {
    fn print_op(&self) -> String {
        print_object_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_object_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️DemoCases
/// 🌱 One representative value per variant — single source of truth for
/// `ops_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_mutation_cases() -> Vec<SemioObjectMutation> {
    let ref_of = |subset: &str, id: &str| semio_framework_artifact_reference::ArtifactRef { artifact_id: id.into(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() } };
    vec![
        SemioObjectMutation::MoveObject(MoveObject { translation: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } }),
        SemioObjectMutation::RotateObject(RotateObject { rotation: SemioQuaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }),
        SemioObjectMutation::ScaleObject(ScaleObject { scale: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } }),
        SemioObjectMutation::CreateBrep(CreateBrep { child_id: "t1".into(), target: ref_of("brep", "t1") }),
        SemioObjectMutation::DeleteBrep(DeleteBrep {}),
        SemioObjectMutation::CreateMesh(CreateMesh { child_id: "t2".into(), target: ref_of("mesh", "t2") }),
        SemioObjectMutation::DeleteMesh(DeleteMesh {}),
        SemioObjectMutation::CreateProperties(CreateProperties { child_id: "t3".into(), target: ref_of("value", "t3") }),
        SemioObjectMutation::DeleteProperties(DeleteProperties {}),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::object::schema::mutations::*;
use crate::standards::v1::subsets::object::schema::diff::SemioObjectDiff;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::object::schema::mutations::create_brep;
use crate::standards::v1::subsets::object::schema::mutations::create_mesh;
use crate::standards::v1::subsets::object::schema::mutations::create_properties;
use crate::standards::v1::subsets::object::schema::mutations::delete_brep;
use crate::standards::v1::subsets::object::schema::mutations::delete_mesh;
use crate::standards::v1::subsets::object::schema::mutations::delete_properties;
use crate::standards::v1::subsets::object::schema::mutations::move_object;
use crate::standards::v1::subsets::object::schema::mutations::rotate_object;
use crate::standards::v1::subsets::object::schema::mutations::scale_object;

/// 📥️ Decodes this facet's own externally-tagged (`{"<VariantName>": {<snake_case payload>}}`)
/// JSON projection — no `#[value(rename_all)]` sits on this enum or its payload structs, which is
/// exactly the shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json` vectors
/// carry — into a real [`SemioObjectMutation`]. `create-brep`/`create-mesh`/`create-properties` carry a `store::os_io::ArtifactRef` `target`
/// field, the exact value an external caller cannot construct by hand; decoding the committed vector
/// is what turns those six child-lifecycle kinds from unreachable into exercisable.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_semio_object_mutation_json(text: &str) -> Result<SemioObjectMutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;
