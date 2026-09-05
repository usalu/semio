//! 🖍️ Drawing-document metadata mutation case — Rust adapter. Relocated out of the artifact-level
//! Duplicated verbatim (only relative paths adjusted) from `../../../🏷️metadata/🧪️tests/🪪️mutate-drawing-1-metadata/🦀️.rs` by shard F4 (this ticket) to close `unregistered-mutation-vocabulary` at the `✳️any/🧬️schema/🧬️mutations` + `✳️any/🚪️io/🧬️mutations` owner — same mechanism E3 already proved on `sequence`: reuse the already-manifested capability, no new v2 manifest entry.
//! `mutate-drawing-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION` so
//! this subset's own kinds have a subset-owned test. Recorded no-oracle decision
//! `drawing-mutation-semantics` (`../../🔮️oracle/🔣️.json`): `s.draw.drawing` is a
//! semio-NATIVE format with no third-party reader or writer, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification vectors literally — no recomputation, no second
//! implementation of drawing semantics — and `subject` drives this repository's own vocabulary over
//! this subset's `DrawingMutation` variants.
//!
//! **Where the assertion lives.** A recorded no-oracle case runs NO oracle role — the runner
//! resolves an oracle implementation from the feature's `@oracle-` tag and this feature has none —
//! so every law this case claims is asserted INSIDE the subject handler, through the shared
//! `✏️s/🔌️plugins/🗄️stdio/🧪️oracle/⚖️law` module.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};
use semio_s_plugin_stdio_test_oracle::law;

//#region 🔖️Kinds
/// 🏷️ This subset's own slice of `KINDS` in `../../../🏷️metadata/🧬️schema/🧬️mutations/🦀️.rs` — duplicated,
/// not imported, because the oracle-only build must not link the subject crate.
const KINDS: &[&str] = &["rename-layer", "set-layer-locked", "set-layer-visible"];

/// 👁️ Kinds whose committed specification vector declares NO movement — a refusal or an
/// accepted no-op — so the observability law must not be claimed for them.
const UNOBSERVABLE: &[&str] = &[];

//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after)` specification vector for one kind, read literally
/// via `include_str!` — this IS the independently handcrafted evidence the no-oracle decision rests
/// on, never recomputed here.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str) {
    match kind {
        "rename-layer" => (
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames-shape-a-without-touching-its-id/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames-shape-a-without-touching-its-id/🦠️mutation/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/✏️rename-layer/🧪️tests/✏️renames-shape-a-without-touching-its-id/📸️snapshot/➡️after/🔣️.json"),
        ),
        "set-layer-locked" => (
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/🔒️locks-shape-a/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/🔒️locks-shape-a/🦠️mutation/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/🔒️set-layer-locked/🧪️tests/🔒️locks-shape-a/📸️snapshot/➡️after/🔣️.json"),
        ),
        "set-layer-visible" => (
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/🙈️hides-shape-a/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/🙈️hides-shape-a/🦠️mutation/🔣️.json"),
            include_str!("../../../🏷️metadata/🧬️schema/🧬️mutations/👁️set-layer-visible/🧪️tests/🙈️hides-shape-a/📸️snapshot/➡️after/🔣️.json"),
        ),
        other => panic!("mutate-drawing-1-metadata: {other:?} is not a declared kind of this subset"),
    }
}

/// 🔣️ A committed fixture parsed through the platform's own JSON reader.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("mutate-drawing-1-metadata: a committed fixture must be valid JSON: {error}"))
}

/// 🚨️ This subset declares no no-op/refusal vectors, so every kind must raise nothing.
#[cfg(feature = "sut")]
fn declared_code(_kind: &str) -> Option<&'static str> {
    None
}

//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER document, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, after) = fixture_text(kind);
        law::mutation_is_observable(kind, &canonical(after), &canonical(before), UNOBSERVABLE)?;
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE document — undoing any mutation must land
/// exactly where its specification vector started.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_drawing::artifacts::drawing::standards::v1::subsets::any::schema::mutations::{apply_drawing_mutation_json, undo_drawing_mutation_json};

    /// 📥️ Splits a bridge answer into the resulting document and the diagnostic codes it raised.
    fn answer(text: &str) -> Result<(Json, Vec<String>), String> {
        let value = parse_json(text)?;
        let document = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let codes = value.array("messages").iter().map(|code| match code {
            Json::String(text) => text.clone(),
            other => other.to_string(),
        }).collect();
        Ok((document, codes))
    }

    /// 🚨️ A declared no-op or refusal must raise exactly the code its leaf's committed outcome names.
    fn raised(kind: &str, codes: &[String]) -> Result<(), String> {
        match super::declared_code(kind) {
            None => Ok(()),
            Some(code) if codes.iter().any(|raised| raised == code) => Ok(()),
            Some(code) => Err(format!("mutate-{kind}: the committed vector declares the diagnostic {code:?}, but applying it raised {codes:?}")),
        }
    }

    /// 🎯️ Applies the kind to its committed before-document and asserts the result IS the committed
    /// after-document, that the mutation moved the compared projection unless its own vector declares
    /// otherwise, and that a declared refusal really was refused.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, after) = super::fixture_text(kind);
            let (document, codes) = answer(&apply_drawing_mutation_json(before, mutation)?)?;
            let expected = super::canonical(after);
            if let Some(first) = law::divergence(&document, &expected) {
                return Err(format!("mutate-{kind}: the applied document does not match the committed after-document — {first}"));
            }
            law::mutation_is_observable(kind, &document, &super::canonical(before), super::UNOBSERVABLE)?;
            raised(kind, &codes)?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }

    /// ↩️ The inverse law in role: applying the kind and then EVERY step of its own computed inverse
    /// must restore the committed before-document — member positions included, which is what a
    /// delete undone by re-appending would fail.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after) = super::fixture_text(kind);
            let (document, _codes) = answer(&undo_drawing_mutation_json(before, mutation)?)?;
            law::inverse_restores(kind, &document, &super::canonical(before))?;
            Ok(Outcome::with_raw(document.to_string().into_bytes(), document))
        }
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registration is by FULL expanded scenario
/// id, so the loop mirrors the feature's `Examples` tables exactly.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind));
        }
    }
    built
}
//#endregion 🔖️Registration
