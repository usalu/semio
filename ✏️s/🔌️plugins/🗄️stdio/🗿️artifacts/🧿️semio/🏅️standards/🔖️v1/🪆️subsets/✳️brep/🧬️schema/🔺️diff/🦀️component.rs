//! 🔺️ SemioBrepDiff — handcrafted sparse diff over `SemioBrepSnapshot`. No
//! `replacement: Option<SemioBrepSnapshot>` full-replace slot — even a whole-document overwrite's
//! diff is the sparse field-by-field `SemioBrepDiff::between(base, next)`.
//!
//! All 6 collections (`vertices`/`edges`/`loops`/`faces`/`shells`/`solids`) are id-keyed and
//! diffed via the SHARED `crate::artifacts::semio::standards::v1::subsets::any::schema::triples::NamedTripleDiff`
//! (per `w1b-type-ownership.md`: "Use 🧰️triples ... instead of reinventing it"). The generic
//! `apply_named`/`between_named`/`inverse_named`/`absorb_named` algebra functions below are this
//! artifact's OWN copy of the small helper set bcf/docx each keep locally (no shared "diff
//! algebra" module exists yet — see the "shared infra gaps" note in the wave report).
//!
//! `DiffCodec` is hand-rolled (dsl-derive gap: `NamedTripleDiff<K,D,T>` has no `DslField` impl —
//! f6-final-summary.md §4) using the same bracket-depth-aware hex grammar bcf/svg/gif established,
//! reusing the shared `enc_named_triple`/`dec_named_triple`/`split_top_level`/`strip_brackets`
//! codec primitives from `🧰️triples` rather than re-deriving them.

use crate::artifacts::semio::standards::v1::subsets::any::schema::geometry::SemioPoint3;
use crate::artifacts::semio::standards::v1::subsets::any::schema::triples::{dec_named_triple, enc_named_triple, split_top_level, strip_brackets, NamedModified, NamedTripleDiff};
use crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::{BrepCurve, BrepEdge, BrepFace, BrepLoop, BrepLoopEdge, BrepShell, BrepShellFace, BrepSolid, BrepSolidShell, BrepSurface, BrepVertex, SemioBrepSnapshot};
use protocol::command::DiffAlgebra;
use protocol::MutationDiff;
use serde::{Deserialize, Serialize};

//#region 🔖️GenericNamedEngine
/// 🏷️ Name/key-keyed collection algebra, generic over key `K`, item `T`, per-field diff `D` — this
/// artifact's own copy of the bcf/docx-established shape (see module doc comment), operating on
/// the SHARED `NamedTripleDiff` type from `🧰️triples` rather than a locally re-declared one.
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
fn absorb_named<K, T, D>(d1: NamedTripleDiff<K, D, T>, d2: NamedTripleDiff<K, D, T>, key_of: impl Fn(&T) -> K, absorb_item: impl Fn(D, D) -> D, apply_item: impl Fn(&mut T, &D)) -> NamedTripleDiff<K, D, T>
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
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepVertexDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub point: Option<SemioPoint3>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepEdgeDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_vertex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_vertex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub curve: Option<BrepCurve>,
}

/// 🔺️ `edges` is whole-value replaced (the loop's traversal order + orientation set is a weak
/// value, per the recipe — never sub-diffed).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepLoopDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<Vec<BrepLoopEdge>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepFaceDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outer_loop: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_loops: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<BrepSurface>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepShellDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<BrepShellFace>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrepSolidDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shells: Option<Vec<BrepSolidShell>>,
}

pub type BrepVerticesDiff = NamedTripleDiff<String, BrepVertexDiff, BrepVertex>;
pub type BrepEdgesDiff = NamedTripleDiff<String, BrepEdgeDiff, BrepEdge>;
pub type BrepLoopsDiff = NamedTripleDiff<String, BrepLoopDiff, BrepLoop>;
pub type BrepFacesDiff = NamedTripleDiff<String, BrepFaceDiff, BrepFace>;
pub type BrepShellsDiff = NamedTripleDiff<String, BrepShellDiff, BrepShell>;
pub type BrepSolidsDiff = NamedTripleDiff<String, BrepSolidDiff, BrepSolid>;

/// 🔺️ Diff for `s.stdio.semio.brep`. `schema` is an identity field — never appears here.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemioBrepDiff {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<BrepVerticesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edges: Option<BrepEdgesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loops: Option<BrepLoopsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<BrepFacesDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shells: Option<BrepShellsDiff>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solids: Option<BrepSolidsDiff>,
}
//#endregion 🔖️DiffTypes

//#region 🔖️PerEntityApply
fn apply_vertex(v: &mut BrepVertex, d: &BrepVertexDiff) {
    if let Some(p) = &d.point {
        v.point = *p;
    }
}
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
}
fn apply_loop(l: &mut BrepLoop, d: &BrepLoopDiff) {
    if let Some(v) = &d.edges {
        l.edges = v.clone();
    }
}
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
}
fn apply_shell(s: &mut BrepShell, d: &BrepShellDiff) {
    if let Some(v) = &d.faces {
        s.faces = v.clone();
    }
}
fn apply_solid(s: &mut BrepSolid, d: &BrepSolidDiff) {
    if let Some(v) = &d.shells {
        s.shells = v.clone();
    }
}
//#endregion 🔖️PerEntityApply

//#region 🔖️PerEntityBetween
fn between_vertex(a: &BrepVertex, b: &BrepVertex) -> Option<BrepVertexDiff> {
    let point = if a.point != b.point { Some(b.point) } else { None };
    if point.is_none() {
        None
    } else {
        Some(BrepVertexDiff { point })
    }
}
fn between_edge(a: &BrepEdge, b: &BrepEdge) -> Option<BrepEdgeDiff> {
    let start_vertex = if a.start_vertex != b.start_vertex { Some(b.start_vertex.clone()) } else { None };
    let end_vertex = if a.end_vertex != b.end_vertex { Some(b.end_vertex.clone()) } else { None };
    let curve = if a.curve != b.curve { Some(b.curve.clone()) } else { None };
    if start_vertex.is_none() && end_vertex.is_none() && curve.is_none() {
        None
    } else {
        Some(BrepEdgeDiff { start_vertex, end_vertex, curve })
    }
}
fn between_loop(a: &BrepLoop, b: &BrepLoop) -> Option<BrepLoopDiff> {
    let edges = if a.edges != b.edges { Some(b.edges.clone()) } else { None };
    if edges.is_none() {
        None
    } else {
        Some(BrepLoopDiff { edges })
    }
}
fn between_face(a: &BrepFace, b: &BrepFace) -> Option<BrepFaceDiff> {
    let outer_loop = if a.outer_loop != b.outer_loop { Some(b.outer_loop.clone()) } else { None };
    let inner_loops = if a.inner_loops != b.inner_loops { Some(b.inner_loops.clone()) } else { None };
    let surface = if a.surface != b.surface { Some(b.surface.clone()) } else { None };
    let orientation = if a.orientation != b.orientation { Some(b.orientation) } else { None };
    if outer_loop.is_none() && inner_loops.is_none() && surface.is_none() && orientation.is_none() {
        None
    } else {
        Some(BrepFaceDiff { outer_loop, inner_loops, surface, orientation })
    }
}
fn between_shell(a: &BrepShell, b: &BrepShell) -> Option<BrepShellDiff> {
    let faces = if a.faces != b.faces { Some(b.faces.clone()) } else { None };
    if faces.is_none() {
        None
    } else {
        Some(BrepShellDiff { faces })
    }
}
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
fn inverse_vertex(base: &BrepVertex, d: &BrepVertexDiff) -> BrepVertexDiff {
    BrepVertexDiff { point: d.point.as_ref().map(|_| base.point) }
}
fn inverse_edge(base: &BrepEdge, d: &BrepEdgeDiff) -> BrepEdgeDiff {
    BrepEdgeDiff { start_vertex: d.start_vertex.as_ref().map(|_| base.start_vertex.clone()), end_vertex: d.end_vertex.as_ref().map(|_| base.end_vertex.clone()), curve: d.curve.as_ref().map(|_| base.curve.clone()) }
}
fn inverse_loop(base: &BrepLoop, d: &BrepLoopDiff) -> BrepLoopDiff {
    BrepLoopDiff { edges: d.edges.as_ref().map(|_| base.edges.clone()) }
}
fn inverse_face(base: &BrepFace, d: &BrepFaceDiff) -> BrepFaceDiff {
    BrepFaceDiff {
        outer_loop: d.outer_loop.as_ref().map(|_| base.outer_loop.clone()),
        inner_loops: d.inner_loops.as_ref().map(|_| base.inner_loops.clone()),
        surface: d.surface.as_ref().map(|_| base.surface.clone()),
        orientation: d.orientation.as_ref().map(|_| base.orientation),
    }
}
fn inverse_shell(base: &BrepShell, d: &BrepShellDiff) -> BrepShellDiff {
    BrepShellDiff { faces: d.faces.as_ref().map(|_| base.faces.clone()) }
}
fn inverse_solid(base: &BrepSolid, d: &BrepSolidDiff) -> BrepSolidDiff {
    BrepSolidDiff { shells: d.shells.as_ref().map(|_| base.shells.clone()) }
}
//#endregion 🔖️PerEntityInverse

//#region 🔖️PerEntityAbsorb
fn absorb_vertex_diff(mut a: BrepVertexDiff, b: BrepVertexDiff) -> BrepVertexDiff {
    if b.point.is_some() {
        a.point = b.point;
    }
    a
}
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
    a
}
fn absorb_loop_diff(mut a: BrepLoopDiff, b: BrepLoopDiff) -> BrepLoopDiff {
    if b.edges.is_some() {
        a.edges = b.edges;
    }
    a
}
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
    a
}
fn absorb_shell_diff(mut a: BrepShellDiff, b: BrepShellDiff) -> BrepShellDiff {
    if b.faces.is_some() {
        a.faces = b.faces;
    }
    a
}
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
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.vertices, d, |item| item.id.clone(), |item| item.id.clone(), ["vertices"])?;
            apply_named(&mut next.vertices, d, |v: &BrepVertex| v.id.clone(), apply_vertex);
        }
        if let Some(d) = &self.edges {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.edges, d, |item| item.id.clone(), |item| item.id.clone(), ["edges"])?;
            apply_named(&mut next.edges, d, |e: &BrepEdge| e.id.clone(), apply_edge);
        }
        if let Some(d) = &self.loops {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.loops, d, |item| item.id.clone(), |item| item.id.clone(), ["loops"])?;
            apply_named(&mut next.loops, d, |l: &BrepLoop| l.id.clone(), apply_loop);
        }
        if let Some(d) = &self.faces {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.faces, d, |item| item.id.clone(), |item| item.id.clone(), ["faces"])?;
            apply_named(&mut next.faces, d, |f: &BrepFace| f.id.clone(), apply_face);
        }
        if let Some(d) = &self.shells {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.shells, d, |item| item.id.clone(), |item| item.id.clone(), ["shells"])?;
            apply_named(&mut next.shells, d, |s: &BrepShell| s.id.clone(), apply_shell);
        }
        if let Some(d) = &self.solids {
            crate::artifacts::semio::standards::v1::subsets::any::schema::triples::validate_named_triple(&next.solids, d, |item| item.id.clone(), |item| item.id.clone(), ["solids"])?;
            apply_named(&mut next.solids, d, |s: &BrepSolid| s.id.clone(), apply_solid);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        self.vertices = match (self.vertices.take(), other.vertices) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |v: &BrepVertex| v.id.clone(), absorb_vertex_diff, apply_vertex)),
        };
        self.edges = match (self.edges.take(), other.edges) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |e: &BrepEdge| e.id.clone(), absorb_edge_diff, apply_edge)),
        };
        self.loops = match (self.loops.take(), other.loops) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |l: &BrepLoop| l.id.clone(), absorb_loop_diff, apply_loop)),
        };
        self.faces = match (self.faces.take(), other.faces) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |f: &BrepFace| f.id.clone(), absorb_face_diff, apply_face)),
        };
        self.shells = match (self.shells.take(), other.shells) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |s: &BrepShell| s.id.clone(), absorb_shell_diff, apply_shell)),
        };
        self.solids = match (self.solids.take(), other.solids) {
            (None, b) => b,
            (a, None) => a,
            (Some(a), Some(b)) => Some(absorb_named(a, b, |s: &BrepSolid| s.id.clone(), absorb_solid_diff, apply_solid)),
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
pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
pub(crate) fn hex_decode(s: &str) -> Result<Vec<u8>, String> {
    if s.len() % 2 != 0 {
        return Err(format!("odd hex length: {s:?}"));
    }
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}
pub(crate) fn enc_str(s: &str) -> String {
    hex_encode(s.as_bytes())
}
pub(crate) fn dec_str(s: &str) -> Result<String, String> {
    String::from_utf8(hex_decode(s)?).map_err(|e| e.to_string())
}
pub(crate) fn parse_f64(s: &str) -> Result<f64, String> {
    s.parse().map_err(|e: std::num::ParseFloatError| e.to_string())
}
pub(crate) fn parse_u32(s: &str) -> Result<u32, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
pub(crate) fn enc_bool(b: bool) -> &'static str {
    if b {
        "1"
    } else {
        "0"
    }
}
pub(crate) fn parse_bool(s: &str) -> Result<bool, String> {
    match s {
        "1" => Ok(true),
        "0" => Ok(false),
        other => Err(format!("bad bool {other:?}")),
    }
}
pub(crate) fn encode_option<T>(opt: &Option<T>, enc: impl Fn(&T) -> String) -> String {
    match opt {
        None => "[0]".to_string(),
        Some(v) => format!("[1,{}]", enc(v)),
    }
}
pub(crate) fn decode_option<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Option<T>, String> {
    let inner = strip_brackets(s)?;
    match split_top_level(inner, ',').as_slice() {
        ["0"] => Ok(None),
        [tag, value] if *tag == "1" => Ok(Some(dec(value)?)),
        other => Err(format!("option decode: bad shape {other:?}")),
    }
}
pub(crate) fn enc_list<T>(items: &[T], enc: impl Fn(&T) -> String) -> String {
    format!("[{}]", items.iter().map(|it| enc(it)).collect::<Vec<_>>().join(","))
}
pub(crate) fn dec_list<T>(s: &str, dec: impl Fn(&str) -> Result<T, String>) -> Result<Vec<T>, String> {
    split_top_level(strip_brackets(s)?, ',').into_iter().filter(|s| !s.is_empty()).map(|entry| dec(entry)).collect()
}
//#endregion 🔖️Primitives

//#region 🔖️ValueCodecs
pub(crate) fn enc_point3(p: &SemioPoint3) -> String {
    format!("[{},{},{}]", p.x, p.y, p.z)
}
pub(crate) fn dec_point3(s: &str) -> Result<SemioPoint3, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [x, y, z] = parts.as_slice() else { return Err(format!("point3: expected 3 fields, got {}", parts.len())) };
    Ok(SemioPoint3 { x: parse_f64(x)?, y: parse_f64(y)?, z: parse_f64(z)? })
}

/// 📈️ `L[origin,direction]` / `C[center,axis,radius]` / `E[center,axis,radiusMajor,radiusMinor]` /
/// `N[controlPoints,weights,degree,knots]` — single-letter tag prefix, same convention as bcf's
/// `enc_camera`/svg's `enc_xml_node`.
pub(crate) fn enc_curve(c: &BrepCurve) -> String {
    match c {
        BrepCurve::Line { origin, direction } => format!("L[{},{}]", enc_point3(origin), enc_point3(direction)),
        BrepCurve::Circle { center, axis, radius } => format!("C[{},{},{}]", enc_point3(center), enc_point3(axis), radius),
        BrepCurve::Ellipse { center, axis, radius_major, radius_minor } => {
            format!("E[{},{},{},{}]", enc_point3(center), enc_point3(axis), radius_major, radius_minor)
        }
        BrepCurve::Nurbs { control_points, weights, degree, knots } => format!("N[{},{},{},{}]", enc_list(control_points, enc_point3), enc_list(weights, |w: &f64| w.to_string()), degree, enc_list(knots, |k: &f64| k.to_string()),),
    }
}
pub(crate) fn dec_curve(s: &str) -> Result<BrepCurve, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "L" => {
            let [origin, direction] = parts.as_slice() else { return Err(format!("curve line: expected 2 fields, got {}", parts.len())) };
            Ok(BrepCurve::Line { origin: dec_point3(origin)?, direction: dec_point3(direction)? })
        }
        "C" => {
            let [center, axis, radius] = parts.as_slice() else { return Err(format!("curve circle: expected 3 fields, got {}", parts.len())) };
            Ok(BrepCurve::Circle { center: dec_point3(center)?, axis: dec_point3(axis)?, radius: parse_f64(radius)? })
        }
        "E" => {
            let [center, axis, radius_major, radius_minor] = parts.as_slice() else { return Err(format!("curve ellipse: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve::Ellipse { center: dec_point3(center)?, axis: dec_point3(axis)?, radius_major: parse_f64(radius_major)?, radius_minor: parse_f64(radius_minor)? })
        }
        "N" => {
            let [control_points, weights, degree, knots] = parts.as_slice() else { return Err(format!("curve nurbs: expected 4 fields, got {}", parts.len())) };
            Ok(BrepCurve::Nurbs { control_points: dec_list(control_points, dec_point3)?, weights: dec_list(weights, parse_f64)?, degree: parse_u32(degree)?, knots: dec_list(knots, parse_f64)? })
        }
        other => Err(format!("curve: unknown tag {other:?}")),
    }
}

/// 🗺️ `P[origin,normal]` / `C[origin,axis,radius]` (cylinder) / `O[origin,axis,radius,halfAngle]`
/// (cone) / `S[center,radius]` (sphere) / `T[center,axis,majorRadius,minorRadius]` (torus) /
/// `N[controlPoints,weights,uCount,vCount,degreeU,degreeV,knotsU,knotsV]`.
pub(crate) fn enc_surface(s: &BrepSurface) -> String {
    match s {
        BrepSurface::Plane { origin, normal } => format!("P[{},{}]", enc_point3(origin), enc_point3(normal)),
        BrepSurface::Cylinder { origin, axis, radius } => format!("C[{},{},{}]", enc_point3(origin), enc_point3(axis), radius),
        BrepSurface::Cone { origin, axis, radius, half_angle } => format!("O[{},{},{},{}]", enc_point3(origin), enc_point3(axis), radius, half_angle),
        BrepSurface::Sphere { center, radius } => format!("S[{},{}]", enc_point3(center), radius),
        BrepSurface::Torus { center, axis, major_radius, minor_radius } => format!("T[{},{},{},{}]", enc_point3(center), enc_point3(axis), major_radius, minor_radius),
        BrepSurface::Nurbs { control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v } => format!(
            "N[{},{},{},{},{},{},{},{}]",
            enc_list(control_points, enc_point3),
            enc_list(weights, |w: &f64| w.to_string()),
            u_count,
            v_count,
            degree_u,
            degree_v,
            enc_list(knots_u, |k: &f64| k.to_string()),
            enc_list(knots_v, |k: &f64| k.to_string()),
        ),
    }
}
pub(crate) fn dec_surface(s: &str) -> Result<BrepSurface, String> {
    let (tag, rest) = s.split_at(1);
    let inner = strip_brackets(rest)?;
    let parts = split_top_level(inner, ',');
    match tag {
        "P" => {
            let [origin, normal] = parts.as_slice() else { return Err(format!("surface plane: expected 2 fields, got {}", parts.len())) };
            Ok(BrepSurface::Plane { origin: dec_point3(origin)?, normal: dec_point3(normal)? })
        }
        "C" => {
            let [origin, axis, radius] = parts.as_slice() else { return Err(format!("surface cylinder: expected 3 fields, got {}", parts.len())) };
            Ok(BrepSurface::Cylinder { origin: dec_point3(origin)?, axis: dec_point3(axis)?, radius: parse_f64(radius)? })
        }
        "O" => {
            let [origin, axis, radius, half_angle] = parts.as_slice() else { return Err(format!("surface cone: expected 4 fields, got {}", parts.len())) };
            Ok(BrepSurface::Cone { origin: dec_point3(origin)?, axis: dec_point3(axis)?, radius: parse_f64(radius)?, half_angle: parse_f64(half_angle)? })
        }
        "S" => {
            let [center, radius] = parts.as_slice() else { return Err(format!("surface sphere: expected 2 fields, got {}", parts.len())) };
            Ok(BrepSurface::Sphere { center: dec_point3(center)?, radius: parse_f64(radius)? })
        }
        "T" => {
            let [center, axis, major_radius, minor_radius] = parts.as_slice() else { return Err(format!("surface torus: expected 4 fields, got {}", parts.len())) };
            Ok(BrepSurface::Torus { center: dec_point3(center)?, axis: dec_point3(axis)?, major_radius: parse_f64(major_radius)?, minor_radius: parse_f64(minor_radius)? })
        }
        "N" => {
            let [control_points, weights, u_count, v_count, degree_u, degree_v, knots_u, knots_v] = parts.as_slice() else {
                return Err(format!("surface nurbs: expected 8 fields, got {}", parts.len()));
            };
            Ok(BrepSurface::Nurbs {
                control_points: dec_list(control_points, dec_point3)?,
                weights: dec_list(weights, parse_f64)?,
                u_count: parse_u32(u_count)?,
                v_count: parse_u32(v_count)?,
                degree_u: parse_u32(degree_u)?,
                degree_v: parse_u32(degree_v)?,
                knots_u: dec_list(knots_u, parse_f64)?,
                knots_v: dec_list(knots_v, parse_f64)?,
            })
        }
        other => Err(format!("surface: unknown tag {other:?}")),
    }
}

pub(crate) fn enc_loop_edge(le: &BrepLoopEdge) -> String {
    format!("[{},{}]", enc_str(&le.edge), enc_bool(le.orientation))
}
pub(crate) fn dec_loop_edge(s: &str) -> Result<BrepLoopEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [edge, orientation] = parts.as_slice() else { return Err(format!("loop edge: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoopEdge { edge: dec_str(edge)?, orientation: parse_bool(orientation)? })
}

pub(crate) fn enc_shell_face(sf: &BrepShellFace) -> String {
    format!("[{},{}]", enc_str(&sf.face), enc_bool(sf.orientation))
}
pub(crate) fn dec_shell_face(s: &str) -> Result<BrepShellFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [face, orientation] = parts.as_slice() else { return Err(format!("shell face: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShellFace { face: dec_str(face)?, orientation: parse_bool(orientation)? })
}

pub(crate) fn enc_solid_shell(ss: &BrepSolidShell) -> String {
    format!("[{},{}]", enc_str(&ss.shell), enc_bool(ss.is_void))
}
pub(crate) fn dec_solid_shell(s: &str) -> Result<BrepSolidShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [shell, is_void] = parts.as_slice() else { return Err(format!("solid shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepSolidShell { shell: dec_str(shell)?, is_void: parse_bool(is_void)? })
}

pub(crate) fn enc_vertex(v: &BrepVertex) -> String {
    format!("[{},{}]", enc_str(&v.id), enc_point3(&v.point))
}
pub(crate) fn dec_vertex(s: &str) -> Result<BrepVertex, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, point] = parts.as_slice() else { return Err(format!("vertex: expected 2 fields, got {}", parts.len())) };
    Ok(BrepVertex { id: dec_str(id)?, point: dec_point3(point)? })
}

pub(crate) fn enc_edge(e: &BrepEdge) -> String {
    format!("[{},{},{},{}]", enc_str(&e.id), enc_str(&e.start_vertex), enc_str(&e.end_vertex), enc_curve(&e.curve))
}
pub(crate) fn dec_edge(s: &str) -> Result<BrepEdge, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, start_vertex, end_vertex, curve] = parts.as_slice() else { return Err(format!("edge: expected 4 fields, got {}", parts.len())) };
    Ok(BrepEdge { id: dec_str(id)?, start_vertex: dec_str(start_vertex)?, end_vertex: dec_str(end_vertex)?, curve: dec_curve(curve)? })
}

pub(crate) fn enc_loop(l: &BrepLoop) -> String {
    format!("[{},{}]", enc_str(&l.id), enc_list(&l.edges, enc_loop_edge))
}
pub(crate) fn dec_loop(s: &str) -> Result<BrepLoop, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, edges] = parts.as_slice() else { return Err(format!("loop: expected 2 fields, got {}", parts.len())) };
    Ok(BrepLoop { id: dec_str(id)?, edges: dec_list(edges, dec_loop_edge)? })
}

pub(crate) fn enc_face(f: &BrepFace) -> String {
    format!("[{},{},{},{},{}]", enc_str(&f.id), enc_str(&f.outer_loop), enc_list(&f.inner_loops, |s: &String| enc_str(s)), enc_surface(&f.surface), enc_bool(f.orientation),)
}
pub(crate) fn dec_face(s: &str) -> Result<BrepFace, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, outer_loop, inner_loops, surface, orientation] = parts.as_slice() else { return Err(format!("face: expected 5 fields, got {}", parts.len())) };
    Ok(BrepFace { id: dec_str(id)?, outer_loop: dec_str(outer_loop)?, inner_loops: dec_list(inner_loops, dec_str)?, surface: dec_surface(surface)?, orientation: parse_bool(orientation)? })
}

pub(crate) fn enc_shell(sh: &BrepShell) -> String {
    format!("[{},{}]", enc_str(&sh.id), enc_list(&sh.faces, enc_shell_face))
}
pub(crate) fn dec_shell(s: &str) -> Result<BrepShell, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, faces] = parts.as_slice() else { return Err(format!("shell: expected 2 fields, got {}", parts.len())) };
    Ok(BrepShell { id: dec_str(id)?, faces: dec_list(faces, dec_shell_face)? })
}

pub(crate) fn enc_solid(so: &BrepSolid) -> String {
    format!("[{},{}]", enc_str(&so.id), enc_list(&so.shells, enc_solid_shell))
}
pub(crate) fn dec_solid(s: &str) -> Result<BrepSolid, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [id, shells] = parts.as_slice() else { return Err(format!("solid: expected 2 fields, got {}", parts.len())) };
    Ok(BrepSolid { id: dec_str(id)?, shells: dec_list(shells, dec_solid_shell)? })
}
//#endregion 🔖️ValueCodecs

//#region 🔖️DiffValueCodecs
fn enc_vertex_diff(d: &BrepVertexDiff) -> String {
    format!("[{}]", encode_option(&d.point, enc_point3))
}
fn dec_vertex_diff(s: &str) -> Result<BrepVertexDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepVertexDiff { point: decode_option(inner, dec_point3)? })
}

fn enc_edge_diff(d: &BrepEdgeDiff) -> String {
    format!("[{},{},{}]", encode_option(&d.start_vertex, |v: &String| enc_str(v)), encode_option(&d.end_vertex, |v: &String| enc_str(v)), encode_option(&d.curve, enc_curve))
}
fn dec_edge_diff(s: &str) -> Result<BrepEdgeDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [start_vertex, end_vertex, curve] = parts.as_slice() else { return Err(format!("edge diff: expected 3 fields, got {}", parts.len())) };
    Ok(BrepEdgeDiff { start_vertex: decode_option(start_vertex, dec_str)?, end_vertex: decode_option(end_vertex, dec_str)?, curve: decode_option(curve, dec_curve)? })
}

fn enc_loop_diff(d: &BrepLoopDiff) -> String {
    format!("[{}]", encode_option(&d.edges, |v: &Vec<BrepLoopEdge>| enc_list(v, enc_loop_edge)))
}
fn dec_loop_diff(s: &str) -> Result<BrepLoopDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepLoopDiff { edges: decode_option(inner, |s| dec_list(s, dec_loop_edge))? })
}

fn enc_face_diff(d: &BrepFaceDiff) -> String {
    format!(
        "[{},{},{},{}]",
        encode_option(&d.outer_loop, |v: &String| enc_str(v)),
        encode_option(&d.inner_loops, |v: &Vec<String>| enc_list(v, |s: &String| enc_str(s))),
        encode_option(&d.surface, enc_surface),
        encode_option(&d.orientation, |b: &bool| enc_bool(*b).to_string()),
    )
}
fn dec_face_diff(s: &str) -> Result<BrepFaceDiff, String> {
    let parts = split_top_level(strip_brackets(s)?, ',');
    let [outer_loop, inner_loops, surface, orientation] = parts.as_slice() else { return Err(format!("face diff: expected 4 fields, got {}", parts.len())) };
    Ok(BrepFaceDiff { outer_loop: decode_option(outer_loop, dec_str)?, inner_loops: decode_option(inner_loops, |s| dec_list(s, dec_str))?, surface: decode_option(surface, dec_surface)?, orientation: decode_option(orientation, parse_bool)? })
}

fn enc_shell_diff(d: &BrepShellDiff) -> String {
    format!("[{}]", encode_option(&d.faces, |v: &Vec<BrepShellFace>| enc_list(v, enc_shell_face)))
}
fn dec_shell_diff(s: &str) -> Result<BrepShellDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepShellDiff { faces: decode_option(inner, |s| dec_list(s, dec_shell_face))? })
}

fn enc_solid_diff(d: &BrepSolidDiff) -> String {
    format!("[{}]", encode_option(&d.shells, |v: &Vec<BrepSolidShell>| enc_list(v, enc_solid_shell)))
}
fn dec_solid_diff(s: &str) -> Result<BrepSolidDiff, String> {
    let inner = strip_brackets(s)?;
    Ok(BrepSolidDiff { shells: decode_option(inner, |s| dec_list(s, dec_solid_shell))? })
}
//#endregion 🔖️DiffValueCodecs

//#region 🔖️TopLevel
fn print_brep_diff(d: &SemioBrepDiff) -> String {
    let mut tokens: Vec<String> = Vec::new();
    if let Some(v) = &d.vertices {
        tokens.push(format!("vertices={}", enc_named_triple(v, |k: &String| enc_str(k), enc_vertex_diff, enc_vertex)));
    }
    if let Some(v) = &d.edges {
        tokens.push(format!("edges={}", enc_named_triple(v, |k: &String| enc_str(k), enc_edge_diff, enc_edge)));
    }
    if let Some(v) = &d.loops {
        tokens.push(format!("loops={}", enc_named_triple(v, |k: &String| enc_str(k), enc_loop_diff, enc_loop)));
    }
    if let Some(v) = &d.faces {
        tokens.push(format!("faces={}", enc_named_triple(v, |k: &String| enc_str(k), enc_face_diff, enc_face)));
    }
    if let Some(v) = &d.shells {
        tokens.push(format!("shells={}", enc_named_triple(v, |k: &String| enc_str(k), enc_shell_diff, enc_shell)));
    }
    if let Some(v) = &d.solids {
        tokens.push(format!("solids={}", enc_named_triple(v, |k: &String| enc_str(k), enc_solid_diff, enc_solid)));
    }
    tokens.join(" ")
}
fn parse_brep_diff(line: &str) -> Result<SemioBrepDiff, String> {
    let mut d = SemioBrepDiff::default();
    if line.is_empty() {
        return Ok(d);
    }
    for token in line.split(' ') {
        if let Some(rest) = token.strip_prefix("vertices=") {
            d.vertices = Some(dec_named_triple(rest, dec_str, dec_vertex_diff, dec_vertex)?);
        } else if let Some(rest) = token.strip_prefix("edges=") {
            d.edges = Some(dec_named_triple(rest, dec_str, dec_edge_diff, dec_edge)?);
        } else if let Some(rest) = token.strip_prefix("loops=") {
            d.loops = Some(dec_named_triple(rest, dec_str, dec_loop_diff, dec_loop)?);
        } else if let Some(rest) = token.strip_prefix("faces=") {
            d.faces = Some(dec_named_triple(rest, dec_str, dec_face_diff, dec_face)?);
        } else if let Some(rest) = token.strip_prefix("shells=") {
            d.shells = Some(dec_named_triple(rest, dec_str, dec_shell_diff, dec_shell)?);
        } else if let Some(rest) = token.strip_prefix("solids=") {
            d.solids = Some(dec_named_triple(rest, dec_str, dec_solid_diff, dec_solid)?);
        } else {
            return Err(format!("brep diff: unknown token {token:?}"));
        }
    }
    Ok(d)
}

/// 🧪️ Real LEB128-varint-length-prefixed binary primitives (`store::pack_rt::write_varint_u64` /
/// `store::ByteReader`) backing the real `DiffCodec::encode_diff`/`decode_diff` below — replaces
/// the old `print_diff().into_bytes()` text-as-binary shortcut.
fn write_bytes_lp(out: &mut Vec<u8>, bytes: &[u8]) {
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}
fn read_bytes_lp(reader: &mut store::ByteReader<'_>) -> Result<Vec<u8>, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    Ok(reader.read_bytes(len).map_err(|e| e.to_string())?.to_vec())
}
fn write_str_lp(out: &mut Vec<u8>, s: &str) {
    write_bytes_lp(out, s.as_bytes());
}
fn read_str_lp(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    String::from_utf8(read_bytes_lp(reader)?).map_err(|e| e.to_string())
}

impl protocol::DiffCodec for SemioBrepDiff {
    fn print_diff(&self) -> String {
        print_brep_diff(self)
    }
    fn parse_diff(line: &str) -> Result<Self, store::TextError> {
        parse_brep_diff(line).map_err(|e| store::TextError::new(e, dsl::TextSpan::at(1, 1)))
    }
    /// ⚡️ Real binary diff frame, replacing the old `print_diff().into_bytes()` text-as-binary
    /// shortcut. `format u8` + `presence u8` (bit0=`vertices`, bit1=`edges`, bit2=`loops`,
    /// bit3=`faces`, bit4=`shells`, bit5=`solids`) are two REAL fixed fields; each present
    /// collection then follows as its own varint-length-prefixed opaque blob (the same
    /// `enc_*_diff` bracket/hex text this type's `print_diff` already produces) — independently-
    /// delimited segments rather than one bare trailing `bytes` because there can be 0-6 of them
    /// (chaining a `Cond` per-segment hits the `protocol-cond-cannot-chain` gap: a second
    /// `if`-guard on a field that was itself only conditionally decoded hard-errors `eval_cond`).
    fn encode_diff(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        let mut presence = 0u8;
        if self.vertices.is_some() {
            presence |= 0b0000_0001;
        }
        if self.edges.is_some() {
            presence |= 0b0000_0010;
        }
        if self.loops.is_some() {
            presence |= 0b0000_0100;
        }
        if self.faces.is_some() {
            presence |= 0b0000_1000;
        }
        if self.shells.is_some() {
            presence |= 0b0001_0000;
        }
        if self.solids.is_some() {
            presence |= 0b0010_0000;
        }
        let mut out = vec![DIFF_BINARY_FORMAT, presence];
        if let Some(v) = &self.vertices {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_vertex_diff, enc_vertex));
        }
        if let Some(v) = &self.edges {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_edge_diff, enc_edge));
        }
        if let Some(v) = &self.loops {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_loop_diff, enc_loop));
        }
        if let Some(v) = &self.faces {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_face_diff, enc_face));
        }
        if let Some(v) = &self.shells {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_shell_diff, enc_shell));
        }
        if let Some(v) = &self.solids {
            write_str_lp(&mut out, &enc_named_triple(v, |k: &String| enc_str(k), enc_solid_diff, enc_solid));
        }
        Ok(out)
    }
    fn decode_diff(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const DIFF_BINARY_FORMAT: u8 = 1;
        if bytes.len() < 2 {
            return Err(protocol::ProtocolError::Malformed { what: "diff header", offset: 0, detail: "truncated (need format+presence)".to_string() });
        }
        if bytes[0] != DIFF_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "diff format", offset: 0, detail: format!("unsupported diff format {}", bytes[0]) });
        }
        let presence = bytes[1];
        let mut reader = store::ByteReader::new(&bytes[2..]);
        let mut next_blob = |what: &'static str| -> Result<String, protocol::ProtocolError> { read_str_lp(&mut reader).map_err(|e| protocol::ProtocolError::Malformed { what, offset: 2, detail: e }) };
        let vertices = if presence & 0b0000_0001 != 0 {
            Some(dec_named_triple(&next_blob("diff vertices blob")?, dec_str, dec_vertex_diff, dec_vertex).map_err(|e| protocol::ProtocolError::Malformed { what: "diff vertices text", offset: 2, detail: e })?)
        } else {
            None
        };
        let edges =
            if presence & 0b0000_0010 != 0 { Some(dec_named_triple(&next_blob("diff edges blob")?, dec_str, dec_edge_diff, dec_edge).map_err(|e| protocol::ProtocolError::Malformed { what: "diff edges text", offset: 2, detail: e })?) } else { None };
        let loops =
            if presence & 0b0000_0100 != 0 { Some(dec_named_triple(&next_blob("diff loops blob")?, dec_str, dec_loop_diff, dec_loop).map_err(|e| protocol::ProtocolError::Malformed { what: "diff loops text", offset: 2, detail: e })?) } else { None };
        let faces =
            if presence & 0b0000_1000 != 0 { Some(dec_named_triple(&next_blob("diff faces blob")?, dec_str, dec_face_diff, dec_face).map_err(|e| protocol::ProtocolError::Malformed { what: "diff faces text", offset: 2, detail: e })?) } else { None };
        let shells = if presence & 0b0001_0000 != 0 {
            Some(dec_named_triple(&next_blob("diff shells blob")?, dec_str, dec_shell_diff, dec_shell).map_err(|e| protocol::ProtocolError::Malformed { what: "diff shells text", offset: 2, detail: e })?)
        } else {
            None
        };
        let solids = if presence & 0b0010_0000 != 0 {
            Some(dec_named_triple(&next_blob("diff solids blob")?, dec_str, dec_solid_diff, dec_solid).map_err(|e| protocol::ProtocolError::Malformed { what: "diff solids text", offset: 2, detail: e })?)
        } else {
            None
        };
        Ok(SemioBrepDiff { vertices, edges, loops, faces, shells, solids })
    }
}
//#endregion 🔖️TopLevel
//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioBrepDiff` cases (empty/no-op, a full removed/modified/added sweep both
/// directions across every collection, plus a bare insert) — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🎹️composer/🦀️component.rs`. Self-
/// contained (does not reach into `#[cfg(test)] mod tests`'s own private `sweep_a`/`sweep_b`,
/// since a private item of a child module is not visible to its parent).
#[cfg(test)]
pub(crate) fn demo_diff_cases() -> Vec<SemioBrepDiff> {
    let mut a = SemioBrepSnapshot::default();
    a.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }, BrepVertex { id: "v-removed".into(), point: SemioPoint3::default() }];
    a.edges = vec![BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } }];
    a.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }];
    a.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true }];
    a.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }];
    a.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }];

    let mut b = SemioBrepSnapshot::default();
    b.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 } }, BrepVertex { id: "v-added".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }];
    b.edges = vec![BrepEdge {
        id: "e1".into(),
        start_vertex: "v1".into(),
        end_vertex: "v-added".into(),
        curve: BrepCurve::Nurbs { control_points: vec![SemioPoint3::default(), SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }], weights: vec![1.0, 1.0], degree: 1, knots: vec![0.0, 0.0, 1.0, 1.0] },
    }];
    b.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] }];
    b.faces = vec![BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec!["l1".into()], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 2.0 }, orientation: false }];
    b.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: false }] }];
    b.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: true }] }, BrepSolid { id: "so-added".into(), shells: vec![] }];

    vec![SemioBrepDiff::default(), <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&a, &b), <SemioBrepDiff as DiffAlgebra<SemioBrepSnapshot>>::between(&b, &a)]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region 🔖️Fixtures
    /// 🧱️ Every collection carries: one "keep" item touched in EVERY sub-field, one item present
    /// only in `sweep_a` (removed), and (in `sweep_b`) one item present only there (added).
    fn sweep_a() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 0.0, y: 0.0, z: 0.0 } }, BrepVertex { id: "v-removed".into(), point: SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 } }];
        s.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "v1".into(), end_vertex: "v1".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 1.0, y: 0.0, z: 0.0 } } },
            BrepEdge { id: "e-removed".into(), start_vertex: "v-removed".into(), end_vertex: "v-removed".into(), curve: BrepCurve::Circle { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 1.0 } },
        ];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: true }] }, BrepLoop { id: "l-removed".into(), edges: vec![] }];
        s.faces = vec![
            BrepFace { id: "f1".into(), outer_loop: "l1".into(), inner_loops: vec![], surface: BrepSurface::Plane { origin: SemioPoint3::default(), normal: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 } }, orientation: true },
            BrepFace { id: "f-removed".into(), outer_loop: "l-removed".into(), inner_loops: vec![], surface: BrepSurface::Sphere { center: SemioPoint3::default(), radius: 1.0 }, orientation: true },
        ];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: true }] }, BrepShell { id: "s-removed".into(), faces: vec![] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: false }] }, BrepSolid { id: "so-removed".into(), shells: vec![] }];
        s
    }

    fn sweep_b() -> SemioBrepSnapshot {
        let mut s = SemioBrepSnapshot::default();
        s.vertices = vec![BrepVertex { id: "v1".into(), point: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 } }, BrepVertex { id: "v-added".into(), point: SemioPoint3 { x: 2.0, y: 2.0, z: 2.0 } }];
        s.edges = vec![
            BrepEdge { id: "e1".into(), start_vertex: "v-added".into(), end_vertex: "v-added".into(), curve: BrepCurve::Circle { center: SemioPoint3 { x: 1.0, y: 1.0, z: 1.0 }, axis: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 }, radius: 2.0 } },
            BrepEdge { id: "e-added".into(), start_vertex: "v-added".into(), end_vertex: "v-added".into(), curve: BrepCurve::Line { origin: SemioPoint3::default(), direction: SemioPoint3 { x: 0.0, y: 1.0, z: 0.0 } } },
        ];
        s.loops = vec![BrepLoop { id: "l1".into(), edges: vec![BrepLoopEdge { edge: "e1".into(), orientation: false }] }, BrepLoop { id: "l-added".into(), edges: vec![] }];
        s.faces = vec![
            BrepFace {
                id: "f1".into(),
                outer_loop: "l1-alt".into(),
                inner_loops: vec!["l-added".into()],
                surface: BrepSurface::Cylinder { origin: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, radius: 5.0 },
                orientation: false,
            },
            BrepFace {
                id: "f-added".into(),
                outer_loop: "l1".into(),
                inner_loops: vec![],
                surface: BrepSurface::Torus { center: SemioPoint3::default(), axis: SemioPoint3 { x: 0.0, y: 0.0, z: 1.0 }, major_radius: 3.0, minor_radius: 1.0 },
                orientation: true,
            },
        ];
        s.shells = vec![BrepShell { id: "s1".into(), faces: vec![BrepShellFace { face: "f1".into(), orientation: false }] }, BrepShell { id: "s-added".into(), faces: vec![] }];
        s.solids = vec![BrepSolid { id: "so1".into(), shells: vec![BrepSolidShell { shell: "s1".into(), is_void: true }] }, BrepSolid { id: "so-added".into(), shells: vec![] }];
        s
    }
    //#endregion 🔖️Fixtures

    //#region 🔖️between_roundtrip_law
    #[test]
    fn between_roundtrip_law_and_field_sweep_both_directions() {
        let (a, b) = (sweep_a(), sweep_b());
        let d_ab = SemioBrepDiff::between(&a, &b);
        assert_eq!(d_ab.apply(&a).expect("apply must succeed for a well-formed fixture"), b);
        let d_ba = SemioBrepDiff::between(&b, &a);
        assert_eq!(d_ba.apply(&b).expect("apply must succeed for a well-formed fixture"), a);
        assert!(SemioBrepDiff::between(&a, &a).is_empty());
    }
    //#endregion 🔖️between_roundtrip_law

    //#region 🔖️field_sweep
    #[test]
    fn field_sweep_every_field_present_in_diff() {
        let (a, b) = (sweep_a(), sweep_b());
        let d = SemioBrepDiff::between(&a, &b);

        let vertices = d.vertices.as_ref().expect("vertices diff present");
        assert_eq!(vertices.removed, vec!["v-removed".to_string()]);
        assert_eq!(vertices.added.iter().map(|v| v.id.clone()).collect::<Vec<_>>(), vec!["v-added".to_string()]);
        assert!(vertices.modified.iter().any(|m| m.key == "v1" && m.diff.point.is_some()));

        let edges = d.edges.as_ref().expect("edges diff present");
        assert_eq!(edges.removed, vec!["e-removed".to_string()]);
        assert_eq!(edges.added.iter().map(|e| e.id.clone()).collect::<Vec<_>>(), vec!["e-added".to_string()]);
        let e1 = edges.modified.iter().find(|m| m.key == "e1").expect("e1 modified");
        assert!(e1.diff.start_vertex.is_some() && e1.diff.end_vertex.is_some() && e1.diff.curve.is_some());

        let loops = d.loops.as_ref().expect("loops diff present");
        assert_eq!(loops.removed, vec!["l-removed".to_string()]);
        assert_eq!(loops.added.iter().map(|l| l.id.clone()).collect::<Vec<_>>(), vec!["l-added".to_string()]);
        assert!(loops.modified.iter().any(|m| m.key == "l1" && m.diff.edges.is_some()));

        let faces = d.faces.as_ref().expect("faces diff present");
        assert_eq!(faces.removed, vec!["f-removed".to_string()]);
        assert_eq!(faces.added.iter().map(|f| f.id.clone()).collect::<Vec<_>>(), vec!["f-added".to_string()]);
        let f1 = faces.modified.iter().find(|m| m.key == "f1").expect("f1 modified");
        assert!(f1.diff.outer_loop.is_some() && f1.diff.inner_loops.is_some() && f1.diff.surface.is_some() && f1.diff.orientation.is_some());

        let shells = d.shells.as_ref().expect("shells diff present");
        assert_eq!(shells.removed, vec!["s-removed".to_string()]);
        assert_eq!(shells.added.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["s-added".to_string()]);
        assert!(shells.modified.iter().any(|m| m.key == "s1" && m.diff.faces.is_some()));

        let solids = d.solids.as_ref().expect("solids diff present");
        assert_eq!(solids.removed, vec!["so-removed".to_string()]);
        assert_eq!(solids.added.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), vec!["so-added".to_string()]);
        assert!(solids.modified.iter().any(|m| m.key == "so1" && m.diff.shells.is_some()));
    }
    //#endregion 🔖️field_sweep

    //#region 🔖️inverse_law
    #[test]
    fn inverse_law_diff_level_round_trips() {
        let (a, b) = (sweep_a(), sweep_b());
        let d = SemioBrepDiff::between(&a, &b);
        let inv = d.inverse(&a);
        assert_eq!(inv.apply(&d.apply(&a).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"), a);
    }
    //#endregion 🔖️inverse_law

    //#region 🔖️absorb_law
    #[test]
    fn absorb_law_add_then_remove_of_same_added_key_cancels() {
        let base = SemioBrepSnapshot::default();
        let mut d1 = SemioBrepDiff::default();
        d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { id: "v-new".into(), point: SemioPoint3 { x: 1.0, y: 2.0, z: 3.0 } }] });
        let mut d2 = SemioBrepDiff::default();
        d2.vertices = Some(BrepVerticesDiff { removed: vec!["v-new".into()], modified: vec![], added: vec![] });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        d1.absorb(d2);
        assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), sequential);
        assert_eq!(d1.apply(&base).expect("apply must succeed for a well-formed fixture"), base, "add-then-remove-of-same-key must net to a no-op");
    }

    #[test]
    fn absorb_law_add_then_setfield_patches_added_payload() {
        let base = SemioBrepSnapshot::default();
        let mut d1 = SemioBrepDiff::default();
        d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![], added: vec![BrepVertex { id: "v-new".into(), point: SemioPoint3::default() }] });
        let mut d2 = SemioBrepDiff::default();
        d2.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![NamedModified { key: "v-new".into(), diff: BrepVertexDiff { point: Some(SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 }) } }], added: vec![] });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        d1.absorb(d2);
        let result = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
        assert_eq!(result, sequential);
        assert_eq!(result.vertices[0].point, SemioPoint3 { x: 5.0, y: 5.0, z: 5.0 });
    }

    #[test]
    fn absorb_law_modify_then_remove_drops_pending_patch() {
        let mut base = SemioBrepSnapshot::default();
        base.vertices.push(BrepVertex { id: "v1".into(), point: SemioPoint3::default() });
        let mut d1 = SemioBrepDiff::default();
        d1.vertices = Some(BrepVerticesDiff { removed: vec![], modified: vec![NamedModified { key: "v1".into(), diff: BrepVertexDiff { point: Some(SemioPoint3 { x: 9.0, y: 9.0, z: 9.0 }) } }], added: vec![] });
        let mut d2 = SemioBrepDiff::default();
        d2.vertices = Some(BrepVerticesDiff { removed: vec!["v1".into()], modified: vec![], added: vec![] });
        let sequential = d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture");
        d1.absorb(d2);
        let result = d1.apply(&base).expect("apply must succeed for a well-formed fixture");
        assert_eq!(result, sequential);
        assert!(result.vertices.is_empty());
    }

    #[test]
    fn absorb_law_associativity() {
        let base = sweep_a();
        let mid = sweep_b();
        let mut after = sweep_b();
        after.vertices.push(BrepVertex { id: "v-extra".into(), point: SemioPoint3 { x: 7.0, y: 8.0, z: 9.0 } });
        let d1 = SemioBrepDiff::between(&base, &mid);
        let d2 = SemioBrepDiff::between(&mid, &after);
        let mut absorbed = d1.clone();
        absorbed.absorb(d2.clone());
        assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), after);
        assert_eq!(absorbed.apply(&base).expect("apply must succeed for a well-formed fixture"), d2.apply(&d1.apply(&base).expect("apply must succeed for a well-formed fixture")).expect("apply must succeed for a well-formed fixture"));
    }
    //#endregion 🔖️absorb_law

    //#region 🔖️diff_codec_text_binary_roundtrip_law
    #[test]
    fn diff_codec_text_binary_roundtrip_law() {
        use protocol::DiffCodec;
        let (a, b) = (sweep_a(), sweep_b());
        let cases = vec![SemioBrepDiff::default(), SemioBrepDiff::between(&a, &b), SemioBrepDiff::between(&b, &a)];
        for d in cases {
            let printed = d.print_diff();
            assert!(!printed.contains('\n'), "print_diff must be one line, got {printed:?}");
            let parsed = SemioBrepDiff::parse_diff(&printed).unwrap_or_else(|e| panic!("parse_diff({printed:?}) failed: {e}"));
            assert_eq!(parsed, d, "print_diff/parse_diff round-trip mismatch (printed {printed:?})");

            let encoded = d.encode_diff().unwrap_or_else(|e| panic!("encode_diff failed: {e}"));
            let decoded = SemioBrepDiff::decode_diff(&encoded).unwrap_or_else(|e| panic!("decode_diff failed: {e}"));
            assert_eq!(decoded, d, "encode_diff/decode_diff round-trip mismatch");
        }
    }
    //#endregion 🔖️diff_codec_text_binary_roundtrip_law
}
//#endregion 🔖️Tests
