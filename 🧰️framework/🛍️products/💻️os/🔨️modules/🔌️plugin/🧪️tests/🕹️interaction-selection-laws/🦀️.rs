use super::*;

/// 🔬️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B23 — the four verdicts of
/// [`interaction_selection_loss_v1`], each stated as the browser shape that produces it. The healthy
/// row is the one that matters most: a store that never answered is NOT a loss when the leftover
/// overlay puts the same ids back into the snapshot the render reads.
#[test]
fn interaction_selection_loss_names_every_way_a_pick_can_disappear() {
    let picked = "vortex=object:seed-left-001";
    let nothing = "vortex=object:";
    assert_eq!(interaction_selection_loss_v1(picked, picked, picked, true), None, "the render source carries the pick");
    assert_eq!(interaction_selection_loss_v1(picked, picked, picked, false), None, "an unchanged store is healthy while the render source still names the pick");
    assert_eq!(interaction_selection_loss_v1("", nothing, nothing, false), None, "a dispatch that carried no id can lose none");
    assert_eq!(interaction_selection_loss_v1(nothing, nothing, nothing, false), None, "a present-but-empty domain carried no id either");
    assert_eq!(interaction_selection_loss_v1(picked, nothing, nothing, true), Some("validate-state-pruned"), "validate_state dropped the pick against its topology");
    assert_eq!(interaction_selection_loss_v1(picked, picked, nothing, false), Some("persist-skipped"), "the validated half compared equal so no edit was minted and the pick never reached the render");
    assert_eq!(interaction_selection_loss_v1(picked, picked, nothing, true), Some("store-readback-lost"), "the store took the edit and did not answer with it");
}

/// 🔬️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31 — an app-authored selection write owns the
/// leftover overlay for the domains it names, so a deliberate emptying survives the very next read
/// while an unrelated domain's in-flight pick keeps its cover.
#[test]
fn an_app_selection_write_retires_the_leftover_overlay_of_the_domains_it_names() {
    let selection = |ids: &[&str]| protocol::DomainSelection { granularity: "object".to_string(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: ids.last().map(|id| id.to_string()) };
    let state = |entries: &[(&str, &[&str])]| protocol::InteractionState {
        selection: entries.iter().map(|(domain, ids)| (domain.to_string(), selection(ids))).collect(),
        hover: BTreeMap::new(),
        active_mode: BTreeMap::new(),
        active_granularity: BTreeMap::new(),
    };
    let overlay = state(&[("vortex", &["object-1"]), ("tree", &["node-7"])]);
    let leftover_ids = vec!["object-1".to_string(), "node-7".to_string()];
    let cleared = state(&[("vortex", &[]), ("tree", &["node-7"])]);
    let (retired, kept) = leftover_after_app_selection_write_v1(Some(&overlay), &leftover_ids, &cleared, &["vortex".to_string()]);
    let retired = retired.expect("an overlay that existed stays present");
    assert_eq!(retired.selection.get("vortex").map(|domain| domain.ids.clone()), Some(Vec::new()), "the written domain's overlay entry becomes the freshly computed selection, empty included");
    assert_eq!(retired.selection.get("tree").map(|domain| domain.ids.clone()), Some(vec!["node-7".to_string()]), "a domain the write never named keeps its cover");
    assert_eq!(kept, vec!["node-7".to_string()], "the flat leftover ids keep only what is still selected somewhere");

    let reselected = state(&[("vortex", &["clone-2"]), ("tree", &["node-7"])]);
    let (retired, kept) = leftover_after_app_selection_write_v1(Some(&overlay), &leftover_ids, &reselected, &["vortex".to_string()]);
    assert_eq!(retired.expect("overlay").selection.get("vortex").map(|domain| domain.ids.clone()), Some(vec!["clone-2".to_string()]), "a re-select publishes the new ids, not the pre-write ones");
    assert_eq!(kept, vec!["node-7".to_string()], "the replaced id leaves the flat leftover list with it");

    let (retired, kept) = leftover_after_app_selection_write_v1(None, &[], &cleared, &["vortex".to_string()]);
    assert!(retired.is_none() && kept.is_empty(), "no overlay in flight means nothing to retire");
}
