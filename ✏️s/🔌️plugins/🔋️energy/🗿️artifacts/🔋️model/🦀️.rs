//! 🎪 Energy model artifact — headless BEM document surface over `crate::model::Model`.

#![allow(clippy::too_many_arguments)]

extern crate semio_s_artifact_energy_model_diff as diff;
extern crate semio_s_artifact_energy_model_inferences as inferences;
extern crate semio_s_artifact_energy_model_io as io;
extern crate semio_s_artifact_energy_model_mutations as mutations;
extern crate semio_s_artifact_energy_model_schema as schema;
extern crate semio_s_artifact_energy_model_snapshot as snapshot;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_model as model;
#[cfg(test)]
extern crate semio_s_plugin_energy_engine_schedule as schedule;
#[cfg(test)]
extern crate semio_framework_os_kernel as store;

use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use crate::snapshot::{ENERGY_MODEL_DOCUMENT_SCHEMA, EnergyModelSnapshot};
use crate::mutations::EnergyModelMutation;

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, relocated
/// off `⚙️engine` to the artifact root — `declaration()` describes the artifact itself, not engine
/// behaviour) — replaces the old side-effecting `register()`, which the plugin root called
/// unconditionally (energy has no document apps, so there was never a `.setup()` narrowing here to
/// begin with). The retired root-level registrar only called `register_composer_entries(v1::entries())`
/// — exactly what `.composers(...)` below now does through `register_all` — so no imperative alias
/// remains. `.composers(...)` reaches `🚪️io/🦀️.rs`'s own `io_registry` module (the one with
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

    type CapabilityRow<'a> = (&'a str, &'a str, &'a str, &'a [(&'a str, &'a str)], Option<(&'a str, &'a str)>);
    let rows: &[CapabilityRow<'_>] = &[
        ("s.energy.model.standard.v1", "standard", "1", &[], None),
        ("s.energy.model.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.energy.model.schema.artifact", "schema", "s.energy.model", &[("schema", "s.energy.model")], None),
        ("s.energy.model.inference.artifact", "inference", "s.energy.model.inference", &[("schema", "s.energy.model.inference")], None),
        ("s.energy.model.composer.native", "composer", "s.energy.model@1/*", &[("dialect", "s.energy.model@1/*")], None),
        ("s.energy.model.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.energy.model.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.energy.model.grammar.1", "grammar", "energy.model", &[("grammar", "energy.model")], None),
        ("s.energy.model.grammar.2", "grammar", "energy.model.op", &[("grammar", "energy.model.op")], None),
        ("s.energy.model.grammar.3", "grammar", "energy.model.diff", &[("grammar", "energy.model.diff")], None),
        ("s.energy.model.grammar.4", "grammar", "energy.model.pack", &[("grammar", "energy.model.pack")], None),
        ("s.energy.model.grammar.5", "grammar", "energy.model.spr", &[("grammar", "energy.model.spr")], None),
        ("s.energy.model.codec.document-1", "codec", "energy.model:energy", &[("codec", "energy.model"), ("codec-extension", "12:energy.model:energy")], None),
        ("s.energy.model.localization.en", "localization", "Energy Model", &[], Some(("en", "Energy Model"))),
        ("s.energy.model.localization.de", "localization", "Energiemodell", &[], Some(("de", "Energiemodell"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.energy.model")?);
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
        .schema(crate::schema::energy_model_artifact_schema_descriptor())
        .inferences([crate::inferences::energy_model_artifact_inference_descriptor()])
        .composers(crate::io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec_bare::<EnergyModelSnapshot, EnergyModelMutation>(ENERGY_MODEL_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.energy.model", standard: StandardId("1"), subset: SubsetId("*") })
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring the
/// `🗒️note` exemplar's helper of the same shape.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model",
                    extension: Some("energy"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(crate::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::snapshot::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(crate::mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(crate::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(crate::diff::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(crate::diff::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::snapshot::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "energy.model.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(crate::mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(crate::mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("energy.model.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Declaration

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
            }
        }
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_declaration_tests;
//#endregion 🧪️Tests
