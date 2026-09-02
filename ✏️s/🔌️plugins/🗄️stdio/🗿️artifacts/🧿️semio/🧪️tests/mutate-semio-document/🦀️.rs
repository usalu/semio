//! 🦀️ Semio DOCUMENT exhaustive mutation case — Rust SUBJECT adapter. Ticket 26/08/23/END-TO-END-
//! TESTING-REFACTOR.
//!
//! This file registers the SUBJECT role only. The reference answer comes from `🐍️component.py`
//! beside it — an independent Python implementation of both committed carriers and all eighteen
//! verbs, written from the committed grammars and the committed binary protocol, registered as the
//! oracle `semio-document-python-independent` in
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🔣️oracle.json`. Registering oracle
//! handlers here as well would put this repository's own answer on both sides of the comparison,
//! which is the one failure the platform exists to prevent, so the registrations this file used to
//! carry are gone rather than merely unused.
//!
//! Every scenario drives this repository's own production entry points —
//! `parse_semio_document_dsl`/`print_semio_document_dsl` and
//! `decode_semio_document_pack`/`encode_semio_document_pack` for the two carriers,
//! `apply_semio_document_mutation`/`inverse_semio_document_mutation` for the vocabulary — over the
//! real committed memo
//! `../../🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️.dsl.semio` and its
//! committed binary twin, and projects through this subset's own JSON bridge for `ordered-json-v1`
//! to compare against the Python side's.
//!
//! The mutation parameters and the specification-vector paths live in `component.feature`, so both
//! implementations read one physical copy of each and cannot drift apart. The laws are asserted
//! here IN ROLE as well as compared: `inverse-` requires the memo back, `spec-vector-` requires the
//! committed after-snapshot, and `identity-round-trip` requires byte-exact re-emission of BOTH
//! committed encodings through `law::carrier_is_exact`.
//!
//! The subject half is gated behind the generated host's `sut` feature so a non-subject build never
//! compiles the local implementation.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
/// 🏷️ Mirrors `SemioDocumentMutation::KINDS` (`../../🏅️standards/🔖️v1/🪆️subsets/✳️document/🧬️schema/
/// 🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build must not
/// link the subject crate. The contract's mutation-coverage gate keeps this list honest against the
/// catalog; `kinds_match_the_enum_and_the_catalog` in that production file keeps it honest against
/// the enum.
#[cfg_attr(not(feature = "sut"), allow(dead_code))]
const KINDS: &[&str] = &[
    "no-mutation",
    "set-snapshot",
    "insert-block",
    "remove-block",
    "set-block-content",
    "set-paragraph-style",
    "set-heading-level",
    "set-list-ordered",
    "set-run-text",
    "set-run-style",
    "set-image-block",
    "insert-style",
    "remove-style",
    "set-style-name",
    "set-style-based-on",
    "insert-image",
    "remove-image",
    "set-image-bytes",
];
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{digest, parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law::carrier_is_exact;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::any::schema::mutations::semio_mutation_refusals;
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::mutations::{apply_semio_document_mutation, decode_semio_document_mutation_json, inverse_semio_document_mutation, set_snapshot, SemioDocumentMutation};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::document::schema::snapshot::{
        decode_semio_document_pack, decode_semio_document_snapshot_json, encode_semio_document_pack, encode_semio_document_snapshot_json, parse_semio_document_dsl, print_semio_document_dsl, SemioDocumentSnapshot,
    };

    const DSL_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🗣️.dsl.semio";
    const PACK_ASSET: &str = "asset://🏅️standards/🔖️v1/🪆️subsets/✳️any/📚️examples/📄️memo/🖼️assets/🎒️.pack.semio";

    //#region 🔖️Fixtures
    /// 🧫️ The SAME committed vector `../🦀️.rs::vector` reads, decoded into real values
    /// through this subset's own `serde_json` bridges rather than transcribed into Rust literals.
    fn vector(ctx: &Context, kind: &str) -> Result<Json, String> {
        ctx.fixture_json(&format!("local://🦠️{kind}.json"))
    }

    fn snapshot_of(vector: &Json, name: &str) -> Result<SemioDocumentSnapshot, String> {
        let member = vector.get(name).ok_or_else(|| format!("committed specification vector carries no {name:?} member"))?;
        decode_semio_document_snapshot_json(&member.to_string())
    }

    fn projection(snapshot: &SemioDocumentSnapshot) -> Result<Json, String> {
        parse_json(&encode_semio_document_snapshot_json(snapshot))
    }

    fn disagreement(what: &str, got: &SemioDocumentSnapshot, expected: &SemioDocumentSnapshot) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_semio_document_snapshot_json(got), encode_semio_document_snapshot_json(expected))
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️Inputs
    /// 📄️ The real committed memo, parsed by this repository's own DSL codec.
    fn memo(ctx: &Context) -> Result<SemioDocumentSnapshot, String> {
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("the committed memo must be UTF-8: {error}"))?;
        parse_semio_document_dsl(&text)
    }

    /// 🦠️ The verb the scenario declares, read from the feature's own doc string through this
    /// subset's own JSON bridge — no transcription into Rust literals.
    fn declared(ctx: &Context) -> Result<SemioDocumentMutation, String> {
        decode_semio_document_mutation_json(ctx.doc_string()?)
    }

    /// 🧭️ `declared(ctx)`, except for the `no-mutation` scenario id. `NoMutation` was dropped from
    /// `SemioDocumentMutation` (`no` is not an APPROVED_VERB), so the committed doc string
    /// `{"mutation":"noMutation"}` no longer decodes; this maps that scenario onto
    /// `SetSnapshot(base.clone())` instead of failing, keeping the "nothing changes" law alive
    /// rather than deleting the scenario.
    fn declared_for(ctx: &Context, base: &SemioDocumentSnapshot) -> Result<SemioDocumentMutation, String> {
        if ctx.scenario.id.ends_with("no-mutation") {
            return Ok(SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base.clone() }));
        }
        declared(ctx)
    }

    fn run(current: &mut SemioDocumentSnapshot, mutation: &SemioDocumentMutation, what: &str) -> Result<(), String> {
        let outcome = apply_semio_document_mutation(current, mutation);
        let refusals = semio_mutation_refusals(&outcome);
        if refusals.is_empty() {
            return Ok(());
        }
        Err(format!("{what}: mutation rejected: {refusals:?}"))
    }

    fn member_text(vector: &Json, name: &str) -> Result<String, String> {
        Ok(vector.get(name).ok_or_else(|| format!("committed specification vector carries no {name:?} member"))?.to_string())
    }
    //#endregion 🔖️Inputs

    //#region 🔖️Handlers
    /// 🎯️ One verb applied to the real committed memo through the production entry point.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let mut current = memo(ctx)?;
        let mutation = declared_for(ctx, &current)?;
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// ↩️ The metamorphic inverse law on the real committed memo: applying the verb and then its OWN
    /// computed inverse must restore the artifact exactly. The MUTATED snapshot travels in the
    /// projection alongside the restored one, so the eighteen rows cannot all project the same
    /// restored value and compare vacuously.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let base = memo(ctx)?;
        let mutation = declared_for(ctx, &base)?;
        let mut current = base.clone();
        run(&mut current, &mutation, ctx.scenario.id.as_str())?;
        let mutated = projection(&current)?;
        for step in inverse_semio_document_mutation(&mutation, &base) {
            run(&mut current, &step, ctx.scenario.id.as_str())?;
        }
        if current != base {
            return Err(disagreement(&format!("{}: undoing the mutation did not restore the real committed memo", ctx.scenario.id), &current, &base));
        }
        let projection = Json::Object(vec![("mutated".to_string(), mutated), ("restored".to_string(), projection(&current)?)]);
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🧫️ The same verb on its committed `(before, mutation, after)` vector — a THIRD statement of
    /// what the verb means, independent of both implementations, kept from before this oracle
    /// existed rather than replaced by it.
    pub fn spec_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let vector = vector(ctx, kind)?;
            let mut current = snapshot_of(&vector, "before")?;
            let expected = snapshot_of(&vector, "after")?;
            let mutation = if kind == "no-mutation" { SemioDocumentMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: current.clone() }) } else { decode_semio_document_mutation_json(&member_text(&vector, "mutation")?)? };
            run(&mut current, &mutation, ctx.scenario.id.as_str())?;
            if current != expected {
                return Err(disagreement(&format!("{}: the applied snapshot does not match the committed after-snapshot", ctx.scenario.id), &current, &expected));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🔁️ The real memo through both of its committed encodings.
    /// 🔒️ **The byte half of the identity law — asserted as `carrier_is_exact` on BOTH files.**
    /// `.dsl.semio` is a fixed-layout record grammar and `.pack.semio` is its binary twin; the two
    /// committed example artifacts were produced by these very codecs, so reproducing them BYTE FOR
    /// BYTE is the correct answer here and `law::reparsed_not_copied` would be exactly backwards.
    /// Nor is it a self-comparison: the Python oracle re-emits the SAME two files, the text from
    /// the committed grammar and the binary from the committed protocol plus a record layout
    /// derived from the committed bytes, and the two sides' digests of what each emitted are
    /// compared.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let expected = snapshot_of(&vector(ctx, "no-mutation")?, "before")?;
        let text = String::from_utf8(ctx.fixture_bytes(DSL_ASSET)?).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_semio_document_dsl(&text)?;
        if parsed != expected {
            return Err(disagreement("identity-round-trip: parse_dsl of the real artifact does not match the committed before-snapshot the specification vectors start from", &parsed, &expected));
        }
        let printed = print_semio_document_dsl(&parsed);
        carrier_is_exact(printed.as_bytes(), text.as_bytes())?;
        let reparsed = parse_semio_document_dsl(&printed)?;
        if reparsed != expected {
            return Err(disagreement("identity-round-trip: printing the snapshot back to DSL and reparsing it lost content", &reparsed, &expected));
        }
        let pack_bytes = ctx.fixture_bytes(PACK_ASSET)?;
        let unpacked = decode_semio_document_pack(&pack_bytes)?;
        if unpacked != expected {
            return Err(disagreement("identity-round-trip: the committed binary twin decodes to a different snapshot than the committed text artifact", &unpacked, &expected));
        }
        let repacked_bytes = encode_semio_document_pack(&parsed);
        carrier_is_exact(&repacked_bytes, &pack_bytes)?;
        let repacked = decode_semio_document_pack(&repacked_bytes)?;
        if repacked != expected {
            return Err(disagreement("identity-round-trip: encoding the snapshot to a pack and decoding it back lost content", &repacked, &expected));
        }
        let projection = Json::Object(vec![
            ("document".to_string(), projection(&parsed)?),
            ("dslDigest".to_string(), Json::String(digest(printed.as_bytes()))),
            ("packDigest".to_string(), Json::String(digest(&repacked_bytes))),
            ("dslLength".to_string(), Json::Number(printed.len() as f64)),
            ("packLength".to_string(), Json::Number(repacked_bytes.len() as f64)),
        ]);
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly. Subject only — the oracle role
/// belongs to `🐍️component.py`.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        for kind in KINDS {
            built = built
                .subject(&format!("mutate-{kind}"), subject::mutate)
                .subject(&format!("inverse-{kind}"), subject::inverse)
                .subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
