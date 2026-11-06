//! 🧮️ Staged cumulative escrow reconciliation; authenticated capture and handback are required before mounting.
use super::{GuestRelayGuestPurpose,GuestRelayGuestSubloan,GuestUsage,invalid};
use semio_framework_job::OriginalNativeRecipientIdentity;
use semio_framework_value::ValueError;
use semio_plugin_guest_contract::original_actor_context::OriginalEscrowRow;

#[derive(Clone,Copy,Default)]
pub(super) struct Currency{pub items:u64,pub copy:u64,pub work:u64}
impl Currency{
 fn add(self,other:Self)->Result<Self,ValueError>{Ok(Self{items:self.items.checked_add(other.items).ok_or_else(overflow)?,copy:self.copy.checked_add(other.copy).ok_or_else(overflow)?,work:self.work.checked_add(other.work).ok_or_else(overflow)?})}
 fn sub(self,other:Self)->Result<Self,ValueError>{Ok(Self{items:self.items.checked_sub(other.items).ok_or_else(overflow)?,copy:self.copy.checked_sub(other.copy).ok_or_else(overflow)?,work:self.work.checked_sub(other.work).ok_or_else(overflow)?})}
 fn fits(self,other:Self)->bool{self.items<=other.items&&self.copy<=other.copy&&self.work<=other.work}
}
fn overflow()->ValueError{invalid("escrow cumulative currency is inconsistent")}
fn held(row:&OriginalEscrowRow)->Currency{Currency{items:row.held_items,copy:row.held_copy,work:row.held_work}}
fn redeemed(row:&OriginalEscrowRow)->Currency{Currency{items:row.redeemed_items,copy:row.redeemed_copy,work:row.redeemed_work}}
fn released(row:&OriginalEscrowRow)->Currency{Currency{items:row.released_items,copy:row.released_copy,work:row.released_work}}
fn scope(descriptor:GuestRelayGuestSubloan)->Result<Currency,ValueError>{let budget=descriptor.budget();Ok(Currency{items:u64::try_from(budget.retained_policy.maximum_items).map_err(|_|overflow())?,copy:u64::try_from(budget.retained_policy.maximum_copy_bytes).map_err(|_|overflow())?,work:budget.fuel})}
fn usage(value:GuestUsage)->Result<Currency,ValueError>{Ok(Currency{items:u64::try_from(value.items).map_err(|_|overflow())?,copy:u64::try_from(value.copy).map_err(|_|overflow())?,work:value.work})}

/// 🪪️ Each entry is born only from a genuine issuer descriptor and retained until its Source closes.
#[derive(Clone,Copy)]
pub(super) struct IssuedOrigin{pub descriptor:GuestRelayGuestSubloan,pub producer:OriginalNativeRecipientIdentity,pub fresh:GuestUsage,pub aggregate:GuestUsage,settled:bool,held:Currency,redeemed:Currency}
impl IssuedOrigin{
 pub(super) fn is_settled(&self)->bool{self.settled}
 pub(super) fn new(descriptor:GuestRelayGuestSubloan,producer:OriginalNativeRecipientIdentity)->Self{Self{descriptor,producer,fresh:GuestUsage::default(),aggregate:GuestUsage::default(),settled:false,held:Currency::default(),redeemed:Currency::default()}}
}
#[derive(Clone,Copy)]
struct Claim{row:OriginalEscrowRow,producer:OriginalNativeRecipientIdentity,purpose:GuestRelayGuestPurpose}
/// 📚️ Retains authenticated claims through incremental Source-paid reconciliation.
pub(super) struct GuestEscrowBook{producer:Option<OriginalNativeRecipientIdentity>,source:Option<(u64,u64,u64)>,claims:[Option<Claim>;32],last_revision:Option<u64>}
impl Default for GuestEscrowBook{fn default()->Self{Self{producer:None,source:None,claims:[None;32],last_revision:None}}}
impl GuestEscrowBook{
 pub(super) fn validate_snapshot_owner(&self,producer:OriginalNativeRecipientIdentity,source:[u64;3])->Result<(),ValueError>{if self.producer.is_some_and(|old|old!=producer)||self.source.is_some_and(|old|old!=(source[0],source[1],source[2])){return Err(invalid("SRE1 changed its retained producer or Source identity"))}Ok(())}
 pub(super) fn next_publication(&self)->Result<u64,ValueError>{match self.last_revision{None=>Ok(1),Some(revision)=>revision.checked_add(1).ok_or_else(||invalid("SRE1 publication serial overflow"))}}
 pub(super) fn claim_count(&self)->usize{self.claims.iter().filter(|claim|claim.is_some()).count()}
 pub(super) fn slot_occupied(&self,index:usize)->bool{self.claims[index].is_some()}
 pub(super) fn slot_matches(&self,index:usize,row:&[u64;17])->Option<bool>{self.claims[index].as_ref().map(|claim|{let r=&claim.row;r.source_address==row[0]&&r.source_birth_generation==row[1]&&r.recipient_address==row[2]&&r.serial==row[5]})}
 pub(super) fn verify_slot_field(&self,index:usize,producer:OriginalNativeRecipientIdentity,purpose:GuestRelayGuestPurpose,row:&[u64;17],check:usize)->Result<(),ValueError>{
  let Some(old)=self.claims[index]else{return Ok(())};let r=&old.row;let valid=match check{0=>old.producer==producer&&old.purpose==purpose&&r.source_address==row[0]&&r.source_birth_generation==row[1]&&r.recipient_address==row[2]&&r.destination_address==row[3]&&r.destination_extent==row[4]&&r.serial==row[5]&&r.origin_epoch==row[6]&&(r.status!=2||row[7]==2),1|2|3=>{let axis=check-1;let held=[r.held_items,r.held_copy,r.held_work][axis];let redeemed=[r.redeemed_items,r.redeemed_copy,r.redeemed_work][axis];let released=[r.released_items,r.released_copy,r.released_work][axis];row[8+axis]>=held&&row[11+axis]>=redeemed&&row[14+axis]>=released&&(r.status!=2||row[8+axis]==held&&row[11+axis]==redeemed&&row[14+axis]==released)},_=>false};if !valid{return Err(invalid("escrow claim changed its immutable origin or cumulative totals"))}Ok(())
 }
 pub(super) fn claim_axis_delta(&self,index:usize,row:&[u64;17],axis:usize)->Result<[u64;3],ValueError>{if axis>=3{return Err(invalid("escrow currency axis is invalid"))}let previous=self.claims[index].as_ref().map(|claim|{let r=&claim.row;[[r.held_items,r.held_copy,r.held_work][axis],[r.redeemed_items,r.redeemed_copy,r.redeemed_work][axis],[r.released_items,r.released_copy,r.released_work][axis]]}).unwrap_or([0;3]);Ok([row[8+axis].checked_sub(previous[0]).ok_or_else(overflow)?,row[11+axis].checked_sub(previous[1]).ok_or_else(overflow)?,row[14+axis].checked_sub(previous[2]).ok_or_else(overflow)?])}
 pub(super) fn check_origin_axis(origin:&IssuedOrigin,delta:[u64;3],fresh:u64,axis:usize)->Result<(),ValueError>{if axis>=3{return Err(invalid("escrow origin currency axis is invalid"))}let grant=scope(origin.descriptor)?;let cap=[grant.items,grant.copy,grant.work][axis];let old_held=[origin.held.items,origin.held.copy,origin.held.work][axis];let old_redeemed=[origin.redeemed.items,origin.redeemed.copy,origin.redeemed.work][axis];let held=old_held.checked_add(delta[0]).and_then(|v|v.checked_sub(delta[1])).and_then(|v|v.checked_sub(delta[2])).ok_or_else(overflow)?;let redeemed=old_redeemed.checked_add(delta[1]).ok_or_else(overflow)?;if fresh.checked_add(redeemed).and_then(|v|v.checked_add(held)).is_none_or(|v|v>cap){return Err(invalid("escrow exceeds its genuine issued purpose scope"))}Ok(())}
 pub(super) fn settled_fresh_field(origin:&IssuedOrigin,index:usize)->Result<Option<u64>,ValueError>{if !origin.settled{return Ok(None)}let v=origin.fresh;let raw=match index{0=>u64::try_from(v.items).map_err(|_|overflow())?,1=>u64::try_from(v.copy).map_err(|_|overflow())?,2=>u64::try_from(v.capacity).map_err(|_|overflow())?,3=>u64::try_from(v.release).map_err(|_|overflow())?,4=>v.work,_=>return Err(invalid("escrow fresh receipt field is invalid"))};Ok(Some(raw))}
 pub(super) fn settled_aggregate_field(origin:&IssuedOrigin,index:usize)->Result<Option<u64>,ValueError>{if !origin.settled{return Ok(None)}let v=origin.aggregate;let raw=match index{0=>u64::try_from(v.items).map_err(|_|overflow())?,1=>u64::try_from(v.copy).map_err(|_|overflow())?,2=>u64::try_from(v.capacity).map_err(|_|overflow())?,3=>u64::try_from(v.release).map_err(|_|overflow())?,4=>v.work,_=>return Err(invalid("escrow aggregate receipt field is invalid"))};Ok(Some(raw))}
 pub(super) fn recorded_origin_fresh(origin:&IssuedOrigin,axis:usize)->Result<u64,ValueError>{let v=usage(origin.fresh)?;match axis{0=>Ok(v.items),1=>Ok(v.copy),2=>Ok(v.work),_=>Err(invalid("escrow fresh currency axis is invalid"))}}
 pub(super) fn claim_slot_copy_bytes()->usize{std::mem::size_of::<Option<Claim>>()}
 pub(super) fn apply_claim(&mut self,slot:usize,fields:[u64;17],producer:OriginalNativeRecipientIdentity,purpose:GuestRelayGuestPurpose)->Result<(),ValueError>{if slot>=32{return Err(invalid("escrow journal slot is invalid"))}self.claims[slot]=Some(Claim{row:OriginalEscrowRow{source_address:fields[0],source_birth_generation:fields[1],recipient_address:fields[2],destination_address:fields[3],destination_extent:fields[4],serial:fields[5],origin_epoch:fields[6],status:fields[7],held_items:fields[8],held_copy:fields[9],held_work:fields[10],redeemed_items:fields[11],redeemed_copy:fields[12],redeemed_work:fields[13],released_items:fields[14],released_copy:fields[15],released_work:fields[16]},producer,purpose});Ok(())}
 pub(super) fn apply_origin_axis(origin:&mut IssuedOrigin,axis:usize,delta:[u64;3])->Result<(),ValueError>{let(held,redeemed)=match axis{0=>(&mut origin.held.items,&mut origin.redeemed.items),1=>(&mut origin.held.copy,&mut origin.redeemed.copy),2=>(&mut origin.held.work,&mut origin.redeemed.work),_=>return Err(invalid("escrow journal axis is invalid"))};let next_held=held.checked_add(delta[0]).and_then(|v|v.checked_sub(delta[1])).and_then(|v|v.checked_sub(delta[2])).ok_or_else(overflow)?;let next_redeemed=redeemed.checked_add(delta[1]).ok_or_else(overflow)?;*held=next_held;*redeemed=next_redeemed;Ok(())}
 pub(super) fn apply_receipt_field(origin:&mut IssuedOrigin,field:usize,aggregate:u64,fresh:u64)->Result<(),ValueError>{if field==4{origin.aggregate.work=aggregate;origin.fresh.work=fresh;return Ok(())}let a=usize::try_from(aggregate).map_err(|_|overflow())?;let f=usize::try_from(fresh).map_err(|_|overflow())?;match field{0=>{origin.aggregate.items=a;origin.fresh.items=f},1=>{origin.aggregate.copy=a;origin.fresh.copy=f},2=>{origin.aggregate.capacity=a;origin.fresh.capacity=f},3=>{origin.aggregate.release=a;origin.fresh.release=f},_=>return Err(invalid("escrow journal receipt field is invalid"))}Ok(())}
 pub(super) fn finish_receipt(origin:&mut IssuedOrigin){origin.settled=true}
 pub(super) fn finish_snapshot(&mut self,producer:OriginalNativeRecipientIdentity,source:[u64;3],revision:u64)->Result<(),ValueError>{self.validate_snapshot_owner(producer,source)?;if revision!=self.next_publication()?{return Err(invalid("escrow journal publication changed"))}self.producer=Some(producer);self.source=Some((source[0],source[1],source[2]));self.last_revision=Some(revision);Ok(())}
 pub(super) fn terminal_is_empty(&self)->bool{self.producer.is_none()&&self.source.is_none()&&self.claims.iter().all(Option::is_none)&&self.last_revision.is_none()}
 pub(super) fn pending(&self,purpose:GuestRelayGuestPurpose)->Result<Currency,ValueError>{let mut total=Currency::default();for claim in self.claims.iter().flatten(){if claim.purpose==purpose{total=total.add(held(&claim.row).sub(redeemed(&claim.row).add(released(&claim.row))?)?)?}}Ok(total)}

}
