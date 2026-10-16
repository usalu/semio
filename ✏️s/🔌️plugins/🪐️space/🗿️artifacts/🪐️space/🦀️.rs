//! 🪐️ S Space index artifact — document entity (constitutional: general). Ticket
//! 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C4: the space's own artifact index —
//! one document per hub space (document id `index`), listing every artifact that lives inside that
//! space. Mirrors the sibling `🏠️home` artifact's shape (declaration/definition/dialect), scaled down:
//! no config lane, no stdio import/export composers this wave (lane 2-B / a follow-up ticket owns the
//! real editor/viewer UI and any composer wiring).

#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_value_derive as value_derive;

#[path = "🫀️index/🦀️.rs"]
pub mod index;

pub use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};

pub use semio_s_artifact_space_space_schema::{SPACE_INDEX_DIALECT, S_SPACE_INDEX_DOCUMENT_SCHEMA, artifact_kind, space_index_topic_contribution};

//#region 🔖️Declaration
/// 🔖️ This artifact's declaration — mirrors `semio_s_artifact_space_home::declaration()`'s shape. No
/// `.inferences(...)`/`.composers(...)` calls this wave: the index has no bespoke text inference and no
/// stdio import/export composer this ticket needs (C4 only asks for the snapshot + the four mutations +
/// the projection helper).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    ArtifactDefinition::new(ArtifactIdentity::parse("s.space.space")?)
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.space.space.schema.artifact")?, ArtifactCapabilityKind::schema()).descriptor(b"s.space.space")?.claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::schema(), "s.space.space")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.space.space.composer.native")?, ArtifactCapabilityKind::composer())
                .descriptor(b"s.space.space@1/*")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::dialect(), "s.space.space@1/*")?)?,
        )?
        .capability(
            ArtifactCapability::new(ArtifactIdentity::parse("s.space.space.codec.document")?, ArtifactCapabilityKind::codec())
                .descriptor(b"s.space:sspace")?
                .claim(ArtifactIdentityClaim::new(ArtifactIdentityNamespace::codec(), "s.space")?)?
                .claim(ArtifactIdentityClaim::codec_extension("s.space", "sspace")?)?,
        )?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.space.localization.en")?, ArtifactCapabilityKind::localization()).descriptor(b"Artifacts")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("en")?, "Artifacts")?)?)?
        .capability(ArtifactCapability::new(ArtifactIdentity::parse("s.space.space.localization.de")?, ArtifactCapabilityKind::localization()).descriptor(b"Artefakte")?.localization(ArtifactLocalization::new(ArtifactLocale::parse("de")?, "Artefakte")?)?)
}

/// 🔖️ Assembles s.space's typed runtime declaration.
pub fn declaration() -> Result<semio_framework_plugin::ArtifactDeclaration, semio_framework_plugin::ArtifactDefinitionError> {
    semio_framework_plugin::ArtifactDeclaration::builder(definition()?)
        .schema(standards::v1::subsets::any::schema::sspace_index_schema_descriptor())
        .document_codec::<semio_framework_plugin::EditorApp<editor::space_index::SpaceIndexEditor>>()
        .try_build()
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
                        #[path = "."]
                        pub mod schema {
                            pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::schema::*;
                            
                            pub use semio_s_artifact_space_space_diff::standards::v1::subsets::any::schema::diff;
                            pub use semio_s_artifact_space_space_mutations::standards::v1::subsets::any::schema::operations;
                            pub use semio_s_artifact_space_space_schema::standards::v1::subsets::any::schema::snapshot;
                            pub use semio_s_artifact_space_space_mutations::standards::v1::subsets::any::schema::mutations;
                        }
                        pub use semio_s_artifact_space_space_editor::standards::v1::subsets::any::io;
                        pub use semio_s_artifact_space_space_diff::standards::v1::subsets::any::examples;
                    }
                }
            }
        }

#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod space_index {
pub use semio_s_artifact_space_space_editor::editor::space_index::*;
        
pub use semio_s_artifact_space_space_schema::editor::space_index::presence;

        pub use semio_s_artifact_space_space_diff::editor::space_index::config;

        pub use semio_s_artifact_space_space_editor::editor::space_index::commands;

        pub use semio_s_artifact_space_space_editor::editor::space_index::panels;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_space_space_editor::editor::space_index::modes::edit::*;
                

                pub use semio_s_artifact_space_space_editor::editor::space_index::modes::edit::windows;
            }
        }
    }
}

#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod space_index {
pub use semio_s_artifact_space_space_editor::viewer::space_index::*;
        
pub use semio_s_artifact_space_space_diff::viewer::space_index::presence;
pub use semio_s_artifact_space_space_diff::viewer::space_index::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_space_space_diff::viewer::space_index::modes::view::*;
                

                pub use semio_s_artifact_space_space_diff::viewer::space_index::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_space_space_diff::examples;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod hoisted_snapshot_0;
