//! 📝️ Note artifact — the document entity this plugin's app edits: an infinite-canvas block tree
//! (text/image/table/math/ink/group blocks).

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_framework_value_derive::{FromValue};

#[cfg(test)]
use std::collections::BTreeMap;

//#region 🔖️Register
/// 🔖️ This artifact's OLD-channel definition (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1).
/// KEPT unread by the new declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM,
/// debt D1 — deleted repo-wide only once every plugin has migrated, not this pass): the real en/de
/// localized names (`"Note"`/`"Notiz"`) still live only on these `ArtifactCapability` rows, and
/// `NotePlayApp::app_schema()` owns the app CONFIG/PRESENCE schema, an app-scope concern neither the
/// old nor the new declaration type has a field for. The `io_registry::entries()`/`NoteComposer`
/// machinery this comment block's own `"composer"` rows once cross-checked against is deleted
/// (`🚪️io/🦀️.rs`'s `io()` replaces it); the capability rows themselves are inert now, kept
/// only because nothing on this pass's boundary reads or removes `definition()`'s callers.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.note.note.standard.v1", "standard", "1", &[], None),
        ("s.note.note.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.note.note.schema.artifact", "schema", "s.note.note", &[("schema", "s.note.note")], None),
        ("s.note.note.inference.artifact", "inference", "s.note.note.inference", &[("schema", "s.note.note.inference")], None),
        ("s.note.note.composer.note", "composer", "s.note.note@1/*", &[("dialect", "s.note.note@1/*")], None),
        ("s.note.note.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.note.note.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.note.note.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.note.note.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.note.note.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.note.note.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.note.note.grammar.document", "grammar", "note.document", &[("grammar", "note.document")], None),
        ("s.note.note.grammar.op", "grammar", "note.op", &[("grammar", "note.op")], None),
        ("s.note.note.grammar.diff", "grammar", "note.diff", &[("grammar", "note.diff")], None),
        ("s.note.note.grammar.pack", "grammar", "note.pack", &[("grammar", "note.pack")], None),
        ("s.note.note.grammar.spr", "grammar", "note.spr", &[("grammar", "note.spr")], None),
        ("s.note.note.codec.document.v1", "codec", "note.document:note", &[("codec", "note.document"), ("codec-extension", "13:note.document:note")], None),
        ("s.note.note.localization.en", "localization", "Note", &[], Some(("en", "Note"))),
        ("s.note.note.localization.de", "localization", "Notiz", &[], Some(("de", "Notiz"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.note.note")?);
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

/// 🗿️ New declaration tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM design.md §2)
/// — replaces the OLD `declaration()`/`pilot_languages()` pair outright (atomic cutover, no dual
/// registration channel). `localization: &[]` is a documented shortfall, not an oversight: the real
/// en/de localized names (`"Note"`/`"Notiz"`) still live on `definition()`'s kept
/// `ArtifactCapability` rows (debt D1) — wiring them into this field is real follow-up work, not
/// required for this pass (`📓️recipe-subset.md` §4c, matches the stdio pilot's identical deviation).
pub fn artifact<PA>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<editor::note::NotePlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<viewer::note::NoteViewer>>>,
{
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.note.note").expect("canonical note kind"), localization: &[], standards: vec![standards::v1::standard()] }
}
//#endregion 🔖️Register

pub use semio_s_artifact_note_note_v1::{artifact_kind};

pub use semio_s_artifact_note_note_v1::{NOTE_DIALECT};

pub use semio_s_artifact_note_note_schema::{NOTE_DOCUMENT_SCHEMA, NoteBlockNode, NoteCamera, NoteImageAsset, NoteTableCell, NoteTextChild, NoteTextParagraph, NoteTextRun, default_true, default_zoom, note_block_text, note_text_child_handle, note_text_child_record, paragraphs_from_text_snapshot, text_snapshot_from_paragraphs};
pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff::NoteDiff;
pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations::NoteMutation;
pub use semio_s_artifact_note_note_schema::NoteSnapshot;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

pub use semio_s_artifact_note_note_diff::{INK_CANVAS_DOCUMENT_SCHEMA, entry, ink_wire_blocks_from_note, is_kind, note_block_value_from_ink_wire, note_canvas_document_json};

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
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::*;

                    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::snapshot;
                    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::any::schema::inferences;
                    pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::schema::diff;
                    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::schema::mutations;
                }
                #[path = "."]
                pub mod io {
                    pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::*;
pub mod sqlite {
    pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::io::sqlite::*;
    pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::sqlite::snapshot;
}

                    #[path = "."]
                    pub mod import {
                        #[path = "."]
                        pub mod deserializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_note_note_mutations::standards::v1::subsets::any::io::import::deserializers::artifacts::svg;
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::pdf;
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::png;
                                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::io::import::deserializers::artifacts::json;
                                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::import::deserializers::artifacts::dwg;
                                pub use semio_s_artifact_note_note_mut_family_4::standards::v1::subsets::any::io::import::deserializers::artifacts::dxf;
                            }
                        }
                    }
                    #[path = "."]
                    pub mod export {
                        #[path = "."]
                        pub mod serializers {
                            #[path = "."]
                            pub mod artifacts {
                                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::export::serializers::artifacts::svg;
                                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::export::serializers::artifacts::pdf;
                                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::export::serializers::artifacts::png;
                                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::io::export::serializers::artifacts::json;
                                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::io::export::serializers::artifacts::dwg;
                                pub use semio_s_artifact_note_note_diff::standards::v1::subsets::any::io::export::serializers::artifacts::dxf;
                            }
                        }
                    }
                }
                pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::any::examples;
                pub use semio_s_artifact_note_note_v1::standards::v1::subsets::any::*;

            }
            #[path = "."]
            pub mod ink {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_pencil_width {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_pencil_width::inverse;

                        }
                        #[path = "."]
                        pub mod change_eraser_radius {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_eraser_radius::inverse;

                        }
                        #[path = "."]
                        pub mod change_block_ink_width {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::change_block_ink_width::inverse;

                        }
                        #[path = "."]
                        pub mod edit_block_ink_stroke {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::ink::schema::mutations::edit_block_ink_stroke::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod text {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_text {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::text::schema::mutations::edit_block_text::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod math {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod edit_block_math {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::math::schema::mutations::edit_block_math::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod table {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod insert_table_row {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_row::inverse;

                        }
                        #[path = "."]
                        pub mod remove_table_row {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_row::inverse;

                        }
                        #[path = "."]
                        pub mod insert_table_column {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::insert_table_column::inverse;

                        }
                        #[path = "."]
                        pub mod remove_table_column {
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::*;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::diff;
                            pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::table::schema::mutations::remove_table_column::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod asset {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_asset {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::create_asset::inverse;

                        }
                        #[path = "."]
                        pub mod replace_asset_payload {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::replace_asset_payload::inverse;

                        }
                        #[path = "."]
                        pub mod delete_asset {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::asset::schema::mutations::delete_asset::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod block {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod create_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::create_block::inverse;

                        }
                        #[path = "."]
                        pub mod delete_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::delete_block::inverse;

                        }
                        #[path = "."]
                        pub mod delete_blocks {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::delete_blocks::inverse;

                        }
                        #[path = "."]
                        pub mod duplicate_block {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_block::inverse;

                        }
                        #[path = "."]
                        pub mod duplicate_blocks {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::block::schema::mutations::duplicate_blocks::inverse;

                        }
                        #[path = "."]
                        pub mod move_block_to_container {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::move_block_to_container::inverse;

                        }
                        #[path = "."]
                        pub mod drag_blocks {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::drag_blocks::inverse;

                        }
                        #[path = "."]
                        pub mod rename_block {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::rename_block::inverse;

                        }
                        #[path = "."]
                        pub mod change_block_visible {
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::*;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::diff;
                            pub use semio_s_artifact_note_note_mut_family_2::standards::v1::subsets::block::schema::mutations::change_block_visible::inverse;

                        }
                        #[path = "."]
                        pub mod change_block_locked {
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::*;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::diff;
                            pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::schema::mutations::change_block_locked::inverse;

                        }
                        #[path = "."]
                        pub mod move_block {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::move_block::inverse;

                        }
                        #[path = "."]
                        pub mod resize_block {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::resize_block::inverse;

                        }
                        #[path = "."]
                        pub mod change_block_font_size {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::block::schema::mutations::change_block_font_size::inverse;

                        }
                    }
                }
            
pub mod io {
    pub use semio_s_artifact_note_note_insert_table_row::standards::v1::subsets::block::io::*;
    pub use semio_s_artifact_note_note_schema::standards::v1::subsets::block::io::binary;
    pub mod text {
        pub use semio_s_artifact_note_note_mut_family::standards::v1::subsets::block::io::text::*;
        pub use semio_s_artifact_note_note_diff::standards::v1::subsets::block::io::text::mutations;
    }
}
}
            #[path = "."]
            pub mod canvas {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod change_grid_visible {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_visible::inverse;

                        }
                        #[path = "."]
                        pub mod change_grid_spacing {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_spacing::inverse;

                        }
                        #[path = "."]
                        pub mod change_grid_subdivisions {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_subdivisions::inverse;

                        }
                        #[path = "."]
                        pub mod change_grid_opacity {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_grid_opacity::inverse;

                        }
                        #[path = "."]
                        pub mod change_snap_enabled {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_enabled::inverse;

                        }
                        #[path = "."]
                        pub mod change_snap_grid_spacing {
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::*;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::diff;
                            pub use semio_s_artifact_note_note_mut_family_3::standards::v1::subsets::canvas::schema::mutations::change_snap_grid_spacing::inverse;

                        }
                    }
                }
            }
            #[path = "."]
            pub mod document {
                #[path = "."]
                pub mod schema {
                    #[path = "."]
                    pub mod mutations {
                        #[path = "."]
                        pub mod rename_note {
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::*;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::diff;
                            pub use semio_s_artifact_note_note_delete_blocks::standards::v1::subsets::document::schema::mutations::rename_note::inverse;

                        }
                    }
                }
            }
        }
        pub use semio_s_artifact_note_note_v1::standards::v1::*;

    }
}

pub use crate::standards::v1::subsets::any::schema;

pub mod op {
    pub use crate::standards::v1::subsets::any::schema::mutations::NoteMutation;
}

pub use semio_s_artifact_note_note_diff::diff;
pub mod mutations {
    pub use crate::standards::v1::subsets::any::schema::mutations::*;
}
pub use semio_s_artifact_note_note_mutations::snapshot;
pub mod examples {
    pub use super::standards::v1::subsets::any::examples::*;
}

/// ✏️ The mutation-capable surface (contract §2.1/§2.4) — every leaf `#[path]`-mounted from the real
/// subset dir under `🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/…`.
#[path = "."]
pub mod editor {
    #[path = "."]
    pub mod note {
        pub use semio_s_artifact_note_note_v1::editor::note::*;
pub use semio_s_artifact_note_note_diff::editor::note::demo_session;

        pub use semio_s_artifact_note_note_diff::editor::note::config;

        pub use semio_s_artifact_note_note_diff::editor::note::support;

        pub use semio_s_artifact_note_note_kinds_catalog::editor::note::dispatch_ctx;

        pub use semio_s_artifact_note_note_v1::editor::note::command_enum;

        pub use semio_s_artifact_note_note_v1::editor::note::retained;

        #[path = "."]
        pub mod window {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::*;

            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::window::schema;
        }

        pub mod presence {
    pub use semio_s_artifact_note_note_diff::editor::note::presence::*;
    pub mod io {
        pub use semio_s_artifact_note_note_mut_family::editor::note::presence::io::*;
        pub mod binary {
            pub use semio_s_artifact_note_note_insert_table_row::editor::note::presence::io::binary::*;
            pub use semio_s_artifact_note_note_diff::editor::note::presence::io::binary::mutations;
        }
        pub mod text {
            pub use semio_s_artifact_note_note_insert_table_row::editor::note::presence::io::text::*;
            pub use semio_s_artifact_note_note_diff::editor::note::presence::io::text::mutations;
        }
    }
}

        pub use semio_s_artifact_note_note_diff::editor::note::terminology;

        #[path = "."]
        pub mod commands {
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::add_block;
            pub use semio_s_artifact_note_note_kinds_catalog::editor::note::commands::delete_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::delete_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::duplicate_selection;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_input;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::engagement_submit;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::ink_apply_events;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::move_block;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::navigator_engagement_input;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_down;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_down_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_left;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_left_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_right;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_right_fast;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_up;
            pub use semio_s_artifact_note_note_nudge_selection::editor::note::commands::nudge_selection_up_fast;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::patch_blocks;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::save_download;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_active_example;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_camera_zoom;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_eraser_radius;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::load_document_json;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_opacity;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_spacing;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_subdivisions;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_grid_visible;
            pub use semio_s_artifact_note_note_tests_hides_the_grid::editor::note::commands::set_pencil_width;
            pub use semio_s_artifact_note_note_mut_family_4::editor::note::commands::set_snap_enabled;
            pub use semio_s_artifact_note_note_v1::editor::note::commands::set_snap_grid_spacing;
        }

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod edit {
                pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::*;

                #[path = "."]
                pub mod windows {
                    #[path = "."]
                    pub mod composite {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::*;

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::camera;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::eraser_point;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::eraser_stroke;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::grid;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::pencil;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::composite::options::snap;
                        }
                    }

                    #[path = "."]
                    pub mod navigator {
                        pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::navigator::*;

                        #[path = "."]
                        pub mod options {
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::navigator::options::grid_visible;
                            pub use semio_s_artifact_note_note_delete_blocks::editor::note::modes::edit::windows::navigator::options::zoom;
                        }
                    }
                }
            }
        }

        #[path = "."]
        pub mod panels {
            pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::catalogue;
            pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::document;
            pub use semio_s_artifact_note_note_delete_blocks::editor::note::panels::inspection;
        }
    }
}

/// 👁️ The read-only surface (contract §2.2/§2.6) — a genuinely independent module tree from
/// `editor` above, never `#[path]`-mounting anything under `✏️editor/`: that would let
/// `policyViewerPurityBreaches`' substring check on the sibling editor module catch a real
/// dependency, but the deeper reason is architectural — the viewer must stay constructible without
/// ever touching the editor's mutation-capable types.
#[path = "."]
pub mod viewer {
    #[path = "."]
    pub mod note {
        pub use semio_s_artifact_note_note_v1::viewer::note::*;

        pub use semio_s_artifact_note_note_diff::viewer::note::presence;
        pub use semio_s_artifact_note_note_diff::viewer::note::config;

        #[path = "."]
        pub mod modes {
            #[path = "."]
            pub mod view {
                pub use semio_s_artifact_note_note_mut_family::viewer::note::modes::view::*;

                pub use semio_s_artifact_note_note_diff::viewer::note::modes::view::windows;
            }
        }
    }
}

pub use semio_s_artifact_note_note_v1::{note_child_restore_projection};

#[cfg(all(test,not(target_arch="wasm32")))]
#[path="🧪️tests/⚡️quick/🦀️.rs"]
mod quick;

pub use semio_s_artifact_note_note_mutations::host;

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_0;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🧪️tests/🔬️unit/🦀️.rs"]
mod hoisted_tests_1;
