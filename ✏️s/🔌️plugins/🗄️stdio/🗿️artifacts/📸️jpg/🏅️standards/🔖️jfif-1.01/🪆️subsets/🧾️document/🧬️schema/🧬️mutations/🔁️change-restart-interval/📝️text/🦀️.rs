//! 📝️ Direct change-restart-interval text codec.
use super::*;
use crate::artifacts::jpg::schema::diff::*;
use crate::artifacts::jpg::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-restart-interval";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ChangeRestartInterval(ChangeRestartIntervalMutation { restart_interval }) = value else { return None };
    Some(format!("change-restart-interval restart-interval={}", diff::encode_option(restart_interval, |v| v.to_string())))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let usize_arg = |key: &str| -> Result<usize, String> { arg(key)?.parse().map_err(|error: std::num::ParseIntError| error.to_string()) };
    Ok(JpgMutation::ChangeRestartInterval(crate::artifacts::jpg::schema::mutations::ChangeRestartIntervalMutation { restart_interval: diff::decode_option(arg("restart-interval")?, diff::parse_u16)? }))
}
