//! 🖼️ Raster artifact — document entities (constitutional: general).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

//#region 📚️Examples
//#endregion 📚️Examples

#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🦀️.rs"]
mod app_raster_demo_session_tests;
#[cfg(test)]
#[path = "🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🎬️demo/🧪️tests/🧩️example/🦀️.rs"]
mod art_raster_demo_tests;
extern crate semio_framework_schema as schema;

//#region 🔖️Constants
//#endregion 🔖️Constants

//#region 🗂️OwnedMap


//#endregion 🗂️OwnedMap

//#region 🔖️Types


//#endregion 🔖️Types

//#region 🧩️Composition
/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (design map §4: "raster→C:image layers
/// R:drawing"): a pixel layer's real image bytes used to live INLINE on `RasterSnapshot.assets:
/// BTreeMap<String, RasterImageAsset>` (a duplicated bytes-blob type, never `s.stdio.semio/v1/image`
/// itself). `assets` is now `BTreeMap<String, RasterAssetChild>` — one composed `s.stdio.semio.image`
/// CHILD per asset id, content-addressed, never embedded bytes. `image_key: Option<String>` on
/// `RasterLayerNode::Pixel` is UNCHANGED — it still addresses into this same id-keyed collection, only
/// the map's VALUE type changed from bytes to a handle. `drawing` (`SemioDrawingSnapshot`, used by
/// `🚪️io`'s SVG export/DWG import bridge) was checked and found to be ALREADY a pure, non-persisted IO
/// conversion — raster never owns/duplicates a `drawing` field, it only ever calls stdio's real
/// `SemioDrawingSnapshot`/`DrawNode` types directly at conversion time (`drawing_snapshot_from_raster`/
/// `page_drawing_from_world`, `🚪️io/🦀️.rs`). That already satisfies "consumes/reads drawing
/// content but doesn't own it" — no `ArtifactLink` was needed because there was no persisted/duplicated
/// drawing field to convert.


//#endregion 🧩️Composition

//#region 🔖️Operations


//#endregion 🔖️Operations


/// 📸️ Persisted raster snapshot — defined in `📸️snapshot/🧬️schema`, re-exported here.


//#region 🔖️Dialect
//#endregion 🔖️Dialect

//#region 🔖️ArtifactKind
//#endregion 🔖️ArtifactKind

//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) —
/// replaces the old side-effecting `register()`, which called four different global registries
/// directly from a plugin `.setup()` callback. `crate::editor::raster::config::schema::
/// register_app_schema()` is the one exception, still called from `🖨️raster/🦀️.rs`'s own
/// `.setup()`: it registers the `RasterPlayApp` CONFIG schema, an app-scope concern
/// `ArtifactDeclaration` deliberately has no field for (see that struct's own doc) —
/// `register_app_schema_descriptor` is not in the W1 census's artifact-scoped function set.
/// Relocated from `⚙️engine/🦀️.rs` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g3): `⚙️engine` was removed from the taxonomy and `declaration()` describes the artifact,
/// not engine behaviour, so its home is the artifact root alongside `artifact_kind()`. The
/// `io_registry::entries()` call below is now re-qualified onto `subsets::any::io::io_registry`
/// (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): the whole `⚙️engine` file this
/// function moved out of has since been dissolved into `🧬️schema/`/`🚪️io/`/the app, per rule 5.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.raster.raster.standard.v1", "standard", "1", &[], None),
        ("s.raster.raster.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.raster.raster.schema.artifact", "schema", "s.raster.raster", &[("schema", "s.raster.raster")], None),
        ("s.raster.raster.inference.artifact", "inference", "s.raster.raster.inference", &[("schema", "s.raster.raster.inference")], None),
        ("s.raster.raster.composer.native", "composer", "s.raster.raster@1/*", &[("dialect", "s.raster.raster@1/*")], None),
        ("s.raster.raster.composer.format-1", "composer", "s.stdio.gif@87a/*", &[("dialect", "s.stdio.gif@87a/*")], None),
        ("s.raster.raster.composer.format-2", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.raster.raster.composer.format-4", "composer", "s.stdio.jpg@jfif-1.01/*", &[("dialect", "s.stdio.jpg@jfif-1.01/*")], None),
        ("s.raster.raster.composer.format-5", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.raster.raster.composer.format-6", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.raster.raster.composer.format-8", "composer", "s.stdio.bmp@v3/*", &[("dialect", "s.stdio.bmp@v3/*")], None),
        ("s.raster.raster.composer.format-9", "composer", "s.stdio.tiff@6.0/*", &[("dialect", "s.stdio.tiff@6.0/*")], None),
        ("s.raster.raster.grammar.1", "grammar", "raster.document", &[("grammar", "raster.document")], None),
        ("s.raster.raster.grammar.2", "grammar", "raster.op", &[("grammar", "raster.op")], None),
        ("s.raster.raster.grammar.3", "grammar", "raster.document.diff", &[("grammar", "raster.document.diff")], None),
        ("s.raster.raster.grammar.4", "grammar", "raster.pack", &[("grammar", "raster.pack")], None),
        ("s.raster.raster.grammar.5", "grammar", "raster.spr", &[("grammar", "raster.spr")], None),
        ("s.raster.raster.codec.document-1", "codec", "raster.document:raster", &[("codec", "raster.document"), ("codec-extension", "15:raster.document:raster")], None),
        ("s.raster.raster.localization.en", "localization", "Raster", &[], Some(("en", "Raster"))),
        ("s.raster.raster.localization.de", "localization", "Raster", &[], Some(("de", "Raster"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.raster.raster")?);
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
        .schema(semio_s_artifact_raster_raster_model::raster_artifact_schema_descriptor())
        .inferences([semio_s_artifact_raster_raster_model::inferences::raster_artifact_inference_descriptor()])
        .composers(semio_s_artifact_raster_raster_io::io_registry::entries())
        .languages(pilot_languages())
        .document_codec::<semio_framework_plugin::app::EditorApp<semio_s_artifact_raster_raster_editor::RasterPlayApp>>()
        .try_build()
}

/// 📌️ Handcrafted facet grammars (text) and protocols (binary) for in-process execution — built once
/// and leaked to a `&'static` slice since `dsl::passthrough_hooks` isn't `const fn`, mirroring
/// 🗒️note's own `pilot_languages()` convention.
fn pilot_languages() -> &'static [semio_framework_dsl::LanguageSpec] {
    static LANGUAGES: std::sync::OnceLock<Vec<semio_framework_dsl::LanguageSpec>> = std::sync::OnceLock::new();
    LANGUAGES
        .get_or_init(|| {
            vec![
                semio_framework_dsl::LanguageSpec {
                    id: "raster.document",
                    extension: Some("raster"),
                    role: semio_framework_dsl::LanguageRole::Document,
                    grammar: Some(semio_s_artifact_raster_raster_model::io::text::snapshot::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_raster_raster_model::io::text::snapshot::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_raster_raster_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_raster_raster_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("raster.document"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "raster.op",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Ops,
                    grammar: Some(semio_s_artifact_raster_raster_mutations::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_raster_raster_mutations::text::COMPONENT_GRAMMAR_PATH),
                    protocol: Some(semio_s_artifact_raster_raster_mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_raster_raster_mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("raster.op"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "raster.document.diff",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Diff,
                    grammar: Some(semio_s_artifact_raster_raster_diff::io::text::COMPONENT_GRAMMAR_SEMIO),
                    grammar_path: Some(semio_s_artifact_raster_raster_diff::io::text::COMPONENT_GRAMMAR_PATH),
                    protocol: None,
                    protocol_path: None,
                    hooks: semio_framework_dsl::passthrough_hooks("raster.document.diff"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "raster.pack",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Pack,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_raster_raster_model::io::binary::snapshot::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_raster_raster_model::io::binary::snapshot::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("raster.pack"),
                },
                semio_framework_dsl::LanguageSpec {
                    id: "raster.spr",
                    extension: None,
                    role: semio_framework_dsl::LanguageRole::Spr,
                    grammar: None,
                    grammar_path: None,
                    protocol: Some(semio_s_artifact_raster_raster_mutations::binary::COMPONENT_PROTOCOL_SEMIO),
                    protocol_path: Some(semio_s_artifact_raster_raster_mutations::binary::COMPONENT_PROTOCOL_PATH),
                    hooks: semio_framework_dsl::passthrough_hooks("raster.spr"),
                },
            ]
        })
        .as_slice()
}
//#endregion 🔖️Register

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests


// ---- Shims: keep pre-migration module paths resolving for external callers ----


