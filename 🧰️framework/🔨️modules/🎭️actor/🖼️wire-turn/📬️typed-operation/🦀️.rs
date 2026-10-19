//! 📬️ Fixed actor result framing preserves original recipient, operation and generation.
use semio_framework_diagnostic::{Fault,FaultCode,FaultOrigin};
use semio_framework_value_derive::{ToValue,FromValue};
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneSource,RetainedOwnedProjection},retirement::controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};
pub const TYPED_OPERATION_RESULT_PAGE_BYTES:usize=4096;
pub const TYPED_OPERATION_RESULT_HEADER_BYTES:usize=b"semio.typed-operation-page.v1\0".len()+30;
#[path="📣️delivery/🦀️.rs"]
mod delivery;
pub use delivery::ResultPageDelivery;
#[path="📥️receiving/🦀️.rs"]
mod receiving;
pub use receiving::ResultPageWireReceiver;

/// 🪪️ Every result and acknowledgement carries the exact original operation identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct TypedOperationResultToken {
    pub receiver: u32,
    pub operation: u64,
    pub generation: u64,
    pub sequence: u32,
    pub attempt: u8,
}

/// 🛤️ A closed actor result vocabulary defines every publication lane.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub enum TypedOperationResultLane {
    Artifact,
    Config,
    Draft,
    Presence,
    Transient,
    WindowConfig,
    WindowTransient,
    Child,
    Interaction,
    Effect,
    Event,
    Ui,
    Download,
    Terminal,
    Fault,
    Diagnostic,
}

enum PageBody{Cold(Vec<u8>),Original(ManuallyDrop<Vec<u8>>),Source(RetainedCloneSource<Vec<u8>>),Projection(RetainedOwnedProjection<Vec<u8>>),Closing(ControlledRetirement<Vec<u8>>),Empty}

/// 📃️ One bounded actor result retains its original backing and immutable recipient token.
pub struct TypedOperationResultPage {
    pub token: TypedOperationResultToken,
    pub lane: TypedOperationResultLane,
    body: PageBody,
    len: usize,
}

impl TypedOperationResultPage {
    pub const RENDERER_PAGE_MAGIC: &'static [u8] = b"semio.typed-operation-page.v1\0";
    pub const RENDERER_ACK_MAGIC: &'static [u8] = b"semio.typed-operation-ack.v1\0";

    pub fn try_new(token: TypedOperationResultToken, lane: TypedOperationResultLane, bytes: &[u8]) -> Result<Self, Fault> {
        if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed page authority"));
        }
        Ok(Self { token, lane, body:PageBody::Cold(bytes.to_vec()), len: bytes.len() })
    }

    pub fn try_serialize_cold<T: semio_framework_value::ToValue>(token: TypedOperationResultToken, lane: TypedOperationResultLane, value: &T) -> Result<Self, Fault> {
        let text = semio_framework_pack_json::to_json_string(value);
        let encoded = text.as_bytes();
        if encoded.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("interactive-job.result-page-capacity"), "one typed-operation result exceeds its fixed incremental encoder authority"));
        }
        Self::try_new(token,lane,encoded)
    }

    pub fn bytes(&self) -> &[u8] {
        let bytes=match &self.body{PageBody::Cold(bytes)=>bytes.as_slice(),PageBody::Original(bytes)=>bytes.as_slice(),PageBody::Source(owner)=>owner.try_authority().expect("live original page").as_slice(),PageBody::Projection(owner)=>owner.borrow().expect("live projected page").get().as_slice(),_=>&[]};
        &bytes[..self.len.min(bytes.len())]
    }

    /// 📦️ Moves the fixed original backing without copying any payload byte.
    pub fn from_original(token:TypedOperationResultToken,lane:TypedOperationResultLane,bytes:Vec<u8>,len:usize)->Self{assert!(len<=bytes.len()&&bytes.capacity()==TYPED_OPERATION_RESULT_PAGE_BYTES);Self{token,lane,body:PageBody::Original(ManuallyDrop::new(bytes)),len}}
    fn frame_bytes()->usize{size_of::<Self>()*2}
    fn fits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&g.maximum_copy_bytes>=d.copy_bytes&&g.maximum_capacity_bytes>=d.capacity_bytes&&g.maximum_release_bytes>=d.release_bytes&&g.maximum_depth>=d.depth}
    fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"typed result page has no live presentation authority")}
    /// 🎟️ Quotes either original sealing or the exact small projection handoff.
    pub fn presentation_demand(&self)->Result<RetirementDemand,ValueError>{match &self.body{PageBody::Cold(_)|PageBody::Original(_)=>Ok(RetirementDemand{copy_bytes:RetainedCloneSource::<Vec<u8>>::constructor_copy_bytes()+Self::frame_bytes(),capacity_bytes:RetainedCloneSource::<Vec<u8>>::owned_constructor_capacity_bytes::<()>(),depth:1,..Default::default()}),PageBody::Source(_)=>{let mut d=RetainedOwnedProjection::<Vec<u8>>::constructor_demand();d.copy_bytes+=Self::frame_bytes();Ok(d)},_=>Err(Self::invalid())}}
    /// 🧷️ Seals the original backing first, then lends one funded immutable receiver alias.
    pub fn present(&mut self,grant:RetainedCloneGrant)->Result<(Option<Self>,RetainedCloneProgress),ValueError>{let d=self.presentation_demand()?;if !Self::fits(d,grant){return Ok((None,Default::default()))}if let PageBody::Source(owner)=&self.body{let(body,mut p)=owner.project_owned(0,|bytes|bytes,grant)?;p.copied_bytes+=Self::frame_bytes();return Ok((Some(Self{token:self.token,lane:self.lane,body:PageBody::Projection(body),len:self.len}),p))}let body=std::mem::replace(&mut self.body,PageBody::Empty);let bytes=match body{PageBody::Cold(bytes)=>bytes,PageBody::Original(bytes)=>ManuallyDrop::into_inner(bytes),_=>unreachable!()};match RetainedCloneSource::admit_owned(bytes,(),grant){Ok((owner,mut p))=>{self.body=PageBody::Source(owner);p.copied_bytes+=Self::frame_bytes();Ok((None,p))},Err((error,bytes,()))=>{self.body=PageBody::Original(ManuallyDrop::new(bytes));Err(error)}}}
    pub fn terminal_is_empty(&self)->bool{matches!(self.body,PageBody::Empty)}
    /// ♻️ Quotes every original backing or receiver-alias retirement action.
    pub fn next_close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{match &self.body{PageBody::Cold(_)|PageBody::Original(_)=>Ok(RetirementDemand{copy_bytes:Self::frame_bytes(),depth:1,..Default::default()}),PageBody::Source(owner)=>{let mut d=owner.next_close_demands(body.saturating_sub(Self::frame_bytes()))?;d.copy_bytes+=Self::frame_bytes();Ok(d)},PageBody::Projection(owner)=>{let mut d=owner.close_demand(body.saturating_sub(Self::frame_bytes()))?;d.copy_bytes+=Self::frame_bytes();Ok(d)},PageBody::Closing(owner)=>{let mut d=owner.next_demand(body.saturating_sub(Self::frame_bytes()))?;d.copy_bytes+=Self::frame_bytes();Ok(d)},PageBody::Empty=>Ok(Default::default())}}
    /// 🎟️ Closes a receiver alias before ACK, or the original owner after ACK, under the original turn.
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{let d=self.next_close_demand(grant.maximum_copy_bytes)?;if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}if !Self::fits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-Self::frame_bytes(),..grant};let step=match &mut self.body{PageBody::Source(owner)=>owner.close_step(child)?,PageBody::Projection(owner)=>owner.close_step(child)?,PageBody::Closing(owner)=>owner.step(child)?,_=>{let body=std::mem::replace(&mut self.body,PageBody::Empty);let bytes=match body{PageBody::Cold(bytes)=>bytes,PageBody::Original(bytes)=>ManuallyDrop::into_inner(bytes),_=>unreachable!()};self.body=PageBody::Closing(ControlledRetirement::new(bytes).unwrap_or_else(|_|unreachable!("page bytes support typed retirement")));return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}};let empty=match &self.body{PageBody::Source(owner)=>owner.terminal_is_empty(),PageBody::Projection(owner)=>owner.terminal_is_empty(),PageBody::Closing(owner)=>owner.terminal_is_empty(),_=>false};if empty{self.body=PageBody::Empty;self.len=0;let mut p=step.progress();p.copied_bytes+=Self::frame_bytes();return Ok(RetainedCloneStep::Complete(p))}Ok(step)}

    pub fn renderer_exchange_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::RENDERER_PAGE_MAGIC.len() + 30 + self.len);
        bytes.extend_from_slice(Self::RENDERER_PAGE_MAGIC);
        bytes.extend_from_slice(&self.token.receiver.to_le_bytes());
        bytes.extend_from_slice(&self.token.operation.to_le_bytes());
        bytes.extend_from_slice(&self.token.generation.to_le_bytes());
        bytes.extend_from_slice(&self.token.sequence.to_le_bytes());
        bytes.push(self.token.attempt);
        bytes.push(match self.lane {
            TypedOperationResultLane::Artifact => 0,
            TypedOperationResultLane::Config => 1,
            TypedOperationResultLane::Draft => 2,
            TypedOperationResultLane::Presence => 3,
            TypedOperationResultLane::Transient => 4,
            TypedOperationResultLane::Child => 5,
            TypedOperationResultLane::Effect => 6,
            TypedOperationResultLane::Event => 7,
            TypedOperationResultLane::Ui => 8,
            TypedOperationResultLane::Download => 9,
            TypedOperationResultLane::Terminal => 10,
            TypedOperationResultLane::Fault => 11,
            TypedOperationResultLane::Interaction => 12,
            TypedOperationResultLane::WindowTransient => 13,
            TypedOperationResultLane::WindowConfig => 14,
            TypedOperationResultLane::Diagnostic => 15,
        });
        bytes.extend_from_slice(&(self.len as u32).to_le_bytes());
        bytes.extend_from_slice(self.bytes());
        bytes
    }

    /// 🧾️ Writes the exact original wire header into a caller-owned fixed destination.
    pub fn renderer_header(&self,bytes:&mut[u8;TYPED_OPERATION_RESULT_HEADER_BYTES]){let magic=Self::RENDERER_PAGE_MAGIC.len();bytes[..magic].copy_from_slice(Self::RENDERER_PAGE_MAGIC);let header=&mut bytes[magic..];header[..4].copy_from_slice(&self.token.receiver.to_le_bytes());header[4..12].copy_from_slice(&self.token.operation.to_le_bytes());header[12..20].copy_from_slice(&self.token.generation.to_le_bytes());header[20..24].copy_from_slice(&self.token.sequence.to_le_bytes());header[24]=self.token.attempt;header[25]=match self.lane{TypedOperationResultLane::Artifact=>0,TypedOperationResultLane::Config=>1,TypedOperationResultLane::Draft=>2,TypedOperationResultLane::Presence=>3,TypedOperationResultLane::Transient=>4,TypedOperationResultLane::Child=>5,TypedOperationResultLane::Effect=>6,TypedOperationResultLane::Event=>7,TypedOperationResultLane::Ui=>8,TypedOperationResultLane::Download=>9,TypedOperationResultLane::Terminal=>10,TypedOperationResultLane::Fault=>11,TypedOperationResultLane::Interaction=>12,TypedOperationResultLane::WindowTransient=>13,TypedOperationResultLane::WindowConfig=>14,TypedOperationResultLane::Diagnostic=>15};header[26..30].copy_from_slice(&(self.len as u32).to_le_bytes());}

    pub fn renderer_ack_token(bytes: &[u8]) -> Option<TypedOperationResultToken> {
        let body = bytes.strip_prefix(Self::RENDERER_ACK_MAGIC)?;
        if body.len() != 25 {
            return None;
        }
        Some(TypedOperationResultToken {
            receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
            operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
            generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
            sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
            attempt: body[24],
        })
    }
}
impl Drop for TypedOperationResultPage{fn drop(&mut self){assert!(std::thread::panicking()||matches!(self.body,PageBody::Cold(_)|PageBody::Empty),"typed result page reached Drop before funded original or receiver closure")}}
