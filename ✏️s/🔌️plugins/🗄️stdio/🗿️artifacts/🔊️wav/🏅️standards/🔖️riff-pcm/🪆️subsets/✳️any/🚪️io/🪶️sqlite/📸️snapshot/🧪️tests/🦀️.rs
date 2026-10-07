use crate::standards::riff_pcm::subsets::any::io::sqlite::snapshot::*;
use crate::standards::riff_pcm::subsets::any::schema::snapshot::*;
#[path="💰️backing/🦀️.rs"]
mod owned_requests;

fn owned_control_laws()->serde_json::Value{serde_json::from_str(include_str!("../🧫️fixtures/🎛️controls.json")).unwrap()}
fn owned_control_payload(snapshot:&WavSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io::IoPayload{match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io::IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io::IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot))}}

#[test]
fn sqlite_snapshot_wav_integer_query_samples_match_exact_native_words(){
 let cases:serde_json::Value=serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🧫️fixtures/🎯️integer-query.json"))).unwrap();
 for case in cases["cases"].as_array().unwrap(){let bits=u32::from_str_radix(case["binary32Bits"].as_str().unwrap(),16).unwrap();let snapshot=WavSnapshot{data:WavData::Float32(vec![f32::from_bits(bits)]),..fixture()};let mut database=snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let integer=case["integer"].as_str().unwrap().parse::<i64>().unwrap();database.table_mut("wav_float32_sample").unwrap().rows[0].values[5]=SqliteValue::Integer(integer);assert_eq!(WavSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).is_ok(),case["accept32"].as_bool().unwrap(),"WAV query INTEGER {integer}");}
}

#[test]
fn sqlite_snapshot_wav_both_controlled_native_directions_preserve_the_actual_parent(){
 use store::sqlite_snapshot::SnapshotEncoding;let expected=fixture();
 for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let payload=owned_control_payload(&expected,encoding);let actual=WavSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(actual,expected);let output=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).expect("actual WAV owner must admit controlled output");let actual=WavSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();assert_eq!(actual,expected);assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|false,Default::default())).is_err());}
}

#[test]
fn sqlite_snapshot_wav_authored_schema_is_admitted_before_native_ownership(){
 use store::sqlite_snapshot::SnapshotEncoding;let expected=fixture();
 for offset in owned_control_laws()["schemaLimitOffsets"].as_array().unwrap(){let maximum=WavSnapshot::SQLITE_SCHEMA.len().checked_add_signed(offset.as_i64().unwrap()as isize).unwrap();let limits=SqliteDatabaseLimits{max_schema_bytes:maximum,..Default::default()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=owned_control_payload(&expected,encoding);let mut work=false;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{work|=event.completed>0;true};let result=WavSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut progress,limits));assert_eq!(result.is_ok(),offset==&serde_json::json!(0),"{encoding:?} input schema offset {offset}");if maximum<WavSnapshot::SQLITE_SCHEMA.len(){assert!(!work)}let mut work=false;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|{work|=event.completed>0;true};let result=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut progress,limits));assert_eq!(result.is_ok(),maximum==WavSnapshot::SQLITE_SCHEMA.len(),"{encoding:?} output schema offset {offset}");if maximum<WavSnapshot::SQLITE_SCHEMA.len(){assert!(!work)}}}
}

#[test]
fn sqlite_snapshot_wav_exact_native_row_forecast_matches_independent_sqlite(){
 use std::io::Write;use store::sqlite_snapshot::SnapshotEncoding;let expected=fixture();let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let bytes=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const d=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));let n=0;for(const r of d.query(\"SELECT name FROM sqlite_schema WHERE type='table'\").all())n+=d.query('SELECT COUNT(*) AS n FROM \"'+r.name+'\"').get().n;await Bun.write(Bun.stdout,String(n));d.close();";let mut child=std::process::Command::new("bun").args(["-e",script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let count=String::from_utf8(output.stdout).unwrap().parse::<usize>().unwrap();assert_eq!(count,owned_control_laws()["exactFixtureRows"].as_u64().unwrap()as usize);
 for offset in owned_control_laws()["rowLimitOffsets"].as_array().unwrap(){let maximum=count.checked_add_signed(offset.as_i64().unwrap()as isize).unwrap();let limits=SqliteDatabaseLimits{max_rows:maximum,..Default::default()};for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=owned_control_payload(&expected,encoding);let decoded=WavSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(decoded.is_ok(),maximum==count,"{encoding:?} input row bound {maximum}");if let Err(error)=decoded{assert_eq!(error.kind.as_str(),owned_control_laws()["rowRefusalKind"].as_str().unwrap(),"{encoding:?} input row origin");}let encoded=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(encoded.is_ok(),maximum==count,"{encoding:?} output row bound {maximum}");if let Err(error)=encoded{assert_eq!(error.kind.as_str(),owned_control_laws()["rowRefusalKind"].as_str().unwrap(),"{encoding:?} output row origin");}}}
}

#[test]
fn sqlite_snapshot_wav_large_owned_text_cancels_inside_all_four_phases(){
 use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotPhase};let laws=owned_control_laws();let mut expected=fixture();expected.schema="😀".repeat(laws["largeCharacters"].as_u64().unwrap()as usize);let database=expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let input=owned_control_payload(&expected,SnapshotEncoding::Text);let boundary=laws["checkpointBytes"].as_u64().unwrap()as usize;
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut reached=false;let mut progress=|event:store::sqlite_snapshot::SqliteSnapshotProgress|if event.phase==phase&&event.total>boundary&&event.completed>=boundary&&event.completed<event.total{reached=true;false}else{true};let mut control=SqliteSnapshotControl::new(&mut progress,Default::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>expected.to_sqlite_database(&mut control).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>WavSnapshot::from_sqlite_database(&database,&mut control).map(|_|()),SqliteSnapshotPhase::DecodeNative=>WavSnapshot::decode_sqlite_snapshot_native(&input,&mut control).map(|_|()),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut control).map(|_|()),_=>unreachable!()};assert!(result.is_err(),"{phase:?}");drop(control);assert!(reached,"{phase:?} must reach the interior owned text copy");}
 let limits=SqliteDatabaseLimits{max_value_bytes:laws["maximumOwnedBytes"].as_u64().unwrap()as usize,..Default::default()};assert!(expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(WavSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let input=owned_control_payload(&expected,encoding);assert!(WavSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
}
#[test]
fn sqlite_snapshot_wav_manual_metadata_preserves_neutral_fields_and_enforces_cumulative_admission(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🏭️schema/🔣️.json")).unwrap();let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;let tiny=fixture["tinyBytes"].as_u64().unwrap()as usize;
 for(index,producer)in[{let semio_framework_dsl_record::Shape::Record(record)=<WavData as semio_framework_dsl_record::DslField>::shape()else{panic!("WavData record")};record},{let semio_framework_dsl_record::Shape::Record(record)=<WavChunkRef as semio_framework_dsl_record::DslField>::shape()else{panic!("WavChunkRef record")};record}].iter().enumerate(){
  let mut admitted=|_|true;let mut encoding=semio_framework_value::NativeEncodeControl::new(maximum,&mut admitted);let encoded=producer.encode(&mut encoding).unwrap();let exact=encoding.owned_bytes();assert!(exact>0);
  let mut admitted=|_|true;let mut decoding=semio_framework_value::NativeDecodeControl::new(maximum,&mut admitted);let decoded=producer.decode(&mut decoding).unwrap();
  for record in[encoded,decoded]{let fields:Vec<_>=record.fields.iter().map(|field|serde_json::json!([field.id,field.key,field.optional])).collect();assert_eq!(serde_json::json!(fields),fixture["records"][index]["fields"]);let semio_framework_dsl_record::Shape::Enum(labels)=&record.fields[0].shape else{panic!("declared enum")};assert_eq!(serde_json::json!(labels),fixture["records"][index]["tags"]);}
  assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(exact,&mut |_|true)).is_ok());assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(exact-1,&mut |_|true)).is_err());
  let mut admitted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(tiny,&mut admitted);assert!(producer.encode(&mut control).is_err());assert_eq!(control.owned_bytes(),0);assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(tiny,&mut |_|true)).is_err());assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(maximum,&mut |_|false)).is_err());assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(maximum,&mut |_|false)).is_err());
 }
}
#[test]
fn sqlite_snapshot_wav_controlled_native_owner_preserves_full_fixture_and_enforces_caller_limits(){
 use semio_framework_os_kernel::{io::IoPayload,sqlite_snapshot::{SqliteDatabaseLimits,SqliteSnapshotControl},ArtifactSqliteSnapshot};
 let snapshot=fixture();let limits=SqliteDatabaseLimits::default();
 for payload in[IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)),IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot))]{
  assert_eq!(WavSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).unwrap(),snapshot);
  assert!(WavSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|false,limits)).is_err());
  assert!(WavSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_value_bytes:1,..limits})).is_err());
  assert!(WavSnapshot::decode_sqlite_snapshot_native(&payload,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:1,..limits})).is_err());
 }
}
#[test]
fn sqlite_snapshot_wav_canonical_json_preserves_raw_sample_words_and_full_chunk_indices() {
    let words: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();
    let bits: Vec<u32> = words["ieee754Binary32Bits"].as_array().unwrap().iter().map(|word| word.as_u64().unwrap() as u32).collect();
    let snapshot = WavSnapshot { data: WavData::Float32(bits.iter().copied().map(f32::from_bits).collect()), chunk_order: vec![WavChunkRef::Other(u64::MAX)], ..fixture() };
    let text = semio_framework_pack_json::to_json_string(&snapshot);
    let independent: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(independent["data"]["value"], serde_json::Value::Array(bits.iter().map(|word| serde_json::json!({"bits":word})).collect()));
    assert_eq!(independent["chunkOrder"][0]["value"], u64::MAX.to_string());
    let restored: WavSnapshot = semio_framework_pack_json::from_json_str(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let WavData::Float32(samples) = restored.data else { panic!("float32 kind"); };
    assert_eq!(samples.iter().map(|sample| sample.to_bits()).collect::<Vec<_>>(), bits);
    for invalid in [r#"{"kind":"other","value":0}"#, r#"{"kind":"other","value":"00"}"#, r#"{"kind":"other","value":"+1"}"#, r#"{"kind":"other","value":"18446744073709551616"}"#, r#"{"kind":"format","value":"0"}"#] {
        assert!(semio_framework_pack_json::from_json_str::<WavChunkRef>(invalid, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
    for invalid in [r#"{"kind":"float32","value":[0.0]}"#, r#"{"kind":"float32","value":[{"bits":4294967296}]}"#, r#"{"kind":"float32","value":[{"bits":0,"extra":0}]}"#] {
        assert!(semio_framework_pack_json::from_json_str::<WavData>(invalid, semio_framework_pack_json::JsonMemberPolicy::Reject).is_err());
    }
}

#[test]
fn sqlite_snapshot_wav_authored_grammar_and_protocol_admit_complete_logical_records() {
    let grammar = semio_framework_dsl::parse_grammar(include_str!("../../../📝️text/📸️snapshot/📖️.grammar.semio")).unwrap();
    let recognizer = semio_framework_dsl::Recognizer::compile(&grammar, &semio_framework_os_kernel::os_dsl::grammar::family_fragments().expect("OS family grammar"), semio_framework_os_kernel::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
    for snapshot in [fixture(), WavSnapshot::default(), crate::standards::riff_pcm::subsets::any::io::decode_wav(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav")).unwrap()] {
        let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
        assert!(recognizer.recognize(body).unwrap(), "{body}");
    }
    let protocol = semio_framework_dsl::parse_protocol(include_str!("../../../💾️binary/📸️snapshot/📡️.protocol.semio")).unwrap();
    assert_eq!(protocol.schema, "stdio.wav");
    assert_eq!(protocol.version, 1);
}

#[test]
fn sqlite_snapshot_wav_shipped_demo_assets_match_owned_native_source() {
    let snapshot = crate::standards::riff_pcm::subsets::any::io::decode_wav(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav")).unwrap();
    assert_eq!(<WavSnapshot as store::ArtifactDsl>::parse_dsl(include_str!("../../../../📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio")).unwrap(), snapshot);
    assert_eq!(<WavSnapshot as store::ArtifactPack>::decode_pack(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎒️.pack.semio")).unwrap(), snapshot);
}

use semio_framework_os_kernel::{
    sqlite_snapshot::{export_sqlite_database, import_sqlite_database, SqliteDatabaseLimits, SqliteSnapshotControl, SqliteValue},
    ArtifactSqliteSnapshot,
};

fn fixture() -> WavSnapshot {
    semio_framework_pack_json::from_json_str(include_str!("../🧫️fixtures/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap()
}
#[test]
fn sqlite_snapshot_wav_complete_unsigned64_chunk_indices_survive_owned_relationships() {
    let indices: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🧭️indices.json")).unwrap();
    let mut snapshot = fixture();
    snapshot.chunk_order = indices["unsigned64ChunkIndices"].as_array().unwrap().iter().map(|value| WavChunkRef::Other(value.as_str().unwrap().parse().unwrap())).collect();
    assert_eq!(roundtrip(&snapshot), snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_wav_actual_erased_capability_retains_all_owned_sample_states() {
    use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding};
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();
    let floats = patterns["ieee754Binary32Bits"].as_array().unwrap().iter().map(|value| f32::from_bits(value.as_u64().unwrap() as u32)).collect();
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("Wav erased SQLite")
        .version("0.0.1")
        .package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap())
        .try_build()
        .unwrap();
    let codec = store::document_codec(crate::STDIO_WAV_DOCUMENT_SCHEMA).await.unwrap().unwrap();
    let provider = codec.snapshot_sqlite.as_ref().unwrap();
    let dialect: ArtifactDialect = crate::WAV_DIALECT.into();
    for data in [WavData::Pcm16(vec![i16::MIN, i16::MAX]), WavData::Pcm8(vec![0, 255]), WavData::Raw(vec![0, 255]), WavData::Float32(floats)] {
        let snapshot = WavSnapshot {
            schema: "WAV 世界\0".into(),
            fmt: WavFmt { ext: Some(Vec::new()), ..fixture().fmt },
            data,
            fmt_pad_byte: 255,
            data_pad_byte: 255,
            other_chunks: vec![RiffChunk { fourcc: "自由 🎶".into(), data: Vec::new(), pad_byte: 255 }],
            chunk_order: vec![WavChunkRef::Format, WavChunkRef::Samples, WavChunkRef::Other(0), WavChunkRef::Other(u64::MAX)],
        };
        let expected = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let payload = match encoding {
                SnapshotEncoding::Binary => IoPayload::Binary(<WavSnapshot as store::ArtifactPack>::encode_pack_with(&snapshot, &store::PackEncodeOptions::default()).expect("complete owned snapshot encoding")),
                SnapshotEncoding::Text => IoPayload::Text(<WavSnapshot as store::ArtifactDsl>::print_dsl(&snapshot)),
            };
            let database = (provider.export)(&codec.schema, &dialect, &payload, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database, expected);
            let restored = (provider.import)(&codec.schema, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let restored = match restored {
                IoPayload::Binary(bytes) => <WavSnapshot as store::ArtifactPack>::decode_pack(&bytes).unwrap(),
                IoPayload::Text(text) => <WavSnapshot as store::ArtifactDsl>::parse_dsl(&text).unwrap(),
            };
            assert_eq!(restored.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), expected);
        }
    }
}

#[test]
fn sqlite_snapshot_wav_owned_encoding_matches_exact_declared_file_bounds() {
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let snapshot = fixture();
        let expected=owned_control_payload(&snapshot,encoding);let length=match &expected{store::io::IoPayload::Binary(bytes)=>bytes.len(),store::io::IoPayload::Text(text)=>text.len()};
        let exact=snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length,..Default::default()})).unwrap();
        match(expected,exact){(store::io::IoPayload::Binary(expected),store::io::IoPayload::Binary(actual))=>assert_eq!(actual,expected),(store::io::IoPayload::Text(expected),store::io::IoPayload::Text(actual))=>assert_eq!(actual,expected),_=>panic!("declared encoding must agree")};
        assert!(snapshot.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,SqliteDatabaseLimits{max_file_bytes:length-1,..Default::default()})).is_err());
        let snapshot = WavSnapshot { data: WavData::Raw(vec![255; 131073]), ..snapshot };
        let mut reached = false;
        assert!(snapshot
            .encode_sqlite_snapshot_native(
                encoding,
                &mut SqliteSnapshotControl::new(
                    &mut |event| {
                        if event.phase == SqliteSnapshotPhase::EncodeNative && event.completed > 0 {
                            reached = true;
                            false
                        } else {
                            true
                        }
                    },
                    SqliteDatabaseLimits::default()
                )
            )
            .is_err());
        assert!(reached);
    }
}
fn roundtrip(value: &WavSnapshot) -> WavSnapshot {
    let database = value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let database = import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    WavSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap()
}

#[test]
fn sqlite_snapshot_wav_semantic_fields_and_all_sample_kinds() {
    let snapshot = fixture();
    assert_eq!(roundtrip(&snapshot), snapshot);
    let oracle: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&semio_framework_value::ToValue::to_value(&snapshot))).unwrap();
    assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../🧫️fixtures/🔣️.json")).unwrap());
    for data in [WavData::Pcm16(vec![i16::MIN, i16::MAX]), WavData::Pcm8(vec![0, 255]), WavData::Raw(vec![0, 255, 17])] {
        let value = WavSnapshot {
            schema: "自由 世界".into(),
            data,
            fmt: WavFmt { ext: Some(Vec::new()), ..snapshot.fmt.clone() },
            fmt_pad_byte: 255,
            data_pad_byte: 255,
            other_chunks: vec![RiffChunk { fourcc: "自由 🎶".into(), data: Vec::new(), pad_byte: 255 }],
            chunk_order: Vec::new(),
        };
        assert_eq!(roundtrip(&value), value);
    }
    for ext in [None, Some(Vec::new())] {
        let value = WavSnapshot { fmt: WavFmt { ext, ..WavFmt::default() }, ..WavSnapshot::default() };
        assert_eq!(roundtrip(&value), value);
    }
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();
    let patterns: Vec<u32> = patterns["ieee754Binary32Bits"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
    let value = WavSnapshot { data: WavData::Float32(patterns.iter().copied().map(f32::from_bits).collect()), ..snapshot.clone() };
    let WavData::Float32(restored) = roundtrip(&value).data else { panic!("float32 kind lost") };
    assert_eq!(restored.iter().map(|value| value.to_bits()).collect::<Vec<_>>(), patterns);
    let mut database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    database.table_mut("wav_pcm16_sample").unwrap().rows[0].values[1] = SqliteValue::Integer(9);
    assert!(WavSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_rows: 1, ..SqliteDatabaseLimits::default() })).is_err());
}

#[test]
fn sqlite_snapshot_wav_independent_sqlite_interprets_and_edits_semantic_entities() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let mut expected = fixture();
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT o.ordinal,c.fourcc FROM wav_chunk_order o JOIN wav_other_chunk c ON c.id=o.other_chunk_id ORDER BY o.ordinal').all();if(rows.length!==3||rows[0].fourcc!=='LIST'||rows[1].fourcc!=='fmt ')throw Error('relationships');db.query('UPDATE wav_pcm16_sample SET sample=42 WHERE ordinal=0').run();db.query('UPDATE wav_format SET channels=3').run();db.query('UPDATE wav_other_chunk_byte SET octet=99 WHERE chunk_id=1 AND ordinal=1').run();await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let database = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let WavData::Pcm16(samples) = &mut expected.data else { panic!() };
    samples[0] = 42;
    expected.fmt.channels = 3;
    expected.other_chunks[0].data[1] = 99;
    assert_eq!(WavSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap(), expected);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_wav_actual_declaration_preserves_owned_state_over_io() {
    use {semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot,semio_framework_artifact_reference::ArtifactDialect};
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    semio_framework_plugin::Plugin::<semio_framework_plugin::app::NoPluginApp>::builder("stdio")
        .label("WAV SQLite declaration")
        .version("0.0.1")
        .package_id("semio:stdio")
        .artifact(crate::declaration(crate::definition().unwrap()).unwrap())
        .try_build()
        .unwrap();
    let dialect: ArtifactDialect = crate::WAV_DIALECT.into();
    let mut snapshot = fixture();
    snapshot.schema = "Full Snapshot 世界".into();
    snapshot.fmt.ext = Some(Vec::new());
    snapshot.fmt_pad_byte = 255;
    let mut phases = Vec::new();
    let bytes = io_export_sqlite_snapshot(&dialect, &snapshot, SnapshotEncoding::Binary, SqliteDatabaseLimits::default(), &mut |event| {
        phases.push(event.phase);
        true
    })
    .await
    .unwrap()
    .value;
    assert_eq!(io_import_sqlite_snapshot::<WavSnapshot>(&dialect, &bytes, SqliteDatabaseLimits::default(), &mut |_| true).await.unwrap().value, snapshot);
    assert!(!phases.iter().any(|phase| matches!(phase, SqliteSnapshotPhase::EncodeNative | SqliteSnapshotPhase::DecodeNative)));
}

#[test]
fn sqlite_snapshot_wav_ieee754_samples_survive_independent_sqlite_and_detect_inconsistent_edits() {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();
    let patterns: Vec<u32> = patterns["ieee754Binary32Bits"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u32).collect();
    let snapshot = WavSnapshot { data: WavData::Float32(patterns.iter().copied().map(f32::from_bits).collect()), ..WavSnapshot::default() };
    let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const classes=db.query('SELECT numeric_class,COUNT(*) AS n FROM wav_float32_sample GROUP BY numeric_class').all();if(classes.find(r=>r.numeric_class==='nan').n!==3||classes.find(r=>r.numeric_class==='negative_zero').n!==1)throw Error('numeric class');if(db.query('SELECT ieee754_binary32_bits,sample FROM wav_float32_sample WHERE ordinal=10').get().ieee754_binary32_bits!==2139095041)throw Error('signaling NaN lost');await Bun.write(Bun.stdout,db.serialize());db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let database = import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let WavData::Float32(restored) = WavSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().data else { panic!() };
    assert_eq!(restored.iter().map(|value| value.to_bits()).collect::<Vec<_>>(), patterns);
    for (column, value) in [(3, SqliteValue::Integer(1065353216)), (4, SqliteValue::Text("nan".into())), (5, SqliteValue::Real(42.0))] {
        let mut invalid = database.clone();
        invalid.table_mut("wav_float32_sample").unwrap().rows[0].values[column] = value;
        assert!(WavSnapshot::from_sqlite_database(&invalid, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).is_err());
    }
}

#[test]
fn sqlite_snapshot_wav_data_equality_preserves_every_literal_ieee_word(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();
 let words=fixture["ieee754Binary32Bits"].as_array().unwrap();
 for left in words{for right in words{
  let left=left.as_u64().unwrap()as u32;let right=right.as_u64().unwrap()as u32;
  let a=WavData::Float32(vec![f32::from_bits(left)]);let b=WavData::Float32(vec![f32::from_bits(right)]);
  assert_eq!(a==b,left==right);assert_eq!(semio_framework_value::ToValue::to_value(&a)==semio_framework_value::ToValue::to_value(&b),left==right);
 }}
 assert_ne!(WavData::Pcm8(vec![0]),WavData::Raw(vec![0]));
 assert_ne!(WavData::Float32(vec![0.0]),WavData::Float32(vec![0.0,0.0]));
}

#[test]
fn sqlite_snapshot_wav_required_padding_defaults_match_the_closed_neutral_contract() {
    let laws: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🎚️presence/🔣️.json")).unwrap();
    for role in laws["cases"].as_array().unwrap() {
        let mut wire: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
        for field in ["fmtPadByte", "dataPadByte"] {
            if role["present"].as_bool().unwrap() {
                wire.as_object_mut().unwrap().insert(field.into(), role["value"].clone());
            } else {
                wire.as_object_mut().unwrap().remove(field);
            }
        }
        for chunk in wire["otherChunks"].as_array_mut().unwrap() {
            if role["present"].as_bool().unwrap() {
                chunk.as_object_mut().unwrap().insert("padByte".into(), role["value"].clone());
            } else {
                chunk.as_object_mut().unwrap().remove("padByte");
            }
        }
        let snapshot: WavSnapshot = semio_framework_pack_json::from_json_str(&wire.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        assert_eq!(u64::from(snapshot.fmt_pad_byte), laws["canonicalPads"]["format"].as_u64().unwrap());
        assert_eq!(u64::from(snapshot.data_pad_byte), laws["canonicalPads"]["samples"].as_u64().unwrap());
        assert!(snapshot.other_chunks.iter().all(|chunk| u64::from(chunk.pad_byte) == laws["canonicalPads"]["auxiliary"].as_u64().unwrap()));
        let database = snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
        assert_eq!(database.table("wav_format").unwrap().rows[0].values[7], SqliteValue::Integer(0));
        assert_eq!(database.table("wav_data").unwrap().rows[0].values[2], SqliteValue::Integer(0));
        assert!(database.table("wav_other_chunk").unwrap().rows.iter().all(|row| row.values[4] == SqliteValue::Integer(0)));
        let file = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
        let restored_database = import_sqlite_database(&file, Default::default(), &mut |_| true).unwrap();
        assert_eq!(WavSnapshot::from_sqlite_database(&restored_database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap(), snapshot);
    }
}

#[test]
fn sqlite_snapshot_wav_complete_cells_have_independent_pretyped_native_admission(){
 use store::sqlite_snapshot::SnapshotEncoding;use std::{io::Write,process::{Command,Stdio}};
 let plan:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/📏️cells/🔣️.json")).unwrap();
 let words:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔢️float32.json")).unwrap();let indices:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️indices.json")).unwrap();
 for extension in plan["extensionStates"].as_array().unwrap(){
  for kind in plan["sampleKinds"].as_array().unwrap(){
   let mut input=fixture();input.schema="WAV 世界\0".into();input.fmt.ext=match extension.as_str().unwrap(){"absent"=>None,"empty"=>Some(Vec::new()),"present"=>input.fmt.ext.take(),_=>panic!("authored extension state")};
   input.data=match kind.as_str().unwrap(){"pcm16"=>WavData::Pcm16(vec![i16::MIN,-1,0,1,i16::MAX]),"pcm8"=>WavData::Pcm8(vec![0,255,127]),"float32"=>WavData::Float32(words["ieee754Binary32Bits"].as_array().unwrap().iter().map(|word|f32::from_bits(word.as_u64().unwrap()as u32)).collect()),"raw"=>WavData::Raw(vec![0,255,127]),_=>panic!("authored sample kind")};
   input.chunk_order=vec![WavChunkRef::Format,WavChunkRef::Samples];input.chunk_order.extend(indices["unsigned64ChunkIndices"].as_array().unwrap().iter().map(|index|WavChunkRef::Other(index.as_str().unwrap().parse().unwrap())));
   let database=input.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();let bytes=export_sqlite_database(&database,Default::default(),&mut |_|true).unwrap();
   let mut child=Command::new("bun").args(["-e",r####"import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});try{if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const tables=db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all().map(({name})=>{const columns=db.query('PRAGMA table_info('+name+')').all(),expression=columns.map(({name})=>{const c='"'+name.replaceAll('"','""')+'"';return "CASE typeof("+c+") WHEN 'integer' THEN 8 WHEN 'real' THEN 8 WHEN 'text' THEN length(CAST("+c+" AS BLOB)) WHEN 'blob' THEN length("+c+") WHEN 'null' THEN 0 ELSE NULL END"}).join('+'),r=db.query('SELECT COUNT(*) AS rows,COALESCE(SUM('+expression+'),0) AS bytes FROM '+name).get();return{table:name,columns:columns.length,rows:Number(r.rows),semanticBytes:Number(r.bytes)}});await Bun.write(Bun.stdout,JSON.stringify(tables));}finally{db.close()}"####]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let tables:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();let tables=tables.as_array().unwrap();
   assert_eq!(serde_json::json!(tables.iter().map(|t|serde_json::json!([t["table"],t["columns"]])).collect::<Vec<_>>()),plan["tableWidths"]);
   let rows=tables.iter().map(|t|t["rows"].as_u64().unwrap()as usize).sum::<usize>();let cells=tables.iter().map(|t|t["semanticBytes"].as_u64().unwrap()as usize).sum::<usize>();let exact=SqliteDatabaseLimits{max_rows:rows,max_value_bytes:cells,..Default::default()};
   assert_eq!(input.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),database);
   for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{
    let expected=owned_control_payload(&input,encoding);assert_eq!(input.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap(),expected);
    let actual=WavSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,exact)).unwrap();assert_eq!(actual,input);actual.retire_sqlite_snapshot();
    for short in[SqliteDatabaseLimits{max_rows:rows-1,..exact},SqliteDatabaseLimits{max_value_bytes:cells-1,..exact}]{
     assert!(input.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"WAV full semantic encode {kind} {extension} {encoding:?} {short:?}");
     assert!(WavSnapshot::decode_sqlite_snapshot_native(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err(),"WAV full semantic decode {kind} {extension} {encoding:?}");
    }
   }
   for short in[SqliteDatabaseLimits{max_rows:rows-1,..exact},SqliteDatabaseLimits{max_value_bytes:cells-1,..exact}]{assert!(input.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());assert!(WavSnapshot::from_sqlite_database(&database,&mut SqliteSnapshotControl::new(&mut |_|true,short)).is_err());}
  }
 }
}

#[test]
fn sqlite_snapshot_wav_borrowed_carriers_match_the_neutral_manual_records(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🏭️schema/🔣️.json")).unwrap();
 for(index,record)in[<WavData as semio_framework_dsl_record::BorrowedDslRecord>::RECORD,<WavChunkRef as semio_framework_dsl_record::BorrowedDslRecord>::RECORD].iter().enumerate(){let fields:Vec<_>=record.fields.iter().map(|field|serde_json::json!([field.id,field.key,field.optional])).collect();assert_eq!(serde_json::json!(fields),fixture["records"][index]["fields"]);let semio_framework_dsl_record::BorrowedShape::Enum(labels)=record.fields[0].shape else{panic!("WAV tagged record")};assert_eq!(serde_json::json!(labels),fixture["records"][index]["tags"]);}
}
