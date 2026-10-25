//! 🪪️ Stable element identity: the keyed-reconciliation vocabulary ([`ReconciliationKey`], [`ElementId`]),
//! the window-space [`Bounds`] rect and the geometry-only [`Hitbox`].
//!
//! Lives below [`crate::dispatch`] on purpose: the dispatch tree indexes hit-testable regions by
//! [`ElementId`] and [`Hitbox`], while the element model above it registers into the dispatch tree
//! during prepaint. Putting the identity vocabulary in the lower crate is what keeps that dependency
//! pointing one way.
//!
//! **`ElementId` stability:** `ElementId::new(parent, key)` folds the parent's own id together with a
//! [`ReconciliationKey`] through a hand-implemented FxHash (the same small, well-known rotate-xor-multiply
//! hash `rustc-hash` uses) — no crate dependency added. Two calls with equal `(parent, key)` always
//! produce the same id, which is exactly what lets retained per-id state (hover, scroll offset,
//! animation clocks) survive an element tree rebuilt from scratch every frame.

//#region 🔖️Identity

/// 🔑️ Stable child identity for keyed reconciliation, mirroring the wgpu-old target's
/// `tree::NodeKey`: an explicit key when the node declares one, else a `(discriminant, ordinal)`
/// positional fallback among same-kind siblings. A `Positional` element's identity does not survive a
/// reorder — only `Explicit` keys do — which is inherent to a positional fallback, not a bug here.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ReconciliationKey {
    Explicit(String),
    Positional(u32, u32),
}

const FXHASH_SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;

/// 🌀️ One FxHash mixing step: rotate-xor-multiply, the same primitive `rustc-hash` uses for every
/// machine word it ingests.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn fxhash_word(hash: u64, word: u64) -> u64 {
    (hash.rotate_left(5) ^ word).wrapping_mul(FXHASH_SEED)
}

/// 🧮️ Folds `bytes` into `hash` eight bytes at a time (the tail chunk is zero-padded), matching
/// `rustc-hash`'s own byte-slice handling closely enough for this crate's purpose: a fast, stable,
/// dependency-free identity hash — not a cryptographic or DoS-resistant one.
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn fxhash_bytes(hash: u64, bytes: &[u8]) -> u64 {
    let mut hash = hash;
    for chunk in bytes.chunks(8) {
        let mut word_bytes = [0u8; 8];
        word_bytes[..chunk.len()].copy_from_slice(chunk);
        hash = fxhash_word(hash, u64::from_le_bytes(word_bytes));
    }
    hash
}

/// 🧱️ A per-frame-rebuilt element's stable identity: `fxhash(parent ElementId, reconciliation key)`.
/// Deliberately a **separate identity domain** from [`ui_contract::UiNodeId`] (a protocol identity for
/// one row of a [`ui_contract::UiSnapshot`]'s flat node table) and from any entity id the headless
/// runtime mints — the three never alias each other, even when, as is common, one `UiNodeRecord`
/// happens to lower into exactly one `Element`. Stability across a from-scratch rebuild (never a
/// database key, never an index) is what lets hover state, scroll offsets, edit buffers and animation
/// clocks in `RetainedStore` survive frame to frame.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ElementId(u64);

impl ElementId {
    /// 🌱️ Combines an optional parent id with `key`. `parent: None` seeds the hash the same way a
    /// child would seed it from a parent id of `FXHASH_SEED` — there is no reserved all-zero root
    /// sentinel, so a root element's id is exactly as well-distributed as any other.
    pub fn new(parent: Option<ElementId>, key: &ReconciliationKey) -> Self {
        let seed = parent.map_or(FXHASH_SEED, |id| id.0);
        let hash = match key {
            ReconciliationKey::Explicit(text) => fxhash_bytes(seed, text.as_bytes()),
            ReconciliationKey::Positional(discriminant, ordinal) => fxhash_word(fxhash_word(seed, u64::from(*discriminant)), u64::from(*ordinal)),
        };
        Self(hash)
    }
}

//#endregion 🔖️Identity

//#region 🔖️Bounds

/// 📐️ A resolved, absolute (window-space) rect in logical pixels — the same type
/// [`semio_framework_ui_render_packet::LayoutRect`] already is, reused here rather than duplicated,
/// since `prepaint`/`paint` bounds and scene-primitive rects are exactly the same kind of value.
pub type Bounds = semio_framework_ui_render_packet::LayoutRect;

/// 🎯️ One hit-testable region's geometry, passed to `PrepaintCx::register` alongside the flags and
/// listeners [`crate::DispatchTree`] actually needs to interpret it (parent link, overlay bit,
/// bindings) — `Hitbox` itself stays geometry-only and is what [`crate::DispatchTree`] indexes by
/// [`crate::HitboxId`] for spatial lookups (`crate::DispatchTree::hitboxes`/`bounds`), never the
/// source of tree structure.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hitbox {
    pub element: ElementId,
    pub bounds: Bounds,
    pub clips_children: bool,
    pub hit_transparent: bool,
}

//#endregion 🔖️Bounds
