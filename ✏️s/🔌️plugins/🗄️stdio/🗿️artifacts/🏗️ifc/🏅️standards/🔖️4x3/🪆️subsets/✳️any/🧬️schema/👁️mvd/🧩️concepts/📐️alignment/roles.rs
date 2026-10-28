//! 🛤️ Native Alignment clauses share the released layout, parameter and owning-length roster.
pub(super) const LAYOUTS: [&str; 3] = ["IFCALIGNMENTHORIZONTAL", "IFCALIGNMENTVERTICAL", "IFCALIGNMENTCANT"];
pub(super) const PARAMETERS: [&str; 3] = ["IFCALIGNMENTHORIZONTALSEGMENT", "IFCALIGNMENTVERTICALSEGMENT", "IFCALIGNMENTCANTSEGMENT"];
pub(super) const LENGTHS: [&str; 3] = ["SegmentLength", "HorizontalLength", "HorizontalLength"];
pub(super) fn kind(value: &str) -> Option<usize> {
    match value {
        "horizontal" => Some(0),
        "vertical" => Some(1),
        "cant" => Some(2),
        _ => None,
    }
}
