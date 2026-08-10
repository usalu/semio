use crate::artifacts::xml::{XmlSnapshot};
use crate::artifacts::xml::schema::mutations::{XmlMutation, apply_xml_mutation};

/// ▶️ Applies a set-snapshot mutation.
pub fn apply(projection: &mut XmlSnapshot, mutation: &XmlMutation) {
    apply_xml_mutation(projection, mutation);
}
