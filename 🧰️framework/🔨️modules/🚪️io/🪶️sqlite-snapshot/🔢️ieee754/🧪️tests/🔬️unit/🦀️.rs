//! 🔢️ Authored IEEE 754 field tests of the artifact snapshot reader.
use super::*;
use super::super::{SqliteDatabaseLimits,export_sqlite_database,import_sqlite_database};
const SQL:&str="CREATE TABLE typed_scalar (id INTEGER PRIMARY KEY, value64 REAL, value32 REAL, value64_ieee754_bits INTEGER, value64_numeric_class TEXT, value32_ieee754_bits INTEGER, value32_numeric_class TEXT)";
const COLUMNS:&[FloatColumn]=&[FloatColumn::Binary64(1),FloatColumn::Binary32(2)];
fn bits()->(Vec<u64>,Vec<u32>){(vec![0,0x8000000000000000,1,0x000fffffffffffff,0x7ff0000000000000,0xfff0000000000000,0x7ff8000000000011,0x7ff0000000000007],vec![0,0x80000000,1,0x007fffff,0x7f800000,0xff800000,0x7fc00011,0x7f800007])}
fn fixture()->SqliteDatabase{let mut callback=|_|true;let mut control=SqliteSnapshotControl::new(&mut callback,SqliteDatabaseLimits::default());let mut projection=Projection::new(SQL,&mut control).unwrap();let(a,b)=bits();for(a,b)in a.into_iter().zip(b){insert_ieee754(&mut projection,"typed_scalar",&[Cell::Real(f64::from_bits(a)),Cell::Float32(f32::from_bits(b))],COLUMNS).unwrap();}insert_ieee754(&mut projection,"typed_scalar",&[Cell::Null,Cell::Null],COLUMNS).unwrap();projection.finish().unwrap()}
fn law(db:&SqliteDatabase){let(a,b)=bits();for(row,(a,b))in db.table("typed_scalar").unwrap().rows.iter().zip(a.into_iter().zip(b)){let row=FloatRow::new(row,COLUMNS).unwrap();assert_eq!(row.real(1).unwrap().to_bits(),a);assert_eq!(row.binary32(2).unwrap().to_bits(),b);assert!(!row.is_null(1).unwrap());assert!(!row.is_null(2).unwrap());}let row=FloatRow::new(db.table("typed_scalar").unwrap().rows.last().unwrap(),COLUMNS).unwrap();assert!(row.is_null(1).unwrap());assert!(row.is_null(2).unwrap());}
#[test]
fn authored_ieee_fields_preserve_signed_zero_infinities_subnormals_and_nan_payloads(){let db=fixture();law(&db);let bytes=export_sqlite_database(&db,SqliteDatabaseLimits::default(),&mut |_|true).unwrap();law(&import_sqlite_database(&bytes,SqliteDatabaseLimits::default(),&mut |_|true).unwrap());}
#[test]
fn authored_ieee_fields_reject_inconsistent_class_bits_query_and_optional_presence(){let original=fixture();for change in 0..4{let mut db=original.clone();let row=&mut db.table_mut("typed_scalar").unwrap().rows[0];match change{0=>row.values[1]=SqliteValue::Real(1.0),1=>row.values[4]=SqliteValue::Text("nan".into()),2=>row.values[5]=SqliteValue::Integer(i64::MAX),_=>row.values[3]=SqliteValue::Null}assert!(FloatRow::new(row,COLUMNS).is_err());}}
#[test]
fn authored_ieee_integer_queries_reject_rounding_against_independent_sqlite(){
    use std::io::Write;
    use std::process::{Command,Stdio};
    let input=include_str!("../🧫️fixtures/🎯️integer-query.json");
    let script="import{Database}from'bun:sqlite';const{cases}=await Bun.stdin.json(),db=new Database(':memory:',{safeIntegers:true}),view=new DataView(new ArrayBuffer(8));db.run('CREATE TABLE exact_integer(id INTEGER PRIMARY KEY,value)');for(const[i,c]of cases.entries()){const value=BigInt(c.integer);db.run('INSERT INTO exact_integer VALUES(?,?)',[i+1,value]);if(db.query('SELECT value FROM exact_integer WHERE id=?').get(i+1).value!==value)throw Error('integer identity');view.setBigUint64(0,BigInt('0x'+c.binary64Bits));if((BigInt(view.getFloat64(0))===value)!==c.accept64)throw Error('binary64 oracle');view.setUint32(0,parseInt(c.binary32Bits,16));if((BigInt(view.getFloat32(0))===value)!==c.accept32)throw Error('binary32 oracle');}db.close();await Bun.write(Bun.stdout,String(cases.length));";
    let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(input.as_bytes()).unwrap();
    let output=child.wait_with_output().unwrap();
    assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));
    assert_eq!(String::from_utf8_lossy(&output.stdout).trim(),"8");
    eprintln!("[DEBUG] Independent SQLite and DataView validated eight exact integer query cases");
    let fixture:serde_json::Value=serde_json::from_str(input).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let integer=case["integer"].as_str().unwrap().parse::<i64>().unwrap();
        let word64=u64::from_str_radix(case["binary64Bits"].as_str().unwrap(),16).unwrap();
        let word32=u32::from_str_radix(case["binary32Bits"].as_str().unwrap(),16).unwrap();
        let row=SqliteRow{rowid:1,values:vec![SqliteValue::Integer(1),SqliteValue::Integer(integer),SqliteValue::Integer(integer),SqliteValue::Integer(word64 as i64),SqliteValue::Text("finite".into()),SqliteValue::Integer(i64::from(word32)),SqliteValue::Text("finite".into())]};
        assert_eq!(read_binary64(&row,1,COLUMNS).is_ok(),case["accept64"].as_bool().unwrap(),"binary64 query {integer}");
        assert_eq!(read_binary32(&row,2,COLUMNS).is_ok(),case["accept32"].as_bool().unwrap(),"binary32 query {integer}");
    }
}
#[test]
fn authored_ieee_fields_are_independently_queryable_sqlite_scalars(){use std::io::Write;use std::process::{Command,Stdio};let bytes=export_sqlite_database(&fixture(),SqliteDatabaseLimits::default(),&mut |_|true).unwrap();let script="import{Database}from'bun:sqlite';const db=Database.deserialize(new Uint8Array(await Bun.stdin.arrayBuffer()));if(db.query('PRAGMA integrity_check').get().integrity_check!=='ok')throw Error('integrity');if(db.query(\"SELECT count(*) AS n FROM typed_scalar WHERE value64_numeric_class='nan' AND value64 IS NULL AND typeof(value64_ieee754_bits)='integer'\").get().n!==2)throw Error('NaN scalar fields');if(db.query(\"SELECT count(*) AS n FROM typed_scalar WHERE value32_numeric_class='finite' AND typeof(value32)='real'\").get().n!==4)throw Error('finite scalars');await Bun.write(Bun.stdout,db.serialize());db.close();";let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));law(&import_sqlite_database(&output.stdout,SqliteDatabaseLimits::default(),&mut |_|true).unwrap());}
