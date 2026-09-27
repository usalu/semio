//! 🦀️ OBJ 3.0 material-subset mutation case — Rust adapter, the SUBJECT half. Exhaustive for this subset's own 2-kind
//! vocabulary (`set-mtllib`, `set-usemtl`, the `obj-3.0-material` catalog of `../../🔮️oracles/🔣️.json`).
//!
//! The oracle is `three-obj-3-0-document-reader`, a third-party READER: the expected document of every row is the
//! COMMITTED `➡️after.obj` of the row's pair (`⬅️before.obj` for an inverse row), answered by `🟦️.ts`, and the
//! `obj-3-0-document-compare-v1` pipeline reads it and this subject's `actual-obj` with three's OBJLoader. The subject
//! fully parses the committed `⬅️before.obj` into `ObjSnapshot`, applies the typed mutation through this subset's own
//! `apply_obj_mutation` and re-serializes from the model alone (no byte pass-through). An inverse row applies the kind
//! and then its inverse — a re-`set-*` of the value the committed before-document carries.

use semio_repo_test_host::Adapter;

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::io::{decode_obj, encode_obj};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::mutations::{apply_obj_mutation, set_mtllib, set_usemtl, ObjMutation};
    use semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::snapshot::ObjUsemtlRange;

    fn text(value: &Json, key: &str) -> Result<String, String> {
        match value.get(key) {
            Some(Json::String(text)) => Ok(text.clone()),
            _ => Err(format!("expected text field {key:?}")),
        }
    }

    fn index(value: &Json, key: &str) -> Result<usize, String> {
        match value.get(key) {
            Some(Json::Number(number)) => Ok(*number as usize),
            _ => Err(format!("expected numeric field {key:?}")),
        }
    }

    /// 🦠️ The row's `(kind, params)` as the typed mutation this subset owns; any other kind is an error.
    fn mutation(kind: &str, params: &Json) -> Result<ObjMutation, String> {
        Ok(match kind {
            "set-mtllib" => ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: text(params, "mtllib").ok() }),
            "set-usemtl" => ObjMutation::SetUsemtl(set_usemtl::SetUsemtl { usemtl: params.array("usemtl").iter().map(|entry| Ok(ObjUsemtlRange { face_index_from: index(entry, "faceIndexFrom")?, material: text(entry, "material")? })).collect::<Result<Vec<_>, String>>()? }),
            other => return Err(format!("mutate-obj-3-0-material: {other:?} is not a kind of this subset")),
        })
    }

    /// ↩️ The inverse: re-`set-*` of the value the committed before-document carries.
    fn inverse(kind: &str, before: &semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot) -> Result<ObjMutation, String> {
        Ok(match kind {
            "set-mtllib" => ObjMutation::SetMtllib(set_mtllib::SetMtllib { mtllib: before.mtllib.clone() }),
            "set-usemtl" => ObjMutation::SetUsemtl(set_usemtl::SetUsemtl { usemtl: before.usemtl.clone() }),
            other => return Err(format!("mutate-obj-3-0-material: {other:?} has no inverse in this subset")),
        })
    }

    /// 📦️ Runs the row: decodes the committed before-document, applies the row's kind (and, for an inverse row, its
    /// inverse), re-encodes, and answers the document as the `actual-obj` artifact the pipeline reads. A forward row must
    /// not hand back its input bytes; a restored document may — the committed pairs are this encoder's canonical form.
    fn run(ctx: &Context, undo: bool) -> Result<Outcome, String> {
        let uri = ctx.step_fixture_uris().into_iter().find(|uri| uri.ends_with("/⬅️before.obj")).ok_or_else(|| "the row names no committed ⬅️before.obj".to_string())?;
        let input = ctx.fixture_bytes(&uri)?;
        let before = decode_obj(std::str::from_utf8(&input).map_err(|error| error.to_string())?).map_err(|error| format!("decode_obj failed: {error}"))?;
        let spec = ctx.doc_json()?;
        let empty = Json::Object(Vec::new());
        let kind = spec.str("kind");
        let mut snapshot = before.clone();
        apply_obj_mutation(&mut snapshot, &mutation(&kind, spec.get("params").unwrap_or(&empty))?);
        if undo {
            apply_obj_mutation(&mut snapshot, &inverse(&kind, &before)?);
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
