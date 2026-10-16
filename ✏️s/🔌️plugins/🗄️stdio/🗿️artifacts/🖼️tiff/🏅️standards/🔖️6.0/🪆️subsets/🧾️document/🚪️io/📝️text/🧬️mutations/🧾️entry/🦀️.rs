//! 🧾️ Registry entry of the text codecs of TiffMutation, shared by the aggregator and every direct codec leaf.
use crate::schema::mutations::TiffMutation;

pub struct Entry {
    pub opcode: &'static str,
    pub print: fn(&TiffMutation) -> Option<String>,
    pub parse: fn(&str) -> Result<TiffMutation, semio_framework_diagnostic::TextError>,
}
