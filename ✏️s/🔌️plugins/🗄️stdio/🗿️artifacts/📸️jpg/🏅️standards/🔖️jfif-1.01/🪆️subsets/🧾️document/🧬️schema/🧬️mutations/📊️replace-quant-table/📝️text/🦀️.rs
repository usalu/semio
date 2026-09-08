//! 📝️ Direct replace-quant-table text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-quant-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ReplaceQuantTable(ReplaceQuantTableMutation { table }) = value else { return None };
    Some(format!("replace-quant-table table={}", enc_quant_table(table)))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ReplaceQuantTable(ReplaceQuantTableMutation { table: dec_quant_table(arg("table")?)? }))
}
