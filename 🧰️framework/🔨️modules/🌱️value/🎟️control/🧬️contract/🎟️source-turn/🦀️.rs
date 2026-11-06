//! 🎟️ The actual Source dispatch owns its original non-copyable token.
use std::sync::atomic::{AtomicU64,Ordering};
static ISSUERS:AtomicU64=AtomicU64::new(1);
/// 🪪️ Names an actual Source issuer without exposing its representation.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct OriginalSourceTurnIssuerIdentity(u64);
/// 🔖️ Names one actual dispatch of one original Source issuer.
#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
pub struct OriginalSourceTurnIdentity{issuer:OriginalSourceTurnIssuerIdentity,dispatch:u64}
/// 🚧️ Reports refusal without manufacturing a substitute turn.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum OriginalSourceTurnError{DispatchExhausted,ForeignIssuer}
/// 🏠️ Lives in the actual Source owner and issues distinct dispatch tokens.
pub struct OriginalSourceTurnIssuer{identity:OriginalSourceTurnIssuerIdentity,next:AtomicU64}
impl OriginalSourceTurnIssuer{
 /// 🆕️ Binds a new issuer identity to its actual owning Source.
 pub fn new()->Self{Self{identity:OriginalSourceTurnIssuerIdentity(ISSUERS.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).expect("original Source issuer identity exhausted")),next:AtomicU64::new(1)}}
 /// 🎬️ Issues one owned token that the actual dispatch frame retains across Pending.
 pub fn issue(&self)->Result<OriginalSourceTurn,OriginalSourceTurnError>{let dispatch=self.next.fetch_update(Ordering::Relaxed,Ordering::Relaxed,|value|value.checked_add(1)).map_err(|_|OriginalSourceTurnError::DispatchExhausted)?;Ok(OriginalSourceTurn{identity:OriginalSourceTurnIdentity{issuer:self.identity,dispatch}})}
 /// 🔎️ Validates a genuine token against its actual issuing Source.
 pub fn validate(&self,turn:&OriginalSourceTurn)->Result<(),OriginalSourceTurnError>{if turn.identity.issuer==self.identity{Ok(())}else{Err(OriginalSourceTurnError::ForeignIssuer)}}
 /// 🏷️ Observes this actual issuer without exposing a numeric constructor.
 pub fn identity(&self)->OriginalSourceTurnIssuerIdentity{self.identity}
}
impl Default for OriginalSourceTurnIssuer{fn default()->Self{Self::new()}}
/// 🧳️ The caller-owned dispatch keeps this exact token with its original paid wallet.
#[derive(Debug)]
pub struct OriginalSourceTurn{identity:OriginalSourceTurnIdentity}
impl OriginalSourceTurn{
 /// 🧭️ Borrows the opaque identity without duplicating its actual owning token.
 pub fn identity(&self)->OriginalSourceTurnIdentity{self.identity}
 /// 🫴️ Lends the actual retained token for a shorter recovery turn.
 pub fn witness(&self)->OriginalSourceTurnWitness<'_>{OriginalSourceTurnWitness{turn:self}}
}
/// 🧵️ A borrowed witness cannot outlive its actual original dispatch token.
#[derive(Clone,Copy)]
pub struct OriginalSourceTurnWitness<'turn>{turn:&'turn OriginalSourceTurn}
impl OriginalSourceTurnWitness<'_>{
 /// 🧾️ Reads the identity of this actual borrowed dispatch.
 pub fn identity(self)->OriginalSourceTurnIdentity{self.turn.identity}
 /// 🧷️ Validates a continuation against the actual retained token it originally used.
 pub fn matches(self,identity:OriginalSourceTurnIdentity)->bool{self.turn.identity==identity}
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn source_turn_requires_the_original_issuer_and_retained_dispatch(){let cases:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/cases.json")).unwrap();let source=OriginalSourceTurnIssuer::new();let foreign=OriginalSourceTurnIssuer::new();let original=source.issue().unwrap();let next=source.issue().unwrap();let alien=foreign.issue().unwrap();let alien_next=foreign.issue().unwrap();for row in cases.as_array().unwrap(){let issuer=row["sameIssuer"].as_bool().unwrap();let dispatch=row["sameDispatch"].as_bool().unwrap();let turn=match(issuer,dispatch){(true,true)=>&original,(true,false)=>&next,(false,true)=>&alien,(false,false)=>&alien_next};assert_eq!(source.validate(turn).is_ok(),issuer);assert_eq!(turn.witness().matches(original.identity()),row["accepted"].as_bool().unwrap());}}
 #[test]
 fn source_turn_token_remains_owned_outside_an_interrupted_borrow(){let source=OriginalSourceTurnIssuer::new();let mut resident=Some(source.issue().unwrap());let identity=resident.as_ref().unwrap().identity();let interrupted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||{let witness=resident.as_ref().unwrap().witness();assert!(witness.matches(identity));panic!("original source dispatch interrupted")}));assert!(interrupted.is_err());let original=resident.take().unwrap();source.validate(&original).unwrap();assert!(original.witness().matches(identity));assert_ne!(source.issue().unwrap().identity(),identity);}
}
