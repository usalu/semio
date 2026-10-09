//! 📄️ PDF 1.4 resolved pages in their declared reading order.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1_4::subsets::base::schema::snapshot::{PdfSnapshot, PageDoc};
use store::sqlite_snapshot::{self, SqliteDatabase as Db, SqliteSnapshotControl as Control, SqliteSnapshotPhase as Phase, artifact::{Cell as C, Projection,FloatColumn,FloatRow,insert_ieee754}};

const PAGE_FLOATS:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];

impl store::ArtifactSqliteSnapshot for PdfSnapshot {
    fn encode_sqlite_snapshot_native(&self,encoding:sqlite_snapshot::SnapshotEncoding,control:&mut Control<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{(||->Result<(),ValueError>{control.checkpoint(Phase::EncodeNative,0,0)?;control.check_rows(self.pages.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF page count overflow"))?)?;Ok(())})()?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)}
    fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut Control<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
    fn preflight_sqlite_snapshot_encoding(&self, _: sqlite_snapshot::SnapshotEncoding, control: &mut Control<'_>) -> Result<(),ValueError> {(||->Result<(),ValueError>{
        let mut bound = sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
        bound.add(4096)?;
        bound.repeated(self.schema.len(), 16)?;
        for page in &self.pages { bound.add(8192)?; bound.repeated(page.text.len(), 16)?; }
        bound.finish()
    })()}
    fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,_database:&Db,control:&mut Control<'_>)->store::io_schema::IoResult<()>{(||->Result<store::io_schema::IoOutcome<()>,ValueError>{
        if dialect.artifact_kind!="s.stdio.pdf"||dialect.standard!="1.4"{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF1.4 snapshot does not own this dialect"));}
        control.checkpoint(Phase::ProjectSnapshot,0,1)?;
        let diagnostics=match dialect.subset.as_str(){"*"=>Vec::new(),"a"=>crate::standards::v1_4::subsets::a::io::check_pdf_a_conformance(self),"x"=>crate::standards::v1_4::subsets::x::io::check_pdf_x_conformance(self),_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF1.4 subset has no semantic validator"))};
        control.checkpoint(Phase::ProjectSnapshot,1,1)?;
        Ok(store::io_schema::IoOutcome{value:(),diagnostics})
    })().map_err(store::io_schema::IoError::from_value_error)}
    fn to_sqlite_database(&self, control: &mut Control<'_>) -> Result<Db,ValueError> {(||->Result<Db,ValueError>{
        let mut projection=Projection::new(Self::SQLITE_SCHEMA,control)?;
        projection.check_rows(self.pages.len().checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"PDF page count overflow"))?)?;
        projection.insert("pdf14_document",&[C::Text(&self.schema)])?;
        for(ordinal,page)in self.pages.iter().enumerate(){insert_ieee754(&mut projection,"pdf14_page",&[C::Integer(1),C::Integer(ordinal as i64),C::Real(page.width),C::Real(page.height),C::Text(&page.text)],PAGE_FLOATS)?;}
        projection.finish()
    })()}
    fn from_sqlite_database(db:&Db,control:&mut Control<'_>)->Result<Self,ValueError>{
        use semio_framework_value::DecodedValue;
        use sqlite_snapshot::transfer;
        control.checkpoint(Phase::ReconstructSnapshot,0,0)?;
        transfer::validate_database_controlled(db,Self::SQLITE_SCHEMA,Phase::ReconstructSnapshot,control)?;
        control.check_database(db,Phase::ReconstructSnapshot)?;
        let document=db.table("pdf14_document")?.single_row()?;
        if document.rowid!=1||document.values.len()!=2||document.integer(0)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF document identity"));}
        let rows=&db.table("pdf14_page")?.rows;
        let mut sorted=transfer::reserve(rows.len(),control)?;
        for(index,row)in rows.iter().enumerate(){
            control.checkpoint(Phase::ReconstructSnapshot,index,rows.len())?;
            if row.values.len()!=10||row.rowid<=0||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF page identity or document relationship"));}
            sorted.push((row.integer(2)?,FloatRow::new(row,PAGE_FLOATS)?));
        }
        transfer::heap_sort(&mut sorted,Phase::ReconstructSnapshot,control,|left,right,_|Ok(left.1.rowid.cmp(&right.1.rowid)))?;
        for index in 1..sorted.len(){
            control.checkpoint(Phase::ReconstructSnapshot,index,sorted.len())?;
            if sorted[index-1].1.rowid==sorted[index].1.rowid{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid PDF page identity or document relationship"));}
        }
        transfer::heap_sort(&mut sorted,Phase::ReconstructSnapshot,control,|left,right,_|Ok(left.0.cmp(&right.0)))?;
        let mut pages=DecodedValue::new(transfer::reserve(rows.len(),control)?,|pages:Vec<PageDoc>|drop(pages));
        for(index,(ordinal,row))in sorted.into_iter().enumerate(){
            let expected=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"PDF page ordinal exceeds INTEGER width"))?;
            if ordinal!=expected{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"PDF page order must be contiguous and unique"));}
            control.checkpoint(Phase::ReconstructSnapshot,index,rows.len())?;
            pages.get_mut().push(PageDoc{width:row.real(3)?,height:row.real(4)?,text:sqlite_snapshot::artifact::Reconstruction::new(control)?.text(row.text(5)?)?});
        }
        let schema=sqlite_snapshot::artifact::Reconstruction::new(control)?.text(document.text(1)?)?;
        control.checkpoint(Phase::ReconstructSnapshot,rows.len(),rows.len())?;
        Ok(Self{schema,pages:pages.take()})
    }
}

#[cfg(test)]
#[path = "🧪️tests/💰️backing/🦀️.rs"]
mod backing;

#[cfg(test)]
mod tests {
    #[test]
    fn sqlite_snapshot_pdf14_full_concrete_backing_preserves_owned_pages_and_caller_limits() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔢ieee.json")).unwrap();
        assert_eq!(cases["backing"]["authority"], "completeSystemAllocatorRequests");
        let mut snapshot = fixture();
        snapshot.schema = "literal PDF 世界\0".into();
        for (index, case) in cases["binary64"].as_array().unwrap().iter().enumerate() {
            let value = f64::from_bits(u64::from_str_radix(case["bits"].as_str().unwrap(), 16).unwrap());
            snapshot.pages.push(PageDoc { width: value, height: value, text: format!("page-{index}\0😀") });
        }
        snapshot.pages[0].text = "x".repeat(131_072);
        crate::standards::v1_4::subsets::base::io::sqlite::snapshot::backing::verify(&snapshot);
    }

    use crate::standards::v1_4::subsets::base::io::sqlite::snapshot::*;
    use store::ArtifactSqliteSnapshot;
    use semio_framework_value::FromValue;
    fn fixture()->PdfSnapshot{PdfSnapshot::from_value(serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()).unwrap()}
    async fn erased_roundtrip(snapshot: &PdfSnapshot, encoding: sqlite_snapshot::SnapshotEncoding) -> PdfSnapshot {
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_os_kernel::io::IoPayload,semio_framework_os_kernel::io::io_mechanism::io_route,semio_framework_os_kernel::io::io_mechanism::io_run};
        crate::register_sqlite_test_declaration();
        let dialect = ArtifactDialect { artifact_kind: "s.stdio.pdf".into(), standard: "1.4".into(), subset: "*".into() };
        let sqlite = semio_framework_os_kernel::io_schema::SQLITE_SNAPSHOT.into();
        let payload = match encoding { sqlite_snapshot::SnapshotEncoding::Binary => IoPayload::Binary(store::ArtifactPack::encode_pack(snapshot)), sqlite_snapshot::SnapshotEncoding::Text => IoPayload::Text(store::ArtifactDsl::print_dsl(snapshot)) };
        let bytes = io_run(&io_route(&dialect, &sqlite, 1).await.unwrap().value, payload).await.unwrap().value;
        let payload = io_run(&io_route(&sqlite, &dialect, 1).await.unwrap().value, bytes).await.unwrap().value;
        match payload { IoPayload::Binary(bytes) => store::ArtifactPack::decode_pack(&bytes).unwrap(), IoPayload::Text(text) => store::ArtifactDsl::parse_dsl(&text).unwrap() }
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf14_actual_erased_binary_preserves_owned_schema_pages_and_ieee() {
        let snapshot = fixture(); assert_eq!(erased_roundtrip(&snapshot, sqlite_snapshot::SnapshotEncoding::Binary).await, snapshot);
        erased_ieee(sqlite_snapshot::SnapshotEncoding::Binary).await;
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf14_actual_erased_text_preserves_owned_schema_pages_and_ieee() {
        let snapshot = fixture(); assert_eq!(erased_roundtrip(&snapshot, sqlite_snapshot::SnapshotEncoding::Text).await, snapshot);
        erased_ieee(sqlite_snapshot::SnapshotEncoding::Text).await;
    }
    async fn erased_ieee(encoding: sqlite_snapshot::SnapshotEncoding) {
        let cases: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔢ieee.json")).unwrap();
        let pages = cases["binary64"].as_array().unwrap().iter().map(|case| { let bits = u64::from_str_radix(case["bits"].as_str().unwrap(), 16).unwrap(); PageDoc { width: f64::from_bits(bits), height: f64::from_bits(bits), text: case["class"].as_str().unwrap().into() } }).collect();
        let snapshot = PdfSnapshot { schema: "every-owned-pdf14-word".into(), pages };
        let restored = erased_roundtrip(&snapshot, encoding).await;
        assert_eq!(restored.schema, snapshot.schema); assert_eq!(restored.pages.len(), snapshot.pages.len());
        for (expected, actual) in snapshot.pages.iter().zip(restored.pages) { assert_eq!(actual.width.to_bits(), expected.width.to_bits()); assert_eq!(actual.height.to_bits(), expected.height.to_bits()); assert_eq!(actual.text, expected.text); }
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf14_owned_io_preserves_arbitrary_schema_and_text(){
        use {semio_framework_artifact_reference::ArtifactDialect,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_os_kernel::io::register_native_snapshot_codec,semio_framework_os_kernel::io::io_mechanism::io_export_sqlite_snapshot,semio_framework_os_kernel::io::io_mechanism::io_import_sqlite_snapshot};let native=Dialect{artifact_kind:"s.stdio.pdf",standard:StandardId("1.4"),subset:SubsetId("*")};register_native_snapshot_codec(native,store::ArtifactCodec::of::<PdfSnapshot,crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation>(crate::STDIO_PDF_DOCUMENT_SCHEMA)).unwrap();let dialect:ArtifactDialect=native.into();let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut phases=Vec::new();let file=io_export_sqlite_snapshot(&dialect,&snapshot,sqlite_snapshot::SnapshotEncoding::Binary,limits,&mut |event|{phases.push(event.phase);true}).await.unwrap().value;assert_eq!(io_import_sqlite_snapshot::<PdfSnapshot>(&dialect,&file,limits,&mut |_|true).await.unwrap().value,snapshot);assert!(!phases.iter().any(|phase|matches!(phase,Phase::DecodeNative|Phase::EncodeNative)));
    }
    #[test]
    fn sqlite_snapshot_pdf14_independent_sql_page_query_and_edit(){
        use std::{io::Write,process::{Command,Stdio}};let mut snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');const rows=db.query('SELECT p.ordinal,p.width,p.height,p.text FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal').all();if(rows.length!==3)throw Error('page query');db.query(\"UPDATE pdf14_page SET text='independent edit' WHERE ordinal=1\").run();await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let loaded=sqlite_snapshot::import_sqlite_database(&output.stdout,limits,&mut |_|true).unwrap();snapshot.pages[1].text="independent edit".into();assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap(),snapshot);
    }
    #[test]
    fn sqlite_snapshot_pdf14_preserves_ieee_width_and_height(){
        let cases:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔢ieee.json")).unwrap();let pages=cases["binary64"].as_array().unwrap().iter().map(|value|{let bits=u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap();PageDoc{width:f64::from_bits(bits),height:f64::from_bits(bits),text:value["class"].as_str().unwrap().into()}}).collect();let snapshot=PdfSnapshot{schema:"every-ieee-page".into(),pages};let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();for(row,case)in db.table("pdf14_page").unwrap().rows.iter().zip(cases["binary64"].as_array().unwrap()){assert_eq!(row.text(7).unwrap(),case["class"].as_str().unwrap());assert_eq!(row.integer(6).unwrap() as u64,u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap());}let bytes=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();
        use std::{io::Write,process::{Command,Stdio}};let script="import {Database} from 'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok'||db.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT CAST(width_bits AS TEXT) AS bits,width_class AS class,width IS NULL AS nullQuery FROM pdf14_page ORDER BY ordinal').all()));db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let independent:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();for(row,case)in independent.as_array().unwrap().iter().zip(cases["binary64"].as_array().unwrap()){assert_eq!(row["class"],case["class"]);let bits=u64::from_str_radix(case["bits"].as_str().unwrap(),16).unwrap();assert_eq!(row["bits"].as_str().unwrap(),(bits as i64).to_string());assert_eq!(row["nullQuery"].as_i64().unwrap(),i64::from(case["class"]=="nan"));}
        let loaded=sqlite_snapshot::import_sqlite_database(&bytes,limits,&mut |_|true).unwrap();let restored=PdfSnapshot::from_sqlite_database(&loaded,&mut Control::new(&mut |_|true,limits)).unwrap();for(before,after)in snapshot.pages.iter().zip(restored.pages){assert_eq!(before.width.to_bits(),after.width.to_bits());assert_eq!(before.height.to_bits(),after.height.to_bits());}
        let mut bad=loaded;bad.table_mut("pdf14_page").unwrap().rows[0].values[7]=sqlite_snapshot::SqliteValue::Text("finite".into());assert!(PdfSnapshot::from_sqlite_database(&bad,&mut Control::new(&mut |_|true,limits)).is_err());
    }
    #[test]
    fn sqlite_snapshot_pdf14_preserves_all_pages_and_empty_documents(){
        let limits=sqlite_snapshot::SqliteDatabaseLimits::default();let mut progress=|_|true;let mut control=Control::new(&mut progress,limits);
        for snapshot in [fixture(),PdfSnapshot{schema:"empty-schema".into(),pages:Vec::new()}]{
            let db=snapshot.to_sqlite_database(&mut control).unwrap();assert_eq!(db.table("pdf14_page").unwrap().rows.len(),snapshot.pages.len());
            assert_eq!(PdfSnapshot::from_sqlite_database(&db,&mut control).unwrap(),snapshot);
            let file=sqlite_snapshot::export_sqlite_database(&db,limits,&mut |_|true).unwrap();let loaded=sqlite_snapshot::import_sqlite_database(&file,limits,&mut |_|true).unwrap();assert_eq!(PdfSnapshot::from_sqlite_database(&loaded,&mut control).unwrap(),snapshot);
        }
        let mut db=fixture().to_sqlite_database(&mut control).unwrap();db.table_mut("pdf14_page").unwrap().rows[0].values[5]=sqlite_snapshot::SqliteValue::Text("SQL edited".into());assert_eq!(PdfSnapshot::from_sqlite_database(&db,&mut control).unwrap().pages[0].text,"SQL edited");
    }
    #[test]
    fn sqlite_snapshot_pdf14_rejects_bad_relations_and_bounds_work(){
        let snapshot=fixture();let limits=sqlite_snapshot::SqliteDatabaseLimits::default();assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|false,limits)).is_err());
        let mut small=limits;small.max_rows=2;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());small=limits;small.max_value_bytes=1;assert!(snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,small)).is_err());
        let db=snapshot.to_sqlite_database(&mut Control::new(&mut |_|true,limits)).unwrap();assert!(PdfSnapshot::from_sqlite_database(&db,&mut Control::new(&mut |_|false,limits)).is_err());
        for column in [1,2]{let mut bad=db.clone();bad.table_mut("pdf14_page").unwrap().rows[0].values[column]=sqlite_snapshot::SqliteValue::Integer(999);assert!(PdfSnapshot::from_sqlite_database(&bad,&mut Control::new(&mut |_|true,limits)).is_err());}
        let mut large=snapshot;large.pages=vec![large.pages[0].clone();1000];let mut calls=0;assert!(large.to_sqlite_database(&mut Control::new(&mut |_|{calls+=1;calls<4},limits)).is_err());
    }
}

#[cfg(test)]
mod own14_public_page_domain {
    use crate::standards::v1_4::subsets::base::io::sqlite::snapshot::*;
    use store::ArtifactSqliteSnapshot;
    const CORPUS: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../\u{1f3c5}️standards/4️⃣1.4/\u{1fa86}️subsets/\u{1f9f1}️base/✏️editor/\u{1f9eb}️fixtures/\u{1f4c4}️resolved-page-domain/\u{1f523}️.json"));
    fn owner(edited: bool) -> PdfSnapshot {
        let fixture: serde_json::Value = serde_json::from_str(CORPUS).unwrap();
        let pages = fixture["pages"].as_array().unwrap().iter().enumerate().map(|(index, page)| {
            let values = if edited && index == fixture["sqlEdit"]["page"].as_u64().unwrap() as usize { &fixture["sqlEdit"] } else { page };
            PageDoc { width: values["width"].as_f64().unwrap(), height: values["height"].as_f64().unwrap(), text: values["text"].as_str().unwrap().into() }
        }).collect();
        PdfSnapshot { schema: fixture["schema"].as_str().unwrap().into(), pages }
    }
    fn independent_edit(bytes: &[u8], encoding: &str) -> Vec<u8> {
        use std::{io::Write, process::{Command, Stdio}};
        let script = r#"import {Database} from 'bun:sqlite';import {Buffer} from 'node:buffer';const fixture=JSON.parse(Bun.argv.at(-2)),db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()),{safeIntegers:true});const same=(a,b)=>{if(JSON.stringify(a)!==JSON.stringify(b))throw Error('complete independent PDF14 owner query mismatch');};same(db.query('PRAGMA integrity_check').get(),{integrity_check:'ok'});same(db.query('PRAGMA foreign_key_check').all(),[]);same(db.query("SELECT name FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").all().map(x=>x.name),['pdf14_document','pdf14_page','semio_snapshot']);same(db.query('SELECT CAST(id AS TEXT) id,artifact_kind,standard,subset,CAST(schema_version AS TEXT) schema_version,native_encoding FROM semio_snapshot').all(),[{id:'1',artifact_kind:'s.stdio.pdf',standard:'1.4',subset:'*',schema_version:'1',native_encoding:Bun.argv.at(-1)}]);const word=value=>{const b=Buffer.alloc(8);b.writeDoubleLE(value);return b.readBigInt64LE().toString();};same(db.query('SELECT d.schema,CAST(p.ordinal AS TEXT) ordinal,p.width,p.height,p.text,CAST(p.width_bits AS TEXT) widthBits,CAST(p.height_bits AS TEXT) heightBits,p.width_class widthClass,p.height_class heightClass FROM pdf14_document d JOIN pdf14_page p ON p.document_id=d.id ORDER BY p.ordinal').all(),fixture.pages.map((p,i)=>({schema:fixture.schema,ordinal:String(i),width:p.width,height:p.height,text:p.text,widthBits:word(p.width),heightBits:word(p.height),widthClass:'finite',heightClass:'finite'})));const e=fixture.sqlEdit;db.query("UPDATE pdf14_page SET width=?,height=?,text=?,width_bits=CAST(? AS INTEGER),height_bits=CAST(? AS INTEGER),width_class='finite',height_class='finite' WHERE ordinal=?").run(e.width,e.height,e.text,word(e.width),word(e.height),e.page);same(db.query('PRAGMA foreign_key_check').all(),[]);await Bun.write(Bun.stdout,db.serialize());db.close();"#;
        let mut child = Command::new("bun").args(["-e", script, CORPUS, encoding]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
        child.stdin.take().unwrap().write_all(bytes).unwrap();
        let output = child.wait_with_output().unwrap();
        assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
        output.stdout
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_pdf14_populated_actual_owner_public_both_forms_and_independent_native_edit() {
        use semio_framework_os_kernel::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot};
        crate::register_sqlite_test_declaration();
        let dialect = semio_framework_artifact_reference::ArtifactDialect { artifact_kind: "s.stdio.pdf".into(), standard: "1.4".into(), subset: "*".into() };
        let expected = owner(false);
        let edited = owner(true);
        let limits = sqlite_snapshot::SqliteDatabaseLimits::default();
        for encoding in [sqlite_snapshot::SnapshotEncoding::Binary, sqlite_snapshot::SnapshotEncoding::Text] {
            let file = io_export_sqlite_snapshot(&dialect, &expected, encoding, limits, &mut |_| true).await.unwrap().value;
            assert_eq!(&file[..16], b"SQLite format 3\0");
            let restored = io_import_sqlite_snapshot::<PdfSnapshot>(&dialect, &file, limits, &mut |_| true).await.unwrap().value;
            assert_eq!(restored, expected);
            restored.retire_sqlite_snapshot();
            let raw = independent_edit(&file, match encoding { sqlite_snapshot::SnapshotEncoding::Binary => "binary", sqlite_snapshot::SnapshotEncoding::Text => "text" });
            let changed = io_import_sqlite_snapshot::<PdfSnapshot>(&dialect, &raw, limits, &mut |_| true).await.unwrap().value;
            assert_eq!(changed, edited);
            let native = crate::standards::v1_4::subsets::base::io::encode_pdf(&changed).unwrap();
            assert!(native.starts_with(b"%PDF-1.4"));
            let parsed = crate::standards::v1_4::subsets::base::io::decode_pdf(&native).unwrap();
            assert_eq!(parsed, edited);
            parsed.retire_sqlite_snapshot();
            changed.retire_sqlite_snapshot();
        }
        assert_eq!(expected, owner(false));
        expected.retire_sqlite_snapshot();
        edited.retire_sqlite_snapshot();
        eprintln!("[DEBUG] own14 real typed public binary text files full independent two-page SQL and native edit");
    }
}
