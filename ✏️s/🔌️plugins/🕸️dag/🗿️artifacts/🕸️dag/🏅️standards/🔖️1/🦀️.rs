//! 🏅️ DAG standard root — `s.dag.dag@1`. Mounts the one subset (`any`) and declares this
//! standard's media (design.md §1/§2).

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

/// 🏅️ `mimes` is a documented synthesis, not a literal carry-over — no real MIME registration
/// exists anywhere in the pre-migration code for this artifact (the old `definition()`'s capability
/// rows only ever claimed a codec id `dag.dag` and an extension `dag`, never a mime type; same
/// finding as the `🎬️sequence` pilot). `extensions` IS the real, carried-over value.
pub fn standard<A: crate::DagApplication>() -> StandardDeclaration<A> {
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.dag+json"], extensions: &["dag"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems10
pub mod artifact_items_10 {
//#region 🔖️Register
/// 🔖️ This artifact's declaration (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) — replaces
/// the old side-effecting `register()`, which called four different global registries directly from a
/// plugin `.setup()` callback. `crate::editor::dag::config::schema::register_app_schema()` is the one
/// exception, still called from `🕸️dag/🦀️.rs`'s own `.setup()`: it registers the `DagPlayApp`
/// CONFIG/PRESENCE schema, an app-scope concern `ArtifactDeclaration` deliberately has no field for
/// (see that struct's own doc) — `register_app_schema_descriptor` is not in §6's artifact-scoped
/// function set. Relocated from `⚙️engine` (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE
/// reloc-g2): `declaration()` describes the artifact (kind, schema, io ports, ownership), which is not
/// engine behaviour.
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};

    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.dag.dag.standard.v1", "standard", "1", &[], None),
        ("s.dag.dag.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.dag.dag.schema.artifact", "schema", "s.dag.dag", &[("schema", "s.dag.dag")], None),
        ("s.dag.dag.inference.artifact", "inference", "s.dag.dag.inference", &[("schema", "s.dag.dag.inference")], None),
        ("s.dag.dag.composer.native", "composer", "s.dag.dag@1/*", &[("dialect", "s.dag.dag@1/*")], None),
        ("s.dag.dag.composer.format-2", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.dag.dag.grammar.1", "grammar", "dag.document", &[("grammar", "dag.document")], None),
        ("s.dag.dag.grammar.2", "grammar", "dag.op", &[("grammar", "dag.op")], None),
        ("s.dag.dag.grammar.3", "grammar", "dag.diff", &[("grammar", "dag.diff")], None),
        ("s.dag.dag.grammar.4", "grammar", "dag.pack", &[("grammar", "dag.pack")], None),
        ("s.dag.dag.grammar.5", "grammar", "dag.spr", &[("grammar", "dag.spr")], None),
        ("s.dag.dag.codec.document-1", "codec", "dag.dag:dag", &[("codec", "dag.dag"), ("codec-extension", "7:dag.dag:dag")], None),
        ("s.dag.dag.localization.en", "localization", "DAG", &[], Some(("en", "DAG"))),
        ("s.dag.dag.localization.de", "localization", "Gerichteter azyklischer Graph", &[], Some(("de", "Gerichteter azyklischer Graph"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.dag.dag")?);
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

/// 🔖️ New declaration tree root (design.md §1/§2) — replaces `declaration()` (the old
/// `ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec::<...>()` chain) outright. No dual registration: the plugin root's
/// `.declare_artifact(artifact())` call is the ONLY registration channel for this artifact.
/// `definition()` (old `ArtifactDefinition`/capability rows) is KEPT per debt D1 — not deleted
/// repo-wide until W6 — but has zero callers left from this file.
pub fn artifact<A: DagApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.dag.dag").expect("canonical dag kind"), localization: &[], standards: vec![crate::standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait DagApplication:
    semio_framework_plugin::PluginApp + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::dag::DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>> + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::dag::DagViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

impl<A> DagApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::dag::DagPlayApp>, semio_s_artifact_stdio_semio::SemioMembers>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::dag::DagViewer>, semio_s_artifact_stdio_semio::SemioMembers>>
{
}

//#endregion 🔖️Register
}
//#endregion 🧩️ArtifactItems10
