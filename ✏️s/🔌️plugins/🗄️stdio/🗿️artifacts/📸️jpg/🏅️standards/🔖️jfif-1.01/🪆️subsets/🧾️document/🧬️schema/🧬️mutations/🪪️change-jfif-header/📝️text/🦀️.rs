//! 📝️ Direct change-jfif-header text codec.
use super::*;
use crate::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "change-jfif-header";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn print(value: &JpgMutation) -> Option<String> {
    let JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation { version, density_units, x_density, y_density, thumbnail }) = value else { return None };
    Some(format!("change-jfif-header version={} density-units={} x-density={x_density} y-density={y_density} thumbnail={}", enc_version(version), enc_density_units(density_units), encode_option(thumbnail, enc_thumbnail),))
}
pub fn parse(line: &str) -> Result<JpgMutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    if keyword != TEXT_OPCODE {
        return Err(format!("expected {TEXT_OPCODE}"));
    }
    let args: std::collections::BTreeMap<&str, &str> = rest.split(' ').filter(|part| !part.is_empty()).map(|token| token.split_once('=').ok_or_else(|| format!("bad argument {token}"))).collect::<Result<_, _>>()?;
    let arg = |key: &str| args.get(key).copied().ok_or_else(|| format!("missing {key}"));
    Ok(JpgMutation::ChangeJfifHeader(ChangeJfifHeaderMutation {
        version: dec_version(arg("version")?)?,
        density_units: dec_density_units(arg("density-units")?)?,
        x_density: parse_u16(arg("x-density")?)?,
        y_density: parse_u16(arg("y-density")?)?,
        thumbnail: decode_option(arg("thumbnail")?, dec_thumbnail)?,
    }))
}
