//! 🦀️ glTF 2.0 mutation case — Rust adapter. Covers the 7 kinds `GLTF_MUTATION_LEAF_DESCRIPTORS`
//! (`../../🏅️standards/🔖️2.0/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`) registers today —
//! `mutate-<kind>`/`inverse-<kind>` each, plus one identity round trip. The oracle performs every
//! kind by independent GLB-container and JSON-tree manipulation (`../../🏅️standards/🔖️2.0/
//! 🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`, using `json` 0.12 as the JSON layer only, never this
//! subset's own codec or descriptors); the subject fully parses into `GltfSnapshot` via `decode_glb`
//! and re-serializes with `encode_glb` alone (no byte pass-through), dispatching through each real
//! leaf's own `DESCRIPTOR` function pointers directly rather than the full envelope/registry layer.
//! Both results are read back by the INDEPENDENT `project_gltf` reader before the `semantic-gltf-v1`
//! profile compares them.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::{oracle_apply_mutation, project_gltf, round_trip, undo_create_scene};

//#region 🔖️Kinds
/// 🏷️ Mirrors `GLTF_MUTATION_LEAF_DESCRIPTORS`'s 7 registered leaves (`../../🏅️standards/🔖️2.0/
/// 🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`). Kept as a plain literal here rather than
/// imported since this adapter's oracle-only build never links the subject crate — the contract
/// gate (mutation coverage against the `gltf-2-0-any` catalog) is what keeps the two lists honest.
const KINDS: &[&str] = &["bind-node-child", "bind-scene-root-node", "change-material-alpha-mode", "change-material-double-sided", "create-scene", "unbind-node-child", "unbind-scene-root-node"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "local://🧊️base-with-nested-node.glb";

/// 🧫️ Copies the derived-once fixture into the work directory and returns its bytes; the committed
/// fixture itself is never written to.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.glb"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️JsonBuild
fn json_obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(entries.into_iter().map(|(key, value)| (key.to_string(), value)).collect())
}
fn json_num(value: f64) -> Json {
    Json::Number(value)
}
fn json_str(value: &str) -> Json {
    Json::String(value.to_string())
}
fn json_spec(kind: &str, params: Json) -> Json {
    json_obj(vec![("kind", json_str(kind)), ("params", params)])
}
//#endregion 🔖️JsonBuild

//#region 🔖️Inverse
/// ↩️ The semantically correct inverse spec for one forward `(kind, params)` pair against the
/// derived fixture's own known real state (`../../🏅️standards/🔖️2.0/🪆️subsets/✳️any/📚️examples/
/// 🌱️metabolism/🖼️assets/🧊️base.glb` with node 1 moved from the scene's 271-entry root list into
/// node 0's own `children` — see the feature file), computed independently here since the oracle
/// role must not link the subject crate. `create-scene` has no catalog kind of its own to invert
/// through — production dispatches its inverse via the SAME descriptor's `phase: Inverse`, not a
/// separate `delete-scene` leaf — so its caller uses `undo_create_scene` directly instead of this
/// function.
fn inverse_spec(kind: &str) -> Json {
    match kind {
        "bind-node-child" => json_spec("unbind-node-child", json_obj(vec![("parent", json_num(2.0)), ("child", json_num(3.0))])),
        "unbind-node-child" => json_spec("bind-node-child", json_obj(vec![("parent", json_num(0.0)), ("child", json_num(1.0)), ("position", json_num(0.0))])),
        "bind-scene-root-node" => json_spec("unbind-scene-root-node", json_obj(vec![("scene", json_num(0.0)), ("node", json_num(1.0))])),
        "unbind-scene-root-node" => json_spec("bind-scene-root-node", json_obj(vec![("scene", json_num(0.0)), ("node", json_num(5.0)), ("position", json_num(4.0))])),
        "change-material-alpha-mode" => json_spec("change-material-alpha-mode", json_obj(vec![("material", json_num(0.0)), ("alphaMode", json_str("OPAQUE"))])),
        "change-material-double-sided" => json_spec("change-material-double-sided", json_obj(vec![("material", json_num(0.0)), ("doubleSided", Json::Bool(false))])),
        other => json_spec(other, json_obj(vec![])),
    }
}
//#endregion 🔖️Inverse

//#region 🔖️Oracle
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_gltf(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let restored = if kind == "create-scene" { undo_create_scene(&mutated, 0)? } else { oracle_apply_mutation(&mutated, &inverse_spec(&kind))? };
    let projection = project_gltf(&restored)?;
    Ok(Outcome::with_raw(restored, projection))
}

fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = round_trip(&input)?;
    let projection = project_gltf(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::io::{decode_glb, encode_glb};
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::mutations::{
        bind_node_child, bind_scene_root_node, change_material_alpha_mode, change_material_double_sided, create_scene, unbind_node_child, unbind_scene_root_node, GltfMutationLeafDescriptor,
    };
    use semio_s_plugin_stdio::artifacts::gltf::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
    use semio_s_plugin_stdio_test_oracle::artifacts::gltf::standards::v2_0::subsets::any::project_gltf;

    //#region 🔖️DescriptorLookup
    /// 🧭️ The real descriptor for one catalog kind — the same 7 `DESCRIPTOR` consts
    /// `GLTF_MUTATION_LEAF_DESCRIPTORS` assembles, addressed directly rather than through the full
    /// command-id/phase/envelope registry (`../../🏅️standards/🔖️2.0/🪆️subsets/✳️any/🔨️modules/
    /// 🧭️mutation-dispatch/🦀️component.rs`), which this thin per-kind dispatch does not need.
    fn descriptor_for_kind(kind: &str) -> Option<GltfMutationLeafDescriptor> {
        match kind {
            "bind-node-child" => Some(bind_node_child::DESCRIPTOR),
            "bind-scene-root-node" => Some(bind_scene_root_node::DESCRIPTOR),
            "change-material-alpha-mode" => Some(change_material_alpha_mode::DESCRIPTOR),
            "change-material-double-sided" => Some(change_material_double_sided::DESCRIPTOR),
            "create-scene" => Some(create_scene::DESCRIPTOR),
            "unbind-node-child" => Some(unbind_node_child::DESCRIPTOR),
            "unbind-scene-root-node" => Some(unbind_scene_root_node::DESCRIPTOR),
            _ => None,
        }
    }
    //#endregion 🔖️DescriptorLookup

    //#region 🔖️Codec
    /// 📐️ Full parse → typed descriptor plan/apply → re-serialize from the model alone — the
    /// no-byte-pass-through rule this wave exists to enforce. `payload` is the mutation spec's own
    /// `params`, serialized straight to bytes: every one of the 7 leaves' payload struct field names
    /// (`parent`/`child`/`position`, `scene`/`node`/`position`, `material`/`alphaMode`,
    /// `material`/`doubleSided`) already matches the feature file's own `params` shape exactly, so no
    /// per-kind field translation is needed here.
    fn apply_and_encode(before: &GltfSnapshot, kind: &str, payload: &[u8]) -> Result<(Vec<u8>, GltfSnapshot), String> {
        let descriptor = descriptor_for_kind(kind).ok_or_else(|| format!("unrecognised mutation kind {kind:?}"))?;
        let plan = (descriptor.plan)(payload, before).map_err(|error| error.to_string())?;
        let applied = (descriptor.apply_diff)(&plan.diff_payload, before).map_err(|error| error.to_string())?;
        let bytes = encode_glb(&applied.snapshot)?;
        Ok((bytes, applied.snapshot))
    }
    //#endregion 🔖️Codec

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = decode_glb(&input)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let payload = spec.get("params").unwrap_or(&empty).to_string().into_bytes();
        let (bytes, _) = apply_and_encode(&before, &kind, &payload)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let before = decode_glb(&input)?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let empty = Json::Object(Vec::new());
        let payload = spec.get("params").unwrap_or(&empty).to_string().into_bytes();
        let descriptor = descriptor_for_kind(&kind).ok_or_else(|| format!("unrecognised mutation kind {kind:?}"))?;
        let plan = (descriptor.plan)(&payload, &before).map_err(|error| error.to_string())?;
        let after = (descriptor.apply_diff)(&plan.diff_payload, &before).map_err(|error| error.to_string())?.snapshot;
        let restored = (descriptor.apply_inverse)(&plan.inverse_payload, &after).map_err(|error| error.to_string())?.snapshot;
        let bytes = encode_glb(&restored)?;
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_glb(&input)?;
        let bytes = encode_glb(&snapshot)?;
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_gltf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
    //#endregion 🔖️Handlers
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
