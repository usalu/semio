//! 🔤️ Preserves complete intrinsic occurrences across physical Text writers.
use super::*;

#[test]
fn retained_intrinsic_objects_preserve_all_literal_occurrences_and_words() {
    use semio_framework_value::NativeEncodeControl;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔤️occurrences/🔣️.json")).unwrap();
    let schema:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔤️occurrences/🧬️schema/🔣️.json")).unwrap();
    {use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e","import Ajv from 'ajv/dist/2020.js';import {Database} from 'bun:sqlite';import assert from 'node:assert/strict';const x=JSON.parse(await Bun.stdin.text());const validate=new Ajv({strict:true}).compile(x.schema);assert(validate(x.fixture));assert(!validate({...x.fixture,foreign:true}));const db=new Database(':memory:');try{db.exec('CREATE TABLE occurrences (ordinal INTEGER PRIMARY KEY, name TEXT NOT NULL);CREATE TABLE words (ordinal INTEGER PRIMARY KEY, bits BLOB NOT NULL CHECK(length(bits)=8));');for(const [i,key]of x.fixture.objectKeys.entries())db.query('INSERT INTO occurrences VALUES (?,?)').run(i,key);for(const [i,word]of x.fixture.words.entries()){const bytes=Buffer.alloc(8);bytes.writeBigUInt64BE(BigInt('0x'+word));db.query('INSERT INTO words VALUES (?,?)').run(i,bytes);}assert.deepEqual(db.query('SELECT name FROM occurrences ORDER BY ordinal').all().map(r=>r.name),x.fixture.objectKeys);assert.deepEqual(db.query('SELECT lower(hex(bits)) AS word FROM words ORDER BY ordinal').all().map(r=>r.word),x.fixture.words);}finally{db.close();}console.log('[DEBUG] independent AJV and SQLite retain duplicate keys and nine exact IEEE words');"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture}).to_string().as_bytes()).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));eprint!("{}",String::from_utf8_lossy(&output.stdout));}
    fn same(a:&DslValue,b:&DslValue)->bool {
        match(a,b) {
            (DslValue::Null,DslValue::Null)=>true,
            (DslValue::Bool(a),DslValue::Bool(b))=>a==b,
            (DslValue::Number(Number::UInt(a)),DslValue::Number(Number::UInt(b)))=>a==b,
            (DslValue::Number(Number::Int(a)),DslValue::Number(Number::Int(b)))=>a==b,
            (DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>a.to_bits()==b.to_bits(),
            (DslValue::String(a),DslValue::String(b))=>a==b,
            (DslValue::Bytes(a),DslValue::Bytes(b))=>a==b,
            (DslValue::Array(a),DslValue::Array(b))=>a.len()==b.len()&&a.iter().zip(b).all(|(a,b)|same(a,b)),
            (DslValue::Object(a),DslValue::Object(b))=>a.len()==b.len()&&a.iter().zip(b).all(|((ak,av),(bk,bv))|ak==bk&&same(av,bv)),
            _=>false,
        }
    }
    for word in fixture["words"].as_array().unwrap() {
        let bits=u64::from_str_radix(word.as_str().unwrap(),16).unwrap();
        let value=DslValue::Object(vec![
            ("z".into(),DslValue::Null),
            ("a".into(),DslValue::Bool(true)),
            ("z".into(),DslValue::Number(Number::UInt(u64::MAX))),
            ("int".into(),DslValue::Number(Number::Int(i64::MIN))),
            ("positiveInt".into(),DslValue::Number(Number::Int(1))),
            ("float".into(),DslValue::Number(Number::Float(f64::from_bits(bits)))),
            ("text".into(),DslValue::String("NUL\0 世界".into())),
            ("bytes".into(),DslValue::Bytes(vec![0,1,127,128,255])),
            ("array".into(),DslValue::Array(vec![DslValue::Array(vec![]),DslValue::Object(vec![])])),
            ("object".into(),DslValue::Object(vec![("z".into(),DslValue::String("first".into())),("a".into(),DslValue::Null),("z".into(),DslValue::String("last".into()))])),
        ]);
        let DslValue::Object(entries)=&value else {unreachable!()};
        assert_eq!(entries.iter().map(|(key,_)|key.as_str()).collect::<Vec<_>>(),fixture["objectKeys"].as_array().unwrap().iter().map(|key|key.as_str().unwrap()).collect::<Vec<_>>());
        let spec=||RecordSpec::new(Some("document"),RecordLayout::Inline,vec![FieldSpec::new(0,"value",Shape::Value)]);
        let source=||RecordValue{fields:[(0,FieldValue::Value(value.clone()))].into_iter().collect()};
        let expected=print(&source(),&spec(),JoinMode::Document);
        let mut accepted=|_|true;
        let controlled=print_controlled(&source(),&spec(),JoinMode::Document,1000000,&mut NativeEncodeControl::new(10000000,&mut accepted)).unwrap();
        assert_eq!(controlled,expected);
        let grants=fixture["grants"].as_array().unwrap().iter().map(|grant|grant.as_u64().unwrap()as usize).chain([usize::MAX]);
        for grant in grants {
            let mut writer=RetainedRecordWriter::new(source(),spec(),JoinMode::Document,1000000);
            let mut accepted=|_|true;
            let mut control=NativeEncodeControl::new(10000000,&mut accepted);
            let mut turns=0;
            let actual=loop {
                let before=writer.progress();
                assert!(writer.step(0,&mut control).unwrap().is_none());
                assert_eq!(writer.progress(),before);
                turns+=1;assert!(turns<100000);
                if let Some(text)=writer.step(grant,&mut control).unwrap(){break text;}
            };
            assert_eq!(actual,expected);
            let parsed=parse_exact(&actual,&spec(),&ParseOptions::default()).unwrap();
            let FieldValue::Value(restored)=parsed.get(0).expect("intrinsic field is absent") else {panic!("intrinsic field changed shape")};
            assert!(same(restored,&value),"complete occurrence or IEEE word changed: {bits:016x}");
        }
    }
    eprintln!("[DEBUG] intrinsic Object order and duplicate values preserved across all Text writers and grants");
}
