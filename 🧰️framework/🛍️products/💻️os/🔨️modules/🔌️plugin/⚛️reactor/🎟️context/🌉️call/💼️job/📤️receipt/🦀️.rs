//! 💼️ Owned job metadata embeds the original fixed actor receipt while body ownership remains with its producer.
use super::{OriginalActorFixedCall, OriginalActorFixedKind, OriginalActorFixedReceipt};
use semio_framework_actor::{OriginalJobCallFailure, OriginalJobOutcome, OriginalJobOutputState, OriginalJobStartReply, OriginalJobStepReply, OriginalJobOutputReply, OriginalJobOutputAckReply, OriginalJobCancelReply, RetainedTurnInput};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};

#[derive(Clone, Copy, Debug)]
pub struct OriginalOwnedJobChunk { pub offset: u64, pub byte_length: Option<u64>, pub output_closed: bool }
#[derive(Clone, Copy, Debug)]
pub enum OriginalOwnedJobStatus {
    Start { admitted: bool, returned_source: bool },
    Step { outcome: Option<OriginalJobOutcome>, source_closed: bool },
    Output { chunk: Option<OriginalOwnedJobChunk>, source_closed: bool },
    Acknowledge { acknowledged: bool, source_closed: bool },
    Cancel { admitted: bool },
    Close { admitted: bool },
}
#[derive(Clone, Copy, Debug)]
pub struct OriginalOwnedJobCall { pub input: RetainedTurnInput, pub spent: RetainedCloneProgress, pub work_units_spent: u64, pub status: OriginalOwnedJobStatus, pub refusal: Option<OriginalJobCallFailure> }
impl OriginalOwnedJobCall {
    pub fn borrow_start(reply: &OriginalJobStartReply) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Start { admitted: reply.admitted, returned_source: reply.returned_source.is_some() }, refusal: reply.refusal } }
    pub fn borrow_step(reply: &OriginalJobStepReply<OriginalJobOutcome>) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Step { outcome: reply.outcome, source_closed: reply.source_closed }, refusal: reply.refusal } }
    pub fn borrow_output(reply: &OriginalJobOutputReply) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Output { chunk: reply.chunk.as_ref().map(|chunk| OriginalOwnedJobChunk { offset: chunk.offset, byte_length: chunk.bytes.as_ref().map(|bytes| bytes.len() as u64), output_closed: chunk.output_closed }), source_closed: reply.source_closed }, refusal: reply.refusal } }
    pub fn borrow_acknowledge(reply: &OriginalJobOutputAckReply) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Acknowledge { acknowledged: reply.acknowledged, source_closed: reply.source_closed }, refusal: reply.refusal } }
    pub fn borrow_cancel(reply: &OriginalJobCancelReply) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Cancel { admitted: reply.admitted }, refusal: reply.refusal } }
    pub fn borrow_close(reply: &OriginalJobCancelReply) -> Self { Self { input: reply.input, spent: reply.spent, work_units_spent: reply.work_units_spent, status: OriginalOwnedJobStatus::Close { admitted: reply.admitted }, refusal: reply.refusal } }
}

/// 🧱️ The fixed metadata frame never serializes, clones or releases the original source or output body.
pub struct OriginalOwnedJobFixedReceipt { bytes: [u8; 160] }
impl OriginalOwnedJobFixedReceipt {
    pub const fn empty() -> Self { Self { bytes: [0; 160] } }
    pub fn bytes(&self) -> &[u8; 160] { &self.bytes }
    pub fn write(&mut self, call: OriginalOwnedJobCall) {
        let (tag, authority) = match call.refusal { None => (0, None), Some(OriginalJobCallFailure::Authority(kind)) => (1, Some(kind)), Some(OriginalJobCallFailure::Semantic) => (2, None) };
        let mut body = [0u64; 5];
        let (kind, flags) = match call.status {
            OriginalOwnedJobStatus::Start { admitted, returned_source } => (1, u8::from(admitted) | (u8::from(returned_source) << 1)),
            OriginalOwnedJobStatus::Step { outcome, source_closed } => {
                if let Some(value) = outcome { body = [value.job, value.operation, value.generation, match value.state { OriginalJobOutputState::Running => 0, OriginalJobOutputState::Done => 1, OriginalJobOutputState::Failed => 2 }, value.byte_length]; }
                (2, u8::from(outcome.is_some()) | (u8::from(source_closed) << 1))
            }
            OriginalOwnedJobStatus::Output { chunk, source_closed } => {
                let flags = chunk.map_or(0, |value| { body[0] = value.offset; body[1] = value.byte_length.unwrap_or(0); 1 | (u8::from(value.byte_length.is_some()) << 1) | (u8::from(value.output_closed) << 2) });
                (3, flags | (u8::from(source_closed) << 3))
            }
            OriginalOwnedJobStatus::Acknowledge { acknowledged, source_closed } => (4, u8::from(acknowledged) | (u8::from(source_closed) << 1)),
            OriginalOwnedJobStatus::Cancel { admitted } => (5, u8::from(admitted)),
            OriginalOwnedJobStatus::Close { admitted } => (6, u8::from(admitted)),
        };
        self.bytes[..8].copy_from_slice(&[b'S', b'J', b'H', b'1', kind, flags, tag, 0]);
        let mut actor = OriginalActorFixedReceipt::empty();
        actor.write(OriginalActorFixedCall { input: call.input, spent: call.spent, work_units_spent: call.work_units_spent, kind: OriginalActorFixedKind::Failure { authority } });
        self.bytes[8..120].copy_from_slice(actor.bytes());
        for (index, value) in body.into_iter().enumerate() { self.bytes[120 + index * 8..128 + index * 8].copy_from_slice(&value.to_le_bytes()); }
    }
    pub fn read(bytes: &[u8]) -> Result<OriginalOwnedJobCall, ValueError> {
        if bytes.len() != 160 || &bytes[..4] != b"SJH1" || bytes[7] != 0 { return Err(invalid()); }
        let actor = OriginalActorFixedReceipt::read(&bytes[8..120])?;
        let OriginalActorFixedKind::Failure { authority } = actor.kind else { return Err(invalid()) };
        let refusal = match (bytes[6], authority) { (0, None) => None, (1, Some(kind)) => Some(OriginalJobCallFailure::Authority(kind)), (2, None) => Some(OriginalJobCallFailure::Semantic), _ => return Err(invalid()) };
        let mut body = [0u64; 5];
        for (index, value) in body.iter_mut().enumerate() { *value = u64::from_le_bytes(bytes[120 + index * 8..128 + index * 8].try_into().unwrap()); }
        let flags = bytes[5];
        let empty = body.iter().all(|value| *value == 0);
        let status = match bytes[4] {
            1 if flags <= 3 && empty => OriginalOwnedJobStatus::Start { admitted: flags & 1 != 0, returned_source: flags & 2 != 0 },
            2 if flags <= 3 => {
                let outcome = if flags & 1 == 0 { if !empty { return Err(invalid()); } None } else { Some(OriginalJobOutcome { job: body[0], operation: body[1], generation: body[2], state: match body[3] { 0 => OriginalJobOutputState::Running, 1 => OriginalJobOutputState::Done, 2 => OriginalJobOutputState::Failed, _ => return Err(invalid()) }, byte_length: body[4] }) };
                OriginalOwnedJobStatus::Step { outcome, source_closed: flags & 2 != 0 }
            }
            3 if flags <= 15 && body[2..].iter().all(|value| *value == 0) => {
                let chunk = if flags & 1 == 0 { if flags & 6 != 0 || !empty { return Err(invalid()); } None } else { if flags & 2 == 0 && body[1] != 0 { return Err(invalid()); } Some(OriginalOwnedJobChunk { offset: body[0], byte_length: (flags & 2 != 0).then_some(body[1]), output_closed: flags & 4 != 0 }) };
                OriginalOwnedJobStatus::Output { chunk, source_closed: flags & 8 != 0 }
            }
            4 if flags <= 3 && empty => OriginalOwnedJobStatus::Acknowledge { acknowledged: flags & 1 != 0, source_closed: flags & 2 != 0 },
            5 if flags <= 1 && empty => OriginalOwnedJobStatus::Cancel { admitted: flags != 0 },
            6 if flags <= 1 && empty => OriginalOwnedJobStatus::Close { admitted: flags != 0 },
            _ => return Err(invalid()),
        };
        Ok(OriginalOwnedJobCall { input: actor.input, spent: actor.spent, work_units_spent: actor.work_units_spent, status, refusal })
    }
}
/// 🔒️ Resident immutable metadata stays borrowed until exact physical handback.
pub struct OriginalOwnedJobReceiptLoan { receipt: OriginalOwnedJobFixedReceipt, borrowed: bool }
impl OriginalOwnedJobReceiptLoan {
 pub const fn empty()->Self{Self{receipt:OriginalOwnedJobFixedReceipt::empty(),borrowed:false}}
 pub fn produce(&mut self,body:impl FnOnce()->OriginalOwnedJobCall)->Result<&[u8;160],ValueError>{if self.borrowed{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original owned job receipt remains borrowed"))}self.receipt.write(body());self.borrowed=true;Ok(self.receipt.bytes())}
 pub fn hand_back(&mut self,pointer:usize,length:usize)->bool{if !self.borrowed||pointer!=self.receipt.bytes().as_ptr()as usize||length!=160{return false}self.borrowed=false;true}
 pub fn owns_pointer(&self,pointer:usize)->bool{pointer==self.receipt.bytes().as_ptr()as usize}
}

fn invalid() -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, "original owned job header contains an invalid tag, flag, extent or scalar body") }

#[cfg(any(test,feature="original-actor-context-testing"))]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;
