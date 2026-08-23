//! 🦀️ DOCX ECMA-376/✳️any exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
//!
//! Every scenario copies the real, committed `📜️example-readme.docx` fixture (derived once from
//! this repository's own real `README.md` — see the feature file's own header for the full
//! provenance) into the case work directory first; the committed fixture is never written to.
//! `oracle` drives the registered `zip`+`quick-xml` composition
//! (`../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_docx`/`encode_docx`/`apply_docx_mutation` over the full 13-kind `DocxMutation`
//! vocabulary. Both results are read back by the SAME independent `project_docx_ecma_376` (the
//! `zip`+`quick-xml` composition) before the `semantic-docx-ecma-376-mutate-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never links `semio-s-plugin-stdio`, whose subject phase is peer-blocked right now (concurrent
//! os-kernel refactor).

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_docx_ecma_376};

//#region 🔖️Kinds
/// 📇️ Kebab-case spelling of every `DocxMutation` variant, mirrored from
/// `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️component.rs`'s own
/// `KINDS` — duplicated rather than imported because the ORACLE-only build of this adapter must
/// never link `semio-s-plugin-stdio` (see this file's own header); `kinds_const_matches_enum_
/// variants_in_declaration_order` on the production side and the framework's own catalog-
/// completeness gate on this side are what keep the two lists honest against each other.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "insert-block", "remove-block", "set-block-content", "set-run-text", "set-run-formatting", "insert-style", "remove-style", "set-style-name", "set-style-based-on", "set-part", "remove-part"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "shared://📜️example-readme.docx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("example-readme.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id -- the scenario's own `<id>`/`<params>`
/// spec is carried in its doc string, not in the function it dispatches to.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ One handler shared by every `inverse-<kind>` scenario id.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law: the `zip`+`quick-xml` composition fully
/// parses the real document and re-serializes it from its own tree alone (the same "no-mutation"
/// routing `oracle_apply_mutation` already gives every other kind), independent evidence that a full
/// parse/re-serialize is possible before the SUBJECT is held to the same standard below.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let no_mutation = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))]);
    let bytes = oracle_apply_mutation(&input, &no_mutation)?;
    let projection = project_docx_ecma_376(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, KINDS};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::io::export::serializers::encode_docx;
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::io::import::deserializers::decode_docx;
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::schema::diff::{resolve_blocks, DocxBlockPath, DocxPathSegment};
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::schema::mutations::apply_docx_mutation;
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::schema::snapshot::{DocxBlock, DocxParagraph, DocxRun, DocxStyle, DocxTable, DocxTableCell, DocxTableRow};
    use semio_s_plugin_stdio::artifacts::docx::{DocxMutation, DocxSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::any::project_docx_ecma_376;

    //#region 🔖️SpecCodec
    fn number_field(value: &Json, key: &str) -> f64 {
        match value.get(key) {
            Some(Json::Number(number)) => *number,
            _ => 0.0,
        }
    }

    fn usize_field(value: &Json, key: &str) -> usize {
        number_field(value, key).max(0.0) as usize
    }

    fn bool_field(value: &Json, key: &str) -> bool {
        matches!(value.get(key), Some(Json::Bool(true)))
    }

    fn non_empty(value: &Json, key: &str) -> Option<String> {
        match value.get(key) {
            Some(Json::String(text)) if !text.is_empty() => Some(text.clone()),
            _ => None,
        }
    }

    /// 🧭️ The same `{"segments":[{"blockIndex":..,"row":..,"cell":..}, ...], "index": N}` shape the
    /// oracle side speaks, decoded into the PRODUCTION `DocxBlockPath` here instead of the oracle's
    /// own independent path type.
    fn json_to_path(value: &Json) -> DocxBlockPath {
        let segments = value.array("segments").iter().map(|s| DocxPathSegment { block_index: usize_field(s, "blockIndex"), row: usize_field(s, "row"), cell: usize_field(s, "cell") }).collect();
        DocxBlockPath { segments, index: usize_field(value, "index") }
    }

    /// 🔎️ The same owned block-spec JSON grammar the oracle side speaks
    /// (`{"kind":"paragraph"|"table", ...}`), decoded into the PRODUCTION `DocxBlock` here instead
    /// of the oracle's own independent tree type.
    fn json_to_block(value: &Json) -> Result<DocxBlock, String> {
        match value.str("kind").as_str() {
            "paragraph" => Ok(DocxBlock::Paragraph(DocxParagraph {
                runs: value
                    .array("runs")
                    .iter()
                    .map(|r| DocxRun { text: r.str("text"), bold: bool_field(r, "bold"), italic: bool_field(r, "italic"), underline: bool_field(r, "underline"), extra_run_properties: Vec::new() })
                    .collect(),
                style: non_empty(value, "style"),
                extra_paragraph_properties: Vec::new(),
            })),
            "table" => Ok(DocxBlock::Table(DocxTable {
                rows: value
                    .array("rows")
                    .iter()
                    .map(|row| {
                        Ok::<_, String>(DocxTableRow {
                            cells: row.array("cells").iter().map(|cell| Ok::<_, String>(DocxTableCell { blocks: cell.array("blocks").iter().map(json_to_block).collect::<Result<_, _>>()?, extra_cell_properties: Vec::new() })).collect::<Result<_, _>>()?,
                            extra_row_properties: Vec::new(),
                        })
                    })
                    .collect::<Result<_, _>>()?,
                extra_table_properties: Vec::new(),
            })),
            other => Err(format!("unknown block kind {other:?}")),
        }
    }

    fn json_to_style(value: &Json) -> DocxStyle {
        DocxStyle { id: value.str("id"), name: value.str("name"), based_on: non_empty(value, "basedOn") }
    }

    /// 📄️ The scenario's `<id>`/`<params>` spec turned into the ONE typed `DocxMutation` this subset
    /// declares for it. `set-snapshot` replaces `document.body`/`document.styles` only — the real OPC
    /// parts stay exactly as `decode_docx` read them (see the feature file's own header note).
    fn mutation_from_spec(spec: &Json, base: &DocxSnapshot) -> Result<DocxMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(DocxMutation::NoMutation),
            "set-snapshot" => {
                let mut snapshot = base.clone();
                snapshot.document.body = params.array("body").iter().map(json_to_block).collect::<Result<_, _>>()?;
                snapshot.document.styles = params.array("styles").iter().map(|s| Ok::<_, String>(json_to_style(s))).collect::<Result<_, _>>()?;
                Ok(DocxMutation::SetSnapshot { snapshot })
            }
            "insert-block" => Ok(DocxMutation::InsertBlock { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), block: json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))? }),
            "remove-block" => Ok(DocxMutation::RemoveBlock { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)) }),
            "set-block-content" => Ok(DocxMutation::SetBlockContent { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), block: json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))? }),
            "set-run-text" => Ok(DocxMutation::SetRunText { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), run_index: usize_field(&params, "runIndex"), text: params.str("text") }),
            "set-run-formatting" => Ok(DocxMutation::SetRunFormatting { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), run_index: usize_field(&params, "runIndex"), bold: bool_field(&params, "bold"), italic: bool_field(&params, "italic"), underline: bool_field(&params, "underline") }),
            "insert-style" => Ok(DocxMutation::InsertStyle { style: json_to_style(&params.get("style").cloned().unwrap_or(Json::Null)) }),
            "remove-style" => Ok(DocxMutation::RemoveStyle { id: params.str("id") }),
            "set-style-name" => Ok(DocxMutation::SetStyleName { id: params.str("id"), name: params.str("name") }),
            "set-style-based-on" => Ok(DocxMutation::SetStyleBasedOn { id: params.str("id"), based_on: non_empty(&params, "basedOn") }),
            "set-part" => Ok(DocxMutation::SetPart { path: params.str("path"), content_type: params.str("contentType"), bytes: params.str("content").into_bytes() }),
            "remove-part" => Ok(DocxMutation::RemovePart { path: params.str("path") }),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Inverse
    fn block_at<'a>(base: &'a DocxSnapshot, path: &DocxBlockPath) -> Option<&'a DocxBlock> {
        resolve_blocks(&base.document.body, &path.segments)?.get(path.index)
    }

    fn style_at<'a>(base: &'a DocxSnapshot, id: &str) -> Option<&'a DocxStyle> {
        base.document.styles.iter().find(|style| style.id == id)
    }

    /// ↩️ `DocxMutation::inverse` in closed form -- every variant's own `Mutation::inverse` arm,
    /// transplanted rather than called through the trait, same precedent `mutate-zip-2-0`'s own
    /// `invert_zip_mutation` and `mutate-xml-1-0`'s own `inverse_of` give: written in closed form so
    /// this adapter needs no extra crate dependency beyond `semio-s-plugin-stdio` itself.
    fn inverse_of(mutation: &DocxMutation, base: &DocxSnapshot) -> DocxMutation {
        match mutation {
            DocxMutation::NoMutation => DocxMutation::NoMutation,
            DocxMutation::SetSnapshot { .. } => DocxMutation::SetSnapshot { snapshot: base.clone() },
            DocxMutation::InsertBlock { path, .. } => DocxMutation::RemoveBlock { path: path.clone() },
            DocxMutation::RemoveBlock { path } => match block_at(base, path) {
                Some(block) => DocxMutation::InsertBlock { path: path.clone(), block: block.clone() },
                None => DocxMutation::NoMutation,
            },
            DocxMutation::SetBlockContent { path, .. } => match block_at(base, path) {
                Some(block) => DocxMutation::SetBlockContent { path: path.clone(), block: block.clone() },
                None => DocxMutation::NoMutation,
            },
            DocxMutation::SetRunText { path, run_index, .. } => {
                let old = resolve_blocks(&base.document.body, &path.segments).and_then(|blocks| blocks.get(path.index)).and_then(|block| match block { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None }).map(|run| run.text.clone());
                match old {
                    Some(text) => DocxMutation::SetRunText { path: path.clone(), run_index: *run_index, text },
                    None => DocxMutation::NoMutation,
                }
            }
            DocxMutation::SetRunFormatting { path, run_index, .. } => {
                let old = resolve_blocks(&base.document.body, &path.segments).and_then(|blocks| blocks.get(path.index)).and_then(|block| match block { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None });
                match old {
                    Some(run) => DocxMutation::SetRunFormatting { path: path.clone(), run_index: *run_index, bold: run.bold, italic: run.italic, underline: run.underline },
                    None => DocxMutation::NoMutation,
                }
            }
            DocxMutation::InsertStyle { style } => DocxMutation::RemoveStyle { id: style.id.clone() },
            DocxMutation::RemoveStyle { id } => match style_at(base, id) {
                Some(style) => DocxMutation::InsertStyle { style: style.clone() },
                None => DocxMutation::NoMutation,
            },
            DocxMutation::SetStyleName { id, .. } => match style_at(base, id) {
                Some(style) => DocxMutation::SetStyleName { id: id.clone(), name: style.name.clone() },
                None => DocxMutation::NoMutation,
            },
            DocxMutation::SetStyleBasedOn { id, .. } => match style_at(base, id) {
                Some(style) => DocxMutation::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() },
                None => DocxMutation::NoMutation,
            },
            DocxMutation::SetPart { path, .. } => match base.opc.part(path) {
                Some(part) => DocxMutation::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() },
                None => DocxMutation::RemovePart { path: path.clone() },
            },
            DocxMutation::RemovePart { path } => match base.opc.part(path) {
                Some(part) => DocxMutation::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() },
                None => DocxMutation::NoMutation,
            },
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_docx(&mutable_input(ctx)?).map_err(|error| format!("decode_docx failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let mut snapshot = base;
        apply_docx_mutation(&mut snapshot, &mutation);
        let bytes = encode_docx(&snapshot).map_err(|error| format!("encode_docx failed: {error}"))?;
        let projection = project_docx_ecma_376(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = decode_docx(&mutable_input(ctx)?).map_err(|error| format!("decode_docx failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &base)?;
        let undo = inverse_of(&mutation, &base);
        let mut snapshot = base;
        apply_docx_mutation(&mut snapshot, &mutation);
        apply_docx_mutation(&mut snapshot, &undo);
        let bytes = encode_docx(&snapshot).map_err(|error| format!("encode_docx failed: {error}"))?;
        let projection = project_docx_ecma_376(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🔒️ The no-byte-pass-through rule: the subject must fully parse the real artifact into its
    /// typed snapshot and re-serialize from the model alone -- `decode_docx`/`encode_docx` are this
    /// subset's ONLY channel from input to output.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_docx(&input).map_err(|error| format!("decode_docx failed: {error}"))?;
        let output = encode_docx(&snapshot).map_err(|error| format!("encode_docx failed: {error}"))?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_docx_ecma_376(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Handlers

    /// 🧭️ Re-exported so `super::adapter()` can register the same 13-kind sweep for the subject role
    /// without duplicating `KINDS` a third time.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 13 kinds -- the scenario id only selects which fixture row's
/// `<id>`/`<params>` doc string the shared handler reads, per `Adapter::oracle`/`subject`'s own
/// per-scenario dispatch table.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
    }
    built = built.oracle("identity-round-trip", identity_round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        for kind in subject::SUBJECT_KINDS {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
