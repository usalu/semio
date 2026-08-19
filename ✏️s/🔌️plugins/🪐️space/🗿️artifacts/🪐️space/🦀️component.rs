//! 🪐️ S Space index artifact — document entity (constitutional: general). Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4: the space's own artifact index —
//! one document per hub space (document id `index`), listing every artifact that lives inside that
//! space. Mirrors the sibling `🏠️home` artifact's shape (declaration/definition/dialect), scaled down:
//! no config lane, no stdio import/export composers this wave (lane 2-B / a follow-up ticket owns the
//! real editor/viewer UI and any composer wiring).

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

pub use crate::artifacts::space::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
pub use crate::artifacts::space::standards::v1::subsets::any::schema::diff::SSpaceDiff;
pub use crate::artifacts::space::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};

pub const S_SPACE_INDEX_DOCUMENT_SCHEMA: &str = "s.space";

//#region 🔖️Dialect
/// 🪪️ Lives at the ARTIFACT level (not under `editor`/`viewer`), mirroring `HOME_DIALECT`'s own
/// placement doc — `artifact_kind` matches this subset's `#[artifact_schema(id = "s.space.space")]`,
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Canonical
/// surface id: `s.space.space@1/*#editor` / `s.space.space@1/*#viewer` (contract §1 grammar).
pub const SPACE_INDEX_DIALECT: semio_framework_plugin::app::Dialect =
    semio_framework_plugin::app::Dialect { artifact_kind: "s.space.space", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ OS artifact kind for this document.
pub async fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "space.sspace".into(),
        name: "Space Artifacts".into(),
        source_format: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(),
        component_kind: "space-index".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: S_SPACE_INDEX_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration — mirrors `crate::artifacts::home::declaration()`'s shape. No
/// `.inferences(...)`/`.composers(...)` calls this wave: the index has no bespoke text inference and no
/// stdio import/export composer this ticket needs (C4 only asks for the snapshot + the four mutations +
/// the projection helper).
pub async fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.space")?)
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.space.space")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.space.space")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.composer.native")?, ArtifactCapabilityKind::composer()).descriptor(b"s.space.space@1/*")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.space.space@1/*")?)?)?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.space.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.space:sspace")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.space")?)?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::extension(), "sspace")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Artifacts")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Artifacts")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Artefakte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Artefakte")?)?)
}

/// 🔖️ Assembles s.space's typed runtime declaration.
pub async fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(crate::artifacts::space::standards::v1::subsets::any::schema::sspace_index_schema_descriptor())
        .document_codec::<semio_framework_plugin::EditorApp<crate::editor::space_index::SpaceIndexEditor>>()
        .try_build()
}
//#endregion 🔖️Declaration
