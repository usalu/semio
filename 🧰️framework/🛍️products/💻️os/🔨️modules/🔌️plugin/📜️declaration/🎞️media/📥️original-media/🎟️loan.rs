//! 🧾️ Defines the original picker loan and fixed page extent.

pub const ORIGINAL_MEDIA_PAGE_BYTES: usize = 4096;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OriginalMediaLoan<'a> {
    pub operation_id: u64,
    pub generation: u64,
    pub instance_id: u32,
    pub document_id: &'a str,
    pub base_revision: &'a [u8; 32],
}

pub(crate) fn media_type_valid(value: &str) -> bool {
    let valid_token = |piece: &str| {
        let mut bytes = piece.bytes();
        let alphanumeric = |byte: u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
        bytes.next().is_some_and(alphanumeric) && bytes.all(|byte| alphanumeric(byte) || matches!(byte, b'.' | b'+' | b'-'))
    };
    value.len() <= 255 && value.split_once('/').is_some_and(|(kind, subtype)| valid_token(kind) && valid_token(subtype))
}

