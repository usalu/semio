//! 🪪️ SHA256 identity streams complete canonical fields from original ordered roots under caller grants.
use super::{ArtifactCanonicalJson,ArtifactCanonicalJsonCursor};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_hash::Sha256;
#[path="🎮️birth/🦀️.rs"]
mod birth;
pub use birth::ArtifactCanonicalSourceIdentityBirth;
const DOMAIN:&[u8]=b"semio.canonical-source-identity/v1";
const META:usize=3*size_of::<usize>()+1;
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
/// 🎟️ One canonical source event retains its actual byte and physical ownership receipt.
#[derive(Default)]
pub struct ArtifactCanonicalSourceIdentityStep{pub progress:RetainedCloneProgress,pub byte:Option<u8>,pub ready:bool}
/// 🌲️ The caller retains genuine immutable input roots; this cursor binds each original address and streams every field.
pub struct ArtifactCanonicalSourceIdentityCursor{cursor:ArtifactCanonicalJsonCursor,hash:Sha256,digest:Option<[u8;32]>,root:Option<usize>,roots:usize,index:usize,offset:usize,phase:u8}
impl ArtifactCanonicalSourceIdentityCursor{
 pub fn constructor_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub fn admit(roots:usize,grant:RetainedCloneGrant)->Result<Option<(Self,RetainedCloneProgress)>,ValueError>{if roots>64{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"canonical source identity exceeds64 original roots"));}let d=Self::constructor_demand();if !fits(d,grant){return Ok(None);}Ok(Some((Self{cursor:ArtifactCanonicalJsonCursor::default(),hash:Sha256::new(),digest:None,root:None,roots,index:0,offset:0,phase:0},RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})))}
 pub fn source_index(&self)->Option<usize>{(self.phase==2&&!self.cursor.is_complete()).then_some(self.index)}
 pub fn is_ready(&self)->bool{self.phase==6&&self.digest.is_some()}
 pub fn digest(&self)->Option<&[u8;32]>{self.is_ready().then(||self.digest.as_ref().unwrap())}
 pub fn terminal_is_empty(&self)->bool{self.phase==8&&self.digest.is_none()&&self.root.is_none()}
 pub fn next_demand(&self)->RetirementDemand{match self.phase{0|1|4=>RetirementDemand{copy_bytes:META+3,depth:1,..Default::default()},2 if !self.cursor.is_complete()=>{let mut d=self.cursor.next_encode_demand();d.copy_bytes+=META+self.cursor.next_encode_output_bound()+2;d.depth+=1;d},2=>RetirementDemand{copy_bytes:META,depth:1,..Default::default()},3=>RetirementDemand{copy_bytes:size_of::<ArtifactCanonicalJsonCursor>()+META+3,depth:1,..Default::default()},5=>RetirementDemand{copy_bytes:size_of::<Sha256>()+32+META,depth:1,..Default::default()},7=>RetirementDemand{copy_bytes:size_of::<Self>(),depth:1,..Default::default()},_=>Default::default()}}
 pub fn advance(&mut self,source:Option<&dyn ArtifactCanonicalJson>,grant:RetainedCloneGrant)->Result<ArtifactCanonicalSourceIdentityStep,ValueError>{if self.phase>=7{return Err(invalid("canonical source identity is closing"));}if self.is_ready(){return Ok(ArtifactCanonicalSourceIdentityStep{ready:true,..Default::default()});}let d=self.next_demand();if !fits(d,grant){return Ok(Default::default());}let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};let mut byte=None;match self.phase{
 0=>{let count=(self.roots as u64).to_be_bytes();let length=DOMAIN.len()+1+count.len();if self.offset==length{self.phase=1;}else{byte=Some(if self.offset<DOMAIN.len(){DOMAIN[self.offset]}else if self.offset==DOMAIN.len(){0}else{count[self.offset-DOMAIN.len()-1]});self.offset+=1;}},
 1=>{byte=Some(b'[');self.phase=if self.roots==0{4}else{2};},
 2 if self.cursor.is_complete()=>self.phase=3,
 2=>{let source=source.ok_or_else(||invalid("canonical identity lacks its original source root"))?;let address=source as*const dyn ArtifactCanonicalJson as*const()as usize;if self.root.is_some_and(|root|root!=address){return Err(invalid("canonical identity original source address changed"));}self.root=Some(address);let overhead=META+self.cursor.next_encode_output_bound()+2;let mut output=[0u8;1];let step=self.cursor.encode_chunk_admitted(source,&mut output,RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-overhead,maximum_depth:grant.maximum_depth-1,..grant}).map_err(|error|{let mut progress=error.reason.retained_progress();progress.copied_bytes=progress.copied_bytes.checked_add(META).expect("bounded canonical identity refusal copy is representable");progress.copied_items=progress.copied_items.max(1);error.reason.with_retained_progress(progress)})?;p=step.ownership.progress();p.copied_bytes+=META+usize::from(step.written_bytes!=0)*3;p.copied_items=p.copied_items.max(1);if step.written_bytes!=0{byte=Some(output[0]);}},
 3=>{self.root=None;self.cursor=ArtifactCanonicalJsonCursor::default();self.index+=1;if self.index==self.roots{self.phase=4;}else{byte=Some(b',');self.phase=2;}},
 4=>{byte=Some(b']');self.phase=5;},
 5=>{self.digest=Some(self.hash.clone().finalize());self.phase=6;},
 _=>return Err(invalid("canonical source identity frontier is invalid")),
 }if let Some(byte)=byte{self.hash.update(&[byte]);}if !p.fits(grant){return Err(invalid("canonical source identity exceeded its original grant").with_retained_progress(p));}Ok(ArtifactCanonicalSourceIdentityStep{progress:p,byte,ready:self.is_ready()})}
 pub fn begin_close(&mut self){if !self.terminal_is_empty(){self.phase=7;}}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<ArtifactCanonicalSourceIdentityStep,ValueError>{if self.terminal_is_empty(){return Ok(Default::default());}if self.phase!=7{return Err(invalid("canonical identity close was not requested"));}let d=self.next_demand();if !fits(d,grant){return Ok(Default::default());}self.cursor=ArtifactCanonicalJsonCursor::default();self.hash=Sha256::new();self.digest=None;self.root=None;self.phase=8;Ok(ArtifactCanonicalSourceIdentityStep{progress:RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()},..Default::default()})}
}
impl Drop for ArtifactCanonicalSourceIdentityCursor{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"canonical source identity must retain its funded close witness");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
