//! 🛬️ DWG's borrowed literal domain occurrences precede every typed field allocation.
use crate::dsl;
use semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits;
use dsl::{FieldValue as V,RecordValue,NativeDecodeControl};
type R<'a>=Option<&'a RecordValue>;
fn record(value:Option<&V>)->Result<R<'_>,String>{match value{None|Some(V::Absent)=>Ok(None),Some(V::Record(value))=>Ok(Some(value)),_=>Err("DWG owned native component requires a record".into())}}
fn field(record:R<'_>,id:u16)->Option<&V>{record.and_then(|value|value.get(id))}
fn child(record:R<'_>,id:u16)->Result<R<'_>,String>{self::record(field(record,id))}
fn list(value:Option<&V>)->Result<&[V],String>{match value{None|Some(V::Absent)=>Ok(&[]),Some(V::List(values))=>Ok(values),_=>Err("DWG owned native occurrence requires a list".into())}}
fn tag(record:R<'_>,id:u16)->Result<u32,String>{match field(record,id){Some(V::Enum(value))=>Ok(*value),_=>Err("DWG owned native kind is missing".into())}}
struct Rows<'a,'p>{count:usize,maximum:usize,native:&'a mut NativeDecodeControl<'p>}
impl Rows<'_,'_>{
 fn add(&mut self,count:usize)->Result<(),String>{let total=self.count.checked_add(count).ok_or("DWG borrowed row count overflow")?;if total>self.maximum{return Err("DWG borrowed native occurrences exceed row limit".into())}self.native.step()?;self.count=total;Ok(())}
 fn lists(&mut self,row:R<'_>,ids:&[u16])->Result<(),String>{for &id in ids{self.add(list(field(row,id))?.len())?;}Ok(())}
 fn values(&mut self,values:&[V])->Result<(),String>{self.add(values.len())?;for value in values{let row=record(Some(value))?;if tag(row,0)?==8{match field(row,7){Some(V::Bytes64(bytes))=>self.add(bytes.len())?,_=>return Err("DWG native XRecord intrinsic octets missing".into())}}self.native.step()?;}Ok(())}
 fn expression(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;if let Some(value)=child(row,3)?{match tag(Some(value),0)?{2=>self.lists(Some(value),&[2])?,3=>self.lists(Some(value),&[3])?,0|1|4|5|6|7=>{},_=>return Err("DWG native evaluation value kind unknown".into())}}Ok(())}
 fn associative_action(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.lists(row,&[5])}
 fn element(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.expression(child(row,0)?)}
 fn grip(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.element(child(row,0)?)?;self.lists(row,&[1])}
 fn properties(&mut self,value:Option<&V>)->Result<(),String>{let values=list(value)?;self.add(values.len())?;for value in values{self.lists(record(Some(value))?,&[0])?;}Ok(())}
 fn two_point(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.element(child(row,0)?)?;self.lists(row,&[3,4,6])?;self.properties(field(row,5))}
 fn one_point(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.element(child(row,0)?)?;self.lists(row,&[3])?;self.properties(field(row,4))}
 fn block_action(&mut self,row:R<'_>)->Result<(),String>{self.add(1)?;self.expression(child(row,0)?)?;self.lists(row,&[2,3,4])}
 fn record_body(&mut self,row:R<'_>)->Result<(),String>{let kind=tag(row,1)?;let value=child(row,kind as u16+2)?;match kind{
  0|1=>self.add(2),
  2|5=>self.add(3),
  3=>{self.add(2)?;self.lists(value,&[4])},
  4=>{self.add(2)?;self.lists(value,&[6,9])},
  6=>self.add(11),
  _=>Err("DWG native symbol-table record kind unknown".into())
 }}
 fn entity(&mut self,row:R<'_>)->Result<(),String>{let kind=tag(row,0)?;let value=child(row,kind as u16+1)?;self.add(4)?;match kind{
  0=>self.lists(value,&[1,2,4]),
  1|9=>self.lists(value,&[1,4]),
  2=>{self.lists(value,&[5])?;let vertices=list(field(value,6))?;self.add(vertices.len())?;for vertex in vertices{self.lists(record(Some(vertex))?,&[0])?;}Ok(())},
  3|4|18=>Ok(()),
  5=>self.lists(value,&[1,2,4,6]),
  6=>{self.lists(value,&[1,2,3])?;self.lists(child(value,0)?,&[1,2,8,16])},
  7=>{self.add(1)?;self.lists(value,&[1,4,5,12,13,14,15,18,19,24,25,26])},
  8=>self.lists(value,&[1,3]),
  10=>self.lists(value,&[1,2,3]),
  11=>self.lists(value,&[2,3,4]),
  12=>self.lists(value,&[7,8,9]),
  13=>self.lists(value,&[1]),
  14|15=>self.lists(value,&[3]),
  16=>self.lists(value,&[2]),
  17=>self.lists(value,&[1]),
  _=>Err("DWG native entity kind unknown".into())
 }}
 fn constraints(&mut self,row:R<'_>)->Result<(),String>{
  self.add(1)?;self.associative_action(child(row,0)?)?;self.lists(row,&[3])?;
  let planes=list(field(row,2))?;self.add(planes.len())?;for plane in planes{self.add(list(Some(plane))?.len())?;}
  for node in list(field(row,4))?{
   let node=record(Some(node))?;let kind=tag(node,0)?;let id=match kind{0=>1,1|3|5|7|8|9|10|12=>2,2=>3,4=>4,6|13=>5,11=>6,_=>return Err("DWG native constraint node kind unknown".into())};let value=child(node,id)?;
   let core=match kind{
    0=>{self.add(3)?;self.lists(value,&[1])?;child(child(value,0)?,0)?},
    2=>{self.add(3)?;self.lists(value,&[1,2,5,6])?;child(child(value,0)?,0)?},
    4=>{self.add(4)?;self.lists(value,&[2])?;child(child(child(value,0)?,0)?,0)?},
    6|13=>{self.add(3)?;child(child(value,0)?,0)?},
    11=>{self.add(3)?;self.lists(value,&[1,2])?;child(child(value,0)?,0)?},
    _=>{self.add(2)?;child(value,0)?}
   };self.lists(core,&[1])?;
  }Ok(())
 }
 fn table_style(&mut self,row:R<'_>)->Result<(),String>{self.add(21)?;for id in[3,4,5,6]{let borders=child(child(row,id)?,6)?;for edge in 0..6{if child(borders,edge)?.is_some(){self.add(2)?;}}}Ok(())}
 fn body(&mut self,row:R<'_>)->Result<(),String>{
  let kind=tag(row,0)?;if kind>42{return Err("DWG owned native body kind unknown".into())}let value=child(row,kind as u16+1)?;
  match kind{
   0=>{self.add(1)?;self.lists(value,&[0])},
   1=>{self.add(1)?;let kind=tag(value,1)?;let id=match kind{0=>3,1|2|4|5|6|7=>2,3=>4,8=>5,_=>return Err("DWG native table-control kind unknown".into())};let control=child(value,id)?;self.lists(control,&[0])?;if kind==8{self.lists(control,&[1])?;}Ok(())},
   2=>self.record_body(value),
   3=>{self.add(1)?;self.values(list(field(value,0))?)?;self.lists(value,&[1])},
   4=>self.entity(value),
   5=>self.add(1),
   6|7=>self.add(2),
   8|9=>{self.add(1)?;self.expression(child(value,0)?)},
   10=>{self.add(1)?;self.associative_action(child(value,0)?)?;self.lists(value,&[9])},
   11|13|14|15|19|20|21=>self.add(1),
   12=>self.add(9),
   16=>{self.add(1)?;self.lists(value,&[0,1])},
   17=>{self.add(1)?;self.expression(child(value,0)?)?;self.lists(value,&[4,5,11])?;self.properties(field(value,6))},
   18=>{self.add(1)?;self.expression(child(value,0)?)?;self.lists(value,&[4,11])?;self.properties(field(value,5))?;let states=list(field(value,12))?;self.add(states.len())?;for state in states{self.lists(record(Some(state))?,&[1,2])?;}Ok(())},
   22=>{self.add(1)?;self.lists(value,&[1])},
   23=>self.table_style(value),
   24=>{self.add(2)?;let elements=list(field(value,9))?;for _ in elements{self.add(2)?;}Ok(())},
   25=>{self.add(10)?;self.lists(child(value,12)?,&[2])},
   26=>{self.add(11)?;for id in[4,6,8,10,11,13]{self.lists(child(value,id)?,&[5])?;}Ok(())},
   27|38=>{self.add(1)?;self.block_action(child(value,0)?)},
   28=>{self.add(1)?;self.associative_action(child(value,0)?)?;self.lists(value,&[2])},
   29=>self.constraints(value),
   30=>{self.add(1)?;self.two_point(child(value,0)?)?;self.lists(value,&[4])},
   31=>{self.add(1)?;self.grip(child(value,0)?)?;self.lists(value,&[1])},
   32=>{self.add(1)?;self.grip(child(value,0)?)?;self.lists(value,&[2])},
   33=>{self.add(1)?;self.grip(child(value,0)?)},
   34=>{self.add(1)?;self.two_point(child(value,0)?)},
   35=>{self.add(1)?;self.grip(child(value,0)?)?;self.lists(value,&[3])},
   36=>{self.add(1)?;self.block_action(child(value,0)?)?;let points=list(field(value,3))?;self.add(points.len())?;for point in points{self.add(list(Some(point))?.len())?;}for id in[4,5]{let items=list(field(value,id))?;self.add(items.len())?;for item in items{self.lists(record(Some(item))?,&[1])?;}}Ok(())},
   37=>{self.add(2)?;let base=child(value,0)?;self.block_action(child(base,0)?)?;self.lists(base,&[1,5])},
   39=>{self.add(1)?;self.one_point(child(value,0)?)?;self.lists(value,&[1,2])},
   40|41=>{self.add(1)?;self.two_point(child(value,0)?)?;self.lists(child(value,6)?,&[0])},
   42=>{self.add(2)?;self.lists(value,&[6,7,8,12,13,18,24,25,26,27,28,29,32,33,40])},
   _=>Err("DWG owned native body kind unknown".into())
  }
 }
}
pub(super) fn root(input:&RecordValue,native:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits)->Result<usize,String>{
 native.scoped_stage(|native|{
  native.begin_stage(0)?;let mut rows=Rows{count:0,maximum:limits.max_rows,native};rows.add(22)?;
  let root=Some(input);let header=child(root,5)?;
  for id in[5,6]{rows.lists(child(header,id)?,&[0,1,2,3,4,6,7,8,10,11,12,13,14,15])?;}
  let classes=list(field(root,6))?;rows.add(classes.len())?;for class in classes{rows.lists(record(Some(class))?,&[10])?;}
  rows.lists(root,&[7])?;rows.lists(child(root,8)?,&[11])?;rows.lists(child(root,12)?,&[2])?;rows.lists(child(root,13)?,&[3,4])?;rows.lists(child(root,14)?,&[9])?;
  let drawing=child(root,4)?;rows.lists(drawing,&[0,2,3])?;
  let objects=list(field(drawing,1))?;rows.add(objects.len())?;
  for object in objects{
   let object=record(Some(object))?;rows.lists(object,&[5,7])?;
   let extended=list(field(object,8))?;rows.add(extended.len())?;for value in extended{rows.values(list(field(record(Some(value))?,1))?)?;}
   if let Some(body)=child(object,9)?{rows.body(Some(body))?;}
  }
  rows.native.checkpoint()?;Ok(rows.count)
 })
}

