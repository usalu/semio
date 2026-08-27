//! 📝️ Direct change-background text codec.
use super::*;
use crate::artifacts::png::schema::diff::*;
use crate::artifacts::png::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-background";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeBackground(ChangeBackgroundMutation { bkgd }) = value else { return None };
    Some(format!("change-background bkgd={}", encode_option(bkgd, enc_background)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::ChangeBackground(crate::artifacts::png::schema::mutations::ChangeBackgroundMutation { bkgd: decode_option(arg("bkgd")?, dec_background)? }))
}
