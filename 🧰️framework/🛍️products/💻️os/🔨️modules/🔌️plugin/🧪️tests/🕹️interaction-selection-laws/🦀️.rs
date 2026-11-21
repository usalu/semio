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

/// 🔬️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `selection-prune-interact` — a selection made in
/// one bundled example must not survive into the next one. Two examples, two documents: `shell@solid`
/// is picked in `Box Shell Preview`, then `Sphere Cut With Torus` is loaded and only ITS handles are
/// in topology, so `validate_state` empties the declared `graph` domain. Everything the pick left
/// behind has to go with it — the undeclared `vortex` mirror `overlay_leftover_ids_into_vortex` wrote,
/// the leftover cover, and the flat leftover ids — or the very next
/// `interaction_selection_snapshot` lays the dead id straight back over the empty store.
#[test]
fn a_document_change_prunes_the_vortex_mirror_and_the_leftover_cover_with_the_selection() {
    let selection = |ids: &[&str]| protocol::DomainSelection { granularity: "object".to_string(), ids: ids.iter().map(|id| id.to_string()).collect(), anchor_id: ids.last().map(|id| id.to_string()) };
    let state = |entries: &[(&str, &[&str])]| protocol::InteractionState {
        selection: entries.iter().map(|(domain, ids)| (domain.to_string(), selection(ids))).collect(),
        hover: BTreeMap::new(),
        active_mode: BTreeMap::new(),
        active_granularity: BTreeMap::new(),
    };
    let declared = vec!["graph".to_string()];

    let validated = state(&[("graph", &[]), ("vortex", &["shell@solid"])]);
    let overlay = state(&[("graph", &["shell@solid"]), ("vortex", &["shell@solid"])]);
    let leftover_ids = vec!["shell@solid".to_string()];
    let (pruned, retired, kept) = interaction_after_document_change_v1(&validated, &declared, Some(&overlay), &leftover_ids);
    assert_eq!(pruned.selection.get("vortex").map(|domain| domain.ids.clone()), Some(Vec::new()), "the mirror may only name what a declared domain still names");
    assert_eq!(pruned.selection.get("vortex").and_then(|domain| domain.anchor_id.clone()), None, "an anchor whose id is gone is gone with it");
    let retired = retired.expect("an overlay that existed stays present");
    assert_eq!(retired.selection.get("graph").map(|domain| domain.ids.clone()), Some(Vec::new()), "the cover may not outlive the ids it was covering");
    assert_eq!(retired.selection.get("vortex").map(|domain| domain.ids.clone()), Some(Vec::new()), "nor may the mirror's own cover");
    assert!(kept.is_empty(), "the flat leftover ids keep only what the new document still offers");

    let survived = state(&[("graph", &["brep_bool_cut_5@solid"]), ("vortex", &["brep_bool_cut_5@solid", "shell@solid"])]);
    let (pruned, retired, kept) = interaction_after_document_change_v1(&survived, &declared, Some(&overlay), &leftover_ids);
    assert_eq!(pruned.selection.get("graph").map(|domain| domain.ids.clone()), Some(vec!["brep_bool_cut_5@solid".to_string()]), "a declared domain keeps exactly what validate_state left it");
    assert_eq!(pruned.selection.get("vortex").map(|domain| domain.ids.clone()), Some(vec!["brep_bool_cut_5@solid".to_string()]), "the mirror drops the id no declared domain names any more");
    assert_eq!(retired.expect("overlay").selection.get("graph").map(|domain| domain.ids.clone()), Some(Vec::new()), "the cover of a pick the new document lost is retired even when another id survived");
    assert!(kept.is_empty(), "and its flat id with it");

    let untouched = state(&[("graph", &["shell@solid"]), ("vortex", &["shell@solid"])]);
    let (pruned, retired, kept) = interaction_after_document_change_v1(&untouched, &declared, Some(&overlay), &leftover_ids);
    assert_eq!(pruned, untouched, "a document edit that touched nothing selected changes nothing");
    assert_eq!(retired.expect("overlay").selection.get("graph").map(|domain| domain.ids.clone()), Some(vec!["shell@solid".to_string()]), "an in-flight pick during an unrelated edit keeps its cover");
    assert_eq!(kept, leftover_ids, "and its flat id");

    let flat_only = protocol::InteractionState { selection: [("nodes".to_string(), selection(&["widget-3"]))].into_iter().collect(), hover: BTreeMap::new(), active_mode: BTreeMap::new(), active_granularity: BTreeMap::new() };
    let (pruned, _, _) = interaction_after_document_change_v1(&flat_only, &["nodes".to_string()], None, &[]);
    assert_eq!(pruned.selection.get("nodes").map(|domain| domain.ids.clone()), Some(vec!["widget-3".to_string()]), "a DECLARED domain the topology has no entry for (Flat) has no membership information and keeps every id");
}

#[semio_framework_async_macros::async_test]
async fn retained_hover_admits_exact_targets_and_explicit_pointer_clears() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("👆️hover-targets.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let source = case["args"].as_object().unwrap();
        let arguments = DslValue::object(source.iter().map(|(key, value)| (key.clone(), DslValue::String(value.as_str().unwrap().to_owned()))));
        let parsed = parse_interaction_hover_targets(Some(&arguments), INTERACTION_HOVER_ACTION_ID).await;
        if case["fault"].as_bool().unwrap_or(false) {
            assert!(parsed.is_err(), "{}", case["name"]);
        } else {
            let targets = parsed.unwrap();
            let projected = targets.iter().map(|target| serde_json::json!({"id": target.id, "granularity": target.granularity})).collect::<Vec<_>>();
            assert_eq!(serde_json::json!(projected), case["expected"], "{}", case["name"]);
        }
    }
    let direct = &fixture["cases"][0];
    let id = direct["args"]["targetId"].as_str().unwrap();
    assert!(id.len() as u64 <= fixture["maximumInteractionIdBytes"].as_u64().unwrap());
    assert!(direct["expected"].to_string().len() as u64 > fixture["maximumUiTextBytes"].as_u64().unwrap());
}

#[semio_framework_async_macros::async_test]
async fn window_hover_groups_expand_before_topology_membership_validation() {
    use semio_plugin_interaction::interaction_topology::{
        INTERACTION_HOVER_GROUP_GRANULARITY, resolve_interaction_hover_group,
    };
    let ids = vec!["h:solid@!iport".to_owned(), "h:solid@port".to_owned()];
    let domain = protocol::DomainTopology {
        ordered: ids.iter().map(|id| protocol::TopologyNode {
            id: id.clone(), granularity: "handle".into(), parent: None,
        }).collect(),
    };
    let topology = protocol::InteractionTopology { domains: [("graph".into(), domain.clone())].into_iter().collect() };
    let outline = protocol::InteractionOutline {
        id: "graph".into(),
        granularity_ids: vec!["handle".into()],
        selection: protocol::SelectionSpec {
            modes: vec![protocol::SelectionMode::Single], methods: vec![], merges: vec![], transitive: false, broadcast: false,
        },
    };
    let args = DslValue::object([("targets".to_owned(), DslValue::String(serde_json::json!([
        {"granularity": INTERACTION_HOVER_GROUP_GRANULARITY, "id": "window-owned-group"}
    ]).to_string()))]);
    let parsed = parse_interaction_hover_targets(Some(&args), INTERACTION_HOVER_ACTION_ID).await.unwrap();
    let targets = resolve_interaction_hover_group(parsed, |reference| {
        assert_eq!(reference, "window-owned-group");
        Ok(Some(("handle", ids.clone())))
    }).unwrap().unwrap();
    let hover = protocol::next_hover(&protocol::HoverSpec::default(), &domain, &protocol::HoverInput {
        channel: "pointer".into(), targets,
    }).await;
    let state = protocol::InteractionState { hover: [("graph".into(), hover)].into_iter().collect(), ..Default::default() };
    let validated = protocol::validate_state(&[outline], &topology, &state).await;
    assert_eq!(validated.hover["graph"].ids, ids);
    let parsed = parse_interaction_hover_targets(Some(&args), INTERACTION_HOVER_ACTION_ID).await.unwrap();
    assert!(resolve_interaction_hover_group(parsed, |_| Ok(None)).unwrap().is_none());
    assert!(resolve_interaction_hover_group(vec![], |_| panic!("clear must bypass group lookup")).unwrap().unwrap().is_empty());
}
