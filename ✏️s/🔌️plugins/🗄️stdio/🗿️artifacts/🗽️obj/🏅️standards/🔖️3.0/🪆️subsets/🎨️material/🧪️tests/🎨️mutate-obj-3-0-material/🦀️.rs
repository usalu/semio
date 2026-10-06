//! 🦀️ OBJ 3.0 material-subset mutation case — Rust adapter, the SUBJECT half. Exhaustive for this subset's own 2-kind
//! vocabulary (`set-mtllib`, `set-usemtl`, the `obj-3.0-material` catalog of `../../🔮️oracles/🔣️.json`).
//!
//! The oracle is `three-obj-3-0-document-reader`, a third-party READER: the expected document of every row is the
//! COMMITTED `➡️after.obj` of the row's pair (`⬅️before.obj` for an inverse row), answered by `🟦️.ts`, and the
//! `obj-3-0-document-compare-v1` pipeline reads it and this subject's `actual-obj` with three's OBJLoader. The subject
//! fully parses the committed `⬅️before.obj` into `ObjSnapshot`, applies the typed mutation through this subset's own
//! `apply_obj_mutation` and re-serializes from the model alone (no byte pass-through). Every row's `params` is the leaf
//! wire payload, decoded by `ObjMutation`'s own payload constructor; an inverse row applies the kind and then the
//! production inverse — a re-`set-*` of the value the committed before-document carries.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::io::{decode_obj, encode_obj};
    use semio_repo_test_host::law::wire_operation;
    use semio_s_artifact_stdio_obj::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::mutations::apply_obj_mutation;

    /// 📦️ Runs the row: decodes the committed before-document, applies the row's wire payload decoded by `ObjMutation`'s
    /// own payload constructor (and, for an inverse row, the production inverse computed against the before-document),
    /// re-encodes, and answers the document as the `actual-obj` artifact the pipeline reads. A forward row must not hand
    /// back its input bytes; a restored document may — the committed pairs are this encoder's canonical form.
    fn run(ctx: &Context, undo: bool) -> Result<Outcome, String> {
        let uri = ctx.step_input_uris().into_iter().find(|uri| uri.ends_with("/⬅️before.obj")).ok_or_else(|| "the row names no committed ⬅️before.obj".to_string())?;
        let input = ctx.input_bytes(&uri)?;
        let mut snapshot = decode_obj(std::str::from_utf8(&input).map_err(|error| error.to_string())?).map_err(|error| format!("decode_obj failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let kind = spec.str("kind");
        let forward = wire_operation(&kind, &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)?;
        let backward = if undo { mutation_inverse(&forward, &snapshot).expect("valid retained mutation inverse fixture") } else { Vec::new() };
        apply_obj_mutation(&mut snapshot, &forward);
        for mutation in &backward {
            apply_obj_mutation(&mut snapshot, mutation);
        }
        let bytes = encode_obj(&snapshot).into_bytes();
        if !undo && bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let path = ctx.artifact("actual-obj", "actual.obj")?;
        std::fs::write(&path, &bytes).map_err(|error| error.to_string())?;
        let projection = Json::Object(vec![("kind".to_string(), Json::String(kind)), ("bytes".to_string(), Json::Number(bytes.len() as f64))]);
        Ok(Outcome::with_raw(bytes, projection).artifact("actual-obj", &path, "model/obj"))
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        run(ctx, false)
    }

    pub fn inverse_row(ctx: &Context) -> Result<Outcome, String> {
        run(ctx, true)
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration by Scenario Outline base id; the subject half is `sut`-gated so a build without the subset never
/// compiles it.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse_row);
    built
}
//#endregion 🔖️Registration
