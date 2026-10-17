//! 🧪️ Root-bound tests of the `stdio.pdf` artifact: the registration hook of its sqlite declaration and the owned structural hashes and record assets.

use crate::*;
use semio_framework_test_fixtures::{fixture_bytes, fixture_path, fixture_text};

pub(crate) fn register_sqlite_test_declaration() {
    static REGISTERED: std::sync::OnceLock<()> = std::sync::OnceLock::new();
    REGISTERED.get_or_init(|| { semio_framework_plugin::PluginBuilder::<_, semio_framework_plugin::app::NoPluginApp>::new("stdio").label("PDF SQLite declaration").version("0.0.1").package_id("semio:stdio").artifact(declaration(definition().unwrap()).unwrap()).try_build().unwrap(); });
}

#[test]
fn sqlite_snapshot_pdf_native_factories_publish_exact_structural_hashes() {
    let manifest: serde_json::Value = serde_json::from_str(ARTIFACT_DEFINITION_SCHEMA).unwrap();
    let hashes = native_codecs().into_iter().map(|factory| (factory.id, (factory.codec)().pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>())).collect::<Vec<_>>();
    for factory in native_codecs() {
        let codec = (factory.codec)();
        let actual = codec.pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
        let declared = manifest["codecs"].as_array().unwrap().iter().find(|entry| entry["native_factory"]["factory_id"] == factory.id).unwrap();
        assert_eq!(declared["native_factory"]["artifact_schema"], codec.schema);
        assert_eq!(declared["native_factory"]["pack_schema_hash"].as_str().unwrap(), actual, "owner structural hashes {hashes:?}");
    }
}

#[test]
fn sqlite_snapshot_pdf_owned_assets_describe_each_exact_record() {
    use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
    let snapshot14 = standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot { schema: "owned-fields".into(), pages: vec![standards::v1_4::subsets::base::schema::snapshot::PageDoc { width: f64::INFINITY, height: f64::from_bits(0x7ff0000000000001), text: "a\"b\nUnicode 🗄️".into() }] };
    let snapshot17 = PdfSnapshot { schema: "owned-fields".into(), declared_version: "1.7-extension".into(), ..Default::default() };
    let cases = [
        (snapshot14.print_dsl(), <standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot as ArtifactPack>::record_spec().unwrap(), fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio"), fixture_text!("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")),
        (snapshot17.print_dsl(), <PdfSnapshot as ArtifactPack>::record_spec().unwrap(), fixture_text!("../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/📖️.grammar.semio"), fixture_text!("../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")),
    ];
    for (text, record, grammar, protocol) in cases {
        let (envelope, body) = store::semio_format::split_text_preamble(&text).unwrap();
        let source = format!("{}\n{body}", envelope.envelope_id());
        assert!(semio_framework_dsl::Recognizer::compile(&semio_framework_dsl::parse_grammar(grammar).unwrap(), &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments").recognize(&source).unwrap(), "{source}");
        let protocol = semio_framework_dsl::parse_protocol(protocol).unwrap();
        let fields = protocol.blocks.iter().find_map(|block| match block { semio_framework_dsl::Block::Record { name, fields, .. } if name == "snapshot" => Some(fields), _ => None }).unwrap();
        assert_eq!(fields.iter().map(|field| field.name.as_str()).collect::<Vec<_>>(), record.fields.iter().map(|field| field.key.as_str()).collect::<Vec<_>>());
    }
    let wrong = store::semio_format::SemioEnvelope::from_envelope_id("stdio.pdf", store::semio_format::Component::Dsl, 1).unwrap();
    let text = snapshot17.print_dsl(); let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
    assert!(<PdfSnapshot as ArtifactDsl>::parse_dsl(&store::semio_format::wrap_text(&wrong, body)).is_err());
}

/// 📖️ Proves PDF 1.7 logical carriers, independent content oracles and exact protocol custody.
#[test]
fn owned_fixture_publication_reports_canonical_logical_carriers() {
    use store::{pack::snapshot::ArtifactDsl, pack::snapshot::ArtifactPack};
    let snapshot = crate::standards::v1_7::subsets::base::io::export::text_document(&[(300.0, 400.0, "Native PDF 1.7 receipt")]);
    let text = snapshot.print_dsl();
    let binary = snapshot.encode_pack_with(&Default::default()).unwrap();
    assert_eq!(PdfSnapshot::parse_dsl(&text).unwrap(), snapshot);
    assert_eq!(PdfSnapshot::decode_pack_with(&binary, &Default::default()).unwrap(), snapshot);
    let factories = crate::native_codecs();
    let selected = factories.iter().filter(|factory| factory.id == "stdio.native.pdf17.v1").collect::<Vec<_>>();
    assert_eq!(selected.len(), 1);
    let factory = selected[0];
    let codec = (factory.codec)();
    let kind = (factory.kind)();
    let compiled = fixture_bytes!("../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio");
    let current = std::fs::read(fixture_path!("../../🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
    assert_eq!(current.as_slice(), compiled, "the live native receipt requires the current compiled protocol");
    let digest = semio_framework_hash::Sha256::digest(compiled);
    assert_ne!(codec.pack_schema_hash, [0; 32]);
    assert_eq!(kind.id, crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID);
    assert_eq!(codec.schema, crate::standards::v1_7::subsets::base::schema::snapshot::STDIO_PDF17_DOCUMENT_SCHEMA);
    assert_eq!(codec.extension, "pdf");
    let hex = |bytes: &[u8]| bytes.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    let receipt = serde_json::json!({"schemaVersion":1,"artifactKind":kind.id,"artifactSchema":codec.schema,"factoryId":factory.id,"extension":codec.extension,"packSchemaHash":hex(&codec.pack_schema_hash),"protocolSourceSha256":hex(&digest)});
    eprintln!("native-codec-publication={receipt}");
}
