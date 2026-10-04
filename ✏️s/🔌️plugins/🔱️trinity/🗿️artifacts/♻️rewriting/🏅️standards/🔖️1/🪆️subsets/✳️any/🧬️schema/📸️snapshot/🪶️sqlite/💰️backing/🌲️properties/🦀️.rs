//! 🌲️ iterative six-family reconstruction using paid retained row identities.
use super::indexes::{Rows,invalid};
use semio_framework_graph::manifest::{PropertyBag,PropertyValue};
use semio_framework_value::{ValueError,ValueRefusalKind,DecodedValue};
use semio_framework_value::FromValue;
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteRow,transfer,artifact::{read_binary64,FloatColumn}};
fn invariant(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvariantViolated,message)}
pub(super) fn retire<T:FromValue>(value:T){T::retire_decoded(value)}
pub(super) struct Reader<'a,'c,'p>{pub(super) rows:Rows<'a>,pub(super) control:&'c mut SqliteSnapshotControl<'p>,bytes:usize}
impl<'a,'c,'p> Reader<'a,'c,'p>{
 pub(super) fn new(rows:Rows<'a>,control:&'c mut SqliteSnapshotControl<'p>)->Self{Self{rows,control,bytes:0}}
 fn reserve_value(&mut self,n:usize)->Result<(),ValueError>{let next=self.bytes.checked_add(n).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Rewriting semantic value bytes overflow"))?;self.control.check_value_bytes(next)?;self.bytes=next;Ok(())}
 pub(super) fn text(&mut self,row:&SqliteRow,column:usize)->Result<String,ValueError>{let value=row.text(column)?;self.reserve_value(value.len())?;transfer::copy_text(value,SqliteSnapshotPhase::ReconstructSnapshot,self.control)}
 pub(super) fn optional(&mut self,row:&SqliteRow,column:usize)->Result<Option<String>,ValueError>{match row.optional_text(column)?{Some(value)=>{self.reserve_value(value.len())?;Ok(Some(transfer::copy_text(value,SqliteSnapshotPhase::ReconstructSnapshot,self.control)?))},None=>Ok(None)}}
 pub(super) fn scalar(&mut self)->Result<(),ValueError>{self.reserve_value(8)}
 pub(super) fn property(&mut self,root:i64)->Result<PropertyValue,ValueError>{
  enum Frame<'a>{Enter(i64),Array(Vec<&'a SqliteRow>),Object(Vec<&'a SqliteRow>)}
  let count=self.rows.count(2);if count==0{return Err(invalid("Rewriting property requires a root entity"))}
  let mut pending=transfer::reserve(count,self.control)?;pending.push(Frame::Enter(root));
  let mut values=DecodedValue::new(transfer::reserve::<PropertyValue>(count,self.control)?,retire);
  let mut completed=0usize;
  while let Some(frame)=pending.pop(){
   self.rows.step(self.control)?;
   match frame{
    Frame::Enter(id)=>{
     let row=self.rows.take(2,id,self.control)?;
     match row.text(1)?{
      "null"=>values.get_mut().push(PropertyValue::Null),
      "bool"=>{let body=self.rows.body(4,id,self.control)?;let value=body.integer(2)?;if value!=0&&value!=1{return Err(invalid("Rewriting boolean requires zero or one"))}self.scalar()?;values.get_mut().push(PropertyValue::Bool(value==1));},
      "number"=>{let body=self.rows.body(5,id,self.control)?;let value=read_binary64(body,2,&[FloatColumn::Binary64(2)])?;self.scalar()?;values.get_mut().push(PropertyValue::Number(value));},
      "string"=>{let body=self.rows.body(6,id,self.control)?;let value=self.text(body,2)?;values.get_mut().push(PropertyValue::String(value));},
      "array"|"object"=>{
       let object=row.text(1)?=="object";let children=self.rows.children(if object{8}else{7},id,true,self.control)?;
       if object{for pair in children.windows(2){if pair[0].text(3)?.as_bytes()>=pair[1].text(3)?.as_bytes(){return Err(invalid("Rewriting object keys require unique binary order"))}}}
       if pending.len().checked_add(children.len()).and_then(|n|n.checked_add(1)).filter(|n|*n<=pending.capacity()).is_none(){return Err(invariant("Rewriting property frontier exceeds admitted entity slots"))}
       let offset=if object{4}else{3};let ids=transfer::reserve::<i64>(children.len(),self.control)?;
       let mut ids=ids;for child in &children{ids.push(child.integer(offset)?);}
       pending.push(if object{Frame::Object(children)}else{Frame::Array(children)});
       for id in ids.into_iter().rev(){pending.push(Frame::Enter(id));}
      },
      _=>return Err(invalid("Rewriting property family is undeclared")),
     }
    },
    Frame::Array(children)=>{
     let start=values.get().len().checked_sub(children.len()).ok_or_else(||invariant("Rewriting array children absent"))?;
     let output=DecodedValue::new(transfer::reserve::<PropertyValue>(children.len(),self.control)?,retire);let mut output=output;
     for value in values.get_mut().drain(start..){output.get_mut().push(value);}
     values.get_mut().push(PropertyValue::Array(output.take()));
    },
    Frame::Object(children)=>{
     let start=values.get().len().checked_sub(children.len()).ok_or_else(||invariant("Rewriting object children absent"))?;
     let output=DecodedValue::new(transfer::reserve::<(String,PropertyValue)>(children.len(),self.control)?,retire);let mut output=output;
     for child in &children{let key=self.text(child,3)?;output.get_mut().push((key,PropertyValue::Null));}
     for((_,target),value)in output.get_mut().iter_mut().zip(values.get_mut().drain(start..)){*target=value;}
     values.get_mut().push(PropertyValue::Object(PropertyBag::from_admitted(output.take())));
    },
   }
   completed=completed.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Rewriting property progress overflow"))?;
   self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,completed,0)?;
  }
  if values.get().len()!=1{return Err(invariant("Rewriting property walk did not settle one root"))}
  values.get_mut().pop().ok_or_else(||invariant("Rewriting property root absent"))
 }
}
