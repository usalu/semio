//! 🫴️ Paid Probe SQL buffers and semantic partials remain in the original receiving frame.
use super::*;
use semio_framework_value::{NativeDecodeControl,NativeEncodeControl,RetainedCloneGrant,RetainedCloneProgress};
use store::NativeSnapshotBodyWallet;
use std::{cmp::Ordering,mem::size_of};

pub(super) trait Port{
 fn control<R>(&mut self,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>)->R)->R;
 fn phase(&self)->SqliteSnapshotPhase;
 fn vector<T>(&mut self,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>;
 fn text(&mut self,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>;
 fn work(&mut self,depth:usize,copy:usize)->Result<(),ValueError>;
}
pub(super) struct Direct<'a,'b>{pub control:&'a mut SqliteSnapshotControl<'b>,pub phase:SqliteSnapshotPhase}
impl Port for Direct<'_,'_>{
 fn control<R>(&mut self,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>)->R)->R{operation(self.control)}
 fn phase(&self)->SqliteSnapshotPhase{self.phase}
 fn vector<T>(&mut self,count:usize,output:&mut Vec<T>,_:usize)->Result<(),ValueError>{*output=store::sqlite_snapshot::transfer::reserve(count,self.control)?;Ok(())}
 fn text(&mut self,text:&str,output:&mut String,_:usize)->Result<(),ValueError>{self.control.admit_allocation_bytes(text.len())?;output.try_reserve_exact(text.len()).map_err(|_|refusal(ValueRefusalKind::AllocationFailed,"Probe semantic text allocation failed"))?;let mut copied=0;while copied<text.len(){let mut end=copied.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[copied..end]);copied=end;self.control.checkpoint(self.phase,copied,text.len())?;}Ok(())}
 fn work(&mut self,_:usize,_:usize)->Result<(),ValueError>{self.control.checkpoint(self.phase,0,0)}
}
pub(super) struct Decode<'a,'b,'c>{pub control:&'a mut SqliteSnapshotControl<'b>,pub native:&'a mut NativeDecodeControl<'c>,pub body:&'a mut NativeSnapshotBodyWallet}
pub(super) struct Encode<'a,'b,'c>{pub control:&'a mut SqliteSnapshotControl<'b>,pub native:&'a mut NativeEncodeControl<'c>,pub body:&'a mut NativeSnapshotBodyWallet}
fn bytes<T>(count:usize)->Result<usize,ValueError>{count.checked_mul(size_of::<T>()).filter(|bytes|*bytes<=isize::MAX as usize).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"Probe semantic backing extent overflow"))}
fn admit(control:&SqliteSnapshotControl<'_>,bytes:usize)->Result<(),ValueError>{if bytes>control.allocation_remaining_bytes(){return Err(refusal(ValueRefusalKind::OwnershipLimit,"Probe semantic backing exceeds original SQL allowance"))}Ok(())}
fn receipt(control:&mut SqliteSnapshotControl<'_>,before:usize,after:usize)->Result<(),ValueError>{control.admit_native_allocation_bytes(after.checked_sub(before).ok_or_else(||refusal(ValueRefusalKind::InvariantViolated,"Probe native receipt regressed"))?)}
macro_rules! native_port{
 ($port:ident,$phase:ident,$vector:ident,$text:ident,$event:ty)=>{
 impl Port for $port<'_,'_,'_>{
  fn control<R>(&mut self,operation:impl FnOnce(&mut SqliteSnapshotControl<'_>)->R)->R{operation(self.control)}
  fn phase(&self)->SqliteSnapshotPhase{SqliteSnapshotPhase::$phase}
  fn vector<T>(&mut self,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{admit(self.control,bytes::<T>(count)?)?;self.control.checkpoint(self.phase(),0,count)?;let before=self.native.owned_bytes();let result=self.body.$vector(self.native,count,output,depth);receipt(self.control,before,self.native.owned_bytes())?;result}
  fn text(&mut self,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{admit(self.control,text.len())?;self.control.checkpoint(self.phase(),0,text.len())?;let before=self.native.owned_bytes();let phase=self.phase();let control=&mut *self.control;let mut observer=|event:$event|control.observe_progress(phase,event.completed,event.total);let body=&mut *self.body;let result=self.native.scoped_observer(&mut observer,|native|body.$text(native,text,output,depth));drop(observer);receipt(self.control,before,self.native.owned_bytes())?;result}
  fn work(&mut self,depth:usize,copy:usize)->Result<(),ValueError>{self.control.checkpoint(self.phase(),0,0)?;self.native.checkpoint()?;self.body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:depth})?;self.body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})}
 }
 };
}
native_port!(Decode,ProjectSnapshot,allocate_vec_into,copy_text_into,semio_framework_value::native_decoding::NativeDecodeProgress);
native_port!(Encode,ReconstructSnapshot,allocate_encode_vec_into,copy_encode_text_into,semio_framework_value::native_encoding::NativeEncodeProgress);
pub(super) fn refusal(kind:ValueRefusalKind,message:&'static str)->ValueError{ValueError::literal(kind,message)}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Projection{pub database:SqliteDatabase,row:SqliteRow,value_bytes:usize}
impl Projection{pub fn new()->Self{Self{database:SqliteDatabase{tables:Vec::new()},row:SqliteRow{rowid:0,values:Vec::new()},value_bytes:0}}}
fn census<P:Port>(value:&DslValue,depth:usize,count:&mut usize,port:&mut P)->Result<(),ValueError>{
 port.work(depth+1,0)?;if depth>semio_framework_pack_json::MAX_DEPTH as usize{return Err(refusal(ValueRefusalKind::DepthLimit,"Probe syntax exceeds declared depth"))}
 *count=count.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::WorkLimit,"Probe node count overflow"))?;port.control(|control|control.check_rows(*count))?;
 match value{DslValue::Array(values)=>for value in values{census(value,depth+1,count,port)?},DslValue::Object(entries)=>{for(index,(name,value))in entries.iter().enumerate(){for(previous,_)in &entries[..index]{if compare_text(previous,name,port)?.is_eq(){return Err(refusal(ValueRefusalKind::InvalidValue,"Duplicate Probe object member"))}}census(value,depth+1,count,port)?}},DslValue::Bytes(_)=>return Err(refusal(ValueRefusalKind::InvalidValue,"Probe JSON has no byte-array node")),DslValue::Number(Number::Float(value))if !value.is_finite()=>return Err(refusal(ValueRefusalKind::InvalidValue,"Probe number is not finite")),_=>{}}
 Ok(())
}
fn cell<P:Port>(frame:&mut Projection,value:SqliteValue,port:&mut P,depth:usize)->Result<(),ValueError>{port.work(depth+1,size_of::<SqliteValue>())?;frame.row.values.push(value);Ok(())}
fn text_cell<P:Port>(frame:&mut Projection,text:&str,port:&mut P,depth:usize)->Result<(),ValueError>{cell(frame,SqliteValue::Text(String::new()),port,depth)?;let Some(SqliteValue::Text(output))=frame.row.values.last_mut()else{unreachable!()};port.text(text,output,depth+1)}
pub(super) fn project<P:Port>(source:&DslValue,frame:&mut Projection,port:&mut P)->Result<(),ValueError>{
 let mut count=0;census(source,0,&mut count,port)?;let schema=<super::super::ProbeSnapshot as store::ArtifactSqliteSnapshot>::SQLITE_SCHEMA.trim().trim_end_matches(';');
 if schema.len()+10>port.control(|control|control.limits()).max_schema_bytes||port.control(|control|control.limits()).max_columns<8||port.control(|control|control.limits()).max_tables<1{return Err(refusal(ValueRefusalKind::OwnershipLimit,"Probe authored SQL schema exceeds original limits"))}
 port.vector(1,&mut frame.database.tables,1)?;port.work(1,size_of::<store::sqlite_snapshot::SqliteTable>())?;frame.database.tables.push(store::sqlite_snapshot::SqliteTable{name:String::new(),sql:String::new(),rows:Vec::new()});
 let table=&mut frame.database.tables[0];port.text("probe_node",&mut table.name,1)?;port.text(schema,&mut table.sql,1)?;port.vector(count,&mut table.rows,1)?;
 fn node<P:Port>(source:&DslValue,parent:Option<i64>,ordinal:Option<usize>,name:Option<&str>,depth:usize,frame:&mut Projection,port:&mut P)->Result<(),ValueError>{
  port.work(depth+1,0)?;let id=i64::try_from(frame.database.tables[0].rows.len()+1).map_err(|_|refusal(ValueRefusalKind::WorkLimit,"Probe node identity overflow"))?;
  let word=if let DslValue::Number(number)=source{Some(super::number_text(*number)?)}else{None};
  let kind=match source{DslValue::Null=>"null",DslValue::Bool(_)=>"boolean",DslValue::Number(_)=>"number",DslValue::String(_)=>"string",DslValue::Array(_)=>"array",DslValue::Object(_)=>"object",DslValue::Bytes(_)=>unreachable!()};
  let position=ordinal.map(|value|i64::try_from(value).map_err(|_|refusal(ValueRefusalKind::WorkLimit,"Probe member ordinal overflow"))).transpose()?;
  let count=8+usize::from(parent.is_some())*8+usize::from(position.is_some())*8+name.map_or(0,str::len)+kind.len()+usize::from(matches!(source,DslValue::Bool(_)))*8+word.as_ref().map_or(0,|word|word.text().len())+if let DslValue::String(text)=source{text.len()}else{0};
  frame.value_bytes=frame.value_bytes.checked_add(count).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"Probe semantic value extent overflow"))?;port.control(|control|control.check_value_bytes(frame.value_bytes))?;
  port.vector(8,&mut frame.row.values,depth+1)?;frame.row.rowid=id;cell(frame,SqliteValue::Integer(id),port,depth)?;cell(frame,parent.map(SqliteValue::Integer).unwrap_or(SqliteValue::Null),port,depth)?;cell(frame,position.map(SqliteValue::Integer).unwrap_or(SqliteValue::Null),port,depth)?;
  if let Some(name)=name{text_cell(frame,name,port,depth)?}else{cell(frame,SqliteValue::Null,port,depth)?}text_cell(frame,kind,port,depth)?;cell(frame,if let DslValue::Bool(value)=source{SqliteValue::Integer(i64::from(*value))}else{SqliteValue::Null},port,depth)?;
  if let Some(word)=word{text_cell(frame,word.text(),port,depth)?}else{cell(frame,SqliteValue::Null,port,depth)?}
  if let DslValue::String(text)=source{text_cell(frame,text,port,depth)?}else{cell(frame,SqliteValue::Null,port,depth)?}
  port.work(depth+1,size_of::<SqliteRow>())?;frame.database.tables[0].rows.push(std::mem::replace(&mut frame.row,SqliteRow{rowid:0,values:Vec::new()}));
  match source{DslValue::Array(values)=>for(index,value)in values.iter().enumerate(){node(value,Some(id),Some(index),None,depth+1,frame,port)?},DslValue::Object(entries)=>for(index,(name,value))in entries.iter().enumerate(){node(value,Some(id),Some(index),Some(name),depth+1,frame,port)?},_=>{}}
  Ok(())
 }
 node(source,None,None,None,0,frame,port)?;port.control(|control|control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,count))
}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Reconstruction{ids:Vec<usize>,parents:Vec<(Option<usize>,u8)>,groups:Vec<usize>,names:Vec<usize>,values:Vec<Option<DslValue>>,text:String,number:Option<semio_framework_pack_json::JsonNumberCursor>,used:usize}
impl Reconstruction{pub fn new()->Self{Self{ids:Vec::new(),parents:Vec::new(),groups:Vec::new(),names:Vec::new(),values:Vec::new(),text:String::new(),number:None,used:0}}}
fn integer(row:&SqliteRow,column:usize)->Result<i64,ValueError>{match row.values.get(column){Some(SqliteValue::Integer(value))=>Ok(*value),_=>Err(refusal(ValueRefusalKind::InvalidValue,"Probe integer column is invalid"))}}
fn text(row:&SqliteRow,column:usize)->Result<&str,ValueError>{match row.values.get(column){Some(SqliteValue::Text(value))=>Ok(value),_=>Err(refusal(ValueRefusalKind::InvalidValue,"Probe text column is invalid"))}}
fn parent(row:&SqliteRow)->Result<Option<i64>,ValueError>{match row.values.get(1){Some(SqliteValue::Null)=>Ok(None),Some(SqliteValue::Integer(value))=>Ok(Some(*value)),_=>Err(refusal(ValueRefusalKind::InvalidValue,"Probe parent column is invalid"))}}
fn compare_text<P:Port>(left:&str,right:&str,port:&mut P)->Result<Ordering,ValueError>{for(index,(a,b))in left.bytes().zip(right.bytes()).enumerate(){if index%256==0{port.work(1,0)?}let order=a.cmp(&b);if !order.is_eq(){return Ok(order)}}port.work(1,0)?;Ok(left.len().cmp(&right.len()))}
fn sort<P:Port>(indices:&mut[usize],port:&mut P,mut compare:impl FnMut(usize,usize,&mut P)->Result<Ordering,ValueError>)->Result<(),ValueError>{
 fn sift<P:Port>(indices:&mut[usize],mut root:usize,end:usize,port:&mut P,compare:&mut impl FnMut(usize,usize,&mut P)->Result<Ordering,ValueError>)->Result<(),ValueError>{while root<end/2{port.work(1,0)?;let mut child=root*2+1;if child+1<end&&compare(indices[child],indices[child+1],port)?.is_lt(){child+=1}if !compare(indices[root],indices[child],port)?.is_lt(){break}port.work(1,2*size_of::<usize>())?;indices.swap(root,child);root=child;}Ok(())}
 for root in(0..indices.len()/2).rev(){sift(indices,root,indices.len(),port,&mut compare)?}for end in(1..indices.len()).rev(){port.work(1,2*size_of::<usize>())?;indices.swap(0,end);sift(indices,0,end,port,&mut compare)?}Ok(())
}
fn position<P:Port>(rows:&[SqliteRow],ids:&[usize],identity:i64,port:&mut P)->Result<Option<usize>,ValueError>{let(mut low,mut high)=(0,ids.len());while low<high{port.work(1,0)?;let middle=low+(high-low)/2;match rows[ids[middle]].rowid.cmp(&identity){Ordering::Less=>low=middle+1,Ordering::Greater=>high=middle,Ordering::Equal=>return Ok(Some(ids[middle]))}}Ok(None)}
fn range<P:Port>(rows:&[SqliteRow],groups:&[usize],identity:i64,port:&mut P)->Result<std::ops::Range<usize>,ValueError>{let(mut low,mut high)=(0,groups.len());while low<high{port.work(1,0)?;let middle=low+(high-low)/2;if integer(&rows[groups[middle]],1)?<identity{low=middle+1}else{high=middle}}let start=low;high=groups.len();while low<high{port.work(1,0)?;let middle=low+(high-low)/2;if integer(&rows[groups[middle]],1)?<=identity{low=middle+1}else{high=middle}}Ok(start..low)}
pub(super) fn reconstruct<P:Port>(database:&SqliteDatabase,frame:&mut Reconstruction,port:&mut P)->Result<DslValue,ValueError>{
 let rows=&database.tables.iter().find(|table|table.name.eq_ignore_ascii_case("probe_node")).ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"Probe authored node table is missing"))?.rows;let count=rows.len();port.control(|control|control.check_rows(count))?;port.control(|control|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,count))?;
 port.vector(count,&mut frame.ids,1)?;port.vector(count,&mut frame.parents,1)?;port.vector(count,&mut frame.groups,1)?;port.vector(count,&mut frame.names,1)?;port.vector(count,&mut frame.values,1)?;
 let mut root=None;for(index,row)in rows.iter().enumerate(){port.work(1,0)?;if row.values.len()!=8||row.rowid<=0||integer(row,0)?!=row.rowid{return Err(refusal(ValueRefusalKind::InvalidValue,"Invalid Probe node identity"))}port.work(1,size_of::<usize>()+size_of::<Option<DslValue>>()+size_of::<(Option<usize>,u8)>())?;frame.ids.push(index);frame.values.push(None);frame.parents.push((None,0));if parent(row)?.is_none(){if root.replace(index).is_some()||row.values[2]!=SqliteValue::Null||row.values[3]!=SqliteValue::Null{return Err(refusal(ValueRefusalKind::InvalidValue,"Probe requires one root without member metadata"))}}else{port.work(1,size_of::<usize>())?;frame.groups.push(index);}}
 let root=root.ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"Probe requires one root"))?;sort(&mut frame.ids,port,|a,b,_|Ok(rows[a].rowid.cmp(&rows[b].rowid)))?;
 for pair in frame.ids.windows(2){port.work(1,0)?;if rows[pair[0]].rowid==rows[pair[1]].rowid{return Err(refusal(ValueRefusalKind::InvalidValue,"Duplicate Probe node identity"))}}
 for(index,row)in rows.iter().enumerate(){let owner=parent(row)?.map(|identity|position(rows,&frame.ids,identity,port)?.ok_or_else(||refusal(ValueRefusalKind::InvalidValue,"Probe parent does not exist"))).transpose()?;port.work(1,size_of::<Option<usize>>())?;frame.parents[index].0=owner;}
 for index in 0..count{if frame.parents[index].1==2{continue}let mut next=Some(index);while let Some(index)=next{port.work(1,0)?;let entry=&mut frame.parents[index];match entry.1{0=>{port.work(1,size_of::<u8>())?;entry.1=1;next=entry.0},1=>return Err(refusal(ValueRefusalKind::InvalidValue,"Cyclic Probe syntax")),2=>break,_=>unreachable!()}}let mut next=Some(index);while let Some(index)=next{port.work(1,0)?;let entry=&mut frame.parents[index];if entry.1!=1{break}port.work(1,size_of::<u8>())?;entry.1=2;next=entry.0;}}
 sort(&mut frame.groups,port,|a,b,_|Ok(integer(&rows[a],1)?.cmp(&integer(&rows[b],1)?).then(integer(&rows[a],2)?.cmp(&integer(&rows[b],2)?))))?;
 let mut previous=None;let mut ordinal=0i64;for index in &frame.groups{port.work(1,0)?;let row=&rows[*index];let owner=integer(row,1)?;if previous!=Some(owner){previous=Some(owner);ordinal=0;}if integer(row,2)?!=ordinal{return Err(refusal(ValueRefusalKind::InvalidValue,"Probe member ordinals must be contiguous and unique"))}ordinal=ordinal.checked_add(1).ok_or_else(||refusal(ValueRefusalKind::WorkLimit,"Probe member ordinal overflow"))?;}
 fn node<P:Port>(index:usize,depth:usize,rows:&[SqliteRow],frame:&mut Reconstruction,port:&mut P)->Result<(),ValueError>{
  port.work(depth+1,0)?;if depth>semio_framework_pack_json::MAX_DEPTH as usize{return Err(refusal(ValueRefusalKind::DepthLimit,"Probe syntax exceeds declared depth"))}let row=&rows[index];let members=range(rows,&frame.groups,row.rowid,port)?;let kind=text(row,4)?;let empty=[SqliteValue::Null,SqliteValue::Null,SqliteValue::Null];let scalar=&row.values[5..];if kind!="array"&&kind!="object"&&!members.is_empty(){return Err(refusal(ValueRefusalKind::InvalidValue,"Scalar Probe node cannot own children"))}
  match kind{
   "null"if scalar==empty=>{port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(DslValue::Null)},
   "boolean"if scalar[1..]==empty[1..]=>{port.control(|control|control.admit_reconstruction_bytes(8))?;let value=match integer(row,5)?{0=>false,1=>true,_=>return Err(refusal(ValueRefusalKind::InvalidValue,"Probe boolean must be zero or one"))};port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(DslValue::Bool(value));},
   "number"if scalar[0]==SqliteValue::Null&&scalar[2]==SqliteValue::Null=>{let value=super::number_value(text(row,6)?,&mut frame.number,port)?;port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(value)},
   "string"if scalar[..2]==empty[..2]=>{let source=text(row,7)?;port.control(|control|control.admit_reconstruction_bytes(source.len()))?;port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(DslValue::String(String::new()));let Some(DslValue::String(output))=frame.values[index].as_mut()else{unreachable!()};port.text(source,output,depth+1)?;},
   "array"|"object"if scalar==empty=>{
    if kind=="object"{frame.names.clear();for position in members.clone(){port.work(depth+1,size_of::<usize>())?;frame.names.push(frame.groups[position]);}sort(&mut frame.names,port,|a,b,port|compare_text(text(&rows[a],3)?,text(&rows[b],3)?,port))?;for pair in frame.names.windows(2){if compare_text(text(&rows[pair[0]],3)?,text(&rows[pair[1]],3)?,port)?.is_eq(){return Err(refusal(ValueRefusalKind::InvalidValue,"Duplicate Probe object member"))}}port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(DslValue::Object(Vec::new()));let Some(DslValue::Object(entries))=frame.values[index].as_mut()else{unreachable!()};port.vector(members.len(),entries,depth+1)?;}else{port.work(depth+1,size_of::<Option<DslValue>>())?;frame.values[index]=Some(DslValue::Array(Vec::new()));let Some(DslValue::Array(values))=frame.values[index].as_mut()else{unreachable!()};port.vector(members.len(),values,depth+1)?;}
    for position in members{let child=frame.groups[position];if kind=="array"&&rows[child].values[3]!=SqliteValue::Null{return Err(refusal(ValueRefusalKind::InvalidValue,"Probe array member cannot have a name"))}node(child,depth+1,rows,frame,port)?;if kind=="object"{let name=text(&rows[child],3)?;port.control(|control|control.admit_reconstruction_bytes(name.len()))?;port.text(name,&mut frame.text,depth+1)?;port.work(depth+1,size_of::<(String,DslValue)>())?;let value=frame.values[child].take().unwrap();let name=std::mem::take(&mut frame.text);let Some(DslValue::Object(entries))=frame.values[index].as_mut()else{unreachable!()};entries.push((name,value));}else{port.work(depth+1,size_of::<DslValue>())?;let value=frame.values[child].take().unwrap();let Some(DslValue::Array(values))=frame.values[index].as_mut()else{unreachable!()};values.push(value);}}
   },
   _=>return Err(refusal(ValueRefusalKind::InvalidValue,"Invalid Probe scalar columns")),
  }
  frame.used+=1;Ok(())
 }
 node(root,0,rows,frame,port)?;if frame.used!=count{return Err(refusal(ValueRefusalKind::InvalidValue,"Unreachable Probe node"))}port.control(|control|control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count))?;port.work(1,size_of::<DslValue>())?;Ok(frame.values[root].take().unwrap())
}
