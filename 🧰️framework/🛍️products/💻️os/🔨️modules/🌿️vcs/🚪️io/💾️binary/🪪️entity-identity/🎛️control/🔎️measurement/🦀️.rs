//! 👓️ Tagged readonly framing measurements never become consuming source receipts.
use super::{serde,OriginalOperationReceipt};

pub struct OriginalOperationProjection<'a>{pub(super) receipt:&'a OriginalOperationReceipt,pub(super) retirement:usize}
impl serde::Serialize for OriginalOperationProjection<'_>{
    fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeStruct;let receipt=self.receipt;let mut output=serializer.serialize_struct("OriginalOperationProjection",10)?;output.serialize_field("kind","original-retirement-measurement")?;output.serialize_field("maximum",&receipt.maximum)?;output.serialize_field("owned",&receipt.owned)?;output.serialize_field("finished",&receipt.finished)?;output.serialize_field("reserved_return",&receipt.reserved_return)?;output.serialize_field("phase_start_owned",&receipt.phase_start_owned)?;output.serialize_field("phase_start_reserved_return",&receipt.phase_start_reserved_return)?;output.serialize_field("received_output",&receipt.received_output)?;output.serialize_field("received_input",&receipt.received_input)?;output.serialize_field("received_retirement",&self.retirement)?;output.end()}
}
