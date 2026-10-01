//! 🦀️ CommonMark mutation case — Rust adapter.
//!
//! Every scenario copies the one real, committed README fixture into the case work directory
//! first; the committed fixture is never written to. `oracle` drives the registered `comrak`
//! reference implementation through this subset's own oracle module (`oracle_apply_mutation`,
//! `oracle_apply_mutation_inverse`, `project_md`), whose answer is the projection of the AST it edited; `subject`
//! drives this repository's own
//! `MdMutation`/`apply_md_mutation`/`MdSnapshot::from_text`/`MdSnapshot::to_text` — the real, typed,
//! event-sourced mutation pipeline, not an ad hoc text edit. Both results are read back by the
//! INDEPENDENT `comrak`-backed `project_md` before the `ordered-json-v1` profile compares them. The
//! subject half is gated behind the generated host's `sut` feature so the oracle-only run never
//! compiles the local implementation.

use semio_repo_test_host::{Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::md::standards::v_commonmark::subsets::any::{oracle_apply_mutation, oracle_apply_mutation_inverse, oracle_round_trip, project_md};


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
/// 👁️ The forward mutation, with the OBSERVABILITY law asserted in role: a kind whose parameters
/// leave the semantic projection exactly where it was has not been tested by this scenario at all
/// -- it proves only that the reference library declined to error.
/// Every `Examples` row is chosen against the real artifact's actual content for that reason, and
/// this check is what keeps them so.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let kind = spec.str("kind");
    let input = mutable_input(ctx, INPUT, "input.md")?;
    let (bytes, projection) = oracle_apply_mutation(&input, &spec)?;
    if projection_divergence(&projection, &project_md(&input)?).is_none() {
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

/// ↩️ Applies the row's forward mutation, then the oracle's own algebraic inverse of it (computed
/// from the ORIGINAL document's own INDEPENDENT `comrak` projection, the same restore-the-prior-value
/// law `MdMutation::inverse` implements) to one `comrak` tree, and asserts the restoration against
/// the ORIGINAL document's own projection before ever reaching the framework's oracle-vs-subject
/// comparison.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let input = mutable_input(ctx, INPUT, "input.md")?;
    let original_projection = project_md(&input)?;
    let (restored, projection) = oracle_apply_mutation_inverse(&input, &spec)?;
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
    let bytes = oracle_round_trip(&input)?;
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
    use semio_s_artifact_stdio_md::schema::mutations::apply_md_mutation;
    use semio_s_plugin_stdio_test_oracle::law::wire_operation;
    use semio_s_artifact_stdio_md::{mutation_from_payload_json, mutation_inverse, mutation_payload_json};
    use semio_s_artifact_stdio_md::{MdMutation, MdSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::md::standards::v_commonmark::subsets::any::project_md;

    //#region 🔖️SpecCodec
    /// 📄️ The scenario's `<id>`/`<params>` spec decoded as the leaf wire payload it is, through the aggregate's own
    /// derive-generated payload constructor — never re-declared field by field here.
    fn mutation_from_spec(spec: &Json) -> Result<MdMutation, String> {
        wire_operation(&spec.str("kind"), &spec.get("params").cloned().unwrap_or(Json::Null), mutation_from_payload_json, mutation_payload_json)
    }
    //#endregion 🔖️SpecCodec

    //#region 🔖️Handlers
    /// 👁️ The forward mutation through this subset's own vocabulary, re-rendered and projected.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, INPUT, "input.md")?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let mut snapshot = MdSnapshot::from_text(&text);
        apply_md_mutation(&mut snapshot, &mutation_from_spec(&spec)?);
        let bytes = snapshot.to_text().into_bytes();
        if bytes == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_md(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// ↩️ The subset's OWN `Mutation::inverse` (`inverse_md_mutation`), applied after the forward step and asserted
    /// against the original document's projection — the implementation's algebra, not a transcription of it.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let spec = ctx.doc_json()?;
        let input = mutable_input(ctx, INPUT, "input.md")?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("input is not valid UTF-8: {error}"))?;
        let original = MdSnapshot::from_text(&text);
        let original_projection = project_md(&input)?;
        let kind = spec.str("kind");
        let mutation = mutation_from_spec(&spec)?;
        let undo = mutation_inverse(&mutation, &original);
        let mut restored = original;
        apply_md_mutation(&mut restored, &mutation);
        if restored.to_text().into_bytes() == input {
            return Err("byte pass-through: mutated output is bit-identical to the input".to_string());
        }
        for step in &undo {
            apply_md_mutation(&mut restored, step);
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
    built = built.oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
