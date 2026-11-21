//! 📂 Projects the already retained picker identity into the existing Pack effect argument.

use super::{ingress::OriginalMediaLoan, raw_picker::RawPicker};
use semio_framework_value::{DslValue, ToValue};

pub fn picker_args<T: Copy, const N: usize>(picker: &RawPicker<T, N>) -> Option<DslValue> {
    let (loan, kind, maximum) = picker.issued()?;
    let mut hex = String::with_capacity(64);
    for byte in loan.base_revision {
        use std::fmt::Write;
        write!(hex, "{byte:02x}").expect("writing to String");
    }
    let loan = DslValue::object([
        ("operationId".into(), format!("{:016x}", loan.operation_id).to_value()),
        ("generation".into(), format!("{:016x}", loan.generation).to_value()),
        ("instanceId".into(), loan.instance_id.to_value()),
        ("documentId".into(), loan.document_id.to_value()),
        ("baseRevision".into(), hex.to_value()),
    ]);
    let request = DslValue::object([
        ("loan".into(), loan),
        ("kind".into(), kind.to_value()),
        ("maximumFileBytes".into(), maximum.to_value()),
    ]);
    Some(DslValue::object([("originalMedia".into(), request)]))
}

pub struct DecodedLoan<'a> {
    operation_id: u64,
    generation: u64,
    instance_id: u32,
    document_id: &'a str,
    base_revision: [u8; 32],
}

impl<'a> DecodedLoan<'a> {
    /// 🔍 Reads the admitted control's exact wire identity without deriving a revision from UI state.
    pub fn read(operation: &str, generation: &str, instance_id: u32, document_id: &'a str, revision: &str) -> Option<Self> {
        let hex = |value: &str, width| {
            value.len() == width && value.bytes().all(|ch| ch.is_ascii_digit() || (b'a'..=b'f').contains(&ch))
        };
        if !hex(operation, 16) || !hex(generation, 16) || !hex(revision, 64) || instance_id == 0
            || document_id.is_empty() || document_id.chars().take(257).count() > 256
        {
            return None;
        }
        let mut base_revision = [0; 32];
        for (index, byte) in base_revision.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&revision[index * 2..index * 2 + 2], 16).ok()?;
        }
        Some(Self {
            operation_id: u64::from_str_radix(operation, 16).ok()?,
            generation: u64::from_str_radix(generation, 16).ok()?,
            instance_id,
            document_id,
            base_revision,
        })
    }

    pub fn as_loan(&self) -> OriginalMediaLoan<'_> {
        OriginalMediaLoan { operation_id: self.operation_id, generation: self.generation, instance_id: self.instance_id,
            document_id: self.document_id, base_revision: &self.base_revision }
    }
}
