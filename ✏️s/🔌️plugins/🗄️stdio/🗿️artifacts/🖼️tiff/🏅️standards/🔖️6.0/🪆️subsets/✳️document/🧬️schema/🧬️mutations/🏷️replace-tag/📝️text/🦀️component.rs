//! 📝️ Direct replace-tag text codec.
use super::*;
use crate::artifacts::tiff::schema::diff::*;
use crate::artifacts::tiff::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-tag";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag, kind, values }) = value else { return None };
    Some({ format!("replace-tag ifd-index={ifd_index} tag={tag} kind={} values={}", enc_field_type(*kind), enc_values(values)) })
}
pub fn parse(line: &str) -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::ReplaceTag(crate::artifacts::tiff::schema::mutations::ReplaceTagMutation { ifd_index: usize_arg("ifd-index")?, tag: arg("tag")?.parse::<u16>().map_err(|error| error.to_string())?, kind: dec_field_type(arg("kind")?)?, values: dec_values(arg("values")?)? }))
}
