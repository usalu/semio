use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    formats: Vec<Format>,
    unsupported: Vec<Unsupported>,
    transport: Transport,
    lifecycle: Lifecycle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Format {
    app_id: String,
    format_kind: String,
    extension: String,
    media_type: String,
    binary: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Unsupported {
    app_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Transport {
    schema: String,
    octets: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Lifecycle {
    current_instance_id: u64,
    opened_instance_id: u64,
    expected: Expected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    save_instance_id: u64,
    preserved_instance_id: u64,
    opened_instance_id: u64,
    opened_history_entries: usize,
}

#[test]
fn natural_file_actions_publish_exact_paired_codec_metadata() {
    let fixture: Fixture = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧫️fixtures/📄️natural-file-lifecycle/🔣️.json"))).expect("natural-file lifecycle fixture parses");
    assert_eq!(fixture.formats.len(), 28);
    for row in fixture.formats {
        let codec = crate::NaturalFileCodec {
            format_kind: Box::leak(row.format_kind.clone().into_boxed_str()),
            extension: Box::leak(row.extension.clone().into_boxed_str()),
            media_type: Box::leak(row.media_type.clone().into_boxed_str()),
            binary: row.binary,
        };
        let actions = crate::app::natural_file_action_definitions(codec);
        assert_eq!(actions[0].id, crate::SAVE_ARTIFACT_FILE_ACTION_ID, "{} save action", row.app_id);
        assert_eq!(actions[1].id, crate::OPEN_ARTIFACT_FILE_ACTION_ID, "{} open action", row.app_id);
        for action in actions {
            let value = serde_json::to_value(action).expect("action serializes");
            let defaults = value["args"].as_array().expect("action args").iter().map(|argument| (argument["id"].as_str().expect("arg id"), &argument["default"])).collect::<std::collections::BTreeMap<_, _>>();
            assert_eq!(defaults["formatKind"].as_str(), Some(row.format_kind.as_str()));
            assert_eq!(defaults["extension"].as_str(), Some(row.extension.as_str()));
            assert_eq!(defaults["mediaType"].as_str(), Some(row.media_type.as_str()));
            assert_eq!(defaults["binary"].as_bool(), Some(row.binary));
        }
    }
    let descriptor = crate::app::MediaArtifactDescriptor {
        edge_id: None,
        port_id: Some(crate::NATURAL_FILE_PORT.to_string()),
        kind_id: Some("s.stdio.csv@rfc4180".into()),
        media_type: None,
        wire: crate::MediaWireFormat::Binary { format_kind: "s.stdio.csv@rfc4180".into() },
        blob_hash: None,
    };
    let descriptor = serde_json::Value::from(semio_framework_value::ToValue::to_value(&descriptor));
    assert_eq!(descriptor["portId"], crate::NATURAL_FILE_PORT);
    assert_eq!(descriptor["kindId"], "s.stdio.csv@rfc4180");
    assert_eq!(descriptor["wire"]["kind"], "binary");
    assert_eq!(descriptor["wire"]["format_kind"], "s.stdio.csv@rfc4180");
    let media = crate::app::natural_file_input_media(
        crate::NaturalFileCodec { format_kind: Box::leak(fixture.transport.schema.clone().into_boxed_str()), extension: ".bin", media_type: "application/octet-stream", binary: true },
        crate::MediaType { class: crate::MediaClass::Data, form: crate::MediaForm::Value },
        fixture.transport.octets.clone(),
    );
    let crate::MediaPayload::Intrinsic { schema, value: semio_framework_value::DslValue::Bytes(bytes) } = media.payload else { panic!("natural bytes must remain intrinsic octets") };
    assert_eq!(schema, fixture.transport.schema);
    assert_eq!(bytes, fixture.transport.octets);
    assert!(fixture.unsupported.iter().all(|row| !row.app_id.is_empty()));
    assert_ne!(fixture.lifecycle.current_instance_id, fixture.lifecycle.opened_instance_id);
    assert_eq!(fixture.lifecycle.expected.save_instance_id, fixture.lifecycle.current_instance_id);
    assert_eq!(fixture.lifecycle.expected.preserved_instance_id, fixture.lifecycle.current_instance_id);
    assert_eq!(fixture.lifecycle.expected.opened_instance_id, fixture.lifecycle.opened_instance_id);
    assert_eq!(fixture.lifecycle.expected.opened_history_entries, 0);
}
