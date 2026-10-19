//! 🔲️ Retained diffuse-gray radiosity accepts original spans and preserves exact blackbody properties.
use semio_framework_math_dense::{DenseSolveBudget,DenseSolveDemand,DenseSolveFault,DenseSolveStatus,DenseInverseCursor};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
enum Phase{Faces,Factors,Reciprocal,Matrix,Inverse,Sum,Emit,Complete,Fault}
/// 🚫️ Physical and numeric refusals remain typed without repaired emissivity or invented exchange.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
#[repr(u8)]
pub enum EnclosureExchangeFault{Dimensions,InvalidArea,InvalidEmissivity,InvalidViewFactor,NonReciprocal,Singular,NonFiniteArithmetic,InvalidSnapshot}
/// ⏳️ Each turn is pending, complete, refused by the genuine grant, or terminal with a typed fault.
#[derive(Clone,Copy,Debug,PartialEq)]
pub enum EnclosureExchangeStatus{Pending,Complete,Refused(DenseSolveDemand),Fault(EnclosureExchangeFault)}
/// 🧾️ Original numeric span traffic uses the shared scalar demand contract.
#[derive(Clone,Copy,Debug,PartialEq)]
pub struct EnclosureExchangeTurn{pub status:EnclosureExchangeStatus,pub receipt:DenseSolveDemand}
/// 🧭️ Fixed control owns no numeric workspace; radiosity matrices and results stay with the caller.
#[derive(Clone,Debug,PartialEq)]
pub struct EnclosureExchangeCursor{n:usize,scan:usize,row:usize,column:usize,inner:usize,phase:Phase,reflecting:bool,sum:f64,fault:Option<EnclosureExchangeFault>,inverse:DenseInverseCursor}
impl EnclosureExchangeCursor{
 /// 🌱️ Declared original span lengths are checked without scanning or allocating.
 pub fn new(n:usize,emissivity_length:usize,view_length:usize,matrix_length:usize,inverse_length:usize,exchange_length:usize)->Self{
  let valid=n.checked_mul(n).is_some_and(|length|emissivity_length==n&&[view_length,matrix_length,inverse_length,exchange_length].into_iter().all(|actual|actual==length));
  Self{n,scan:0,row:0,column:0,inner:0,phase:if !valid{Phase::Fault}else if n==0{Phase::Complete}else{Phase::Faces},reflecting:true,sum:0.0,fault:if valid{None}else{Some(EnclosureExchangeFault::Dimensions)},inverse:DenseInverseCursor::new(n,matrix_length,inverse_length)}
 }
 /// 📏️ Scalar preparation and matrix multiplication admit one original bounded action per turn.
 pub fn demand(&self)->DenseSolveDemand{
  let bytes=match self.phase{Phase::Faces=>16,Phase::Factors=>8,Phase::Reciprocal=>32,Phase::Matrix=>24,Phase::Inverse=>return self.inverse.demand(),Phase::Sum=>16,Phase::Emit=>if self.reflecting{8}else{24},Phase::Complete|Phase::Fault=>return Default::default()};
  DenseSolveDemand{items:1,copy_bytes:bytes,depth:1,capacity_bytes:0}
 }
 fn fail(&mut self,fault:EnclosureExchangeFault,receipt:DenseSolveDemand)->EnclosureExchangeTurn{self.phase=Phase::Fault;self.fault=Some(fault);EnclosureExchangeTurn{status:EnclosureExchangeStatus::Fault(fault),receipt}}
 /// 🪡️ Exact blackbodies use I-(1-epsilon)F; every matrix product remains one original scalar contribution.
 pub fn step(&mut self,areas:&[f64],emissivities:&[f64],view:&[f64],matrix:&mut[f64],inverse:&mut[f64],exchange:&mut[f64],budget:DenseSolveBudget)->EnclosureExchangeTurn{
  if self.phase==Phase::Fault{return EnclosureExchangeTurn{status:EnclosureExchangeStatus::Fault(self.fault.expect("original enclosure fault")),receipt:Default::default()};}
  if areas.len()!=self.n||emissivities.len()!=self.n||self.n.checked_mul(self.n)!=Some(view.len())||[matrix.len(),inverse.len(),exchange.len()].into_iter().any(|length|length!=view.len()){return self.fail(EnclosureExchangeFault::Dimensions,Default::default());}
  if self.phase==Phase::Complete{return EnclosureExchangeTurn{status:EnclosureExchangeStatus::Complete,receipt:Default::default()};}
  let demand=self.demand();if budget.items<demand.items||budget.copy_bytes<demand.copy_bytes||budget.depth<demand.depth{return EnclosureExchangeTurn{status:EnclosureExchangeStatus::Refused(demand),receipt:Default::default()};}
  match self.phase{
   Phase::Faces=>{let area=areas[self.scan];let emissivity=emissivities[self.scan];if !area.is_finite()||area<=0.0{return self.fail(EnclosureExchangeFault::InvalidArea,demand);}if !emissivity.is_finite()||!(0.0..=1.0).contains(&emissivity){return self.fail(EnclosureExchangeFault::InvalidEmissivity,demand);}self.reflecting&=emissivity==0.0;self.scan+=1;if self.scan==self.n{self.scan=0;self.phase=Phase::Factors;}},
   Phase::Factors=>{let factor=view[self.scan];if !factor.is_finite()||!(0.0..=1.0).contains(&factor){return self.fail(EnclosureExchangeFault::InvalidViewFactor,demand);}self.sum+=factor;self.scan+=1;if self.scan%self.n==0{if self.sum>1.0+1e-8{return self.fail(EnclosureExchangeFault::InvalidViewFactor,demand);}self.sum=0.0;}if self.scan==view.len(){self.scan=0;self.phase=Phase::Reciprocal;}},
   Phase::Reciprocal=>{let i=self.scan/self.n;let j=self.scan%self.n;let left=areas[i]*view[self.scan];let right=areas[j]*view[j*self.n+i];if !left.is_finite()||!right.is_finite(){return self.fail(EnclosureExchangeFault::NonFiniteArithmetic,demand);}if (left-right).abs()>1e-8*left.abs().max(right.abs()).max(1.0){return self.fail(EnclosureExchangeFault::NonReciprocal,demand);}self.scan+=1;if self.scan==view.len(){self.scan=0;self.phase=if self.reflecting{Phase::Emit}else{Phase::Matrix};}},
   Phase::Matrix=>{let i=self.scan/self.n;let j=self.scan%self.n;matrix[self.scan]=if i==j{1.0}else{0.0}-(1.0-emissivities[i])*view[self.scan];self.scan+=1;if self.scan==matrix.len(){self.scan=0;self.phase=Phase::Inverse;}},
   Phase::Inverse=>{let turn=self.inverse.step(matrix,inverse,budget);match turn.status{DenseSolveStatus::Complete=>{self.phase=Phase::Sum;self.inner=0;self.sum=0.0;},DenseSolveStatus::Fault(fault)=>return self.fail(match fault{DenseSolveFault::Singular=>EnclosureExchangeFault::Singular,DenseSolveFault::Dimensions=>EnclosureExchangeFault::Dimensions,_=>EnclosureExchangeFault::NonFiniteArithmetic},turn.receipt),DenseSolveStatus::Refused(demand)=>return EnclosureExchangeTurn{status:EnclosureExchangeStatus::Refused(demand),receipt:turn.receipt},DenseSolveStatus::Pending=>{}}return EnclosureExchangeTurn{status:EnclosureExchangeStatus::Pending,receipt:turn.receipt};},
   Phase::Sum=>{let sum=self.sum+view[self.row*self.n+self.inner]*inverse[self.inner*self.n+self.column];if !sum.is_finite(){return self.fail(EnclosureExchangeFault::NonFiniteArithmetic,demand);}self.sum=sum;self.inner+=1;if self.inner==self.n{self.phase=Phase::Emit;}},
   Phase::Emit=>{let value=if self.reflecting||self.row==self.column{0.0}else{emissivities[self.row]*self.sum*emissivities[self.column]};if !value.is_finite(){return self.fail(EnclosureExchangeFault::NonFiniteArithmetic,demand);}exchange[self.row*self.n+self.column]=value;self.column+=1;self.inner=0;self.sum=0.0;if self.column==self.n{self.column=0;self.row+=1;}self.phase=if self.row==self.n{Phase::Complete}else if self.reflecting{Phase::Emit}else{Phase::Sum};},
   Phase::Complete|Phase::Fault=>unreachable!(),
  }
  EnclosureExchangeTurn{status:if self.phase==Phase::Complete{EnclosureExchangeStatus::Complete}else{EnclosureExchangeStatus::Pending},receipt:demand}
 }
 /// 🧾️ Fixed portable words retain every radiosity and nested inverse scalar phase.
 pub fn snapshot(&self)->[u64;26]{let mut words=[0;26];words[..11].copy_from_slice(&[1,self.n as u64,self.scan as u64,self.row as u64,self.column as u64,self.inner as u64,self.phase as u64,u64::from(self.reflecting),self.sum.to_bits(),self.fault.map_or(0,|fault|fault as u64+1),0]);words[11..25].copy_from_slice(&self.inverse.snapshot());words}
 /// 🛂️ Restoration checks every original span bound and nested solver contract before continuation.
 pub fn from_snapshot(words:[u64;26])->Result<Self,EnclosureExchangeFault>{
  let bad=EnclosureExchangeFault::InvalidSnapshot;if words[0]!=1||words[10]!=0||words[25]!=0||words[7]>1{return Err(bad);}let n=usize::try_from(words[1]).map_err(|_|bad)?;let length=n.checked_mul(n).ok_or(bad)?;
  let bound=|index:usize,limit:usize|usize::try_from(words[index]).ok().filter(|value|*value<=limit).ok_or(bad);let scan=bound(2,length)?;let row=bound(3,n)?;let column=bound(4,n.saturating_sub(1))?;let inner=bound(5,n)?;
  let phase=match words[6]{0=>Phase::Faces,1=>Phase::Factors,2=>Phase::Reciprocal,3=>Phase::Matrix,4=>Phase::Inverse,5=>Phase::Sum,6=>Phase::Emit,7=>Phase::Complete,8=>Phase::Fault,_=>return Err(bad)};let sum=f64::from_bits(words[8]);if !sum.is_finite(){return Err(bad);}let fault=match words[9]{0=>None,1=>Some(EnclosureExchangeFault::Dimensions),2=>Some(EnclosureExchangeFault::InvalidArea),3=>Some(EnclosureExchangeFault::InvalidEmissivity),4=>Some(EnclosureExchangeFault::InvalidViewFactor),5=>Some(EnclosureExchangeFault::NonReciprocal),6=>Some(EnclosureExchangeFault::Singular),7=>Some(EnclosureExchangeFault::NonFiniteArithmetic),8=>Some(bad),_=>return Err(bad)};
  if (phase==Phase::Fault)!=fault.is_some(){return Err(bad);}if !matches!(phase,Phase::Fault|Phase::Complete)&&n==0{return Err(bad);}if phase==Phase::Faces&&scan>=n{return Err(bad);}if matches!(phase,Phase::Factors|Phase::Reciprocal|Phase::Matrix)&&scan>=length{return Err(bad);}if matches!(phase,Phase::Sum|Phase::Emit)&&row>=n{return Err(bad);}if phase==Phase::Sum&&inner>=n{return Err(bad);}if phase==Phase::Complete&&row!=n&&n!=0{return Err(bad);}
  let inverse=DenseInverseCursor::from_snapshot(words[11..25].try_into().map_err(|_|bad)?).map_err(|_|bad)?;if inverse.snapshot()[1]!=words[1]{return Err(bad);}if matches!(phase,Phase::Faces|Phase::Factors|Phase::Reciprocal|Phase::Matrix)||(words[7]==1&&matches!(phase,Phase::Emit|Phase::Complete)){if inverse.snapshot()!=DenseInverseCursor::new(n,length,length).snapshot(){return Err(bad);}}if words[7]==0&&matches!(phase,Phase::Sum|Phase::Emit|Phase::Complete)&&inverse.snapshot()[7]!=12{return Err(bad);}Ok(Self{n,scan,row,column,inner,phase,reflecting:words[7]==1,sum,fault,inverse})
 }
}
impl semio_framework_value::ToValue for EnclosureExchangeCursor{
 fn to_value(&self)->semio_framework_value::DslValue{semio_framework_value::ToValue::to_value(&self.snapshot())}
 fn to_value_controlled(&self,control:&mut semio_framework_value::native_encoding::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{semio_framework_value::ToValue::to_value_controlled(&self.snapshot(),control)}
}
impl semio_framework_value::FromValue for EnclosureExchangeCursor{
 fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;26] as semio_framework_value::FromValue>::from_value(value)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid enclosure exchange checkpoint"))}
 fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{let words=<[u64;26] as semio_framework_value::FromValue>::from_value_controlled(value,control)?;Self::from_snapshot(words).map_err(|_|semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"invalid enclosure exchange checkpoint"))}
}
impl Serialize for EnclosureExchangeCursor{fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{self.snapshot().serialize(serializer)}}
impl<'de> Deserialize<'de> for EnclosureExchangeCursor{fn deserialize<D:serde::Deserializer<'de>>(deserializer:D)->Result<Self,D::Error>{let words=<[u64;26]>::deserialize(deserializer)?;Self::from_snapshot(words).map_err(|_|serde::de::Error::custom("invalid enclosure exchange checkpoint"))}}
impl semio_framework_value::retirement::RetireOwned for EnclosureExchangeCursor{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{semio_framework_value::retirement::RetireOwned::retirement(self.snapshot())}
 fn retirement_birth_bytes(&self)->Option<usize>{semio_framework_value::retirement::RetireOwned::retirement_birth_bytes(&self.snapshot())}
 fn controlled_retirement_supported()->bool{true}
}
use serde::{Serialize,Deserialize};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;
