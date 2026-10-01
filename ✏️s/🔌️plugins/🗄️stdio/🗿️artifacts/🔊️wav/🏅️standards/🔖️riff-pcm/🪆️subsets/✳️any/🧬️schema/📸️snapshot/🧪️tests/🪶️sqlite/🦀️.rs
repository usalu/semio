use super::*;
#[test]
fn sqlite_snapshot_wav_authored_grammar_and_protocol_admit_complete_logical_records() {
    let grammar = dsl::parse_grammar(include_str!("../../📝️text/📖️.grammar.semio")).unwrap();
    let recognizer = dsl::Recognizer::compile(&grammar);
    for snapshot in [fixture(), WavSnapshot::default(), crate::standards::riff_pcm::subsets::any::io::decode_wav(include_bytes!("../../../../📚️examples/🎬️demo/🖼️assets/🎧️example/🔊️.wav")).unwrap()] {
        let text = <WavSnapshot as store::ArtifactDsl>::print_dsl(&snapshot);
        let (_, body) = store::semio_format::split_text_preamble(&text).unwrap();
        assert!(recognizer.recognize(body).unwrap(), "{body}");
    }
    let protocol = dsl::parse_protocol(include_str!("../../💾️binary/📡️.protocol.semio")).unwrap();
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
    let mut value:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();for reference in value["chunkOrder"].as_array_mut().unwrap(){if reference["kind"]=="other"{reference["value"]=serde_json::Value::from(reference["value"].as_str().unwrap().parse::<u64>().unwrap());}}pack::json::from_json_str(&value.to_string()).unwrap()
}
#[test]
fn sqlite_snapshot_wav_complete_unsigned64_chunk_indices_survive_owned_relationships() {
    let indices: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🧭️indices.json")).unwrap();
    let mut snapshot = fixture();
    snapshot.chunk_order = indices["unsigned64ChunkIndices"].as_array().unwrap().iter().map(|value| WavChunkRef::Other(value.as_str().unwrap().parse().unwrap())).collect();
    assert_eq!(roundtrip(&snapshot), snapshot);
}

#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_wav_actual_erased_capability_retains_all_owned_sample_states() {
    use semio_framework_os_kernel::{
        io::{ArtifactDialect, IoPayload},
        sqlite_snapshot::SnapshotEncoding,
    };
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️float32.json")).unwrap();
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
fn sqlite_snapshot_wav_owned_encoding_preflight_checks_bounds_before_allocation() {
    use semio_framework_os_kernel::sqlite_snapshot::{SnapshotEncoding, SqliteSnapshotPhase};
    for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
        let snapshot = fixture();
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        assert!(snapshot
            .preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_file_bytes: 1024, ..SqliteDatabaseLimits::default() }))
            .unwrap_err()
            .contains("native encoding exceeds file byte limit"));
        let snapshot = WavSnapshot { data: WavData::Raw(vec![255; 131073]), ..snapshot };
        let mut reached = false;
        assert!(snapshot
            .preflight_sqlite_snapshot_encoding(
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
    let mut oracle: serde_json::Value = serde_json::from_str(&pack::json::to_json_string(&protocol::ToValue::to_value(&snapshot))).unwrap();for reference in oracle["chunkOrder"].as_array_mut().unwrap(){if reference["kind"]=="other"{reference["value"]=serde_json::Value::String(reference["value"].as_u64().unwrap().to_string());}}
    assert_eq!(oracle, serde_json::from_str::<serde_json::Value>(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap());
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
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️float32.json")).unwrap();
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
    use semio_framework_os_kernel::io::{
        io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot},
        ArtifactDialect,
    };
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
    let patterns: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔢️float32.json")).unwrap();
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
