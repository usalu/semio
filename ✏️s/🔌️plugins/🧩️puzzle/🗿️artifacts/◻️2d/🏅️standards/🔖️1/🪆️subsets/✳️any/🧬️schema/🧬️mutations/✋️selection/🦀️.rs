//! selection transform they share: the resolved targets, the per-target outcome and the localized amounts.
//! ✋️ Puzzle2d selection mutations — dragging, rotating and scaling a selection of nodes and target regions, and the

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::Puzzle2dSnapshot;
use {semio_framework_value::list::PagedList, semio_framework_value::paged::PagedUtf8};
use crate::standards::v1::subsets::any::schema::records::puzzle2d_targets_invariant;

/// 🧭️ The members of a parametric selection leaf's target set the transform acts on, in document order, with the
/// `mutation.partial` warnings of the ones it skips. `targets` is classified by document membership against `base`: a
/// node id goes through `nodes`, a target-region id through `regions` (none when the transform has no meaning for an
/// axis-aligned rectangle). An empty or repeated target set is the Fatal `mutation.invariant` the payload schema's
/// `minItems`/`uniqueItems` forbid. Absent ids, locked members and inapplicable regions are skipped with one warning
/// per reason (in that order, ids in payload order); nothing left is `mutation.target-missing`.
pub struct Puzzle2dSelection<'a> {
    pub nodes: Vec<&'a crate::standards::v1::subsets::any::schema::records::Puzzle2dNode>,
    pub regions: Vec<&'a crate::standards::v1::subsets::any::schema::records::Puzzle2dTargetRegion>,
    pub warnings: Vec<protocol::MutationMessage>,
}

/// 🗃️ Classifies a selection leaf's `targets` against `base`; `Err` is the leaf's final refusal outcome.
pub fn puzzle2d_selection<'a>(base: &'a Puzzle2dSnapshot, targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>, regions_apply: bool) -> Result<Puzzle2dSelection<'a>, protocol::MutationOutcome<Puzzle2dDiff>> {
    let owners = || targets.iter().map(PagedUtf8::to_string_owner).collect::<Vec<_>>();
    if let Err(reason) = puzzle2d_targets_invariant(targets) {
        return Err(protocol::MutationOutcome::fatal("mutation.invariant", reason, owners()));
    }
    let (mut missing, mut locked, mut fixed, mut survivors) = (Vec::<String>::new(), Vec::<String>::new(), Vec::<String>::new(), std::collections::BTreeSet::<&PagedUtf8<{ usize::MAX }>>::new());
    for id in targets {
        match (base.nodes.iter().find(|entry| &entry.id == id), base.target_regions.iter().find(|entry| &entry.id == id)) {
            (Some(entry), _) if entry.locked != Some(true) => {
                survivors.insert(id);
            }
            (Some(_), _) => locked.push(id.to_string_owner()),
            (None, Some(entry)) if entry.locked => locked.push(id.to_string_owner()),
            (None, Some(_)) if !regions_apply => fixed.push(id.to_string_owner()),
            (None, Some(_)) => {
                survivors.insert(id);
            }
            (None, None) => missing.push(id.to_string_owner()),
        }
    }
    if survivors.is_empty() {
        return Err(protocol::MutationOutcome::error("mutation.target-missing", format!("none of the {} target(s) is an unlocked node or target region this transform applies to", targets.len()), owners()));
    }
    let warnings = [(missing, "not on this board"), (locked, "locked"), (fixed, "axis-aligned target regions do not rotate")]
        .into_iter()
        .filter(|(ids, _)| !ids.is_empty())
        .map(|(ids, reason)| protocol::MutationMessage::warning("mutation.partial", format!("{} of {} target(s) skipped ({reason}): {}", ids.len(), targets.len(), ids.join(", "))).at(ids))
        .collect();
    Ok(Puzzle2dSelection {
        nodes: base.nodes.iter().filter(|entry| survivors.contains(&entry.id)).collect(),
        regions: base.target_regions.iter().filter(|entry| regions_apply && survivors.contains(&entry.id)).collect(),
        warnings,
    })
}

/// 📦️ The outcome of a selection leaf's per-member patches: the sparse diff plus the classification warnings, or the
/// `mutation.no-op` warning (after them) when no surviving member changes.
pub fn puzzle2d_selection_outcome(
    selection: Puzzle2dSelection<'_>,
    targets: &PagedList<PagedUtf8<{ usize::MAX }>, { usize::MAX }>,
    nodes: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dNodeModification>,
    regions: Vec<crate::standards::v1::subsets::any::schema::diff::Puzzle2dTargetRegionModification>,
) -> protocol::MutationOutcome<Puzzle2dDiff> {
    use crate::standards::v1::subsets::any::schema::diff::Puzzle2dNodesDelta;
    use crate::standards::v1::subsets::any::schema::diff::Puzzle2dTargetRegionsDelta;
    if nodes.is_empty() && regions.is_empty() {
        return protocol::MutationOutcome::new(Puzzle2dDiff::default()).absorb_messages(selection.warnings.into_iter().chain([protocol::MutationMessage::warning("mutation.no-op", "no changes to apply").at(targets.iter().map(PagedUtf8::to_string_owner).collect::<Vec<_>>())]));
    }
    protocol::MutationOutcome::new(Puzzle2dDiff {
        nodes: (!nodes.is_empty()).then(|| Puzzle2dNodesDelta { modified: nodes, ..Default::default() }),
        target_regions: (!regions.is_empty()).then(|| Puzzle2dTargetRegionsDelta { modified: regions, ..Default::default() }),
        ..Default::default()
    })
    .absorb_messages(selection.warnings)
}

/// 🔄️ A point turned by an angle (`sin`, `cos`) about a pivot. The zero turn is the identity exactly (the rotation formula alone
/// lands a ulp off for an awkward pivot), so an identity rotation states no row.
pub fn puzzle2d_rotated(point: (f64, f64), pivot: (f64, f64), sin: f64, cos: f64) -> (f64, f64) {
    if sin == 0.0 && cos == 1.0 {
        return point;
    }
    (pivot.0 + (point.0 - pivot.0) * cos - (point.1 - pivot.1) * sin, pivot.1 + (point.0 - pivot.0) * sin + (point.1 - pivot.1) * cos)
}

/// 🔢️ A selection label's number as `(en, de)`: at most two decimals, trailing zeros and a negative zero
/// dropped, a decimal point in English and a decimal comma in German.
pub fn puzzle2d_selection_number(value: f64) -> (String, String) {
    let rounded = (value * 100.0).round() / 100.0;
    let text = format!("{:.2}", if rounded == 0.0 { 0.0 } else { rounded });
    let en = text.trim_end_matches('0').trim_end_matches('.').to_string();
    let de = en.replace('.', ",");
    (en, de)
}

/// 🔠️ A selection label's counted noun, `(en, de)`: "1 item" / "1 Element", "3 items" / "3 Elemente".
pub fn puzzle2d_selection_items(count: usize) -> (String, String) {
    match count {
        1 => ("1 item".to_string(), "1 Element".to_string()),
        count => (format!("{count} items"), format!("{count} Elemente")),
    }
}
