//! 🦀️ Semio TEXT exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! **This file no longer serves the oracle role.** The reference for `semio-v1-text-mutate` is the
//! registered oracle `semio-text-python-independent` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/
//! 🧪️oracle/🔣️.json`) — an independent Python implementation of the semio text carrier and
//! its seven verbs, written from the committed grammar and protocol documents, living beside this
//! file as `🐍️component.py`. The runner dispatches the oracle role to that adapter and the subject
//! role here, and compares the two projections under `@comparison-ordered-json-v1`. Registering
//! oracle handlers here as well would put this repository's own answer on both sides of that
//! comparison, which is the precise failure the platform exists to prevent.
//!
//! **What the handlers assert in role.** Parity across the two implementations is the primary
//! evidence, but each side still states its own law so a scenario can fail for the right reason with
//! a readable message: `inverse-<kind>` requires the mutation's OWN computed inverse to restore the
//! note, `spec-vector-<kind>` requires the applied snapshot to be the committed after-snapshot, and
//! `identity-round-trip` requires both committed encodings to be reproduced byte for byte through
//! `law::carrier_is_exact`.
//!
//! **How the fixture reaches typed values.** The generated test host links only `semio-repo-test-host`
//! and, behind `sut`, this subset's own crate — no `serde`, no `serde_json`, and this crate's
//! `protocol`/`store` extern-crate aliases are private (`📦️glue.rs`), so neither `protocol::Mutation` nor
//! a `serde` derive is nameable from here. The subset's own production code therefore exports the bridges
//! this adapter needs, whose signatures name only reachable types:
//! `decode_semio_text_snapshot_json`/`encode_semio_text_snapshot_json` (`../../🏅️standards/🔖️v1/
//! 🪆️subsets/✳️text/🧬️schema/📸️snapshot/🦀️component.rs`), `decode_semio_text_mutation_json`/
//! `inverse_semio_text_mutation` (`…/🧬️mutations/🦀️component.rs`) and the DSL/pack pass-throughs
//! `parse_semio_text_dsl`/`print_semio_text_dsl`/`encode_semio_text_pack`/`decode_semio_text_pack`.
//! Every input is read from a fixture the FEATURE declares — the mutation parameters from the
//! scenario's doc string, the specification vectors from the `asset://` URIs its steps name — so
//! neither adapter holds a transcription that could drift away from what the other one read.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioTextMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️text/🧬️schema/
/// 🧬️mutations/🦀️component.rs`) — duplicated, not imported, because the generated host builds this
/// file with and without the subject crate. The contract's mutation-coverage gate keeps this list
/// honest against the catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps
/// it honest against the enum.
#[cfg(feature = "sut")]
const KINDS: &[&str] = &["insert-run", "remove-run", "edit-run", "change-run-language", "reorder-runs", "add-mark", "remove-mark"];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::mutations::{apply_semio_text_mutation, decode_semio_text_mutation_json, inverse_semio_text_mutation, SemioTextMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::text::schema::snapshot::{decode_semio_text_pack, decode_semio_text_snapshot_json, encode_semio_text_pack, encode_semio_text_snapshot_json, parse_semio_text_dsl, print_semio_text_dsl, SemioTextSnapshot};

    //#region 🔖️Input
    /// 📰️ The document every mutation row runs on: 384 real runs of the real German article
    /// "Zukunft Bau: Entwerfen mit Bestand", derived ONCE from this repository's own committed
    /// HTML 5 fixture by `🐍️derive-text-fixture.py` in the ticket folder.
    const ARTICLE_DSL: &str = "local://🗣️zukunft-bau-entwerfen-mit-bestand.dsl.semio";
    /// 🎒️ The same article in its binary envelope, written by the PYTHON implementation — so this
    /// codec reproducing it is a cross-language byte agreement, not a codec agreeing with itself.
    const ARTICLE_PACK: &str = "local://🎒️zukunft-bau-entwerfen-mit-bestand.pack.semio";
    /// 🗣️ The tiny committed note — an unmarked English run, an English run carrying a `bold` mark
    /// and a German run carrying a `link` mark with a non-empty `href`. It is kept for the BYTE half
    /// of the identity law: its two files were written by THIS codec, so the Python side reproducing
    /// them is the other direction of the same cross-language agreement.
    const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🗣️example.dsl.semio";
    /// 🎒️ The same note in its binary envelope, written by a separate codec from the DSL text.
    const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️text/📚️examples/📃️note/🖼️assets/🎒️example.pack.semio";

    /// 📰️ The article, parsed through this repository's own DSL codec.
    fn note(ctx: &Context) -> Result<SemioTextSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(ARTICLE_DSL)?).map_err(|error| format!("the article artifact is not UTF-8: {error}"))?;
        parse_semio_text_dsl(&text)
    }

    /// 📜️ The scenario's own committed mutation parameters — the feature owns the vector.
    fn mutation(ctx: &Context) -> Result<SemioTextMutation, String> {
        decode_semio_text_mutation_json(ctx.doc_string()?).map_err(|error| format!("{}: the scenario's mutation payload must decode: {error}", ctx.scenario.id))
    }

    /// 🧫️ Every `asset://` URI the scenario's steps name, in step order. The feature is the single
    /// place the specification-vector paths are written down; both adapters read them from there.
    fn step_assets(ctx: &Context) -> Vec<String> {
        let mut found = Vec::new();
        for (_, text) in &ctx.scenario.steps {
            let mut rest = text.as_str();
            while let Some(at) = rest.find("asset://") {
                let tail = &rest[at..];
                let end = tail.find(char::is_whitespace).unwrap_or(tail.len());
                found.push(tail[..end].to_string());
                rest = &tail[end..];
            }
        }
        found
    }

    fn vector(ctx: &Context, position: usize, label: &str) -> Result<String, String> {
        let uri = step_assets(ctx).into_iter().nth(position).ok_or_else(|| format!("{}: the scenario names no {label} asset", ctx.scenario.id))?;
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{uri} is not UTF-8: {error}"))
    }

    fn apply(current: &mut SemioTextSnapshot, step: &SemioTextMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_text_mutation(current, step);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: the mutation was rejected: {refusals:?}"))
    }

    fn projection(snapshot: &SemioTextSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_text_snapshot_json(snapshot))
    }

    /// 🚨️ A failure message that names WHAT disagreed, in the same JSON both sides project, so a red
    /// scenario is readable without re-running anything.
    fn disagreement(what: &str, got: &SemioTextSnapshot, expected: &SemioTextSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_text_snapshot_json(got), encode_semio_text_snapshot_json(expected))
    }
    //#endregion 🔖️Input

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed note by this repository's codec alone.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = note(ctx)?;
        apply(&mut current, &mutation(ctx)?, &ctx.scenario.id)?;
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(print_semio_text_dsl(&current).into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real note: applying the verb and then its OWN computed
    /// inverse must restore the note exactly — run addressing, nested mark indices and the `href` a
    /// `link` mark carries included.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = note(ctx)?;
        let step = mutation(ctx)?;
        let mut current = base.clone();
        apply(&mut current, &step, &ctx.scenario.id)?;
        let mutated = projection(&current)?;
        for undo in inverse_semio_text_mutation(&step, &base) {
            apply(&mut current, &undo, &ctx.scenario.id)?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the note", ctx.scenario.id), &current, &base));
        }
        Ok(Outcome::projection(Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)])))
    }

    /// 🧫️ The same verb on its committed handcrafted `(before, mutation, after)` vector — a THIRD
    /// statement of what the verb means, independent of both implementations.
    pub fn spec_vector(ctx: &Context) -> Result<Outcome, String> {
        let mut current = decode_semio_text_snapshot_json(&vector(ctx, 0, "before-snapshot")?)?;
        let step = decode_semio_text_mutation_json(&vector(ctx, 1, "mutation")?)?;
        let expected = decode_semio_text_snapshot_json(&vector(ctx, 2, "after-snapshot")?)?;
        apply(&mut current, &step, &ctx.scenario.id)?;
        if current != expected {
            return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
        }
        Ok(Outcome::projection(projection(&current)?))
    }

    /// 🔁️ One document's two encodings, each re-emitted from the parsed document and required back
    /// byte for byte. `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary
    /// twin, so an exact re-emission is the CORRECT answer here and the wave's "output must not
    /// equal input" tripwire would be exactly backwards; its MIRROR law is asserted in its place
    /// through `law::carrier_is_exact`, which fails with the offset of the first differing byte.
    fn carrier_pair(ctx: &Context, dsl_uri: &str, pack_uri: &str, what: &str) -> Result<(SemioTextSnapshot, Json), String> {
        let dsl_bytes = ctx.fixture_bytes(dsl_uri)?;
        let text = String::from_utf8(dsl_bytes.clone()).map_err(|error| format!("identity-round-trip: {what} is not UTF-8: {error}"))?;
        let parsed = parse_semio_text_dsl(&text)?;
        let printed = print_semio_text_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), &dsl_bytes)?;
        let reparsed = parse_semio_text_dsl(&printed)?;
        if reparsed != parsed {
            return Err(disagreement(&format!("identity-round-trip: printing {what} back to DSL and reparsing it lost content"), &reparsed, &parsed));
        }
        let pack_bytes = ctx.fixture_bytes(pack_uri)?;
        let unpacked = decode_semio_text_pack(&pack_bytes)?;
        if unpacked != parsed {
            return Err(disagreement(&format!("identity-round-trip: the binary twin of {what} decodes to a different document than its text"), &unpacked, &parsed));
        }
        let repacked_bytes = encode_semio_text_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_text_pack(&repacked_bytes)?;
        if repacked != parsed {
            return Err(disagreement(&format!("identity-round-trip: encoding {what} to a pack and decoding it back lost content"), &repacked, &parsed));
        }
        let report = Json::Object(vec![
            ("document".to_string(), projection(&parsed)?),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(digest(&repacked_bytes))),
            ("dslLength".to_string(), Json::Number(printed.len() as f64)),
            ("packLength".to_string(), Json::Number(repacked_bytes.len() as f64)),
        ]);
        Ok((parsed, report))
    }

    /// 🔁️ Both documents, in both encodings — four files, all four reproduced byte for byte. The
    /// committed note's two files are this codec's own output and the Python implementation
    /// reproduces them from the grammar and the protocol alone; the article's two files are the
    /// PYTHON implementation's output and this codec has to reproduce THOSE, 384 runs and 344 marks
    /// among them.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let (note, note_report) = carrier_pair(ctx, DSL_ASSET, PACK_ASSET, "the committed note")?;
        if note.runs.len() != 3 {
            return Err(format!("identity-round-trip: the committed note is the three-run artifact this case describes, but decoded as {} run(s)", note.runs.len()));
        }
        let (article, article_report) = carrier_pair(ctx, ARTICLE_DSL, ARTICLE_PACK, "the article")?;
        let marks: usize = article.runs.iter().map(|run| run.marks.len()).sum();
        if article.runs.len() != 384 || marks != 344 {
            return Err(format!("identity-round-trip: the article is the 384-run 344-mark document this case describes, but decoded as {} run(s) and {marks} mark(s)", article.runs.len()));
        }
        Ok(Outcome::projection(Json::Object(vec![("note".to_string(), note_report), ("article".to_string(), article_report)])))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Only subject handlers are
/// registered: the oracle role belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector);
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
