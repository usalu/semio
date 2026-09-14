
use super::*;
use crate::editor::puzzle3d::precompute::Puzzle3dCollision;
use crate::standards::v1::subsets::any::schema::ObjectKindRepresentation;

#[test]
fn fill_distribution_excludes_zero_weight_vortices() {
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Placed".to_string(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![
                ObjectKindVortexTemplate { vortex_kind: Some("c-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() },
                ObjectKindVortexTemplate { vortex_kind: Some("b-s".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, 1.0]), ..Default::default() },
            ],
        }],
        vortices: vec![
            VortexKindCatalog { id: "c-b".to_string(), default_cable_kind: None, ..Default::default() },
            VortexKindCatalog { id: "c-t".to_string(), default_cable_kind: None, ..Default::default() },
            VortexKindCatalog { id: "b-s".to_string(), default_cable_kind: None, ..Default::default() },
        ],
        cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
    };
    let candidates = vec![BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 0 }, BrushCompatibleCandidate { object_kind_id: "Placed".to_string(), source_vortex_index: 1 }];
    let mut weights = BrushKindWeights::default();
    weights.vortex_weights.insert("c-b".to_string(), 0.0);
    weights.vortex_weights.insert("c-t".to_string(), 0.0);
    weights.vortex_weights.insert("b-s".to_string(), 1.0);
    weights.object_weights.insert("Placed".to_string(), 1.0);
    let mut rng = 7u32;
    let ordered = order_brush_fill_compatible_candidates(&candidates, Some("b-s"), 1, Some("Host"), &catalogs, &weights, &mut rng);
    assert_eq!(ordered.len(), 1);
    assert_eq!(ordered[0].source_vortex_index, 1);
    let targets = vec![
        BrushFillVortexTarget { full_id: "host:v0".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("c-t".to_string()), vortex_index: 0 },
        BrushFillVortexTarget { full_id: "host:v1".to_string(), object_id: "host".to_string(), object_kind: Some("Host".to_string()), vortex_kind: Some("b-s".to_string()), vortex_index: 1 },
    ];
    let target_ordered = weighted_order_fill_vortex_targets(&targets, &weights, &mut rng);
    assert_eq!(target_ordered.len(), 1);
    assert_eq!(target_ordered[0].vortex_kind.as_deref(), Some("b-s"));
}

#[test]
fn brush_placement_emits_attraction_with_id_and_directed_root() {
    let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Placed".to_string(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
        }],
        vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
        cables: vec![],
    };
    let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
    assert_eq!(next.attractions.len(), 1, "brush placement should append exactly one attraction");
    let attraction = &next.attractions[0];
    assert!(!attraction.id.is_empty(), "brush-placed attraction must carry a non-empty id (regression: engine attractions with no id were silently dropped by fixture_from_engine_json)");
    assert_eq!(attraction.attracting, "host:v0", "the pre-existing target vortex must stay the resolution root");
    assert!(attraction.attracted.starts_with(&format!("{}:", next.objects[0].id)), "the newly placed object's vortex must be the attracted (non-root) side");
    assert_eq!(attraction.gap, 0.0);
    assert_eq!(attraction.rotation, 0.0);
}

/// 🪪️ Regression: successive brush placements must mint distinct object ids when the fixture grows.
#[test]
fn successive_brush_placements_never_collide_on_object_id() {
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Placed".to_string(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/test/placed.glb".to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
        }],
        vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
        cables: vec![],
    };
    let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".to_string(), object_kind_id: "Placed".to_string(), source_vortex_index: 0, origin: [1.0, 2.0, 3.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    let mut fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
    let mut ids = std::collections::HashSet::new();
    for i in 0..8 {
        fixture = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
        let placed = fixture.objects.last().expect("placement should append an object");
        assert!(ids.insert(placed.id.clone()), "brush placement #{i} minted a duplicate object id {:?}", placed.id);
        // Successive placements target the same fixed `host:v0`, so only the first actually attaches;
        // reset attractions so every iteration re-exercises `apply_brush_placement_to_fixture` fresh.
        fixture.attractions.clear();
    }
}

#[test]
fn vortex_port_shape_and_compatibility() {
    assert_eq!(puzzle3d_vortex_port_shape("foo circular bar"), Some("circular"));
    assert_eq!(puzzle3d_vortex_port_shape("foo rectangular bar"), Some("rectangular"));
    assert_eq!(puzzle3d_vortex_port_shape("plain"), None);
    assert!(puzzle3d_vortex_port_shapes_compatible("plain", "foo circular bar"));
    assert!(puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz circular qux"));
    assert!(!puzzle3d_vortex_port_shapes_compatible("foo circular bar", "baz rectangular qux"));
}

#[test]
fn single_letter_port_family_and_compatibility() {
    assert_eq!(puzzle3d_single_letter_port_family("a-socket"), Some('a'));
    assert_eq!(puzzle3d_single_letter_port_family("ab-socket"), None);
    assert_eq!(puzzle3d_single_letter_port_family("A-socket"), None);
    assert_eq!(puzzle3d_single_letter_port_family("plain"), None);
    assert!(puzzle3d_single_letter_port_families_compatible("plain", "a-socket"));
    assert!(puzzle3d_single_letter_port_families_compatible("a-socket", "a-plug"));
    assert!(!puzzle3d_single_letter_port_families_compatible("a-socket", "b-plug"));
}

#[test]
fn resolve_cable_and_attraction_kind_defaults_and_lookup() {
    let catalogs = KindCatalogBundle {
        objects: vec![],
        vortices: vec![VortexKindCatalog { id: "vk".into(), default_cable_kind: Some("  cable.custom  ".into()), ..Default::default() }, VortexKindCatalog { id: "vk-empty".into(), default_cable_kind: Some("   ".into()), ..Default::default() }],
        cables: vec![CableKindCatalog { id: "cable.custom".into(), default_attraction_kind: Some("attraction.custom".into()), ..Default::default() }],
    };
    assert_eq!(resolve_cable_kind_for_vortex("vk", &catalogs), "cable.custom");
    assert_eq!(resolve_cable_kind_for_vortex("vk-empty", &catalogs), DEFAULT_CABLE_KIND_ID);
    assert_eq!(resolve_cable_kind_for_vortex("missing", &catalogs), DEFAULT_CABLE_KIND_ID);
    assert_eq!(resolve_attraction_kind_for_cable("cable.custom", &catalogs), "attraction.custom");
    assert_eq!(resolve_attraction_kind_for_cable("missing", &catalogs), "");
}

#[test]
fn compat_pair_matches_and_specificity_rank() {
    let rule = KindCompatEntry { source: "a".into(), target: "b".into(), bidirectional: false, important: false, specificity: None };
    assert!(compat_pair_matches(&rule, "a", "b"));
    assert!(!compat_pair_matches(&rule, "b", "a"));
    let bidi = KindCompatEntry { bidirectional: true, ..rule };
    assert!(compat_pair_matches(&bidi, "b", "a"));
    assert_eq!(specificity_rank(Some("general")), 0);
    assert_eq!(specificity_rank(Some("object")), 1);
    assert_eq!(specificity_rank(Some("attraction")), 2);
    assert_eq!(specificity_rank(Some("cable")), 3);
    assert_eq!(specificity_rank(Some("vortex")), 4);
    assert_eq!(specificity_rank(Some("unknown")), 4);
    assert_eq!(specificity_rank(None), 4);
}

#[test]
fn attraction_gesture_rule_applies_specificity_branches() {
    let catalogs = KindCatalogBundle {
        objects: vec![],
        vortices: vec![VortexKindCatalog { id: "sv".into(), default_cable_kind: Some("cable.a".into()), ..Default::default() }, VortexKindCatalog { id: "tv".into(), default_cable_kind: Some("cable.b".into()), ..Default::default() }],
        cables: vec![CableKindCatalog { id: "cable.a".into(), default_attraction_kind: Some("attr.a".into()), ..Default::default() }, CableKindCatalog { id: "cable.b".into(), default_attraction_kind: Some("attr.b".into()), ..Default::default() }],
    };
    let attracting = AttractionVortexContext { object_kind: Some("ObjA".into()), vortex_kind: Some("sv".into()) };
    let attracted = AttractionVortexContext { object_kind: Some("ObjB".into()), vortex_kind: Some("tv".into()) };
    let rule_for = |source: &str, target: &str, specificity: Option<&str>| KindCompatEntry { source: source.into(), target: target.into(), bidirectional: false, important: false, specificity: specificity.map(String::from) };
    assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("general")), &attracting, &attracted, &catalogs));
    assert!(attraction_gesture_rule_applies(&rule_for("ObjA", "ObjB", Some("object")), &attracting, &attracted, &catalogs));
    assert!(attraction_gesture_rule_applies(&rule_for("attr.a", "attr.b", Some("attraction")), &attracting, &attracted, &catalogs));
    assert!(attraction_gesture_rule_applies(&rule_for("cable.a", "cable.b", Some("cable")), &attracting, &attracted, &catalogs));
    assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", None), &attracting, &attracted, &catalogs));
    assert!(attraction_gesture_rule_applies(&rule_for("sv", "tv", Some("weird")), &attracting, &attracted, &catalogs));
    assert!(!attraction_gesture_rule_applies(&rule_for("sv", "other", Some("general")), &attracting, &attracted, &catalogs));
}

#[test]
fn vortices_attraction_compatible_for_drag_branches() {
    let catalogs = KindCatalogBundle { objects: vec![], vortices: vec![], cables: vec![] };
    let a_circ = AttractionVortexContext { object_kind: None, vortex_kind: Some("x circular y".into()) };
    let a_rect = AttractionVortexContext { object_kind: None, vortex_kind: Some("x rectangular y".into()) };
    assert!(!vortices_attraction_compatible_for_drag(&a_circ, &a_rect, &[], &catalogs), "incompatible port shapes must reject regardless of rules");

    let a_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("a-socket".into()) };
    let b_letter = AttractionVortexContext { object_kind: None, vortex_kind: Some("b-plug".into()) };
    assert!(!vortices_attraction_compatible_for_drag(&a_letter, &b_letter, &[], &catalogs), "mismatched single-letter families must reject");

    let sv = AttractionVortexContext { object_kind: None, vortex_kind: Some("sv".into()) };
    let tv = AttractionVortexContext { object_kind: None, vortex_kind: Some("tv".into()) };
    assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[], &catalogs), "no rules means compatible");

    let unrelated = KindCompatEntry { source: "sv".into(), target: "other".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
    assert!(!vortices_attraction_compatible_for_drag(&sv, &tv, &[unrelated], &catalogs), "no matching rule must reject");

    let low = KindCompatEntry { source: "sv".into(), target: "tv".into(), bidirectional: false, important: false, specificity: Some("general".into()) };
    let important = KindCompatEntry { important: true, ..low.clone() };
    assert!(vortices_attraction_compatible_for_drag(&sv, &tv, &[low, important], &catalogs), "an important match among matched rules must keep it compatible");
}

#[test]
fn brush_stack_pair_helpers() {
    assert_eq!(brush_stack_vortex_base("column bottom"), Some("column"));
    assert_eq!(brush_stack_vortex_base("column top"), Some("column"));
    assert_eq!(brush_stack_vortex_base("column"), None);
    assert!(brush_stack_bottom_top_pair("column bottom", "column top"));
    assert!(!brush_stack_bottom_top_pair("column top", "column bottom"));
    assert!(brush_stack_top_bottom_pair("column top", "column bottom"));
    assert!(!brush_stack_top_bottom_pair("column bottom", "column top"));
    assert!(brush_stack_mate_pair("column bottom", "column top"));
    assert!(brush_stack_mate_pair("column top", "column bottom"));
    assert!(!brush_stack_mate_pair("column bottom", "beam top"));
    assert!(!brush_stack_mate_pair("x circular column bottom", "x rectangular column top"), "incompatible port shapes must reject even a stack mate pair");
}

#[test]
fn brush_candidate_rank_scores_kind_match_and_stack_and_tambour_rules() {
    let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
    let template = ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() };
    let same_kind = BrushCompatibleCandidate { object_kind_id: "Host".into(), source_vortex_index: 0 };
    let score = brush_candidate_rank(&same_kind, &template, &target);
    assert_eq!(score, 15_000, "matching object kind (+10000) plus a stack mate pair (+5000)");

    let target_tambour = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
    let capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door tambour circular".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() };
    let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
    assert!(brush_candidate_rank(&capital, &capsule_template, &target_tambour) < 0, "capital on tambour must be penalized");

    let cylindric = BrushCompatibleCandidate { object_kind_id: "Cylindric Tambour".into(), source_vortex_index: 0 };
    assert!(brush_candidate_rank(&cylindric, &capsule_template, &target_tambour) > 0, "cylindric tambour stacking onto a mid-tambour host should score positively");
}

#[test]
fn host_accepts_candidate_rule_branches() {
    let rules = BrushHostRules::default();
    let target = AttractionVortexContext { object_kind: Some("Tambour".into()), vortex_kind: Some("door tambour circular".into()) };
    let door_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), point: [1.0, 0.0, 0.0], direction: None, ..Default::default() };

    let capital = BrushCompatibleCandidate { object_kind_id: "Capital".into(), source_vortex_index: 0 };
    assert!(!host_accepts_candidate(&rules, &target, &capital, &door_capsule_template), "reject_capital_on_tambour must reject Capital");

    let storey = BrushCompatibleCandidate { object_kind_id: "Last Storey".into(), source_vortex_index: 0 };
    assert!(!host_accepts_candidate(&rules, &target, &storey, &door_capsule_template), "reject_last_single_storey_on_mid_tambour must reject Last Storey on a Tambour host");

    let door_ok = BrushCompatibleCandidate { object_kind_id: "Door".into(), source_vortex_index: 0 };
    assert!(host_accepts_candidate(&rules, &target, &door_ok, &door_capsule_template), "a door capsule far enough on x and close enough on y must be accepted");

    let non_capsule_template = ObjectKindVortexTemplate { vortex_kind: Some("not a capsule".into()), point: [1.0, 0.0, 0.0], direction: None, ..Default::default() };
    assert!(!host_accepts_candidate(&rules, &target, &door_ok, &non_capsule_template), "a door tambour target requires a door-capsule source vortex");

    let close_template = ObjectKindVortexTemplate { vortex_kind: Some("door capsule".into()), point: [0.1, 0.0, 0.0], direction: None, ..Default::default() };
    assert!(!host_accepts_candidate(&rules, &target, &door_ok, &close_template), "the door capsule position must satisfy the minimum absolute x");

    let door_rule_off = BrushHostRules { door_tambour_requires_door_capsule: false, ..BrushHostRules::default() };
    assert!(host_accepts_candidate(&door_rule_off, &target, &door_ok, &non_capsule_template), "disabling door_tambour_requires_door_capsule accepts regardless of the source vortex kind");
}

#[test]
fn brush_placement_uses_host_orientation_branches() {
    let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
    assert!(!brush_placement_uses_host_orientation(&target, "column bottom", "Host"), "stack mate pairs never use host orientation");
    assert!(!brush_placement_uses_host_orientation(&target, "other", "Host"), "different vortex kinds never use host orientation");
    assert!(brush_placement_uses_host_orientation(&target, "column top", "Host"), "matching vortex kind and object kind uses host orientation");
    assert!(!brush_placement_uses_host_orientation(&target, "column top", "OtherKind"), "matching vortex kind but a different candidate kind rejects host orientation");
}

#[test]
fn resolve_object_kind_mesh_url_prefers_catalog_then_falls_back_to_fixture() {
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Kind".into(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/catalog.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![],
        }],
        vortices: vec![],
        cables: vec![],
    };
    let fixture = Fixture { attractions: vec![], target_volumes: vec![], objects: vec![] };
    assert_eq!(resolve_object_kind_mesh_url("Kind", &catalogs, &fixture), Some("/catalog.glb".to_string()));

    let empty_catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Kind".into(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![],
        }],
        vortices: vec![],
        cables: vec![],
    };
    let fixture_with_object = Fixture {
        attractions: vec![],
        target_volumes: vec![],
        objects: vec![FixtureObject {
            id: "o1".into(),
            object_kind: Some("Kind".into()),
            anchor: Default::default(),
            mesh_url: Some("/fixture.glb".into()),
            origin: [0.0, 0.0, 0.0],
            orientation: None,
            scale: None,
            vortices: vec![],
        }],
    };
    assert_eq!(resolve_object_kind_mesh_url("Kind", &empty_catalogs, &fixture_with_object), Some("/fixture.glb".to_string()));
    assert_eq!(resolve_object_kind_mesh_url("Missing", &empty_catalogs, &fixture_with_object), None);
}

#[test]
fn brush_compatible_candidates_filters_and_sorts() {
    let catalogs = KindCatalogBundle {
        objects: vec![
            ObjectKind { id: "NoMesh".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }] },
            ObjectKind {
                id: "NoVortices".into(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/a.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![],
            },
            ObjectKind {
                id: "Match".into(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/b.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }],
            },
        ],
        vortices: vec![],
        cables: vec![],
    };
    let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("sv".into()) };
    let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
    assert_eq!(candidates.len(), 1, "kinds with no mesh url or no vortices must be excluded: {candidates:?}");
    assert_eq!(candidates[0].object_kind_id, "Match");
}

#[test]
fn brush_compatible_candidates_stack_target_only_matches_mates() {
    let catalogs = KindCatalogBundle {
        objects: vec![
            ObjectKind {
                id: "Mate".into(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/a.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("column bottom".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }],
            },
            ObjectKind {
                id: "NotMate".into(),
                representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/b.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
                scale: None,
                vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("beam".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }],
            },
        ],
        vortices: vec![],
        cables: vec![],
    };
    let target = AttractionVortexContext { object_kind: Some("Host".into()), vortex_kind: Some("column top".into()) };
    let candidates = brush_compatible_candidates(&target, &catalogs, &[], &BrushHostRules::default());
    assert_eq!(candidates.len(), 1, "a stack-top target must only match stack mates: {candidates:?}");
    assert_eq!(candidates[0].object_kind_id, "Mate");
}

#[test]
fn blocked_vortex_full_ids_and_enumeration_excludes_them() {
    let attractions = vec![AttractionProps { id: "a1".into(), attracting: "host:v0".into(), attracted: "guest:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 }];
    let blocked = blocked_vortex_full_ids(&attractions);
    assert!(blocked.contains("host:v0") && blocked.contains("guest:v0"));

    let fixture = Fixture {
        attractions,
        target_volumes: vec![],
        objects: vec![
            FixtureObject {
                id: "host".into(),
                object_kind: Some("Host".into()),
                anchor: Default::default(),
                mesh_url: None,
                origin: [0.0, 0.0, 0.0],
                orientation: None,
                scale: None,
                vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }],
            },
            FixtureObject {
                id: "free".into(),
                object_kind: Some("Free".into()),
                anchor: Default::default(),
                mesh_url: None,
                origin: [0.0, 0.0, 0.0],
                orientation: None,
                scale: None,
                vortices: vec![VortexProps { id: "v0".into(), vortex_kind: None, position: [0.0, 0.0, 0.0], direction: None }],
            },
        ],
    };
    let targets = enumerate_brush_fill_vortex_targets(&fixture);
    assert_eq!(targets.len(), 1);
    assert_eq!(targets[0].full_id, "free:v0");
}

#[test]
fn vortex_world_from_object_none_for_missing_index() {
    let object = FixtureObject { id: "o".into(), object_kind: None, anchor: Default::default(), mesh_url: None, origin: [1.0, 2.0, 3.0], orientation: None, scale: None, vortices: vec![] };
    assert!(vortex_world_from_object(&object, 0).is_none());
}

#[test]
fn weight_lookup_helpers_default_to_one_or_gate_on_zero() {
    let mut weights = BrushKindWeights::default();
    weights.object_weights.insert("A".into(), 2.0);
    weights.vortex_weights.insert("v".into(), 0.0);
    assert_eq!(brush_kind_weight_value(&weights.object_weights, "A"), 2.0);
    assert_eq!(brush_kind_weight_value(&weights.object_weights, "missing"), 1.0);
    assert!(!brush_target_vortex_allows_suggestion(Some("v"), &weights));
    assert!(brush_target_vortex_allows_suggestion(Some("other"), &weights));
    assert!(brush_target_vortex_allows_suggestion(None, &weights));

    let target = BrushFillVortexTarget { full_id: "f".into(), object_id: "o".into(), object_kind: None, vortex_kind: Some("v".into()), vortex_index: 0 };
    assert_eq!(fill_vortex_target_weight(&target, &weights), 0.0);
}

#[test]
fn weighted_sample_without_replacement_edge_cases() {
    let items = vec![1, 2, 3];
    let mut rng = 42u32;
    let single: Vec<i32> = weighted_sample_without_replacement(&[1], |_| 1.0, &mut rng);
    assert_eq!(single, vec![1]);
    let all_zero: Vec<i32> = weighted_sample_without_replacement(&items, |_| 0.0, &mut rng);
    assert!(all_zero.is_empty(), "all-zero weights leave nothing eligible");
    let sampled = weighted_sample_without_replacement(&items, |_| 1.0, &mut rng);
    let mut sorted = sampled;
    sorted.sort_unstable();
    assert_eq!(sorted, items, "every eligible item appears exactly once");
}

#[test]
fn fill_rng_is_deterministic_for_a_given_seed() {
    let mut a = 123u32;
    let mut b = 123u32;
    for _ in 0..5 {
        assert_eq!(fill_rng(&mut a), fill_rng(&mut b));
    }
    assert_ne!(a, 123);
}

#[test]
fn fill_candidate_diversity_score_rewards_distance_within_same_kind() {
    let candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 3 };
    assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Other")), 0, "a different target object kind never scores");
    assert_eq!(fill_candidate_diversity_score(&candidate, 0, Some("Kind")), 1000 + 300);
    assert_eq!(fill_candidate_diversity_score(&candidate, 3, Some("Kind")), 1000);
}

#[test]
fn brush_preview_from_candidate_none_branches() {
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Kind".into(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/mesh.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }],
        }],
        vortices: vec![],
        cables: vec![],
    };
    let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
    let target_ctx = AttractionVortexContext { object_kind: None, vortex_kind: None };
    let world = TargetVortexWorld { position: [0.0, 0.0, 0.0], direction: [0.0, 0.0, -1.0], reference_orientation: None };

    let missing_kind = BrushCompatibleCandidate { object_kind_id: "Missing".into(), source_vortex_index: 0 };
    assert!(brush_preview_from_candidate("t", &missing_kind, &target_ctx, world, &catalogs, &fixture).is_none());

    let bad_index = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 5 };
    assert!(brush_preview_from_candidate("t", &bad_index, &target_ctx, world, &catalogs, &fixture).is_none());

    let empty_mesh_catalogs = KindCatalogBundle {
        objects: vec![ObjectKind { id: "Kind".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }] }],
        vortices: vec![],
        cables: vec![],
    };
    let ok_candidate = BrushCompatibleCandidate { object_kind_id: "Kind".into(), source_vortex_index: 0 };
    assert!(brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &empty_mesh_catalogs, &fixture).is_none(), "a missing mesh url must yield no preview");

    let preview = brush_preview_from_candidate("t", &ok_candidate, &target_ctx, world, &catalogs, &fixture).expect("a valid candidate should produce a preview");
    assert_eq!(preview.mesh_url, "/mesh.glb");
    assert_eq!(preview.object_kind_id, "Kind");
}

#[test]
fn apply_brush_placement_to_fixture_rejects_missing_kind_template_or_mesh() {
    let fixture = Fixture { attractions: vec![], objects: vec![], target_volumes: vec![] };
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind { id: "Kind".into(), representations: vec![], scale: None, vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }] }],
        vortices: vec![],
        cables: vec![],
    };

    let missing_kind = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Missing".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_kind, &catalogs).objects.len(), 0);

    let missing_template = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 9, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_template, &catalogs).objects.len(), 0);

    let missing_mesh = BrushPlacePayload { target_vortex_full_id: "t:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    assert_eq!(apply_brush_placement_to_fixture(&fixture, &missing_mesh, &catalogs).objects.len(), 0, "no resolvable mesh url means the placement must be rejected");
}

#[test]
fn apply_brush_placement_to_fixture_rejects_duplicate_attraction_target() {
    let catalogs = KindCatalogBundle {
        objects: vec![ObjectKind {
            id: "Kind".into(),
            representations: vec![ObjectKindRepresentation { id: "r0".into(), name: String::new(), url: "/mesh.glb".into(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
            scale: None,
            vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("sv".into()), point: [0.0, 0.0, 0.0], direction: None, ..Default::default() }],
        }],
        vortices: vec![],
        cables: vec![],
    };
    let payload = BrushPlacePayload { target_vortex_full_id: "host:v0".into(), object_kind_id: "Kind".into(), source_vortex_index: 0, origin: [0.0, 0.0, 0.0], orientation: [0.0, 0.0, 0.0, 1.0], scale: None };
    let fixture = Fixture {
        attractions: vec![AttractionProps { id: "a".into(), attracting: "host:v0".into(), attracted: "other:v0".into(), gap: 0.0, shift: 0.0, rise: 0.0, rotation: 0.0, turn: 0.0, tilt: 0.0, x: 0.0, y: 0.0 }],
        objects: vec![],
        target_volumes: vec![],
    };
    let next = apply_brush_placement_to_fixture(&fixture, &payload, &catalogs);
    assert_eq!(next.objects.len(), 0, "a target vortex that is already attracting must reject the placement");
}

//#region ⏯️BrushSuggestionsRun
/// ⏯️ Ticket 26/09/13/INTERACTIVE-TOOLS-VISIBLE-PROCESS lane W2-C — the brush candidate search is a read-only
/// tool run: every candidate it tests is a trace record, every verdict reaches the instance's link, and the job
/// follows the link's target. The scene and the laws are read from `🧫️fixtures/🎞️brush-suggestions-run.json`.
use crate::editor::puzzle3d::Puzzle3dInstanceOperationOwner;
use semio_framework_job::{root_cancel_token, Generation, OperationId, StepBudget};
use semio_framework_tool_run::{ToolRunId, ToolRunStep, ToolRunTick, ToolRunTraceOp, ToolRunTraceStore};

const BRUSH_SUGGESTIONS_RUN_FIXTURE: &str = include_str!("../../🧫️fixtures/🎞️brush-suggestions-run.json");
const BRUSH_SEARCH_TARGET: &str = "host:v0";

fn brush_search_cube() -> (Vec<f32>, Vec<u32>) {
    (
        vec![-4.0, -4.0, -4.0, 4.0, -4.0, -4.0, 4.0, 4.0, -4.0, -4.0, 4.0, -4.0, -4.0, -4.0, 4.0, 4.0, -4.0, 4.0, 4.0, 4.0, 4.0, -4.0, 4.0, 4.0],
        vec![0, 1, 2, 0, 2, 3, 4, 6, 5, 4, 7, 6, 0, 4, 5, 0, 5, 1, 2, 6, 7, 2, 7, 3, 0, 3, 7, 0, 7, 4, 1, 5, 6, 1, 6, 2],
    )
}

fn brush_search_kind(id: &str, url: &str) -> ObjectKind {
    ObjectKind {
        id: id.to_string(),
        representations: vec![ObjectKindRepresentation { id: id.to_lowercase(), name: String::new(), url: url.to_string(), mime: String::new(), tags: vec![], lod: None, description: String::new() }],
        scale: None,
        vortices: vec![ObjectKindVortexTemplate { vortex_kind: Some("port-b".to_string()), point: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]), ..Default::default() }],
    }
}

/// 🧪️ The fixture scene: one host vortex, two compatible kinds, and optionally a body of the host's own kind
/// carrying its own mesh parked exactly where both candidates dock.
fn brush_search_scene(blocker: bool) -> SceneConfig {
    let mut objects = vec![FixtureObject {
        id: "host".to_string(),
        object_kind: Some("Host".to_string()),
        anchor: Default::default(),
        mesh_url: Some("/test/unregistered.glb".to_string()),
        origin: [12.0, 0.0, 0.0],
        orientation: Some([0.0, 0.0, 0.0, 1.0]),
        scale: None,
        vortices: vec![VortexProps { id: "v0".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 0.0, 0.0], direction: Some([0.0, 0.0, -1.0]) }],
    }];
    if blocker {
        objects.push(FixtureObject { id: "blocker".to_string(), mesh_url: Some("/test/blocker.glb".to_string()), vortices: vec![], ..objects[0].clone() });
    }
    SceneConfig {
        fixture: Fixture { attractions: vec![], target_volumes: vec![], objects },
        kind_catalogs: Some(KindCatalogBundle {
            objects: vec![brush_search_kind("Near", "/test/near.glb"), brush_search_kind("Late", "/test/late.glb")],
            vortices: vec![VortexKindCatalog { id: "port-a".to_string(), default_cable_kind: None, ..Default::default() }, VortexKindCatalog { id: "port-b".to_string(), default_cable_kind: None, ..Default::default() }],
            cables: vec![CableKindCatalog { id: "cable.link".to_string(), default_attraction_kind: None, ..Default::default() }],
        }),
        kind_compatibility: vec![KindCompatEntry { source: "port-b".to_string(), target: "port-a".to_string(), bidirectional: true, important: false, specificity: Some("vortex".to_string()) }],
        overlap_budget: 0.02,
        seed: 1,
        host_rules: BrushHostRules::default(),
        weights: BrushKindWeights::default(),
    }
}

/// 🥽️ The fixture's mesh store: every registered test mesh is the cube, the host's own mesh is unknown.
fn brush_search_meshes(url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    (url != "/test/unregistered.glb").then(brush_search_cube)
}

fn brush_search_lane() -> Vec<String> {
    ["box", "/test/blocker.glb", "/test/late.glb", "/test/near.glb", "/test/unregistered.glb"].map(str::to_string).to_vec()
}

fn brush_run_identity(generation: u32) -> ToolRunIdentity {
    ToolRunIdentity { generation, ..ToolRunIdentity::new(ToolRunId { app_instance_id: 1, run: 1 }, [0; 32]) }
}

fn brush_run_owner(target: Option<&str>) -> ArtifactInstanceOperationOwnerHandle {
    let owner = ArtifactInstanceOperationOwnerHandle::new(Box::new(Puzzle3dInstanceOperationOwner::default()));
    brush_run_link(&owner, |link| link.hover(target.map(str::to_string)));
    owner
}

fn brush_run_link<R>(owner: &ArtifactInstanceOperationOwnerHandle, apply: impl FnOnce(&mut BrushSuggestionsLink) -> R) -> R {
    owner.with_mut::<Puzzle3dInstanceOperationOwner, _>(|owner| Ok(apply(&mut owner.brush_suggestions))).expect("the brush suggestions link")
}

fn brush_run_job(owner: &ArtifactInstanceOperationOwnerHandle, port: &ToolRunJobPort, scene: SceneConfig, lane: Vec<String>, meshes: BrushSuggestionsMeshSource, generation: u32) -> BrushSuggestionsRunJob<Puzzle3dInstanceOperationOwner> {
    BrushSuggestionsRunJob::new(owner.clone(), port.clone(), brush_run_identity(generation), Arc::new(scene), lane, meshes, crate::editor::puzzle3d::puzzle3d_fallback_mesh_buffers()).expect("fallback body")
}

fn brush_run_never() -> Option<u64> {
    Some(0)
}

/// 🚦️ One job step with `fuel` units and no deadline, its retained tick page decoded and returned to its ledger.
fn brush_run_step(job: &mut BrushSuggestionsRunJob<Puzzle3dInstanceOperationOwner>, fuel: u64) -> Option<ToolRunTick> {
    let mut sequence = 0;
    let mut context = StepContext::new(OperationId(91), Generation(1), StepBudget::new(fuel, u64::MAX), root_cancel_token(), brush_run_never, &mut sequence);
    brush_run_settle(job.step(&mut context))
}

fn brush_run_settle(outcome: StepOutcome) -> Option<ToolRunTick> {
    match outcome {
        StepOutcome::PreviewReady(mut payload) => {
            let page = payload.single_page().expect("a tick is one payload page").to_vec();
            while !payload.terminal_is_empty() {
                payload.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            }
            Some(ToolRunTick::decode(&page).expect("tick decodes"))
        }
        StepOutcome::Yield => None,
        other => panic!("a brush suggestions run only yields ticks: {other:?}"),
    }
}

/// 🪞️ The ledger side of a brush suggestions run: the resident trace, every record in order, the steps.
struct BrushRunMirror {
    trace: ToolRunTraceStore,
    records: Vec<(u64, ToolRunVerdict, u16)>,
    clears: usize,
    steps: Vec<ToolRunStep>,
    progress: Option<ToolRunProgress>,
}

impl BrushRunMirror {
    fn new(generation: u32) -> Self {
        Self { trace: ToolRunTraceStore::new(brush_run_identity(generation)), records: Vec::new(), clears: 0, steps: Vec::new(), progress: None }
    }

    fn apply(&mut self, tick: ToolRunTick) {
        for page in &tick.trace {
            for op in &page.ops {
                match op {
                    ToolRunTraceOp::Upsert { key, verdict, reason, .. } => self.records.push((*key, *verdict, *reason)),
                    ToolRunTraceOp::Clear => self.clears += 1,
                    ToolRunTraceOp::Retire { .. } => {}
                }
            }
            self.trace.apply_page(page).expect("pages of the one run and generation");
        }
        self.steps.extend(tick.steps);
        if tick.progress.is_some() {
            self.progress = tick.progress;
        }
    }

    /// 🔁️ Steps until the job waits on its port; answers the steps it took.
    fn drive(&mut self, job: &mut BrushSuggestionsRunJob<Puzzle3dInstanceOperationOwner>, port: &ToolRunJobPort, fuel: u64) -> usize {
        for step in 0..100_000 {
            if port.is_waiting() {
                return step;
            }
            if let Some(tick) = brush_run_step(job, fuel) {
                self.apply(tick);
            }
        }
        panic!("the brush suggestions run never settled");
    }

    fn finals(&self) -> Vec<(u64, ToolRunVerdict, u16)> {
        self.records.iter().copied().filter(|(_, verdict, _)| *verdict != ToolRunVerdict::Testing).collect()
    }

    fn words(&self) -> Vec<String> {
        self.finals().iter().map(|(_, verdict, reason)| format!("{}:{}", verdict.as_str(), BrushSuggestionsRunReason::ALL[usize::from(*reason)].id())).collect()
    }

    fn step_words(&self) -> Vec<String> {
        self.steps
            .iter()
            .map(|step| {
                let args = step.args.iter().map(|arg| arg.to_plain_string()).collect::<Vec<_>>().join(",");
                let base = format!("{}:{}", step.kind.as_str(), BrushSuggestionsRunReason::ALL[usize::from(step.reason)].id());
                if args.is_empty() { base } else { format!("{base}:{args}") }
            })
            .collect()
    }
}

/// ⚖️ LANGUAGE-NEUTRAL LAW: every fixture case reaches exactly its verdicts, free kinds, counters and steps; every
/// candidate is first upserted `testing` and then decided exactly once; the link holds the same result.
#[test]
fn brush_suggestions_run_matches_the_language_neutral_fixture() {
    let fixture: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run fixture");
    for case in fixture["cases"].as_array().expect("cases") {
        let name = case["name"].as_str().expect("name");
        let mut scene = brush_search_scene(case["blocker"].as_bool().expect("blocker"));
        for (kind, weight) in case["vortexWeights"].as_object().expect("vortex weights") {
            scene.weights.vortex_weights.insert(kind.clone(), weight.as_f64().expect("weight"));
        }
        let target = case["target"].as_str().expect("target");
        let (owner, port) = (brush_run_owner(Some(target)), ToolRunJobPort::default());
        let mut job = brush_run_job(&owner, &port, scene, brush_search_lane(), brush_search_meshes, 0);
        let mut mirror = BrushRunMirror::new(0);
        mirror.drive(&mut job, &port, u64::MAX);
        let expect_words = |key: &str| case[key].as_array().expect(key).iter().map(|word| word.as_str().expect("word").to_string()).collect::<Vec<_>>();
        assert_eq!(mirror.words(), expect_words("verdicts"), "{name}: verdicts");
        assert_eq!(mirror.step_words(), expect_words("steps"), "{name}: steps");
        let progress = mirror.progress.as_ref().expect("a settled search reports its progress");
        assert_eq!(progress.counters.iter().map(|counter| counter.value).collect::<Vec<_>>(), case["counters"].as_array().expect("counters").iter().map(|value| value.as_u64().expect("counter")).collect::<Vec<_>>(), "{name}: counters");
        assert_eq!(progress.stage, BrushSuggestionsRunStage::Idle.index(), "{name}: a settled search waits in the idle stage");
        for (key, _, _) in mirror.finals() {
            let upserts: Vec<ToolRunVerdict> = mirror.records.iter().filter(|(record, _, _)| *record == key).map(|(_, verdict, _)| *verdict).collect();
            assert_eq!(upserts.first(), Some(&ToolRunVerdict::Testing), "{name}: candidate {key} is shown under test before it is decided");
            assert_eq!(upserts.iter().filter(|verdict| **verdict != ToolRunVerdict::Testing).count(), 1, "{name}: candidate {key} is decided exactly once");
            assert_eq!(mirror.trace.record(key).map(|record| record.verdict), mirror.finals().iter().find(|(record, _, _)| *record == key).map(|(_, verdict, _)| *verdict), "{name}: the resident trace keeps the verdict");
        }
        let free = brush_run_link(&owner, |link| link.found(target).map(|found| found.free().map(|preview| preview.object_kind_id.clone()).collect::<Vec<_>>()).unwrap_or_default());
        assert_eq!(free, expect_words("free"), "{name}: the link lists exactly the free candidates");
        assert!(brush_run_link(&owner, |link| link.found(target).is_some_and(|found| found.done)), "{name}: the link reads the search as done");
    }
}

/// 🐛️ Red→green (`📓️wave-W1-B.md` §6, `📓️wave-W0-C2.md` §3): a placed body of the host's kind that carries its OWN
/// mesh was resolved by kind — to the host's unregistered mesh — so it was never a collision body and every candidate
/// docking into it read free. Both the run and the precompute lane puzzle 5d still reads resolve a placed object's own
/// mesh first, the renderer's law.
#[test]
fn a_body_carrying_its_own_mesh_where_every_candidate_docks_collides_them_all() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut job = brush_run_job(&owner, &port, brush_search_scene(true), brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.words(), vec!["danger:collision", "danger:collision"]);
    let (positions, indices) = brush_search_cube();
    let mut engine = Puzzle3dCollision::new();
    for url in ["/test/near.glb", "/test/late.glb", "/test/blocker.glb"] {
        engine.register_mesh(url.to_string(), &positions, &indices);
    }
    engine.scene = Some(Arc::new(brush_search_scene(true)));
    let result = (0..64).map(|_| engine.compute_brush_cache_entry(BRUSH_SEARCH_TARGET)).find(|result| !result.unknown_pending).expect("the precompute lane settles");
    assert!(result.free.is_empty(), "every candidate docks into the parked body: {:?}", result.free);
    let mut clear = Puzzle3dCollision::new();
    for url in ["/test/near.glb", "/test/late.glb", "/test/blocker.glb"] {
        clear.register_mesh(url.to_string(), &positions, &indices);
    }
    clear.scene = Some(Arc::new(brush_search_scene(false)));
    let result = (0..64).map(|_| clear.compute_brush_cache_entry(BRUSH_SEARCH_TARGET)).find(|result| !result.unknown_pending).expect("the precompute lane settles");
    assert_eq!(result.free.len(), 2, "with nothing parked there both candidates are free");
}

/// ⛽️ LAW: one fuel unit is one candidate verdict, so a single `toolRunStep` shows exactly one decided candidate.
#[test]
fn brush_suggestions_run_step_with_one_unit_of_fuel_decides_at_most_one_candidate() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut job = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    let mut decided_ticks = 0;
    for _ in 0..10_000 {
        if port.is_waiting() {
            break;
        }
        let Some(tick) = brush_run_step(&mut job, 1) else { continue };
        let decided = tick.trace.iter().flat_map(|page| &page.ops).filter(|op| matches!(op, ToolRunTraceOp::Upsert { verdict, .. } if *verdict != ToolRunVerdict::Testing)).count();
        assert!(decided <= 1, "a one-unit step decided {decided} candidates");
        decided_ticks += decided;
        mirror.apply(tick);
    }
    assert!(port.is_waiting(), "the search settles one unit at a time");
    assert_eq!(decided_ticks, 2, "both candidates are decided, one per unit");
}

/// 🎯️ LAW: the run follows the link's target. A settled search waits on its port and costs no step; a new target
/// clears the trace and searches it; no target clears the trace, the link's result and waits again. A gesture wakes it.
#[test]
fn brush_suggestions_run_follows_the_link_target_and_waits_while_settled() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut scene = brush_search_scene(false);
    scene.fixture.objects[0].vortices.push(VortexProps { id: "v1".to_string(), vortex_kind: Some("port-a".to_string()), position: [0.0, 40.0, 0.0], direction: Some([0.0, 0.0, -1.0]) });
    let mut job = brush_run_job(&owner, &port, scene, brush_search_lane(), brush_search_meshes, 0);
    let mut mirror = BrushRunMirror::new(0);
    mirror.drive(&mut job, &port, u64::MAX);
    brush_run_link(&owner, |link| assert_eq!(link.port.is_some(), true, "the job hands the link its wake port"));
    assert!(brush_run_step(&mut job, u64::MAX).is_none(), "a settled search on an unchanged target emits nothing");
    assert!(port.is_waiting());
    brush_run_link(&owner, |link| link.hover(Some("host:v1".to_string())));
    assert!(!port.is_waiting(), "a gesture wakes the run");
    let before = mirror.clears;
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.clears, before + 1, "a new target clears the trace first");
    assert_eq!(mirror.trace.len(), 2, "the trace now holds only the new target's candidates");
    assert!(brush_run_link(&owner, |link| link.found("host:v1").is_some_and(|found| found.done && found.free().count() == 2)));
    assert!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).is_none()), "the link holds one target's result");
    brush_run_link(&owner, |link| link.hover(None));
    mirror.drive(&mut job, &port, u64::MAX);
    assert_eq!(mirror.trace.len(), 0, "no target, no trace");
    assert!(brush_run_link(&owner, |link| link.found("host:v1").is_none()), "no target, no result");
}

/// 🧹️ LAW: a closing job retires the link result it published and never one its successor (the next generation)
/// already published.
#[test]
fn a_closing_brush_suggestions_job_retires_only_its_own_result() {
    let (owner, port) = (brush_run_owner(Some(BRUSH_SEARCH_TARGET)), ToolRunJobPort::default());
    let mut first = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 0);
    BrushRunMirror::new(0).drive(&mut first, &port, u64::MAX);
    port.wake();
    let mut second = brush_run_job(&owner, &port, brush_search_scene(false), brush_search_lane(), brush_search_meshes, 1);
    BrushRunMirror::new(1).drive(&mut second, &port, u64::MAX);
    first.begin_close();
    assert_eq!(first.close_step(64, 1 << 16), InteractiveJobCloseStep::Complete);
    assert!(first.terminal_is_empty());
    assert_eq!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).map(|found| found.writer)), Some((1, 1)), "the successor's result survives");
    second.begin_close();
    assert_eq!(second.close_step(64, 1 << 16), InteractiveJobCloseStep::Complete);
    assert!(brush_run_link(&owner, |link| link.found(BRUSH_SEARCH_TARGET).is_none()), "the last writer's close retires its result");
}

fn brush_run_example_scene(document: &str) -> (SceneConfig, Vec<String>, Vec<String>) {
    let text = match document {
        "nakagin" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_NAKAGIN_EXAMPLE_TEXT,
        "concrete-forest" => crate::standards::v1::subsets::any::schema::snapshot::text::PUZZLE3D_CONCRETE_FOREST_EXAMPLE_TEXT,
        other => panic!("unknown example document {other}"),
    };
    let snapshot = crate::standards::v1::subsets::any::schema::snapshot::text::parse_dsl(text).expect("example parses");
    let envelope = crate::editor::puzzle3d::scene_from_snapshot(&snapshot, Default::default(), "brush");
    let scene = crate::editor::puzzle3d::scene_config(&envelope).expect("scene config");
    let targets = scene.fixture.objects.iter().flat_map(|object| object.vortices.iter().map(|vortex| puzzle3d_vortex_full_id(&object.id, &vortex.id))).collect();
    (scene, crate::editor::puzzle3d::modes::edit::windows::main::mesh_lane(&envelope.fixture), targets)
}

fn brush_run_no_meshes(_url: &str) -> Option<(Vec<f32>, Vec<u32>)> {
    None
}

/// ⚖️ ORACLE (`parry3d`): on Concrete Forest with the app's box fallback, every candidate the run decided for the
/// fixture's targets is recomputed as an exact convex hull against every placed body except the docking host; the
/// overlap volume comes from parry point containment. A decisive overlap (at least twice the budget, enough
/// expected samples) must be a collision, a decisive clearance (at most half the budget, or separated hulls) must be
/// free. Every decisive verdict must agree.
#[test]
fn brush_suggestions_run_collision_verdicts_agree_with_the_parry3d_oracle() {
    use parry3d::query::PointQuery;
    use parry3d::shape::Shape;
    let fixture: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run fixture");
    let law = &fixture["laws"]["parryOracle"];
    let (samples, decisive_hits, cells) = (law["samples"].as_f64().expect("samples"), law["decisiveHits"].as_f64().expect("decisive hits"), law["gridCells"].as_u64().expect("grid cells") as usize);
    let identity = parry3d::math::Isometry::identity();
    let (mut decisive, mut ambiguous, mut collisions, mut frees) = (0usize, 0usize, 0usize, 0usize);
    let mut disagreements = Vec::new();
    for document in law["documents"].as_array().expect("documents") {
        let (scene, lane, targets) = brush_run_example_scene(document["document"].as_str().expect("document"));
        let budget = scene.overlap_budget;
        let (positions, _) = crate::editor::puzzle3d::puzzle3d_fallback_mesh_buffers();
        let hull = |pose: &Pose3d| {
            let points: Vec<parry3d::math::Point<f32>> = positions.chunks(3).map(|vertex| pose.transform_point(&crate::editor::puzzle3d::precompute::geometry::Point3d::new(vertex[0], vertex[1], vertex[2]))).map(|world| parry3d::math::Point::new(world.x(), world.y(), world.z())).collect();
            parry3d::shape::ConvexPolyhedron::from_convex_hull(&points).expect("box hull")
        };
        let catalogs = scene.kind_catalogs.clone().unwrap_or_default();
        let placed: Vec<(String, parry3d::shape::ConvexPolyhedron)> = scene.fixture.objects.iter().filter(|object| resolve_placed_object_mesh_url(object, &catalogs, &scene.fixture).is_some()).map(|object| (object.id.clone(), hull(&pose_isometry(object.origin, object.orientation.unwrap_or([0.0, 0.0, 0.0, 1.0]), &object.scale)))).collect();
        let (owner, port) = (brush_run_owner(None), ToolRunJobPort::default());
        let mut job = brush_run_job(&owner, &port, scene.clone(), lane, brush_run_no_meshes, 0);
        for target in targets.iter().take(document["targets"].as_u64().expect("targets") as usize) {
            brush_run_link(&owner, |link| link.hover(Some(target.clone())));
            BrushRunMirror::new(0).drive(&mut job, &port, u64::MAX);
            let Some(found) = brush_run_link(&owner, |link| link.found(target).cloned()) else { continue };
            let host = target.split(':').next().expect("host id");
            for (key, (preview, verdict)) in found.previews.iter().zip(&found.verdicts).enumerate() {
                let (Some(preview), BrushSuggestionVerdict::Free | BrushSuggestionVerdict::Collision) = (preview, verdict) else { continue };
                let candidate = hull(&pose_isometry(preview.origin, preview.orientation, &preview.scale));
                let (mut collides, mut uncertain) = (false, false);
                for (_, other) in placed.iter().filter(|(id, _)| id != host) {
                    let (left, right) = (candidate.compute_local_aabb(), other.compute_local_aabb());
                    if !parry3d::bounding_volume::BoundingVolume::intersects(&left, &right) || parry3d::query::distance(&identity, &candidate, &identity, other).expect("convex distance") > 0.0 {
                        continue;
                    }
                    let (min, max) = (left.mins.sup(&right.mins), left.maxs.inf(&right.maxs));
                    let size = max - min;
                    let box_volume = f64::from(size.x) * f64::from(size.y) * f64::from(size.z);
                    let mut inside = 0usize;
                    for x in 0..cells {
                        for y in 0..cells {
                            for z in 0..cells {
                                let at = |index: usize, axis: usize| min[axis] + size[axis] * ((index as f32 + 0.5) / cells as f32);
                                let point = parry3d::math::Point::new(at(x, 0), at(y, 1), at(z, 2));
                                inside += usize::from(candidate.contains_local_point(&point) && other.contains_local_point(&point));
                            }
                        }
                    }
                    let volume = box_volume * inside as f64 / (cells * cells * cells) as f64;
                    let (expected_hits, threshold_hits) = (samples * volume / box_volume.max(f64::MIN_POSITIVE), samples * budget / box_volume.max(f64::MIN_POSITIVE));
                    if volume >= 2.0 * budget && expected_hits >= decisive_hits {
                        collides = true;
                    } else if !(volume <= 0.5 * budget && threshold_hits >= decisive_hits) {
                        uncertain = true;
                    }
                }
                let ours = *verdict == BrushSuggestionVerdict::Collision;
                collisions += usize::from(ours);
                frees += usize::from(!ours);
                if !collides && uncertain {
                    ambiguous += 1;
                    continue;
                }
                decisive += 1;
                if ours != collides {
                    disagreements.push(format!("{target} candidate {key} ({}) ours={verdict:?} parry collides={collides}", preview.object_kind_id));
                }
            }
        }
    }
    assert!(disagreements.is_empty(), "{} of {decisive} decisive verdicts disagree with parry3d:\n{}", disagreements.len(), disagreements.join("\n"));
    assert!(collisions > 0 && frees > 0, "the oracle must decide both collisions ({collisions}) and free candidates ({frees})");
    assert!(ambiguous * 10 <= decisive, "at most one in ten verdicts may fall inside the sampling band: {ambiguous} of {decisive}");
}

/// ⏱️ LAW: hovering never blocks. On Nakagin, the largest example, every real-clock step of the run under the
/// interactive lane budget stays below 2 ms — preparation, target listing and collision units alike — across the
/// fixture's targets. The best of the fixture's cold runs is taken, so concurrent builds cannot fake a regression.
#[test]
fn brush_suggestions_run_step_stays_below_the_interactive_ceiling_for_nakagin() {
    let fixture: serde_json::Value = serde_json::from_str(BRUSH_SUGGESTIONS_RUN_FIXTURE).expect("brush suggestions run fixture");
    let law = &fixture["laws"]["interactive"];
    let (scene, lane, targets) = brush_run_example_scene(law["document"].as_str().expect("document"));
    let budget_us = law["budgetUs"].as_u64().expect("budget");
    let worst_runs: Vec<(u128, usize)> = (0..law["runs"].as_u64().expect("runs"))
        .map(|_| {
            let (owner, port) = (brush_run_owner(None), ToolRunJobPort::default());
            let mut job = brush_run_job(&owner, &port, scene.clone(), lane.clone(), brush_run_no_meshes, 0);
            let (mut worst, mut steps) = (0u128, 0usize);
            for target in targets.iter().take(law["targets"].as_u64().expect("targets") as usize) {
                brush_run_link(&owner, |link| link.hover(Some(target.clone())));
                while !port.is_waiting() {
                    let mut sequence = 0;
                    let now = semio_framework_job::default_now_us().expect("clock");
                    let budget = StepBudget::from_duration(semio_framework_job::INTERACTIVE_LANE_FUEL, now, semio_framework_job::INTERACTIVE_LANE_WALL_US).expect("budget");
                    let mut context = StepContext::new(OperationId(92), Generation(1), budget, root_cancel_token(), semio_framework_job::default_now_us, &mut sequence);
                    let started = std::time::Instant::now();
                    let outcome = job.step(&mut context);
                    worst = worst.max(started.elapsed().as_micros());
                    drop(brush_run_settle(outcome));
                    steps += 1;
                }
            }
            (worst, steps)
        })
        .collect();
    let (best, steps) = worst_runs.iter().copied().min().expect("runs");
    assert!(best < u128::from(budget_us), "the worst step of the best run took {best} µs over {steps} steps against {budget_us} µs (runs: {worst_runs:?})");
}
//#endregion ⏯️BrushSuggestionsRun
