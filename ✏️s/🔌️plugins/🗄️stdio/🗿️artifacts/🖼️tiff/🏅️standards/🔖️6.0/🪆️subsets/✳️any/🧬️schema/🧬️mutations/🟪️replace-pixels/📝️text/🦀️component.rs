//! 📝️ Direct replace-pixels text codec.
use super::*;
use crate::artifacts::tiff::schema::diff::*;
use crate::artifacts::tiff::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ReplacePixels(ReplacePixelsMutation { pixels }) = value else { return None };
    Some(format!("replace-pixels pixels={}", hex_encode(pixels)))
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::ReplacePixels(crate::artifacts::tiff::schema::mutations::ReplacePixelsMutation { pixels: hex_decode(arg("pixels")?)? }))
}
