//! 🪶️ Every authored Remodeling entity and exact scalar against independent SQLite.
use super::*;

#[test]
fn sqlite_snapshot_remodeling_authored_schema_limit_precedes_native_ownership(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::*;let expected=fixture();for maximum in[SCHEMA.len(),SCHEMA.len()-1]{let limits=SqliteDatabaseLimits{max_schema_bytes:maximum,..Default::default()};for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&expected,encoding);let mut work=false;let mut progress=|event:SqliteSnapshotProgress|{work|=event.completed>0;true};let result=expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut progress,limits));assert_eq!(result.is_ok(),maximum==SCHEMA.len());if maximum<SCHEMA.len(){assert!(!work)}let mut work=false;let mut progress=|event:SqliteSnapshotProgress|{work|=event.completed>0;true};let result=super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut progress,limits));assert_eq!(result.is_ok(),maximum==SCHEMA.len());if maximum<SCHEMA.len(){assert!(!work)}}}
}
#[test]
fn sqlite_snapshot_remodeling_erased_binary_and_text_capability_preserves_exact_typed_parent(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::*;let codec=store::ArtifactCodec::bare::<super::super::RemodelingSnapshot,crate::RemodelingMutation>(crate::REMODELING_DOCUMENT_SCHEMA);let capability=codec.snapshot_sqlite.expect("owning factory must expose the authored parent");let expected=fixture();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.remodel.remodeling".into(),standard:"1".into(),subset:"*".into()};
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&expected,encoding);let d=(capability.export)(crate::REMODELING_DOCUMENT_SCHEMA,&dialect,&input,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;let output=(capability.import)(crate::REMODELING_DOCUMENT_SCHEMA,&dialect,d,encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap().value;let actual=super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&output,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap();same(&actual,&expected);}
}
use semio_framework_plugin::{PluginApp,VcsArtifactApp,EditorApp,__semio_dispatch_PluginApp,plugin_app_close_prelude::*};
semio_framework_dispatch_macros::dyn_enum_close!{
 /// 📸️ The actual Remodeling declared VCS editor closure.
 enum SqliteApps:PluginApp{Editor(VcsArtifactApp<EditorApp<crate::editor::remodeling::RemodelingPlayApp>>),Viewer(VcsArtifactApp<ViewerApp<crate::viewer::remodeling::RemodelingViewer>>)}
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_remodeling_actual_declaration_routes_queryable_files_over_io(){
 use store::sqlite_snapshot::*;semio_framework_plugin::Plugin::<SqliteApps>::builder("remodel").label("Remodel owned SQLite").version("0.0.1").package_id("semio:remodel").declare_artifact(crate::artifact::<SqliteApps>()).try_build().unwrap();let codec=store::document_codec(crate::REMODELING_DOCUMENT_SCHEMA).await.unwrap().unwrap();assert!(codec.snapshot_sqlite.is_some());let expected=crate::default_remodeling_scene();let dialect=store::io_schema::ArtifactDialect{artifact_kind:"s.remodel.remodeling".into(),standard:"1".into(),subset:"*".into()};
 for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let bytes=store::io::io_mechanism::io_export_sqlite_snapshot(&dialect,&expected,encoding,Default::default(),&mut |_|true).await.unwrap().value;assert!(bytes.starts_with(b"SQLite format 3\0"));let actual=store::io::io_mechanism::io_import_sqlite_snapshot::<super::super::RemodelingSnapshot>(&dialect,&bytes,Default::default(),&mut |_|true).await.unwrap().value;same(&actual,&expected);}
}


#[test]
fn sqlite_snapshot_remodeling_independent_sql_edits_refuse_unowned_and_inconsistent_fields(){
 use store::sqlite_snapshot::*;let d=database(&fixture());let input=serde_json::json!({"bytes":export_sqlite_database(&d,Default::default(),&mut |_|true).unwrap(),"sql":laws()["malformedSql"]});
 let script=r#"import{Database}from'bun:sqlite';const v=JSON.parse(await Bun.stdin.text());const result=[];for(const sql of v.sql){const db=Database.deserialize(new Uint8Array(v.bytes));db.run('PRAGMA foreign_keys=OFF');db.run('PRAGMA ignore_check_constraints=ON');db.run(sql);result.push(Array.from(db.serialize()));db.close()}await Bun.write(Bun.stdout,JSON.stringify(result));"#;
 let bytes:Vec<Vec<u8>>=serde_json::from_slice(&oracle(script,input.to_string().as_bytes())).unwrap();assert_eq!(bytes.len(),laws()["malformedSql"].as_array().unwrap().len());
 for(sql,bytes)in laws()["malformedSql"].as_array().unwrap().iter().zip(bytes){let result=import_sqlite_database(&bytes,Default::default(),&mut |_|true).and_then(|d|restore(&d,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())));assert!(result.is_err(),"{sql}");}
}
#[test]
fn sqlite_snapshot_remodeling_scalar_integer_queries_do_not_round_into_binary64_words(){
 let original=database(&fixture());for case in laws()["integerQueries"].as_array().unwrap(){let query=case[0].as_str().unwrap().parse::<i64>().unwrap();let bits=u64::from_str_radix(case[1].as_str().unwrap(),16).unwrap();let mut d=original.clone();let row=&mut d.table_mut("remodel_stream").unwrap().rows[0];row.values[7]=SqliteValue::Integer(query);row.values[8]=SqliteValue::Integer(bits as i64);row.values[9]=SqliteValue::Text("finite".into());let result=restore(&d,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default()));assert_eq!(result.is_ok(),case[2].as_bool().unwrap(),"{query}");if let Ok(actual)=result{assert_eq!(actual.streams[0].sync_offset_ms.to_bits(),bits);}}
}
fn payload(s:&super::super::RemodelingSnapshot,encoding:store::sqlite_snapshot::SnapshotEncoding)->store::io_schema::IoPayload{use store::ArtifactSqliteSnapshot;s.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap()}
#[test]
fn sqlite_snapshot_remodeling_both_native_directions_admit_exact_rows_before_owned_values(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::*;let expected=fixture();let d=database(&expected);let count=d.tables.iter().map(|t|t.rows.len()).sum::<usize>();
 for maximum in[count,count-1]{let limits=SqliteDatabaseLimits{max_rows:maximum,..Default::default()};assert_eq!(project(&expected,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),maximum==count);for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&expected,encoding);let actual=super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits));assert_eq!(actual.is_ok(),maximum==count,"{encoding:?} input");if let Ok(actual)=actual{same(&actual,&expected)}assert_eq!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_ok(),maximum==count,"{encoding:?} output");}}
}
#[test]
fn sqlite_snapshot_remodeling_four_owned_phases_cancel_before_work(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::*;let expected=fixture();let d=database(&expected);let input=payload(&expected,SnapshotEncoding::Binary);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut stop=|_|false;let mut c=SqliteSnapshotControl::new(&mut stop,Default::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>project(&expected,&mut c).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>restore(&d,&mut c).map(|_|()),SqliteSnapshotPhase::DecodeNative=>super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&input,&mut c).map(|_|()),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Binary,&mut c).map(|_|()),_=>unreachable!()};assert!(result.is_err(),"{phase:?}");}
}
#[test]
fn sqlite_snapshot_remodeling_large_unicode_copies_observe_both_budget_and_cancellation(){
 use store::ArtifactSqliteSnapshot;use store::sqlite_snapshot::*;let mut expected=fixture();expected.id="😀".repeat(laws()["control"]["largeCharacters"].as_u64().unwrap()as usize);let d=database(&expected);let input=payload(&expected,SnapshotEncoding::Text);
 for phase in[SqliteSnapshotPhase::ProjectSnapshot,SqliteSnapshotPhase::ReconstructSnapshot,SqliteSnapshotPhase::DecodeNative,SqliteSnapshotPhase::EncodeNative]{let mut reached=false;let mut progress=|event:SqliteSnapshotProgress|if event.phase==phase&&event.total>=65536&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true};let mut c=SqliteSnapshotControl::new(&mut progress,Default::default());let result=match phase{SqliteSnapshotPhase::ProjectSnapshot=>project(&expected,&mut c).map(|_|()),SqliteSnapshotPhase::ReconstructSnapshot=>restore(&d,&mut c).map(|_|()),SqliteSnapshotPhase::DecodeNative=>super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&input,&mut c).map(|_|()),SqliteSnapshotPhase::EncodeNative=>expected.encode_sqlite_snapshot_native(SnapshotEncoding::Text,&mut c).map(|_|()),_=>unreachable!()};assert!(result.is_err(),"{phase:?}");drop(c);assert!(reached,"{phase:?}");}
 let limits=SqliteDatabaseLimits{max_value_bytes:laws()["control"]["maximumOwnedBytes"].as_u64().unwrap()as usize,..Default::default()};assert!(restore(&d,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());for encoding in[SnapshotEncoding::Text,SnapshotEncoding::Binary]{let input=payload(&expected,encoding);assert!(expected.encode_sqlite_snapshot_native(encoding,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());assert!(super::super::RemodelingSnapshot::decode_sqlite_snapshot_native(&input,&mut SqliteSnapshotControl::new(&mut |_|true,limits)).is_err());}
}

fn laws()->serde_json::Value{serde_json::from_str(include_str!("../../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()}
fn word32()->Vec<f32>{laws()["binary32Words"].as_array().unwrap().iter().map(|v|f32::from_bits(u32::from_str_radix(v.as_str().unwrap(),16).unwrap())).collect()}
fn word64()->Vec<f64>{laws()["binary64Words"].as_array().unwrap().iter().map(|v|f64::from_bits(u64::from_str_radix(v.as_str().unwrap(),16).unwrap())).collect()}
fn child<S>(artifact_id:&str)->store::ArtifactChild<S>{let raw=laws()["literalChild"].clone();store::ArtifactChild::new(raw["childId"].as_str().unwrap().into(),store::io_schema::ArtifactRef{artifact_id:artifact_id.into(),dialect:store::io_schema::ArtifactDialect{artifact_kind:raw["artifactKind"].as_str().unwrap().into(),standard:raw["standard"].as_str().unwrap().into(),subset:raw["subset"].as_str().unwrap().into()}})}
fn fixture()->super::super::RemodelingSnapshot{
 let f=word32();let d=word64();let mut s=crate::default_remodeling_scene();s.id="世界\0literal".into();
 let raw=laws()["literalChild"].clone();
 s.assets.insert("".into(),child(raw["artifactId"].as_str().unwrap()));s.assets.insert("alias".into(),child(""));
 let chunks=laws()["byteChunks"].as_array().unwrap().iter().map(|v|ByteBuffer(v.as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect())).collect();
 s.durable_artifacts.insert("".into(),RemodelingDurableArtifact{kind:"unresolved\0世界".into(),mime:Some("".into()),width:u32::MAX,height:0,chunks});
 let frame=FrameRef{index:u32::MAX,timestamp_ms:d[1],asset_id:"unresolved\0".into()};
 let stream=MediaStream{id:"".into(),name:"世界".into(),kind:crate::MediaKind::Video,camera_id:Some("".into()),sync_offset_ms:d[3],fps_hint:d[4],frames:vec![frame.clone(),frame],source:Some(VideoSource{name:"".into(),container:"".into(),codec:VideoCodec::Unknown,duration_ms:d[5],frame_count:u32::MAX,width:u32::MAX,height:0})};s.streams=vec![stream.clone(),stream];
 let camera=CameraCalibration{id:"".into(),label:"".into(),model:"literal".into(),fx:d[0],fy:d[1],cx:d[2],cy:d[3],skew:d[5],distortion:[f[1],f[2],f[3],f[6],f[7]],rms_reprojection_px:Some(f[8]),locked:true};s.calibration.cameras=vec![camera.clone(),camera];
 s.calibration.rig=vec![RigExtrinsic{camera_id:"unresolved".into(),rotation_wxyz:[f[1],f[2],f[6],f[7]],translation_m:[f[4],f[5],f[8]]}];
 let observation=GcpObservation{stream_id:"".into(),frame_index:u32::MAX,pixel:[f[6],f[7]]};let point=GroundControlPoint{id:"".into(),name:"".into(),world_position:[d[1],d[5],d[6]],observations:vec![observation.clone(),observation]};s.gcps=vec![point.clone(),point];
 s.params.geo.origin_lon=Some(d[5]);s.params.geo.origin_lat=Some(d[6]);s.params.geo.origin_alt=Some(d[1]);s.params.mesh.tsdf_voxel_size_mm=f[7];s.params.motion.min_track_quality=f[8];s.params.matching.ratio_test=f[1];s.params.sfm.ransac_threshold_px=f[4];s.params.dense.confidence_threshold=f[5];
 let report=WatertightReportSnapshot{euler_characteristic:i64::MIN,genus:Some(i64::MAX),signed_volume:d[5],self_intersection_pairs:Some(u32::MAX),vertex_count:u32::MAX,..Default::default()};
 s.results.mesh.mesh=child(raw["artifactId"].as_str().unwrap());s.results.mesh.texture_asset_id=Some("".into());s.results.mesh.source=MeshSource::Imported;s.results.mesh.watertight=Some(report.clone());
 s.results.sparse=Some(SparseCloud{points:Float32Buffer::Inline{values:f.clone()},colors:Some(ByteBuffer(vec![]))});
 s.results.dense=Some(DenseCloud{positions:Float32Buffer::Content{content_id:"unresolved\0世界".into(),chunk_count:u64::MAX},colors:Some(ByteBuffer(vec![0,1,127,128,255])),confidence:Some(Float32Buffer::Inline{values:vec![]}),classification:Some(ByteBuffer(vec![]))});
 s.results.trajectory=Some(CameraTrajectory{poses:vec![CameraPosePreview{camera_id:"".into(),rotation_wxyz:[f[1],f[2],f[6],f[7]],translation:[f[4],f[5],f[8]]}]});
 let track=MotionTrackSummary{id:"".into(),length:u32::MAX,class:TrackClass::Moving,mean_speed_m_s:f[7]};s.results.tracks=vec![track.clone(),track];
 s.results.geo=Some(GeoProducts{dsm_asset_id:None,dtm_asset_id:Some("".into()),ortho_asset_id:Some("unresolved".into())});
 s.results.qc=Some(QcReportSnapshot{reprojection_rms_px:d[5],gcp_checkpoint_rmse:Some(d[6]),watertight:Some(report),mean_track_length:f[6],registered_frame_ratio:f[7],dense_coverage_ratio:f[8],warnings:vec!["".into(),"世界\0".into(),"世界\0".into()]});s
}
fn database(s:&super::super::RemodelingSnapshot)->SqliteDatabase{project(s,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap()}
fn restored(d:&SqliteDatabase)->super::super::RemodelingSnapshot{restore(d,&mut SqliteSnapshotControl::new(&mut |_|true,Default::default())).unwrap()}
fn same(a:&super::super::RemodelingSnapshot,b:&super::super::RemodelingSnapshot){assert_eq!(store::ArtifactDsl::print_dsl(a),store::ArtifactDsl::print_dsl(b));assert_eq!(store::ArtifactPack::encode_pack(a),store::ArtifactPack::encode_pack(b));}
fn oracle(script:&str,input:&[u8])->Vec<u8>{use std::io::Write;let mut child=std::process::Command::new("bun").args(["-e",script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));if !output.stderr.is_empty(){eprintln!("{}",String::from_utf8_lossy(&output.stderr));}output.stdout}
#[test]
fn sqlite_snapshot_remodeling_complete_entities_match_actual_sqlite_and_exact_native(){
 use store::sqlite_snapshot::*;let expected=fixture();let d=database(&expected);assert_eq!(d.tables.len(),laws()["tableCount"].as_u64().unwrap()as usize);assert!(d.tables.iter().all(|table|!table.rows.is_empty()));same(&restored(&d),&expected);let bytes=export_sqlite_database(&d,Default::default(),&mut |_|true).unwrap();
 let input=serde_json::json!({"bytes":bytes,"words":laws()["binary32Words"]});
 let script=r#"import{Database}from'bun:sqlite';const v=JSON.parse(await Bun.stdin.text());const db=Database.deserialize(new Uint8Array(v.bytes));if(db.query('PRAGMA foreign_key_check').all().length)throw Error('foreign ownership');const samples=db.query('SELECT sample_bits FROM remodel_float_sample ORDER BY ordinal').all();if(samples.length!==v.words.length)throw Error('samples');for(let i=0;i<samples.length;i++)if(samples[i].sample_bits!==Number.parseInt(v.words[i],16))throw Error('binary32 word');const content=db.query("SELECT chunk_count FROM remodel_float_buffer WHERE storage='content'").get();if(content.chunk_count!=='18446744073709551615')throw Error('unsigned64');const b=db.query('SELECT octets FROM remodel_durable_chunk ORDER BY ordinal').all();if(b.length!==2||b[0].octets.join(',')!=='0,1,127,128,255'||b[1].octets.length)throw Error('literal octets');await Bun.write(Bun.stdout,db.serialize());db.close();"#;
 let actual=import_sqlite_database(&oracle(script,input.to_string().as_bytes()),Default::default(),&mut |_|true).unwrap();same(&restored(&actual),&expected);
}
