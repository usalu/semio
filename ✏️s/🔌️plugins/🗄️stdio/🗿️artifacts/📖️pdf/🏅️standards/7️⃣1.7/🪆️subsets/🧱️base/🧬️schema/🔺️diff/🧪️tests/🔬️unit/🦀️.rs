
    use super::*;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfIndirectObject, STDIO_PDF17_DOCUMENT_SCHEMA};

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn oref(num: u32, gen: u16) -> ObjRef {
        ObjRef { num, gen }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn page(mb: [f64; 4], cb: Option<[f64; 4]>, rotate: i32, text: &str) -> PdfPage {
        PdfPage { media_box: mb, crop_box: cb, rotate, text: text.into() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn dict(entries: Vec<(&str, PdfObject)>) -> PdfObject {
        PdfObject::Dict(entries.into_iter().map(|(k, v)| PdfDictEntry { key: k.into(), value: v }).collect())
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn entry(k: &str, v: PdfObject) -> PdfDictEntry {
        PdfDictEntry { key: k.into(), value: v }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn base_snapshot() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.7".into(),
            pages: vec![page([0.0, 0.0, 100.0, 100.0], None, 0, "one"), page([0.0, 0.0, 50.0, 50.0], None, 0, "two")],
            info: PdfInfo { title: Some("Base".into()), ..Default::default() },
            objects: vec![PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(3))]) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(7) }],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(2)), entry("Extra", PdfObject::Bool(true))],
        }
    }

    //#region between_roundtrip_law
    #[test]
    fn between_roundtrip_law_value_scalars_and_kind_change() {
        let cases = [
            (PdfObject::Null, PdfObject::Bool(true)),
            (PdfObject::Bool(true), PdfObject::Bool(false)),
            (PdfObject::Int(1), PdfObject::Int(2)),
            (PdfObject::Real(1.5.into()), PdfObject::Real(2.5.into())),
            (PdfObject::Str(b"a".to_vec()), PdfObject::Str(b"b".to_vec())),
            (PdfObject::Name("A".into()), PdfObject::Name("B".into())),
            (PdfObject::Ref(oref(1, 0)), PdfObject::Ref(oref(2, 1))),
            (PdfObject::Int(1), PdfObject::Name("one".into())), // kind change -> Replace
        ];
        for (a, b) in cases {
            match value_diff_between(&a, &b) {
                None => assert_eq!(a, b),
                Some(d) => assert_eq!(apply_value_diff(&d, &a), b, "a={a:?} b={b:?}"),
            }
        }
    }

    #[test]
    fn between_roundtrip_law_nested_array_and_dict() {
        let a = dict(vec![("Kids", PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Int(2)])), ("N", PdfObject::Int(1))]);
        let b = dict(vec![("Kids", PdfObject::Array(vec![PdfObject::Int(1), PdfObject::Int(20), PdfObject::Int(30)])), ("N", PdfObject::Int(2)), ("Extra", PdfObject::Bool(true))]);
        let d_ab = value_diff_between(&a, &b).expect("must differ");
        assert_eq!(apply_value_diff(&d_ab, &a), b);
        let d_ba = value_diff_between(&b, &a).expect("must differ");
        assert_eq!(apply_value_diff(&d_ba, &b), a);
    }

    #[test]
    fn between_roundtrip_law_snapshot_level() {
        let (a, b) = (base_snapshot(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
    }

    #[test]
    fn between_self_is_empty() {
        let a = base_snapshot();
        assert!(PdfDiff::between(&a, &a).is_empty());
    }
    //#endregion between_roundtrip_law

    //#region inverse_law
    #[test]
    fn inverse_law_diff_level() {
        let (a, b) = (base_snapshot(), sweep_b());
        let d = PdfDiff::between(&a, &b);
        let mid = d.apply(&a).unwrap();
        assert_eq!(mid, b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&mid).unwrap(), a);
    }
    //#endregion inverse_law

    //#region absorb_law (pages / index-keyed)
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn pages_diff(d: PdfPagesDiff) -> PdfDiff {
        PdfDiff { pages: Some(d), ..Default::default() }
    }

    #[test]
    fn absorb_law_pages_insert_then_remove_before() {
        // base=[a,b,c]; d1=Insert(2,f) -> mid=[a,b,f,c]; d2=Remove(0) -> after=[b,f,c].
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b"), page([0.0; 4], None, 0, "c")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "f") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert_eq!(p.removed, vec![0]);
                assert_eq!(p.added.len(), 1);
                assert_eq!(p.added[0].index, 1);
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_insert_insert_same_index_both_survive() {
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "f") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 2, page: page([0.0; 4], None, 0, "g") }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => assert_eq!(p.added.len(), 2, "both inserts must survive"),
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { pages: vec![], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { added: vec![PdfPageAdded { index: 0, page: page([0.0; 4], None, 0, "x") }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { rotate: Some(90), ..Default::default() } }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert!(p.modified.is_empty(), "the patch must land INSIDE the carried added payload");
                assert_eq!(p.added[0].page.rotate, 90);
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "a"), page([0.0; 4], None, 0, "b")], ..base_snapshot() };
        let d1 = pages_diff(PdfPagesDiff { modified: vec![PdfPageModified { index: 0, diff: PdfPageDiff { rotate: Some(180), ..Default::default() } }], ..Default::default() });
        let d2 = pages_diff(PdfPagesDiff { removed: vec![0], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.pages {
            Some(p) => {
                assert_eq!(p.removed, vec![0]);
                assert!(p.modified.is_empty());
            }
            None => panic!("expected pages diff"),
        }
    }

    #[test]
    fn absorb_law_pages_associativity() {
        let s0 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "1"), page([0.0; 4], None, 0, "2"), page([0.0; 4], None, 0, "3")], ..base_snapshot() };
        let s1 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "1"), page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "3")], ..base_snapshot() };
        let s2 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "3"), page([0.0; 4], None, 0, "4")], ..base_snapshot() };
        let s3 = PdfSnapshot { pages: vec![page([0.0; 4], None, 0, "9"), page([0.0; 4], None, 0, "4")], ..base_snapshot() };
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);
        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law (pages / index-keyed)

    //#region absorb_law (objects / id-keyed)
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn objects_diff(d: PdfObjectsDiff) -> PdfDiff {
        PdfDiff { objects: Some(d), ..Default::default() }
    }

    #[test]
    fn absorb_law_objects_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { objects: vec![], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 0, id: oref(5, 0), value: dict(vec![("X", PdfObject::Int(1))]) }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff {
            modified: vec![PdfObjectModified { id: oref(5, 0), diff: PdfValueDiff::Dict { diff: PdfDictDiff { added: vec![PdfDictAdded { index: 1, key: "Y".into(), item: PdfObject::Int(2) }], ..Default::default() } } }],
            ..Default::default()
        });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => {
                assert!(o.modified.is_empty());
                assert_eq!(o.added[0].value, dict(vec![("X", PdfObject::Int(1)), ("Y", PdfObject::Int(2))]));
            }
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { modified: vec![PdfObjectModified { id: oref(1, 0), diff: PdfValueDiff::Int { value: 9 } }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff { removed: vec![oref(1, 0)], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => {
                assert_eq!(o.removed, vec![oref(1, 0)]);
                assert!(o.modified.is_empty());
            }
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_two_independent_inserts_both_survive() {
        let base = PdfSnapshot { objects: vec![], ..base_snapshot() };
        let d1 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 0, id: oref(5, 0), value: PdfObject::Int(1) }], ..Default::default() });
        let d2 = objects_diff(PdfObjectsDiff { added: vec![PdfObjectAdded { index: 1, id: oref(6, 0), value: PdfObject::Int(2) }], ..Default::default() });
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.objects {
            Some(o) => assert_eq!(o.added.len(), 2),
            None => panic!("expected objects diff"),
        }
    }

    #[test]
    fn absorb_law_objects_associativity() {
        let s0 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }], ..base_snapshot() };
        let s1 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(1) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let s2 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(1, 0), value: PdfObject::Int(9) }, PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }], ..base_snapshot() };
        let s3 = PdfSnapshot { objects: vec![PdfIndirectObject { id: oref(2, 0), value: PdfObject::Int(2) }, PdfIndirectObject { id: oref(3, 0), value: PdfObject::Int(3) }], ..base_snapshot() };
        let d1 = PdfDiff::between(&s0, &s1);
        let d2 = PdfDiff::between(&s1, &s2);
        let d3 = PdfDiff::between(&s2, &s3);
        let mut left = d1.clone();
        left.absorb(d2.clone());
        left.absorb(d3.clone());
        let mut right_tail = d2.clone();
        right_tail.absorb(d3.clone());
        let mut right = d1.clone();
        right.absorb(right_tail);
        assert_eq!(left.apply(&s0).unwrap(), s3);
        assert_eq!(right.apply(&s0).unwrap(), s3);
        assert_eq!(left, right);
    }
    //#endregion absorb_law (objects / id-keyed)

    //#region absorb_law (trailer / name-keyed)
    #[test]
    fn absorb_law_trailer_add_then_setfield_patches_added_payload() {
        let base = PdfSnapshot { trailer: vec![], ..base_snapshot() };
        let d1 = PdfDiff { trailer: Some(PdfDictDiff { added: vec![PdfDictAdded { index: 0, key: "Config".into(), item: dict(vec![]) }], ..Default::default() }), ..Default::default() };
        let d2 = PdfDiff {
            trailer: Some(PdfDictDiff {
                modified: vec![PdfDictModified { key: "Config".into(), diff: PdfValueDiff::Dict { diff: PdfDictDiff { added: vec![PdfDictAdded { index: 0, key: "X".into(), item: PdfObject::Int(5) }], ..Default::default() } } }],
                ..Default::default()
            }),
            ..Default::default()
        };
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.trailer {
            Some(t) => {
                assert!(t.modified.is_empty());
                assert_eq!(t.added[0].item, dict(vec![("X", PdfObject::Int(5))]));
            }
            None => panic!("expected trailer diff"),
        }
    }

    #[test]
    fn absorb_law_trailer_modify_then_remove_drops_pending_patch() {
        let base = PdfSnapshot { trailer: vec![entry("A", PdfObject::Int(1)), entry("B", PdfObject::Int(2))], ..base_snapshot() };
        let d1 = PdfDiff { trailer: Some(PdfDictDiff { modified: vec![PdfDictModified { key: "A".into(), diff: PdfValueDiff::Int { value: 9 } }], ..Default::default() }), ..Default::default() };
        let d2 = PdfDiff { trailer: Some(PdfDictDiff { removed: vec!["A".into()], ..Default::default() }), ..Default::default() };
        let sequential = d2.apply(&d1.apply(&base).unwrap()).unwrap();
        let mut combined = d1.clone();
        combined.absorb(d2.clone());
        assert_eq!(combined.apply(&base).unwrap(), sequential);
        match &combined.trailer {
            Some(t) => {
                assert_eq!(t.removed, vec!["A".to_string()]);
                assert!(t.modified.is_empty());
            }
            None => panic!("expected trailer diff"),
        }
    }
    //#endregion absorb_law (trailer / name-keyed)

    //#region field_sweep
    /// 📏 `sweep_a`/`sweep_b` differ in EVERY mutable field. `pages` is DEPTH-ASYMMETRIC on
    /// purpose (`a` has 3, `b` has 2) so a SINGLE `between(a,b)` call cannot produce both
    /// `removed` and `added` on that positional collection (documented structural trap) --
    /// `removed` shows up in `between(a,b)`, `added` in `between(b,a)`. `objects`/`trailer` are
    /// id-/name-keyed so both directions freely carry removed+modified+added simultaneously.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_a() -> PdfSnapshot {
        base_snapshot()
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn sweep_b() -> PdfSnapshot {
        PdfSnapshot {
            schema: STDIO_PDF17_DOCUMENT_SCHEMA.into(),
            declared_version: "1.4".into(), // scalar change
            pages: vec![
                page([0.0, 0.0, 200.0, 200.0], Some([1.0, 1.0, 50.0, 50.0]), 90, "ONE"), // modified: every field, crop_box None->Some
            ], // base had 2 pages -> this direction's tail (index1) is a `removed`
            info: PdfInfo { title: Some("Changed".into()), author: Some("Ueli".into()), ..Default::default() }, // weak whole-value replace
            objects: vec![
                PdfIndirectObject { id: oref(1, 0), value: dict(vec![("Type", PdfObject::Name("Catalog".into())), ("Count", PdfObject::Int(4)), ("New", PdfObject::Bool(false))]) }, // modified: kept+modified+added inside, "Type" kept, "Count" changed, "New" added (base's implicit lack of "New")
                PdfIndirectObject { id: oref(3, 0), value: PdfObject::Name("Added".into()) },                                                                                        // added (base's obj id=2 is absent here -> removed)
            ],
            trailer: vec![entry("Root", PdfObject::Ref(oref(1, 0))), entry("Size", PdfObject::Int(3)), entry("Prev", PdfObject::Int(100))], // modified Size, added Prev
        }
    }

    #[test]
    fn field_sweep_between_roundtrips_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        assert_eq!(PdfDiff::between(&a, &b).apply(&a).unwrap(), b);
        assert_eq!(PdfDiff::between(&b, &a).apply(&b).unwrap(), a);
        assert!(PdfDiff::between(&a, &a).is_empty());
    }

    #[test]
    fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let ab = PdfDiff::between(&a, &b);
        assert!(ab.declared_version.is_some(), "declaredVersion must be present");
        assert!(ab.info.is_some(), "info must be present (weak whole-value replace)");

        let pages = ab.pages.as_ref().expect("pages diff must be present");
        assert!(!pages.modified.is_empty(), "expected a modified page (every field changed)");
        assert!(!pages.removed.is_empty(), "a has more pages than b -> removed tail expected in between(a,b)");
        let ba_pages = PdfDiff::between(&b, &a).pages.expect("pages diff must be present");
        assert!(!ba_pages.added.is_empty(), "b has fewer pages than a -> added tail expected in between(b,a)");
        let pmod = &pages.modified[0].diff;
        assert!(pmod.media_box.is_some() && pmod.crop_box.is_some() && pmod.rotate.is_some() && pmod.text.is_some(), "every PdfPageDiff field must be exercised: {pmod:?}");
        assert_eq!(pmod.crop_box, Some(Some([1.0, 1.0, 50.0, 50.0])), "crop_box tri-state None->Some must round-trip");

        let objects = ab.objects.as_ref().expect("objects diff must be present");
        assert!(!objects.removed.is_empty(), "obj id=2 only in a -> removed");
        assert!(!objects.modified.is_empty(), "obj id=1 changed in both -> modified");
        assert!(!objects.added.is_empty(), "obj id=3 only in b -> added");
        match &objects.modified[0].diff {
            PdfValueDiff::Dict { diff } => {
                assert!(!diff.modified.is_empty(), "Count field must show as a nested Dict-modified entry");
                assert!(!diff.added.is_empty(), "New key must show as a nested Dict-added entry");
            }
            other => panic!("expected a recursive Dict value diff for obj id=1, got {other:?}"),
        }

        let trailer = ab.trailer.as_ref().expect("trailer diff must be present");
        assert!(!trailer.removed.is_empty(), "Extra must be removed");
        assert!(!trailer.modified.is_empty(), "Size must be modified");
        assert!(!trailer.added.is_empty(), "Prev must be added");
    }
    //#endregion field_sweep
