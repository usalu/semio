//! 🚪️ IO s.dag (1/✳️any) — `io() -> IoDeclaration` (design.md §2/§3): the native codec plus every
//! foreign hop, aggregated from the typed `Serializer<DagSnapshot>`/`Deserializer<DagSnapshot>`
//! leaves under `📥️import/🧩️deserializers`/`📤️export/🧵️serializers`. Replaces the old hand-rolled
//! `ArtifactComposition`/`ComposerEntry` dispatch chain (`derived_composition`/`io_registry`)
//! outright — all io now goes exclusively through the `io_mechanism` registry (design.md rule 3).
//!
//! This root owns four native-codec facets, each relocated here verbatim from `🧬️schema/` (design.md
//! §1 CORRECTION): `📸️snapshot/📝️text` + `📸️snapshot/💾️binary` (the real `ArtifactDsl`/`ArtifactPack`
//! impls for `DagSnapshot`), `🔺️diff/📝️text` + `🔺️diff/💾️binary`, `🧬️mutations/📝️text` +
//! `🧬️mutations/💾️binary` (the real `OpText`/`OpBinary` impls for `DagMutation`), and
//! `💡️inferences/📝️text` + `💡️inferences/💾️binary` (declaration-only — inference values are computed,
//! never authored). `NativeCodecs.{snapshot,diff,mutations,inferences}: LanguagePair { text: None,
//! binary: None }` below leaves their `dsl::LanguageSpec` registration deferred — a real, supported
//! shape per that type's own doc, matching the stdio pilot's and `🎬️sequence`'s identical documented
//! deviation; the underlying codec impls these would point at are unchanged and independently
//! tested either way (see each facet's own `semio_grammar_conformance`/`semio_protocol_conformance`
//! tests).

//#region 🔖️IoDeclaration
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::artifacts::dag::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::artifacts::dag::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::artifacts::dag::{DagMutation, DagSnapshot, DAG_DIALECT, DAG_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<DagSnapshot, export::json::v_rfc8259::any::DagIntoJson>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::json::v_rfc8259::any::JsonIntoDag>(DAG_DIALECT),
                    serializer_entry::<DagSnapshot, export::md::v_commonmark::any::DagIntoMd>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::md::v_commonmark::any::MdIntoDag>(DAG_DIALECT),
                    serializer_entry::<DagSnapshot, export::csv::v_rfc4180::any::DagIntoCsv>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::csv::v_rfc4180::any::CsvIntoDag>(DAG_DIALECT),
                    serializer_entry::<DagSnapshot, export::png::v1_2::any::DagIntoPng>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::png::v1_2::any::PngIntoDag>(DAG_DIALECT),
                    serializer_entry::<DagSnapshot, export::svg::v1_1::any::DagIntoSvg>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::svg::v1_1::any::SvgIntoDag>(DAG_DIALECT),
                    serializer_entry::<DagSnapshot, export::txt::v_utf_8::any::DagIntoTxt>(DAG_DIALECT),
                    deserializer_entry::<DagSnapshot, import::txt::v_utf_8::any::TxtIntoDag>(DAG_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<DagSnapshot, DagMutation>(DAG_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
//#endregion 🔖️IoDeclaration
