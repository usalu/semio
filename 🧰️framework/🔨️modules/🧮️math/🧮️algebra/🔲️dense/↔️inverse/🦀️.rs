//! ↔️ Retained pivoted matrix inversion consumes only original scalar span grants.
use super::{DenseSolveBudget,DenseSolveDemand,DenseSolveFault,DenseSolveStatus,DenseSolveTurn};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
enum Phase{Validate,Identity,Pivot,SwapMatrix,SwapInverse,ReadPivot,NormalizeMatrix,NormalizeInverse,SelectRow,EliminateMatrix,EliminateInverse,AdvancePivot,Complete,Fault}

/// 🧭️ Caller-owned matrix and inverse spans retain all dynamic numeric storage across every cursor checkpoint.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct DenseInverseCursor{n:usize,pivot:usize,row:usize,column:usize,best:usize,scan:usize,phase:Phase,denominator:f64,factor:f64,magnitude:f64,fault:Option<DenseSolveFault>}
impl DenseInverseCursor{
 /// 🌱️ Construction checks only declared dimensions and births no numeric workspace.
 pub fn new(n:usize,matrix_length:usize,inverse_length:usize)->Self{
  let valid=n.checked_mul(n).is_some_and(|length|length==matrix_length&&length==inverse_length);
  Self{n,pivot:0,row:0,column:0,best:0,scan:0,phase:if !valid{Phase::Fault}else if n==0{Phase::Complete}else{Phase::Validate},denominator:0.0,factor:0.0,magnitude:0.0,fault:if valid{None}else{Some(DenseSolveFault::Dimensions)}}
 }
 fn accesses(&self)->(u64,u64){match self.phase{Phase::Validate|Phase::ReadPivot=>(1,0),Phase::Identity=>(0,1),Phase::Pivot=>if self.row<self.n{(1,0)}else{(0,0)},Phase::SwapMatrix|Phase::SwapInverse=>(2,2),Phase::NormalizeMatrix|Phase::NormalizeInverse=>(1,1),Phase::SelectRow=>if self.row<self.n&&self.row!=self.pivot{(1,0)}else{(0,0)},Phase::EliminateMatrix|Phase::EliminateInverse=>(2,1),_ =>(0,0)}}
 /// 📏️ One semantic scalar action has no dynamic capacity or release demand.
 pub fn demand(&self)->DenseSolveDemand{if matches!(self.phase,Phase::Complete|Phase::Fault){return Default::default();}let(reads,writes)=self.accesses();DenseSolveDemand{items:1,copy_bytes:(reads+writes)*8,depth:1,capacity_bytes:0}}
 fn fail(&mut self,fault:DenseSolveFault,receipt:DenseSolveDemand)->DenseSolveTurn{self.phase=Phase::Fault;self.fault=Some(fault);DenseSolveTurn{status:DenseSolveStatus::Fault(fault),receipt}}
 /// 🪡️ Accepted turns touch at most two original reads and two original writes without temporary arrays.
 pub fn step(&mut self,matrix:&mut[f64],inverse:&mut[f64],budget:DenseSolveBudget)->DenseSolveTurn{
  if self.phase==Phase::Fault{return DenseSolveTurn{status:DenseSolveStatus::Fault(self.fault.expect("original inverse fault")),receipt:Default::default()};}
  if self.n.checked_mul(self.n)!=Some(matrix.len())||matrix.len()!=inverse.len(){return self.fail(DenseSolveFault::Dimensions,Default::default());}
  if self.phase==Phase::Complete{return DenseSolveTurn{status:DenseSolveStatus::Complete,receipt:Default::default()};}
  let demand=self.demand();if budget.items<demand.items||budget.copy_bytes<demand.copy_bytes||budget.depth<demand.depth{return DenseSolveTurn{status:DenseSolveStatus::Refused(demand),receipt:Default::default()};}
  let read= DenseSolveDemand{items:1,copy_bytes:self.accesses().0*8,depth:1,capacity_bytes:0};
  match self.phase{
   Phase::Validate=>{if !matrix[self.scan].is_finite(){return self.fail(DenseSolveFault::NonFiniteInput,read);}self.scan+=1;if self.scan==matrix.len(){self.scan=0;self.phase=Phase::Identity;}},
   Phase::Identity=>{inverse[self.scan]=if self.scan/self.n==self.scan%self.n{1.0}else{0.0};self.scan+=1;if self.scan==inverse.len(){self.scan=0;self.row=0;self.phase=Phase::Pivot;}},
   Phase::Pivot=>if self.row<self.n{let magnitude=matrix[self.row*self.n+self.pivot].abs();if magnitude>self.magnitude{self.magnitude=magnitude;self.best=self.row;}self.row+=1;}else{if self.magnitude==0.0{return self.fail(DenseSolveFault::Singular,read);}self.column=0;self.phase=if self.best==self.pivot{Phase::ReadPivot}else{Phase::SwapMatrix};},
   Phase::SwapMatrix=>{matrix.swap(self.pivot*self.n+self.column,self.best*self.n+self.column);self.column+=1;if self.column==self.n{self.column=0;self.phase=Phase::SwapInverse;}},
   Phase::SwapInverse=>{inverse.swap(self.pivot*self.n+self.column,self.best*self.n+self.column);self.column+=1;if self.column==self.n{self.column=0;self.phase=Phase::ReadPivot;}},
   Phase::ReadPivot=>{let denominator=matrix[self.pivot*self.n+self.pivot];if !denominator.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read);}if denominator==0.0{return self.fail(DenseSolveFault::Singular,read);}self.denominator=denominator;self.column=0;self.phase=Phase::NormalizeMatrix;},
   Phase::NormalizeMatrix|Phase::NormalizeInverse=>{let index=self.pivot*self.n+self.column;let value=if self.phase==Phase::NormalizeMatrix{matrix[index]}else{inverse[index]}/self.denominator;if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read);}if self.phase==Phase::NormalizeMatrix{matrix[index]=value;}else{inverse[index]=value;}self.column+=1;if self.column==self.n{self.column=0;self.phase=if self.phase==Phase::NormalizeMatrix{Phase::NormalizeInverse}else{self.row=0;Phase::SelectRow};}},
   Phase::SelectRow=>if self.row==self.n{self.phase=Phase::AdvancePivot;}else if self.row==self.pivot{self.row+=1;}else{self.factor=matrix[self.row*self.n+self.pivot];self.column=0;self.phase=Phase::EliminateMatrix;},
   Phase::EliminateMatrix|Phase::EliminateInverse=>{let index=self.row*self.n+self.column;let other=self.pivot*self.n+self.column;let value=if self.phase==Phase::EliminateMatrix{matrix[index]-self.factor*matrix[other]}else{inverse[index]-self.factor*inverse[other]};if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read);}if self.phase==Phase::EliminateMatrix{matrix[index]=value;}else{inverse[index]=value;}self.column+=1;if self.column==self.n{self.column=0;self.phase=if self.phase==Phase::EliminateMatrix{Phase::EliminateInverse}else{self.row+=1;Phase::SelectRow};}},
   Phase::AdvancePivot=>{self.pivot+=1;if self.pivot==self.n{self.phase=Phase::Complete;}else{self.row=self.pivot;self.best=self.pivot;self.column=0;self.magnitude=0.0;self.phase=Phase::Pivot;}},
   Phase::Complete|Phase::Fault=>unreachable!(),
  }
  DenseSolveTurn{status:if self.phase==Phase::Complete{DenseSolveStatus::Complete}else{DenseSolveStatus::Pending},receipt:demand}
 }
 /// 🧾️ Fixed words preserve every inner phase and scalar register without owning either numeric span.
 pub fn snapshot(&self)->[u64;14]{[1,self.n as u64,self.pivot as u64,self.row as u64,self.column as u64,self.best as u64,self.scan as u64,self.phase as u64,self.denominator.to_bits(),self.factor.to_bits(),self.magnitude.to_bits(),self.fault.map_or(0,|f|f as u64+1),0,0]}
 /// 🛂️ Restored cursors refuse invalid declared bounds or foreign phase/register layouts.
 pub fn from_snapshot(words:[u64;14])->Result<Self,DenseSolveFault>{
  let bad=DenseSolveFault::InvalidSnapshot;if words[0]!=1||words[12]!=0||words[13]!=0{return Err(bad);}let n=usize::try_from(words[1]).map_err(|_|bad)?;let length=n.checked_mul(n).ok_or(bad)?;
  let phase=match words[7]{0=>Phase::Validate,1=>Phase::Identity,2=>Phase::Pivot,3=>Phase::SwapMatrix,4=>Phase::SwapInverse,5=>Phase::ReadPivot,6=>Phase::NormalizeMatrix,7=>Phase::NormalizeInverse,8=>Phase::SelectRow,9=>Phase::EliminateMatrix,10=>Phase::EliminateInverse,11=>Phase::AdvancePivot,12=>Phase::Complete,13=>Phase::Fault,_=>return Err(bad)};
  let counter=|index:usize,limit:usize|usize::try_from(words[index]).ok().filter(|value|*value<=limit).ok_or(bad);
  let pivot=counter(2,n)?;let row=counter(3,n)?;let column=counter(4,n)?;let best=counter(5,n.saturating_sub(1))?;let scan=counter(6,length)?;
  let denominator=f64::from_bits(words[8]);let factor=f64::from_bits(words[9]);let magnitude=f64::from_bits(words[10]);if ![denominator,factor,magnitude].into_iter().all(f64::is_finite)||magnitude<0.0{return Err(bad);}
  let fault=match words[11]{0=>None,1=>Some(DenseSolveFault::Dimensions),2=>Some(DenseSolveFault::NonFiniteInput),3=>Some(DenseSolveFault::Singular),4=>Some(DenseSolveFault::NonFiniteArithmetic),5=>Some(DenseSolveFault::InvalidSnapshot),_=>return Err(bad)};
  if (phase==Phase::Fault)!=fault.is_some(){return Err(bad);}if !matches!(phase,Phase::Fault|Phase::Complete)&&n==0{return Err(bad);}if !matches!(phase,Phase::Fault|Phase::Complete)&&pivot>=n{return Err(bad);}
  if matches!(phase,Phase::Validate|Phase::Identity)&&scan>=length{return Err(bad);}if matches!(phase,Phase::SwapMatrix|Phase::SwapInverse|Phase::NormalizeMatrix|Phase::NormalizeInverse|Phase::EliminateMatrix|Phase::EliminateInverse)&&column>=n{return Err(bad);}if matches!(phase,Phase::EliminateMatrix|Phase::EliminateInverse)&&row>=n{return Err(bad);}
  if phase==Phase::Complete&&pivot!=n{return Err(bad);}if matches!(phase,Phase::NormalizeMatrix|Phase::NormalizeInverse)&&denominator==0.0{return Err(bad);}if matches!(phase,Phase::SwapMatrix|Phase::SwapInverse)&&best==pivot{return Err(bad);}if matches!(phase,Phase::EliminateMatrix|Phase::EliminateInverse)&&row==pivot{return Err(bad);}
  Ok(Self{n,pivot,row,column,best,scan,phase,denominator,factor,magnitude,fault})
 }
}

impl semio_framework_value::ToValue for DenseInverseCursor{
 fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::ToValue::to_value(&self.snapshot())}
 fn to_value_controlled(&self,control:&mut semio_framework_value::native_encoding::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{semio_framework_value::ToValue::to_value_controlled(&self.snapshot(),control)}
}
impl semio_framework_value::FromValue for DenseInverseCursor{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;14] as semio_framework_value::FromValue>::from_value(value)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid inverse cursor snapshot"))}
 fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;14] as semio_framework_value::FromValue>::from_value_controlled(value,control)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid inverse cursor snapshot"))}
}
impl semio_framework_value::serde::Serialize for DenseInverseCursor{fn serialize<S:semio_framework_value::serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{semio_framework_value::serde::Serialize::serialize(&self.snapshot(),serializer)}}
impl<'de> semio_framework_value::serde::Deserialize<'de> for DenseInverseCursor{fn deserialize<D:semio_framework_value::serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let words=<[u64;14] as semio_framework_value::serde::Deserialize>::deserialize(deserializer)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::serde::de::Error::custom("invalid inverse cursor snapshot"))}}
impl semio_framework_value::retirement::RetireOwned for DenseInverseCursor{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.snapshot())}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.snapshot())}
 fn controlled_retirement_supported()->bool{true}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"] mod tests;
