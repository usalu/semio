//! 🔧️ Entity diff application of the DXF diff: the emptiness and field-change helpers, the inverse and the apply functions and the index element behaviour of an entity. See [`super`].

use super::*;

//#region 🔖️EntityDiff
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn entity_diff_is_empty(d: &DxfEntityDiff) -> bool {
    match d {
        DxfEntityDiff::Replace { .. } => false,
        DxfEntityDiff::Line(x) => x == &DxfLineDiff::default(),
        DxfEntityDiff::Circle(x) => x == &DxfCircleDiff::default(),
        DxfEntityDiff::Arc(x) => x == &DxfArcDiff::default(),
        DxfEntityDiff::Polyline(x) => x == &DxfPolylineDiff::default(),
        DxfEntityDiff::Text(x) => x == &DxfTextDiff::default(),
        DxfEntityDiff::Solid(x) => x == &DxfSolidDiff::default(),
        DxfEntityDiff::Insert(x) => x == &DxfInsertDiff::default(),
        DxfEntityDiff::Other(x) => x == &DxfOtherDiff::default(),
    }
}

/// 🧭️ Kind-matched pairwise `between` — same kind at both positions produces a sparse
/// kind-specific diff; a kind change produces `Replace{entity: b}`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn entity_field_changes(a: &DxfEntity, b: &DxfEntity) -> DxfEntityDiff {
    match (a, b) {
        (DxfEntity::Line(DxfLine { start: sa, end: ea, layer: la, unknown_group_codes: ua }), DxfEntity::Line(DxfLine { start: sb, end: eb, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Line(DxfLineDiff { start: (sa != sb).then_some(*sb), end: (ea != eb).then_some(*eb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Circle(DxfCircle { center: ca, radius: ra, layer: la, unknown_group_codes: ua }), DxfEntity::Circle(DxfCircle { center: cb, radius: rb, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Circle(DxfCircleDiff { center: (ca != cb).then_some(*cb), radius: (ra != rb).then_some(*rb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Arc(DxfArc { center: ca, radius: ra, start_angle: saa, end_angle: eaa, layer: la, unknown_group_codes: ua }), DxfEntity::Arc(DxfArc { center: cb, radius: rb, start_angle: sab, end_angle: eab, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Arc(DxfArcDiff {
                center: (ca != cb).then_some(*cb),
                radius: (ra != rb).then_some(*rb),
                start_angle: (saa != sab).then_some(*sab),
                end_angle: (eaa != eab).then_some(*eab),
                layer: (la != lb).then(|| lb.clone()),
                unknown_group_codes: (ua != ub).then(|| ub.clone()),
            })
        }
        (DxfEntity::Polyline(DxfPolyline { vertices: va, closed: cla, layer: la, unknown_group_codes: ua }), DxfEntity::Polyline(DxfPolyline { vertices: vb, closed: clb, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Polyline(DxfPolylineDiff { vertices: (va != vb).then(|| vb.clone()), closed: (cla != clb).then_some(*clb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Text(DxfText { position: pa, height: ha, value: vaa, layer: la, unknown_group_codes: ua }), DxfEntity::Text(DxfText { position: pb, height: hb, value: vab, layer: lb, unknown_group_codes: ub })) => DxfEntityDiff::Text(DxfTextDiff {
            position: (pa != pb).then_some(*pb),
            height: (ha != hb).then_some(*hb),
            value: (vaa != vab).then(|| vab.clone()),
            layer: (la != lb).then(|| lb.clone()),
            unknown_group_codes: (ua != ub).then(|| ub.clone()),
        }),
        (DxfEntity::Solid(DxfSolid { points: pa, layer: la, unknown_group_codes: ua }), DxfEntity::Solid(DxfSolid { points: pb, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Solid(DxfSolidDiff { points: (pa != pb).then_some(*pb), layer: (la != lb).then(|| lb.clone()), unknown_group_codes: (ua != ub).then(|| ub.clone()) })
        }
        (DxfEntity::Insert(DxfInsert { block_name: ba, position: pa, scale: sca, rotation: ra, layer: la, unknown_group_codes: ua }), DxfEntity::Insert(DxfInsert { block_name: bb, position: pb, scale: scb, rotation: rb, layer: lb, unknown_group_codes: ub })) => {
            DxfEntityDiff::Insert(DxfInsertDiff {
                block_name: (ba != bb).then(|| bb.clone()),
                position: (pa != pb).then_some(*pb),
                scale: (sca != scb).then_some(*scb),
                rotation: (ra != rb).then_some(*rb),
                layer: (la != lb).then(|| lb.clone()),
                unknown_group_codes: (ua != ub).then(|| ub.clone()),
            })
        }
        (DxfEntity::Other(DxfOther { kind: ka, group_codes: ga }), DxfEntity::Other(DxfOther { kind: kb, group_codes: gb })) if ka == kb => DxfEntityDiff::Other(DxfOtherDiff { group_codes: (ga != gb).then(|| gb.clone()) }),
        _ => DxfEntityDiff::Replace { entity: b.clone() },
    }
}

/// ↩️ The entity patch restoring exactly the fields `d` sets back to their `base` values; a diff of another kind than the base
/// entity restores the whole base entity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_entity_diff(d: &DxfEntityDiff, base: &DxfEntity) -> DxfEntityDiff {
    match (d, base) {
        (DxfEntityDiff::Line(x), DxfEntity::Line(DxfLine { start, end, layer, unknown_group_codes })) => DxfEntityDiff::Line(DxfLineDiff { start: x.start.map(|_| *start), end: x.end.map(|_| *end), layer: x.layer.as_ref().map(|_| layer.clone()), unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()) }),
        (DxfEntityDiff::Circle(x), DxfEntity::Circle(DxfCircle { center, radius, layer, unknown_group_codes })) => DxfEntityDiff::Circle(DxfCircleDiff { center: x.center.map(|_| *center), radius: x.radius.map(|_| *radius), layer: x.layer.as_ref().map(|_| layer.clone()), unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()) }),
        (DxfEntityDiff::Arc(x), DxfEntity::Arc(DxfArc { center, radius, start_angle, end_angle, layer, unknown_group_codes })) => DxfEntityDiff::Arc(DxfArcDiff {
            center: x.center.map(|_| *center),
            radius: x.radius.map(|_| *radius),
            start_angle: x.start_angle.map(|_| *start_angle),
            end_angle: x.end_angle.map(|_| *end_angle),
            layer: x.layer.as_ref().map(|_| layer.clone()),
            unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()),
        }),
        (DxfEntityDiff::Polyline(x), DxfEntity::Polyline(DxfPolyline { vertices, closed, layer, unknown_group_codes })) => DxfEntityDiff::Polyline(DxfPolylineDiff { vertices: x.vertices.as_ref().map(|_| vertices.clone()), closed: x.closed.map(|_| *closed), layer: x.layer.as_ref().map(|_| layer.clone()), unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()) }),
        (DxfEntityDiff::Text(x), DxfEntity::Text(DxfText { position, height, value, layer, unknown_group_codes })) => DxfEntityDiff::Text(DxfTextDiff {
            position: x.position.map(|_| *position),
            height: x.height.map(|_| *height),
            value: x.value.as_ref().map(|_| value.clone()),
            layer: x.layer.as_ref().map(|_| layer.clone()),
            unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()),
        }),
        (DxfEntityDiff::Solid(x), DxfEntity::Solid(DxfSolid { points, layer, unknown_group_codes })) => DxfEntityDiff::Solid(DxfSolidDiff { points: x.points.map(|_| *points), layer: x.layer.as_ref().map(|_| layer.clone()), unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()) }),
        (DxfEntityDiff::Insert(x), DxfEntity::Insert(DxfInsert { block_name, position, scale, rotation, layer, unknown_group_codes })) => DxfEntityDiff::Insert(DxfInsertDiff {
            block_name: x.block_name.as_ref().map(|_| block_name.clone()),
            position: x.position.map(|_| *position),
            scale: x.scale.map(|_| *scale),
            rotation: x.rotation.map(|_| *rotation),
            layer: x.layer.as_ref().map(|_| layer.clone()),
            unknown_group_codes: x.unknown_group_codes.as_ref().map(|_| unknown_group_codes.clone()),
        }),
        (DxfEntityDiff::Other(x), DxfEntity::Other(DxfOther { group_codes, .. })) => DxfEntityDiff::Other(DxfOtherDiff { group_codes: x.group_codes.as_ref().map(|_| group_codes.clone()) }),
        _ => DxfEntityDiff::Replace { entity: base.clone() },
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_line_diff(d: &DxfLineDiff, start: &mut [f64; 3], end: &mut [f64; 3], layer: &mut String, unknown: &mut Vec<(i32, DxfValue)>) {
    if let Some(v) = d.start {
        *start = v;
    }
    if let Some(v) = d.end {
        *end = v;
    }
    if let Some(v) = &d.layer {
        *layer = v.clone();
    }
    if let Some(v) = &d.unknown_group_codes {
        *unknown = v.clone();
    }
}

/// ▶️ Applies a kind-specific diff to an entity — used both by `diff_apply` (real position) and
/// by absorb's `Replace`+kind-diff branch (patch-into-the-carried-replacement, same shape as the
/// recipe's canonical `Insert+SetField` "patch into added payload" case).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_entity_diff(d: &DxfEntityDiff, item: &mut DxfEntity) {
    match (d, item) {
        (DxfEntityDiff::Line(ld), DxfEntity::Line(DxfLine { start, end, layer, unknown_group_codes })) => apply_line_diff(ld, start, end, layer, unknown_group_codes),
        (DxfEntityDiff::Circle(cd), DxfEntity::Circle(DxfCircle { center, radius, layer, unknown_group_codes })) => {
            if let Some(v) = cd.center {
                *center = v;
            }
            if let Some(v) = cd.radius {
                *radius = v;
            }
            if let Some(v) = &cd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &cd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Arc(ad), DxfEntity::Arc(DxfArc { center, radius, start_angle, end_angle, layer, unknown_group_codes })) => {
            if let Some(v) = ad.center {
                *center = v;
            }
            if let Some(v) = ad.radius {
                *radius = v;
            }
            if let Some(v) = ad.start_angle {
                *start_angle = v;
            }
            if let Some(v) = ad.end_angle {
                *end_angle = v;
            }
            if let Some(v) = &ad.layer {
                *layer = v.clone();
            }
            if let Some(v) = &ad.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Polyline(pd), DxfEntity::Polyline(DxfPolyline { vertices, closed, layer, unknown_group_codes })) => {
            if let Some(v) = &pd.vertices {
                *vertices = v.clone();
            }
            if let Some(v) = pd.closed {
                *closed = v;
            }
            if let Some(v) = &pd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &pd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Text(td), DxfEntity::Text(DxfText { position, height, value, layer, unknown_group_codes })) => {
            if let Some(v) = td.position {
                *position = v;
            }
            if let Some(v) = td.height {
                *height = v;
            }
            if let Some(v) = &td.value {
                *value = v.clone();
            }
            if let Some(v) = &td.layer {
                *layer = v.clone();
            }
            if let Some(v) = &td.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Solid(sd), DxfEntity::Solid(DxfSolid { points, layer, unknown_group_codes })) => {
            if let Some(v) = sd.points {
                *points = v;
            }
            if let Some(v) = &sd.layer {
                *layer = v.clone();
            }
            if let Some(v) = &sd.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Insert(id), DxfEntity::Insert(DxfInsert { block_name, position, scale, rotation, layer, unknown_group_codes })) => {
            if let Some(v) = &id.block_name {
                *block_name = v.clone();
            }
            if let Some(v) = id.position {
                *position = v;
            }
            if let Some(v) = id.scale {
                *scale = v;
            }
            if let Some(v) = id.rotation {
                *rotation = v;
            }
            if let Some(v) = &id.layer {
                *layer = v.clone();
            }
            if let Some(v) = &id.unknown_group_codes {
                *unknown_group_codes = v.clone();
            }
        }
        (DxfEntityDiff::Other(od), DxfEntity::Other(DxfOther { group_codes, .. })) => {
            if let Some(v) = &od.group_codes {
                *group_codes = v.clone();
            }
        }
        _ => {}
    }
}

impl DxfIndexElem for DxfEntity {
    type Diff = DxfEntityDiff;
    fn diff_is_empty(d: &Self::Diff) -> bool {
        entity_diff_is_empty(d)
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        entity_field_changes(a, b)
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        inverse_entity_diff(d, base)
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let DxfEntityDiff::Replace { entity } = d {
            *item = entity.clone();
        } else {
            apply_entity_diff(d, item);
        }
    }
    /// ➕️ Structural, base-free absorb over entity diffs: two same-kind kind-specific diffs
    /// merge field-by-field (LWW); a `Replace` on either side wins (mid/after ultimately becomes
    /// that literal entity), with a trailing kind-specific `other` patched INTO the carried
    /// replacement payload — the recipe's canonical "patch into added/replaced payload" case.
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        *base = match (base.clone(), other) {
            (DxfEntityDiff::Replace { .. }, DxfEntityDiff::Replace { entity: e2 }) => DxfEntityDiff::Replace { entity: e2 },
            (DxfEntityDiff::Replace { mut entity }, other_diff) => {
                apply_entity_diff(&other_diff, &mut entity);
                DxfEntityDiff::Replace { entity }
            }
            (_, DxfEntityDiff::Replace { entity }) => DxfEntityDiff::Replace { entity },
            (DxfEntityDiff::Line(mut a), DxfEntityDiff::Line(b)) => {
                if b.start.is_some() {
                    a.start = b.start;
                }
                if b.end.is_some() {
                    a.end = b.end;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Line(a)
            }
            (DxfEntityDiff::Circle(mut a), DxfEntityDiff::Circle(b)) => {
                if b.center.is_some() {
                    a.center = b.center;
                }
                if b.radius.is_some() {
                    a.radius = b.radius;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Circle(a)
            }
            (DxfEntityDiff::Arc(mut a), DxfEntityDiff::Arc(b)) => {
                if b.center.is_some() {
                    a.center = b.center;
                }
                if b.radius.is_some() {
                    a.radius = b.radius;
                }
                if b.start_angle.is_some() {
                    a.start_angle = b.start_angle;
                }
                if b.end_angle.is_some() {
                    a.end_angle = b.end_angle;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Arc(a)
            }
            (DxfEntityDiff::Polyline(mut a), DxfEntityDiff::Polyline(b)) => {
                if b.vertices.is_some() {
                    a.vertices = b.vertices;
                }
                if b.closed.is_some() {
                    a.closed = b.closed;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Polyline(a)
            }
            (DxfEntityDiff::Text(mut a), DxfEntityDiff::Text(b)) => {
                if b.position.is_some() {
                    a.position = b.position;
                }
                if b.height.is_some() {
                    a.height = b.height;
                }
                if b.value.is_some() {
                    a.value = b.value;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Text(a)
            }
            (DxfEntityDiff::Solid(mut a), DxfEntityDiff::Solid(b)) => {
                if b.points.is_some() {
                    a.points = b.points;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Solid(a)
            }
            (DxfEntityDiff::Insert(mut a), DxfEntityDiff::Insert(b)) => {
                if b.block_name.is_some() {
                    a.block_name = b.block_name;
                }
                if b.position.is_some() {
                    a.position = b.position;
                }
                if b.scale.is_some() {
                    a.scale = b.scale;
                }
                if b.rotation.is_some() {
                    a.rotation = b.rotation;
                }
                if b.layer.is_some() {
                    a.layer = b.layer;
                }
                if b.unknown_group_codes.is_some() {
                    a.unknown_group_codes = b.unknown_group_codes;
                }
                DxfEntityDiff::Insert(a)
            }
            (DxfEntityDiff::Other(mut a), DxfEntityDiff::Other(b)) => {
                if b.group_codes.is_some() {
                    a.group_codes = b.group_codes;
                }
                DxfEntityDiff::Other(a)
            }
            (_, other_diff) => other_diff,
        };
    }
}
//#endregion 🔖️EntityDiff
