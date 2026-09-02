//! 📝️ Direct insert-palette-entry text codec.
use super::*;
use crate::artifacts::bmp::schema::diff::*;
use crate::artifacts::bmp::schema::mutations::text::Entry;
pub const TEXT_OPCODE: &str = "insert-palette-entry";
pub const CODEC: Entry = Entry { opcode: TEXT_OPCODE, print, parse };

pub fn spec() -> dsl::RecordSpec {
    let mut spec = dsl::__rt::newtype_variant_spec::<InsertPaletteEntryMutation>();
    spec.keyword = Some(TEXT_OPCODE.into());
    spec
}
pub fn print(value: &BmpMutation) -> Option<String> {
    let BmpMutation::InsertPaletteEntry(payload) = value else { return None };
    Some(dsl::print(&dsl::__rt::newtype_variant_to_record(payload), &spec(), dsl::JoinMode::Inline))
}
pub fn parse(line: &str) -> Result<BmpMutation, String> {
    let value = dsl::parse(line, &spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline }).map_err(|error| error.to_string())?;
    dsl::__rt::newtype_variant_from_record(&value).map(BmpMutation::InsertPaletteEntry).map_err(|error| error.to_string())
}
