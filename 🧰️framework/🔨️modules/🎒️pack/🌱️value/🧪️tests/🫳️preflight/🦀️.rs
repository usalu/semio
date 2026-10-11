//! 🫳️ Actual core producers define exact forecasts, scratch requests and full refusal ownership.
use super::{encode_document,measure_document_borrowed,EncodeOptions};
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as S,BorrowedShape as H,DslField,FieldValue,JoinMode,RecordLayout,RecordSpec,native_encoding::FieldProjectionSource};
use semio_framework_dsl_record_derive::{DslRecord,DslScalar};
use semio_framework_value::{NativeEncodeControl,ValueError,ValueRefusalKind as K};

#[derive(Debug,PartialEq,DslRecord)]
struct CsvField{value:String,quoted:bool}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvRecord{fields:Vec<CsvField>}
#[derive(Debug,PartialEq,DslRecord)]
struct CsvOwner{schema:String,has_header:bool,records:Vec<CsvRecord>}
#[derive(Debug,PartialEq,DslScalar)]
enum Ending{Lf,Crlf}
#[derive(Debug,PartialEq,DslRecord)]
struct TsvOwner{schema:String,records:Vec<Vec<String>>,trailing_newline:bool,line_ending:Ending}
#[derive(Debug,PartialEq,DslRecord)]
struct Symbols{values:Vec<String>}

fn text()->H{H::Text}
fn text_list()->H{H::List(text)}
static CSV_FIELD:[F;2]=[F::new(0,"value",H::Text),F::new(1,"quoted",H::Bool)];
fn csv_field()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV_FIELD}}
fn csv_field_shape()->H{H::Record(csv_field)}
static CSV_RECORD:[F;1]=[F::new(0,"fields",H::List(csv_field_shape))];
fn csv_record()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV_RECORD}}
fn csv_record_shape()->H{H::Record(csv_record)}
static CSV:[F;3]=[F::new(0,"schema",H::Text),F::new(1,"has-header",H::Bool),F::new(2,"records",H::List(csv_record_shape))];
fn csv_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&CSV}}
static TSV:[F;4]=[F::new(0,"schema",H::Text),F::new(1,"records",H::List(text_list)),F::new(2,"trailing-newline",H::Bool),F::new(3,"line-ending",H::Enum(&[("lf",0),("crlf",1)]))];
fn tsv_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&TSV}}
static SYMBOLS:[F;1]=[F::new(0,"values",H::List(text))];
fn symbols_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&SYMBOLS}}

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
fn csv(row:&serde_json::Value)->CsvOwner{CsvOwner{schema:row["schema"].as_str().unwrap().into(),has_header:row["hasHeader"].as_bool().unwrap(),records:row["records"].as_array().unwrap().iter().map(|row|CsvRecord{fields:row.as_array().unwrap().iter().map(|field|CsvField{value:field["value"].as_str().unwrap().into(),quoted:field["quoted"].as_bool().unwrap()}).collect()}).collect()}}
fn tsv(row:&serde_json::Value)->TsvOwner{TsvOwner{schema:row["schema"].as_str().unwrap().into(),records:row["records"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|value|value.as_str().unwrap().into()).collect()).collect(),trailing_newline:row["trailingNewline"].as_bool().unwrap(),line_ending:match row["lineEnding"].as_str().unwrap(){"lf"=>Ending::Lf,"crlf"=>Ending::Crlf,_=>panic!("closed ending")}}}
fn owned<T:DslField>(source:&T)->(RecordSpec,semio_framework_dsl_record::RecordValue){let semio_framework_dsl_record::Shape::Record(spec)=T::shape()else{panic!("declared record")};let FieldValue::Record(record)=source.to_value()else{panic!("declared record")};((spec.ordinary)(),record)}
fn measure<T:FieldProjectionSource>(source:&T,spec:&S,pack:bool,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{if pack{let mut options=EncodeOptions::default();options.limits.max_file_len=maximum as u64;measure_document_borrowed(source,spec,&options,control)}else{semio_framework_dsl_record::measure_print_borrowed(source,spec,maximum,control)}}
fn failure<T:FieldProjectionSource>(source:&T,spec:&S,pack:bool,maximum:usize,control:&mut NativeEncodeControl<'_>,kind:K){
 let ((actual,diagnostic),requested,released)=crate::test_allocation::observe_backing(||{let error=measure(source,spec,pack,maximum,control).expect_err("actual borrowed refusal");let result=(error.kind,match &error.message { std::borrow::Cow::Borrowed(message) => {match error.kind{K::OwnershipLimit=>assert_eq!(*message,"native encoding ownership exceeds caller limit"),K::Canceled=>assert_eq!(*message,"native encoding canceled"),other=>panic!("unexpected literal refusal {other:?}")}0}, std::borrow::Cow::Owned(message) => {assert!(message.capacity()>0);message.capacity()} });drop(error);result});
 assert_eq!(actual,kind);assert_eq!(requested,released,"complete failed scratch and diagnostic release");assert!(requested<=control.owned_bytes().checked_add(diagnostic).unwrap(),"full requests include actual owned diagnostic");
}
fn parity<T:DslField>(source:&T,spec:&S,pack:bool,spill:bool){
 let (ordinary_spec,record)=owned(source);
 if pack{let mut allow=|_|true;let mut verify=NativeEncodeControl::new(0,&mut allow);assert_eq!(super::borrowed_preflight::borrowed_schema_hash(*spec,&mut verify).unwrap(),super::schema_hash(&ordinary_spec),"canonical structural graph digest matches actual producer");assert_eq!(verify.owned_bytes(),0);}
 let bytes=if pack{encode_document(&ordinary_spec,&record,&EncodeOptions::default()).unwrap()}else{semio_framework_dsl_record::print(&record,&ordinary_spec,JoinMode::Document).into_bytes()};
 let mut allow=|_|true;let mut control=NativeEncodeControl::new(if spill{usize::MAX}else{0},&mut allow);
 let (predicted,requested,released)=crate::test_allocation::observe_backing(||measure(source,spec,pack,usize::MAX,&mut control));
 assert_eq!(predicted.unwrap(),bytes.len(),"exact real canonical producer length");assert_eq!(requested,released,"successful traversal scratch fully released");assert_eq!(requested,control.owned_bytes(),"complete actual scratch requests");if spill&&pack{assert!(requested>0)}else{assert_eq!(requested,0)}
 let scratch=requested;let mut exact=NativeEncodeControl::new(scratch,&mut allow);let (predicted,requests,releases)=crate::test_allocation::observe_backing(||measure(source,spec,pack,bytes.len(),&mut exact));assert_eq!(predicted.unwrap(),bytes.len());assert_eq!(requests,scratch);assert_eq!(releases,scratch);assert_eq!(exact.owned_bytes(),scratch);
 let mut file_short=NativeEncodeControl::new(usize::MAX,&mut allow);failure(source,spec,pack,bytes.len()-1,&mut file_short,K::WorkLimit);
 if scratch>0{
  let mut zero=NativeEncodeControl::new(0,&mut allow);failure(source,spec,pack,usize::MAX,&mut zero,K::OwnershipLimit);assert_eq!(zero.owned_bytes(),0);
  let mut short=NativeEncodeControl::new(scratch-1,&mut allow);failure(source,spec,pack,usize::MAX,&mut short,K::OwnershipLimit);
  let mut cumulative=NativeEncodeControl::new(scratch*2-1,&mut allow);assert_eq!(measure(source,spec,pack,usize::MAX,&mut cumulative).unwrap(),bytes.len());let previous=cumulative.owned_bytes();failure(source,spec,pack,usize::MAX,&mut cumulative,K::OwnershipLimit);assert!(cumulative.owned_bytes()>=previous);
  let mut fired=false;let mut cancel=|_|{if crate::test_allocation::observed_requested_bytes().is_some_and(|bytes|bytes>0){fired=true;false}else{true}};let mut canceled=NativeEncodeControl::new(usize::MAX,&mut cancel);failure(source,spec,pack,usize::MAX,&mut canceled,K::Canceled);drop(canceled);assert!(fired,"real post-materialization scratch cancellation");
 }
 let mut reject=|_|false;let mut start=NativeEncodeControl::new(usize::MAX,&mut reject);failure(source,spec,pack,usize::MAX,&mut start,K::Canceled);assert_eq!(start.owned_bytes(),0);
 assert_eq!(source.to_value(),FieldValue::Record(record),"retained full owner unchanged");
}
fn law(pack:bool){
 let fixture=fixture();assert_eq!(fixture["contract"]["packCodec"],1);
 for row in fixture["csv"].as_array().unwrap(){parity(&csv(row),&csv_spec(),pack,false)}
 for row in fixture["tsv"].as_array().unwrap(){parity(&tsv(row),&tsv_spec(),pack,false)}
 let long=fixture["longText"]["text"].as_str().unwrap().repeat(fixture["longText"]["repeat"].as_u64().unwrap()as usize);
 let source=CsvOwner{schema:"stdio.csv".into(),has_header:false,records:vec![CsvRecord{fields:vec![CsvField{value:long,quoted:true}]}]};parity(&source,&csv_spec(),pack,false);
 let mut interior=false;let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|{if progress.completed>=256{interior=true;false}else{true}};let mut control=NativeEncodeControl::new(usize::MAX,&mut cancel);failure(&source,&csv_spec(),pack,usize::MAX,&mut control,K::Canceled);drop(control);assert!(interior,"real long payload interior boundary");
 let repeated=fixture["repeatedInlineSymbols"]["text"].as_str().unwrap();let count=fixture["repeatedInlineSymbols"]["repeat"].as_u64().unwrap()as usize;parity(&Symbols{values:vec![repeated.into();count]},&symbols_spec(),pack,false);
 let prefix=fixture["distinctSymbols"]["prefix"].as_str().unwrap();let count=fixture["distinctSymbols"]["count"].as_u64().unwrap();let repeat=fixture["distinctSymbols"]["repeat"].as_u64().unwrap();let mut values=Vec::new();for _ in 0..repeat{for index in 0..count{values.push(format!("{prefix}{index}"));}}parity(&Symbols{values},&symbols_spec(),pack,true);
}
#[test]
fn record_borrowed_preflight_text_exact_full_owners_and_cancellation(){law(false)}
#[test]
fn record_borrowed_preflight_pack_exact_compression_spill_and_cumulative_ownership(){law(true)}

type TableRow=(String,Option<bool>,Option<i64>,Option<u64>,Option<f64>,Option<u32>,Vec<String>);
struct TableStatements{rows:Vec<TableRow>,statements:Vec<(String,String)>}
static TABLE_ROW:[F;7]=[F::new(0,"text",H::Text),F::new(1,"flag",H::Bool),F::new(2,"signed",H::Int),F::new(3,"unsigned",H::UInt),F::new(4,"real",H::Float),F::new(5,"kind",H::Enum(&[("keep",0),("erase",1)])),F::new(6,"nested",H::List(text))];
fn table_row_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&TABLE_ROW}}
static STATEMENT_PAYLOAD:[F;1]=[F::new(0,"value",H::Text)];
fn append_spec()->S{S{keyword:Some("append"),layout:RecordLayout::Inline,fields:&STATEMENT_PAYLOAD}}
fn remove_spec()->S{S{keyword:Some("remove"),layout:RecordLayout::Inline,fields:&STATEMENT_PAYLOAD}}
static TABLE_STATEMENTS:[F;2]=[F::new(0,"rows",H::Table(table_row_spec)),F::new(1,"commands",H::Statements(&[("append",append_spec),("remove",remove_spec),("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk",remove_spec)]))];
fn table_statements_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&TABLE_STATEMENTS}}
fn table_row_ordinary()->RecordSpec{use semio_framework_dsl_record::{FieldSpec,Shape};RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"text",Shape::Text),FieldSpec::new(1,"flag",Shape::Bool),FieldSpec::new(2,"signed",Shape::Int),FieldSpec::new(3,"unsigned",Shape::UInt),FieldSpec::new(4,"real",Shape::Float),FieldSpec::new(5,"kind",Shape::Enum(vec![("keep".into(),0),("erase".into(),1)])),FieldSpec::new(6,"nested",Shape::List(Box::new(Shape::Text)))])}
fn statement_ordinary()->RecordSpec{RecordSpec::new(None,RecordLayout::Inline,vec![semio_framework_dsl_record::FieldSpec::new(0,"value",semio_framework_dsl_record::Shape::Text)])}
impl DslField for TableStatements{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{
  use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;
  let absent=||V::Absent;match path{
   []=>Ok(V::Record(&[0,1])),[0]=>Ok(V::List(self.rows.len())),[1]=>Ok(V::Statements(self.statements.len())),
   [0,row]if *row<self.rows.len()=>Ok(V::Record(&[0,1,2,3,4,5,6])),
   [0,row,column]if *row<self.rows.len()=>{let row=&self.rows[*row];Ok(match column{0=>V::Text(&row.0),1=>row.1.map(V::Bool).unwrap_or_else(absent),2=>row.2.map(V::Int).unwrap_or_else(absent),3=>row.3.map(V::UInt).unwrap_or_else(absent),4=>row.4.map(V::Float).unwrap_or_else(absent),5=>row.5.map(V::Enum).unwrap_or_else(absent),6=>V::List(row.6.len()),_=>return Err(semio_framework_dsl_record::native_encoding::projection_path_error())})},
   [0,row,6,index]if *row<self.rows.len()&&*index<self.rows[*row].6.len()=>Ok(V::Text(&self.rows[*row].6[*index])),
   [1,index]if *index<self.statements.len()=>Ok(V::Record(&[0])),
   [1,index,0]if *index<self.statements.len()=>Ok(V::Text(&self.statements[*index].1)),
   _=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())
  }
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path==[1]{self.statements.get(index).map(|row|row.0.as_str()).ok_or_else(semio_framework_dsl_record::native_encoding::projection_path_error)}else{Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
 fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Record(semio_framework_dsl_record::RecordSpecProducer{ordinary:table_statements_ordinary,encoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only")),decoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only"))})}
 fn to_value(&self)->FieldValue{use semio_framework_dsl_record::RecordValue;let mut root=RecordValue::default();let rows=self.rows.iter().map(|row|{let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(row.0.clone()));record.fields.insert(1,row.1.map(FieldValue::Bool).unwrap_or(FieldValue::Absent));record.fields.insert(2,row.2.map(FieldValue::Int).unwrap_or(FieldValue::Absent));record.fields.insert(3,row.3.map(FieldValue::UInt).unwrap_or(FieldValue::Absent));record.fields.insert(4,row.4.map(FieldValue::Float).unwrap_or(FieldValue::Absent));record.fields.insert(5,row.5.map(FieldValue::Enum).unwrap_or(FieldValue::Absent));record.fields.insert(6,FieldValue::List(row.6.iter().cloned().map(FieldValue::Text).collect()));FieldValue::Record(record)}).collect();root.fields.insert(0,FieldValue::List(rows));root.fields.insert(1,FieldValue::Statements(self.statements.iter().map(|(key,value)|{let mut record=RecordValue::default();record.fields.insert(0,FieldValue::Text(value.clone()));(key.clone(),record)}).collect()));FieldValue::Record(root)}
 fn from_value(_: &FieldValue)->Result<Self,String>{Err("oracle owner is immutable".into())}
}
fn table_statements_ordinary()->RecordSpec{
 use semio_framework_dsl_record::{FieldSpec,Shape,RecordSpecProducer};
 let row=RecordSpecProducer{ordinary:table_row_ordinary,encoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only")),decoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only"))};
 let statement=RecordSpecProducer{ordinary:statement_ordinary,encoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only")),decoding:|_|Err(ValueError::new(K::UnsupportedOwner,"oracle metadata is ordinary only"))};
 RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"rows",Shape::Table(row)),FieldSpec::new(1,"commands",Shape::Statements(vec![("append".into(),statement),("remove".into(),statement),("kkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkk".into(),statement)]))])
}
fn table_statements_fixture()->TableStatements{
 let fixture=fixture();let recipe=&fixture["tableStatements"];let sparse=recipe["sparseOrdinals"].as_array().unwrap();let count=recipe["rowCount"].as_u64().unwrap()as usize;let text=recipe["longText"]["prefix"].as_str().unwrap().repeat(recipe["longText"]["repeat"].as_u64().unwrap()as usize);
 let rows=(0..count).map(|index|{let present=!sparse.iter().any(|value|value.as_u64().unwrap()as usize==index);(if index==0{text.clone()}else{format!("row.雪.{index}")},present.then_some(index%2==0),present.then_some(if index%2==0{i64::MIN}else{i64::MAX}),present.then_some(u64::MAX-index as u64),present.then_some(if index%2==0{-0.0}else{index as f64}),present.then_some(index as u32%2),vec![String::new(),format!("nested.{index}")])}).collect();
 let statements=recipe["keywords"].as_array().unwrap().iter().zip(recipe["statementPayloads"].as_array().unwrap()).map(|(key,value)|(key.as_str().unwrap().into(),value.as_str().unwrap().into())).collect();
 TableStatements{rows,statements}
}
#[test]
fn record_borrowed_preflight_pack_table_and_statements_exact_ordinary_wire(){
 let source=table_statements_fixture();let count=source.rows.len();let(spec,record)=owned(&source);let bytes=encode_document(&spec,&record,&EncodeOptions::default()).unwrap();
 let script=concat!(include_str!("🔮️pack/🟦️.ts"),"\nconst actual=readBareClosedRecordPack(new Uint8Array(await Bun.stdin.arrayBuffer()));await Bun.write(Bun.stdout,JSON.stringify(actual));");
 use std::{io::Write,process::{Command,Stdio}};let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));let actual:serde_json::Value=serde_json::from_slice(&output.stdout).unwrap();assert_eq!(actual["0"]["tableRows"].as_array().unwrap().len(),count);
 for(index,row)in source.rows.iter().enumerate(){let actual=&actual["0"]["tableRows"][index];assert_eq!(actual["0"],row.0);assert_eq!(actual["1"],row.1.map(serde_json::Value::Bool).unwrap_or(serde_json::Value::Null));assert_eq!(actual["2"],row.2.map(|value|serde_json::Value::String(value.to_string())).unwrap_or(serde_json::Value::Null));assert_eq!(actual["3"],row.3.map(|value|serde_json::Value::String(value.to_string())).unwrap_or(serde_json::Value::Null));assert_eq!(actual["4"],row.4.map(|value|serde_json::json!({"bits":format!("{:016x}",value.to_bits())})).unwrap_or(serde_json::Value::Null));assert_eq!(actual["5"],row.5.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null));assert_eq!(actual["6"],serde_json::json!(row.6));}
 for(index,(key,value))in source.statements.iter().enumerate(){assert_eq!(actual["1"]["statements"][index]["keyword"],*key);assert_eq!(actual["1"]["statements"][index]["record"]["0"],*value);}
 parity(&source,&table_statements_spec(),true,false);eprintln!("[DEBUG] immutable sparse table bitmaps and forced statement symbols match ordinary Pack exactly");
}

#[derive(Debug,PartialEq,DslRecord)]
struct NestedOwner{payload:CsvField}
#[derive(Debug,PartialEq,DslRecord)]
struct PackedSigned{values:Vec<i64>}
static NESTED:[F;1]=[F::new(0,"payload",H::Record(csv_field))];
fn nested_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&NESTED}}
fn signed()->H{H::Int}
static SIGNED:[F;1]=[F::new(0,"values",H::List(signed))];
fn signed_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&SIGNED}}
fn independent_pack(bytes:&[u8])->serde_json::Value{
 use std::{io::Write,process::{Command,Stdio}};
 let script=concat!(include_str!("🔮️pack/🟦️.ts"),"\nconst actual=readBareClosedRecordPack(new Uint8Array(await Bun.stdin.arrayBuffer()));await Bun.write(Bun.stdout,JSON.stringify(actual));");
 let mut child=Command::new("bun").args(["-e",script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(bytes).unwrap();let output=child.wait_with_output().unwrap();assert!(output.status.success(),"{}",String::from_utf8_lossy(&output.stderr));serde_json::from_slice(&output.stdout).unwrap()
}
fn logical_depth<T:DslField>(source:&T,spec:&S,maximum:u16){
 let(ordinary,record)=owned(source);let mut options=EncodeOptions::default();options.limits.max_depth=maximum;let bytes=encode_document(&ordinary,&record,&options).unwrap();independent_pack(&bytes);
 let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);let(actual,requests,releases)=crate::test_allocation::observe_backing(||measure_document_borrowed(source,spec,&options,&mut control));assert_eq!(actual.unwrap(),bytes.len());assert_eq!((requests,releases,control.owned_bytes()),(0,0,0));
 options.limits.max_depth=maximum-1;assert!(encode_document(&ordinary,&record,&options).is_err());let mut control=NativeEncodeControl::new(0,&mut allow);let((kind,diagnostic),requests,releases)=crate::test_allocation::observe_backing(||{let refusal=measure_document_borrowed(source,spec,&options,&mut control).expect_err("one-short logical wire depth refuses");(refusal.kind,match &refusal.message { std::borrow::Cow::Borrowed(_) => 0, std::borrow::Cow::Owned(message) => message.capacity() })});assert_eq!(kind,K::DepthLimit);assert_eq!(requests,releases);assert_eq!(requests,diagnostic);assert_eq!(control.owned_bytes(),0);assert_eq!(source.to_value(),FieldValue::Record(record));
}
#[test]
fn record_borrowed_preflight_pack_nested_record_logical_depth(){
 let corpus=fixture();let recipe=&corpus["logicalDepth"];let source=NestedOwner{payload:CsvField{value:recipe["nestedText"].as_str().unwrap().into(),quoted:true}};logical_depth(&source,&nested_spec(),recipe["nestedRecord"].as_u64().unwrap()as u16);eprintln!("[DEBUG] nested Record has its ordinary bare frame at logical level two");
}
#[test]
fn record_borrowed_preflight_pack_packed_numeric_logical_depth(){
 let corpus=fixture();let recipe=&corpus["logicalDepth"];let source=PackedSigned{values:recipe["signedWords"].as_array().unwrap().iter().map(|word|word.as_str().unwrap().parse().unwrap()).collect()};logical_depth(&source,&signed_spec(),recipe["packedSignedList"].as_u64().unwrap()as u16);let(spec,record)=owned(&source);assert_eq!(independent_pack(&encode_document(&spec,&record,&EncodeOptions::default()).unwrap())["0"],recipe["signedWords"]);eprintln!("[DEBUG] mandatory packed numeric List stays at its ordinary logical value level");
}
#[test]
fn record_borrowed_preflight_pack_table_statements_logical_depth(){
 let source=table_statements_fixture();let corpus=fixture();logical_depth(&source,&table_statements_spec(),corpus["logicalDepth"]["tableStatements"].as_u64().unwrap()as u16);eprintln!("[DEBUG] table fallback and bare statement frames retain the ordinary logical depth");
}
fn reference_row_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&REFERENCE_ROW}}
static REFERENCE_ROW:[F;7]=[F::new(0,"text",H::Ref("document")),F::new(1,"flag",H::Bool),F::new(2,"signed",H::Int),F::new(3,"unsigned",H::UInt),F::new(4,"real",H::Float),F::new(5,"kind",H::Enum(&[("keep",0),("erase",1)])),F::new(6,"nested",H::List(text))];
static REFERENCE_ROOT:[F;2]=[F::new(0,"rows",H::Table(reference_row_spec)),TABLE_STATEMENTS[1]];
fn reference_row_ordinary()->RecordSpec{let mut spec=table_row_ordinary();spec.fields[0].shape=semio_framework_dsl_record::Shape::Ref("document");spec}
#[test]
fn record_borrowed_preflight_pack_long_forced_reference_column(){
 use semio_framework_dsl_record::{Shape,RecordSpecProducer};let source=table_statements_fixture();let(mut ordinary,record)=owned(&source);let borrowed=S{keyword:None,layout:RecordLayout::Inline,fields:&REFERENCE_ROOT};ordinary.fields[0].shape=Shape::Table(RecordSpecProducer{ordinary:reference_row_ordinary,encoding:|_|Err(ValueError::new(K::UnsupportedOwner,"ordinary reference oracle metadata")),decoding:|_|Err(ValueError::new(K::UnsupportedOwner,"ordinary reference oracle metadata"))});let bytes=encode_document(&ordinary,&record,&EncodeOptions::default()).expect("single long Ref table cell requires a forced symbol");assert_eq!(independent_pack(&bytes)["0"]["tableRows"][0]["0"],source.rows[0].0);let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);assert_eq!(super::borrowed_preflight::borrowed_schema_hash(borrowed,&mut control).unwrap(),super::schema_hash(&ordinary));let(actual,requests,releases)=crate::test_allocation::observe_backing(||measure_document_borrowed(&source,&borrowed,&EncodeOptions::default(),&mut control));assert_eq!(actual.unwrap(),bytes.len());assert_eq!((requests,releases,control.owned_bytes()),(0,0,0));eprintln!("[DEBUG] independent Node DEFLATE readback preserves a single forced Ref larger than 128 bytes");
}

#[derive(Debug,PartialEq,DslRecord)]
struct AbsentOwner{value:Option<String>}
static ABSENT:[F;1]=[F::new(0,"value",H::Text)];
#[test]
fn record_borrowed_preflight_pack_absent_root_has_no_wire_child_depth(){
 let source=AbsentOwner{value:None};let(spec,record)=owned(&source);let corpus=fixture();let mut options=EncodeOptions::default();options.limits.max_depth=corpus["logicalDepth"]["rootAbsent"].as_u64().unwrap()as u16;let bytes=encode_document(&spec,&record,&options).unwrap();assert_eq!(independent_pack(&bytes),serde_json::json!({}));let borrowed=S{keyword:None,layout:RecordLayout::Inline,fields:&ABSENT};let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);let(actual,requests,releases)=crate::test_allocation::observe_backing(||measure_document_borrowed(&source,&borrowed,&options,&mut control));assert_eq!(actual.unwrap(),bytes.len());assert_eq!((requests,releases,control.owned_bytes()),(0,0,0));assert_eq!(source.to_value(),FieldValue::Record(record));eprintln!("[DEBUG] omitted absent root fields have no wire child or caller depth debit");
}

struct SpatialOwner{coordinate:[f64;3],direction:[f64;3]}
static SPATIAL:[F;2]=[F::new(0,"coordinate",H::Coord(3)),F::new(1,"direction",H::Dir)];
fn spatial_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&SPATIAL}}
fn spatial_ordinary()->RecordSpec{use semio_framework_dsl_record::{FieldSpec,Shape};RecordSpec::new(None,RecordLayout::Inline,vec![FieldSpec::new(0,"coordinate",Shape::Coord(3)),FieldSpec::new(1,"direction",Shape::Dir)])}
impl DslField for SpatialOwner{
 fn projection_view(&self,path:&[usize])->Result<semio_framework_dsl_record::native_encoding::FieldProjectionView<'_>,ValueError>{use semio_framework_dsl_record::native_encoding::FieldProjectionView as V;match path{[]=>Ok(V::Record(&[0,1])),[0]|[1]=>Ok(V::Tuple(3)),[0,index]if *index<3=>Ok(V::Float(self.coordinate[*index])),[1,index]if *index<3=>Ok(V::Float(self.direction[*index])),_=>Err(semio_framework_dsl_record::native_encoding::projection_path_error())}}
 fn shape()->semio_framework_dsl_record::Shape{semio_framework_dsl_record::Shape::Record(semio_framework_dsl_record::RecordSpecProducer{ordinary:spatial_ordinary,encoding:|_|Err(ValueError::new(K::UnsupportedOwner,"ordinary oracle shape")),decoding:|_|Err(ValueError::new(K::UnsupportedOwner,"ordinary oracle shape"))})}
 fn to_value(&self)->FieldValue{let mut value=semio_framework_dsl_record::RecordValue::default();value.fields.insert(0,FieldValue::Tuple(self.coordinate.into_iter().map(FieldValue::Float).collect()));value.fields.insert(1,FieldValue::Tuple(self.direction.into_iter().map(FieldValue::Float).collect()));FieldValue::Record(value)}
 fn from_value(_: &FieldValue)->Result<Self,String>{Err("immutable oracle source".into())}
}
#[test]
fn record_borrowed_preflight_pack_authored_coord_and_direction_exact_original_words(){
 let corpus=fixture();let recipe=&corpus["spatial"];let words=|key:&str|->[f64;3]{recipe[key]["words"].as_array().unwrap().iter().map(|word|f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap())).collect::<Vec<_>>().try_into().unwrap()};
 let source=SpatialOwner{coordinate:words("coordinate"),direction:words("direction")};let(spec,record)=owned(&source);let bytes=encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let readback=independent_pack(&bytes);
 for(id,key)in[("0","coordinate"),("1","direction")]{let expected=serde_json::Value::Array(recipe[key]["words"].as_array().unwrap().iter().map(|word|serde_json::json!({"bits":word.as_str().unwrap()})).collect());assert_eq!(readback[id],expected);}
 parity(&source,&spatial_spec(),true,false);eprintln!("[DEBUG] canonical borrowed Coord and Dir retain exact ordinary packed-f64 words with zero scratch");
}

#[derive(Debug,PartialEq,DslRecord)]
struct IntrinsicOwner{value:semio_framework_value::DslValue}
static INTRINSIC:[F;1]=[F::new(0,"value",H::Value)];
fn intrinsic_spec()->S{S{keyword:None,layout:RecordLayout::Inline,fields:&INTRINSIC}}
/// 🧿️ The neutral intrinsic corpus forecasts exact actual Pack bytes with owned cancellation and no source mirror.
#[test]
fn record_borrowed_preflight_intrinsic_values_match_canonical_bytes_and_controls(){
 let fixture=fixture();for row in fixture["intrinsic"].as_array().unwrap(){let value=semio_framework_value::DslValue::from(row);assert_eq!(serde_json::Value::from(&value),*row);let source=IntrinsicOwner{value};let(spec,record)=owned(&source);let bytes=encode_document(&spec,&record,&EncodeOptions::default()).unwrap();let(decoded,_)=super::decode_document(&bytes,&spec,&Default::default()).unwrap();assert_eq!(decoded,record);let Some(FieldValue::Value(decoded))=decoded.fields.get(&0)else{panic!("canonical intrinsic field tag")};assert_eq!(serde_json::Value::from(decoded),*row);let text=semio_framework_dsl_record::print(&record,&spec,JoinMode::Document);let decoded=semio_framework_dsl_record::parse_exact(&text,&spec,&Default::default()).unwrap();assert_eq!(decoded,record,"Text preserves intrinsic field and exact number kinds");let Some(FieldValue::Value(decoded))=decoded.fields.get(&0)else{panic!("Text intrinsic field tag")};assert_eq!(serde_json::Value::from(decoded),*row);parity(&source,&intrinsic_spec(),true,false);parity(&source,&intrinsic_spec(),false,false);}
 let prefix=fixture["distinctSymbols"]["prefix"].as_str().unwrap();let count=fixture["distinctSymbols"]["count"].as_u64().unwrap();let source=IntrinsicOwner{value:semio_framework_value::DslValue::Array((0..count).map(|index|semio_framework_value::DslValue::String(format!("{prefix}{index}"))).collect())};parity(&source,&intrinsic_spec(),true,true);
 eprintln!("[DEBUG] Neutral intrinsic null/bool/numbers/UTF8/nested arrays/objects match actual canonical Pack and Text lengths, exact bounds, spill ownership and cancellation");
}
