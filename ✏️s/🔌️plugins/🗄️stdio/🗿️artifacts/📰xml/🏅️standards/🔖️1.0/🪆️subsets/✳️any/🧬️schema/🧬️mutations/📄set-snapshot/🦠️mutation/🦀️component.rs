use crate::artifacts::xml::schema::mutations::{apply_xml_mutation, XmlMutation};
use crate::artifacts::xml::XmlSnapshot;

/// ▶️ Applies a set-snapshot mutation.
pub async fn apply(projection: &mut XmlSnapshot, mutation: &XmlMutation) {
    apply_xml_mutation(projection, mutation);
}
