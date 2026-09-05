//! 🦀️ Note document ink mutation case — Rust adapter. Relocated out of the artifact-level
//! `mutate-note-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! this subset's own kinds have a subset-owned test.
//!
//! Recorded no-oracle decision superseded: this case carries `@oracle-note-python-independent`, so
//! the runner DOES dispatch the oracle role — `🐍️.py` beside this file — and this file registers
//! SUBJECT handlers only, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law/🦀️.rs` module.
//!
//! The evidence rests on the committed `(before, mutation, after, outcome)` specification vector
//! under each of this subset's `🧬️mutations/<slug>/🧪️tests/<fixture>/` leaves. Those files are read
//! HERE through `asset://`, so the plan pins their digests and a silently edited vector changes the
//! plan rather than the result. Nothing is transcribed into this file: the mapping from kind to
//! vector is data in the feature's own `Examples` table, and the only literal this adapter carries is
//! the kind list the production module's `kinds_match_the_enum_and_the_catalog` keeps honest.

use semio_repo_test_host::Adapter;

//#region 🔖️Kinds
#[cfg(feature = "sut")]
/// 🏷️ This subset's own slice of `NoteMutation::KINDS`
/// (`../../🧬️schema/🧬️mutations/🦀️.rs`) — duplicated, not imported, because the oracle-only build
/// must not link the subject crate.
const KINDS: &[&str] = &["change-pencil-width", "change-eraser-radius", "change-block-ink-width", "edit-block-ink-stroke"];

#[cfg(feature = "sut")]
/// 👁️ Deliberately EMPTY: every one of this subset's committed vectors moves the document, so no
/// kind is exempted from the observability law.
const GUARD_VECTORS: &[&str] = &[

];

#[cfg(feature = "sut")]
/// 🧫️ Where a `<vector>` cell from the feature's `Examples` tables is rooted, relative to this
/// case's owner — this subset itself, which is what `asset://` resolves against.
const VECTORS: &str = "asset://🧬️schema/🧬️mutations";
//#endregion 🔖️Kinds

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
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
    built
}
//#endregion 🔖️Registration
