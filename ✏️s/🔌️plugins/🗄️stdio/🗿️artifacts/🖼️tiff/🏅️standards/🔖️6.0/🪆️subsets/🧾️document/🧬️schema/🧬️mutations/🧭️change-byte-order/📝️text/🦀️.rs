//! 📝️ Direct change-byte-order text codec.
use super::*;
use crate::artifacts::tiff::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-byte-order";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order }) = value else { return None };
    Some(format!("change-byte-order byte-order={}", enc_byte_order(*byte_order)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(TiffMutation::ChangeByteOrder(ChangeByteOrderMutation { byte_order: dec_byte_order(arg("byte-order")?)? }))
}
