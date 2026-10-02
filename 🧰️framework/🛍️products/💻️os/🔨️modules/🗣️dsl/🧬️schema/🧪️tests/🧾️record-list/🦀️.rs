//! 🧾️ Canonical braces preserve ordered optional and empty record-list items.
use super::*;
macro_rules! ordinary_fixture_spec {
    ($spec:path) => { crate::os_dsl::RecordSpecProducer { ordinary: $spec, decoding: |_| Err("ordinary-only test metadata has no controlled construction".into()), encoding: |_| Err("ordinary-only test metadata has no controlled construction".into()) } };
}

use std::{io::Write,process::{Command,Stdio}};
fn row_spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"min",Shape::Float).optional(),FieldSpec::new(2,"max",Shape::Float).optional(),FieldSpec::new(3,"label",Shape::Text).optional(),FieldSpec::new(4,"children",Shape::List(Box::new(Shape::Record(ordinary_fixture_spec!(row_spec))))).optional()])}
fn document_spec()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"items",Shape::List(Box::new(Shape::Record(ordinary_fixture_spec!(row_spec)))))])}
fn row_json(record:&RecordValue)->serde_json::Value{let mut out=serde_json::Map::new();for(id,key)in[(1,"min"),(2,"max"),(3,"label"),(4,"children")]{match record.get(id){Some(FieldValue::Float(v))=>{out.insert(key.into(),serde_json::json!(v));},Some(FieldValue::Text(v))=>{out.insert(key.into(),serde_json::json!(v));},Some(FieldValue::List(v))=>{out.insert(key.into(),rows_json(v));},Some(FieldValue::Absent)|None=>{},other=>panic!("unexpected {other:?}")}}out.into()}
fn rows_json(rows:&[FieldValue])->serde_json::Value{serde_json::Value::Array(rows.iter().map(|v|match v{FieldValue::Record(row)=>row_json(row),_=>panic!("not a record")}).collect())}

fn controlled_row_spec<C:producer::NativeSchemaControl>(control:&mut C)->Result<RecordSpec,String>{
    let mut fields=control.allocate_vec(4)?;
    for(id,key,shape)in[(1,"min",Shape::Float),(2,"max",Shape::Float),(3,"label",Shape::Text),(4,"children",Shape::List(producer::boxed(Shape::Record(row_producer()),control)?))]{fields.push(producer::field(id,key,shape,control)?.optional());}
    producer::record(None,RecordLayout::Inline,fields,control)
}
fn row_producer()->RecordSpecProducer{RecordSpecProducer{ordinary:row_spec,decoding:|control|controlled_row_spec(control),encoding:|control|controlled_row_spec(control)}}

#[derive(dsl::DslRecord)]
struct OptionalBlockValue{value:String}
#[derive(dsl::DslRecord)]
struct OptionalBlockOwner{#[dsl(block)] child:Option<OptionalBlockValue>}

#[test]
fn list_record_optional_block_controlled_projection_preserves_absence(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["optionalBlocks"].as_array().unwrap(){
        let source=OptionalBlockOwner{child:case["value"].as_str().map(|value|OptionalBlockValue{value:value.into()})};let ordinary=source.__dsl_to_record();
        let mut accept=|_|true;let mut control=crate::os_dsl::NativeEncodeControl::new(65536,&mut accept);let controlled=source.__dsl_to_record_controlled(&mut control).unwrap();assert_eq!(controlled,ordinary);
        let spec=OptionalBlockOwner::__dsl_spec();assert_eq!(print(&ordinary,&spec,JoinMode::Inline),case["source"].as_str().unwrap());
        for mode in [JoinMode::Inline,JoinMode::Document]{let actual=print_controlled(&controlled,&spec,mode,65536,&mut control).unwrap();assert_eq!(actual,print(&ordinary,&spec,mode));assert_eq!(parse_exact(&actual,&spec,&ParseOptions::default()).unwrap(),ordinary);}
        let bytes=crate::os_pack::encode_record_body_controlled(&spec,&controlled,&crate::PackEncodeOptions::default(),&mut control).unwrap();assert_eq!(crate::os_pack::decode_record_body_exact(&bytes,&spec,&crate::PackDecodeOptions::default()).unwrap(),ordinary);
    }
}

#[test]
fn list_record_nested_table_controlled_text_matches_canonical_boundaries(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"items",Shape::List(Box::new(Shape::Table(row_producer()))))]);
    for case in fixture["cases"].as_array().unwrap(){
        let source=case["nestedTableSource"].as_str().unwrap();let record=parse_exact(source,&spec,&ParseOptions::default()).unwrap();
        let Some(FieldValue::List(tables))=record.get(1)else{panic!("table list")};let FieldValue::List(rows)=&tables[0]else{panic!("rows")};assert_eq!(rows_json(rows),case["rows"]);
        for mode in [JoinMode::Inline,JoinMode::Document]{
            let ordinary=print(&record,&spec,mode);if mode==JoinMode::Inline{assert_eq!(ordinary,source,"{}",case["id"]);}
            let actual=print_controlled(&record,&spec,mode,65536,&mut crate::os_dsl::NativeEncodeControl::new(65536,&mut |_|true)).unwrap();assert_eq!(actual,ordinary,"{}",case["id"]);assert_eq!(parse_exact(&actual,&spec,&ParseOptions::default()).unwrap(),record);
        }
    }
}
#[test]
fn list_record_boundaries_preserve_optional_empty_nested_order(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();for case in fixture["cases"].as_array().unwrap(){let source=case["source"].as_str().unwrap();let record=parse_exact(source,&document_spec(),&ParseOptions::default()).unwrap_or_else(|e|panic!("{}: {e}",case["id"]));let Some(FieldValue::List(rows))=record.get(1)else{panic!("missing rows")};assert_eq!(rows_json(rows),case["rows"],"{}",case["id"]);assert_eq!(print(&record,&document_spec(),JoinMode::Inline),source,"{}",case["id"]);let document=print(&record,&document_spec(),JoinMode::Document);assert_eq!(parse_exact(&document,&document_spec(),&ParseOptions::default()).unwrap(),record);}for source in fixture["invalid"].as_array().unwrap(){assert!(parse_exact(source.as_str().unwrap(),&document_spec(),&ParseOptions::default()).is_err(),"{source}");}}
#[test]
fn list_record_boundaries_match_independent_sqlite_and_json_schema(){let script=r#"import Ajv from 'ajv/dist/2020.js';import{Database}from'bun:sqlite';const{fixture,schema}=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).compile(schema);if(!validate(fixture))throw Error(JSON.stringify(validate.errors));const db=new Database(':memory:');db.exec('CREATE TABLE item(case_id TEXT,ordinal INTEGER,min REAL,max REAL,label TEXT,PRIMARY KEY(case_id,ordinal))');for(const c of fixture.cases){for(const [i,r]of c.rows.entries())db.query('INSERT INTO item VALUES(?,?,?,?,?)').run(c.id,i,r.min??null,r.max??null,r.label??null);const rows=db.query('SELECT ordinal,min,max,label FROM item WHERE case_id=? ORDER BY ordinal').all(c.id);if(rows.length!==c.rows.length)throw Error(c.id);for(const [i,row]of rows.entries())for(const key of ['min','max','label'])if(row[key]!== (c.rows[i][key]??null))throw Error(c.id+key);}db.exec('CREATE TABLE optional_block(id INTEGER PRIMARY KEY,value TEXT)');for(const [i,c]of fixture.optionalBlocks.entries())db.query('INSERT INTO optional_block VALUES(?,?)').run(i,c.value);const blocks=db.query('SELECT value FROM optional_block ORDER BY id').all();for(const [i,row]of blocks.entries())if(row.value!==fixture.optionalBlocks[i].value)throw Error('block presence');await Bun.write(Bun.stdout,'ok');"#;let input=serde_json::json!({"fixture":serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🔣️.json")).unwrap(),"schema":serde_json::from_str::<serde_json::Value>(include_str!("🧫️fixtures/🧬️schema/🔣️.json")).unwrap()});let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(input.to_string().as_bytes()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success(),"{}",String::from_utf8_lossy(&out.stderr));assert_eq!(out.stdout,b"ok");}
#[test]
fn list_record_boundaries_keep_scalar_and_nested_scalar_lists(){
    let cases=[(Shape::List(Box::new(Shape::Int)),"items=[ 1 2 3 ]"),(Shape::List(Box::new(Shape::List(Box::new(Shape::Int)))),"items=[ [ 1 2 ] [ ] [ 3 ] ]")];
    for(shape,source)in cases{let spec=RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(1,"items",shape)]);let record=parse_exact(source,&spec,&ParseOptions::default()).unwrap();assert_eq!(print(&record,&spec,JoinMode::Inline),source);}
}
