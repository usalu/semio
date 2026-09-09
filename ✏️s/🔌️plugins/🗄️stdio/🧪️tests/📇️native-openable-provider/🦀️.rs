use std::collections::BTreeSet;

#[test]
fn native_catalog_dependency_is_exactly_its_compiled_owner() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../../🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🔗️compiled-dependencies/🔣️.json"))).unwrap();
    let expected = semio_s_plugin_stdio::registry::native_artifact_catalog_dependency().unwrap();
    assert_eq!(serde_json::to_value(&expected).unwrap(), fixture["nativeCases"][0]["dependencies"][0]);
    let rows = fixture["nativeCases"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    for row in rows {
        let dependencies: Vec<semio_framework::PluginDependency> = serde_json::from_value(row["dependencies"].clone()).unwrap();
        let accepted = semio_s_plugin_stdio::registry::validate_native_artifact_catalog_dependency(&dependencies).is_ok();
        assert_eq!(accepted, row["accepted"].as_bool().unwrap(), "{}", row["id"]);
        let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&semio_framework::to_dsl_value(&dependencies).unwrap());
        let decoded: Vec<semio_framework::PluginDependency> = semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
        assert_eq!(semio_s_plugin_stdio::registry::validate_native_artifact_catalog_dependency(&decoded).is_ok(), accepted);
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
        let value = semio_framework::to_dsl_value(&plugin.manifest.topic_contributions).unwrap();
        let bytes = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
        let roundtrip: Vec<semio_framework::TopicContribution> = semio_framework::from_dsl_value(semio_framework_os_kernel::pack_rt::decode_wire_value(&bytes).unwrap()).unwrap();
        assert_eq!(serde_json::to_value(&roundtrip).unwrap(), row["contributions"]);
    }
}

use semio_framework_plugin::{ArtifactCapability, ArtifactCapabilityKind, ArtifactDefinition, ArtifactIdentity, ArtifactIdentityClaim, ArtifactIdentityNamespace};
use semio_s_plugin_stdio::registry::native_codec_factory_receipts;

#[test]
fn native_composition_and_validation_claims_are_disjoint_but_each_exclusive() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️registry/🧫️fixtures/🧾️claim-authority/🔣️.json"))).unwrap();
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
    assert_eq!(receipts.len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.factory_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| receipt.descriptor_codec_id.as_str()).collect::<BTreeSet<_>>().len(), 26);
    assert_eq!(receipts.iter().map(|receipt| (receipt.artifact_kind.as_str(), receipt.schema.as_str())).collect::<BTreeSet<_>>().len(), 26);
    assert!(receipts.iter().all(|receipt| receipt.pack_schema_hash != [0; 32] && receipt.instantiate().is_ok()));
}

#[test]
fn native_catalog_matches_every_decoded_descriptor_kind_without_guest_app_assembly() {
    use semio_s_plugin_stdio::registry::{artifact_definitions, native_codec_artifact_kinds, validate_native_codec_artifact_kinds};
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️registry/🧫️fixtures/📇️native-catalog-surface/🔣️.json"))).unwrap();
    let expected = native_codec_artifact_kinds();
    assert_eq!(artifact_definitions().unwrap().len(), fixture["definitionCount"].as_u64().unwrap() as usize);
    assert_eq!(expected.len(), fixture["codecCount"].as_u64().unwrap() as usize);
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
            "name" => actual[0].name = "Foreign".into(),
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
    use semio_s_plugin_stdio::registry::{native_artifact_catalog_contribution, validate_native_artifact_catalog_contributions};
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
    let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../📇️registry/🧫️fixtures/📇️native-catalog-surface/🧪️commitment.json"))).unwrap();
    let contribution = native_artifact_catalog_contribution().unwrap();
    let original = serde_json::to_value(&contribution).unwrap();
    println!("[DEBUG] native-catalog-payload={}", serde_json::to_string(&original["payload"]).unwrap());
    assert_eq!(original["topic"], fixture["topic"]);
    assert_eq!(original["payload"]["definitions"].as_array().unwrap().len(), 36);
    assert_eq!(original["payload"]["codecs"].as_array().unwrap().len(), 26);
    for case in fixture["cases"].as_array().unwrap() {
        let started = std::time::Instant::now();
        let progress = |stage: &str| {
            std::io::Write::write_fmt(&mut std::io::stderr().lock(), format_args!("[DEBUG] native-catalog case={} stage={stage} elapsed-ms={}\n", case["id"], started.elapsed().as_millis())).unwrap();
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
        let value = semio_framework::to_dsl_value(&actual).unwrap();
        let packed = semio_framework_os_kernel::pack_rt::encode_wire_value(&value);
        progress("encoded");
        let decoded = semio_framework_os_kernel::pack_rt::decode_wire_value(&packed).unwrap();
        progress("decoded");
        assert_eq!(semio_framework_os_kernel::pack_rt::encode_wire_value(&decoded), packed);
        let roundtrip: Vec<semio_framework::TopicContribution> = semio_framework::from_dsl_value(decoded).unwrap();
        assert_eq!(serde_json::to_value(&roundtrip).unwrap(), serde_json::to_value(&actual).unwrap());
        assert_eq!(validate_native_artifact_catalog_contributions(&roundtrip).is_ok(), accepted, "Pack {}", case["id"]);
        progress("complete");
    }
}
