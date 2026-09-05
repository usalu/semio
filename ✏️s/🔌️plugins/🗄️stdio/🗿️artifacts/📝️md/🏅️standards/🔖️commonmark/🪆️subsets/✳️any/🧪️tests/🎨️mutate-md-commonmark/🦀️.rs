//! 🦀️ CommonMark mutation case — Rust adapter.
//!
//! Every scenario copies the one real, committed README fixture into the case work directory
//! first; the committed fixture is never written to. `oracle` drives the registered `comrak`
//! reference implementation through this subset's own oracle module (`oracle_apply_mutation`,
//! `project_md`, `inverse_mutation_spec`); `subject` drives this repository's own
//! `MdMutation`/`apply_md_mutation`/`MdSnapshot::from_text`/`MdSnapshot::to_text` — the real, typed,
//! event-sourced mutation pipeline, not an ad hoc text edit. Both results are read back by the
//! INDEPENDENT `comrak`-backed `project_md` before the `ordered-json-v1` profile compares them. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::md::standards::v_commonmark::subsets::any::{inverse_mutation_spec, oracle_apply_mutation, project_md};

//#region 🔖️Kinds
/// 🗂️ `no-mutation` plus `MdMutation`'s kebab-case `KINDS` (schema/mutations/component.rs).
/// Duplicated here, rather than imported, because the oracle-only host build never links the SUT
/// crate at all (it is an optional dependency gated behind the `sut` feature this crate's own
/// registration loop runs unconditionally), so this list has to be reachable without it.
/// `no-mutation` is NOT one of that production `KINDS`' entries -- it carries no `MdMutation`
/// variant of its own (dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration: `no` is not
/// an approved semantic verb) and is handled directly by `subject::mutate`/`subject::inverse` below
/// as the identity probe this case's own feature file names it.
const KINDS: [&str; 6] = ["no-mutation", "set-snapshot", "insert-block", "remove-block", "replace-block", "set-inlines"];
//#endregion 🔖️Kinds

//#region 🔖️Input
/// 📄️ One real fixture serves both roles: the mutate/inverse scenarios and the identity round trip
/// all read the same real README, each scenario copying it independently into its own case work
/// directory first.
const INPUT: &str = "shared://📖️readme.md";

/// 🧫️ Copies the immutable fixture into the work directory and returns its bytes.
fn mutable_input(ctx: &Context, uri: &str, name: &str) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(uri, Some(name))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 👁️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind other than
/// `no-mutation` whose parameters leave the semantic projection exactly where it was has not been
/// tested by this scenario at all -- it proves only that the reference library declined to error.
/// Every `Examples` row is chosen against the real artifact's actual content for that reason, and
/// this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, INPUT, "input.md")?;
    let bytes = oracle_apply_mutation(&input, &spec)?;
    let projection = project_md(&bytes)?;
    if kind != "no-mutation" && projection_divergence(&projection, &project_md(&input)?).is_none() {
        return Err(format!("{kind:?} left the semantic projection exactly as it found it -- a mutation whose parameters make it a no-op against the real artifact is not a test of that kind"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}

/// ⚖️ First point at which two projections diverge, as a character offset into the canonical
/// rendering plus the window around it on both sides -- an equality check whose failure names WHAT
/// changed rather than only that something did.
fn projection_divergence(restored: &Json, original: &Json) -> Option<String> {
    let (left, right): (Vec<char>, Vec<char>) = (restored.to_string().chars().collect(), original.to_string().chars().collect());
    if left == right {
        return None;
    }
    let at = left.iter().zip(right.iter()).position(|(a, b)| a != b).unwrap_or(left.len().min(right.len()));
    let window = |text: &[char]| text.iter().skip(at.saturating_sub(60)).take(160).collect::<String>();
    Some(format!("first divergence at char {at} of {} vs {} -- got …{}… want …{}…", left.len(), right.len(), window(&left), window(&right)))
}

/// ↩️ Applies the row's forward mutation, then this subset's own algebraic inverse of it (computed
/// by `inverse_mutation_spec` from the ORIGINAL document's own INDEPENDENT `comrak` projection, the
/// same restore-the-prior-value law `MdMutation::inverse` implements), and asserts the restoration
/// against the ORIGINAL document's own projection before ever reaching the framework's
/// oracle-vs-subject comparison.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, INPUT, "input.md")?;
    let original_projection = project_md(&input)?;
    let mutated = oracle_apply_mutation(&input, &spec)?;
    let inverse_spec = inverse_mutation_spec(&input, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &inverse_spec)?;
    let projection = project_md(&restored)?;
    if let Some(divergence) = projection_divergence(&projection, &original_projection) {
        return Err(format!("inverse law violated: {:?} followed by its own inverse did not restore the original document's projection -- {divergence}", spec.str("kind")));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the round-trip law, ASSERTED rather than narrated: `comrak` parses the real
/// document into its own AST and re-renders CommonMark from that AST alone, so BOTH halves of the
/// law are checkable here without a subject -- the re-rendered bytes must differ from the input
/// (CommonMark is not a byte-preserving carrier: the renderer re-derives every marker, fence and
/// escape from the tree, and this repository's own README uses concrete syntax `comrak` does not
/// reproduce verbatim), and the re-rendered document's own block projection must still equal the
/// input's, since the projection carries only what `MdBlock`/`MdInline` themselves carry and is
/// therefore blind to the writer freedom the feature file documents.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx, INPUT, "input.md")?;
    let spec = Json::Object(vec![("kind".to_string(), Json::String("no-mutation".to_string())), ("params".to_string(), Json::Object(Vec::new()))]);
    let bytes = oracle_apply_mutation(&input, &spec)?;
    if bytes == input {
        return Err("byte pass-through: the oracle's re-rendered bytes are bit-identical to the input, so nothing here proves the document was parsed rather than copied".to_string());
    }
    let projection = project_md(&bytes)?;
    let original = project_md(&input)?;
    if let Some(divergence) = projection_divergence(&projection, &original) {
        return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic block projection -- {divergence}"));
    }
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{mutable_input, INPUT};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::md::schema::diff::navigate_container;
    use semio_s_plugin_stdio::artifacts::md::schema::mutations::{apply_md_mutation, MdPathStep};
    use semio_s_plugin_stdio::artifacts::md::schema::mutations::{insert_block::InsertBlock, remove_block::RemoveBlock, replace_block::ReplaceBlock, set_inlines::SetInlines, set_snapshot::SetSnapshot};
    use semio_s_plugin_stdio::artifacts::md::schema::snapshot::{MdBlock, MdInline};
    use semio_s_plugin_stdio::artifacts::md::{MdMutation, MdSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::md::standards::v_commonmark::subsets::any::project_md;

    //#region 🔖️Json
    fn json_array(value: Option<&Json>) -> Vec<Json> {
        match value {
            Some(Json::Array(items)) => items.clone(),
            _ => Vec::new(),
        }
    }
    fn json_usize(json: &Json, key: &str) -> Result<usize, String> {
        match json.get(key) {
            Some(Json::Number(value)) => Ok(*value as usize),
            _ => Err(format!("mutation params carry no numeric '{key}'")),
        }
    }
    fn json_u8(json: &Json, key: &str) -> Result<u8, String> {
        match json.get(key) {
            Some(Json::Number(value)) => Ok(*value as u8),
            _ => Err(format!("mutation params carry no numeric '{key}'")),
        }
    }
    fn optional_string(json: &Json, key: &str) -> Option<String> {
        match json.get(key) {
            Some(Json::String(value)) => Some(value.clone()),
            _ => None,
        }
    }
    //#endregion 🔖️Json

    //#region 🔖️Build
    /// 🏗️ Builds the real `MdBlock`/`MdInline` tree the spec's JSON describes — the same
    /// `kind`-tagged shape `MdBlock`/`MdInline` themselves serialize as, read here by hand (the
    /// generated host's own minimal `Json` type, not `serde_json`, is what a scenario's params
    /// arrive as).
    fn build_inline(json: &Json) -> Result<MdInline, String> {
        match json.str("kind").as_str() {
            "text" => Ok(MdInline::Text { text: json.str("text") }),
            "emphasis" => Ok(MdInline::Emphasis { inlines: build_inlines(&json.array("inlines"))? }),
            "strong" => Ok(MdInline::Strong { inlines: build_inlines(&json.array("inlines"))? }),
            "code" => Ok(MdInline::Code { literal: json.str("literal") }),
            "link" => Ok(MdInline::Link { text: build_inlines(&json.array("text"))?, url: json.str("url"), title: optional_string(json, "title") }),
            "image" => Ok(MdInline::Image { alt: json.str("alt"), url: json.str("url"), title: optional_string(json, "title") }),
            "softBreak" => Ok(MdInline::SoftBreak),
            "hardBreak" => Ok(MdInline::HardBreak),
            "htmlInline" => Ok(MdInline::HtmlInline { raw: json.str("raw") }),
            other => Err(format!("mutation inline carries unknown kind {other:?}")),
        }
    }
    fn build_inlines(items: &[Json]) -> Result<Vec<MdInline>, String> {
        items.iter().map(build_inline).collect()
    }

    fn build_block(json: &Json) -> Result<MdBlock, String> {
        match json.str("kind").as_str() {
            "heading" => Ok(MdBlock::Heading { level: json_u8(json, "level")?, inlines: build_inlines(&json.array("inlines"))? }),
            "paragraph" => Ok(MdBlock::Paragraph { inlines: build_inlines(&json.array("inlines"))? }),
            "list" => {
                let ordered = matches!(json.get("ordered"), Some(Json::Bool(true)));
                let tight = matches!(json.get("tight"), Some(Json::Bool(true)));
                let start = match json.get("start") {
                    Some(Json::Number(value)) => Some(*value as u32),
                    _ => None,
                };
                let items = json.array("items").iter().map(|entry| json_array(Some(entry)).iter().map(build_block).collect::<Result<Vec<_>, _>>()).collect::<Result<Vec<_>, _>>()?;
                Ok(MdBlock::List { ordered, start, tight, items })
            }
            "codeBlock" => Ok(MdBlock::CodeBlock { info: optional_string(json, "info"), literal: json.str("literal") }),
            "blockQuote" => Ok(MdBlock::BlockQuote { blocks: json.array("blocks").iter().map(build_block).collect::<Result<Vec<_>, _>>()? }),
            "thematicBreak" => Ok(MdBlock::ThematicBreak),
            "htmlBlock" => Ok(MdBlock::HtmlBlock { raw: json.str("raw") }),
            other => Err(format!("mutation block carries unknown kind {other:?}")),
        }
    }

    fn build_path(params: &Json) -> Result<Vec<MdPathStep>, String> {
        json_array(params.get("path"))
            .iter()
            .map(|step| match step.str("step").as_str() {
                "blockQuote" => Ok(MdPathStep::BlockQuote { index: json_usize(step, "index")? }),
                "listItem" => Ok(MdPathStep::ListItem { index: json_usize(step, "index")?, item: json_usize(step, "item")? }),
                other => Err(format!("path step carries unknown 'step' {other:?}")),
            })
            .collect()
    }

    fn build_snapshot(json: &Json) -> Result<MdSnapshot, String> {
        let blocks = json.array("blocks").iter().map(build_block).collect::<Result<Vec<_>, _>>()?;
        Ok(MdSnapshot { schema: semio_s_plugin_stdio::artifacts::md::STDIO_MD_DOCUMENT_SCHEMA.to_string(), blocks })
    }

    /// 🦠️ Builds the real `MdMutation` the spec describes — the same shape `oracle_apply_mutation`
    /// reads, so both producers see one spec. `no-mutation` has no arm here -- it carries no
    /// `MdMutation` variant (dropped by the `26/08/29/S-END-TO-END` mutation-leaf migration) and is
    /// handled directly by `mutate`/`inverse` below before this function is ever called.
    fn spec_to_mutation(spec: &Json) -> Result<MdMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "set-snapshot" => Ok(MdMutation::SetSnapshot(SetSnapshot { snapshot: build_snapshot(params.get("snapshot").ok_or("set-snapshot: params carry no 'snapshot'")?)? })),
            "insert-block" => Ok(MdMutation::InsertBlock(InsertBlock { path: build_path(&params)?, index: json_usize(&params, "index")?, block: build_block(params.get("block").ok_or("insert-block: params carry no 'block'")?)? })),
            "remove-block" => Ok(MdMutation::RemoveBlock(RemoveBlock { path: build_path(&params)?, index: json_usize(&params, "index")? })),
            "replace-block" => Ok(MdMutation::ReplaceBlock(ReplaceBlock { path: build_path(&params)?, index: json_usize(&params, "index")?, block: build_block(params.get("block").ok_or("replace-block: params carry no 'block'")?)? })),
            "set-inlines" => Ok(MdMutation::SetInlines(SetInlines { path: build_path(&params)?, index: json_usize(&params, "index")?, inlines: build_inlines(&json_array(params.get("inlines")))? })),
            other => Err(format!("mutation kind {other:?} has no subject implementation")),
        }
    }
    //#endregion 🔖️Build

    //#region 🔖️Inverse
    /// ↩️ The real inverse `MdMutation` of `kind`/`params`, restoring `original`'s own prior value —
    /// the exact algebra `MdMutation::inverse` implements, reusing `navigate_container` (already
    /// part of this same crate, no extra dependency) rather than the `protocol::Mutation` trait,
    /// since the generated case crate has no direct `protocol` dependency of its own to reach it
    /// through (same shape the `deflate` case's own `inverse_spec` helper documents). Returns `None`
    /// when there is nothing to invert against (the address the mutation named is stale) or when
    /// `kind` is `no-mutation` -- both mean zero inverse steps to apply, the same "EMPTY vec" shape
    /// production's own `agg_inverse` returns now that `NoMutation` (dropped by the
    /// `26/08/29/S-END-TO-END` mutation-leaf migration) is no longer available as a no-op sentinel.
    fn inverse_mutation_of(kind: &str, params: &Json, original: &MdSnapshot) -> Result<Option<MdMutation>, String> {
        if kind == "no-mutation" {
            return Ok(None);
        }
        let path = build_path(params)?;
        match kind {
            "set-snapshot" => Ok(Some(MdMutation::SetSnapshot(SetSnapshot { snapshot: original.clone() }))),
            "insert-block" => Ok(Some(MdMutation::RemoveBlock(RemoveBlock { path, index: json_usize(params, "index")? }))),
            "remove-block" => {
                let index = json_usize(params, "index")?;
                match navigate_container(&original.blocks, &path).and_then(|container| container.get(index)).cloned() {
                    Some(block) => Ok(Some(MdMutation::InsertBlock(InsertBlock { path, index, block }))),
                    None => Ok(None),
                }
            }
            "replace-block" => {
                let index = json_usize(params, "index")?;
                match navigate_container(&original.blocks, &path).and_then(|container| container.get(index)).cloned() {
                    Some(block) => Ok(Some(MdMutation::ReplaceBlock(ReplaceBlock { path, index, block }))),
                    None => Ok(None),
                }
            }
            "set-inlines" => {
                let index = json_usize(params, "index")?;
                let original_block = navigate_container(&original.blocks, &path).and_then(|container| container.get(index));
                let inlines = match original_block {
                    Some(MdBlock::Heading { inlines, .. }) => Some(inlines.clone()),
                    Some(MdBlock::Paragraph { inlines }) => Some(inlines.clone()),
                    _ => None,
                };
                match inlines {
                    Some(inlines) => Ok(Some(MdMutation::SetInlines(SetInlines { path, index, inlines }))),
                    None => Ok(None),
                }
            }
            other => Err(format!("mutation kind {other:?} has no inverse implementation")),
        }
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Handlers
    /// `no-mutation` is handled BEFORE `spec_to_mutation` is even called: it carries no
    /// `MdMutation` variant of its own (dropped by the `26/08/29/S-END-TO-END` mutation-leaf
    /// migration), so no mutation is applied at all -- `snapshot` stays exactly the parsed input,
    /// then goes through the same `to_text`/byte-pass-through/projection path every other kind does.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, INPUT, "input.md")?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let mut snapshot = MdSnapshot::from_text(&text);
        if spec.str("kind") != "no-mutation" {
            let mutation = spec_to_mutation(&spec)?;
            apply_md_mutation(&mut snapshot, &mutation);
        }
        let bytes = snapshot.to_text().into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_md(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// `no-mutation` is handled the same way `mutate` handles it above: no `MdMutation` to
    /// construct or invert, so `mutated`/`restored` both stay the identity of `original`.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, INPUT, "input.md")?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let original = MdSnapshot::from_text(&text);
        let original_projection = project_md(&input)?;
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        let kind = spec.str("kind");
        let mut mutated = original.clone();
        if kind != "no-mutation" {
            let mutation = spec_to_mutation(&spec)?;
            apply_md_mutation(&mut mutated, &mutation);
        }
        let mutated_bytes = mutated.to_text().into_bytes();
        if mutated_bytes == input {
            return Err("byte pass-through: mutated output is bit-identical to the input".to_string());
        }
        let mut restored = mutated.clone();
        if let Some(inverse_mutation) = inverse_mutation_of(&kind, &params, &original)? {
            apply_md_mutation(&mut restored, &inverse_mutation);
        }
        let restored_bytes = restored.to_text().into_bytes();
        let projection = project_md(&restored_bytes)?;
        if let Some(divergence) = super::projection_divergence(&projection, &original_projection) {
            return Err(format!("inverse law violated: {kind:?} followed by its own inverse did not restore the original document's projection -- {divergence}"));
        }
        Ok(Outcome::with_raw(restored_bytes, projection))
    }

    /// 🔒️ The SUBJECT side of the round-trip law, asserted in role exactly as
    /// `super::round_trip_oracle` asserts it on its own side: the re-encoded bytes must move, AND
    /// the re-encoded document's own block projection must still equal the input's. Only the byte
    /// half was checked here before, so a decode/re-encode that silently reshaped the document
    /// passed -- and every `inverse-<kind>` row inherited the same blind spot, since each of them
    /// restores through the same `to_text`.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx, INPUT, "input.md")?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let snapshot = MdSnapshot::from_text(&text);
        let bytes = snapshot.to_text().into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_md(&bytes)?;
        if let Some(divergence) = super::projection_divergence(&projection, &project_md(&input)?) {
            return Err(format!("round-trip law violated: decode then re-encode did not preserve the semantic block projection -- {divergence}"));
        }
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
