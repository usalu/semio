//! 🏙️ Block 3D artifact — the document entity the 🧊️3d app edits (constitutional: general). Edits
//! exactly one `ObjectKind`: its identity, representations (meshes at LOD/tags — the semio_compose_rs
//! `type` app's successor), and the `VortexKind` templates placed on its rim.

#![allow(clippy::result_large_err)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;

pub trait ArtifactApps:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_3d_editor::Block3dPlayApp>>>
    + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_3d_viewer::Block3dViewer>>>
{
}

impl<PA> ArtifactApps for PA where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::EditorApp<semio_s_artifact_block_3d_editor::Block3dPlayApp>>>
        + From<semio_framework_plugin::app::VcsArtifactApp<semio_framework_plugin::app::ViewerApp<semio_s_artifact_block_3d_viewer::Block3dViewer>>>
{
}

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use semio_s_artifact_stdio_semio_kit::schema::snapshot::{SemioKitSnapshot, SemioKitType};
use semio_s_artifact_block_3d_model::{Block3dVortexKindExtra, Block3dVortexKind};

// #region 🔖️Document

//#region 🔖️VortexKindCatalogComposition

/// 🔀️ Inverse of the split above — reassembles one full `Block3dVortexKind` from its two composed
/// halves.
pub fn vortex_kind_from_parts(kit_type: &SemioKitType, extra: &Block3dVortexKindExtra) -> Block3dVortexKind {
    Block3dVortexKind { id: kit_type.id.clone(), name: kit_type.name.clone(), label: extra.label.clone(), color: extra.color.clone(), default_cable_kind: extra.default_cable_kind.clone() }
}

/// 🔀️ Reassembles the full `Vec<Block3dVortexKind>` catalog from its composed-child half and its
/// block3d-owned overflow half, id-joined. A `SemioKitType` with no matching `Block3dVortexKindExtra`
/// (composed-child content the working-scene cache hasn't seen yet — see `vortex_kinds_of`'s doc
/// comment) is silently dropped rather than fabricated with placeholder label/color.
pub fn vortex_kinds_from_catalog_and_extra(catalog: &SemioKitSnapshot, extra: &[Block3dVortexKindExtra]) -> Vec<Block3dVortexKind> {
    let extra_by_id: std::collections::HashMap<&str, &Block3dVortexKindExtra> = extra.iter().map(|e| (e.id.as_str(), e)).collect();
    catalog.types.iter().filter_map(|kit_type| extra_by_id.get(kit_type.id.as_str()).map(|extra| vortex_kind_from_parts(kit_type, extra))).collect()
}

//#endregion 🔖️VortexKindCatalogComposition

//#region 🔖️WindowView

fn default_arrangement() -> String {
    "overlap".into()
}

fn default_spacing() -> f64 {
    8.0
}

//#endregion 🔖️WindowView

// #endregion 🔖️Document

//#region 🪪️Declaration
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from
/// a plugin `.setup()` callback. `Block3dPlayApp`'s CONFIG/PRESENCE schema — an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) — now registers via
/// `ArtifactEditor::app_schema()` returning `crate::editor::block3d::config::schema::app_schema_descriptor()`
/// (ticket W1c), so `.setup()` is gone from `🧱️block/🦀️.rs` entirely.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.block.block3d.standard.v1", "standard", "1", &[], None),
        ("s.block.block3d.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.block.block3d.schema.artifact", "schema", "s.block.block3d", &[("schema", "s.block.block3d")], None),
        ("s.block.block3d.inference.artifact", "inference", "s.block.block3d.inference", &[("schema", "s.block.block3d.inference")], None),
        ("s.block.block3d.composer.native", "composer", "s.block.block3d@1/*", &[("dialect", "s.block.block3d@1/*")], None),
        ("s.block.block3d.composer.format-1", "composer", "s.stdio.zip@2.0/*", &[("dialect", "s.stdio.zip@2.0/*")], None),
        ("s.block.block3d.composer.format-3", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.block.block3d.composer.format-6", "composer", "s.stdio.txt@utf-8/*", &[("dialect", "s.stdio.txt@utf-8/*")], None),
        ("s.block.block3d.grammar.1", "grammar", "block.block3d", &[("grammar", "block.block3d")], None),
        ("s.block.block3d.grammar.2", "grammar", "block.block3d.op", &[("grammar", "block.block3d.op")], None),
        ("s.block.block3d.grammar.3", "grammar", "block.block3d.diff", &[("grammar", "block.block3d.diff")], None),
        ("s.block.block3d.grammar.4", "grammar", "3d.pack", &[("grammar", "3d.pack")], None),
        ("s.block.block3d.grammar.5", "grammar", "3d.spr", &[("grammar", "3d.spr")], None),
        ("s.block.block3d.codec.document-1", "codec", "block.3d:block3d", &[("codec", "block.3d"), ("codec-extension", "8:block.3d:block3d")], None),
        ("s.block.block3d.localization.en", "localization", "3D Block", &[], Some(("en", "3D Block"))),
        ("s.block.block3d.localization.de", "localization", "3D-Baustein", &[], Some(("de", "3D-Baustein"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.block.block3d")?);
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
/// `.artifact(block3d::declaration())` failed assembly. This channel never runs that check; the real
/// data (schema/inference descriptors, editor/viewer, native codec) is read from
/// `standards::v1::subsets::any::subset()` instead. Mirrors `🗒️note`/`🖍️draw`/`🔱️trinity`'s own
/// migration exactly.
pub fn artifact<PA: ArtifactApps>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.block.block3d").expect("canonical block3d kind"), localization: &[], standards: vec![crate::standards::v1::component::standard::<PA>()] }
}

//#endregion 🪪️Declaration

//#region 🪢️Mounts
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🔬️unit/🦀️.rs"]
mod inferences_unit;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs"]
mod mutations_unit;
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
