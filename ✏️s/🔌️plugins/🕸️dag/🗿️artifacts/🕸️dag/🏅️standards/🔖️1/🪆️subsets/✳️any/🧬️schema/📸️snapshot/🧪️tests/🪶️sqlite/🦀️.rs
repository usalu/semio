//! 🕸️ Actual two-field DAG parent baselines before semantic SQLite opt-in.
use crate::*;
use semio_framework_value::{FromValue, ToValue};
use store::{ArtifactDsl, ArtifactPack};

fn laws() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap()
}
fn full(index: usize) -> DagSnapshot {
    let l = laws();
    let row = &l["nativeCases"][index];
    let child = &row["content"];
    let target = &child["target"];
    let dialect = &target["dialect"];
    DagSnapshot {
        schema: row["schema"].as_str().unwrap().into(),
        content: store::ArtifactChild::new(
            child["childId"].as_str().unwrap().into(),
            store::io_schema::ArtifactRef {
                artifact_id: target["artifactId"].as_str().unwrap().into(),
                dialect: store::io_schema::ArtifactDialect { artifact_kind: dialect["artifactKind"].as_str().unwrap().into(), standard: dialect["standard"].as_str().unwrap().into(), subset: dialect["subset"].as_str().unwrap().into() },
            },
        ),
    }
}
struct Owned(Option<DagSnapshot>);
impl Owned {
    fn new(value: DagSnapshot) -> Self {
        Self(Some(value))
    }
}
impl std::ops::Deref for Owned {
    type Target = DagSnapshot;
    fn deref(&self) -> &DagSnapshot {
        self.0.as_ref().unwrap()
    }
}
impl Drop for Owned {
    fn drop(&mut self) {
        if let Some(value) = self.0.take() {
            let factory = semio_framework_value::retirement::OwnedValueRetirementFactory::<DagContentChild>::default();
            let mut cursor = store::ArtifactOwnedValueRetirementFactory::retire_owned(&factory, value.content);
            loop {
                match cursor.close_step(1, 65536).unwrap() {
                    store::SnapshotRetirementStep::Complete => {
                        assert!(cursor.terminal_is_empty());
                        break;
                    }
                    store::SnapshotRetirementStep::Pending { .. } => {}
                    store::SnapshotRetirementStep::Blocked => panic!("DAG child retirement blocked"),
                }
            }
        }
    }
}
fn assert_full(actual: &DagSnapshot, expected: &DagSnapshot) {
    assert_eq!(actual.schema, expected.schema);
    assert_eq!(actual.content.child_id, expected.content.child_id);
    assert_eq!(actual.content.target, expected.content.target);
}
#[test]
fn sqlite_snapshot_dag_actual_bare_parent_owns_capability() {
    assert!(DagSnapshot::sqlite_snapshot_codec().is_some(), "DAG parent lacks semantic SQLite capability");
}
#[test]
fn sqlite_snapshot_dag_actual_io_declaration_owns_parent_capability() {
    assert!(crate::standards::v1::subsets::any::io::io().native.codec.snapshot_sqlite.is_some(), "DAG actual declared Native document lacks parent capability");
}
#[test]
fn sqlite_snapshot_dag_binary_keeps_unresolved_literal_child_addresses() {
    for index in 0..3 {
        let expected = Owned::new(full(index));
        expected.validate().unwrap();
        let actual = Owned::new(DagSnapshot::decode_pack(&expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_dag_text_keeps_unresolved_literal_child_addresses() {
    for index in 0..3 {
        let expected = Owned::new(full(index));
        expected.validate().unwrap();
        let actual = Owned::new(DagSnapshot::parse_dsl(&expected.print_dsl()).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_dag_controlled_value_constructs_only_actual_parent_fields() {
    for index in 0..3 {
        let expected = Owned::new(full(index));
        let mut yes = |_| true;
        let mut output = semio_framework_value::NativeEncodeControl::new(4 << 20, &mut yes);
        let value = <DagSnapshot as ToValue>::to_value_controlled(&expected, &mut output).unwrap();
        let mut yes = |_| true;
        let mut input = semio_framework_value::NativeDecodeControl::new(4 << 20, &mut yes);
        let actual = Owned::new(<DagSnapshot as FromValue>::from_value_controlled(&value, &mut input).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_dag_declared_json_preserves_literal_parent_child_fields() {
    use crate::standards::v1::subsets::any::io::{export::serializers::artifacts::json::v_rfc8259::any as writer, import::deserializers::artifacts::json::v_rfc8259::any as reader};
    for index in 0..3 {
        let expected = Owned::new(full(index));
        let json = writer::serialize(&expected).unwrap();
        let actual = Owned::new(reader::deserialize(&json).unwrap());
        assert_full(&actual, &expected);
    }
}
#[test]
fn sqlite_snapshot_dag_independent_sqlite_interprets_literal_two_table_parent() {
    let script = r#"import{Database}from'bun:sqlite';const db=new Database(':memory:',{safeIntegers:true});try{db.exec(process.argv[1]);const l=JSON.parse(process.argv[2]);if(db.query("SELECT name FROM sqlite_schema WHERE type='table'").all().length!==2)throw Error('table count');for(const[n,w]of Object.entries(l.tableWidths))if(db.query('PRAGMA table_info('+n+')').all().length!==w)throw Error(n);for(const c of l.nativeCases){const t=c.content.target;db.query('INSERT OR REPLACE INTO dag_document VALUES(1,?)').run(c.schema);db.query('INSERT OR REPLACE INTO dag_content_child VALUES(1,1,?,?,?,?,?)').run(c.content.childId,t.artifactId,t.dialect.artifactKind,t.dialect.standard,t.dialect.subset);const r=db.query('SELECT child_id,artifact_id,artifact_kind,standard,subset FROM dag_content_child').get();if(r.child_id!==c.content.childId||r.artifact_id!==t.artifactId||r.artifact_kind!==t.dialect.artifactKind||r.standard!==t.dialect.standard||r.subset!==t.dialect.subset)throw Error('literal child');if(db.query('PRAGMA foreign_key_check').all().length)throw Error('foreign key')}}finally{db.close()}"#;
    let result = std::process::Command::new("bun").args(["-e", script, include_str!("../../🪶️sqlite/🗄️.sql"), &laws().to_string()]).output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
}
#[test]
fn sqlite_snapshot_dag_actual_erased_binary_text_keeps_all_literal_child_columns() {
    use store::sqlite_snapshot::*;
    let codec = store::ArtifactCodec::bare::<DagSnapshot, DagMutation>(DAG_DOCUMENT_SCHEMA).snapshot_sqlite.expect("missing DAG parent capability");
    let dialect = store::io_schema::ArtifactDialect { artifact_kind: "s.dag.dag".into(), standard: "1".into(), subset: "*".into() };
    for index in 0..3 {
        let expected = Owned::new(full(index));
        for encoding in [SnapshotEncoding::Binary, SnapshotEncoding::Text] {
            let native = match encoding {
                SnapshotEncoding::Binary => store::io_schema::IoPayload::Binary(expected.encode_pack_with(&store::PackEncodeOptions::default()).unwrap()),
                SnapshotEncoding::Text => store::io_schema::IoPayload::Text(expected.print_dsl()),
            };
            let database = (codec.export)(DAG_DOCUMENT_SCHEMA, &dialect, &native, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            assert_eq!(database.tables.len(), 2);
            assert_eq!(database.table("dag_document").unwrap().single_row().unwrap().text(1).unwrap(), expected.schema);
            let child = database.table("dag_content_child").unwrap().single_row().unwrap();
            assert_eq!(child.text(2).unwrap(), expected.content.child_id);
            assert_eq!(child.text(3).unwrap(), expected.content.target.artifact_id);
            assert_eq!(child.text(4).unwrap(), expected.content.target.dialect.artifact_kind);
            assert_eq!(child.text(5).unwrap(), expected.content.target.dialect.standard);
            assert_eq!(child.text(6).unwrap(), expected.content.target.dialect.subset);
            let payload = (codec.import)(DAG_DOCUMENT_SCHEMA, &dialect, database, encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
            let actual = Owned::new(match payload {
                store::io_schema::IoPayload::Text(text) => DagSnapshot::parse_dsl(&text).unwrap(),
                store::io_schema::IoPayload::Binary(bytes) => DagSnapshot::decode_pack(&bytes).unwrap(),
            });
            assert_full(&actual, &expected);
        }
    }
}

#[test]
fn sqlite_snapshot_dag_actual_semantic_rows_admit_exact_count_and_cancel_all_copy_phases() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let value = Owned::new(full(0));
    let exact = SqliteDatabaseLimits { max_rows: 2, ..Default::default() };
    let short = SqliteDatabaseLimits { max_rows: 1, ..Default::default() };
    assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, exact)).is_ok());
    assert!(value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, short)).is_err());
    let mut large = full(0);
    large.content.child_id = "引用😀".repeat(20000);
    large.content.target.artifact_id = "independent alias".into();
    let large = Owned::new(large);
    let database = large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let payload = store::io_schema::IoPayload::Binary(large.encode_pack());
    for phase in [SqliteSnapshotPhase::ProjectSnapshot, SqliteSnapshotPhase::ReconstructSnapshot, SqliteSnapshotPhase::DecodeNative, SqliteSnapshotPhase::EncodeNative] {
        let mut hit = false;
        let mut callback = |p: SqliteSnapshotProgress| {
            if p.phase == phase && p.total >= 65536 && p.completed >= 65536 && p.completed < p.total {
                hit = true;
                false
            } else {
                true
            }
        };
        let mut control = SqliteSnapshotControl::new(&mut callback, Default::default());
        let rejected = match phase {
            SqliteSnapshotPhase::ProjectSnapshot => large.to_sqlite_database(&mut control).is_err(),
            SqliteSnapshotPhase::ReconstructSnapshot => DagSnapshot::from_sqlite_database(&database, &mut control).is_err(),
            SqliteSnapshotPhase::DecodeNative => DagSnapshot::decode_sqlite_snapshot_native(&payload, &mut control).is_err(),
            SqliteSnapshotPhase::EncodeNative => large.encode_sqlite_snapshot_native(SnapshotEncoding::Binary, &mut control).is_err(),
            _ => unreachable!(),
        };
        assert!(rejected);
        assert!(hit, "{:?}", phase);
    }
}
#[test]
fn sqlite_snapshot_dag_owned_relationship_and_dialect_edits_are_rejected() {
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let value = Owned::new(full(2));
    let database = value.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    for change in 0..4 {
        let mut db = database.clone();
        match change {
            0 => db.table_mut("dag_content_child").unwrap().rows[0].values[1] = SqliteValue::Integer(999),
            1 => db.table_mut("dag_content_child").unwrap().rows[0].values[6] = SqliteValue::Text("kit".into()),
            2 => {
                let row = db.table("dag_document").unwrap().rows[0].clone();
                db.table_mut("dag_document").unwrap().rows.push(row)
            }
            _ => db.table_mut("dag_document").unwrap().rows[0].values[1] = SqliteValue::Text("invalid".into()),
        };
        assert!(DagSnapshot::from_sqlite_database(&db, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).is_err());
    }
}

#[test]
fn sqlite_snapshot_dag_independent_sqlite_edits_reconstruct_literal_parent() {
    use std::io::Write;
    use store::sqlite_snapshot::*;
    use store::ArtifactSqliteSnapshot;
    let expected = Owned::new(full(2));
    let database = expected.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap();
    let bytes = export_sqlite_database(&database, Default::default(), &mut |_| true).unwrap();
    let script = r#"import{Database}from'bun:sqlite';const db=Database.deserialize(await Bun.stdin.bytes());if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('SQLite integrity');if(db.query('SELECT count(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==2)throw Error('tables');db.exec("UPDATE dag_content_child SET child_id='independent child',artifact_id='independent target'");process.stdout.write(db.serialize());"#;
    let mut child = std::process::Command::new("bun").args(["--eval", script]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    let result = child.wait_with_output().unwrap();
    assert!(result.status.success(), "{}", String::from_utf8_lossy(&result.stderr));
    let database = import_sqlite_database(&result.stdout, Default::default(), &mut |_| true).unwrap();
    let actual = Owned::new(DagSnapshot::from_sqlite_database(&database, &mut SqliteSnapshotControl::new(&mut |_| true, Default::default())).unwrap());
    assert_eq!(actual.content.child_id, "independent child");
    assert_eq!(actual.content.target.artifact_id, "independent target");
    assert_eq!(actual.content.target.dialect, expected.content.target.dialect);
    assert_eq!(actual.schema, expected.schema);
}
