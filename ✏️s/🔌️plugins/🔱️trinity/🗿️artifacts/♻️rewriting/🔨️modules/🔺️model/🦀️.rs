//! 🔺️ Rewriting model: the `LayoutPoint` record, the rule schema id and the dialect coordinate every surface of this artifact shares.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

//#region 🔖️Types
/// 📍️ Local `{x, y}` twin for a bare `(f64, f64)` tuple — the DSL engine's `DslField` binding has no
/// impl for raw Rust tuples (only named `DslRecord`/`DslScalar` types can bind), so `rule_layout`'s
/// value type is this named record instead, with `From`/`Into` conversions at this crate's own
/// remaining `(f64, f64)` call sites (tests only — no production logic reads `rule_layout` today).
#[derive(Clone, Copy, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetainedClone, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct LayoutPoint {
    pub x: f64,
    pub y: f64,
}

impl From<(f64, f64)> for LayoutPoint {
    fn from((x, y): (f64, f64)) -> Self {
        Self { x, y }
    }
}

impl From<LayoutPoint> for (f64, f64) {
    fn from(point: LayoutPoint) -> Self {
        (point.x, point.y)
    }
}

pub const REWRITE_RULE_SCHEMA: &str = "trinity.rewrite.rule";

/// 🎯️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET: the one `Dialect` coordinate every
/// surface (editor AND viewer) of this artifact shares — lives at the ARTIFACT level, not under
/// `editor`, so a viewer file can read it without ever importing through the sibling `editor` module.
/// `artifact_kind = "s.trinity.rewriting"` matches `#[artifact_schema(id = "s.trinity.rewriting")]` in
/// this subset's own `🧬️schema/🦀️component.rs`; `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location — the canonical surface id is
/// `s.trinity.rewriting@1/*#editor` / `s.trinity.rewriting@1/*#viewer` (contract §1 grammar). NOT to be
/// confused with the unrelated, pre-existing `const DIALECT` inside
/// `derived_analysis::RewritingAnalyzerAnalysis` in this subset's `🧬️schema/🦀️component.rs` — a
/// different trait (`ArtifactAnalysis`), a different string (`"s.rewriting"`), out of scope here.
pub const TRINITY_REWRITING_DIALECT: semio_framework_artifact_reference::Dialect = semio_framework_artifact_reference::Dialect { artifact_kind: "s.trinity.rewriting", standard: semio_framework_artifact_reference::StandardId("1"), subset: semio_framework_artifact_reference::SubsetId::ANY };
//#endregion 🔖️Types

semio_framework_value::artifact_retire_struct!(LayoutPoint { x, y });

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Text × Document per owner-table (`text.rewriting`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "text.rewriting".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Trinity Rewrite Rule", "Trinity-Umschreibregel"),
        source_format: REWRITE_RULE_SCHEMA.into(),
        component_kind: "trinity".into(),
        dimension: "text".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: REWRITE_RULE_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
        import_stdio_kinds: vec!["stdio.json".into(), "stdio.txt".into()],
    }
}
//#endregion 🔖️ArtifactKind
