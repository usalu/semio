//! 🧬️ Opening-preferences config mutation dispatch enum — `os.config.opening`'s two handcrafted
//! kinds, each a single-field tuple wrapping its `🧬️mutations/<slug>/` triad payload. Hand-written
//! dispatch (not `#[derive(dsl::Mutations)]`) because this facet's `Diff` is the whole
//! `OpeningPreferences` value (see `📌️set-default-app`/`🧹clear-default-app`'s `🔺️diff` leaves),
//! matching `📕️norm`'s `NormConfigMutation` precedent rather than a sparse per-field diff type.

// 🩹️ W1-A wiring fix (this facet was authored "not yet wired into any crate's glue.rs" — see the
// module doc above): one hop short. Under the mount this facet's own leaf files already assume
// (`🧬️mutations` a real module wrapping this dispatch enum, itself inside the schema module — the
// SAME depth `📌️set-default-app`/`🧹clear-default-app`'s `use super::super::OpeningConfigMutation;`
// already require two supers to reach), this file's own module sits ONE level inside `🧬️mutations`,
// so `DefaultApp`/`OpeningPreferences` (declared at the schema level) need `super::super`, not `super`.
use super::super::OpeningPreferences;
use protocol::{Mutation, MutationDiff, MutationOutcome};
use serde::{Deserialize, Serialize};

//#region 🔖️Mutations
/// @emoji 🧬️ Typed, invertible opening-preferences mutation vocabulary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mutation", rename_all = "camelCase")]
pub enum OpeningConfigMutation {
    SetDefaultApp(SetDefaultApp),
    ClearDefaultApp(ClearDefaultApp),
}

impl Mutation<OpeningPreferences> for OpeningConfigMutation {
    type Diff = OpeningPreferences;

    /// 🧮️ Mechanical wrap only (26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-
    /// CONFLICTS W0): both leaves already return `MutationOutcome<OpeningPreferences>` (`Self::Diff`
    /// here IS `OpeningPreferences`, so no `.map` needed), forwarded as-is.
    fn diff(&self, base: &OpeningPreferences) -> MutationOutcome<OpeningPreferences> {
        match self {
            OpeningConfigMutation::SetDefaultApp(op) => super::set_default_app::diff::diff(op, base),
            OpeningConfigMutation::ClearDefaultApp(op) => super::clear_default_app::diff::diff(op, base),
        }
    }

    fn inverse(&self, base: &OpeningPreferences) -> Vec<Self> {
        match self {
            OpeningConfigMutation::SetDefaultApp(op) => super::set_default_app::inverse::inverse(op, base),
            OpeningConfigMutation::ClearDefaultApp(op) => super::clear_default_app::inverse::inverse(op, base),
        }
    }
}

/// 🧬️ Applies one pure opening-preferences mutation synchronously.
pub fn apply_opening_config_mutation(snapshot: &mut OpeningPreferences, mutation: &OpeningConfigMutation) -> protocol::MutationApplyResult<()> {
    *snapshot = mutation.diff(snapshot).diff().apply(snapshot)?;
    Ok(())
}

pub fn inverse_opening_config_mutation(snapshot: &OpeningPreferences, mutation: &OpeningConfigMutation) -> Vec<OpeningConfigMutation> {
    mutation.inverse(snapshot)
}

pub use super::clear_default_app::mutation::{clear_default_app, ClearDefaultApp};
pub use super::set_default_app::mutation::{set_default_app, SetDefaultApp};

/// 🏷️ Kebab-case spelling of every [`OpeningConfigMutation`] variant, in declaration order — the
/// vocabulary the `os-config-opening-1-any` mutation catalog (`../../🧪️oracle/🔣️component.json`)
/// declares and `mutate-os-config-opening`'s exhaustive case measures itself against. Two kinds and
/// no more: `OpeningPreferences` holds ONE list of `(dialect, role) -> app` pins, an upsert and an
/// idempotent removal cover it completely, and there is no `set-snapshot` because a preferences
/// document is replaced wholesale through the store's non-history path rather than through history.
/// [`kinds_match_the_enum_and_the_catalog`] keeps this list honest against the enum, since the
/// framework never parses Rust.
pub const KINDS: &[&str] = &["set-default-app", "clear-default-app"];

//#region 🌉️ExternalCodecBridge
/// 📥️ Decodes this facet's internally-tagged (`{"mutation": "setDefaultApp", …}`, camelCase payload
/// fields) JSON projection — exactly the shape the committed
/// `<slug>/🧪️tests/<fixture>/🦠️mutation/🔣️component.json` specification vectors carry — into a real
/// [`OpeningConfigMutation`]. The `mutate-os-config-opening` adapter cannot reach `serde_json` (the
/// generated test host links only `semio-repo-test-host` and the crate that mounts this facet) and
/// cannot name the private `protocol` extern-crate alias either, so the bridge belongs here.
pub fn decode_opening_config_mutation_json(text: &str) -> Result<OpeningConfigMutation, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// 📤️ Renders an [`OpeningPreferences`] as its own camelCase JSON projection — the comparison
/// surface the case's scenarios are measured through, and the shape the committed
/// `<slug>/🧪️tests/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️component.json` vectors are written in.
pub fn encode_opening_preferences_json(snapshot: &OpeningPreferences) -> String {
    serde_json::to_string(snapshot).expect("OpeningPreferences serialization is infallible")
}

/// 📥️ The inverse of [`encode_opening_preferences_json`].
pub fn decode_opening_preferences_json(text: &str) -> Result<OpeningPreferences, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}

/// ▶️ [`apply_opening_config_mutation`]'s reporting twin: applies `mutation` in place and returns
/// every diagnostic it raised as `(code, severity)` pairs. The plain apply discards them, and this
/// facet's whole degenerate surface is diagnostic-only — pinning an app that is already pinned, and
/// clearing a pin that was never set, both leave the document untouched and report
/// `mutation.no-op` at `Warning` — so the pair is the evidence rather than a side channel.
pub fn apply_opening_config_mutation_reporting(snapshot: &mut OpeningPreferences, mutation: &OpeningConfigMutation) -> Vec<(String, String)> {
    let outcome = mutation.diff(snapshot).apply_to(snapshot);
    outcome.messages().iter().map(|message| (message.code.0.clone(), format!("{:?}", message.level))).collect()
}

/// ↩️ The mutation's OWN computed undo steps, named for symmetry with the sibling artifact bridges —
/// what an `inverse-<kind>` scenario has to apply for the metamorphic law to mean anything.
pub fn inverse_opening_config_mutation_steps(mutation: &OpeningConfigMutation, base: &OpeningPreferences) -> Vec<OpeningConfigMutation> {
    mutation.inverse(base)
}
//#endregion 🌉️ExternalCodecBridge
//#endregion 🔖️Mutations

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    /// 🪪️ Declared at the schema level, so from inside this test module it needs one more `super`
    /// than the file-level `use super::super::OpeningPreferences;` above (see this file's header note).
    use super::super::super::DefaultApp;
    use super::*;
    use semio_framework::{AppRef, AppRole, ArtifactDialect};

    /// 🏷️ The three declarations of this vocabulary — the enum, [`KINDS`] and the committed catalog
    /// — must agree, in spelling AND in order. The framework never parses Rust, so without this test
    /// `KINDS` could drift from the enum and the catalog could keep measuring
    /// `mutate-os-config-opening` against a vocabulary the facet no longer has.
    ///
    /// ⚠️ This facet's dispatch enum is HAND-WRITTEN (see the file header: its `Diff` is the whole
    /// `OpeningPreferences` value), so there is no `#[derive(dsl::Mutations)]` descriptor table to
    /// read `kinds()` off. The enum side is therefore checked by constructing one value per variant
    /// and matching it exhaustively — a new variant makes the `match` non-exhaustive and fails to
    /// compile, which is the same guarantee the derive gives elsewhere.
    #[test]
    fn kinds_match_the_enum_and_the_catalog() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let every = [
            OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app }),
            OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Editor }),
        ];
        assert_eq!(KINDS.len(), every.len(), "KINDS must name exactly one entry per declared variant");
        for (kind, value) in KINDS.iter().zip(every.iter()) {
            let spelled = match value {
                OpeningConfigMutation::SetDefaultApp(_) => "set-default-app",
                OpeningConfigMutation::ClearDefaultApp(_) => "clear-default-app",
            };
            assert_eq!(*kind, spelled, "KINDS must match the enum's own declaration order and kebab-case spelling");
        }
        let manifest = include_str!("../../🧪️oracle/🔣️component.json");
        for kind in KINDS {
            assert!(manifest.contains(&format!("\"{kind}\"")), "KINDS entry {kind:?} must also appear in the committed oracle manifest's catalog");
        }
        assert!(manifest.contains("change-merge-policy"), "the sibling merge-policy facet is NOT mounted in any crate's 📦️glue.rs and therefore has no case — the manifest must keep saying so rather than letting the gate read this owner as fully covered");
    }

    #[test]
    fn set_default_app_and_clear_default_app_invert_each_other() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let base = OpeningPreferences::default();

        let set_op = OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() });
        let after_set = set_op.diff(&base).diff().apply(&base).expect("valid set-default diff");
        assert_eq!(after_set.defaults, vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }]);

        let undo = set_op.inverse(&base);
        assert_eq!(undo, vec![OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect: dialect.clone(), role: AppRole::Editor })]);
        let restored = undo[0].diff(&after_set).diff().apply(&after_set);
        assert_eq!(restored, Ok(base));
    }

    //#region 🔖️OutcomeLaws
    /// ✅️ §C2/fan-out-recipe laws (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS`):
    /// `set`/`clear` have no missing-target or Fatal-domain case (an upsert and an idempotent
    /// removal never reference an external entity that could be absent or invariant-violating), so
    /// the only real outcome-law surface here is `mutation.no-op` on the idempotent path.
    #[test]
    fn set_default_app_already_pinned_is_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let app = AppRef { plugin_id: "cad".to_string(), app_id: "s.cad.cad@1/*#editor".to_string() };
        let base = OpeningPreferences { defaults: vec![DefaultApp { dialect: dialect.clone(), role: AppRole::Editor, app: app.clone() }] };
        let outcome = OpeningConfigMutation::SetDefaultApp(SetDefaultApp { dialect, role: AppRole::Editor, app }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }

    #[test]
    fn clear_default_app_without_a_pin_is_no_op() {
        let dialect = ArtifactDialect { artifact_kind: "s.cad.cad".to_string(), standard: "1".to_string(), subset: "*".to_string() };
        let base = OpeningPreferences::default();
        let outcome = OpeningConfigMutation::ClearDefaultApp(ClearDefaultApp { dialect, role: AppRole::Viewer }).diff(&base);
        assert_eq!(outcome.worst_level(), Some(protocol::Severity::Warning));
        assert!(outcome.messages().iter().any(|message| message.code.0 == "mutation.no-op"));
        assert_eq!(outcome.diff(), &base);
    }
    //#endregion 🔖️OutcomeLaws
}
//#endregion 🧪️Tests
