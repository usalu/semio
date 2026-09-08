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

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

#[cfg(feature = "component-app-assembly")]
#[path = "../../🫀️core/🦀️.rs"]
pub mod space_core;

pub use crate::standards::v1::subsets::any::schema::diff::SSpaceDiff;
pub use crate::standards::v1::subsets::any::schema::mutations::SSpaceMutation;
pub use crate::standards::v1::subsets::any::schema::snapshot::{SSpaceSnapshot, SpaceArtifactDialect, SpaceArtifactRow};

pub const S_SPACE_INDEX_DOCUMENT_SCHEMA: &str = "s.space";

//#region 🔖️Dialect
/// 🪪️ Lives at the ARTIFACT level (not under `editor`/`viewer`), mirroring `HOME_DIALECT`'s own
/// placement doc — `artifact_kind` matches this subset's `#[artifact_schema(id = "s.space.space")]`,
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location. Canonical
/// surface id: `s.space.space@1/*#editor` / `s.space.space@1/*#viewer` (contract §1 grammar).
pub const SPACE_INDEX_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.space.space", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
/// 🗂️ OS artifact kind for this document.
pub fn artifact_kind() -> ArtifactKindSpec {
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
#[cfg(feature = "component-app-assembly")]
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
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs"]
                            mod component;
                            pub use component::*;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🦀️.rs"]
                            pub mod diff;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/⚙️operations/🦀️.rs"]
                            pub mod operations;
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs"]
                            pub mod snapshot;
                            #[path = "."]
                            pub mod mutations {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs"]
                                mod component;
                                pub use component::*;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/💾️binary/🦀️.rs"]
                                pub mod binary;
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📝️text/🦀️.rs"]
                                pub mod text;
                                #[path = "."]
                                pub mod create_artifact {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/🧪️tests/🗿️appends-artifact-3-4665d4/🦀️.rs"]
                                    mod tests_appends_artifact_3_to_the_index;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🌱create-artifact/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod delete_artifact {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/🧪️tests/🗿️removes-artifact-2eb687/🦀️.rs"]
                                    mod tests_removes_artifact_2_from_the_index;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🗑️delete-artifact/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod rename_artifact {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/🧪️tests/🗿️renames-artifact-1/🦀️.rs"]
                                    mod tests_renames_artifact_1;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🏷️rename-artifact/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                                #[path = "."]
                                pub mod touch_artifact {
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/🦀️.rs"]
                                    mod component;
                                    pub use component::*;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/💾️binary/🦀️.rs"]
                                    pub mod binary;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/🔺️diff/🦀️.rs"]
                                    pub mod diff;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/↩️inverse/🦀️.rs"]
                                    pub mod inverse;
                                    #[cfg(test)]
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/🧪️tests/🗿️stamps-artifact-1-89ad80/🦀️.rs"]
                                    mod tests_stamps_artifact_1_with_a_new_editor;
                                    #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🕒touch-artifact/📝️text/🦀️.rs"]
                                    pub mod text;
                                }
                            }
                        }
                        #[path = "."]
                        pub mod io {
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs"]
                            mod component;
                            pub use component::*;
                        }
                        #[path = "."]
                        pub mod examples {
                            #[path = "."]
                            pub mod demo {
                                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🦀️.rs"]
                                mod component;
                                pub use component::*;
                            }
                        }
                    }
                }
            }
        }

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod space_index {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod config {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎚️config/🦀️.rs"]
            mod component;
            pub use component::*;
        }

        #[path = "."]
        pub mod commands {
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🔗copy-invite-link/🦀️.rs"]
            pub mod copy_invite_link;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🌱create-artifact/🦀️.rs"]
            pub mod create_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️delete-artifact/🦀️.rs"]
            pub mod delete_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📇fold-directory-events/🦀️.rs"]
            pub mod fold_directory_events;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💌invite-member/🦀️.rs"]
            pub mod invite_member;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗿️open-artifact/🦀️.rs"]
            pub mod open_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗃️open-artifact-with/🦀️.rs"]
            pub mod open_artifact_with;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/💓presence-heartbeat/🦀️.rs"]
            pub mod presence_heartbeat;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚪remove-member/🦀️.rs"]
            pub mod remove_member;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🏷️rename-artifact/🦀️.rs"]
            pub mod rename_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❔request-delete-artifact/🦀️.rs"]
            pub mod request_delete_artifact;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/❕request-invite-member/🦀️.rs"]
            pub mod request_invite_member;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/👁️set-visibility/🦀️.rs"]
            pub mod set_visibility;
            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🕒touch-artifact/🦀️.rs"]
            pub mod touch_artifact;
        }

        #[path = "."]
        pub mod panels {
            #[path = "."]
            pub mod members {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/👥️members/🦀️.rs"]
                mod component;
                pub use component::*;
            }
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🪟️windows/🏠️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}

#[cfg(feature = "component-app-assembly")]
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod space_index {
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🦀️.rs"]
        mod component;
        pub use component::*;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🦀️.rs"]
                mod component;
                pub use component::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod main {
                        #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/👁️viewer/🎭️modes/👁️view/🪟️windows/🏠️main/🦀️.rs"]
                        mod component;
                        pub use component::*;
                    }
                }
            }
        }
    }
}
