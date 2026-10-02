use std::collections::BTreeSet;

#[test]
fn native_catalog_dependency_is_exactly_its_compiled_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json"))).unwrap();
    let expected = semio_hub_stdio::catalog::native_artifact_catalog_dependency().unwrap();
    assert_eq!(serde_json::to_value(&expected).unwrap(), fixture["nativeCases"][0]["dependencies"][0]);
    let rows = fixture["nativeCases"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    for row in rows {
        let expected = row["accepted"].as_bool().unwrap();
        let parsed = serde_json::from_value::<Vec<semio_framework::PluginDependency>>(row["dependencies"].clone());
        let accepted = parsed.as_ref().is_ok_and(|dependencies| semio_hub_stdio::catalog::validate_native_artifact_catalog_dependency(dependencies).is_ok());
        assert_eq!(accepted, expected, "{}", row["id"]);
        let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&row["dependencies"].clone().into());
        let decoded = <Vec<semio_framework::PluginDependency> as semio_framework_value::FromValue>::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap());
        assert_eq!(decoded.is_ok(), parsed.is_ok(), "{}: the wire decoder and the JSON reader refuse the same dependency pins", row["id"]);
        assert_eq!(decoded.is_ok_and(|dependencies| semio_hub_stdio::catalog::validate_native_artifact_catalog_dependency(&dependencies).is_ok()), expected, "{}", row["id"]);
    }
}

#[test]
fn generic_plugin_builder_preserves_domain_owned_topic_contributions() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🧫️fixtures/📇️topic-contributions/🔣️.json"))).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let plugin_id = format!("builder-topic-{}", row["id"].as_str().unwrap());
        let contributions: Vec<semio_framework::TopicContribution> = serde_json::from_value(row["contributions"].clone()).unwrap();
        let mut builder = semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder(&plugin_id).label("Topic Fixture").version("0.1.0").package_id(format!("semio:{plugin_id}"));
        for contribution in contributions {
            builder = builder.contributes_topic(contribution);
        }
        let plugin = builder.try_library().unwrap();
        assert_eq!(serde_json::to_value(&plugin.manifest.topic_contributions).unwrap(), row["contributions"]);
        let value = semio_framework_value::ToValue::to_value(&plugin.manifest.topic_contributions);
        let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
        let roundtrip: Vec<semio_framework::TopicContribution> = semio_framework_value::FromValue::from_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&roundtrip).unwrap(), row["contributions"]);
    }
}

use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
use semio_hub_stdio::catalog::native_codec_factory_receipts;

#[test]
fn native_composition_and_validation_claims_are_disjoint_but_each_exclusive() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️catalog/🧫️fixtures/🧾️claim-authority/🔣️.json"))).unwrap();
    assert_eq!(ArtifactIdentityNamespace::validated_dialect().as_str(), "validated-dialect");
    let rows = fixture["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 8);
    for row in rows {
        let mut definition = ArtifactDefinition::new(ArtifactIdentity::parse("s.stdio.xml").unwrap());
        for (index, claim) in row["claims"].as_array().unwrap().iter().enumerate() {
            let category = claim["category"].as_str().unwrap();
            let authority = if category == "codec" {
                let encoded = ArtifactIdentityClaim::codec_extension(claim["codecSchema"].as_str().unwrap(), claim["extension"].as_str().unwrap()).unwrap();
                assert_eq!(encoded.value(), claim["value"].as_str().unwrap());
                encoded
            } else {
                ArtifactIdentityClaim::new(ArtifactIdentityNamespace::parse(claim["namespace"].as_str().unwrap()).unwrap(), claim["value"].as_str().unwrap()).unwrap()
            };
            let identity = if category == "codec" { format!("s.stdio.xml.standard.v1.codec.claim-{index}.v1") } else { format!("s.stdio.xml.{category}.claim-{index}.v1") };
            let capability = ArtifactCapability::new(ArtifactIdentity::parse(&identity).unwrap(), ArtifactCapabilityKind::parse(category).unwrap()).descriptor(b"claim authority fixture".to_vec()).unwrap().claim(authority).unwrap();
            definition = definition.capability(capability).unwrap();
        }
        let code = definition.validate().err().map(|error| error.code().to_owned()).unwrap_or_else(|| "accepted".into());
        assert_eq!(code, row["code"].as_str().unwrap(), "{}", row["id"]);
    }
}

#[test]
fn artifact_owned_native_codec_receipts_form_one_complete_static_bijection() {
    let receipts = native_codec_factory_receipts().expect("artifact-owned native codec receipts");
    let authored: serde_json::Value = serde_json::from_str(include_str!("../../🔌️plugin/📇️catalog/📜️native-codec-factories.json")).unwrap();
    let expected = authored["receipts"].as_array().unwrap().len();
    assert_eq!(receipts.len(), expected);
    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), expected);
    assert_eq!(receipts.iter().map(|receipt| receipt.descriptor_codec_id.as_str()).collect::<BTreeSet<_>>().len(), expected);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), expected);
    assert!(receipts.iter().all(|receipt| receipt.pack_schema_hash != [0; 32] && receipt.instantiate().is_ok()));
}

#[test]
fn native_catalog_matches_every_decoded_descriptor_kind_without_guest_app_assembly() {
    use semio_hub_stdio::catalog::{artifact_definitions, native_codec_artifact_kinds, validate_native_codec_artifact_kinds};
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️catalog/🧫️fixtures/📇️native-catalog-surface/🔣️.json"))).unwrap();
    let expected = native_codec_artifact_kinds();
    let receipts = native_codec_factory_receipts().unwrap();
    let definition_ids = artifact_definitions().unwrap().iter().map(|definition| definition.identity().as_str().to_owned()).collect::<BTreeSet<_>>();
    assert_eq!(expected.iter().map(|kind| kind.id.as_str()).collect::<BTreeSet<_>>(), receipts.iter().map(|receipt| receipt.artifact_kind.as_str()).collect());
    assert!(expected.iter().all(|kind| definition_ids.contains(&kind.id)));
    let canonical = |values: &[semio_framework_plugin::ArtifactKindSpec]| {
        let mut encoded: Vec<String> = values.iter().map(|kind| serde_json::to_string(kind).unwrap()).collect();
        encoded.sort();
        encoded
    };
    for case in fixture["cases"].as_array().unwrap() {
        let mut actual = expected.clone();
        match case["mutation"].as_str().unwrap() {
            "none" => {}
            "reverse" => actual.reverse(),
            "missing" => {
                actual.pop();
            }
            "extra" => actual.push(actual[0].clone()),
            "duplicate" => actual[1] = actual[0].clone(),
            "identity" => actual[0].id = "foreign.artifact".into(),
            "schema" => actual[0].schema = "foreign.schema".into(),
            "label" => actual[0].label = semio_framework_plugin::LocalizedLabel::native("Foreign", "Fremd"),
            "source-format" => actual[0].source_format = "foreign.format".into(),
            value => panic!("unknown neutral mutation {value}"),
        }
        let accepted = case["accepted"].as_bool().unwrap();
        assert_eq!(canonical(&actual) == canonical(&expected), accepted, "independent serde oracle {}", case["id"]);
        assert_eq!(validate_native_codec_artifact_kinds(&actual).is_ok(), accepted, "{}", case["id"]);
    }
}

#[test]
fn native_catalog_commitment_covers_all_definition_semantics_and_codec_authorities() {
    use semio_hub_stdio::catalog::{native_artifact_catalog_contribution, validate_native_artifact_catalog_contributions};
    fn reverse_object_fields(value: &mut semio_framework::DslValue) {
        match value {
            semio_framework::DslValue::Object(fields) => {
                fields.reverse();
                for (_, child) in fields {
                    reverse_object_fields(child);
                }
            }
            semio_framework::DslValue::Array(values) => {
                for child in values {
                    reverse_object_fields(child);
                }
            }
            _ => {}
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️catalog/🧫️fixtures/📇️native-catalog-surface/🧪️commitment.json"))).unwrap();
    let contribution = native_artifact_catalog_contribution().unwrap();
    let definition_count = semio_hub_stdio::catalog::artifact_definitions().unwrap().len();
    let codec_count = native_codec_factory_receipts().unwrap().len();
    let original = serde_json::to_value(&contribution).unwrap();
    println!("native-catalog-payload={}", serde_json::to_string(&original["payload"]).unwrap());
    assert_eq!(original["topic"], fixture["topic"]);
    assert_eq!(original["payload"]["definitions"].as_array().unwrap().len(), definition_count);
    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), codec_count);
    for case in fixture["cases"].as_array().unwrap() {
        let started = std::time::Instant::now();
        let progress = |stage: &str| {
            std::io::Write::write_fmt(&mut std::io::stderr().lock(), format_args!("[TRACE] native-catalog case={} stage={stage} elapsed-ms={}\n", case["id"], started.elapsed().as_millis())).unwrap();
        };
        progress("begin");
        let mut changed = original.clone();
        let operation = case["operation"].as_str().unwrap();
        let mut value = &mut changed["payload"];
        for part in case["path"].as_array().unwrap() {
            value = match part {
                serde_json::Value::String(key) => &mut value[key],
                serde_json::Value::Number(index) => &mut value[index.as_u64().unwrap() as usize],
                _ => panic!("invalid neutral JSON path"),
            };
        }
        match operation {
            "none" | "reverse-object-fields" | "missing-topic" | "duplicate-topic" | "foreign-topic" | "bare-topic" => {}
            "unknown-field" => {
                value["unknown"] = true.into();
            }
            "replace" => *value = "foreign".into(),
            "toggle" => *value = (!value.as_bool().unwrap()).into(),
            "remove-last" => {
                value.as_array_mut().unwrap().pop();
            }
            "append-first" => {
                let first = value[0].clone();
                value.as_array_mut().unwrap().push(first);
            }
            "reverse" => value.as_array_mut().unwrap().reverse(),
            _ => panic!("unknown neutral operation"),
        }
        if operation == "foreign-topic" {
            changed["topic"] = "stdio.artifact-catalog.v999".into();
        }
        if operation == "bare-topic" {
            changed["topic"] = "stdio.artifact-catalog".into();
        }
        let mut actual = vec![serde_json::from_value::<semio_framework::TopicContribution>(changed.clone()).unwrap()];
        if operation == "reverse-object-fields" {
            let before = actual[0].payload.clone();
            reverse_object_fields(&mut actual[0].payload);
            assert_ne!(before, actual[0].payload);
            assert_eq!(serde_json::to_value(&actual[0]).unwrap(), changed);
            assert_eq!(semio_framework_os_kernel::pack_rt::encode_wire_value(&before), semio_framework_os_kernel::pack_rt::encode_wire_value(&actual[0].payload));
        }
        if operation == "missing-topic" {
            actual.clear();
        }
        if operation == "duplicate-topic" {
            actual.push(actual[0].clone());
        }
        if operation == "bare-topic" {
            actual.push(contribution.clone());
        }
        let accepted = case["accepted"].as_bool().unwrap();
        assert_eq!(actual.len() == 1 && changed == original, accepted, "independent JSON equality {}", case["id"]);
        assert_eq!(validate_native_artifact_catalog_contributions(&actual).is_ok(), accepted, "{}", case["id"]);
        progress("validated");
        let value = semio_framework_value::ToValue::to_value(&actual);
        let packed = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
        progress("encoded");
        let decoded = semio_framework_os_kernel::pack_rt::decode_wire_value(&packed).unwrap();
        progress("decoded");
        assert_eq!(semio_framework_os_kernel::pack_rt::encode_wire_value(&decoded), packed);
        let roundtrip: Vec<semio_framework::TopicContribution> = semio_framework_value::FromValue::from_value(decoded).unwrap();
        assert_eq!(serde_json::to_value(&roundtrip).unwrap(), serde_json::to_value(&actual).unwrap());
        assert_eq!(validate_native_artifact_catalog_contributions(&roundtrip).is_ok(), accepted, "Pack {}", case["id"]);
        progress("complete");
    }
}

/// 🧬️ Generator and law of `🔌️plugin/📇️catalog/📜️native-codec-factories.json`'s `pack_schema_hash` column: each value is
/// the artifact owner's live opaque pack identity, independently of its protocol source SHA-256. With
/// `SEMIO_NATIVE_CODEC_PROJECTION=write` (the stdio `native-codec-projection` verb) the committed values are
/// rewritten in place; otherwise the committed projection must already equal the live receipts byte for byte.
#[test]
fn native_codec_projection_pack_schema_hashes_equal_live_receipts() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🔌️plugin/📇️catalog/📜️native-codec-factories.json");
    let committed = std::fs::read_to_string(&path).unwrap();
    let mut generated = committed.clone();
    use sha2::Digest;
    let projection: serde_json::Value = serde_json::from_str(&committed).unwrap();
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../..");
    for row in projection["receipts"].as_array().unwrap() {
        let source = std::fs::read(repo_root.join(row["protocol_path"].as_str().unwrap())).unwrap();
        assert_eq!(format!("{:x}", sha2::Sha256::digest(&source)), row["protocol_source_sha256"].as_str().unwrap());
        assert_eq!(semio_framework_hash::Sha256::digest(&source).iter().map(|byte| format!("{byte:02x}")).collect::<String>(), row["protocol_source_sha256"].as_str().unwrap());
        let definition: serde_json::Value = serde_json::from_slice(&std::fs::read(repo_root.join(row["definition_path"].as_str().unwrap())).unwrap()).unwrap();
        let declared = definition["codecs"].as_array().unwrap().iter().find(|codec| codec["native_factory"]["factory_id"] == row["factory_id"]).unwrap();
        assert_eq!(declared["native_factory"]["pack_schema_hash"], row["pack_schema_hash"]);
    }
    let key = "\"pack_schema_hash\": \"";
    for receipt in semio_hub_stdio::catalog::live_native_codec_factory_receipts().expect("live artifact-owned native codec receipts") {
        let row = generated.find(&format!("\"factory_id\": \"{}\"", receipt.factory_id)).unwrap_or_else(|| panic!("projection omits {}", receipt.factory_id));
        let start = row + generated[row..].find(key).expect("projection row carries pack_schema_hash") + key.len();
        generated.replace_range(start..start + 64, &receipt.pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>());
    }
    if std::env::var("SEMIO_NATIVE_CODEC_PROJECTION").as_deref() == Ok("write") {
        std::fs::write(&path, &generated).unwrap();
    } else {
        assert_eq!(generated, committed, "stdio native codec projection is stale: run the stdio native-codec-projection verb");
    }
}

/// 🕸️ LAS binds its live structural graph identity independently from protocol-source SHA-256.
#[test]
fn las_native_receipt_binds_canonical_graph_and_independent_blake3() {
    fn varint(out: &mut Vec<u8>, mut value: u64) {
        while value >= 128 { out.push((value as u8 & 127) | 128); value >>= 7; }
        out.push(value as u8);
    }
    fn shape(out: &mut Vec<u8>, value: &serde_json::Value) {
        match value["kind"].as_str().unwrap() {
            "bool" => out.push(1), "int" => out.push(2), "uint" => out.push(3), "text" => out.push(5), "bytes64" => out.push(6),
            "tuple" => { out.push(8); shape(out, &value["item"]); if let Some(len) = value["len"].as_u64() { out.push(1); varint(out, len); } else { out.push(0); } }
            "list" => { out.push(9); shape(out, &value["item"]); }
            "record" => { out.push(10); varint(out, value["record"].as_u64().unwrap()); }
            other => panic!("unexpected LAS structural shape {other}"),
        }
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️publication/🧫️fixtures/🧬️trusted-stdio-catalog/🔣️.json"))).unwrap();
    let spec = <semio_s_artifact_stdio_las::LasSnapshot as semio_framework_os_kernel::ArtifactPack>::record_spec().unwrap();
    let graph = semio_framework_os_kernel::os_pack::PackSchemaGraph::of(&spec);
    let json: serde_json::Value = serde_json::from_str(&semio_framework_os_kernel::os_pack::json::to_string(&graph.to_json())).unwrap();
    let records = json.as_array().unwrap();
    let mut independent = Vec::new();
    varint(&mut independent, records.len() as u64);
    for fields in records {
        let fields = fields.as_array().unwrap(); varint(&mut independent, fields.len() as u64);
        for field in fields {
            varint(&mut independent, field["id"].as_u64().unwrap());
            let key = field["key"].as_str().unwrap(); varint(&mut independent, key.len() as u64); independent.extend_from_slice(key.as_bytes());
            independent.push(u8::from(field["optional"].as_bool().unwrap()) | (u8::from(field["flatten"].as_bool().unwrap()) << 1));
            shape(&mut independent, &field["shape"]);
        }
    }
    assert_eq!(independent, graph.canonical_bytes());
    let hash = blake3::hash(&independent).to_hex().to_string();
    assert_eq!(hash, fixture["structuralCodec"]["packSchemaHash"].as_str().unwrap());
    let receipts = semio_hub_stdio::catalog::live_native_codec_factory_receipts().unwrap();
    let las = receipts.iter().find(|receipt| receipt.artifact_kind == "s.stdio.las").unwrap();
    assert_eq!(las.pack_schema_hash, *blake3::hash(&independent).as_bytes());
    assert_ne!(hash, fixture["structuralCodec"]["protocolSourceSha256"].as_str().unwrap());
}

/// 🔤️ LAW: txt, tsv and html open over the hub through linked native codecs. For every case of the neutral fixture
/// `📇️registry/🧫️fixtures/📇️native-text-codecs` the artifact-owned receipt's codec compiles the source (a new document's ops
/// log is its bare `doc` header) into a pack whose decoded snapshot is the fixture's snapshot and whose printed mirror and ops
/// log compile back to the identical pack; the same snapshots are what independent readers (Python's `str.splitlines`, `csv`
/// and `html.parser`) derive from each source.
#[semio_framework_async_macros::async_test]
async fn text_document_codecs_compile_their_neutral_fixture_through_the_linked_receipts() {
    fn neutral<T: semio_framework_os_kernel::ToValue>(snapshot: T) -> serde_json::Value {
        serde_json::from_str(&pack::json_to_string(&pack::json_from_dsl_value(&snapshot.to_value()))).unwrap()
    }
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️catalog/🧫️fixtures/📇️native-text-codecs/🔣️.json"))).unwrap();
    assert_eq!(fixture["schema"], "semio.stdio.native-text-codecs/v1");
    let receipts = native_codec_factory_receipts().expect("artifact-owned native codec receipts");
    let cases = fixture["cases"].as_array().unwrap();
    for kind in ["s.stdio.txt", "s.stdio.tsv", "s.stdio.html"] {
        assert!(cases.iter().any(|case| case["artifactKind"] == kind), "{kind} has no neutral case");
    }
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let kind = case["artifactKind"].as_str().unwrap();
        let receipt = receipts.iter().find(|receipt| receipt.artifact_kind == kind).unwrap_or_else(|| panic!("{id}: {kind} owns no linked receipt"));
        assert_eq!(receipt.factory_id, case["factoryId"].as_str().unwrap(), "{id}");
        let codec = receipt.instantiate().expect("verified linked codec");
        let genesis = format!("doc cx1 schema=\"{}\"\n", codec.schema);
        let (files, mirror) = (codec.compile_dsl)(case["source"].as_str().unwrap(), &genesis).await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        let printed = (codec.print_mirror)(&files.pack, &files.spr).await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(printed.dsl, mirror, "{id}: the pack prints the mirror its compile returned");
        let (again, _) = (codec.compile_dsl)(&printed.dsl, &printed.ops).await.unwrap_or_else(|error| panic!("{id}: {error:?}"));
        assert_eq!(again.pack, files.pack, "{id}: the mirror compiles back to the identical pack");
        let snapshot = match kind {
            "s.stdio.txt" => neutral(<semio_s_artifact_stdio_txt::TxtSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            "s.stdio.tsv" => neutral(<semio_s_artifact_stdio_tsv::TsvSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            "s.stdio.html" => neutral(<semio_s_artifact_stdio_html::HtmlSnapshot as semio_framework_os_kernel::ArtifactPack>::decode_pack(&files.pack).unwrap()),
            other => panic!("{id}: {other} is outside the text codec fixture"),
        };
        assert_eq!(snapshot, case["snapshot"], "{id}");
    }
}
