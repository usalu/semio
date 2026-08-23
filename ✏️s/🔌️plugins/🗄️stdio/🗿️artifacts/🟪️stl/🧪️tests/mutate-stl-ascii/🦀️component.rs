//! 🦀️ STL ascii mutation case — Rust adapter.
//!
//! Every scenario copies the real, derived-once fixture into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `stl_io` reference
//! implementation (`../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`), `subject`
//! drives this repository's own decode/apply/encode round trip, and both results are read back by
//! the independent `stl_io` reader before the `semantic-mesh-v1` profile compares them. The subject
//! half is gated behind the generated host's `sut` feature so the oracle-only run never compiles the
//! local implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::stl::standards::v_ascii::subsets::any::{oracle_apply_mutation, oracle_inverse_spec, oracle_round_trip};
use semio_s_plugin_stdio_test_oracle::mesh::project_stl;

//#region 🔖️Kinds
/// 🧾️ Mirrors `StlMutation::KINDS` (`../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`)
/// — kept in sync by the contract phase's `mutation-kind-uncovered`/`mutation-kind-undeclared`
/// gates, which fail loudly if this list and the catalog ever drift apart.
const KINDS: [&str; 7] = ["no-mutation", "set-snapshot", "set-solid-name", "insert-triangle", "remove-triangle", "set-triangle-normal", "set-triangle-vertices"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://🧊️hexagonal-cut-concrete-forest-left.stl";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.stl"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_stl(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ Applies `<id>` forward, then its independently computed inverse — both against the SAME
/// untouched `input`, matching `StlMutation::inverse()`'s own base-relative semantics.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = oracle_inverse_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_stl(&restored)?;
    Ok(Outcome::with_raw(restored, projection))
}

fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_round_trip(&input)?;
    if bytes == input {
        return Err("byte pass-through: oracle output is bit-identical to the input".to_string());
    }
    let projection = project_stl(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::subsets::any::io::{decode_stl_ascii, encode_stl_ascii};
    use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::subsets::any::schema::mutations::{apply_stl_mutation, StlMutation};
    use semio_s_plugin_stdio::artifacts::stl::standards::v_ascii::subsets::any::schema::snapshot::{StlSnapshot, StlTriangle};
    use semio_s_plugin_stdio_test_oracle::mesh::project_stl;

    //#region 🔖️SpecReaders
    fn params_of(spec: &Json) -> Json {
        spec.get("params").cloned().unwrap_or(Json::Null)
    }
    fn number(value: &Json, key: &str) -> Option<f64> {
        match value.get(key) {
            Some(Json::Number(number)) => Some(*number),
            _ => None,
        }
    }
    fn string(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) => Some(text.clone()),
            _ => None,
        }
    }
    fn json_vec3(value: &Json) -> Option<[f64; 3]> {
        match value {
            Json::Array(items) if items.len() == 3 => {
                let mut out = [0f64; 3];
                for (slot, item) in out.iter_mut().zip(items.iter()) {
                    *slot = match item {
                        Json::Number(number) => *number,
                        _ => return None,
                    };
                }
                Some(out)
            }
            _ => None,
        }
    }
    fn vec3(value: &Json, key: &str) -> Option<[f64; 3]> {
        json_vec3(value.get(key)?)
    }
    fn vertices3(value: &Json, key: &str) -> Option<[[f64; 3]; 3]> {
        match value.get(key) {
            Some(Json::Array(items)) if items.len() == 3 => Some([json_vec3(&items[0])?, json_vec3(&items[1])?, json_vec3(&items[2])?]),
            _ => None,
        }
    }
    fn triangle_of(value: &Json) -> Option<StlTriangle> {
        Some(StlTriangle { normal: vec3(value, "normal")?, vertices: vertices3(value, "vertices")? })
    }
    fn triangles_of(value: &Json, key: &str) -> Option<Vec<StlTriangle>> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().map(triangle_of).collect(),
            _ => None,
        }
    }
    //#endregion 🔖️SpecReaders

    //#region 🔖️Mutation
    /// 🧭️ Builds the real `StlMutation` a spec describes. `set-snapshot` keeps `base`'s own
    /// `schema`/`solid_name` — the params only ever carry `triangles` (matching the oracle's
    /// `stl_io`-bounded set-snapshot, which cannot touch the name either).
    fn mutation_of(spec: &Json, base: &StlSnapshot) -> Result<StlMutation, String> {
        let params = params_of(spec);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => StlMutation::NoMutation,
            "set-solid-name" => StlMutation::SetSolidName { name: string(&params, "name").ok_or("set-solid-name: missing `name`")? },
            "insert-triangle" => StlMutation::InsertTriangle { index: number(&params, "index").ok_or("insert-triangle: missing `index`")? as usize, triangle: triangle_of(params.get("triangle").ok_or("insert-triangle: missing `triangle`")?).ok_or("insert-triangle: malformed `triangle`")? },
            "remove-triangle" => StlMutation::RemoveTriangle { index: number(&params, "index").ok_or("remove-triangle: missing `index`")? as usize },
            "set-triangle-normal" => StlMutation::SetTriangleNormal { index: number(&params, "index").ok_or("set-triangle-normal: missing `index`")? as usize, normal: vec3(&params, "normal").ok_or("set-triangle-normal: missing `normal`")? },
            "set-triangle-vertices" => StlMutation::SetTriangleVertices { index: number(&params, "index").ok_or("set-triangle-vertices: missing `index`")? as usize, vertices: vertices3(&params, "vertices").ok_or("set-triangle-vertices: missing `vertices`")? },
            "set-snapshot" => StlMutation::SetSnapshot { snapshot: StlSnapshot { schema: base.schema.clone(), solid_name: base.solid_name.clone(), triangles: triangles_of(&params, "triangles").ok_or("set-snapshot: missing/malformed `triangles`")? } },
            kind => return Err(format!("mutation kind {kind:?} is not implemented by the subject")),
        })
    }

    /// ↩️ Mirrors `StlMutation::inverse()` (`../../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`)
    /// independently: the generated oracle-role host never links `protocol`, so the trait method
    /// itself is unreachable here, and this reconstructs the same index-aware inverse by hand
    /// against the pre-mutation `base` — an out-of-range index inverts to `NoMutation`, exactly as
    /// that method does.
    fn inverse_of(spec: &Json, base: &StlSnapshot) -> Result<StlMutation, String> {
        let params = params_of(spec);
        Ok(match spec.str("kind").as_str() {
            "no-mutation" => StlMutation::NoMutation,
            "set-solid-name" => StlMutation::SetSolidName { name: base.solid_name.clone() },
            "insert-triangle" => StlMutation::RemoveTriangle { index: (number(&params, "index").ok_or("insert-triangle: missing `index`")? as usize).min(base.triangles.len()) },
            "remove-triangle" => {
                let index = number(&params, "index").ok_or("remove-triangle: missing `index`")? as usize;
                match base.triangles.get(index) {
                    Some(triangle) => StlMutation::InsertTriangle { index, triangle: *triangle },
                    None => StlMutation::NoMutation,
                }
            }
            "set-triangle-normal" => {
                let index = number(&params, "index").ok_or("set-triangle-normal: missing `index`")? as usize;
                match base.triangles.get(index) {
                    Some(triangle) => StlMutation::SetTriangleNormal { index, normal: triangle.normal },
                    None => StlMutation::NoMutation,
                }
            }
            "set-triangle-vertices" => {
                let index = number(&params, "index").ok_or("set-triangle-vertices: missing `index`")? as usize;
                match base.triangles.get(index) {
                    Some(triangle) => StlMutation::SetTriangleVertices { index, vertices: triangle.vertices },
                    None => StlMutation::NoMutation,
                }
            }
            "set-snapshot" => StlMutation::SetSnapshot { snapshot: base.clone() },
            kind => return Err(format!("mutation kind {kind:?} is not implemented by the subject")),
        })
    }
    //#endregion 🔖️Mutation

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let mut snapshot = decode_stl_ascii(&text).map_err(|error| format!("decode_stl_ascii failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let mutation = mutation_of(&spec, &snapshot)?;
        apply_stl_mutation(&mut snapshot, &mutation);
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        let projection = project_stl(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let text = String::from_utf8(mutable_input(ctx)?).map_err(|error| error.to_string())?;
        let base = decode_stl_ascii(&text).map_err(|error| format!("decode_stl_ascii failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let forward = mutation_of(&spec, &base)?;
        let backward = inverse_of(&spec, &base)?;
        let mut snapshot = base;
        apply_stl_mutation(&mut snapshot, &forward);
        apply_stl_mutation(&mut snapshot, &backward);
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        let projection = project_stl(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let text = String::from_utf8(input.clone()).map_err(|error| error.to_string())?;
        let snapshot = decode_stl_ascii(&text).map_err(|error| format!("decode_stl_ascii failed: {error}"))?;
        let bytes = encode_stl_ascii(&snapshot).into_bytes();
        if bytes == input {
            return Err("byte pass-through: subject output is bit-identical to the input".to_string());
        }
        let projection = project_stl(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
