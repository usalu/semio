//! 👯️ Block 5D artifact — the document entity the 🖐️5d app edits (constitutional: general). Edits
//! exactly one `PartKind`: its identity, both 2d/3d presentations, its representations, and the
//! `GripKind` templates placed on it in both projections (keep each grip's 2d/3d halves as flat scalar
//! fields — see `s/plugin/puzzle/app/5d/dsl/rs/lib.rs:62` for the known pack table-column bug this
//! dodges).

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_5d_editor::Block5dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_5d_viewer::Block5dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_5d_editor::Block5dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_5d_viewer::Block5dViewer>>>
{
}

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

// #region 🔖️Document

// #endregion 🔖️Document

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block5dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactEditor::app_schema()` returning `crate::editor::block5d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️.rs` entirely.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.block.block5d.standard.v1", "standard", "1", &[], None),
        ("s.block.block5d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block.block5d.schema.artifact", "schema", "s.block.block5d", &[("schema", "s.block.block5d")], None),
        ("s.block.block5d.inference.artifact", "inference", "s.block.block5d.inference", &[("schema", "s.block.block5d.inference")], None),
        ("s.block.block5d.composer.native", "composer", "s.block.block5d@1/*", &[("dialect", "s.block.block5d@1/*")], None),
        ("s.block.block5d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block.block5d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block.block5d.composer.format-6", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.block.block5d.grammar.1", "grammar", "block.block5d", &[("grammar", "block.block5d")], None),
        ("s.block.block5d.grammar.2", "grammar", "block.block5d.op", &[("grammar", "block.block5d.op")], None),
        ("s.block.block5d.grammar.3", "grammar", "block.block5d.diff", &[("grammar", "block.block5d.diff")], None),
        ("s.block.block5d.grammar.4", "grammar", "5d.pack", &[("grammar", "5d.pack")], None),
        ("s.block.block5d.grammar.5", "grammar", "5d.spr", &[("grammar", "5d.spr")], None),
        ("s.block.block5d.codec.document-1", "codec", "block.5d:block5d", &[("codec", "block.5d"), ("codec-extension", "8:block.5d:block5d")], None),
        ("s.block.block5d.localization.en", "localization", "5D Block", &[], Some(("en", "5D Block"))),
        ("s.block.block5d.localization.de", "localization", "5D-Baustein", &[], Some(("de", "5D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block.block5d")?);
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

/// 🔖️ New declaration channel (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME,
/// `descriptor-prep`): replaces `declaration()`/`ArtifactDeclaration::builder(...).try_build()` — the
/// old channel's `require_declared_capability_or_record` runs an exact sorted-claims equality check
/// between `definition()`'s hand-authored capability rows and the runtime registration, which is why
/// `.artifact(block5d::declaration())` failed assembly. This channel never runs that check; the real
/// data (schema/inference descriptors, editor/viewer, native codec) is read from
/// `standards::v1::subsets::any::subset()` instead. Mirrors `🗒️note`/`🖍️draw`/`🔱️trinity`'s own
/// migration exactly.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block5d").expect("canonical block5d kind"), localization: &[], standards: vec![crate::standards::v1::component::standard::<PA>()] }
}

//#endregion 🪪️Declaration

                
        
        

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutations_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🔬️unit/🦀️.rs"]
mod schema_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
mod sqlite_snapshot_tests;
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        pub mod component;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                pub mod component;
            }
        }
    }
}
//#endregion 🪢️Mounts
