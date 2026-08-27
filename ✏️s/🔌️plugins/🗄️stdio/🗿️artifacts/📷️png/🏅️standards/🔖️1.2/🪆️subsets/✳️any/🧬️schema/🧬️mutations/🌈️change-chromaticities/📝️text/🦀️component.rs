//! 📝️ Direct change-chromaticities text codec.
use super::*;
use crate::artifacts::png::schema::diff::*;
use crate::artifacts::png::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-chromaticities";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::ChangeChromaticities(ChangeChromaticitiesMutation { chrm }) = value else { return None };
    Some(format!("change-chromaticities chrm={}", encode_option(chrm, enc_chromaticities)))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(PngMutation::ChangeChromaticities(crate::artifacts::png::schema::mutations::ChangeChromaticitiesMutation { chrm: decode_option(arg("chrm")?, dec_chromaticities)? }))
}
