//! 🚪️ IO s.sourcing.curate (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec
//! plus every foreign hop, aggregated from the typed `Serializer<CurateSnapshot>`/
//! `Deserializer<CurateSnapshot>` leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`.
//! Replaces the old hand-rolled `ArtifactComposition`/`ComposerEntry` dispatch chain
//! (`derived_composition`/`io_registry`) outright — all io now goes exclusively through the
//! `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `CurateSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `SourcingMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are
//! computed, never authored). Unlike most W4 subsets, this artifact already carries real
//! hand-authored `dsl::LanguageSpec`s (`pilot_languages()`, artifact root) from the OLD
//! `declaration().languages(...)` channel — `NativeCodecs` below wires them in via
//! `crate::artifacts::curate::language_spec` rather than leaving every `LanguagePair` `None`.
//!
//! ⚠️ Fidelity honesty: `json` import/export is a genuine structural `serde_json::Value` bridge
//! (`IoFidelity::Exact`). `zip`/`png`/`stl`/`obj` import/export are PRE-EXISTING non-functional
//! stubs (the old code tried to decode/encode bytes as the wrong artifact's own pack format —
//! confirmed by inspection, not fixed this pass, no domain-correct geometry↔catalog mapping is
//! defined anywhere) — carried over behaviorally unchanged, labeled `IoFidelity::Lossy` for
//! honesty, same treatment as `txt`'s pre-existing "not yet implemented" stub. See
//! `📓️w4-sourcing-report.md` `## openQuestions`.

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::curate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::curate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::curate::{language_spec, CurateSnapshot, SourcingMutation, SOURCING_CURATE_SCHEMA, SOURCING_DIALECT};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<CurateSnapshot, export::zip::v2_0::any::CurateIntoZip>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::zip::v2_0::any::ZipIntoCurate>(SOURCING_DIALECT),
                    serializer_entry::<CurateSnapshot, export::png::v1_2::any::CurateIntoPng>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::png::v1_2::any::PngIntoCurate>(SOURCING_DIALECT),
                    serializer_entry::<CurateSnapshot, export::json::v_rfc8259::any::CurateIntoJson>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::json::v_rfc8259::any::JsonIntoCurate>(SOURCING_DIALECT),
                    serializer_entry::<CurateSnapshot, export::stl::v_ascii::any::CurateIntoStl>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::stl::v_ascii::any::StlIntoCurate>(SOURCING_DIALECT),
                    serializer_entry::<CurateSnapshot, export::obj::v3_0::any::CurateIntoObj>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::obj::v3_0::any::ObjIntoCurate>(SOURCING_DIALECT),
                    serializer_entry::<CurateSnapshot, export::txt::v_utf_8::any::CurateIntoTxt>(SOURCING_DIALECT),
                    deserializer_entry::<CurateSnapshot, import::txt::v_utf_8::any::TxtIntoCurate>(SOURCING_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: language_spec(dsl::LanguageRole::Document), binary: language_spec(dsl::LanguageRole::Pack) },
            diff: LanguagePair { text: language_spec(dsl::LanguageRole::Diff), binary: None },
            mutations: LanguagePair { text: language_spec(dsl::LanguageRole::Ops), binary: language_spec(dsl::LanguageRole::Spr) },
            inferences: None,
            codec: store::ArtifactCodec::of::<CurateSnapshot, SourcingMutation>(SOURCING_CURATE_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
