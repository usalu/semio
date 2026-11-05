use serde::Deserialize;

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    pub formats: Vec<Format>,
    pub unsupported: Vec<Unsupported>,
    pub transport: Transport,
    pub lifecycle: Lifecycle,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Format {
    pub app_id: String,
    pub format_kind: String,
    pub extension: String,
    pub media_type: String,
    pub binary: bool,
}

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Unsupported {
    pub app_id: String,
}

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transport {
    pub schema: String,
    octets: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Lifecycle {
    current_instance_id: u64,
    opened_instance_id: u64,
    pub expected: Expected,
}

#[doc(hidden)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Expected {
    save_instance_id: u64,
    preserved_instance_id: u64,
    opened_instance_id: u64,
    opened_history_entries: usize,
}

const BATCH_A_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.md@commonmark/*#editor",
    "s.stdio.html@5/*#editor",
    "s.stdio.txt@utf-8/*#editor",
    "s.stdio.xml@1.0/*#editor",
    "s.stdio.xml@1.0/valid#editor",
    "s.stdio.json@rfc8259/*#editor",
    "s.stdio.json@rfc8259/i-json#editor",
];

const BATCH_B_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.csv@rfc4180/*#editor",
    "s.stdio.docx@ecma-376/*#editor",
    "s.stdio.docx@ecma-376/strict#editor",
    "s.stdio.docx@ecma-376/transitional#editor",
    "s.stdio.tsv@iana/*#editor",
    "s.stdio.xlsx@ecma-376/*#editor",
    "s.stdio.xlsx@ecma-376/transitional#editor",
    "s.stdio.xlsx@ecma-376/strict#editor",
    "s.stdio.pptx@ecma-376/*#editor",
    "s.stdio.pptx@ecma-376/transitional#editor",
    "s.stdio.pptx@ecma-376/strict#editor",
    "s.stdio.idml@8.0/*#editor",
];

const BATCH_C_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.png@1.2/*#editor",
    "s.stdio.bmp@v3/*#editor",
    "s.stdio.jpg@jfif-1.01/*#editor",
    "s.stdio.jpg@jfif-1.01/baseline#editor",
    "s.stdio.tiff@6.0/*#editor",
    "s.stdio.tiff@6.0/baseline#editor",
    "s.stdio.gif@87a/*#editor",
    "s.stdio.gif@89a/*#editor",
];

const BATCH_E_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.obj@3.0/*#editor",
    "s.stdio.stl@ascii/*#editor",
    "s.stdio.ply@1.0/*#editor",
    "s.stdio.gltf@2.0/*#editor",
    "s.stdio.las@1.0/*#editor",
    "s.stdio.step@ap214/*#editor",
    "s.stdio.step@ap214/cc1#editor",
    "s.stdio.step@ap214/cc2#editor",
    "s.stdio.step@ap214/cc3#editor",
    "s.stdio.step@ap214/cc4#editor",
    "s.stdio.step@ap214/cc5#editor",
    "s.stdio.step@ap214/cc6#editor",
];

const BATCH_F_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.epw@energyplus/*#editor",
    "s.stdio.bcf@2.1/*#editor",
    "s.stdio.ifc@4/*#editor",
    "s.stdio.ifc@4x3/*#editor",
    "s.stdio.ifc@2x3/*#editor",
    "s.energy.model@1/*#editor",
    "s.stdio.ifc@2x3/cobie#editor",
    "s.stdio.ifc@2x3/cv20#editor",
    "s.stdio.ifc@2x3/sav#editor",
];

const BATCH_G_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.wav@riff-pcm/*#editor",
    "s.stdio.mp3@mpeg1-layer3/*#editor",
    "s.stdio.mp4@isobmff/*#editor",
    "s.stdio.avi@1.0/*#editor",
];

const BATCH_D_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.dxf@r12/*#editor",
    "s.stdio.dwg@ac1018/*#editor",
    "s.stdio.dwg@ac1024/*#editor",
    "s.stdio.pdf@1.4/*#editor",
    "s.stdio.pdf@1.4/a#editor",
    "s.stdio.pdf@1.4/x#editor",
    "s.stdio.pdf@1.7/*#editor",
    "s.stdio.pdf@1.7/a#editor",
    "s.stdio.pdf@1.7/e#editor",
    "s.stdio.pdf@1.7/h#editor",
    "s.stdio.pdf@1.7/ua#editor",
    "s.stdio.pdf@1.7/vt#editor",
    "s.stdio.pdf@1.7/x#editor",
    "s.stdio.svg@1.1/*#editor",
    "s.stdio.svg@1.1/basic#editor",
    "s.stdio.svg@1.1/tiny#editor",
];

const BATCH_H_NATURAL_FILE_APPS: &[&str] = &["s.stdio.zip@2.0/*#editor", "s.stdio.zip@2.0/iso21320#editor", "s.stdio.deflate@rfc1950/*#editor", "s.stdio.binary@raw/*#editor"];

const BATCH_I_NATURAL_FILE_APPS: &[&str] = &[
    "s.stdio.semio@v1/*#editor",
    "s.stdio.semio@v1/animation#editor",
    "s.stdio.semio@v1/audio#editor",
    "s.stdio.semio@v1/brep#editor",
    "s.stdio.semio@v1/cad#editor",
    "s.stdio.semio@v1/document#editor",
    "s.stdio.semio@v1/drawing#editor",
    "s.stdio.semio@v1/flow#editor",
    "s.stdio.semio@v1/graph#editor",
    "s.stdio.semio@v1/image#editor",
    "s.stdio.semio@v1/kit#editor",
    "s.stdio.semio@v1/mesh#editor",
    "s.stdio.semio@v1/model#editor",
    "s.stdio.semio@v1/object#editor",
    "s.stdio.semio@v1/presentation#editor",
    "s.stdio.semio@v1/table#editor",
    "s.stdio.semio@v1/text#editor",
    "s.stdio.semio@v1/value#editor",
    "s.stdio.semio@v1/video#editor",
];

#[test]
fn natural_file_actions_publish_exact_paired_codec_metadata() {
    let fixture: Fixture = serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../🧪️tests/🧫️fixtures/📄️natural-file-lifecycle/🔣️.json"))).expect("natural-file lifecycle fixture parses");
    let declared: std::collections::BTreeSet<&str> = fixture.formats.iter().map(|row| row.app_id.as_str()).collect();
    assert_eq!(declared.len(), fixture.formats.len(), "every natural-file app id is declared exactly once");
    for expected in [BATCH_A_NATURAL_FILE_APPS, BATCH_B_NATURAL_FILE_APPS, BATCH_C_NATURAL_FILE_APPS, BATCH_D_NATURAL_FILE_APPS, BATCH_E_NATURAL_FILE_APPS, BATCH_F_NATURAL_FILE_APPS, BATCH_G_NATURAL_FILE_APPS, BATCH_H_NATURAL_FILE_APPS, BATCH_I_NATURAL_FILE_APPS].into_iter().flatten() {
        assert!(declared.contains(expected), "{expected} must declare its natural file");
    }
    let named: std::collections::BTreeSet<&str> = [BATCH_A_NATURAL_FILE_APPS, BATCH_B_NATURAL_FILE_APPS, BATCH_C_NATURAL_FILE_APPS, BATCH_D_NATURAL_FILE_APPS, BATCH_E_NATURAL_FILE_APPS, BATCH_F_NATURAL_FILE_APPS, BATCH_G_NATURAL_FILE_APPS, BATCH_H_NATURAL_FILE_APPS, BATCH_I_NATURAL_FILE_APPS].into_iter().flatten().copied().collect();
    assert_eq!(declared, named, "every declared natural-file app belongs to exactly one named batch set and vice versa");
    for row in fixture.formats {
        let codec = semio_plugin_decl_app::NaturalFileCodec {
            format_kind: Box::leak(row.format_kind.clone().into_boxed_str()),
            extension: Box::leak(row.extension.clone().into_boxed_str()),
            media_type: Box::leak(row.media_type.clone().into_boxed_str()),
            binary: row.binary,
        };
        let actions = semio_plugin_decl_app::natural_file_action_definitions(codec);
        assert_eq!(actions[0].id, semio_framework::SAVE_ARTIFACT_FILE_ACTION_ID, "{} save action", row.app_id);
        assert_eq!(actions[1].id, semio_framework::OPEN_ARTIFACT_FILE_ACTION_ID, "{} open action", row.app_id);
        for action in actions {
            let value = serde_json::to_value(action).expect("action serializes");
            let defaults = value["args"].as_array().expect("action args").iter().map(|argument| (argument["id"].as_str().expect("arg id"), &argument["default"])).collect::<std::collections::BTreeMap<_, _>>();
            assert_eq!(defaults["formatKind"].as_str(), Some(row.format_kind.as_str()));
            assert_eq!(defaults["extension"].as_str(), Some(row.extension.as_str()));
            assert_eq!(defaults["mediaType"].as_str(), Some(row.media_type.as_str()));
            assert_eq!(defaults["binary"].as_bool(), Some(row.binary));
        }
    }
    let descriptor = semio_plugin_decl_app::MediaArtifactDescriptor {
        edge_id: None,
        port_id: Some(semio_plugin_decl_app::NATURAL_FILE_PORT.to_string()),
        kind_id: Some("s.stdio.csv@rfc4180".into()),
        media_type: None,
        wire: semio_framework::MediaWireFormat::Binary { format_kind: "s.stdio.csv@rfc4180".into() },
        blob_hash: None,
    };
    let descriptor = serde_json::Value::from(semio_framework_value::ToValue::to_value(&descriptor));
    assert_eq!(descriptor["portId"], semio_plugin_decl_app::NATURAL_FILE_PORT);
    assert_eq!(descriptor["kindId"], "s.stdio.csv@rfc4180");
    assert_eq!(descriptor["wire"]["kind"], "binary");
    assert_eq!(descriptor["wire"]["format_kind"], "s.stdio.csv@rfc4180");
    let media = semio_plugin_decl_app::natural_file_input_media(
        semio_plugin_decl_app::NaturalFileCodec { format_kind: Box::leak(fixture.transport.schema.clone().into_boxed_str()), extension: ".bin", media_type: "application/octet-stream", binary: true },
        semio_framework::MediaType { class: semio_framework::MediaClass::Data, form: semio_framework::MediaForm::Value },
        fixture.transport.octets.clone(),
    );
    let semio_framework::MediaPayload::Intrinsic { schema, value: semio_framework_value::DslValue::Bytes(bytes) } = media.payload else { panic!("natural bytes must remain intrinsic octets") };
    assert_eq!(schema, fixture.transport.schema);
    assert_eq!(bytes, fixture.transport.octets);
    assert!(fixture.unsupported.iter().all(|row| !row.app_id.is_empty()));
    assert_ne!(fixture.lifecycle.current_instance_id, fixture.lifecycle.opened_instance_id);
    assert_eq!(fixture.lifecycle.expected.save_instance_id, fixture.lifecycle.current_instance_id);
    assert_eq!(fixture.lifecycle.expected.preserved_instance_id, fixture.lifecycle.current_instance_id);
    assert_eq!(fixture.lifecycle.expected.opened_instance_id, fixture.lifecycle.opened_instance_id);
    assert_eq!(fixture.lifecycle.expected.opened_history_entries, 0);
}
