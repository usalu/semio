//! 🧷️ A replacement's own identity is independent of the execution identity of whoever steps it.
use semio_framework_trace::{Generation,OperationId};
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🪪️ Which fixed slot, operation and artifact generation one replacement job or member belongs to; stored at creation and never derived from a stepping context.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ReplacementIdentity{slot:u64,operation:OperationId,generation:Generation}

/// 🚦️ The single verdict of comparing a stored replacement identity with the store's current slot identity.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum ReplacementStaleness{Current,Replaced,Advanced}

impl ReplacementIdentity{
 /// 🌱️ Names the slot, operation and artifact generation a replacement was admitted under.
 pub fn new(slot:u64,operation:OperationId,generation:Generation)->Self{Self{slot,operation,generation}}
 pub fn slot(self)->u64{self.slot}
 pub fn operation(self)->OperationId{self.operation}
 pub fn generation(self)->Generation{self.generation}
 /// ⚖️ The one shared predicate: another slot or operation was replaced, the same one at another artifact generation was advanced.
 pub fn staleness(self,current:Self)->ReplacementStaleness{
  if current.slot!=self.slot||current.operation!=self.operation{return ReplacementStaleness::Replaced}
  if current.generation!=self.generation{return ReplacementStaleness::Advanced}
  ReplacementStaleness::Current
 }
 /// 🔍️ A bound identity is stale unless a driver installed exactly that identity as current; an unbound capture is never stale.
 pub fn is_stale(stored:Option<Self>,current:Option<Self>)->bool{stored.is_some_and(|stored|current.is_none_or(|current|!stored.staleness(current).is_current()))}
 /// 🚧️ Refuses with a typed invariant unless the stored identity is exactly the current slot identity.
 pub fn require_current(self,current:Self,what:&'static str)->Result<(),ValueError>{
  match self.staleness(current){ReplacementStaleness::Current=>Ok(()),_=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,what))}
 }
}

impl ReplacementStaleness{
 pub fn is_current(self)->bool{self==Self::Current}
}
