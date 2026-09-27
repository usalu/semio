//! 📝️ Direct patch-pixels text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "patch-pixels";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &PngMutation) -> Option<String> {
    let PngMutation::PatchPixels(PatchPixelsMutation { index, remove_count, pixels, move_to }) = value else { return None };
    Some(format!("patch-pixels index={index} remove-count={remove_count} pixels={} move-to={}", hex_encode(pixels), move_to.map_or_else(|| "none".into(), |value| value.to_string())))
}
pub fn parse(line: &str) -> Result<PngMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE { return Err(format!("expected {TEXT_OPCODE}")); }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    let move_to = match arg("move-to")? { "none" => None, value => Some(value.parse::<u64>().map_err(|error| error.to_string())?) };
    Ok(PngMutation::PatchPixels(PatchPixelsMutation {
        index: arg("index")?.parse::<u64>().map_err(|error| error.to_string())?,
        remove_count: arg("remove-count")?.parse::<u64>().map_err(|error| error.to_string())?,
        pixels: hex_decode(arg("pixels")?)?,
        move_to,
    }))
}
