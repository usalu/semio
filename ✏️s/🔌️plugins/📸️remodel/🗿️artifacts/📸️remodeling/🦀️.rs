//! 📸️ Remodeling scene document — schema-only photogrammetry/videogrammetry project state (media
//! streams, calibration, ground control points, reconstruction params/results) shared as event-sourced
//! operations. The actual algorithms live in the editor surface's own `✏️editor/⚙️engine/` topic files
//! (`images`/`video`/`camera`/`feature`/`sfm`/`dense`/`mesh`/`motion`/`geo`/`reconstruction`,
//! relocated out of this artifact tree by 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
//! #2553 — an artifact is a schema plus IO, never an engine), none of which this node references:
//! heavier runtime types (`Se3`, `Intrinsics`, `Distortion`, `WatertightReport`, decoded pyramids,
//! match graphs, depth maps, TSDF volumes) are not designed for durable persistence, so every
//! reference to their shape below is a plain-JSON (or `Packed*`) snapshot the app fills in, never the
//! library type itself.

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;



//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called five different global registries directly from
/// a plugin `.setup()` callback. Concrete window config schemas are registered by the editor's exact
/// Model, Frames, and Report window owners. Relocated from `⚙️engine/🦀️.rs` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE reloc-g3): `⚙️engine` was removed from the taxonomy
/// and `declaration()` describes the artifact, not engine behaviour, so its home is the artifact
/// root alongside `artifact_kind()`.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.remodel.remodeling.standard.v1", "standard", "1", &[], None),
        ("s.remodel.remodeling.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.remodel.remodeling.schema.artifact", "schema", "s.remodel.remodeling", &[("schema", "s.remodel.remodeling")], None),
        ("s.remodel.remodeling.inference.artifact", "inference", "s.remodel.remodeling.inference", &[("schema", "s.remodel.remodeling.inference")], None),
        ("s.remodel.remodeling.composer.native", "composer", "s.remodel.remodeling@1/*", &[("dialect", "s.remodel.remodeling@1/*")], None),
        ("s.remodel.remodeling.composer.format-1", "composer", "s.stdio.las@1.0/*", &[("dialect", "s.stdio.las@1.0/*")], None),
        ("s.remodel.remodeling.composer.format-2", "composer", "s.stdio.ply@1.0/*", &[("dialect", "s.stdio.ply@1.0/*")], None),
        ("s.remodel.remodeling.composer.format-3", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.remodel.remodeling.composer.format-4", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.remodel.remodeling.composer.format-5", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.remodel.remodeling.composer.format-6", "composer", "s.stdio.stl@ascii/*", &[("dialect", "s.stdio.stl@ascii/*")], None),
        ("s.remodel.remodeling.composer.format-7", "composer", "s.stdio.gltf@2.0/*", &[("dialect", "s.stdio.gltf@2.0/*")], None),
        ("s.remodel.remodeling.composer.format-8", "composer", "s.stdio.obj@3.0/*", &[("dialect", "s.stdio.obj@3.0/*")], None),
        ("s.remodel.remodeling.grammar.1", "grammar", "remodeling.document", &[("grammar", "remodeling.document")], None),
        ("s.remodel.remodeling.grammar.2", "grammar", "remodeling.op", &[("grammar", "remodeling.op")], None),
        ("s.remodel.remodeling.grammar.3", "grammar", "remodeling.diff", &[("grammar", "remodeling.diff")], None),
        ("s.remodel.remodeling.grammar.4", "grammar", "remodeling.pack", &[("grammar", "remodeling.pack")], None),
        ("s.remodel.remodeling.grammar.5", "grammar", "remodeling.spr", &[("grammar", "remodeling.spr")], None),
        ("s.remodel.remodeling.codec.document-1", "codec", "remodeling.scene:remodeling", &[("codec", "remodeling.scene"), ("codec-extension", "16:remodeling.scene:remodeling")], None),
        ("s.remodel.remodeling.localization.en", "localization", "Remodeling", &[], Some(("en", "Remodeling"))),
        ("s.remodel.remodeling.localization.de", "localization", "Umbau", &[], Some(("de", "Umbau"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.remodel.remodeling")?);
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
/// registration channel), the same cutover `🗒️note`/`🔱️trinity` already made. The five
/// `dsl::LanguageSpec`s `pilot_languages()` built now live beside their own codec in the subset's
/// `🚪️io/🦀️.rs` `io()`, which `commit_artifact_declarations` registers; `.composers(…)`'s single
/// surviving native row has no field on this tree because `io()`'s typed `IoEntry` slice replaced
/// that channel. `localization: &[]` is a documented shortfall (debt D1): the real en/de names
/// ("Remodeling"/"Umbau") still live on `definition()`'s kept `ArtifactCapability` rows.
pub fn artifact<PA>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_remodel_remodeling_editor::editor::remodeling::RemodelingPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_remodel_remodeling_viewer::viewer::remodeling::RemodelingViewer>>>,
{
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.remodel.remodeling").expect("canonical remodeling kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

//#endregion 🔖️Register


//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "."]
pub mod standards {
    #[path = "."]
    pub mod v1 {
        #[path = "🏅️standards/🔖️1/🦀️.rs"]
        mod standard_root;
        pub use standard_root::*;
        #[path = "."]
        pub mod subsets {
            #[path = "."]
            pub mod any {
                #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs"]
                mod subset_root;
                pub use subset_root::*;
                #[path = "."]
                pub mod io {
                    #[path = "."]
                    pub mod sqlite {
                        #[path = "."]
                        pub mod snapshot {
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧬️owned/🦀️.rs"]
                            mod typed_tests;
                            #[cfg(test)]
                            #[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs"]
                            mod tests;
                        }
                    }
                }
            }
        }
    }
}




