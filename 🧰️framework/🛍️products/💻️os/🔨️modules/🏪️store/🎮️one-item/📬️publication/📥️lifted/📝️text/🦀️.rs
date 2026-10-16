//! 📥️ Items of `text` that belong to `os-store-one-item-publication` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🧵️operation-wire/📝️text/🦀️.rs`.
use semio_framework_value::ValueError;
use crate::lifted_prepared_operation_wire::ArtifactPreparedOperationError;
/// 🌳️ Exact domain text leaves and ordered delimiters without intermediate ownership.
pub enum ArtifactOperationTextNode<'a> {
    Bytes(&'a [u8]),
    Hex(&'a [u8]),
    Scalar,
    Sequence { length: usize, open: &'a [u8], separator: &'a [u8], close: &'a [u8] },
}

/// 🫳️ Ordinal lookup borrows the original typed operation; scalar formatting stays inline.
pub trait ArtifactOperationText: Sync {
    fn operation_text_node(&self, path: &[usize]) -> Result<ArtifactOperationTextNode<'_>, ValueError>;
    fn operation_text_scalar(&self, path: &[usize], offset: usize, output: &mut [u8]) -> Result<(usize, bool), ArtifactPreparedOperationError>;
}
