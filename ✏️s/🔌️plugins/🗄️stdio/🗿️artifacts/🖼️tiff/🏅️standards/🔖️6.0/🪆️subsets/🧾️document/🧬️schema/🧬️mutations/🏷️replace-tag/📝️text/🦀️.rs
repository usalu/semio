//! 📝️ Direct replace-tag text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "replace-tag";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &TiffMutation) -> Option<String> {
    let TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index, tag, values }) = value else { return None };
    Some(format!("replace-tag ifd-index={ifd_index} tag={tag} values={}", enc_values(values)))
}
pub fn parse(line: &str) -> Result<TiffMutation, semio_framework_diagnostic::TextError> {
    let parse = || -> Result<TiffMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: usize_arg("ifd-index")?, tag: arg("tag")?.parse::<u16>().map_err(|error| error.to_string())?, values: dec_values(arg("values")?)? }))
};
    parse().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,semio_framework_diagnostic::TextSpan::at(1,1)))
}
