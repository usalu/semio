use super::*;
use semio_framework_os_kernel::{sqlite_snapshot::*, ArtifactSqliteSnapshot};

fn fixture() -> HtmlSnapshot { let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap(); parse_html_document(fixture["htmlText"].as_str().unwrap()).unwrap() }
fn project(snapshot: &HtmlSnapshot) -> SqliteDatabase { snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap() }
fn restore(database: &SqliteDatabase) -> Result<HtmlSnapshot, String> { HtmlSnapshot::from_sqlite_database(database, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())) }

#[test]
fn sqlite_snapshot_html_owned_dialect_guard_requires_its_declared_coordinate() {
    let source: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🪶️sqlite/🔣️.json")).unwrap();
    let snapshot = fixture(); let database = project(&snapshot);
    let dialect = |value: &serde_json::Value| semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: value["artifactKind"].as_str().unwrap().into(), standard: value["standard"].as_str().unwrap().into(), subset: value["subset"].as_str().unwrap().into() };
    let mut callback = |_| true; let mut control = SqliteSnapshotControl::new(&mut callback, SqliteDatabaseLimits::default());
    let valid = dialect(&source["sqliteDialect"]);
    assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &database, &mut control).unwrap().diagnostics.is_empty());
    for invalid in source["invalidSqliteDialects"].as_array().unwrap() { assert!(snapshot.validate_sqlite_snapshot_subset(&dialect(invalid), &database, &mut control).is_err()); }
    let mut mismatched = database.clone(); mismatched.table_mut("html_document").unwrap().rows[0].values[1] = SqliteValue::Text("different.schema".into());
    assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &mismatched, &mut control).is_err());
    assert!(snapshot.validate_sqlite_snapshot_subset(&valid, &database, &mut SqliteSnapshotControl::new(&mut |_| false, SqliteDatabaseLimits::default())).is_err());
}

#[test]
fn sqlite_snapshot_html_preserves_boolean_empty_attributes_and_raw_text() {
    let snapshot = fixture(); let database = project(&snapshot);
    let attrs = &database.table("html_attribute").unwrap().rows;
    assert_eq!(attrs.iter().find(|row| row.text(3).unwrap() == "disabled").unwrap().optional_text(4).unwrap(), None);
    assert_eq!(attrs.iter().find(|row| row.text(3).unwrap() == "value").unwrap().optional_text(4).unwrap(), Some(""));
    assert_eq!(database.table("html_raw_text").unwrap().rows.iter().map(|row| row.text(1).unwrap()).collect::<Vec<_>>(), ["style", "script"]);
    let bytes = export_sqlite_database(&database, SqliteDatabaseLimits::default(), &mut |_| true).unwrap(); let restored = restore(&import_sqlite_database(&bytes, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap(); assert_eq!(restored, snapshot);
    assert_eq!(<HtmlSnapshot as store::ArtifactPack>::encode_pack(&restored), <HtmlSnapshot as store::ArtifactPack>::encode_pack(&snapshot));
    assert_eq!(<HtmlSnapshot as store::ArtifactDsl>::print_dsl(&restored), <HtmlSnapshot as store::ArtifactDsl>::print_dsl(&snapshot));
}

#[test]
fn sqlite_snapshot_html_optional_and_variant_fields_survive_without_xml_coercion() {
    for root in [HtmlNode::Text { text: "literal 🌠".into() }, HtmlNode::Comment { text: "comment".into() }, HtmlNode::RawText { parent_kind: RawTextKind::Script, text: "x<y".into() }, HtmlNode::RawText { parent_kind: RawTextKind::Style, text: "a>b".into() }, HtmlNode::Element { name: "custom".into(), attributes: vec![HtmlAttr::boolean("enabled"), HtmlAttr::new("empty", ""), HtmlAttr::new("value", "Grüße 🌠")], children: Vec::new() }] {
        let snapshot = HtmlSnapshot { schema: "custom.html".into(), doctype: None, root }; assert_eq!(restore(&project(&snapshot)).unwrap(), snapshot);
    }
    assert_eq!(restore(&project(&HtmlSnapshot::default())).unwrap(), HtmlSnapshot::default());
}

#[test]
fn sqlite_snapshot_html_rejects_invalid_graphs_shapes_and_resource_limits() {
    let snapshot = fixture(); let original = project(&snapshot);
    for alteration in 0..6 { let mut database = original.clone(); match alteration {
        0 => database.table_mut("html_document").unwrap().rows[0].values[3] = SqliteValue::Integer(999),
        1 => { database.table_mut("html_element").unwrap().rows.remove(0); },
        2 => database.table_mut("html_child").unwrap().rows[0].values[3] = SqliteValue::Integer(1),
        3 => database.table_mut("html_attribute").unwrap().rows[0].values[2] = SqliteValue::Integer(10),
        4 => database.table_mut("html_raw_text").unwrap().rows[0].values[1] = SqliteValue::Text("textarea".into()),
        _ => database.table_mut("html_attribute").unwrap().rows[0].values[4] = SqliteValue::Integer(0),
    } assert!(restore(&database).is_err(), "alteration {alteration}"); }
    let rows = original.tables.iter().map(|table| table.rows.len()).sum(); let bytes = original.tables.iter().flat_map(|table| &table.rows).flat_map(|row| &row.values).map(|value| match value { SqliteValue::Null => 0, SqliteValue::Integer(_) | SqliteValue::Real(_) => 8, SqliteValue::Text(text) => text.len(), SqliteValue::Blob(bytes) => bytes.len() }).sum();
    let limits = SqliteDatabaseLimits { max_rows: rows, max_value_bytes: bytes, ..SqliteDatabaseLimits::default() }; assert_eq!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).unwrap(), original);
    for limits in [SqliteDatabaseLimits { max_rows: rows - 1, ..limits }, SqliteDatabaseLimits { max_value_bytes: bytes - 1, ..limits }] { assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err()); assert!(HtmlSnapshot::from_sqlite_database(&original, &mut SqliteSnapshotControl::new(&mut |_| true, limits)).is_err()); }
    assert!(snapshot.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| false, limits)).is_err());
    let mut large = HtmlSnapshot::default(); large.root = HtmlNode::Element { name: "html".into(), attributes: Vec::new(), children: (0..1024).map(|_| HtmlNode::Text { text: "value".into() }).collect() }; let mut checkpoints = 0;
    assert!(large.to_sqlite_database(&mut SqliteSnapshotControl::new(&mut |_| { checkpoints += 1; checkpoints < 3 }, SqliteDatabaseLimits::default())).is_err()); assert_eq!(checkpoints, 3);
}

#[test]
fn sqlite_snapshot_html_independent_queries_edits_and_html_parser_oracle() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let snapshot = fixture(); let bytes = export_sqlite_database(&project(&snapshot), SqliteDatabaseLimits::default(), &mut |_| true).unwrap();
    let script = "import { Database } from 'bun:sqlite'; const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer())); if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw new Error('integrity'); if(db.query('SELECT value FROM html_attribute WHERE name=?').get('disabled').value!==null||db.query('SELECT value FROM html_attribute WHERE name=?').get('value').value!=='')throw new Error('boolean/empty'); if(db.query('SELECT parent_kind FROM html_raw_text ORDER BY node_id').all().map(r=>r.parent_kind).join(',')!=='style,script')throw new Error('raw text'); db.query('UPDATE html_attribute SET value=? WHERE name=?').run('SQLite 🌠 & attribute','z'); await Bun.write(Bun.stdout,db.serialize()); db.close();";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(&bytes).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let edited = restore(&import_sqlite_database(&output.stdout, SqliteDatabaseLimits::default(), &mut |_| true).unwrap()).unwrap();
    let mut expected = snapshot; fn edit(node: &mut HtmlNode) { if let HtmlNode::Element { attributes, children, .. } = node { for attr in attributes { if attr.name == "z" { attr.value = Some("SQLite 🌠 & attribute".into()); } } for child in children { edit(child); } } } edit(&mut expected.root); assert_eq!(edited, expected);
    let oracle = "const source=await Bun.stdin.text(); const names=[]; const attrs=[]; const parser=new HTMLRewriter().on('*',{element(element){names.push(element.tagName); if(element.tagName==='p')for(const attr of element.attributes)attrs.push(attr);}}); await parser.transform(new Response(source)).text(); if(names.join(',')!=='html,head,style,script,body,input,p')throw new Error('HTML tree'); if(attrs[0][0]!=='z'||attrs[0][1]!=='SQLite 🌠 &amp; attribute'||attrs[1][0]!=='a')throw new Error('HTML attributes'); console.log('[DEBUG] independent HTML parser checked ordered entities');";
    let mut child = Command::new("bun").args(["-e", oracle]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap(); child.stdin.take().unwrap().write_all(write_html_document(&edited).as_bytes()).unwrap(); let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr)); assert!(String::from_utf8_lossy(&output.stdout).contains("independent HTML parser checked"));
}

#[test]
fn sqlite_snapshot_html_erased_native_preflight_admission_and_limits() {
    let snapshot = fixture();
    let dialect = semio_framework_os_kernel::io_schema::ArtifactDialect { artifact_kind: "s.stdio.html".into(), standard: "5".into(), subset: "*".into() };
    let codec = <HtmlSnapshot as store::ArtifactPack>::sqlite_snapshot_codec().unwrap();
    for encoding in [SnapshotEncoding::Text, SnapshotEncoding::Binary] {
        snapshot.preflight_sqlite_snapshot_encoding(encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap();
        let actual = (codec.import)("html preflight", &dialect, project(&snapshot), encoding, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits::default())).unwrap().value;
        let expected = match encoding { SnapshotEncoding::Text => semio_framework_os_kernel::io_schema::IoPayload::Text(store::ArtifactDsl::print_dsl(&snapshot)), SnapshotEncoding::Binary => semio_framework_os_kernel::io_schema::IoPayload::Binary(store::ArtifactPack::encode_pack(&snapshot)) };
        assert_eq!(actual, expected);
    }
    let large = HtmlSnapshot { schema: "stdio.html".into(), doctype: None, root: HtmlNode::Element { name: "html".into(), attributes: Vec::new(), children: vec![HtmlNode::Text { text: "x".repeat(100000) }] } };
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Text, &mut SqliteSnapshotControl::new(&mut |_| true, SqliteDatabaseLimits { max_value_bytes: 4096, ..SqliteDatabaseLimits::default() })).is_err());
    assert!(large.preflight_sqlite_snapshot_encoding(SnapshotEncoding::Binary, &mut SqliteSnapshotControl::new(&mut |p| p.phase != SqliteSnapshotPhase::EncodeNative, SqliteDatabaseLimits::default())).is_err());
}
