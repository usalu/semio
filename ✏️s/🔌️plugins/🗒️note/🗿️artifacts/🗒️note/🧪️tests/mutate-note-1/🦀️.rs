//! 🦀️ Note document exhaustive mutation case — Rust adapter. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
//!
//! Recorded no-oracle decision `note-document-mutation-semantics`
//! (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🔣️oracle.json`): `s.note.note` is a
//! semio-NATIVE artifact with no third-party reader or writer, so this case registers SUBJECT
//! handlers only. That is not an omission — the runner resolves an oracle implementation from the
//! feature's `@oracle-` tag, this feature carries `@no-oracle-` instead, and the oracle role is
//! therefore never dispatched for it. Registering an oracle handler here would be dead code that
//! reads as coverage in every listing, so there is none; every law this case claims is asserted
//! inside the subject handlers, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module, whose helpers are dependency-free
//! and format-neutral by their own doc comment.
//!
//! The evidence rests on the committed `(before, mutation, after, outcome)` specification vector
//! under each of the 33 `🧬️mutations/<slug>/🧪️tests/<fixture>/` leaves. Those files are read HERE
//! through `asset://`, so the plan pins their digests and a silently edited vector changes the plan
//! rather than the result. Nothing is transcribed into this file: the mapping from kind to vector
//! is data in the feature's own `Examples` table, and the only literal this adapter carries is the
//! kind list the production module's `kinds_match_the_enum_and_the_catalog` keeps honest.
//!
//! This subset needs no working-scene seeding, unlike forms and playbook: note's blocks and assets
//! are persisted IN the snapshot, so a committed `⬅️before` is the whole before-state and the
//! handler has nothing to reconstruct. That is why all 33 of its vectors could be authored as
//! effect vectors in the first place.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
#[cfg(feature = "sut")]
/// 🏷️ Mirrors `NoteMutation::KINDS`
/// (`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not
/// imported, because the oracle-only build must not link the subject crate. The production module's
/// own `kinds_match_the_enum_and_the_catalog` keeps that list honest against the enum and the
/// catalog; the contract gate keeps this case honest against the catalog.
const KINDS: &[&str] = &[
    "rename-note",
    "change-grid-visible",
    "change-grid-spacing",
    "change-grid-subdivisions",
    "change-grid-opacity",
    "change-snap-enabled",
    "change-snap-grid-spacing",
    "change-pencil-width",
    "change-eraser-radius",
    "create-asset",
    "replace-asset-payload",
    "delete-asset",
    "create-block",
    "delete-block",
    "delete-blocks",
    "duplicate-block",
    "duplicate-blocks",
    "move-block-to-container",
    "drag-blocks",
    "rename-block",
    "change-block-visible",
    "change-block-locked",
    "move-block",
    "resize-block",
    "change-block-font-size",
    "edit-block-text",
    "edit-block-math",
    "change-block-ink-width",
    "edit-block-ink-stroke",
    "insert-table-row",
    "remove-table-row",
    "insert-table-column",
    "remove-table-column",
];

#[cfg(feature = "sut")]
/// 👁️ Deliberately EMPTY: every one of this subset's 33 committed vectors moves the
/// document, so no kind is exempted from the observability law. A kind added here later would have
/// to state its reason in this doc comment and in the feature description, per the shared law
/// module's own contract.
const GUARD_VECTORS: &[&str] = &[

];

#[cfg(feature = "sut")]
/// 🧫️ Where a `<vector>` cell from the feature's `Examples` tables is rooted, relative to this
/// case's owner — the artifact root, which is what `asset://` resolves against.
const VECTORS: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations";

#[cfg(feature = "sut")]
/// 📄️ The real committed example document, in this subset's own `.dsl.semio` text envelope.
const EXAMPLE_ASSET: &str = "asset://🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_note::artifacts::note::standards::v1::subsets::any::io::snapshot::text::{parse_note_dsl, print_note_dsl};
    use semio_s_plugin_note::artifacts::note::standards::v1::subsets::any::schema::mutations::{apply_note_mutation_outcome, decode_note_mutation_json, decode_note_snapshot_json, encode_note_snapshot_json, inverse_note_mutation_steps, NoteMutation};
    use semio_s_plugin_note::artifacts::note::NoteSnapshot;

    //#region 🔖️VectorReading
    /// 🧫️ The scenario's own doc string, which carries the kind and the committed vector directory
    /// the row addresses. The mapping from kind to vector is DATA in the feature's `Examples` table,
    /// so this adapter holds no table of its own that could drift from it.
    fn addressed(ctx: &Context) -> Result<(String, String, Json), String> {
        let spec = ctx.doc_json()?;
        let (kind, vector) = (spec.str("kind"), spec.str("vector"));
        if kind.is_empty() || vector.is_empty() {
            return Err(format!("the scenario doc string must carry both a \"kind\" and a \"vector\", got {}", spec.to_string()));
        }
        Ok((kind, vector, spec))
    }

    fn text_at(ctx: &Context, vector: &str, leaf: &str) -> Result<String, String> {
        let uri = format!("{}/{vector}/{leaf}", super::VECTORS);
        String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("the committed vector {uri} is not UTF-8: {error}"))
    }

    fn snapshot_at(ctx: &Context, vector: &str, leaf: &str, kind: &str) -> Result<NoteSnapshot, String> {
        decode_note_snapshot_json(&text_at(ctx, vector, leaf)?).map_err(|error| format!("the committed {leaf} vector for {kind:?} must decode: {error}"))
    }

    fn mutation_at(ctx: &Context, vector: &str, kind: &str) -> Result<NoteMutation, String> {
        decode_note_mutation_json(&text_at(ctx, vector, "🦠️mutation/🔣️.json")?).map_err(|error| format!("the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(snapshot: &NoteSnapshot) -> Result<Json, String> {
        parse_json(&encode_note_snapshot_json(snapshot))
    }

    /// 🚨️ The `mutation.*` codes a committed `🎯️outcome` vector declares. Two shapes are in use and
    /// both are read here rather than one being assumed: an APPLIED outcome carries its diagnostics
    /// in a `messages` array, a REJECTED one carries the single `code` that refused it.
    fn declared_codes(outcome: &Json) -> Vec<String> {
        let listed: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).filter(|code| !code.is_empty()).collect();
        if !listed.is_empty() {
            return listed;
        }
        match outcome.str("code").as_str() {
            "" => Vec::new(),
            single => vec![single.to_string()],
        }
    }
    //#endregion 🔖️VectorReading

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to its committed before-snapshot and asserts three things in role: the
    /// result IS the committed after-snapshot, the raised diagnostic codes ARE the ones the
    /// committed `🎯️outcome` vector declares, and — for every kind not named in `GUARD_VECTORS` —
    /// the mutation actually moved the compared projection.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, _spec) = addressed(ctx)?;
        let base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let expected = snapshot_at(ctx, &vector, "📸️snapshot/➡️after/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let declared = parse_json(&text_at(ctx, &vector, "🎯️outcome/🔣️.json")?)?;
        let mut current = base.clone();
        let outcome = apply_note_mutation_outcome(&mut current, &mutation);
        let raised: Vec<String> = outcome.messages().iter().map(|message| message.code.0.clone()).collect();
        if raised != declared_codes(&declared) {
            return Err(format!("mutate-{kind}: raised {raised:?}, the committed 🎯️outcome vector declares {:?}", declared_codes(&declared)));
        }
        let (produced, wanted, before) = (projection(&current)?, projection(&expected)?, projection(&base)?);
        if let Some(first) = law::divergence(&produced, &wanted) {
            return Err(format!("mutate-{kind}: the applied snapshot is not the committed after-snapshot — {first}"));
        }
        law::mutation_is_observable(&kind, &produced, &before, super::GUARD_VECTORS)?;
        Ok(Outcome::with_raw(produced.to_string().into_bytes(), produced))
    }

    /// ↩️ The inverse law, asserted in role: applying the kind and then its OWN computed inverse
    /// must land back on the committed before-snapshot's projection, field for field, with no
    /// tolerance and no ignored key.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let (kind, vector, _spec) = addressed(ctx)?;
        let base = snapshot_at(ctx, &vector, "📸️snapshot/⬅️before/🔣️.json", &kind)?;
        let mutation = mutation_at(ctx, &vector, &kind)?;
        let original = projection(&base)?;
        let mut current = base.clone();
        apply_note_mutation_outcome(&mut current, &mutation);
        for step in inverse_note_mutation_steps(&mutation, &base) {
            apply_note_mutation_outcome(&mut current, &step);
        }
        let restored = projection(&current)?;
        law::inverse_restores(&kind, &restored, &original)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    /// 🔁️ The real committed artifact, parsed and printed back. Two laws, both in role: the
    /// reparsed document must carry the same projection, and the printed text must reproduce the
    /// committed bytes EXACTLY. The exact-bytes half is `carrier_is_exact` rather than the wave's
    /// usual no-byte-pass-through tripwire because the committed `🗣️.dsl.semio` is this
    /// subset's OWN printer's output — the repository generated it from this very codec — so
    /// reproducing it is the correct answer and any drift between the committed artifact and the
    /// printer is the defect this scenario exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_bytes(super::EXAMPLE_ASSET)?;
        let text = String::from_utf8(committed.clone()).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_note_dsl(&text)?;
        let printed = print_note_dsl(&parsed);
        let reparsed = parse_note_dsl(&printed)?;
        let (before, after) = (projection(&parsed)?, projection(&reparsed)?);
        law::round_trip_preserves(&after, &before)?;
        law::carrier_is_exact(printed.as_bytes(), &committed)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's two `Examples` tables exactly.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    for kind in KINDS {
        built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
    }
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
