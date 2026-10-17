//! 🧰️ `SemioModelMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use semio_s_artifact_stdio_semio_base::schema::triples::NamedModified;
use semio_s_artifact_stdio_semio_base::schema::triples::NamedTripleDiff;
use crate::schema::diff::SemioModelDiff;
use crate::schema::diff::SemioModelElementDiff;
use crate::schema::snapshot::SemioModelSnapshot;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioQuaternion;
use semio_s_artifact_stdio_semio_base::schema::geometry::SemioTransform;

/// 🧭️ The diff of one relative placement edit: `edit` rewrites the BASE placement of every addressed element, so the leaf
/// replays on any base. An empty or repeated target list is a Fatal `mutation.invariant`, no addressed element left is
/// `mutation.target-missing`, a target the model lacks is skipped as `mutation.partial`, an `identity` motion is
/// `mutation.no-op`.
pub fn relative_placement_diff(targets: &[String], identity: bool, base: &SemioModelSnapshot, edit: impl Fn(SemioTransform) -> SemioTransform) -> protocol::MutationOutcome<SemioModelDiff> {
    if targets.is_empty() || targets.iter().enumerate().any(|(at, id)| targets[..at].contains(id)) {
        return protocol::MutationOutcome::fatal("mutation.invariant", "targets must name at least one element and never one twice", targets.to_vec());
    }
    let missing: Vec<String> = targets.iter().filter(|id| !base.elements.iter().any(|element| element.id == **id)).cloned().collect();
    if missing.len() == targets.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an element of this model", targets.len()), targets.to_vec());
    }
    let partial: Vec<protocol::MutationMessage> =
        (!missing.is_empty()).then(|| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped (not in this model): {}", missing.len(), targets.len(), missing.join(", "))).at(missing)).into_iter().collect();
    if identity {
        return protocol::MutationOutcome::new(SemioModelDiff::default()).absorb_messages(partial.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "an identity motion moves nothing").at(targets.to_vec())]));
    }
    let modified = base
        .elements
        .iter()
        .filter(|element| targets.contains(&element.id))
        .map(|element| {
            NamedModified { key: element.id.clone(), diff: SemioModelElementDiff { placement: Some(edit(element.placement)), ..Default::default() } }
        })
        .collect();
    protocol::MutationOutcome::new(SemioModelDiff { elements: Some(NamedTripleDiff { modified, ..Default::default() }), ..Default::default() }).absorb_messages(partial)
}

/// 🌀️ The Hamilton product `left ⊗ right` — a world-axis turn composed onto an element's own orientation.
pub fn quaternion_product(left: SemioQuaternion, right: SemioQuaternion) -> SemioQuaternion {
    let SemioQuaternion { x: lx, y: ly, z: lz, w: lw } = left;
    let SemioQuaternion { x: rx, y: ry, z: rz, w: rw } = right;
    SemioQuaternion { x: lw * rx + lx * rw + ly * rz - lz * ry, y: lw * ry - lx * rz + ly * rw + lz * rx, z: lw * rz + lx * ry - ly * rx + lz * rw, w: lw * rw - lx * rx - ly * ry - lz * rz }
}

/// 🔢️ A label number `(en, de)`: two decimals at most, trailing zeros dropped, the German decimal comma.
pub fn number_label(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 📐️ A label vector `(en, de)`: "(1, 0, 2.5)" and "(1; 0; 2,5)".
pub fn vector_label(values: [f64; 3]) -> (String, String) {
    let [x, y, z] = values.map(number_label);
    (format!("({}, {}, {})", x.0, y.0, z.0), format!("({}; {}; {})", x.1, y.1, z.1))
}

/// 🔠️ A label's counted noun `(en, de)`: "1 element" / "1 Element", "3 elements" / "3 Elemente".
pub fn element_count_label(count: usize) -> (String, String) {
    match count {
        1 => ("1 element".to_string(), "1 Element".to_string()),
        count => (format!("{count} elements"), format!("{count} Elemente")),
    }
}
