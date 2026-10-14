//! 🩻️ Block 2D artifact — the document entity the ◻️2d app edits (constitutional: general). Edits
//! exactly one `NodeKind`: its identity, rim presentation, and the `HandleKind` templates placed on
//! that rim.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_2d_editor::Block2dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_2d_viewer::Block2dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_2d_editor::Block2dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_2d_viewer::Block2dViewer>>>
{
}

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};

// #region 🔖️Document

// #endregion 🔖️Document

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()` this app used to carry, which called four different global
/// registries directly, and the plugin root's `.setup(crate::register_block_exports)` escape hatch
/// that invoked it. `crate::editor::block2d::config::schema::app_schema_descriptor()` is handed to
/// `ArtifactEditor::app_schema` instead (ticket W1c), not declared here: an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.block.block2d.standard.v1", "standard", "1", &[], None),
        ("s.block.block2d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block.block2d.schema.artifact", "schema", "s.block.block2d", &[("schema", "s.block.block2d")], None),
        ("s.block.block2d.inference.artifact", "inference", "s.block.block2d.inference", &[("schema", "s.block.block2d.inference")], None),
        ("s.block.block2d.composer.native", "composer", "s.block.block2d@1/*", &[("dialect", "s.block.block2d@1/*")], None),
        ("s.block.block2d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block.block2d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block.block2d.composer.format-6", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.block.block2d.grammar.1", "grammar", "block.block2d", &[("grammar", "block.block2d")], None),
        ("s.block.block2d.grammar.2", "grammar", "block.block2d.op", &[("grammar", "block.block2d.op")], None),
        ("s.block.block2d.grammar.3", "grammar", "block.block2d.diff", &[("grammar", "block.block2d.diff")], None),
        ("s.block.block2d.grammar.4", "grammar", "2d.pack", &[("grammar", "2d.pack")], None),
        ("s.block.block2d.grammar.5", "grammar", "2d.spr", &[("grammar", "2d.spr")], None),
        // 🐛️ D2-capability-claim-repairs: `.document_codec::<EditorApp<Block2dPlayApp>>()` derives
        // its extension claim from `<Block2dSnapshot as pack::snapshot::ArtifactDsl>::EXTENSION`
        // (`…/🧬️schema/📸️snapshot/🦀️.rs`), which is `"block2d"`, not `"block"`.
        ("s.block.block2d.codec.document-1", "codec", "block.2d:block2d", &[("codec", "block.2d"), ("codec-extension", "8:block.2d:block2d")], None),
        ("s.block.block2d.localization.en", "localization", "2D Block", &[], Some(("en", "2D Block"))),
        ("s.block.block2d.localization.de", "localization", "2D-Baustein", &[], Some(("de", "2D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block.block2d")?);
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
/// `.artifact(block2d::declaration())` failed assembly. This channel never runs that check; the real
/// data (schema/inference descriptors, editor/viewer, native codec) is read from
/// `standards::v1::subsets::any::subset()` instead. Mirrors `🗒️note`/`🖍️draw`/`🔱️trinity`'s own
/// migration exactly.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block2d").expect("canonical block2d kind"), localization: &[], standards: vec![crate::standards::v1::component::standard::<PA>()] }
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
