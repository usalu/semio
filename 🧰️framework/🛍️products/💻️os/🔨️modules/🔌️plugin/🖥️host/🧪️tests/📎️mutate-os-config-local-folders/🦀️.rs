//! 🦀️ Local-folders exhaustive mutation case — Rust adapter. Recorded no-oracle decision
//! `os-config-local-folders-mutation-semantics` (`../../../../../🎚️config/🔮️oracles/🔣️.json`): `os.config.local-folders`
//! is this operating system's own persisted-local-only device record with no third-party implementation, so `oracle` here reads the committed,
//! independently handcrafted per-kind specification fixtures literally — no recomputation. `subject` drives this
//! repository's own `apply_local_folders_config_mutation_reporting` over the full two-kind vocabulary; every law is asserted
//! INSIDE the subject handler, because a recorded no-oracle case runs no oracle role.

use semio_repo_test_host::{parse_json, Adapter, Context, Json, Outcome};

//#region 🔖️Kinds
/// 🏷️ Mirrors the derive-generated `LocalFoldersConfigMutation` descriptor order — duplicated, not imported, because the
/// oracle-only build must not link the subject crate.
const KINDS: &[&str] = &["attach-local-folder", "detach-local-folder"];
//#endregion 🔖️Kinds

//#region 🔖️Fixtures
/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one kind, read literally.
fn fixture_text(kind: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match kind {
        "attach-local-folder" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/📎️attach-local-folder/🧫️fixtures/📎️remembers-the-folder-beside-another-document/🎯️outcome/🔣️.json"),
        ),
        "detach-local-folder" => (
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🦠️mutation/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../🎚️config/🧬️schema/🧬️mutations/✂️detach-local-folder/🧫️fixtures/✂️forgets-the-folder-and-keeps-its-sibling/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-os-config-local-folders: no specification vector registered for kind {other:?}"),
    }
}

/// 🔎️ Parses one embedded fixture file into the framework's own dependency-free `Json`.
fn canonical(text: &str) -> Json {
    parse_json(text).unwrap_or_else(|error| panic!("committed fixture JSON must parse: {error}"))
}
//#endregion 🔖️Fixtures

//#region 🔖️Oracle
/// 🔮️ The forward reference answer: the committed AFTER bindings, read literally.
fn mutate_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (_before, _mutation, after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
    }
}

/// 🔮️ The inverse reference answer: the committed BEFORE bindings.
fn inverse_oracle_for(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
    move |_ctx: &Context| {
        let (before, _mutation, _after, _outcome) = fixture_text(kind);
        Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
    }
}

/// 🔮️ The unbound-detachment guard's reference answer: the detach vector's committed after-bindings, unchanged.
fn unbound_guard_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (_before, _mutation, after, _outcome) = fixture_text("detach-local-folder");
    Ok(Outcome::with_raw(after.as_bytes().to_vec(), canonical(after)))
}

/// 🔁️ The round trip's reference answer: the detach vector's committed two-binding before-bindings itself.
fn round_trip_oracle(_ctx: &Context) -> Result<Outcome, String> {
    let (before, _mutation, _after, _outcome) = fixture_text("detach-local-folder");
    Ok(Outcome::with_raw(before.as_bytes().to_vec(), canonical(before)))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_plugin_host::opening_config::mutations::{
        apply_local_folders_config_mutation_reporting, decode_local_folders_config_mutation_json, decode_local_folder_bindings_json, encode_local_folder_bindings_json, inverse_local_folders_config_mutation_steps, LocalFolderBindings, LocalFoldersConfigMutation,
        LocalFolderRef,
    };
    use semio_repo_test_host::{parse_json, Context, Json, Outcome};

    //#region 🔖️FixtureDecode
    fn bindings_of(text: &str, label: &str, kind: &str) -> Result<LocalFolderBindings, String> {
        decode_local_folder_bindings_json(text).map_err(|error| format!("mutate-os-config-local-folders: the committed {label}-bindings for {kind:?} must decode: {error}"))
    }

    fn mutation_of(text: &str, kind: &str) -> Result<LocalFoldersConfigMutation, String> {
        decode_local_folders_config_mutation_json(text).map_err(|error| format!("mutate-os-config-local-folders: the committed mutation payload for {kind:?} must decode: {error}"))
    }

    fn projection(bindings: &LocalFolderBindings) -> Result<Json, String> {
        parse_json(&encode_local_folder_bindings_json(bindings))
    }

    fn disagreement(what: &str, got: &LocalFolderBindings, expected: &LocalFolderBindings) -> String {
        format!("{what}\n     got: {}\nexpected: {}", encode_local_folder_bindings_json(got), encode_local_folder_bindings_json(expected))
    }

    fn bound(bindings: &LocalFolderBindings, document_id: &str) -> bool {
        bindings.bindings.iter().any(|entry| entry.document_id == document_id)
    }
    //#endregion 🔖️FixtureDecode

    //#region 🔖️Laws
    /// 👁️ The observability law: the declared document is bound exactly when the row says so, and the sibling entry is
    /// identical before and after.
    fn listing_claims_hold(kind: &str, row: &Json, base: &LocalFolderBindings, after: &LocalFolderBindings) -> Result<(), String> {
        let document = row.str("document");
        let sibling = row.str("sibling");
        if base == after {
            return Err(format!("mutate-{kind}: the mutation left the bindings unchanged — the scenario would report a pass for a mutation it never observed"));
        }
        if bound(after, &document) != (row.str("bound") == "yes") {
            return Err(format!("mutate-{kind}: the feature declares {document:?} bound = {:?} afterwards, but the bindings says otherwise", row.str("bound")));
        }
        let before_sibling = base.bindings.iter().find(|entry| entry.document_id == sibling);
        let after_sibling = after.bindings.iter().find(|entry| entry.document_id == sibling);
        if before_sibling.is_none() || before_sibling != after_sibling {
            return Err(format!("mutate-{kind}: the sibling {sibling:?} must survive untouched"));
        }
        Ok(())
    }

    /// 🎯️ Both committed vectors are clean `applied` vectors, so any diagnostic at all is a divergence.
    fn outcome_matches(kind: &str, declared: &Json, raised: &[(String, String)]) -> Result<(), String> {
        if declared.str("status") != "applied" {
            return Err(format!("mutate-{kind}: both committed local-folders vectors are clean applied vectors, but this one declares {:?}", declared.str("status")));
        }
        if !raised.is_empty() {
            return Err(format!("mutate-{kind}: the committed outcome declares a clean `applied`, but the implementation raised {raised:?}"));
        }
        Ok(())
    }
    //#endregion 🔖️Laws

    //#region 🔖️Handlers
    /// 🎯️ Applies the kind to the committed before-bindings and asserts, in role, that the result IS the committed
    /// after-bindings, that the binding and sibling claims hold, and that the reported diagnostics are the committed ones.
    pub fn mutate(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |ctx: &Context| {
            let row = ctx.doc_json()?;
            let (before, mutation, after, outcome) = super::fixture_text(kind);
            let base = bindings_of(before, "before", kind)?;
            let expected = bindings_of(after, "after", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_local_folders_config_mutation_reporting(&mut current, &mutation);
            if current != expected {
                return Err(disagreement(&format!("mutate-{kind}: the applied bindings does not match the committed after-bindings"), &current, &expected));
            }
            outcome_matches(kind, &parse_json(outcome)?, &raised)?;
            listing_claims_hold(kind, &row, &base, &current)?;
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// ↩️ The metamorphic inverse law: applying the kind and then its OWN computed inverse restores the committed
    /// before-bindings exactly, folders included.
    pub fn inverse(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {
        move |_ctx: &Context| {
            let (before, mutation, _after, _outcome) = super::fixture_text(kind);
            let base = bindings_of(before, "before", kind)?;
            let mutation = mutation_of(mutation, kind)?;
            let mut current = base.clone();
            let raised = apply_local_folders_config_mutation_reporting(&mut current, &mutation);
            if !raised.is_empty() {
                return Err(format!("inverse-{kind}: the forward mutation was rejected: {raised:?}"));
            }
            if current == base {
                return Err(format!("inverse-{kind}: the forward mutation left the bindings untouched, so restoring it proves nothing"));
            }
            for step in inverse_local_folders_config_mutation_steps(&mutation, &base).expect("valid retained mutation inverse fixture") {
                let undone = apply_local_folders_config_mutation_reporting(&mut current, &step);
                if !undone.is_empty() {
                    return Err(format!("inverse-{kind}: an inverse step was rejected: {undone:?}"));
                }
            }
            if current != base {
                return Err(disagreement(&format!("inverse law violated: applying {kind:?} and then its own inverse did not restore the original"), &current, &base));
            }
            let projection = projection(&current)?;
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }
    }

    /// 🚧️ The branch no committed vector can express: detaching a document the bindings do not hold changes nothing, raises
    /// only the no-op warning and offers no undo step (an undo would have to invent the folder).
    pub fn unbound_guard(_ctx: &Context) -> Result<Outcome, String> {
        let (_before, mutation, after, _outcome) = super::fixture_text("detach-local-folder");
        let base = bindings_of(after, "after", "detach-local-folder")?;
        if bound(&base, "cad.drawing.fixture") {
            return Err("unbound-detachment-has-no-undo: the committed after-bindings of the detach vector must no longer bind cad.drawing.fixture".to_string());
        }
        let mutation = mutation_of(mutation, "detach-local-folder")?;
        let mut current = base.clone();
        let raised = apply_local_folders_config_mutation_reporting(&mut current, &mutation);
        if raised.iter().map(|(code, _)| code.as_str()).collect::<Vec<_>>() != vec!["mutation.no-op"] {
            return Err(format!("unbound-detachment-has-no-undo: detaching an unbound document must raise exactly the no-op warning, but raised {raised:?}"));
        }
        if current != base {
            return Err(disagreement("unbound-detachment-has-no-undo: detaching an unbound document must leave the bindings exactly where it was", &current, &base));
        }
        let steps = inverse_local_folders_config_mutation_steps(&mutation, &base).expect("valid retained mutation inverse fixture");
        if !steps.is_empty() {
            return Err(format!("unbound-detachment-has-no-undo: the inverse must be empty, but the implementation offered {} step(s)", steps.len()));
        }
        let projection = projection(&current)?;
        Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
    }

    /// 🔁️ The round trip for a record whose only carrier is its own JSON projection, proven real by reading both ids and
    /// the puzzle's folder path back off the TYPED value.
    pub fn round_trip(_ctx: &Context) -> Result<Outcome, String> {
        let (before, _mutation, _after, _outcome) = super::fixture_text("detach-local-folder");
        let bindings = bindings_of(before, "before", "detach-local-folder")?;
        let ids: Vec<&str> = bindings.bindings.iter().map(|entry| entry.document_id.as_str()).collect();
        if ids != ["cad.drawing.fixture", "puzzle.2d.fixture"] || bindings.bindings[1].folder != (LocalFolderRef::Path { path: "/Users/ada/Documents/puzzles".to_string() }) {
            return Err(format!("local-folders-round-trip: the committed bindings attach cad.drawing.fixture and puzzle.2d.fixture to /Users/ada/Documents/puzzles, but the decoded value holds {}", encode_local_folder_bindings_json(&bindings)));
        }
        let reencoded = encode_local_folder_bindings_json(&bindings);
        let reparsed = bindings_of(&reencoded, "re-encoded", "detach-local-folder")?;
        if reparsed != bindings {
            return Err(disagreement("local-folders-round-trip: decoding the re-encoded bindings did not reproduce the typed value", &reparsed, &bindings));
        }
        let projection = parse_json(&reencoded)?;
        Ok(Outcome::with_raw(reencoded.into_bytes(), projection))
    }
    //#endregion 🔖️Handlers
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls, by FULL expanded scenario id (the feature's `Examples` tables).
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle_for(kind)).oracle(&format!("inverse-{kind}"), inverse_oracle_for(kind));
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate(kind)).subject(&format!("inverse-{kind}"), subject::inverse(kind).expect("valid retained mutation inverse fixture"));
        }
    }
    built = built.oracle("unbound-detachment-has-no-undo", unbound_guard_oracle).oracle("local-folders-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("unbound-detachment-has-no-undo", subject::unbound_guard).subject("local-folders-round-trip", subject::round_trip);
    }
    built
}
//#endregion 🔖️Registration
