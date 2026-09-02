//! 🏠️ S Home launcher artifact — document entity (constitutional: general).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::home::schema::mutations::SHomeMutation;

pub use crate::artifacts::home::schema::diff::SHomeDiff;

pub const S_HOME_DOCUMENT_SCHEMA: &str = "s.home";
pub use crate::artifacts::home::schema::SHomeArtifact;
pub use crate::artifacts::home::snapshot::schema::SHomeSnapshot;

//#region 🔖️Dialect
/// 🪪️ Lives at the ARTIFACT level (not under `editor`/`viewer`) so a viewer file can read it without
/// ever importing through `editor` — `artifact_kind` matches this subset's own
/// `#[artifact_schema(id = "s.space.home")]`, `standard`/`subset` match this file's own
/// `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Canonical surface id: `s.space.home@1/*#editor` /
/// `s.space.home@1/*#viewer` (contract §1 grammar).
pub const HOME_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.space.home", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ OS artifact kind for this document.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "space.shome".into(),
        name: "S Home".into(),
        source_format: S_HOME_DOCUMENT_SCHEMA.into(),
        component_kind: "home".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: S_HOME_DOCUMENT_SCHEMA.into(),
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
/// behaviour) — replaces the old side-effecting `register_artifact_schema()`/
/// `register_artifact_inference()`/`register_io()` trio (each a lone-call wrapper around one global
/// registry) plus the root's old `register_pilot_languages()` 5-language block, folded here into one
/// declarative table. `kind` is `"s.home"`, matching `S_HOME_DOCUMENT_SCHEMA` above — three genuinely
/// distinct namespaces, not two: NOT `space.shome` (the OS-level `ArtifactKindSpec.id`) and NOT
/// `s.space.home` (`HOME_DIALECT.artifact_kind`, the schema-descriptor id, per this subset's own
/// `#[artifact_schema(id = "s.space.home")]`) — see `ArtifactDeclaration::register_all`'s ownership
/// check, which is enforced against the composer table's own dialects. The editor's and studio app's
/// own config/presence schema (`editor::home::config::schema::app_schema_descriptor()`/
/// `engine::space::config::schema::app_schema_descriptor()`) is supplied by `ArtifactEditor::app_schema()`/
/// `ArtifactApp::app_schema()` overrides (ticket W1c) — `ArtifactDeclaration`
/// deliberately has no field for app-scope schema (see that struct's own doc); it is registered by
/// `.document_codec::<A>()` instead, keyed off `A` the same way this declaration is keyed off `kind`.
/// 🧾️ Defines s.home's immutable runtime capability leaves.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.home")?)
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.home.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.space.home")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.space.home")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.inference.artifact")?, ArtifactCapabilityKind::inference())
                .descriptor(b"s.space.home.inference")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.space.home.inference")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.home.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.home@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.home@1/*")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.composer.zip")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.zip@2.0/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.zip@2.0/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.composer.csv")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.csv@rfc4180/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.csv@rfc4180/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.composer.xlsx")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.xlsx@ecma-376/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.xlsx@ecma-376/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.composer.json")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.stdio.json@rfc8259/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.stdio.json@rfc8259/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.home.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.home:shome")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.home")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "shome")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.home.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Home")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Home")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.home.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Startseite")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Startseite")?)?)
}

/// 🔖️ Assembles s.home's typed runtime declaration.
pub async fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::home::schema::home_artifact_schema_descriptor())
        .inferences([crate::artifacts::home::standards::v1::subsets::any::schema::inferences::home_artifact_inference_descriptor()])
        .composers(crate::artifacts::home::standards::v1::subsets::any::io::io_registry::entries())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::home::HomeApp>>()
        .try_build()
}
//#endregion 🔖️Declaration
