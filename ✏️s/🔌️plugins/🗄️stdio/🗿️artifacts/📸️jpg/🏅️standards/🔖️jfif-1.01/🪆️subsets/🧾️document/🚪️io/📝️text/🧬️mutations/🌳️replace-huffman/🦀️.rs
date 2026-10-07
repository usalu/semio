//! 📝️ Direct replace-huffman-table text codec.
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::schema::snapshot::*;
use crate::standards::v_jfif_1_01::subsets::document::io::text::diff::*;
use crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::*;
use crate::standards::v_jfif_1_01::subsets::document::io::text::mutations::Entry;
pub const TEXT_OPCODE: &str = "replace-huffman-table";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table }) = value else { return None };
    Some(format!("replace-huffman-table table={}", enc_huffman_table(table)))
}
pub fn parse(line: &str) -> Result<JpgMutation, semio_framework_diagnostic::TextError> {
    let parse = || -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ReplaceHuffmanTable(ReplaceHuffmanTableMutation { table: dec_huffman_table(arg("table")?)? }))
};
    parse().map_err(|message| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,message,semio_framework_diagnostic::TextSpan::at(1,1)))
}

#[cfg(test)]
#[path="🧪️tests/🎯️direct/🦀️.rs"]
mod tests_direct_behavior;
