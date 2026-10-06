//! 🔺️ SemioBrepDiff — handcrafted sparse diff over `SemioBrepSnapshot`. No
//! `replacement: Option<SemioBrepSnapshot>` full-replace slot — even a whole-document overwrite's
//! diff is the sparse field-by-field `SemioBrepDiff::between(base, next)`.
//!
//! All 6 collections (`vertices`/`edges`/`loops`/`faces`/`shells`/`solids`) are id-keyed and
//! diffed via the SHARED `crate::standards::v1::subsets::base::schema::triples::NamedTripleDiff`
//! (per `w1b-type-ownership.md`: "Use 🧰️triples ... instead of reinventing it"). The generic
//! `apply_named`/`between_named`/`inverse_named`/`absorb_named` algebra functions below are this
//! artifact's OWN copy of the small helper set bcf/docx each keep locally (no shared "diff
//! algebra" module exists yet — see the "shared infra gaps" note in the wave report).
//!
//! `DiffCodec` is hand-rolled (dsl-derive gap: `NamedTripleDiff<K,D,T>` has no `DslField` impl —
//! f6-final-summary.md §4) using the same bracket-depth-aware hex grammar bcf/svg/gif established,
//! reusing the shared `enc_named_triple`/`dec_named_triple`/`split_top_level`/`strip_brackets`
//! codec primitives from `🧰️triples` rather than re-deriving them.

use crate::standards::v1::subsets::base::schema::geometry::native::NativeF64;
use crate::standards::v1::subsets::base::schema::geometry::SemioPoint3;
use crate::standards::v1::subsets::base::schema::triples::{dec_named_triple, enc_named_triple, NamedModified, NamedTripleDiff};


use crate::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot};






use protocol::command::DiffAlgebra;
use protocol::MutationDiff;

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/key-keyed collection algebra, generic over key `K`, item `T`, per-field diff `D` — this
/// artifact's own copy of the bcf/docx-established shape (see module doc comment), operating on
/// the SHARED `NamedTripleDiff` type from `🧰️triples` rather than a locally re-declared one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_named<K, T, D>(base: &[T], other: &[T], key_of: impl Fn(&T) -> K, diff_item: impl Fn(&T, &T) -> Option<D>) -> Option<NamedTripleDiff<K, D, T>>
where
    K: PartialEq + Clone,
    T: Clone + PartialEq,
{
    let mut removed = Vec::new();
    let mut modified = Vec::new();
    for b in base {
        let bk = key_of(b);
        match other.iter().find(|o| key_of(o) == bk) {
            None => removed.push(bk),
            Some(o) if o != b => {
                if let Some(d) = diff_item(b, o) {
                    modified.push(NamedModified { key: bk, diff: d });
                }
            }
            Some(_) => {}
        }
    }
    let mut added = Vec::new();
    for o in other {
        let ok = key_of(o);
        if !base.iter().any(|b| key_of(b) == ok) {
            added.push(o.clone());
        }
    }
    if removed.is_empty() && modified.is_empty() && added.is_empty() {
        None
    } else {
        Some(NamedTripleDiff { removed, modified, added })
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named<K, T, D>(items: &mut Vec<T>, diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, apply_item: impl Fn(&mut T, &D))
where
    K: PartialEq + Clone,
    T: Clone,
{
    items.retain(|i| !diff.removed.contains(&key_of(i)));
    for m in &diff.modified {
        if let Some(item) = items.iter_mut().find(|i| key_of(i) == m.key) {
            apply_item(item, &m.diff);
        }
    }
    for item in &diff.added {
        items.push(item.clone());
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_named<K, T, D>(base_items: &[T], diff: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, inverse_item: impl Fn(&T, &D) -> D) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
{
    let removed: Vec<K> = diff.added.iter().map(&key_of).collect();
    let mut modified = Vec::new();
    for m in &diff.modified {
        if let Some(original) = base_items.iter().find(|i| key_of(i) == m.key) {
            modified.push(NamedModified { key: m.key.clone(), diff: inverse_item(original, &m.diff) });
        }
    }
    let mut added = Vec::new();
    for k in &diff.removed {
        if let Some(original) = base_items.iter().find(|i| &key_of(i) == k) {
            added.push(original.clone());
        }
    }
    NamedTripleDiff { removed, modified, added }
}

/// 🧮️ Name-keyed absorb — identity is the KEY (not position): a `d2`-removal of a `d1`-added key
/// annihilates the add; a `d2`-modify of a `d1`-added key patches into the carried payload;
/// everything else composes directly on the shared key space.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: &NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
where
    K: PartialEq + Clone,
    T: Clone,
    D: Clone,
{
    let d1_added_keys: Vec<K> = d1.added.iter().map(&key_of).collect();
    let mut removed = d1.removed.clone();
    let mut annihilated: Vec<K> = Vec::new();
    for k in &d2.removed {
        if d1_added_keys.contains(k) {
            annihilated.push(k.clone());
        } else if !removed.contains(k) {
            removed.push(k.clone());
        }
    }
    let mut working_added: Vec<T> = d1.added.into_iter().filter(|a| !annihilated.contains(&key_of(a))).collect();
    let mut modified: Vec<NamedModified<K, D>> = d1.modified.into_iter().filter(|m| !removed.contains(&m.key)).collect();
    for m2 in &d2.modified {
        if let Some(added) = working_added.iter_mut().find(|a| key_of(a) == m2.key) {
            apply_item(added, &m2.diff);
            continue;
        }
        if removed.contains(&m2.key) {
            continue;
        }
        match modified.iter_mut().find(|m| m.key == m2.key) {
            Some(existing) => existing.diff = absorb_item(existing.diff.clone(), m2.diff.clone()),
            None => modified.push(NamedModified { key: m2.key.clone(), diff: m2.diff.clone() }),
        }
    }
    for a2 in &d2.added {
        let k2 = key_of(a2);
        match working_added.iter_mut().find(|a| key_of(a) == k2) {
            Some(existing) => *existing = a2.clone(),
            None => working_added.push(a2.clone()),
        }
    }
    NamedTripleDiff { removed, modified, added: working_added }
}
//#endregion 🔖️GenericNamedEngine

//#region 🔖️DiffTypes
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepVertexDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<SemioPoint3>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tol: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepEdgeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start_vertex: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end_vertex: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<BrepCurve>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tol: Option<f64>,
}

/// 🔺️ `edges` is whole-value replaced (the loop's traversal order + orientation set is a weak
/// value, per the recipe — never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepLoopDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BrepLoopEdge>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepFaceDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub outer_loop: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub inner_loops: Option<Vec<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<BrepSurface>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tol: Option<f64>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepShellDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<BrepShellFace>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct BrepSolidDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shells: Option<Vec<BrepSolidShell>>,
}

pub type BrepVerticesDiff = NamedTripleDiff<String, BrepVertexDiff, BrepVertex>;
pub type BrepEdgesDiff = NamedTripleDiff<String, BrepEdgeDiff, BrepEdge>;
pub type BrepLoopsDiff = NamedTripleDiff<String, BrepLoopDiff, BrepLoop>;
pub type BrepFacesDiff = NamedTripleDiff<String, BrepFaceDiff, BrepFace>;
pub type BrepShellsDiff = NamedTripleDiff<String, BrepShellDiff, BrepShell>;
pub type BrepSolidsDiff = NamedTripleDiff<String, BrepSolidDiff, BrepSolid>;

/// 🔺️ Diff for `s.stdio.semio.brep`. `schema` is an identity field — never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioBrepDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<BrepVerticesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<BrepEdgesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub loops: Option<BrepLoopsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<BrepFacesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shells: Option<BrepShellsDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub solids: Option<BrepSolidsDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️PerEntityApply
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_vertex(v: &mut BrepVertex, d: &BrepVertexDiff) {
    if let Some(p) = &d.point {
        v.point = *p;
    }
    if let Some(tol) = d.tol {
        v.tol = tol;
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_edge(e: &mut BrepEdge, d: &BrepEdgeDiff) {
    if let Some(v) = &d.start_vertex {
        e.start_vertex = v.clone();
    }
    if let Some(v) = &d.end_vertex {
        e.end_vertex = v.clone();
    }
    if let Some(v) = &d.curve {
        e.curve = v.clone();
    }
    if let Some(tol) = d.tol {
        e.tol = tol;
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_loop(l: &mut BrepLoop, d: &BrepLoopDiff) {
    if let Some(v) = &d.edges {
        l.edges = v.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_face(f: &mut BrepFace, d: &BrepFaceDiff) {
    if let Some(v) = &d.outer_loop {
        f.outer_loop = v.clone();
    }
    if let Some(v) = &d.inner_loops {
        f.inner_loops = v.clone();
    }
    if let Some(v) = &d.surface {
        f.surface = v.clone();
    }
    if let Some(v) = &d.orientation {
        f.orientation = *v;
    }
    if let Some(tol) = d.tol {
        f.tol = tol;
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_shell(s: &mut BrepShell, d: &BrepShellDiff) {
    if let Some(v) = &d.faces {
        s.faces = v.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_solid(s: &mut BrepSolid, d: &BrepSolidDiff) {
    if let Some(v) = &d.shells {
        s.shells = v.clone();
    }
}
//#endregion 🔖️PerEntityApply

//#region 🔖️PerEntityBetween
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_vertex(a: &BrepVertex, b: &BrepVertex) -> Option<BrepVertexDiff> {
    let point = if a.point != b.point { Some(b.point) } else { None };
    let tol = if a.tol != b.tol { Some(b.tol) } else { None };
    if point.is_none() && tol.is_none() {
        None
    } else {
        Some(BrepVertexDiff { point, tol })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_edge(a: &BrepEdge, b: &BrepEdge) -> Option<BrepEdgeDiff> {
    let start_vertex = if a.start_vertex != b.start_vertex { Some(b.start_vertex.clone()) } else { None };
    let end_vertex = if a.end_vertex != b.end_vertex { Some(b.end_vertex.clone()) } else { None };
    let curve = if a.curve != b.curve { Some(b.curve.clone()) } else { None };
    let tol = if a.tol != b.tol { Some(b.tol) } else { None };
    if start_vertex.is_none() && end_vertex.is_none() && curve.is_none() && tol.is_none() {
        None
    } else {
        Some(BrepEdgeDiff { start_vertex, end_vertex, curve, tol })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_loop(a: &BrepLoop, b: &BrepLoop) -> Option<BrepLoopDiff> {
    let edges = if a.edges != b.edges { Some(b.edges.clone()) } else { None };
    if edges.is_none() {
        None
    } else {
        Some(BrepLoopDiff { edges })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_face(a: &BrepFace, b: &BrepFace) -> Option<BrepFaceDiff> {
    let outer_loop = if a.outer_loop != b.outer_loop { Some(b.outer_loop.clone()) } else { None };
    let inner_loops = if a.inner_loops != b.inner_loops { Some(b.inner_loops.clone()) } else { None };
    let surface = if a.surface != b.surface { Some(b.surface.clone()) } else { None };
    let orientation = if a.orientation != b.orientation { Some(b.orientation) } else { None };
    let tol = if a.tol != b.tol { Some(b.tol) } else { None };
    if outer_loop.is_none() && inner_loops.is_none() && surface.is_none() && orientation.is_none() && tol.is_none() {
        None
    } else {
        Some(BrepFaceDiff { outer_loop, inner_loops, surface, orientation, tol })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_shell(a: &BrepShell, b: &BrepShell) -> Option<BrepShellDiff> {
    let faces = if a.faces != b.faces { Some(b.faces.clone()) } else { None };
    if faces.is_none() {
        None
    } else {
        Some(BrepShellDiff { faces })
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn between_solid(a: &BrepSolid, b: &BrepSolid) -> Option<BrepSolidDiff> {
    let shells = if a.shells != b.shells { Some(b.shells.clone()) } else { None };
    if shells.is_none() {
        None
    } else {
        Some(BrepSolidDiff { shells })
    }
}
//#endregion 🔖️PerEntityBetween

//#region 🔖️PerEntityInverse
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_vertex(base: &BrepVertex, d: &BrepVertexDiff) -> BrepVertexDiff {
    BrepVertexDiff { point: d.point.as_ref().map(|_| base.point), tol: d.tol.map(|_| base.tol) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_edge(base: &BrepEdge, d: &BrepEdgeDiff) -> BrepEdgeDiff {
    BrepEdgeDiff { start_vertex: d.start_vertex.as_ref().map(|_| base.start_vertex.clone()), end_vertex: d.end_vertex.as_ref().map(|_| base.end_vertex.clone()), curve: d.curve.as_ref().map(|_| base.curve.clone()), tol: d.tol.map(|_| base.tol) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_loop(base: &BrepLoop, d: &BrepLoopDiff) -> BrepLoopDiff {
    BrepLoopDiff { edges: d.edges.as_ref().map(|_| base.edges.clone()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_face(base: &BrepFace, d: &BrepFaceDiff) -> BrepFaceDiff {
    BrepFaceDiff {
        outer_loop: d.outer_loop.as_ref().map(|_| base.outer_loop.clone()),
        inner_loops: d.inner_loops.as_ref().map(|_| base.inner_loops.clone()),
        surface: d.surface.as_ref().map(|_| base.surface.clone()),
        orientation: d.orientation.as_ref().map(|_| base.orientation),
        tol: d.tol.map(|_| base.tol),
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_shell(base: &BrepShell, d: &BrepShellDiff) -> BrepShellDiff {
    BrepShellDiff { faces: d.faces.as_ref().map(|_| base.faces.clone()) }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_solid(base: &BrepSolid, d: &BrepSolidDiff) -> BrepSolidDiff {
    BrepSolidDiff { shells: d.shells.as_ref().map(|_| base.shells.clone()) }
}
//#endregion 🔖️PerEntityInverse

//#region 🔖️PerEntityAbsorb
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_vertex_diff(mut a: BrepVertexDiff, b: &BrepVertexDiff) -> BrepVertexDiff {
    if b.point.is_some() {
        a.point = b.point;
    }
    if b.tol.is_some() {
        a.tol = b.tol;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_edge_diff(mut a: BrepEdgeDiff, b: BrepEdgeDiff) -> BrepEdgeDiff {
    if b.start_vertex.is_some() {
        a.start_vertex = b.start_vertex;
    }
    if b.end_vertex.is_some() {
        a.end_vertex = b.end_vertex;
    }
    if b.curve.is_some() {
        a.curve = b.curve;
    }
    if b.tol.is_some() {
        a.tol = b.tol;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_loop_diff(mut a: BrepLoopDiff, b: BrepLoopDiff) -> BrepLoopDiff {
    if b.edges.is_some() {
        a.edges = b.edges;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_face_diff(mut a: BrepFaceDiff, b: BrepFaceDiff) -> BrepFaceDiff {
    if b.outer_loop.is_some() {
        a.outer_loop = b.outer_loop;
    }
    if b.inner_loops.is_some() {
        a.inner_loops = b.inner_loops;
    }
    if b.surface.is_some() {
        a.surface = b.surface;
    }
    if b.orientation.is_some() {
        a.orientation = b.orientation;
    }
    if b.tol.is_some() {
        a.tol = b.tol;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_shell_diff(mut a: BrepShellDiff, b: BrepShellDiff) -> BrepShellDiff {
    if b.faces.is_some() {
        a.faces = b.faces;
    }
    a
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_solid_diff(mut a: BrepSolidDiff, b: BrepSolidDiff) -> BrepSolidDiff {
    if b.shells.is_some() {
        a.shells = b.shells;
    }
    a
}
//#endregion 🔖️PerEntityAbsorb

//#region 🔖️Apply
impl MutationDiff<SemioBrepSnapshot> for SemioBrepDiff {
    fn apply(&self, base: &SemioBrepSnapshot) -> protocol::MutationApplyResult<SemioBrepSnapshot> {
        let mut next = base.clone();
        if let Some(d) = &self.vertices {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.vertices, d, |item| item.id.clone(), |item| item.id.clone(), ["vertices"])?;
            apply_named(&mut next.vertices, d, |v: &BrepVertex| v.id.clone(), apply_vertex);
        }
        if let Some(d) = &self.edges {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.edges, d, |item| item.id.clone(), |item| item.id.clone(), ["edges"])?;
            apply_named(&mut next.edges, d, |e: &BrepEdge| e.id.clone(), apply_edge);
        }
        if let Some(d) = &self.loops {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.loops, d, |item| item.id.clone(), |item| item.id.clone(), ["loops"])?;
            apply_named(&mut next.loops, d, |l: &BrepLoop| l.id.clone(), apply_loop);
        }
        if let Some(d) = &self.faces {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.faces, d, |item| item.id.clone(), |item| item.id.clone(), ["faces"])?;
            apply_named(&mut next.faces, d, |f: &BrepFace| f.id.clone(), apply_face);
        }
        if let Some(d) = &self.shells {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.shells, d, |item| item.id.clone(), |item| item.id.clone(), ["shells"])?;
            apply_named(&mut next.shells, d, |s: &BrepShell| s.id.clone(), apply_shell);
        }
        if let Some(d) = &self.solids {
            crate::standards::v1::subsets::base::schema::triples::validate_named_triple(&next.solids, d, |item| item.id.clone(), |item| item.id.clone(), ["solids"])?;
            apply_named(&mut next.solids, d, |s: &BrepSolid| s.id.clone(), apply_solid);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.vertices = match (self.vertices.take(), other.vertices) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |v: &BrepVertex| v.id.clone(), |a, b| absorb_vertex_diff(a, &b), apply_vertex)),
        };
        self.edges = match (self.edges.take(), other.edges) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |e: &BrepEdge| e.id.clone(), absorb_edge_diff, apply_edge)),
        };
        self.loops = match (self.loops.take(), other.loops) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |l: &BrepLoop| l.id.clone(), absorb_loop_diff, apply_loop)),
        };
        self.faces = match (self.faces.take(), other.faces) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |f: &BrepFace| f.id.clone(), absorb_face_diff, apply_face)),
        };
        self.shells = match (self.shells.take(), other.shells) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |s: &BrepShell| s.id.clone(), absorb_shell_diff, apply_shell)),
        };
        self.solids = match (self.solids.take(), other.solids) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, &b, |s: &BrepSolid| s.id.clone(), absorb_solid_diff, apply_solid)),
        };
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffAlgebra
impl DiffAlgebra<SemioBrepSnapshot> for SemioBrepDiff {
    fn inverse(&self, base: &SemioBrepSnapshot) -> Self {
        Self {
            vertices: self.vertices.as_ref().map(|d| inverse_named(&base.vertices, d, |v: &BrepVertex| v.id.clone(), inverse_vertex)),
            edges: self.edges.as_ref().map(|d| inverse_named(&base.edges, d, |e: &BrepEdge| e.id.clone(), inverse_edge)),
            loops: self.loops.as_ref().map(|d| inverse_named(&base.loops, d, |l: &BrepLoop| l.id.clone(), inverse_loop)),
            faces: self.faces.as_ref().map(|d| inverse_named(&base.faces, d, |f: &BrepFace| f.id.clone(), inverse_face)),
            shells: self.shells.as_ref().map(|d| inverse_named(&base.shells, d, |s: &BrepShell| s.id.clone(), inverse_shell)),
            solids: self.solids.as_ref().map(|d| inverse_named(&base.solids, d, |s: &BrepSolid| s.id.clone(), inverse_solid)),
        }
    }

    fn between(base: &SemioBrepSnapshot, other: &SemioBrepSnapshot) -> Self {
        Self {
            vertices: between_named(&base.vertices, &other.vertices, |v: &BrepVertex| v.id.clone(), between_vertex),
            edges: between_named(&base.edges, &other.edges, |e: &BrepEdge| e.id.clone(), between_edge),
            loops: between_named(&base.loops, &other.loops, |l: &BrepLoop| l.id.clone(), between_loop),
            faces: between_named(&base.faces, &other.faces, |f: &BrepFace| f.id.clone(), between_face),
            shells: between_named(&base.shells, &other.shells, |s: &BrepShell| s.id.clone(), between_shell),
            solids: between_named(&base.solids, &other.solids, |s: &BrepSolid| s.id.clone(), between_solid),
        }
    }

    fn is_empty(&self) -> bool {
        self.vertices.is_none() && self.edges.is_none() && self.loops.is_none() && self.faces.is_none() && self.shells.is_none() && self.solids.is_none()
    }
}
//#endregion 🔖️DiffAlgebra

//#region 🔖️HandcraftedDiffCodec
//#region 🔖️Primitives












//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs


























//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs

















//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel









//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🌉️ExternalCodecBridge

//#endregion 🌉️ExternalCodecBridge

//#region 🔖️Demo
/// 🌱 Representative `SemioBrepDiff` cases (empty/no-op, a full removed/modified/added sweep both
/// directions across every collection, plus a bare insert) — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️.rs`. Self-
/// contained (does not reach into `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`,
/// since a private item of a child module is not visible to its parent).
#[cfg(all(test, feature = "conversion-brep"))]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioBrepDiff> {
    let mut a = SemioBrepSnapshot::default();
    a.vertices = vec![BrepVertex { tol: 1e-7, id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }, BrepVertex { tol: 1e-7, id: "v-removed".into(), point: SemioPoint3::default() }];
    a.edges = vec![BrepEdge { tol: 1e-7, id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
    a.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
    a.faces = vec![BrepFace { tol: 1e-7, id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
    a.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    a.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];

    let mut b = SemioBrepSnapshot::default();
    b.vertices = vec![BrepVertex { tol: 1e-7, id: "v1".into(), point: SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 } }, BrepVertex { tol: 1e-7, id: "v-added".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }];
    b.edges = vec![BrepEdge {
        tol: 1e-7,
        id: "e1".into(),
        start_vertex: "v1".into(),
        end_vertex: "v-added".into(),
        curve: BrepCurve::Nurbs { control_points: vec![SemioPoint3::default(), SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] },
    }];
    b.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] }];
    b.faces = vec![BrepFace { tol: 1e-7, id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec!["l1".into()], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 2.0 }, orientation: false }];
    b.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: false }] }];
    b.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: true }] }, BrepSolid { id: "so-added".into(), shells: vec![] }];

    vec![SemioBrepDiff::default(), <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&a, &b), <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
