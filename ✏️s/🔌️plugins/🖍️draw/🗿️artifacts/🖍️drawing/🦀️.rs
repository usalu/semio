//! ✏️ Drawing artifact — document schema (the `2d.drawing` document type).

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]


use semio_s_artifact_draw_drawing_io::binary;
use semio_s_artifact_draw_drawing_io::text;
#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
// 🧯️ `clippy::result_large_err` — every `🎮️commands/*` handler returns
// `Result<Emit<DrawingMutation, NoConfigMutation>, Fault>`, the exact signature `ArtifactApp::handle`
// and `app_commands!`'s generated `dispatch` require. `Fault` is a framework-owned error type; boxing it
// here would diverge from the trait it must satisfy, and the lint does not fire on the trait impl itself
// (only on the free functions the taxonomy split creates), so this is a pure artefact of decomposition.
// 🎭️ `machine::statechart!` (the canvas tool of `editor::drawing::commands::canvas_pointer_down`) generates code
// containing `#[cfg(feature = "serde")]` gates meant for `machine`'s OWN crate; macro hygiene splices that cfg check
// into the CALLING crate's feature list instead (a rustc macro-expansion limitation, not a real
// conditional-compilation bug here) — this crate declares no `serde` feature, so rustc flags the value as
// unrecognized. Harmless, but a hard error under `-D warnings` without this crate-wide allow.


//#region 🔖️Domain


//#endregion 🔖️Domain

//#region 🔖️ArtifactKind


/// 🔖️ This artifact's OLD capability-row definition (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE W1b) — kept per debt D1 (`📌️important.md`), not
/// deleted repo-wide until W6. The app's empty CONFIG and real PRESENCE facets now register through
/// the editor declaration's app schema descriptor. Superseded as the schema/io/surface registration
/// channel by `artifact()` (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM) — this
/// function's only remaining reader is the `en`/`de` localized name pair (see `artifact()`'s own
/// `localization: &[]` doc).
pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::ArtifactDefinitionError> {
    use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace, ArtifactLocale, ArtifactLocalization};
    let rows: &[semio_framework_plugin::ArtifactCapabilityRow<'_>] = &[
        ("s.draw.drawing.standard.v1", "standard", "1", &[], None),
        ("s.draw.drawing.standard.v1.profile.any", "profile", "any", &[], None),
        ("s.draw.drawing.schema.artifact", "schema", "s.draw.drawing", &[("schema", "s.draw.drawing")], None),
        ("s.draw.drawing.inference.artifact", "inference", "s.draw.drawing.inference", &[("schema", "s.draw.drawing.inference")], None),
        ("s.draw.drawing.composer.svg", "composer", "s.stdio.svg@1.1/*", &[("dialect", "s.stdio.svg@1.1/*")], None),
        ("s.draw.drawing.composer.pdf", "composer", "s.stdio.pdf@1.4/*", &[("dialect", "s.stdio.pdf@1.4/*")], None),
        ("s.draw.drawing.composer.png", "composer", "s.stdio.png@1.2/*", &[("dialect", "s.stdio.png@1.2/*")], None),
        ("s.draw.drawing.composer.json", "composer", "s.stdio.json@rfc8259/*", &[("dialect", "s.stdio.json@rfc8259/*")], None),
        ("s.draw.drawing.composer.dwg", "composer", "s.stdio.dwg@ac1018/*", &[("dialect", "s.stdio.dwg@ac1018/*")], None),
        ("s.draw.drawing.composer.dxf", "composer", "s.stdio.dxf@r12/*", &[("dialect", "s.stdio.dxf@r12/*")], None),
        ("s.draw.drawing.grammar.document", "grammar", "drawing.document", &[("grammar", "drawing.document")], None),
        ("s.draw.drawing.grammar.op", "grammar", "drawing.op", &[("grammar", "drawing.op")], None),
        ("s.draw.drawing.grammar.diff", "grammar", "drawing.diff", &[("grammar", "drawing.diff")], None),
        ("s.draw.drawing.grammar.pack", "grammar", "drawing.pack", &[("grammar", "drawing.pack")], None),
        ("s.draw.drawing.grammar.spr", "grammar", "drawing.spr", &[("grammar", "drawing.spr")], None),
        ("s.draw.drawing.codec.document.v1", "codec", "drawing.document:drawing", &[("codec", "drawing.document"), ("codec-extension", "16:drawing.document:drawing")], None),
        ("s.draw.drawing.localization.en", "localization", "Drawing", &[], Some(("en", "Drawing"))),
        ("s.draw.drawing.localization.de", "localization", "Zeichnung", &[], Some(("de", "Zeichnung"))),
    ];
    let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.draw.drawing")?);
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

/// 🌳️ This artifact's declaration tree root (design.md §1/§2) — replaces the old `declaration()`
/// (`ArtifactDeclaration::builder(...).schema(...).inferences(...).composers(...).languages(...)
/// .document_codec(...)` chain, deleted outright, no dual channel) as the ONLY registration channel
/// for schema/io/viewer/editor rows. `definition()` (old `ArtifactDefinition`/capability rows,
/// above) is kept per debt D1 — not deleted repo-wide until W6 — and `artifact_kind()` is kept
/// because `🦀️.rs`'s own `.activation(...)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME) still reads `artifact_kind().id`; neither has
/// any caller left in this function.
pub fn artifact<A: DrawingApplication>() -> semio_framework_plugin::app::declarations::ArtifactDeclaration<A> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.draw.drawing").expect("canonical drawing kind"), localization: &[], standards: vec![standards::v1::standard()] }
}

/// 🧩️ App fleet capable of hosting this artifact's editor and viewer.
pub trait DrawingApplication:
    semio_framework_plugin::PluginApp
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_draw_drawing_editor::DrawingPlayApp>>>
    + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_draw_drawing_viewer::DrawingViewer>>>
{
}

impl<A> DrawingApplication for A where
    A: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<semio_s_artifact_draw_drawing_editor::DrawingPlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<semio_s_artifact_draw_drawing_viewer::DrawingViewer>>>
{
}

//#endregion 🔖️ArtifactKind


// ---- Shims: keep pre-migration module paths resolving for external callers ----


//#region 📚️Examples

//#endregion 📚️Examples

pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use semio_s_artifact_draw_drawing_write::serializers::artifacts as export;
    use semio_s_artifact_draw_drawing_read::deserializers::artifacts as import;
    use semio_s_artifact_draw_drawing_mutations::DrawingMutation;
    use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
    use semio_s_artifact_draw_drawing_model::{DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
    use semio_framework_os_kernel::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<DrawingSnapshot, export::svg::v1_1::any::DrawingIntoSvg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::pdf::v1_4::any::DrawingIntoPdf>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::png::v1_2::any::DrawingIntoPng>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::json::v_rfc8259::any::DrawingIntoJson>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::json::v_rfc8259::any::JsonIntoDraw>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::svg::v1_1::any::SvgIntoDraw>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dwg::v_ac1018::any::DrawingIntoDwg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dxf::v_r12::any::DrawingIntoDxf>(DRAWING_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            // 🎯️ `LanguagePair { text: None, binary: None }` for every facet: a documented,
            // deliberate scope-narrowing matching every other subset already on the new tree
            // (stdio binary/txt, sequence) — `NativeCodecs`'s own doc calls this a legal, supported
            // shape. The underlying `ArtifactDsl`/`ArtifactPack`/`OpText`/`OpBinary` codecs these
            // would point at are unchanged, independently implemented (see `📸️snapshot/`,
            // `🧬️mutations/` siblings), and independently tested either way.
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::bare_artifact_codec::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}

//#region 🪢️TaxonomyMounts
#[path = "🤖️generated/📇️registry/🦀️.rs"]
pub mod graph_manifest;
#[path = "."]
pub mod standards {
    #[path = "🏅️standards/🔖️1/🦀️.rs"]
    pub mod v1;
}
//#endregion 🪢️TaxonomyMounts
