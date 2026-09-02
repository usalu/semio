//! 🦀️ DOCX ECMA-376/✳️any exhaustive mutation case — Rust adapter. Ticket
//! 26/08/23/END-TO-END-TESTING-REFACTOR wave 7.
//!
//! Every scenario copies the real, committed `📜️example-readme.docx` fixture (derived once from
//! this repository's own real `README.md` — see the feature file's own header for the full
//! provenance) into the case work directory first; the committed fixture is never written to.
//! `oracle` drives the registered `zip`+`quick-xml` composition
//! (`../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️any/🦀️oracle.rs`'s own
//! `oracle_apply_mutation`/`oracle_apply_mutation_inverse`); `subject` drives this repository's own
//! `decode_docx`/`encode_docx`/`apply_docx_mutation` over the full 12-kind `DocxMutation`
//! vocabulary. Both results are read back by the SAME independent `project_docx_ecma_376` (the
//! `zip`+`quick-xml` composition) before the `semantic-docx-ecma-376-mutate-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never links `semio-s-plugin-stdio` -- §5.3's own role separation, NOT a workaround for anything:
//! the Rust subject phase runs (`subject exhaustive --owner 🗄️stdio --case mutate-docx-ecma-376`
//! executes all 25 scenarios), and wave 14 ran the full differential comparison against the oracle.
//!
//! ⚖️ All three laws are asserted IN ROLE, through the shared `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law`
//! module, so a scenario cannot pass merely because `zip`+`quick-xml` declined to error:
//! `mutate-<kind>` must MOVE the compared projection, `inverse-<kind>` must land back on the
//! untouched package's projection, and `identity-round-trip` must both preserve the projection and
//! rebuild an archive that differs from the input. There is no carve-out of any kind here: the
//! profile declares no writer freedom, no kind is exempt from observability, and no axis is dropped
//! from the inverse law. The one inverse this vocabulary genuinely cannot express — removing an
//! INTERIOR style, which `InsertStyle`'s append can never put back — is refused by the oracle
//! outright rather than faked, and the feature says so.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::docx::standards::v_ecma_376::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, project_docx_ecma_376, KINDS};
use semio_s_plugin_stdio_test_oracle::law::{inverse_restores, mutation_is_observable, reparsed_not_copied, round_trip_preserves};

//#region 🔖️Input
const INPUT: &str = "shared://📜️example-readme.docx";

/// 🧫️ Copies the immutable real fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("example-readme.docx"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🈳️ The `no-mutation` spec, which is how the identity round trip asks the reference composition to
/// unzip, parse, re-serialize and rezip the real package without changing anything.
fn no_mutation() -> Json {
    Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(vec![]))])
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🦠️ The forward half, with the OBSERVABILITY law asserted in role: the reference composition
/// applies the kind to the real README document and the result has to differ from the untouched
/// package. Returning the projection uncompared is what made these twelve scenarios pass whenever
/// `zip`+`quick-xml` merely did not error. NOTHING is exempt — `semantic-docx-ecma-376-mutate-v1`
/// declares no writer freedom at all (`ignoreKeys: []`), and the ordered block tree, the ordered
/// style list and the path-keyed digest of every other OPC part between them reach all twelve
/// kinds.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    mutation_is_observable(&spec.str("kind"), &projection, &project_docx_ecma_376(&input)?, &[])?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// ↩️ The INVERSE law, asserted in role without needing the subject: `apply(inverse(m), apply(m,
/// base))` must land back on the ORIGINAL package's own projection, read through the same
/// independent reader. No axis is dropped and no tolerance is allowed.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let bytes = oracle_apply_mutation_inverse(&input, &spec)?;
    let projection = project_docx_ecma_376(&bytes)?;
    inverse_restores(&kind, &projection, &project_docx_ecma_376(&input)?)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔒️ The ORACLE side of the identity law, both halves asserted: the `zip`+`quick-xml` composition
/// fully parses the real package and re-serializes it from its own trees alone (the same
/// `no-mutation` routing every other kind goes through), the rebuilt archive must differ from the
/// input — two independent writers do not agree on compression level, extra fields, entry order or
/// attribute layout, so bit-identical output would mean the input was copied — and the projection
/// must survive intact.
fn identity_round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let bytes = oracle_apply_mutation(&input, &no_mutation())?;
    reparsed_not_copied(&bytes, &input)?;
    let projection = project_docx_ecma_376(&bytes)?;
    round_trip_preserves(&projection, &project_docx_ecma_376(&input)?)?;
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
    use semio_s_plugin_stdio::artifacts::docx::standards::v_ecma_376::subsets::any::schema::mutations::{
        insert_block, insert_style, remove_block, remove_part, remove_style, set_block_content, set_part, set_run_formatting, set_run_text, set_snapshot, set_style_based_on, set_style_name,
    };
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
            "set-snapshot" => {
                let mut snapshot = base.clone();
                snapshot.document.body = params.array("body").iter().map(json_to_block).collect::<Result<_, _>>()?;
                snapshot.document.styles = params.array("styles").iter().map(|s| Ok::<_, String>(json_to_style(s))).collect::<Result<_, _>>()?;
                Ok(DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot }))
            }
            "insert-block" => Ok(DocxMutation::InsertBlock(insert_block::InsertBlock { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), block: json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))? })),
            "remove-block" => Ok(DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)) })),
            "set-block-content" => Ok(DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), block: json_to_block(&params.get("block").cloned().unwrap_or(Json::Null))? })),
            "set-run-text" => Ok(DocxMutation::SetRunText(set_run_text::SetRunText { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), run_index: usize_field(&params, "runIndex"), text: params.str("text") })),
            "set-run-formatting" => Ok(DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { path: json_to_path(&params.get("path").cloned().unwrap_or(Json::Null)), run_index: usize_field(&params, "runIndex"), bold: bool_field(&params, "bold"), italic: bool_field(&params, "italic"), underline: bool_field(&params, "underline") })),
            "insert-style" => Ok(DocxMutation::InsertStyle(insert_style::InsertStyle { style: json_to_style(&params.get("style").cloned().unwrap_or(Json::Null)) })),
            "remove-style" => Ok(DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: params.str("id") })),
            "set-style-name" => Ok(DocxMutation::SetStyleName(set_style_name::SetStyleName { id: params.str("id"), name: params.str("name") })),
            "set-style-based-on" => Ok(DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: params.str("id"), based_on: non_empty(&params, "basedOn") })),
            "set-part" => Ok(DocxMutation::SetPart(set_part::SetPart { path: params.str("path"), content_type: params.str("contentType"), bytes: params.str("content").into_bytes() })),
            "remove-part" => Ok(DocxMutation::RemovePart(remove_part::RemovePart { path: params.str("path") })),
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
    // 🧭️ `NoMutation` was dropped by the mutation-leaf migration (26/08/29/S-END-TO-END); this
    // adapter's own inverse-of-nothing branches now fall back to `SetRunText` on an out-of-range
    // path/run, this subset's own documented no-op (`diff_set_run_text` returns the empty diff when
    // the addressed run does not exist), mirroring the same replacement made in
    // `../../🏅️standards/🔖️ecma-376/🪆️subsets/✳️base/🧬️schema/🧬️mutations/🦀️.rs`'s `agg_inverse` and
    // pptx's own analogous adapter.
    fn inverse_of(mutation: &DocxMutation, base: &DocxSnapshot) -> DocxMutation {
        let documented_no_op = || DocxMutation::SetRunText(set_run_text::SetRunText { path: DocxBlockPath { segments: Vec::new(), index: usize::MAX }, run_index: usize::MAX, text: String::new() });
        match mutation {
            DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { .. }) => DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }),
            DocxMutation::InsertBlock(insert_block::InsertBlock { path, .. }) => DocxMutation::RemoveBlock(remove_block::RemoveBlock { path: path.clone() }),
            DocxMutation::RemoveBlock(remove_block::RemoveBlock { path }) => match block_at(base, path) {
                Some(block) => DocxMutation::InsertBlock(insert_block::InsertBlock { path: path.clone(), block: block.clone() }),
                None => documented_no_op(),
            },
            DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path, .. }) => match block_at(base, path) {
                Some(block) => DocxMutation::SetBlockContent(set_block_content::SetBlockContent { path: path.clone(), block: block.clone() }),
                None => documented_no_op(),
            },
            DocxMutation::SetRunText(set_run_text::SetRunText { path, run_index, .. }) => {
                let old = resolve_blocks(&base.document.body, &path.segments).and_then(|blocks| blocks.get(path.index)).and_then(|block| match block { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None }).map(|run| run.text.clone());
                match old {
                    Some(text) => DocxMutation::SetRunText(set_run_text::SetRunText { path: path.clone(), run_index: *run_index, text }),
                    None => documented_no_op(),
                }
            }
            DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { path, run_index, .. }) => {
                let old = resolve_blocks(&base.document.body, &path.segments).and_then(|blocks| blocks.get(path.index)).and_then(|block| match block { DocxBlock::Paragraph(p) => p.runs.get(*run_index), _ => None });
                match old {
                    Some(run) => DocxMutation::SetRunFormatting(set_run_formatting::SetRunFormatting { path: path.clone(), run_index: *run_index, bold: run.bold, italic: run.italic, underline: run.underline }),
                    None => documented_no_op(),
                }
            }
            DocxMutation::InsertStyle(insert_style::InsertStyle { style }) => DocxMutation::RemoveStyle(remove_style::RemoveStyle { id: style.id.clone() }),
            DocxMutation::RemoveStyle(remove_style::RemoveStyle { id }) => match style_at(base, id) {
                Some(style) => DocxMutation::InsertStyle(insert_style::InsertStyle { style: style.clone() }),
                None => documented_no_op(),
            },
            DocxMutation::SetStyleName(set_style_name::SetStyleName { id, .. }) => match style_at(base, id) {
                Some(style) => DocxMutation::SetStyleName(set_style_name::SetStyleName { id: id.clone(), name: style.name.clone() }),
                None => documented_no_op(),
            },
            DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id, .. }) => match style_at(base, id) {
                Some(style) => DocxMutation::SetStyleBasedOn(set_style_based_on::SetStyleBasedOn { id: id.clone(), based_on: style.based_on.clone() }),
                None => documented_no_op(),
            },
            DocxMutation::SetPart(set_part::SetPart { path, .. }) => match base.opc.part(path) {
                Some(part) => DocxMutation::SetPart(set_part::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() }),
                None => DocxMutation::RemovePart(remove_part::RemovePart { path: path.clone() }),
            },
            DocxMutation::RemovePart(remove_part::RemovePart { path }) => match base.opc.part(path) {
                Some(part) => DocxMutation::SetPart(set_part::SetPart { path: path.clone(), content_type: part.content_type.clone(), bytes: part.bytes.clone() }),
                None => documented_no_op(),
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

    /// 🧭️ Re-exported so `super::adapter()` can register the same 12-kind sweep for the subject
    /// role from the one list the subset's own oracle module declares.
    pub const SUBJECT_KINDS: &[&str] = KINDS;
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. `mutate-<kind>`/`inverse-<kind>` share ONE
/// handler per role across all 12 kinds -- the scenario id only selects which fixture row's
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
