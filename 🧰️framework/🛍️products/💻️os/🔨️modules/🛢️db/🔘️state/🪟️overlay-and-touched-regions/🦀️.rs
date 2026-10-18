//! 🪟️ Overlay and touched regions: the base source and overlay root plus the touched region set. See [`super`].

use super::*;

//#region 🔖️Overlay
/// 🫧️ The read side of a document's immutable base — typically a lazily-decoded `pack`
/// document. `db_artifact` implements this over the real pack reader; `db_state` only depends on
/// the trait, keeping this crate `pack`-decoder-free (it depends on `pack_core`, not `pack`).
pub trait BaseSource {
    fn load(&self, path: &str) -> Result<Option<Vec<u8>>, DbError>;
}

/// 🫙️ A `BaseSource` with nothing in it — the base for a brand-new document, and useful in
/// tests that only care about overlay behavior.
pub struct EmptyBase;

impl BaseSource for EmptyBase {
    fn load(&self, _path: &str) -> Result<Option<Vec<u8>>, DbError> {
        Ok(None)
    }
}

/// ✏️ What an overlay records at a path: either an explicit value that shadows the base,
/// or an explicit tombstone that hides a base value without touching the base itself.
#[derive(Clone)]
enum OverlayValue {
    Set(Vec<u8>),
    Deleted,
}

/// 🏗️ A document's live, mutable-by-replacement state: an immutable base (read lazily,
/// never written) plus a `PMap` overlay of edits on top. Reads fall through overlay → base, per
/// the contract; every mutation returns a new `OverlayRoot` (the base `Rc` is shared, only the
/// overlay `PMap` grows/shrinks) alongside the `TouchedRegion` it touched, for `db_conflict`.
pub struct OverlayRoot<B: BaseSource> {
    pub base: Rc<B>,
    overlay: PMap<String, OverlayValue>,
}

impl<B: BaseSource> OverlayRoot<B> {
    pub fn new(base: B) -> Self {
        OverlayRoot { base: Rc::new(base), overlay: PMap::new() }
    }

    pub fn get(&self, path: &str) -> Result<Option<Vec<u8>>, DbError> {
        match self.overlay.get(&path.to_string()) {
            Some(OverlayValue::Set(bytes)) => Ok(Some(bytes.clone())),
            Some(OverlayValue::Deleted) => Ok(None),
            None => self.base.load(path),
        }
    }

    pub fn set(&self, path: &str, value: Vec<u8>) -> (Self, TouchedRegion) {
        let overlay = self.overlay.insert(path.to_string(), OverlayValue::Set(value));
        (OverlayRoot { base: self.base.clone(), overlay }, TouchedRegion::write(path))
    }

    pub fn delete(&self, path: &str) -> (Self, TouchedRegion) {
        let overlay = self.overlay.insert(path.to_string(), OverlayValue::Deleted);
        (OverlayRoot { base: self.base.clone(), overlay }, TouchedRegion::write(path))
    }

    /// 🔢️ How many paths the overlay has explicitly recorded (set or tombstoned) — every
    /// other path still falls through to `base` untouched.
    pub fn overlay_len(&self) -> usize {
        self.overlay.len()
    }
}

impl<B: BaseSource> Clone for OverlayRoot<B> {
    fn clone(&self) -> Self {
        OverlayRoot { base: self.base.clone(), overlay: self.overlay.clone() }
    }
}
//#endregion 🔖️Overlay

//#region 🔖️TouchedRegion
/// 👣️ Whether a `TouchedRegion` records a read or a write — two reads of the same region
/// never conflict; a write against anything intersecting it does.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TouchKind {
    Read,
    Write,
}

/// 🗺️ A single access against one path in an overlay — `db_conflict`'s primitive unit for
/// touched-region intersection (bloom filters and the command-kind matrix build on top of this,
/// they are `db_conflict`'s own concern). `path` is a `/`-separated segment string (matching
/// `OverlayRoot`'s path shape), so a coarse write to a container path is detected as conflicting
/// with a narrower access nested beneath it.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TouchedRegion {
    pub path: String,
    pub kind: TouchKind,
}

impl TouchedRegion {
    pub fn read(path: impl Into<String>) -> Self {
        TouchedRegion { path: path.into(), kind: TouchKind::Read }
    }

    pub fn write(path: impl Into<String>) -> Self {
        TouchedRegion { path: path.into(), kind: TouchKind::Write }
    }

    /// 🔀️ True iff `self` and `other` name the same path, or one path is a `/`-boundary
    /// prefix of the other (so a whole-subtree write is treated as touching everything beneath
    /// it, and vice versa).
    pub fn path_intersects(&self, other: &TouchedRegion) -> bool {
        path_is_prefix(&self.path, &other.path) || path_is_prefix(&other.path, &self.path)
    }
}

fn path_is_prefix(prefix: &str, path: &str) -> bool {
    if prefix == path {
        return true;
    }
    path.len() > prefix.len() && path.starts_with(prefix) && path.as_bytes()[prefix.len()] == b'/'
}

/// 🧾️ The accumulated reads/writes of one command/transaction against an `OverlayRoot` —
/// what `db_conflict` intersects two of (from concurrent commands against the same base frontier)
/// to decide whether they conflict.
#[derive(Clone, Default, Debug)]
pub struct TouchedSet {
    pub regions: Vec<TouchedRegion>,
}

impl TouchedSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, region: TouchedRegion) {
        self.regions.push(region);
    }

    /// ⚔️ True iff any region in `self` and any region in `other` intersect with at least
    /// one side being a `Write` — read/read intersections never conflict.
    pub fn conflicts_with(&self, other: &TouchedSet) -> bool {
        self.regions.iter().any(|a| other.regions.iter().any(|b| (a.kind == TouchKind::Write || b.kind == TouchKind::Write) && a.path_intersects(b)))
    }
}
//#endregion 🔖️TouchedRegion
