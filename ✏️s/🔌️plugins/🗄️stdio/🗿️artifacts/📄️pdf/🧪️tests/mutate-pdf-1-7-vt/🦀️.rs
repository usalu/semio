//! 🦀️ PDF 1.7 ✳️vt exhaustive conformance-class mutation case — Rust adapter.
//!
//! Every scenario copies the real, committed `🎓️bachelor-thesis` asset into the case work directory
//! first; the committed asset is never written to. `oracle` handlers drive the registered `lopdf`
//! 0.44 reference implementation through this subset's own
//! `../../🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧪️oracle/🦀️component.rs`, `subject` handlers drive this
//! repository's own decode/mutate/encode round trip, and both results are read back by the SAME
//! independent `project_conformance` before the `semantic-pdf-conformance-vt-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never compiles the local implementation.
//!
//! @see ../../🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🦀️component.rs — `check_vt_conformance`, the one
//!      axis list this whole case derives from.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::vt::{oracle_apply_mutation, oracle_arrange, oracle_inverse_spec, oracle_round_trip, project_conformance};

//#region 🔖️Kinds
/// 🧾️ Test-case-local mirror of the `pdf-1-7-vt` catalog. Duplicated, not imported, from
/// `../../🏅️standards/🔖️1.7/🪆️subsets/✳️vt/🧬️schema/🧬️mutations/🦀️component.rs::KINDS` — that module
/// lives in the SUBJECT crate, and the oracle role must not link the subject crate at all, while
/// this loop registers handlers for both roles from one list. A mismatch is caught structurally: the
/// contract phase fails with `mutation-kind-uncovered`/`mutation-kind-undeclared` if this list omits
/// or invents a kind, and the runner fails every unregistered scenario id outright.
const KINDS: &[&str] = &["insert-encryption-dictionary", "remove-encryption-dictionary", "set-output-intent", "remove-output-intent", "set-trim-box", "remove-trim-box", "embed-font-file", "remove-font-file", "insert-javascript-action", "remove-javascript-action", "insert-launch-action", "remove-launch-action", "insert-media-annotation", "remove-media-annotation", "set-dpart-root", "remove-dpart-root", "set-dpart-metadata", "remove-dpart-metadata"];
//#endregion 🔖️Kinds

//#region 🔖️Input
const INPUT: &str = "asset://🏅️standards/🔖️1.4/🪆️subsets/✳️base/📚️examples/🎓️bachelor-thesis/🖼️assets/📄️bachelor-thesis.pdf";

/// 🧫️ Copies the immutable real asset into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("bachelor-thesis.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}

/// 🎬️ The real pre-state a scenario's mutation runs on. For the kinds whose target the committed
/// document does not carry, this is the real document after the reference implementation has
/// independently put it there — the feature file names every one of them rather than papering over
/// it. Every other kind reads the committed bytes untouched.
fn arranged_input(ctx: &Context, spec: &Json) -> Result<Vec<u8>, String> {
    oracle_arrange(&mutable_input(ctx)?, spec)
}
//#endregion 🔖️Input

//#region 🔖️Law
/// 🔬️ First structural divergence between two projections — a dotted field path plus both values,
/// so a law that fails names WHICH axis moved instead of only "not equal". Kept local to this
/// adapter: a case adapter is a leaf that links the framework host and its own subset's oracle, and
/// nothing else.
fn first_divergence(path: &str, expected: &Json, actual: &Json) -> Option<String> {
    let here = if path.is_empty() { "the projection".to_string() } else { path.to_string() };
    let child = |key: &str| if path.is_empty() { key.to_string() } else { format!("{path}.{key}") };
    match (expected, actual) {
        (Json::Object(left), Json::Object(right)) => {
            for (key, value) in left {
                match right.iter().find(|(name, _)| name == key) {
                    Some((_, other)) => {
                        if let Some(found) = first_divergence(&child(key), value, other) {
                            return Some(found);
                        }
                    }
                    None => return Some(format!("{} is gone (the original carried {})", child(key), brief(value))),
                }
            }
            right.iter().find(|(name, _)| !left.iter().any(|(other, _)| other == name)).map(|(name, value)| format!("{} appeared (absent in the original, now {})", child(name), brief(value)))
        }
        (Json::Array(left), Json::Array(right)) => {
            if left.len() != right.len() {
                return Some(format!("{here} has {} entries, the original had {}", right.len(), left.len()));
            }
            left.iter().zip(right.iter()).enumerate().find_map(|(index, (value, other))| first_divergence(&child(&index.to_string()), value, other))
        }
        (left, right) if left == right => None,
        (left, right) => Some(format!("{here} is {} — the original had {}", brief(right), brief(left))),
    }
}

fn brief(value: &Json) -> String {
    let text = value.to_string();
    match text.char_indices().nth(160) {
        Some((cut, _)) => format!("{}…", &text[..cut]),
        None => text,
    }
}
//#endregion 🔖️Law

//#region 🔖️Oracle
/// 🔮️ One handler shared by every `mutate-<kind>` scenario id — and the place the OBSERVABILITY law
/// is asserted in-role: a mutation that leaves this subset's conformance-class projection untouched
/// proves nothing, whatever the reference implementation returned.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let output = oracle_apply_mutation(&base, &spec)?;
    let projection = project_conformance(&output)?;
    if projection == project_conformance(&base)? {
        return Err(format!("mutate-{}: the mutation left the conformance-class projection unchanged — a mutation that is not observable proves nothing", spec.str("kind")));
    }
    Ok(Outcome::with_raw(output, projection))
}

/// ↩️ One handler shared by every `inverse-<kind>` scenario id — and the place the INVERSE law is
/// asserted in-role, without needing the subject: `apply(inverse(m), apply(m, base))` must land back
/// on the pre-state's own projection, read through the same independent reader.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let base = arranged_input(ctx, &spec)?;
    let original = project_conformance(&base)?;
    let mutated = oracle_apply_mutation(&base, &spec)?;
    let undo = oracle_inverse_spec(&base, &spec)?;
    let restored = oracle_apply_mutation(&mutated, &undo)?;
    let projection = project_conformance(&restored)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("inverse-{}: the mutation followed by its own computed inverse ({}) did not restore the document — {}", spec.str("kind"), undo.str("kind"), divergence));
    }
    Ok(Outcome::with_raw(restored, projection))
}

/// 🔒️ The ORACLE side of the no-byte-pass-through law, asserted rather than merely claimed in
/// prose: `lopdf` parses the whole 3,189-object graph and re-serializes a fresh file from that graph
/// alone, and BOTH halves are checked — the bytes must differ from the input (nothing was copied)
/// and their projection must be identical to the input's (nothing was lost).
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let output = oracle_round_trip(&input)?;
    if output == input {
        return Err("byte pass-through: the re-serialized document is bit-identical to the input".to_string());
    }
    let projection = project_conformance(&output)?;
    let original = project_conformance(&input)?;
    if let Some(divergence) = first_divergence("", &original, &projection) {
        return Err(format!("identity round trip: parsing and re-serializing the real document did not preserve its conformance-class projection — {divergence}"));
    }
    Ok(Outcome::with_raw(output, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{arranged_input, mutable_input};
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::{decode_pdf, encode_pdf};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::conformance_support as support;
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::schema::snapshot::{ObjRef, PdfSnapshot};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::vt::schema::mutations::{apply_vt_conformance_mutation, PdfVtMutation, InsertEncryptionDictionary, RemoveEncryptionDictionary, SetOutputIntent, RemoveOutputIntent, SetTrimBox, RemoveTrimBox, EmbedFontFile, RemoveFontFile, InsertJavascriptAction, RemoveJavascriptAction, InsertLaunchAction, RemoveLaunchAction, InsertMediaAnnotation, RemoveMediaAnnotation, SetDpartRoot, RemoveDpartRoot, SetDpartMetadata, RemoveDpartMetadata};
    use semio_s_plugin_stdio_test_oracle::artifacts::pdf::standards::v1_7::subsets::vt::{oracle_inverse_spec, project_conformance};

    fn decode(bytes: &[u8]) -> Result<PdfSnapshot, String> {
        decode_pdf(bytes).map_err(|error| error.to_string())
    }

    fn encode(snapshot: &PdfSnapshot) -> Result<Vec<u8>, String> {
        encode_pdf(snapshot).map_err(|error| error.to_string())
    }

    fn number(value: &Json) -> Option<f64> {
        match value {
            Json::Number(number) => Some(*number),
            _ => None,
        }
    }

    fn four_numbers(params: &Json, key: &str) -> Result<[f64; 4], String> {
        let items = params.array(key);
        if items.len() != 4 {
            return Err(format!("`{key}` must be an array of four numbers"));
        }
        Ok([number(&items[0]).unwrap_or(0.0), number(&items[1]).unwrap_or(0.0), number(&items[2]).unwrap_or(0.0), number(&items[3]).unwrap_or(0.0)])
    }

    /// 🔤️ The donor font-program object an `embed-font-file` names — exactly (`program: {num, gen}`,
    /// what an engine-computed inverse carries) or by ordinal into the document's own program list
    /// (`programOrdinal`, what a feature row can actually author).
    fn program_reference(base: &PdfSnapshot, params: &Json) -> Result<ObjRef, String> {
        if let Some(reference) = params.get("program") {
            let num = reference.get("num").and_then(number).ok_or("`program.num` must be a number")? as u32;
            let gen = reference.get("gen").and_then(number).unwrap_or(0.0) as u16;
            return Ok(ObjRef { num, gen });
        }
        let programs = support::font_programs(base);
        let index = params.get("programOrdinal").and_then(number).unwrap_or(0.0) as usize;
        programs.get(index).copied().ok_or_else(|| format!("program ordinal {index} is out of range ({} font programs)", programs.len()))
    }

    /// 🔀️ The same JSON mutation spec the oracle reads, turned into this repository's own typed
    /// `PdfVtMutation` — the only channel between the feature's parameters and the subject's codec.
    fn mutation_from_spec(base: &PdfSnapshot, spec: &Json) -> Result<PdfVtMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Null);
        Ok(match spec.str("kind").as_str() {
            "insert-encryption-dictionary" => PdfVtMutation::InsertEncryptionDictionary(InsertEncryptionDictionary { version: params.get("version").and_then(number).unwrap_or(2.0) as i64, revision: params.get("revision").and_then(number).unwrap_or(3.0) as i64 }),
            "remove-encryption-dictionary" => PdfVtMutation::RemoveEncryptionDictionary(RemoveEncryptionDictionary { version: params.get("version").and_then(number).unwrap_or(2.0) as i64, revision: params.get("revision").and_then(number).unwrap_or(3.0) as i64 }),
            "set-output-intent" => PdfVtMutation::SetOutputIntent(SetOutputIntent { identifier: params.str("identifier") }),
            "remove-output-intent" => PdfVtMutation::RemoveOutputIntent(RemoveOutputIntent {}),
            "set-trim-box" => PdfVtMutation::SetTrimBox(SetTrimBox { page_index: params.get("pageIndex").and_then(number).unwrap_or(0.0) as usize, trim_box: four_numbers(&params, "trimBox")? }),
            "remove-trim-box" => PdfVtMutation::RemoveTrimBox(RemoveTrimBox { page_index: params.get("pageIndex").and_then(number).unwrap_or(0.0) as usize }),
            "embed-font-file" => PdfVtMutation::EmbedFontFile(EmbedFontFile { descriptor_ordinal: params.get("descriptorOrdinal").and_then(number).unwrap_or(0.0) as usize, key: params.str("key"), program: program_reference(base, &params)? }),
            "remove-font-file" => PdfVtMutation::RemoveFontFile(RemoveFontFile { descriptor_ordinal: params.get("descriptorOrdinal").and_then(number).unwrap_or(0.0) as usize }),
            "insert-javascript-action" => PdfVtMutation::InsertJavascriptAction(InsertJavascriptAction { script: params.str("script") }),
            "remove-javascript-action" => PdfVtMutation::RemoveJavascriptAction(RemoveJavascriptAction { script: params.str("script") }),
            "insert-launch-action" => PdfVtMutation::InsertLaunchAction(InsertLaunchAction { target: params.str("target") }),
            "remove-launch-action" => PdfVtMutation::RemoveLaunchAction(RemoveLaunchAction { target: params.str("target") }),
            "insert-media-annotation" => PdfVtMutation::InsertMediaAnnotation(InsertMediaAnnotation { subtype: params.str("subtype"), title: params.str("title") }),
            "remove-media-annotation" => PdfVtMutation::RemoveMediaAnnotation(RemoveMediaAnnotation { subtype: params.str("subtype"), title: params.str("title") }),
            "set-dpart-root" => PdfVtMutation::SetDpartRoot(SetDpartRoot { job: params.str("job") }),
            "remove-dpart-root" => PdfVtMutation::RemoveDpartRoot(RemoveDpartRoot {}),
            "set-dpart-metadata" => PdfVtMutation::SetDpartMetadata(SetDpartMetadata { job: params.str("job") }),
            "remove-dpart-metadata" => PdfVtMutation::RemoveDpartMetadata(RemoveDpartMetadata {}),
            other => return Err(format!("no subject rule for kind {other:?}")),
        })
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let mut snapshot = decode(&arranged_input(ctx, &spec)?)?;
        let mutation = mutation_from_spec(&snapshot, &spec)?;
        apply_vt_conformance_mutation(&mut snapshot, &mutation);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let base = arranged_input(ctx, &spec)?;
        let mut snapshot = decode(&base)?;
        let forward = mutation_from_spec(&snapshot, &spec)?;
        apply_vt_conformance_mutation(&mut snapshot, &forward);
        let undo = oracle_inverse_spec(&base, &spec)?;
        let backward = mutation_from_spec(&snapshot, &undo)?;
        apply_vt_conformance_mutation(&mut snapshot, &backward);
        let output = encode(&snapshot)?;
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }

    /// 🔁️ Full semantic parse, re-serialized from the model alone — copying, splicing or patching
    /// source bytes is cheating, and this tripwire catches it.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode(&input)?;
        let output = encode(&snapshot)?;
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_conformance(&output)?;
        Ok(Outcome::with_raw(output, projection))
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
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
