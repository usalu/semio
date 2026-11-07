//! 📄️ Populated native editor payloads share neutral expectations and independent JSON observations.
use semio_plugin::ArtifactEditor;
use semio_framework_value::ToValue;
use serde_json::Value;

struct EncodedCase {
    schema: &'static str,
    bytes: Vec<u8>,
    accepts: fn(&[u8]) -> bool,
}

fn accepts<E: ArtifactEditor>(bytes: &[u8]) -> bool { E::decode_natural_file(bytes).is_ok() }

fn check<E: ArtifactEditor>(subset: &str, input: E::Snapshot, fixture: &Value) -> EncodedCase
where E::Snapshot: PartialEq + std::fmt::Debug + Default + ToValue {
    let row = fixture["rows"].as_array().unwrap().iter().find(|row| row["subset"] == subset).unwrap();
    assert_eq!(E::DOCUMENT_SCHEMA, row["schema"].as_str().unwrap());
    let codec = E::natural_file_codec().expect("actual editor declares its natural codec");
    assert_eq!(codec.format_kind, row["formatKind"].as_str().unwrap());
    assert_eq!(codec.extension, row["extension"].as_str().unwrap());
    assert_eq!(codec.media_type, row["mediaType"].as_str().unwrap());
    assert_eq!(codec.binary, row["binary"].as_bool().unwrap());
    assert_ne!(input, E::Snapshot::default(), "{subset}: the real input contains authored content");
    let independent: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&input)).unwrap();
    assert_eq!(independent["schema"], row["schema"]);
    let nonempty = &row["nonempty"];
    let selected = independent.pointer(nonempty["pointer"].as_str().unwrap()).expect("neutral authored-content path exists");
    let count = match nonempty["kind"].as_str().unwrap() {
        "array" => selected.as_array().expect("neutral path names an array").len(),
        "object" => selected.as_object().expect("neutral path names an object").len(),
        other => panic!("unknown neutral content discriminator {other}"),
    };
    assert!(count >= nonempty["minimum"].as_u64().unwrap() as usize);
    let bytes = E::encode_natural_file(&input).expect("actual editor encodes its populated input");
    assert!(!bytes.is_empty());
    let restored = E::decode_natural_file(&bytes).expect("actual editor decodes its own native payload");
    assert_eq!(restored, input);
    let observed: Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).unwrap();
    assert_eq!(observed, independent);
    assert_eq!(E::encode_natural_file(&restored).unwrap(), bytes);
    assert!(E::decode_natural_file(&[]).is_err());
    assert!(E::decode_natural_file(&bytes[..1]).is_err());
    let mut corrupt = bytes.clone();
    corrupt[0] ^= 0xff;
    assert!(E::decode_natural_file(&corrupt).is_err());
    EncodedCase { schema: E::DOCUMENT_SCHEMA, bytes, accepts: accepts::<E> }
}

#[test]
fn nineteen_populated_editor_native_roundtrips_and_cross_subset_refusals() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let cases = [
        check::<semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor>("audio", semio_s_artifact_stdio_semio_audio_model::schema::snapshot::demo_audio_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor>("animation", semio_s_artifact_stdio_semio_animation_schema::schema::snapshot::demo_animation_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor>("brep", semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::demo_brep_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor>("document", semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot::demo_semio_document_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor>("drawing", semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot::demo_drawing_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_value::editor::SemioValueEditor>("value", semio_s_artifact_stdio_semio_value_model::schema::snapshot::demo_semio_value_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_text::editor::SemioTextEditor>("text", semio_s_artifact_stdio_semio_text_model::schema::snapshot::demo_text_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor>("mesh", semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot::demo_mesh_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor>("cad", semio_s_artifact_stdio_semio_cad_snapshot::schema::snapshot::demo_cad_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor>("video", semio_s_artifact_stdio_semio_video_model::schema::snapshot::demo_video_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_image::editor::SemioImageEditor>("image", semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot::demo_image_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor>("presentation", semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot::demo_semio_presentation_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>("graph", semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot::demo_graph_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor>("flow", semio_s_artifact_stdio_semio_flow_snapshot::schema::snapshot::demo_flow_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor>("object", semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot::demo_object_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_model::editor::SemioModelEditor>("model", semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot::demo_semio_model_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_editor::editor::SemioAnyEditor>("*", semio_s_artifact_stdio_semio_snapshot::schema::snapshot::demo_semio_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor>("kit", semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot::demo_kit_snapshot(), &fixture),
        check::<semio_s_artifact_stdio_semio_table::editor::SemioTableEditor>("table", semio_s_artifact_stdio_semio_table_model::schema::snapshot::demo_table_snapshot(), &fixture),
    ];
    assert_eq!(cases.len(), fixture["rows"].as_array().unwrap().len());
    for decoder in &cases {
        for source in &cases {
            assert_eq!((decoder.accepts)(&source.bytes), decoder.schema == source.schema, "{} receiving {}", decoder.schema, source.schema);
        }
    }
}

fn check_examples<E:ArtifactEditor>(subset:&str,input:E::Snapshot,fixture:&Value,examples_fixture:&Value,schema_of:fn(&E::Snapshot)->&str)
where E::Snapshot:semio_framework_pack::snapshot::ArtifactDsl+PartialEq+std::fmt::Debug+Default+ToValue{
    use semio_framework_pack::snapshot::{ArtifactDsl,ArtifactPack};
    let row=examples_fixture["rows"].as_array().unwrap().iter().find(|row|row["subset"]==subset).unwrap();
    let canonical=input.print_dsl();
    let canonical_pack=input.encode_pack();
    assert!(canonical.len()<=262_144&&canonical_pack.len()<=262_144,"bounded canonical example evidence");
    assert_eq!(schema_of(&input),E::DOCUMENT_SCHEMA,"populated source owns the current typed schema");
    let sources=E::examples();assert_eq!(sources.len(),1,"{subset}: exactly one declared typed example");
    let source=&sources[0];assert_eq!(source.id(),row["id"].as_str().unwrap());
    let action=examples_fixture["pickerAction"].as_str().unwrap();
    let mut arguments=Some(semio_framework_value::DslValue::Object(vec![("exampleId".into(),semio_framework_value::DslValue::String(source.id().into()))]));
    let command=E::command_from_action(action,&mut arguments,&mut None).expect("public picker action produces its actual command");
    assert_eq!(E::command_id(&command),action);
    let encoded=semio_os_kernel::OpBinary::encode_op(&command).unwrap();
    if row["commandEncoding"]=="json"{let independent:Value=serde_json::from_slice(&encoded).unwrap();assert!(independent.is_object());}
    let restored=<E::Command as semio_os_kernel::OpBinary>::decode_op(&encoded).unwrap();
    assert_eq!(E::command_id(&restored),action);
    let text=source.document_json();
    assert_eq!(text,canonical,"{subset}: authored example equals actual current typed print");
    let parsed=E::Snapshot::parse_dsl(&text).expect("public example source parses through its actual snapshot");
    assert_eq!(schema_of(&parsed),E::DOCUMENT_SCHEMA);
    assert_eq!(parsed,input,"public example retains every populated field");
    let natural_row=fixture["rows"].as_array().unwrap().iter().find(|row|row["subset"]==subset).unwrap();
    let observed:Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&parsed)).unwrap();
    assert_eq!(observed["schema"],row["schema"]);
    let selected=observed.pointer(natural_row["nonempty"]["pointer"].as_str().unwrap()).unwrap();
    assert!(match natural_row["nonempty"]["kind"].as_str().unwrap(){"array"=>selected.as_array().unwrap().len(),"object"=>selected.as_object().unwrap().len(),_=>unreachable!()}>=natural_row["nonempty"]["minimum"].as_u64().unwrap()as usize);
    let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for path in row["assetCopies"].as_array().unwrap(){assert_eq!(std::fs::read_to_string(root.join(path.as_str().unwrap())).unwrap(),canonical,"current typed DSL copy {}",path);}
    for path in row["packCopies"].as_array().unwrap(){assert_eq!(std::fs::read(root.join(path.as_str().unwrap())).unwrap(),canonical_pack,"current typed Pack copy {}",path);}
    for id in examples_fixture["unknownIds"].as_array().unwrap(){assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<E::Snapshot>(id.as_str().unwrap(),source.id(),&text,E::DOCUMENT_SCHEMA,schema_of).is_err());}
    for bad in examples_fixture["malformedSources"].as_array().unwrap(){assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<E::Snapshot>(source.id(),source.id(),bad.as_str().unwrap(),E::DOCUMENT_SCHEMA,schema_of).is_err());}
    let malformed_envelope=format!("not a semio envelope\n{}",text.split_once('\n').unwrap().1);
    assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<E::Snapshot>(source.id(),source.id(),&malformed_envelope,E::DOCUMENT_SCHEMA,schema_of).is_err());
    let wrong=text.replace(E::DOCUMENT_SCHEMA,"stdio.semio.wrong-subset");
    assert!(semio_s_artifact_stdio_contract::parse_example_snapshot::<E::Snapshot>(source.id(),source.id(),&wrong,E::DOCUMENT_SCHEMA,schema_of).is_err());
}

#[test]
fn typed_example_audio(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_audio::editor::SemioAudioEditor>("audio",semio_s_artifact_stdio_semio_audio_model::schema::snapshot::demo_audio_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_animation(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_animation::editor::SemioAnimationEditor>("animation",semio_s_artifact_stdio_semio_animation_schema::schema::snapshot::demo_animation_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_brep(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_brep::editor::SemioBrepEditor>("brep",semio_s_artifact_stdio_semio_brep_snapshot::schema::snapshot::demo_brep_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_document(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_document::editor::SemioDocumentEditor>("document",semio_s_artifact_stdio_semio_document_snapshot::schema::snapshot::demo_semio_document_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_drawing(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_drawing::editor::SemioDrawingEditor>("drawing",semio_s_artifact_stdio_semio_drawing_snapshot::schema::snapshot::demo_drawing_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_value(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_value::editor::SemioValueEditor>("value",semio_s_artifact_stdio_semio_value_model::schema::snapshot::demo_semio_value_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_text(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_text::editor::SemioTextEditor>("text",semio_s_artifact_stdio_semio_text_model::schema::snapshot::demo_text_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_mesh(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_mesh::editor::SemioMeshEditor>("mesh",semio_s_artifact_stdio_semio_mesh_snapshot::schema::snapshot::demo_mesh_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_cad(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_cad::editor::SemioCadEditor>("cad",semio_s_artifact_stdio_semio_cad_snapshot::schema::snapshot::demo_cad_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_video(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_video::editor::SemioVideoEditor>("video",semio_s_artifact_stdio_semio_video_model::schema::snapshot::demo_video_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_image(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_image::editor::SemioImageEditor>("image",semio_s_artifact_stdio_semio_image_snapshot::schema::snapshot::demo_image_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_presentation(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_presentation::editor::SemioPresentationEditor>("presentation",semio_s_artifact_stdio_semio_presentation_snapshot::schema::snapshot::demo_semio_presentation_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_graph(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_graph::editor::SemioGraphEditor>("graph",semio_s_artifact_stdio_semio_graph_snapshot::schema::snapshot::demo_graph_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_flow(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_flow::editor::SemioFlowEditor>("flow",semio_s_artifact_stdio_semio_flow_snapshot::schema::snapshot::demo_flow_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_object(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_object::editor::SemioObjectEditor>("object",semio_s_artifact_stdio_semio_object_snapshot::schema::snapshot::demo_object_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_model(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_model::editor::SemioModelEditor>("model",semio_s_artifact_stdio_semio_model_snapshot::schema::snapshot::demo_semio_model_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_base(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_editor::editor::SemioAnyEditor>("*",semio_s_artifact_stdio_semio_snapshot::schema::snapshot::demo_semio_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_kit(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_kit::editor::SemioKitEditor>("kit",semio_s_artifact_stdio_semio_kit_snapshot::schema::snapshot::demo_kit_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

#[test]
fn typed_example_table(){
    let fixture:Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let examples:Value=serde_json::from_str(include_str!("🧫️fixtures/📚️examples.json")).unwrap();
    check_examples::<semio_s_artifact_stdio_semio_table::editor::SemioTableEditor>("table",semio_s_artifact_stdio_semio_table_model::schema::snapshot::demo_table_snapshot(),&fixture,&examples,|snapshot|snapshot.schema.as_str());
}

/// 📰️ The complete authored article preserves every run and mark through the current native file codec.
#[test]
fn typed_article_natural_file_preserves_all_authored_runs_and_marks(){
 use semio_framework_pack::snapshot::ArtifactDsl;
 type E=semio_s_artifact_stdio_semio_text::editor::SemioTextEditor;
 type Snapshot=semio_s_artifact_stdio_semio_text_model::schema::snapshot::SemioTextSnapshot;
 let root=std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
 let fixture=root.join("🏅️standards/🔖️v1/🪆️subsets/🔤️text/🧪️tests/🧫️fixtures/🔤️mutate-semio-text");
 let source=std::fs::read_to_string(fixture.join("🏚️zukunft-bau-entwerfen-mit-bestand/🗣️.dsl.semio")).unwrap();
 let input=Snapshot::parse_dsl(&source).expect("the complete current article parses through its actual typed snapshot");
 assert_eq!(input.schema.as_str(),E::DOCUMENT_SCHEMA);
 assert_eq!(input.runs.len(),384);assert_eq!(input.runs.iter().map(|run|run.marks.len()).sum::<usize>(),344);
 let independent:Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&input)).unwrap();
 assert_eq!(independent["schema"],E::DOCUMENT_SCHEMA);let runs=independent["runs"].as_array().unwrap();assert_eq!(runs.len(),384);
 assert_eq!(runs.iter().map(|run|run["marks"].as_array().map_or(0,Vec::len)).sum::<usize>(),344);
 let bytes=E::encode_natural_file(&input).expect("actual editor encodes the complete typed article");
 let restored=E::decode_natural_file(&bytes).expect("actual editor decodes the complete native article");
 assert_eq!(restored,input);assert_eq!(E::encode_natural_file(&restored).unwrap(),bytes);
 let observed:Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&restored)).unwrap();assert_eq!(observed,independent);
 let canonical=input.print_dsl();assert!(canonical.len()<=262_144&&bytes.len()<=262_144);
 assert_eq!(source,canonical,"the authored DSL pin equals its current typed print");
 assert_eq!(std::fs::read(fixture.join("🎒️.pack.semio")).unwrap(),bytes,"the complete article Pack pin equals its actual native encoding");
}
