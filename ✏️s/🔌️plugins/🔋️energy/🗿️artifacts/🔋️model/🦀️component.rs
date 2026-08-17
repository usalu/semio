//! 🎪 Energy model artifact — headless BEM document surface over `crate::Model`.

pub use crate::artifacts::model::schema::diff::EnergyModelDiff;
pub use crate::artifacts::model::schema::mutations::EnergyModelMutation;
pub use crate::artifacts::model::schema::snapshot::EnergyModelSnapshot;

use semio_framework_plugin::{ArtifactKindSpec, Dialect, MediaClass, MediaForm, MediaType, OsMediaCapability, StandardId, SubsetId};

pub use crate::artifacts::model::schema::EnergyModelArtifact;

/// @emoji 🔖️ Document schema / DSL envelope id.

pub const ENERGY_MODEL_DOCUMENT_SCHEMA: &str = "energy.model";

/// @emoji 🧬️ Artifact schema descriptor id.
pub const ENERGY_MODEL_ARTIFACT_SCHEMA_ID: &str = "s.energy.model";

//#region 🔖️Dialect
/// 🪪️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1: the canonical surface-id
/// coordinate for this artifact's ONE subset (`✳️any`) — `s.energy.model@1/*`. Lives at the ARTIFACT
/// root (not under `✏️editor`/`👁️viewer`) so a viewer file can read it without ever importing
/// through the sibling editor module. `artifact_kind` matches this file's own snapshot schema id
/// (`EnergyModelSnapshot`'s `#[artifact_schema(id = "s.energy.model")]`, same as
/// `ENERGY_MODEL_ARTIFACT_SCHEMA_ID` above); `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location on disk.
pub const MODEL_DIALECT: Dialect = Dialect { artifact_kind: ENERGY_MODEL_ARTIFACT_SCHEMA_ID, standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️Composition
/// 🧩️ Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the
/// old `model_json: String` opaque-JSON field (a hand-rolled duplicate of exactly what
/// `s.stdio.semio.value` already generalizes — a typed, structured value tree) is replaced by two
/// fixed composed CHILD slots on `EnergyModelSnapshot`: `structure` (the SOLE lossless source of
/// truth for the whole `crate::model::Model`, folded into one `SemioValue::Map` via a generic
/// JSON bridge — `Model` already derives `Serialize`/`Deserialize`, so `serde_json::Value` is a
/// real, lossless intermediate) and `zones` (a DERIVED, non-authoritative tabular projection —
/// one row per `Zone`, always regenerated alongside `structure` from the SAME model, never an
/// independent source, so the two never diverge — same "one lossless source + one derived
/// convenience projection" split `forms`'s `structure`/`results` and `mathematical`'s
/// `notation`/`results` established). `referenced_model` is a new forward `ArtifactLink` slot
/// (role `"model"`) for the building/spatial model this energy model analyzes — per the design
/// map's `R:model` half. Grepped before writing this: energy has NO existing dependency on any
/// external artifact (`ArtifactLink`/`link_slot` appear nowhere in this plugin today) — like
/// `layout`'s own `referenced_model` finding, this is genuinely NEW forward capability, not a
/// duplication removal, so it is schema/codec-complete but left inert (no mutation dispatch, no
/// resolver read path) — documented honestly rather than wired to a fictional consumer.
///
/// Genuine exception, NOT composed: `crate::model::Model`'s `Surface.vertices_m: Vec<[f64; 3]>`
/// (and `ShadingSurface.vertices_m`) is raw 3D geometry embedded inline, which in principle
/// duplicates what an external spatial model would own — but folding it out into `referenced_model`
/// would mean rewriting how every one of the ~40 engine modules under `🔨️modules/⚡️simulation/
/// ⚙️engine/` (envelope/daylight/solar/geometry/…) resolves surface geometry, from a plain struct
/// field read to a link-resolution round trip through a not-yet-wired `LinkResolver` seam (per the
/// migration recipe's §3 finding: `VcsArtifactApp.children` has zero live content behind it for
/// any plugin yet) — a kernel-dissolution-scale change (DKM's own ticket), not a schema migration.
/// `vertices_m` stays inside `structure`'s lossless `Model` tree, same as every other field.

//#region 🔖️ChildTypes
pub type EnergyStructureChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot>;
pub type EnergyZonesChild = store::ArtifactChild<semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot>;
//#endregion 🔖️ChildTypes

//#region 🔖️Converters
/// 🌉 Generic, lossless `serde_json::Value` <-> `SemioValue` bridge — the SAME "both JSON-
/// equivalent" trade `forms`'s `semio_value_from_dsl`/`dsl_from_semio_value` makes for its own
/// JSON-shaped source. `crate::model::Model` has no byte-array or cross-artifact-ref fields, so
/// `SemioValue::Bytes`/`::Ref` are never PRODUCED by `energy_structure_from_model` below — they are
/// still handled (never a panic) for the theoretical case of a foreign composer writing one into
/// this artifact's own `structure` child.
fn semio_value_from_json(value: &serde_json::Value) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValue, SemioValueEntry};
    match value {
        serde_json::Value::Null => SemioValue::Null,
        serde_json::Value::Bool(value) => SemioValue::Bool { value: *value },
        serde_json::Value::Number(n) => {
            if n.is_i64() || n.is_u64() {
                SemioValue::Int { lexeme: n.to_string() }
            } else {
                SemioValue::Float { lexeme: n.to_string() }
            }
        }
        serde_json::Value::String(value) => SemioValue::Str { value: value.clone() },
        serde_json::Value::Array(items) => SemioValue::List { items: items.iter().map(semio_value_from_json).collect() },
        serde_json::Value::Object(entries) => SemioValue::Map { entries: entries.iter().map(|(key, value)| SemioValueEntry { key: key.clone(), value: semio_value_from_json(value) }).collect() },
    }
}

/// 🌉 Inverse of [`semio_value_from_json`] — real reconstruction, not a stub. `Bytes`/`Ref` degrade
/// honestly (documented above) since `Model` never produces either.
fn json_from_semio_value(value: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue) -> serde_json::Value {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    match value {
        SemioValue::Null => serde_json::Value::Null,
        SemioValue::Bool { value } => serde_json::Value::Bool(*value),
        SemioValue::Int { lexeme } => lexeme.parse::<i64>().map(serde_json::Value::from).unwrap_or_else(|_| serde_json::Value::String(lexeme.clone())),
        SemioValue::Float { lexeme } => lexeme.parse::<f64>().ok().and_then(serde_json::Number::from_f64).map(serde_json::Value::Number).unwrap_or_else(|| serde_json::Value::String(lexeme.clone())),
        SemioValue::Str { value } => serde_json::Value::String(value.clone()),
        SemioValue::Bytes { value } => serde_json::Value::Array(value.iter().map(|b| serde_json::Value::from(*b)).collect()),
        SemioValue::List { items } => serde_json::Value::Array(items.iter().map(json_from_semio_value).collect()),
        SemioValue::Map { entries } => serde_json::Value::Object(entries.iter().map(|entry| (entry.key.clone(), json_from_semio_value(&entry.value))).collect()),
        SemioValue::Ref { .. } => serde_json::Value::Null,
    }
}

/// 🌉 REAL bidirectional converter: the whole `Model` <-> one `s.stdio.semio.value` tree — the SOLE
/// lossless source of truth for this artifact's persisted content.
pub fn energy_structure_from_model(model: &crate::model::Model) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::{SemioValueSnapshot, STDIO_SEMIOVALUE_DOCUMENT_SCHEMA};
    let value = serde_json::to_value(model).unwrap_or(serde_json::Value::Null);
    SemioValueSnapshot { schema: STDIO_SEMIOVALUE_DOCUMENT_SCHEMA.into(), root: semio_value_from_json(&value), nodes: Vec::new() }
}

/// 🌉 Inverse of [`energy_structure_from_model`]. Falls back to `Model::default()` if `structure`'s
/// root doesn't decode into a full `Model` (e.g. a foreign composer wrote a partial/foreign tree) —
/// documented, honest degradation, never a panic, matching the recipe's converter-honesty rule.
pub fn energy_model_from_structure(structure: &semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot) -> crate::model::Model {
    serde_json::from_value(json_from_semio_value(&structure.root)).unwrap_or_default()
}

/// 🌉 REAL, DERIVED converter: one `s.stdio.semio.table` row per `Zone` (`id`/`name`/`volumeM3`/
/// `multiplier`/`conditioned`/`partOfTotalFloorArea`) — a non-authoritative convenience projection,
/// always regenerated alongside `structure` from the SAME model (never an independent source, so
/// the two never diverge). `energy_model_from_structure` alone is authoritative on read; this table
/// is never consulted for reconstruction, mirroring `forms`'s own `results` table exactly.
pub fn energy_zones_table_from_model(model: &crate::model::Model) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::{SemioTableCellKind, SemioTableColumn, SemioTableRow, SemioTableSnapshot, STDIO_SEMIOTABLE_DOCUMENT_SCHEMA};
    use semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValue;
    SemioTableSnapshot {
        schema: STDIO_SEMIOTABLE_DOCUMENT_SCHEMA.into(),
        columns: vec![
            SemioTableColumn { name: "id".into(), kind: SemioTableCellKind::Int },
            SemioTableColumn { name: "name".into(), kind: SemioTableCellKind::Str },
            SemioTableColumn { name: "volumeM3".into(), kind: SemioTableCellKind::Float },
            SemioTableColumn { name: "multiplier".into(), kind: SemioTableCellKind::Int },
            SemioTableColumn { name: "conditioned".into(), kind: SemioTableCellKind::Bool },
            SemioTableColumn { name: "partOfTotalFloorArea".into(), kind: SemioTableCellKind::Bool },
        ],
        rows: model
            .zones
            .iter()
            .map(|zone| SemioTableRow {
                cells: vec![
                    SemioValue::Int { lexeme: zone.id.0.to_string() },
                    SemioValue::Str { value: zone.name.clone() },
                    SemioValue::Float { lexeme: format!("{}", zone.volume_m3) },
                    SemioValue::Int { lexeme: zone.multiplier.to_string() },
                    SemioValue::Bool { value: zone.conditioned },
                    SemioValue::Bool { value: zone.part_of_total_floor_area },
                ],
            })
            .collect(),
    }
}

/// 🔎️ Real read accessor for the composed `structure` child's actual content — derives it fresh
/// from the working scene's cached `Model` every call (never stored separately, so it cannot drift
/// from `structure`'s handle). No render/export call site consumes this yet (energy is a headless
/// engine with no document app — see `📦️glue.rs`'s own "Shape note"), matching `layout`'s honest
/// framing for its own inert `referenced_model` slot: real, tested, not yet wired to a consumer.
pub fn energy_structure_content(snapshot: &EnergyModelSnapshot) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot {
    energy_structure_from_model(&energy_model(snapshot))
}

/// 🔎️ Twin of [`energy_structure_content`] for the `zones` child.
pub fn energy_zones_content(snapshot: &EnergyModelSnapshot) -> semio_s_plugin_stdio::artifacts::semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot {
    energy_zones_table_from_model(&energy_model(snapshot))
}
//#endregion 🔖️Converters

//#region 🔖️WorkingScene
/// 🌱 Ephemeral, session-side cache of the live `Model` behind a `(structure, zones)` handle pair —
/// NEVER persisted (matches the `EngineRep` contract: wholly derived, droppable at any instant,
/// rebuilt from base). No `LinkResolver`/child-dispatch seam exists in `ArtifactApp::handle` yet for
/// this plugin (checked directly against `🔌️plugin/🦀️component.rs`, W1-owned, read-only — same
/// standing gap every prior wave's report documents; also see the migration recipe's §3 finding that
/// `VcsArtifactApp.children` has zero live content behind it for any plugin yet), so this is the
/// only way a persisted content-addressed handle round-trips to the real `Model` within one process
/// — mirrors `mathematical`'s `MATH_SCRATCH`/`forms`'s `FORMS_SCRATCH`. `structure`/`zones` always
/// share ONE scene id (minted together from the SAME `Model` by [`energy_children_from_model`]), so
/// one cache entry serves both.
///
/// ⚠️ Same documented staleness gap as every prior exemplar: store-level undo/redo bypasses
/// `ArtifactApp::handle` entirely, so a handle can in principle go uncached (fresh process, or an
/// undo past this session's history). [`energy_model`] fails soft (`Model::default()`) rather than
/// panicking.
pub struct EnergyWorkingScene {
    pub model: crate::model::Model,
}

thread_local! {
    static ENERGY_SCRATCH: std::cell::RefCell<std::collections::HashMap<String, EnergyWorkingScene>> = std::cell::RefCell::new(std::collections::HashMap::new());
}

fn energy_scene_id(model: &crate::model::Model) -> String {
    use std::hash::{Hash, Hasher};
    let content_json = serde_json::to_string(model).unwrap_or_default();
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    content_json.hash(&mut hasher);
    format!("energy-scene-{:016x}", hasher.finish())
}

/// 🏗️ Mints both composed-child handles for a `Model` AND seeds the scratch cache in one call — the
/// standard way this plugin's mutation-diff/fixture builders create `structure`/`zones` field
/// values; never construct these handles without also caching, or [`energy_model`] will read back
/// `Model::default()`.
pub fn energy_children_from_model(model: &crate::model::Model) -> (EnergyStructureChild, EnergyZonesChild) {
    let scene_id = energy_scene_id(model);
    ENERGY_SCRATCH.with(|cache| {
        cache.borrow_mut().insert(scene_id.clone(), EnergyWorkingScene { model: model.clone() });
    });
    let dialect_for = |subset: &str| store::os_io::ArtifactDialect { artifact_kind: "s.stdio.semio".into(), standard: "v1".into(), subset: subset.into() };
    let target_for = |subset: &str| store::os_io::ArtifactRef { artifact_id: format!("energy-{subset}"), dialect: dialect_for(subset) };
    (store::ArtifactChild::new(scene_id.clone(), target_for("value")), store::ArtifactChild::new(scene_id, target_for("table")))
}

/// 🔎️ Reads the cached working scene behind a snapshot's composed children — `Model::default()`
/// (never a panic) on a cache miss, per this region's own doc comment.
pub fn energy_scene(snapshot: &EnergyModelSnapshot) -> EnergyWorkingScene {
    ENERGY_SCRATCH.with(|cache| cache.borrow().get(&snapshot.structure.child_id).map(|scene| EnergyWorkingScene { model: scene.model.clone() })).unwrap_or_else(|| EnergyWorkingScene { model: crate::model::Model::default() })
}

/// 🔎️ The live `Model` behind a snapshot's composed children — the single read call site every
/// consumer in this plugin now uses instead of the old `.model_json` field (decode-on-demand
/// through `model_from_snapshot`, below).
pub fn energy_model(snapshot: &EnergyModelSnapshot) -> crate::model::Model {
    energy_scene(snapshot).model
}

/// 🏗️ Builds a full `EnergyModelSnapshot` from a literal `Model` — the standard fixture/import
/// constructor replacing the old `model_json: String` struct literal now that `structure`/`zones`
/// are composed child handles, not a plain field.
pub fn energy_snapshot_with_state(schema: impl Into<String>, model: crate::model::Model, referenced_model: Option<store::ArtifactLink>) -> EnergyModelSnapshot {
    let (structure, zones) = energy_children_from_model(&model);
    EnergyModelSnapshot { schema: schema.into(), structure, zones, referenced_model }
}
//#endregion 🔖️WorkingScene
//#endregion 🔖️Composition

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — Data × Value per owner-table (`data.🔋️model`).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "data.🔋️model".into(),
        name: "Energy Model".into(),
        source_format: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        component_kind: "energy".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: ENERGY_MODEL_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
        import_stdio_kinds: vec!["stdio.csv", "stdio.json", "stdio.xlsx", "stdio.zip"],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated
/// off `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the old side-effecting `register()`, which the plugin root called
/// unconditionally (energy has no document apps, so there was never a `.setup()` narrowing here to
/// begin with). The retired root-level registrar only called `register_composer_entries(v1::entries())`
/// — exactly what `.composers(...)` below now does through `register_all` — so no imperative alias
/// remains. `.composers(...)` reaches `🚪️io/🦀️component.rs`'s own `io_registry` module (the one with
/// the actual `ComposerEntry` rows) by its full path —
/// ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES relocated it off the now-deleted
/// `⚙️engine` (an artifact is a schema + io system, never an engine) into `🚪️io/`, updating this
/// qualified reference in lockstep.
/// `register_document_codec()` — folded into this declaration via `.document_codec_bare::<Snapshot,
/// Mutation>(schema)` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1d): `.document_codec::<A:
/// ArtifactApp>()` requires an `ArtifactApp` to bind `A::Snapshot`/`A::Mutation`, and this plugin is a
/// headless library with ZERO apps — there is no `ArtifactApp` to name. `document_codec_bare` is the
/// new sibling closing exactly that gap (see its own doc); the old free fn in `⚙️engine` is deleted
/// with this — nothing else called it.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[(&str, &str, &str, &[(&str, &str)], Option<(&str, &str)>)] = &[
        ("s.model.standard.v1", "standard", "1", &[], None),
        ("s.model.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.model.schema.artifact", "schema", "s.energy.model", &[("schema", "s.energy.model")], None),
        ("s.model.inference.artifact", "inference", "s.energy.model.inference", &[("schema", "s.energy.model.inference")], None),
        ("s.model.composer.native", "composer", "s.model@1/*", &[("dialect", "s.model@1/*")], None),
        ("s.model.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.model.composer.format-2", "composer", "s.stdio.csv@rfc4180/*", &[("dialect", "s.stdio.csv@rfc4180/*")], None),
        ("s.model.composer.format-3", "composer", "s.stdio.xlsx@ecma-376/*", &[("dialect", "s.stdio.xlsx@ecma-376/*")], None),
        ("s.model.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.model.grammar.1", "grammar", "energy.model", &[("grammar", "energy.model")], None),
        ("s.model.grammar.2", "grammar", "energy.model.op", &[("grammar", "energy.model.op")], None),
        ("s.model.grammar.3", "grammar", "energy.model.diff", &[("grammar", "energy.model.diff")], None),
        ("s.model.grammar.4", "grammar", "energy.model.pack", &[("grammar", "energy.model.pack")], None),
        ("s.model.grammar.5", "grammar", "energy.model.spr", &[("grammar", "energy.model.spr")], None),
        ("s.model.codec.document-1", "codec", "energy.model:model", &[("codec", "energy.model"), ("extension", "model")], None),
        ("s.model.localization.en", "localization", "Energy Model", &[], Some(("en", "Energy Model"))),
        ("s.model.localization.de", "localization", "Energiemodell", &[], Some(("de", "Energiemodell"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.model")?);
    for (identity, kind, descriptor, claims, localization) in rows {
        let mut capability = ArtifactCapability::new(ArtifactIdentity::parse(*identity)?, ArtifactCapabilityKind::parse(*kind)?).descriptor(descriptor.as_bytes())?;
        for (namespace, value) in *claims {
            capability = capability.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(*namespace)?, *value)?)?;
        }
        if let Some((locale, text)) = localization {
            capability = capability.localization(ArtifactLocalization::new(ArtifactLocale::parse(*locale)?, *text)?)?;
        }
        definition = definition.capability(capability)?;
    }
    Ok(definition)
}

pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::model::standards::v1::subsets::any::schema::energy_model_artifact_schema_descriptor())
        .inferences([crate::artifacts::model::standards::v1::subsets::any::schema::inferences::energy_model_artifact_inference_descriptor()])
        .composers(crate::artifacts::model::standards::v1::subsets::any::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA)
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🗒️note` exemplar's helper of the same shape.
fn pilot_languages() -> &'static [dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                dsl::LanguageSpec {
                    id: "energy.model",
                    extension: Some("energy"),
                    role: dsl::LanguageRole::Document,
                    grammar: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::dsl::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.op",
                    extension: None,
                    role: dsl::LanguageRole::Ops,
                    grammar: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::op::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.op"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.diff",
                    extension: None,
                    role: dsl::LanguageRole::Diff,
                    grammar: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::artifacts::model::diff::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: dsl::passthrough_hooks("energy.model.diff"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.pack",
                    extension: None,
                    role: dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::snapshot::pack::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.pack"),
                },
                dsl::LanguageSpec {
                    id: "energy.model.spr",
                    extension: None,
                    role: dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::artifacts::model::spr::COMPONENT_PROTOCOL_PATH),
                    hooks: dsl::passthrough_hooks("energy.model.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration
