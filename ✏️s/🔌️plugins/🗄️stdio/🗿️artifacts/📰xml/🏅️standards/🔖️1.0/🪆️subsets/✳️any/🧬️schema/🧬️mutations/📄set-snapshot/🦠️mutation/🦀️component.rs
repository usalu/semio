use crate::artifacts::xml::schema::mutations::{apply_xml_mutation, XmlMutation};
use crate::artifacts::xml::XmlSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut XmlSnapshot, mutation: &XmlMutation) {
    apply_xml_mutation(projection, mutation);
}
