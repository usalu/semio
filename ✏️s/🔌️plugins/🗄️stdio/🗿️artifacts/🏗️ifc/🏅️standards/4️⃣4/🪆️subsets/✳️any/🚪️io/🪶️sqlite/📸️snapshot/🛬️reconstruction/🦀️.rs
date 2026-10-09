//! 🛬️ IFC4 reconstruction follows paid semantic row indexes and exclusive ordered ownership.
use super::{identity,null,unsigned,sort_paid,REAL,IfcSnapshot,IfcHeader,IfcEntity,IfcComplexType,IfcValue};
use crate::standards::v4::subsets::any::io::sqlite::snapshot::native;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,artifact::{validate_ieee754_row,ieee754_is_null,read_binary64}};
use semio_framework_value::NativeDecodeControl;
use semio_framework_dsl_record::__rt::DecodedFieldOwner;
use std::ops::Range;
const TABLES:[(&str,usize);11]=[("ifc_document",3),("ifc_header",1),("ifc_file_description_argument",4),("ifc_file_name_argument",4),("ifc_file_schema_argument",4),("ifc_entity",5),("ifc_complex_type",4),("ifc_argument",5),("ifc_value",11),("ifc_aggregate_element",4),("ifc_typed_argument",4)];
fn invalid(message:&'static str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
#[derive(Clone, Copy)]
struct Relation{parent:(u8,i64),ordinal:i64,id:i64}
struct Table<'a>{rows:Vec<&'a SqliteRow>,seen:Vec<bool>,relations:Vec<Relation>}
impl Table<'_>{
 fn index(&self,id:i64)->Result<usize,ValueError>{self.rows.binary_search_by_key(&id,|row|row.rowid).map_err(|_|invalid("dangling IFC4 relational ownership"))}
 fn row(&self,id:i64)->Result<&SqliteRow,ValueError>{Ok(self.rows[self.index(id)?])}
 fn range(&self,parent:(u8,i64))->Range<usize>{self.relations.partition_point(|edge|edge.parent<parent)..self.relations.partition_point(|edge|edge.parent<=parent)}
}
struct Catalog<'a>{tables:[Table<'a>;11]}
impl<'a> Catalog<'a>{
 fn new(database:&'a SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{
 let mut tables=std::array::from_fn(|_|Table{rows:Vec::new(),seen:Vec::new(),relations:Vec::new()});control.begin_stage(0)?;
 for(index,(name,width))in TABLES.iter().copied().enumerate(){let source=&database.table(name)?.rows;let table=&mut tables[index];table.rows=control.allocate_vec(source.len())?;table.seen=control.allocate_vec(source.len())?;table.seen.resize(source.len(),false);
 let relational=matches!(index,2..=7|9..=10);if relational{table.relations=control.allocate_vec(source.len())?;}
 for row in source{identity(row,width)?;table.rows.push(row);if relational{let(parent,ordinal)=if index==7{let parent=match(&row.values[1],&row.values[2]){(SqliteValue::Integer(id),SqliteValue::Null)=>(0,*id),(SqliteValue::Null,SqliteValue::Integer(id))=>(1,*id),_=>return Err(invalid("IFC4 argument must have exactly one owner"))};(parent,row.integer(3)?)}else{((0,row.integer(1)?),row.integer(2)?)};
 if parent.1<=0||ordinal<0{return Err(invalid("invalid IFC4 relationship owner or ordinal"));}table.relations.push(Relation{parent,ordinal,id:row.rowid});}control.step()?;}
 sort_paid(&mut table.rows,|left,right|{control.step()?;Ok(left.rowid.cmp(&right.rowid))})?;for pair in table.rows.windows(2){if pair[0].rowid==pair[1].rowid{return Err(invalid("duplicate IFC4 relational row identity"));}control.step()?;}
 sort_paid(&mut table.relations,|left,right|{control.step()?;Ok((left.parent,left.ordinal).cmp(&(right.parent,right.ordinal)))})?;let mut parent=None;let mut ordinal=0i64;for edge in &table.relations{if parent!=Some(edge.parent){parent=Some(edge.parent);ordinal=0;}if edge.ordinal!=ordinal{return Err(invalid("IFC4 relationship ordinals must be contiguous and unique"));}ordinal=ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"IFC4 ordinal overflow"))?;control.step()?;}
 }Ok(Self{tables})
 }
 fn take(&mut self,table:usize,id:i64)->Result<&'a SqliteRow,ValueError>{let index=self.tables[table].index(id)?;if std::mem::replace(&mut self.tables[table].seen[index],true){return Err(invalid("multiply owned IFC4 relational row"));}Ok(self.tables[table].rows[index])}
 fn edge(&self,table:usize,index:usize)->Relation{self.tables[table].relations[index]}
 fn ensure_owned(&self,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{for table in &self.tables{for seen in &table.seen{if !seen{return Err(invalid("unowned IFC4 relational row"));}control.step()?;}}Ok(())}
}
struct Forest{state:Vec<u8>,values:DecodedFieldOwner<Vec<Option<IfcValue>>>,pending:Vec<(i64,bool)>}
impl Forest{
 fn new(count:usize,control:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut state=control.allocate_vec(count)?;state.resize(count,0);let mut values=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(count)?,native::close::<Vec<Option<IfcValue>>>);values.as_mut().resize_with(count,||None);let size=count.checked_mul(2).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"IFC4 value frontier size overflow"))?;Ok(Self{state,values,pending:control.allocate_vec(size)?})}
 fn push(&mut self,item:(i64,bool))->Result<(),ValueError>{if self.pending.len()==self.pending.capacity(){return Err(invalid("IFC4 value frontier exceeds unique ownership census"));}self.pending.push(item);Ok(())}
 fn pop(&mut self,catalog:&Catalog<'_>,id:i64)->Result<IfcValue,ValueError>{self.values.as_mut()[catalog.tables[8].index(id)?].take().ok_or_else(||invalid("missing or multiply owned IFC4 value"))}
 fn children(&mut self,catalog:&Catalog<'_>,table:usize,id:i64,control:&mut NativeDecodeControl<'_>)->Result<Vec<IfcValue>,ValueError>{let range=catalog.tables[table].range((0,id));let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<IfcValue>>);for edge in range{let row=catalog.tables[table].row(catalog.edge(table,edge).id)?;output.as_mut().push(self.pop(catalog,row.integer(3)?)?);control.step()?;}Ok(output.take())}
 fn value(&mut self,catalog:&mut Catalog<'_>,id:i64,entities:&[(i64,u64)],words:&[u64],control:&mut NativeDecodeControl<'_>)->Result<IfcValue,ValueError>{
 if !self.pending.is_empty(){return Err(invalid("unfinished IFC4 value frontier"));}self.push((id,false))?;
 while let Some((id,exit))=self.pending.pop(){control.step()?;let index=catalog.tables[8].index(id)?;if !exit{if self.state[index]!=0{return Err(invalid("cyclic or multiply owned IFC4 value"));}self.state[index]=1;let row=catalog.take(8,id)?;let kind=row.text(1)?;let aggregate=catalog.tables[9].range((0,id));let typed=catalog.tables[10].range((0,id));if(kind!="aggregate"&&!aggregate.is_empty())||(kind!="typedValue"&&!typed.is_empty()){return Err(invalid("IFC4 value relationship differs from its variant"));}
 self.push((id,true))?;for edge in aggregate.rev(){let row=catalog.take(9,catalog.edge(9,edge).id)?;self.push((row.integer(3)?,false))?;control.step()?;}for edge in typed.rev(){let row=catalog.take(10,catalog.edge(10,edge).id)?;self.push((row.integer(3)?,false))?;control.step()?;}continue;}
 let row=catalog.tables[8].rows[index];validate_ieee754_row(row,9,REAL)?;let kind=row.text(1)?;let active=match kind{"integer"=>Some(2),"real"=>Some(3),"string"=>Some(4),"enum"=>Some(5),"reference"=>Some(6),"typedValue"=>Some(8),"unset"|"derived"|"aggregate"=>None,_=>return Err(invalid("unknown IFC4 value kind"))};
 for column in 2..9{if Some(column)!=active&&!(kind=="reference"&&column==7){null(row,column)?;}}if kind!="real"&&!ieee754_is_null(row,3,REAL)?{return Err(invalid("inactive IFC4 real companions"));}
 let value=match kind{"unset"=>IfcValue::Unset,"derived"=>IfcValue::Derived,"integer"=>IfcValue::Integer(row.integer(2)?),"real"=>IfcValue::Real(read_binary64(row,3,REAL)?),"string"=>IfcValue::String(control.copy_text(row.text(4)?)?),"enum"=>IfcValue::Enum(control.copy_text(row.text(5)?)?),
 "reference"=>{let word=unsigned(row.text(6)?)?;match &row.values[7]{SqliteValue::Null=>{if words.binary_search(&word).is_ok(){return Err(invalid("resolved IFC4 reference lacks its relationship"));}},SqliteValue::Integer(entity)=>{let actual=entities.binary_search_by_key(entity,|item|item.0).ok().map(|index|entities[index].1);if actual!=Some(word){return Err(invalid("IFC4 reference word differs from its related entity"));}},_=>return Err(invalid("invalid IFC4 resolved reference storage"))}IfcValue::Reference(word)},
 "aggregate"=>IfcValue::Aggregate(self.children(catalog,9,id,control)?),"typedValue"=>{let name=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.copy_text(row.text(8)?)?,native::close::<String>);let items=self.children(catalog,10,id,control)?;IfcValue::TypedValue{name:name.take(),items}},_=>unreachable!()};
 self.state[index]=2;self.values.as_mut()[index]=Some(value);
 }self.pop(catalog,id)
 }
 fn arguments(&mut self,catalog:&mut Catalog<'_>,table:usize,parent:(u8,i64),column:usize,entities:&[(i64,u64)],words:&[u64],control:&mut NativeDecodeControl<'_>)->Result<Vec<IfcValue>,ValueError>{let range=catalog.tables[table].range(parent);let mut output=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(control.allocate_vec(range.len())?,native::close::<Vec<IfcValue>>);for edge in range{let row=catalog.take(table,catalog.edge(table,edge).id)?;output.as_mut().push(self.value(catalog,row.integer(column)?,entities,words,control)?);control.step()?;}Ok(output.take())}
}
fn reconstruct(database:&SqliteDatabase,control:&mut NativeDecodeControl<'_>)->Result<IfcSnapshot,ValueError>{
 let mut catalog=Catalog::new(database,control)?;if catalog.tables[0].rows.len()!=1||catalog.tables[1].rows.len()!=1{return Err(invalid("IFC4 must own one document and one header"));}let document=catalog.take(0,catalog.tables[0].rows[0].rowid)?;let header=catalog.take(1,document.integer(2)?)?;
 let mut result=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(IfcSnapshot{schema:String::new(),header:IfcHeader{file_description:Vec::new(),file_name:Vec::new(),file_schema:Vec::new()},entities:Vec::new()},native::close::<IfcSnapshot>);let out=result.as_mut();out.schema=control.copy_text(document.text(1)?)?;
 let mut entities=control.allocate_vec(catalog.tables[5].rows.len())?;let mut words=control.allocate_vec(catalog.tables[5].rows.len())?;for row in &catalog.tables[5].rows{let word=unsigned(row.text(3)?)?;entities.push((row.rowid,word));words.push(word);control.step()?;}sort_paid(&mut words,|left,right|{control.step()?;Ok(left.cmp(right))})?;for pair in words.windows(2){if pair[0]==pair[1]{return Err(invalid("duplicate IFC4 instance identifier"));}control.step()?;}
 let mut forest=Forest::new(catalog.tables[8].rows.len(),control)?;out.header.file_description=forest.arguments(&mut catalog,2,(0,header.rowid),3,&entities,&words,control)?;out.header.file_name=forest.arguments(&mut catalog,3,(0,header.rowid),3,&entities,&words,control)?;out.header.file_schema=forest.arguments(&mut catalog,4,(0,header.rowid),3,&entities,&words,control)?;
 let range=catalog.tables[5].range((0,document.rowid));out.entities=control.allocate_vec(range.len())?;for edge in range{let row=catalog.take(5,catalog.edge(5,edge).id)?;let word=unsigned(row.text(3)?)?;let mut entity=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(IfcEntity{id:word,name:control.copy_text(row.text(4)?)?,args:Vec::new(),complex:Vec::new()},native::close::<IfcEntity>);
 entity.as_mut().args=forest.arguments(&mut catalog,7,(0,row.rowid),4,&entities,&words,control)?;let range=catalog.tables[6].range((0,row.rowid));entity.as_mut().complex=control.allocate_vec(range.len())?;
 for edge in range{let row=catalog.take(6,catalog.edge(6,edge).id)?;let mut complex=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(IfcComplexType{name:control.copy_text(row.text(3)?)?,args:Vec::new()},native::close::<IfcComplexType>);complex.as_mut().args=forest.arguments(&mut catalog,7,(1,row.rowid),4,&entities,&words,control)?;entity.as_mut().complex.push(complex.take());control.step()?;}out.entities.push(entity.take());control.step()?;}
 catalog.ensure_owned(control)?;if forest.values.as_mut().iter().any(Option::is_some){return Err(invalid("unowned IFC4 typed value"));}control.checkpoint()?;Ok(result.take())
}
pub(super) fn read(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<IfcSnapshot,ValueError>{control.allocation_stage(SqliteSnapshotPhase::ReconstructSnapshot,|remaining,progress,allocation|{let mut callback=|event:semio_framework_value::native_decoding::NativeDecodeProgress|progress(event.completed,event.total);let mut native_allocation=|request:semio_framework_value::native_decoding::NativeDecodeAllocation|allocation(request.bytes);let mut native=NativeDecodeControl::new_forwarded(remaining,&mut callback,&mut native_allocation);let result=reconstruct(database,&mut native);(result,native.owned_bytes())})?}
