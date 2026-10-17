//! 🧪️ SQLite snapshot laws of stdio.dxf: IEEE geometry and group values, controlled output, owned native control, the actual declaration and erased routes.
use crate::standards::v_r12::subsets::any::io::sqlite::snapshot::{value};
use semio_framework_test_fixtures::fixture_text;
    #[test]
    fn sqlite_snapshot_dxf_controlled_output_retains_complete_state_and_interior_admission(){
        let plan:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/📝️text/📸️snapshot/🧪️tests/🧫️fixtures/🛫️encoding/🔣️.json")).unwrap();let mut snapshot=fixture();snapshot.schema=plan["unit"].as_str().unwrap().repeat(plan["repetitions"].as_u64().unwrap() as usize);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=semio_s_artifact_stdio_contract::testing::with_encoding(|native_owner| snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,limits), native_owner)).unwrap();assert_eq!(semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits), native_owner)).unwrap(),snapshot);
            let mut reached=false;let result=semio_s_artifact_stdio_contract::testing::with_encoding(|native_owner| snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |event|{if event.phase==Phase::EncodeNative&&event.total==snapshot.schema.len()&&event.completed>=plan["cancelAfter"].as_u64().unwrap() as usize&&event.completed<event.total{reached=true;false}else{true}},limits), native_owner));assert!(reached);assert!(result.is_err());
            assert!(semio_s_artifact_stdio_contract::testing::with_encoding(|native_owner| snapshot.encode_sqlite_snapshot_native(encoding,&mut Control::new(&mut |_|true,sqlite_snapshot::SqliteDatabaseLimits{max_value_bytes:plan["tinyMaximumBytes"].as_u64().unwrap() as usize,..limits}), native_owner)).is_err());
        }
    }
    #[test]
    fn sqlite_snapshot_dxf_controlled_native_owner_preserves_full_fixture_and_stops_inside_text(){
        use store::{pack::snapshot::ArtifactDsl,pack::snapshot::ArtifactPack};use semio_framework_os_kernel::io::IoPayload;
        let mut snapshot=fixture();snapshot.schema="DXF 世界\0".repeat(16384);let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for payload in [IoPayload::Binary(snapshot.encode_pack()),IoPayload::Text(snapshot.print_dsl())]{
            assert_eq!(semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,limits), native_owner)).unwrap(),snapshot);
            let mut reached=false;let result=semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |event|{if event.phase==Phase::DecodeNative&&event.completed>=65536&&event.completed<event.total{reached=true;false}else{true}},limits), native_owner));assert!(result.is_err());assert!(reached,"cancel inside long native text");
            let mut small=limits;small.max_value_bytes=128;assert!(semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|true,small), native_owner)).is_err());
            assert!(semio_s_artifact_stdio_contract::testing::with_decoding(|native_owner| DxfSnapshot::decode_sqlite_snapshot_native(&payload,&mut Control::new(&mut |_|false,limits), native_owner)).is_err());
        }
    }

    #[test]
    fn sqlite_snapshot_dxf_owned_guard_checks_exact_identity_and_projection(){
        let plan:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎯️dialect.json")).unwrap();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let database=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let dialect=|value:&serde_json::Value|semio_framework_artifact_reference::ArtifactDialect{artifact_kind:value["artifactKind"].as_str().unwrap().into(),standard:value["standard"].as_str().unwrap().into(),subset:value["subset"].as_str().unwrap().into()};let valid=dialect(&plan["valid"]);assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|true,limits)).unwrap().diagnostics.is_empty());for invalid in plan["invalid"].as_array().unwrap(){assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid),&database,&mut Control::new(&mut |_|true,limits)).is_err());}let mut mismatched=database.clone();mismatched.table_mut("dxf_document").unwrap().rows[0].values[1]=V::Text("other.schema".into());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&mismatched,&mut Control::new(&mut |_|true,limits)).is_err());assert!(snapshot.validate_sqlite_snapshot_subset(&valid,&database,&mut Control::new(&mut |_|false,limits)).is_err());
    }
    #[allow(unused_imports)] use pack::snapshot::ArtifactSqliteSnapshot as _;
    #[allow(unused_imports)] use semio_framework_value::{ValueError,ValueRefusalKind};
    #[allow(unused_imports)] use crate::standards::v_r12::subsets::any::schema::snapshot::*;
    #[allow(unused_imports)] use crate::standards::v_r12::subsets::any::io::text::snapshot as snapshot_text;
    use store::os_store::serialization::ArtifactSqliteSnapshotHost;
    #[allow(unused_imports)] use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteValue as V, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C}};
    #[allow(unused_imports)] use std::collections::{BTreeMap, BTreeSet};
use semio_framework_value::FromValue;
    fn fixture() -> DxfSnapshot { DxfSnapshot::from_value(serde_json::from_str(fixture_text!("../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()).unwrap() }

    #[test]
    fn sqlite_snapshot_dxf_preserves_ieee_entity_geometry_and_group_values(){
        use std::{io::Write,process::{Command,Stdio}};let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔢️ieee.json")).unwrap();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        for case in cases["binary64"].as_array().unwrap(){let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut snapshot=fixture();snapshot.header_vars.push(DxfHeaderVar{name:"$IEEE".into(),group_code:40,value:DxfValue::Double{value},extra_group_codes:vec![(10,DxfValue::Point{value:[value;3]})]});snapshot.blocks[0].base_point=[value;3];let DxfEntity::Circle(DxfCircle {radius,center,..})=&mut snapshot.entities[1]else{panic!("circle");};*radius=value;*center=[value;3];let DxfEntity::Polyline(DxfPolyline {vertices,..})=&mut snapshot.entities[3]else{panic!("polyline");};vertices[0].bulge=value;let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();let restored=DxfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();let DxfValue::Double{value}=restored.header_vars.last().unwrap().value else{panic!("double");};assert_eq!(value.to_bits(),bits);let DxfValue::Point{value}=restored.header_vars.last().unwrap().extra_group_codes[0].1 else{panic!("point");};assert!(value.iter().all(|value|value.to_bits()==bits));assert!(restored.blocks[0].base_point.iter().all(|value|value.to_bits()==bits));let DxfEntity::Circle(DxfCircle {radius,center,..})=&restored.entities[1]else{panic!("circle");};assert_eq!(radius.to_bits(),bits);assert!(center.iter().all(|value|value.to_bits()==bits));let DxfEntity::Polyline(DxfPolyline {vertices,..})=&restored.entities[3]else{panic!("polyline");};assert_eq!(vertices[0].bulge.to_bits(),bits);
            let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query(\"SELECT CAST(double_value_bits AS TEXT) AS bits,double_value_class AS class,double_value IS NULL AS nullQuery FROM dxf_header WHERE name='$IEEE'\").get()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&file).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(actual["class"],case["class"]);assert_eq!(actual["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));}
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_declaration_preserves_complete_owned_model(){
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};
        crate::register_sqlite_test_declaration();
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.dxf".into(),standard:"r12".into(),subset:"*".into()};let mut snapshot=fixture();snapshot.schema="complete-owned-DXF-schema".into();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();
        let file=async{let original_file_dialect=&dialect;let original_file_input=&snapshot;let original_file_encoding=sqlite_snapshot::SnapshotEncoding::Binary;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |event|{phases.push(event.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_decoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;assert_eq!(async{let original_file_dialect=&dialect;let original_file_input=&file;let original_file_limits=limits;let original_file_progress:&mut dyn FnMut(semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |event|{phases.push(event.phase);true};semio_framework_os_kernel::io::testing::with_snapshot_file_encoding(semio_framework_os_kernel::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<DxfSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::EncodeNative|Phase::DecodeNative)));
    }
    async fn erased_snapshot(snapshot:&DxfSnapshot,encoding:sqlite_snapshot::SnapshotEncoding)->DxfSnapshot{
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::io_mechanism::io_route};
        use store::{pack::snapshot::ArtifactDsl,pack::snapshot::ArtifactPack};
        crate::register_sqlite_test_declaration();
        let dialect=ArtifactDialect{artifact_kind:"s.stdio.dxf".into(),standard:"r12".into(),subset:"*".into()};
        let sqlite:ArtifactDialect=semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();
        let payload=match encoding{sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(snapshot.encode_pack()),sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(snapshot.print_dsl())};
        let export=io_route(&dialect,&sqlite,1).await.unwrap().value;
        let file=semio_s_artifact_stdio_contract::testing::io_run(&export,payload, Default::default(), &mut |_| true).await.unwrap().value;
        let import=io_route(&sqlite,&dialect,1).await.unwrap().value;
        let payload=semio_s_artifact_stdio_contract::testing::io_run(&import,file, Default::default(), &mut |_| true).await.unwrap().value;
        match payload{IoPayload::Binary(bytes)=>DxfSnapshot::decode_pack(&bytes).unwrap(),IoPayload::Text(text)=>DxfSnapshot::parse_dsl(&text).unwrap()}
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_binary_preserves_complete_owned_model(){assert_eq!(erased_snapshot(&fixture(),sqlite_snapshot::SnapshotEncoding::Binary).await,fixture());}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_text_preserves_complete_owned_model(){assert_eq!(erased_snapshot(&fixture(),sqlite_snapshot::SnapshotEncoding::Text).await,fixture());}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_dxf_actual_erased_routes_preserve_exact_ieee_geometry_and_group_values(){
        let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../🏅️standards/🔖️r12/🪆️subsets/📰️header/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🔢️ieee.json")).unwrap();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary,sqlite_snapshot::SnapshotEncoding::Text]{for case in cases["binary64"].as_array().unwrap(){
            let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();let value=f64::from_bits(bits);let mut snapshot=fixture();
            snapshot.header_vars.push(DxfHeaderVar{name:"$IEEE".into(),group_code:40,value:DxfValue::Double{value},extra_group_codes:vec![(10,DxfValue::Point{value:[value;3]}),(160,DxfValue::Int{value:i64::MIN})]});
            snapshot.blocks[0].base_point=[value;3];let DxfEntity::Circle(DxfCircle {radius,center,..})=&mut snapshot.entities[1]else{panic!("circle");};*radius=value;*center=[value;3];let DxfEntity::Polyline(DxfPolyline {vertices,..})=&mut snapshot.entities[3]else{panic!("polyline");};vertices[0].bulge=value;
            let restored=erased_snapshot(&snapshot,encoding).await;assert_eq!(restored.schema,snapshot.schema);
            let header=restored.header_vars.last().unwrap();let DxfValue::Double{value}=header.value else{panic!("double");};assert_eq!(value.to_bits(),bits);let DxfValue::Point{value}=header.extra_group_codes[0].1 else{panic!("point");};assert_eq!(value.map(f64::to_bits),[bits;3]);assert_eq!(header.extra_group_codes[1].1,DxfValue::Int{value:i64::MIN});assert_eq!(restored.blocks[0].base_point.map(f64::to_bits),[bits;3]);
            let DxfEntity::Circle(DxfCircle {radius,center,..})=&restored.entities[1]else{panic!("circle");};assert_eq!(radius.to_bits(),bits);assert_eq!(center.map(f64::to_bits),[bits;3]);let DxfEntity::Polyline(DxfPolyline {vertices,..})=&restored.entities[3]else{panic!("polyline");};assert_eq!(vertices[0].bulge.to_bits(),bits);
        }}
    }
    #[test]
    fn sqlite_snapshot_dxf_independent_sql_entity_join_and_edit(){
        use std::{io::Write,process::{Command,Stdio}};let mut snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query(\"SELECT e.kind,p.role,p.x,p.y,p.z FROM dxf_entity e JOIN dxf_entity_point p ON p.entity_id=e.id WHERE e.kind='line' ORDER BY e.id,p.ordinal\").all();if(rows.length!==2||rows[0].role!=='start'||rows[1].role!=='end')throw Error('entity join');db.query(\"UPDATE dxf_entity SET radius=123.5,radius_bits=4638390956842811392,radius_class='finite' WHERE kind='circle'\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";
        let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let loaded=sqlite_snapshot::import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();if let DxfEntity::Circle(DxfCircle {radius,..})=&mut snapshot.entities[1]{*radius=123.5;}else{panic!("fixture circle");}assert_eq!(DxfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
    }

    #[test]
    fn sqlite_snapshot_dxf_preserves_every_typed_entity_and_group_code() {
        let snapshot=fixture(); let limits=sqlite_snapshot::SqliteDatabaseLimits::default(); let mut progress=|_|true; let mut control=Control::new(&mut progress,limits);
        let db=snapshot.to_sqlite_database(&mut control).unwrap();
        assert_eq!(db.table("dxf_entity").unwrap().rows.len(),9);
        assert_eq!(db.table("dxf_group_code").unwrap().rows.len(),13);
        assert_eq!(db.table("dxf_vertex").unwrap().rows.len(),2);
        assert_eq!(DxfSnapshot::from_sqlite_database(&db,&mut control).unwrap(),snapshot);
        let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();
        assert_eq!(DxfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),snapshot);
        let mut edited=loaded;let circle=edited.table_mut("dxf_entity").unwrap().rows.iter_mut().find(|row|row.text(4).unwrap()=="circle").unwrap();circle.values[6]=V::Real(123.5);circle.values[18]=V::Integer(123.5f64.to_bits() as i64);circle.values[19]=V::Text("finite".into());
        let changed=DxfSnapshot::from_sqlite_database(&edited,&mut control).unwrap();
        assert!(matches!(changed.entities[1],DxfEntity::Circle(DxfCircle {radius:123.5,..})));
    }

    #[test]
    fn sqlite_snapshot_dxf_rejects_broken_relations_and_controls_allocations() {
        let snapshot=fixture(); let limits=sqlite_snapshot::SqliteDatabaseLimits::default();
        assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|false,limits)).is_err());
        let mut small=limits; small.max_rows=2; assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
        small=limits; small.max_value_bytes=1; assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
        let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();
        assert!(DxfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|false,limits)).is_err());
        for column in [1,2] {let mut broken=db.clone();broken.table_mut("dxf_vertex").unwrap().rows[0].values[column]=V::Integer(999);assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());}
        let mut broken=db.clone(); broken.table_mut("dxf_entity").unwrap().rows.iter_mut().find(|row|row.text(4).unwrap()=="line").unwrap().values[17]=V::Text("wrong kind".into()); assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());
        let mut broken=db.clone();let sql=broken.table("dxf_document").unwrap().sql.replace("schema TEXT NOT NULL","schema TEXT \"NOT NULL\"");broken.table_mut("dxf_document").unwrap().sql=sql;assert!(DxfSnapshot::from_sqlite_database(&broken,&mut Control::new(&mut |_|true,limits)).is_err());
        let mut large=snapshot;large.entities=vec![large.entities[0].clone();1000];let mut calls=0;assert!(large.to_sqlite_database(&mut Control::new(&mut |_|{calls+=1;calls<4},limits)).is_err());
    }
