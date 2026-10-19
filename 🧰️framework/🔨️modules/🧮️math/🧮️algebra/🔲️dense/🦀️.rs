//! 🔲️ A fixed cursor performs one funded dense elimination action over original borrowed spans.
use std::mem::size_of;

/// 🎚️ Admission covers the exact next original source action without allocating solver storage.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DenseSolveBudget { pub items:u64, pub copy_bytes:u64, pub depth:u32, pub capacity_bytes:u64 }

/// 📏️ The caller funds every read, write and fixed cursor transition before observation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DenseSolveDemand { pub items:u64, pub copy_bytes:u64, pub depth:u32, pub capacity_bytes:u64 }

/// ⚠️ Invalid systems terminate with a typed fault rather than a substituted solution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenseSolveFault { Dimensions, NonFiniteInput, Singular, NonFiniteArithmetic, InvalidSnapshot }
impl DenseSolveFault { pub fn name(self)->&'static str {match self {Self::Dimensions=>"dimensions",Self::NonFiniteInput=>"non-finite-input",Self::Singular=>"singular",Self::NonFiniteArithmetic=>"non-finite-arithmetic",Self::InvalidSnapshot=>"invalid-snapshot"}} }

/// 🚦️ Refusal preserves both original spans and the exact pending cursor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenseSolveStatus { Pending, Complete, Fault(DenseSolveFault), Refused(DenseSolveDemand) }

/// 🧾️ Every turn reports actual observed source bytes and zero new backing capacity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DenseSolveTurn { pub status:DenseSolveStatus, pub receipt:DenseSolveDemand }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase { Validate, Pivot, SwapMatrix, SwapRhs, Factor, Eliminate, UpdateRhs, BackBegin, BackSum, BackDivide, Complete, Fault }

/// 🧭️ Partial pivoting retains only indices and three scalar observations between turns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DenseSolveCursor { n:usize, pivot:usize, row:usize, column:usize, best:usize, scan:usize, phase:Phase, magnitude:f64, factor:f64, sum:f64, fault:Option<DenseSolveFault> }

impl DenseSolveCursor {
 pub fn new(order:usize,matrix_len:usize,rhs_len:usize)->Self {
  let valid=order.checked_mul(order)==Some(matrix_len)&&rhs_len==order;
  Self {n:order,pivot:0,row:0,column:0,best:0,scan:0,phase:if !valid {Phase::Fault}else if order==0 {Phase::Complete}else{Phase::Validate},magnitude:0.0,factor:0.0,sum:0.0,fault:(!valid).then_some(DenseSolveFault::Dimensions)}
 }
 pub fn order(&self)->usize {self.n}
 fn accesses(&self)->(u64,u64) {
  match self.phase {
   Phase::Validate|Phase::BackBegin=>(1,0),Phase::Pivot if self.row<self.n=>(1,0),Phase::SwapMatrix|Phase::SwapRhs=>(2,2),Phase::Factor if self.row<self.n=>(2,1),Phase::Eliminate|Phase::UpdateRhs=>(2,1),Phase::BackSum if self.column<self.n=>(2,0),Phase::BackDivide=>(1,1),_=>(0,0)
  }
 }
 pub fn demand(&self)->DenseSolveDemand {
  if matches!(self.phase,Phase::Complete|Phase::Fault){return DenseSolveDemand::default();}
  let (reads,writes)=self.accesses();DenseSolveDemand {items:1,copy_bytes:8*(reads+writes),depth:1,capacity_bytes:0}
 }
 fn fail(&mut self,fault:DenseSolveFault,receipt:DenseSolveDemand)->DenseSolveTurn {self.phase=Phase::Fault;self.fault=Some(fault);DenseSolveTurn{status:DenseSolveStatus::Fault(fault),receipt}}
 pub fn step(&mut self,matrix:&mut[f64],rhs:&mut[f64],budget:DenseSolveBudget)->DenseSolveTurn {
  let zero=DenseSolveDemand::default();
  if self.phase==Phase::Fault {return DenseSolveTurn {status:DenseSolveStatus::Fault(self.fault.expect("fault phase")),receipt:zero};}
  if self.n.checked_mul(self.n)!=Some(matrix.len())||rhs.len()!=self.n{return self.fail(DenseSolveFault::Dimensions,zero);}
  if self.phase==Phase::Complete {return DenseSolveTurn {status:DenseSolveStatus::Complete,receipt:zero};}
  let demand=self.demand();
  if budget.items<demand.items||budget.copy_bytes<demand.copy_bytes||budget.depth<demand.depth{return DenseSolveTurn {status:DenseSolveStatus::Refused(demand),receipt:zero};}
  let reads=self.accesses().0;let read_receipt=DenseSolveDemand {items:1,copy_bytes:8*reads,depth:1,capacity_bytes:0};
  match self.phase {
   Phase::Validate=>{let value=if self.scan<matrix.len(){matrix[self.scan]}else{rhs[self.scan-matrix.len()]};if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteInput,read_receipt);}self.scan+=1;if self.scan==matrix.len()+rhs.len(){self.phase=Phase::Pivot;self.row=0;}},
   Phase::Pivot if self.row<self.n=>{let magnitude=matrix[self.row*self.n+self.pivot].abs();if magnitude>self.magnitude{self.magnitude=magnitude;self.best=self.row;}self.row+=1;},
   Phase::Pivot=>{if self.magnitude==0.0{return self.fail(DenseSolveFault::Singular,read_receipt);}self.column=0;if self.best!=self.pivot{self.phase=Phase::SwapMatrix;}else{self.row=self.pivot+1;self.phase=Phase::Factor;}},
   Phase::SwapMatrix=>{matrix.swap(self.pivot*self.n+self.column,self.best*self.n+self.column);self.column+=1;if self.column==self.n{self.phase=Phase::SwapRhs;}},
   Phase::SwapRhs=>{rhs.swap(self.pivot,self.best);self.row=self.pivot+1;self.phase=Phase::Factor;},
   Phase::Factor if self.row<self.n=>{let factor=matrix[self.row*self.n+self.pivot]/matrix[self.pivot*self.n+self.pivot];if !factor.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read_receipt);}self.factor=factor;matrix[self.row*self.n+self.pivot]=0.0;self.column=self.pivot+1;self.phase=if self.column<self.n{Phase::Eliminate}else{Phase::UpdateRhs};},
   Phase::Factor=>{self.pivot+=1;if self.pivot<self.n{self.row=self.pivot;self.best=self.pivot;self.magnitude=0.0;self.phase=Phase::Pivot;}else{self.row=self.n-1;self.phase=Phase::BackBegin;}},
   Phase::Eliminate=>{let index=self.row*self.n+self.column;let value=matrix[index]-self.factor*matrix[self.pivot*self.n+self.column];if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read_receipt);}matrix[index]=value;self.column+=1;if self.column==self.n{self.phase=Phase::UpdateRhs;}},
   Phase::UpdateRhs=>{let value=rhs[self.row]-self.factor*rhs[self.pivot];if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read_receipt);}rhs[self.row]=value;self.row+=1;self.phase=Phase::Factor;},
   Phase::BackBegin=>{self.sum=rhs[self.row];self.column=self.row+1;self.phase=Phase::BackSum;},
   Phase::BackSum if self.column<self.n=>{let sum=self.sum-matrix[self.row*self.n+self.column]*rhs[self.column];if !sum.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read_receipt);}self.sum=sum;self.column+=1;},
   Phase::BackSum=>self.phase=Phase::BackDivide,
   Phase::BackDivide=>{let value=self.sum/matrix[self.row*self.n+self.row];if !value.is_finite(){return self.fail(DenseSolveFault::NonFiniteArithmetic,read_receipt);}rhs[self.row]=value;if self.row==0{self.phase=Phase::Complete;}else{self.row-=1;self.phase=Phase::BackBegin;}},
   Phase::Complete|Phase::Fault=>unreachable!(),
  }
  DenseSolveTurn {status:if self.phase==Phase::Complete {DenseSolveStatus::Complete}else{DenseSolveStatus::Pending},receipt:demand}
 }
 /// 📸️ Fixed little endian scalar words retain the exact next action across checkpoints.
 pub fn snapshot(&self)->[u64;12] {[1,self.n as u64,self.pivot as u64,self.row as u64,self.column as u64,self.best as u64,self.scan as u64,self.phase as u64,self.magnitude.to_bits(),self.factor.to_bits(),self.sum.to_bits(),self.fault.map_or(0,|f|f as u64+1)]}
 /// 🪞️ Restore rejects impossible phases and indices before exposing original source access.
 pub fn from_snapshot(words:[u64;12])->Result<Self,DenseSolveFault> {
  let invalid=DenseSolveFault::InvalidSnapshot;if words[0]!=1{return Err(invalid);}
  let index=|i|usize::try_from(words[i]).map_err(|_|invalid);
  let phase=match words[7]{0=>Phase::Validate,1=>Phase::Pivot,2=>Phase::SwapMatrix,3=>Phase::SwapRhs,4=>Phase::Factor,5=>Phase::Eliminate,6=>Phase::UpdateRhs,7=>Phase::BackBegin,8=>Phase::BackSum,9=>Phase::BackDivide,10=>Phase::Complete,11=>Phase::Fault,_=>return Err(invalid)};
  let fault=match words[11]{0=>None,1=>Some(DenseSolveFault::Dimensions),2=>Some(DenseSolveFault::NonFiniteInput),3=>Some(DenseSolveFault::Singular),4=>Some(DenseSolveFault::NonFiniteArithmetic),5=>Some(DenseSolveFault::InvalidSnapshot),_=>return Err(invalid)};
  let c=Self {n:index(1)?,pivot:index(2)?,row:index(3)?,column:index(4)?,best:index(5)?,scan:index(6)?,phase,magnitude:f64::from_bits(words[8]),factor:f64::from_bits(words[9]),sum:f64::from_bits(words[10]),fault};
  let count=c.n.checked_mul(c.n).and_then(|n|n.checked_add(c.n)).ok_or(invalid)?;
  if c.pivot>c.n||c.row>c.n||c.column>c.n||c.best>=c.n.max(1)||c.scan>count||!c.magnitude.is_finite()||c.magnitude<0.0||!c.factor.is_finite()||!c.sum.is_finite()||(c.phase==Phase::Fault)!=c.fault.is_some(){return Err(invalid);}
  if !matches!(c.phase,Phase::Complete|Phase::Fault)&&(c.n==0||(c.phase as u8<=Phase::UpdateRhs as u8&&c.pivot>=c.n)||c.scan>=count&&c.phase==Phase::Validate){return Err(invalid);}
  if matches!(c.phase,Phase::SwapMatrix|Phase::Eliminate)&&(c.column>=c.n||c.row>=c.n&&c.phase==Phase::Eliminate){return Err(invalid);}
  if matches!(c.phase,Phase::UpdateRhs|Phase::BackBegin|Phase::BackSum|Phase::BackDivide)&&c.row>=c.n{return Err(invalid);}
  Ok(c)
 }
}
impl semio_framework_value::ToValue for DenseSolveCursor {
 fn to_value(&self)->semio_framework_value::DslValue {semio_framework_value::ToValue::to_value(&self.snapshot())}
 fn to_value_controlled(&self,c:&mut semio_framework_value::native_encoding::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{semio_framework_value::ToValue::to_value_controlled(&self.snapshot(),c)}
}
impl semio_framework_value::FromValue for DenseSolveCursor {
 fn from_value(v:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;12] as semio_framework_value::FromValue>::from_value(v)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid dense cursor snapshot"))}
 fn from_value_controlled(v:&semio_framework_value::DslValue,c:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;12] as semio_framework_value::FromValue>::from_value_controlled(v,c)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid dense cursor snapshot"))}
}
impl semio_framework_value::serde::Serialize for DenseSolveCursor {
 fn serialize<S:semio_framework_value::serde::Serializer>(&self,s:S)->Result<S::Ok,S::Error>{semio_framework_value::serde::Serialize::serialize(&self.snapshot(),s)}
}
impl<'de> semio_framework_value::serde::Deserialize<'de> for DenseSolveCursor {
 fn deserialize<D:semio_framework_value::serde::Deserializer<'de>>(d:D)->Result<Self,D::Error>{let words=<[u64;12] as semio_framework_value::serde::Deserialize>::deserialize(d)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::serde::de::Error::custom("invalid dense cursor snapshot"))}
}
impl semio_framework_value::retirement::RetireOwned for DenseSolveCursor {
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.snapshot())}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.snapshot())}
 fn controlled_retirement_supported()->bool{<[u64;12] as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
 fn retirement_element_copy_bytes()->usize{size_of::<Self>()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="↔️inverse/🦀️.rs"] pub mod inverse;
pub use inverse::DenseInverseCursor;
