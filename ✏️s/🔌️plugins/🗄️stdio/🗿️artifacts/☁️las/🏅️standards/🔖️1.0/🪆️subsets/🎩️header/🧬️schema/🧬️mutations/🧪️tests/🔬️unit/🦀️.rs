
use super::*;
use crate::schema::diff::{LasPointsDiff, LasVlrsDiff};
use protocol::MutationDiff;
use protocol::command::DiffAlgebra;
use protocol::{OpBinary, OpText};

//#region 🔖️mutation_diff_law
fn assert_mutation_diff_law(base: &LasSnapshot, mutation: LasMutation) {
    let expected_diff = mutation.diff(base);
    let mut applied_snapshot = base.clone();
    let returned_diff = apply_las_mutation(&mut applied_snapshot, &mutation);
    assert_eq!(returned_diff, expected_diff, "apply_las_mutation must return mutation.diff(base) for {mutation:?}");
    assert_eq!(expected_diff.diff().apply(base).expect("valid mutation diff"), applied_snapshot, "diff.diff().apply(base) must equal the imperative mutation result for {mutation:?}");
}

#[test]
fn mutation_diff_law() {
    let base = base_snapshot();
    let mut alt = base.clone();
    alt.header.creation_year = 2030;
    assert_mutation_diff_law(&base, LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: alt }));
    assert_mutation_diff_law(&base, LasMutation::SetVersion(set_version::SetVersion { major: 1, minor: 4 }));
    assert_mutation_diff_law(&base, LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "semio".into() }));
    assert_mutation_diff_law(&base, LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: "semio-las-writer".into() }));
    assert_mutation_diff_law(&base, LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: 42, year: 2026 }));
    assert_mutation_diff_law(&base, LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) }));
    assert_mutation_diff_law(&base, LasMutation::SetBounds(set_bounds::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) }));
    assert_mutation_diff_law(&base, LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: [1, 2, 3, 4, 5] }));
    assert_mutation_diff_law(&base, LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") }));
    assert_mutation_diff_law(&base, LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }));
    assert_mutation_diff_law(&base, LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: b"patched".to_vec() }));
    assert_mutation_diff_law(&base, LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: point(9) }));
    assert_mutation_diff_law(&base, LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }));
    assert_mutation_diff_law(&base, LasMutation::SetPoint(set_point::SetPoint { index: 0, point: point(42) }));
}
//#endregion 🔖️mutation_diff_law

//#region 🔖️inverse_law
#[test]
fn inverse_law() {
    let base = base_snapshot();
    let variants = vec![
        LasMutation::SetVersion(set_version::SetVersion { major: 1, minor: 4 }),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "semio".into() }),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: "semio-las-writer".into() }),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: 42, year: 2026 }),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (0.001, 0.001, 0.001), offset: (1000.0, 2000.0, 0.0) }),
        LasMutation::SetBounds(set_bounds::SetBounds { max: (999.0, 888.0, 777.0), min: (-1.0, -2.0, -3.0) }),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: [1, 2, 3, 4, 5] }),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("EXTRA", 200, b"new-vlr") }),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: b"patched".to_vec() }),
        LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: point(9) }),
        LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }),
        LasMutation::SetPoint(set_point::SetPoint { index: 0, point: point(42) }),
    ];
    for m in variants {
        // Mutation-level round trip.
        let mut snap = base.clone();
        apply_las_mutation(&mut snap, &m);
        for inv in m.inverse(&base) {
            apply_las_mutation(&mut snap, &inv);
        }
        assert_eq!(snap, base, "mutation-level inverse must restore base for {m:?}");

        // Diff-level round trip.
        let d = m.diff(&base);
        let mutated = d.diff().apply(&base).expect("valid forward diff");
        let inv_d = d.diff().inverse(&base);
        assert_eq!(inv_d.apply(&mutated).expect("valid inverse diff"), base, "diff-level inverse must restore base for {m:?}");
    }
}
//#endregion 🔖️inverse_law

//#region 🔖️absorb_law
fn assert_absorb_law(base: &LasSnapshot, m1: LasMutation, m2: LasMutation) {
    let d1 = m1.diff(base);
    let mid = d1.diff().apply(base).expect("valid first diff");
    let d2 = m2.diff(&mid);
    let sequential = d2.diff().apply(&mid).expect("valid second diff");

    let mut merged = d1.diff().clone();
    merged.absorb(d2.diff().clone());
    assert_eq!(merged.apply(base).expect("valid absorbed diff"), sequential, "absorb(d1,d2).apply(base) must equal sequential application for {m1:?} + {m2:?}");
}

#[test]
fn absorb_law() {
    let base = base_snapshot();

    // Insert+Remove-before (vlrs): canonical shift case.
    assert_absorb_law(&base, LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("X", 1, b"x") }), LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }));

    // Insert+Insert-same-index (vlrs): both survive.
    assert_absorb_law(&base, LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("X", 1, b"x") }), LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 1, vlr: vlr("Y", 2, b"y") }));

    // Add+SetField (vlrs): patches directly into the still-pending added VLR.
    assert_absorb_law(&base, LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 0, vlr: vlr("X", 1, b"x") }), LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: b"patched".to_vec() }));

    // Modify+Remove (vlrs): a pending field patch on a since-removed base VLR vanishes.
    assert_absorb_law(&base, LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: b"will be dropped".to_vec() }), LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }));

    // Insert+Remove-before (points): same canonical case, other collection.
    assert_absorb_law(&base, LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: point(9) }), LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }));

    // Insert+Insert-same-index (points): both survive.
    assert_absorb_law(&base, LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: point(9) }), LasMutation::InsertPoint(insert_point::InsertPoint { index: 1, point: point(8) }));

    // Add+SetField (points): patches into the pending added point.
    assert_absorb_law(&base, LasMutation::InsertPoint(insert_point::InsertPoint { index: 0, point: point(9) }), LasMutation::SetPoint(set_point::SetPoint { index: 0, point: point(7) }));

    // Insert then annihilate the very same insert (points).
    assert_absorb_law(&base, LasMutation::InsertPoint(insert_point::InsertPoint { index: 0, point: point(9) }), LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }));

    // Two unrelated scalar sets absorb via LWW.
    assert_absorb_law(
        &base,
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "first".into() }),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "second".into() }),
    );
}

#[test]
fn absorb_law_associativity() {
    let base = base_snapshot();
    let d1 = LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: "one".into() }).diff(&base);
    let mid1 = d1.diff().apply(&base).expect("valid first diff");
    let d2 = LasMutation::InsertPoint(insert_point::InsertPoint { index: 0, point: point(9) }).diff(&mid1);
    let mid2 = d2.diff().apply(&mid1).expect("valid second diff");
    let d3 = LasMutation::SetPoint(set_point::SetPoint { index: 0, point: point(7) }).diff(&mid2);

    // (d1∘d2)∘d3
    let mut left = d1.diff().clone();
    left.absorb(d2.diff().clone());
    left.absorb(d3.diff().clone());

    // d1∘(d2∘d3)
    let mut d23 = d2.diff().clone();
    d23.absorb(d3.diff().clone());
    let mut right = d1.diff().clone();
    right.absorb(d23);

    assert_eq!(left.apply(&base).expect("valid left diff"), right.apply(&base).expect("valid right diff"), "absorb must associate");
    assert_eq!(left.apply(&base).expect("valid associated diff"), d3.diff().apply(&mid2).expect("valid third diff"), "associated absorb must match full sequential application");
}
//#endregion 🔖️absorb_law

//#region 🔖️between_roundtrip_law
#[test]
fn between_roundtrip_law() {
    let a = base_snapshot();
    let mut b = base_snapshot();
    b.header.creation_year = 2030;
    b.vlrs.remove(0);
    b.vlrs[0].description = "modified".into();
    b.vlrs.push(vlr("NEW", 300, b"new-vlr"));
    b.points.remove(0);
    b.points[0].classification = 250;
    b.points.push(point(50));

    let d = LasDiff::between(&a, &b);
    assert_eq!(d.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
    let d_rev = LasDiff::between(&b, &a);
    assert_eq!(d_rev.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
    assert!(LasDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");
}
//#endregion 🔖️between_roundtrip_law

//#region 🔖️codec_retention_law
#[test]
fn codec_retention_law() {
    let bytes = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🏅️standards/🔖️1.0/🪆️subsets/🎩️header/📚️examples/🎬️demo/🖼️assets/🧊️.las")).expect("read committed LAS fixture");
    let snap = crate::engine::decode_las(&bytes).expect("decode fixture");
    let reencoded = crate::engine::encode_las(&snap).expect("re-encode fixture");
    let redecoded = crate::engine::decode_las(&reencoded).expect("re-decode fixture");
    // Structural fields are always recomputed on encode (see `LasHeader`'s doc comment); the
    // retained invariant is real content: points, VLR payloads, and the non-structural header
    // fields (scale/offset/bounds/dates/identifiers/points-by-return).
    assert_eq!(redecoded.points.len(), snap.points.len());
    for (a, b) in snap.points.iter().zip(redecoded.points.iter()) {
        assert!((a.x - b.x).abs() < 1e-6);
        assert!((a.y - b.y).abs() < 1e-6);
        assert!((a.z - b.z).abs() < 1e-6);
        assert_eq!(a.classification, b.classification);
    }
    assert_eq!(redecoded.vlrs.len(), snap.vlrs.len());
    for (a, b) in snap.vlrs.iter().zip(redecoded.vlrs.iter()) {
        assert_eq!(a.user_id, b.user_id);
        assert_eq!(a.record_id, b.record_id);
        assert_eq!(a.data, b.data);
    }
    assert_eq!(snap.header.system_identifier, redecoded.header.system_identifier);
    assert_eq!(snap.header.generating_software, redecoded.header.generating_software);
    assert_eq!(snap.header.points_by_return, redecoded.header.points_by_return);
}
//#endregion 🔖️codec_retention_law

//#region 🔖️field_sweep
/// 🌪️ `sweep_a`/`sweep_b` differ in EVERY mutable field: every header scalar, one VLR
/// removed + one modified-in-every-field, one point added + one modified-in-every-field
/// (incl. both tri-states). `vlrs`/`points` are index-keyed so a SINGLE `between()` call can
/// only ever show `removed` XOR `added` on a same-length pair (the recipe's own documented
/// structural limit — see f1-closer-report.md §4.4) — sidestepped here by giving `a`/`b`
/// DIFFERENT lengths per collection and splitting assertions across both `between()`
/// directions, exactly like `txt`'s fix: `vlrs` SHRINKS a->b (removed forward, added
/// backward), `points` GROWS a->b (added forward, removed backward); both collections'
/// `modified` slot (index 0, which exists on both sides either way) is exercised in EVERY
/// direction.
fn sweep_a() -> LasSnapshot {
    LasSnapshot {
        schema: "stdio.las".into(),
        header: LasHeader {
            version_major: 1,
            version_minor: 2,
            system_identifier: "before-system".into(),
            generating_software: "before-software".into(),
            creation_day_of_year: 10,
            creation_year: 2020,
            header_size: 227,
            offset_to_point_data: 227,
            number_of_vlrs: 2,
            point_data_format_id: 1,
            point_data_record_length: 28,
            number_of_point_records: 2,
            points_by_return: [1, 1, 0, 0, 0],
            x_scale: 0.01,
            y_scale: 0.01,
            z_scale: 0.01,
            x_offset: 0.0,
            y_offset: 0.0,
            z_offset: 0.0,
            max_x: 100.0,
            min_x: 0.0,
            max_y: 100.0,
            min_y: 0.0,
            max_z: 100.0,
            min_z: 0.0,
        },
        vlrs: vec![vlr("stay-user", 1, b"stay-data-before"), vlr("gone-user", 2, b"will be removed")],
        points: vec![LasPoint { gps_time: Some(1000.0), ..point(0) }, point(1)],
    }
}

fn sweep_b() -> LasSnapshot {
    LasSnapshot {
        schema: "stdio.las".into(),
        header: LasHeader {
            version_major: 2,
            version_minor: 4,
            system_identifier: "after-system".into(),
            generating_software: "after-software".into(),
            creation_day_of_year: 250,
            creation_year: 2026,
            header_size: 375,
            offset_to_point_data: 500,
            number_of_vlrs: 1,
            point_data_format_id: 3,
            point_data_record_length: 34,
            number_of_point_records: 3,
            points_by_return: [0, 0, 2, 1, 0],
            x_scale: 0.001,
            y_scale: 0.001,
            z_scale: 0.001,
            x_offset: 500.0,
            y_offset: 500.0,
            z_offset: 10.0,
            max_x: 999.0,
            min_x: -1.0,
            max_y: 999.0,
            min_y: -1.0,
            max_z: 50.0,
            min_z: -50.0,
        },
        vlrs: vec![
            // Index 0 stays "alive" but every field changes (exercises `modified`). `a`'s
            // index 1 ("gone-user") has no counterpart here -- exercises `removed` forward
            // (a->b) / `added` backward (b->a), the collection SHRINKS.
            vlr("stay-user-2", 9, b"stay-data-after"),
        ],
        points: vec![
            // Index 0 stays "alive" but every field changes, incl. both tri-states going
            // from Some -> None and None -> Some.
            LasPoint {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                intensity: 500,
                return_number: 4,
                number_of_returns: 5,
                scan_direction_flag: false,
                edge_of_flight_line: false,
                classification: 9,
                scan_angle_rank: 5,
                user_data: 200,
                point_source_id: 42,
                gps_time: None,          // tri-state: Some(1000.0) -> None
                rgb: Some((10, 20, 30)), // tri-state: None -> Some
            },
            point(1),
            // Index 2 is a brand-new point (exercises `added` on the a->b direction).
            point(9),
        ],
    }
}

#[test]
fn field_sweep_covers_every_mutable_field() {
    let a = sweep_a();
    let b = sweep_b();

    let forward = LasDiff::between(&a, &b);
    assert_eq!(forward.apply(&a).expect("valid forward diff"), b, "between(a,b).apply(a) must equal b");
    let backward = LasDiff::between(&b, &a);
    assert_eq!(backward.apply(&b).expect("valid backward diff"), a, "between(b,a).apply(b) must equal a");
    assert!(LasDiff::between(&a, &a).is_empty(), "between(a,a) must be empty");

    // Every header scalar must be diffed forward.
    assert!(forward.version_major.is_some());
    assert!(forward.version_minor.is_some());
    assert!(forward.system_identifier.is_some());
    assert!(forward.generating_software.is_some());
    assert!(forward.creation_day_of_year.is_some());
    assert!(forward.creation_year.is_some());
    assert!(forward.header_size.is_some());
    assert!(forward.offset_to_point_data.is_some());
    assert!(forward.point_data_format_id.is_some());
    assert!(forward.point_data_record_length.is_some());
    assert!(forward.points_by_return.is_some());
    assert!(forward.x_scale.is_some());
    assert!(forward.y_scale.is_some());
    assert!(forward.z_scale.is_some());
    assert!(forward.x_offset.is_some());
    assert!(forward.y_offset.is_some());
    assert!(forward.z_offset.is_some());
    assert!(forward.max_x.is_some());
    assert!(forward.min_x.is_some());
    assert!(forward.max_y.is_some());
    assert!(forward.min_y.is_some());
    assert!(forward.max_z.is_some());
    assert!(forward.min_z.is_some());
    assert!(forward.number_of_vlrs.is_some(), "number_of_vlrs must be diffed (2 -> 1)");
    assert!(forward.number_of_point_records.is_some(), "number_of_point_records must be diffed (2 -> 3)");

    // vlrs: a has 2, b has 1 (SHRINKS) -- index 0 modified in every field, index 1 removed.
    let vd: &LasVlrsDiff = forward.vlrs.as_ref().expect("vlrs diff must be present");
    assert_eq!(vd.modified.len(), 1, "exactly one VLR must be modified");
    assert_eq!(vd.modified[0].index, 0);
    assert_eq!(vd.removed, vec![1], "a->b (shrinking) must show the removed VLR");
    assert!(vd.added.is_empty(), "a->b (shrinking) must not show an added VLR");
    let vmd = &vd.modified[0].diff;
    assert!(vmd.user_id.is_some());
    assert!(vmd.record_id.is_some());
    assert!(vmd.description.is_some());
    assert!(vmd.data.is_some());

    // Backward direction: vlrs GROW 1 -> 2, proving `added` (the tail the forward direction
    // structurally could not show).
    let vd_back: &LasVlrsDiff = backward.vlrs.as_ref().expect("vlrs diff must be present");
    assert_eq!(vd_back.added.len(), 1, "b->a (growing) must show the added (formerly-removed) VLR");
    assert_eq!(vd_back.added[0].index, 1);
    assert!(vd_back.removed.is_empty(), "b->a (growing) must not show a removed VLR");

    // points: a has 2, b has 3 -- index 0 modified (incl. both tri-states), index 2 added.
    let pd: &LasPointsDiff = forward.points.as_ref().expect("points diff must be present");
    assert_eq!(pd.modified.len(), 1, "exactly one point must be modified");
    assert_eq!(pd.modified[0].index, 0);
    assert_eq!(pd.added.len(), 1, "exactly one point must be added");
    assert!(pd.removed.is_empty(), "a->b (growing) must not show a removed point");
    let pmd = &pd.modified[0].diff;
    assert!(pmd.x.is_some());
    assert!(pmd.y.is_some());
    assert!(pmd.z.is_some());
    assert!(pmd.intensity.is_some());
    assert!(pmd.return_number.is_some());
    assert!(pmd.number_of_returns.is_some());
    assert!(pmd.scan_direction_flag.is_some());
    assert!(pmd.edge_of_flight_line.is_some());
    assert!(pmd.classification.is_some());
    assert!(pmd.scan_angle_rank.is_some());
    assert!(pmd.user_data.is_some());
    assert!(pmd.point_source_id.is_some());
    assert_eq!(pmd.gps_time, Some(None), "gps_time tri-state must show a clear (Some(None))");
    assert_eq!(pmd.rgb, Some(Some((10, 20, 30))), "rgb tri-state must show a set (Some(Some(_)))");

    // Backward direction: points shrink 3 -> 2, proving `removed` (the tail the forward
    // direction structurally could not show).
    let pd_back: &LasPointsDiff = backward.points.as_ref().expect("points diff must be present");
    assert_eq!(pd_back.removed, vec![2], "b->a (shrinking) must show the removed (formerly-added) point");
}
//#endregion 🔖️field_sweep

#[test]
fn out_of_range_index_mutation_is_rejected_without_mutating() {
    let base = base_snapshot();
    let mut snap = base.clone();
    let outcome = apply_las_mutation(&mut snap, &LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 99 }));
    assert_eq!(snap, base);
    assert_eq!(outcome.messages()[0].target, vec!["vlrs", "99"]);
    let outcome = apply_las_mutation(&mut snap, &LasMutation::SetPoint(set_point::SetPoint { index: 99, point: point(1) }));
    assert_eq!(snap, base);
    assert_eq!(outcome.messages()[0].target, vec!["points", "99"]);
    let outcome = apply_las_mutation(&mut snap, &LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 99, data: vec![1] }));
    assert_eq!(snap, base);
    assert_eq!(outcome.messages()[0].target, vec!["vlrs", "99"]);
}

//#region 🔖️op_text_binary_roundtrip_law
/// 🧪️ F6 (las): `OpText`/`OpBinary` round-trip law over the full 14-variant vocabulary
/// (hand-rolled, `dsl::DslOps` blocked — see the `OpCodecs` region's doc comment), via
/// `demo_mutation_cases()` — the single source of truth also reused by
/// `⚙️engine/🦀️.rs`'s `ops_grammar_conformance_law`/`protocol_walk_law`.
#[test]
fn op_text_binary_roundtrip_law() {
    for mutation in demo_mutation_cases() {
        let printed = mutation.print_op();
        assert!(!printed.contains('\n'), "print_op must be one line, got {printed:?}");
        let parsed = LasMutation::parse_op(&printed).unwrap_or_else(|e| panic!("parse_op({printed:?}) failed: {e}"));
        assert_eq!(parsed, mutation, "print_op/parse_op round-trip mismatch for {mutation:?} (printed {printed:?})");

        let encoded = mutation.encode_op().unwrap_or_else(|e| panic!("encode_op({mutation:?}) failed: {e}"));
        let decoded = LasMutation::decode_op(&encoded).unwrap_or_else(|e| panic!("decode_op failed: {e}"));
        assert_eq!(decoded, mutation, "encode_op/decode_op round-trip mismatch for {mutation:?}");
    }
}
//#endregion 🔖️op_text_binary_roundtrip_law

//#region 🔖️KindsConformanceLaw
/// 🧾️ `KINDS` must list every `LasMutation` variant, in declaration order, AND the sibling
/// oracle manifest's `mutationCatalogs[].kinds` must declare the exact same list — the first
/// half is a real `match` with no wildcard arm, so this fails to compile the moment a new
/// variant is added to `LasMutation` without a matching kebab-case spelling here, which is what
/// keeps `KINDS` honest against the enum. The second half reads the sibling oracle manifest's
/// `kinds` array as text (the framework never parses Rust, so this is the only side that can
/// prove the manifest matches) and asserts the same list, in the same order.
#[test]
fn kinds_match_enum_and_catalog() {
    fn kind_of(mutation: &LasMutation) -> &'static str {
        match mutation {
            LasMutation::SetSnapshot(_) => "set-snapshot",
            LasMutation::SetVersion(_) => "set-version",
            LasMutation::SetSystemIdentifier(_) => "set-system-identifier",
            LasMutation::SetSoftwareInfo(_) => "set-software-info",
            LasMutation::SetCreationDate(_) => "set-creation-date",
            LasMutation::SetScaleAndOffset(_) => "set-scale-and-offset",
            LasMutation::SetBounds(_) => "set-bounds",
            LasMutation::SetPointsByReturn(_) => "set-points-by-return",
            LasMutation::InsertVlr(_) => "insert-vlr",
            LasMutation::RemoveVlr(_) => "remove-vlr",
            LasMutation::SetVlrData(_) => "set-vlr-data",
            LasMutation::InsertPoint(_) => "insert-point",
            LasMutation::RemovePoint(_) => "remove-point",
            LasMutation::SetPoint(_) => "set-point",
        }
    }
    let samples = [
        LasMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: base_snapshot() }),
        LasMutation::SetVersion(set_version::SetVersion { major: 1, minor: 0 }),
        LasMutation::SetSystemIdentifier(set_system_identifier::SetSystemIdentifier { system_identifier: String::new() }),
        LasMutation::SetSoftwareInfo(set_software_info::SetSoftwareInfo { generating_software: String::new() }),
        LasMutation::SetCreationDate(set_creation_date::SetCreationDate { day_of_year: 0, year: 0 }),
        LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale: (0.0, 0.0, 0.0), offset: (0.0, 0.0, 0.0) }),
        LasMutation::SetBounds(set_bounds::SetBounds { max: (0.0, 0.0, 0.0), min: (0.0, 0.0, 0.0) }),
        LasMutation::SetPointsByReturn(set_points_by_return::SetPointsByReturn { counts: [0; 5] }),
        LasMutation::InsertVlr(insert_vlr::InsertVlr { index: 0, vlr: LasVlr::default() }),
        LasMutation::RemoveVlr(remove_vlr::RemoveVlr { index: 0 }),
        LasMutation::SetVlrData(set_vlr_data::SetVlrData { index: 0, data: Vec::new() }),
        LasMutation::InsertPoint(insert_point::InsertPoint { index: 0, point: LasPoint::default() }),
        LasMutation::RemovePoint(remove_point::RemovePoint { index: 0 }),
        LasMutation::SetPoint(set_point::SetPoint { index: 0, point: LasPoint::default() }),
    ];
    let from_enum: Vec<&'static str> = samples.iter().map(kind_of).collect();
    assert_eq!(from_enum, KINDS, "KINDS must list every LasMutation variant, in declaration order");

    let manifest = include_str!("../../../../🔮️oracle/🔣️.json");
    let needle = "\"kinds\": [";
    let start = manifest.find(needle).expect("manifest declares a kinds array") + needle.len();
    let end = start + manifest[start..].find(']').expect("kinds array is closed");
    let declared: Vec<String> = manifest[start..end].split(',').map(|entry| entry.trim().trim_matches('"').to_string()).filter(|entry| !entry.is_empty()).collect();
    assert_eq!(declared, KINDS, "the oracle manifest's kinds must match LasMutation exactly");
}
//#endregion 🔖️KindsConformanceLaw

//#region 🔖️ScaleAndOffsetRecordLaw
/// 📏️ `SetScaleAndOffset` writes the six header fields and leaves the on-disk point RECORDS
/// exactly where they are, so every coordinate is re-read under the new parameters. The
/// integer each coordinate decoded from is what this asserts on — `record = (coordinate -
/// offset) / scale` before and after — because that integer is what the file carries and what
/// the reference (`las::raw`) preserves. Holding the coordinates fixed instead would rewrite
/// every record, which is a re-quantization rather than the header edit this kind is named
/// for (ticket `26/08/23/END-TO-END-TESTING-REFACTOR`,
/// `mutate-las-1-0::mutate-set-scale-and-offset`).
#[test]
fn set_scale_and_offset_keeps_every_point_record_where_it_is() {
    // 🧫️ Power-of-two scales and coordinates that are exact multiples of them, so every
    // assertion below is about the mutation rather than about binary floating point.
    let mut base = base_snapshot();
    base.header.x_scale = 0.5;
    base.header.y_scale = 0.5;
    base.header.z_scale = 0.5;
    for (index, point) in base.points.iter_mut().enumerate() {
        point.x = 100.0 + index as f64 * 0.5;
        point.y = -50.0 + index as f64;
        point.z = 10.0 + index as f64 * 1.5;
    }
    let records = |snapshot: &LasSnapshot| -> Vec<(i64, i64, i64)> {
        snapshot
            .points
            .iter()
            .map(|point| {
                (
                    ((point.x - snapshot.header.x_offset) / snapshot.header.x_scale).round() as i64,
                    ((point.y - snapshot.header.y_offset) / snapshot.header.y_scale).round() as i64,
                    ((point.z - snapshot.header.z_offset) / snapshot.header.z_scale).round() as i64,
                )
            })
            .collect()
    };
    let before = records(&base);

    for (scale, offset) in [((0.25, 0.25, 0.25), (1000.0, 2000.0, 5.0)), ((2.0, 2.0, 2.0), (0.0, 0.0, 0.0))] {
        let kind = LasMutation::SetScaleAndOffset(set_scale_and_offset::SetScaleAndOffset { scale, offset });
        let mut moved = base.clone();
        apply_las_mutation(&mut moved, &kind);
        assert_eq!(moved.header.x_scale, scale.0, "the header field is written");
        assert_eq!(moved.header.y_offset, offset.1, "the header field is written");
        assert_eq!(records(&moved), before, "the point records must be untouched — only how they are read changed");
        assert_ne!(moved.points[1].x, base.points[1].x, "so the real-world coordinate must move");
        assert_eq!(moved.points[1].x, before[1].0 as f64 * scale.0 + offset.0, "and it must be exactly `record * scale + offset`");

        // ↩️ Exact in BOTH directions, the coarsening one included — that is the direction a
        // re-quantizing reading would round away for good.
        let mut restored = moved;
        for step in &<LasMutation as Mutation<LasSnapshot>>::inverse(&kind, &base) {
            apply_las_mutation(&mut restored, step);
        }
        assert_eq!(restored, base, "putting the old scale and offset back must restore the document exactly");
    }
}
//#endregion 🔖️ScaleAndOffsetRecordLaw
