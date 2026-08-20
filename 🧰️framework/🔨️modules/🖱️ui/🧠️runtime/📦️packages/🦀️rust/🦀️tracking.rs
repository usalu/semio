//! @emoji 🕸️ Actual-read dependency tracking — which entities a surface truly read while presenting.
//!
//! This is *actual*-read tracking, not declared-dependency tracking: a presenter never lists what it
//! depends on. Instead, [`DependencyTracker::begin`] opens a scope, every [`crate::PresentCx::read`]
//! call during that scope records the entity actually touched, and [`DependencyTracker::finish`]
//! replaces the surface's whole read set wholesale with exactly what was recorded. A read the
//! presenter no longer performs is therefore simply absent from the next set — the stale edge in the
//! inverse index disappears on its own, with no bookkeeping by the presenter's author and no explicit
//! unsubscribe call anywhere.
//!
//! Reads only count while a scope is open. [`crate::PresentCx::read`] is the only public entry point
//! that reaches [`DependencyTracker::record_read`], and it only exists during presentation — an event
//! handler reading the same entity through [`crate::Entity::read`] directly never touches this type at
//! all, so its reads structurally cannot become a frame dependency. [`DependencyTracker::record_read`]
//! is additionally a no-op with no scope open, as a second line of defense against a caller that ends
//! up holding a tracker reference outside presentation.
//!
//! 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md. Every `fn`
//! below is plain sync by owner ruling U1, which supersedes this program's general async-everything
//! default for exactly this crate.

use std::collections::{HashMap, HashSet};

//#region 🔖️Tracking

//#region 🆔️EntityId
/// 🆔️ An opaque, stable identity for one entity in the (packet `runtime-entity`) generational store —
/// defined here, not in `🦀️entity.rs`, because [`DependencyTracker`] is the type that actually needs a
/// hashable entity identity and `runtime-entity`'s `Entity<T>`/`WeakEntity<T>` are handles, not ids.
/// `Entity::<T>::id` is expected to return this type directly (a generational `(index, generation)`
/// pair packs trivially into the one `u64`) rather than `runtime-entity` minting a second, differently
/// named identity — see this packet's report for why a duplicate definition here would be worse than
/// this forward reference.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct EntityId(pub u64);
//#endregion 🆔️EntityId

//#region 🕸️DependencyTracker
/// 🕸️ Surface → the set of entities its most recent present actually read, plus the inverse index
/// (entity → the surfaces currently reading it) that makes waking a surface from an entity
/// notification O(surfaces reading that entity) rather than a linear scan of every live surface.
#[derive(Debug, Default)]
pub struct DependencyTracker {
    surface_reads: HashMap<ui_contract::SurfaceId, HashSet<EntityId>>,
    entity_surfaces: HashMap<EntityId, HashSet<ui_contract::SurfaceId>>,
    scopes: Vec<(ui_contract::SurfaceId, HashSet<EntityId>)>,
    dirty: HashSet<ui_contract::SurfaceId>,
}

impl DependencyTracker {
    /// 🎬️ Opens a new actual-read recording scope for `surface`, pushed onto a nesting stack — a
    /// present that itself drives a nested present's `begin`/`finish` pair keeps accruing its own
    /// reads to its own scope once the inner one closes, so nested presents nest correctly.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn begin(&mut self, surface: ui_contract::SurfaceId) {
        self.scopes.push((surface, HashSet::new()));
    }

    /// 👁️ Records that the innermost open scope actually read `entity`. A deliberate no-op with no
    /// scope open — see the module doc for why that is exactly what keeps an event handler's reads
    /// from ever becoming a frame dependency.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn record_read(&mut self, entity: EntityId) {
        if let Some((_, reads)) = self.scopes.last_mut() {
            reads.insert(entity);
        }
    }

    /// 🏁️ Closes the innermost scope — asserted to belong to `surface`, since scopes must close in
    /// the reverse order they opened — and replaces that surface's stored read set wholesale with
    /// exactly what was recorded since the matching `begin`, updating the inverse index to match.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn finish(&mut self, surface: ui_contract::SurfaceId) {
        let (scope_surface, reads) = self.scopes.pop().expect("finish called with no open scope");
        assert_eq!(scope_surface, surface, "present scope nesting mismatch");
        if let Some(previous) = self.surface_reads.remove(&surface) {
            for entity in previous {
                if let Some(surfaces) = self.entity_surfaces.get_mut(&entity) {
                    surfaces.remove(&surface);
                    if surfaces.is_empty() {
                        self.entity_surfaces.remove(&entity);
                    }
                }
            }
        }
        for &entity in &reads {
            self.entity_surfaces.entry(entity).or_default().insert(surface.clone());
        }
        self.surface_reads.insert(surface, reads);
    }

    /// 🔎️ Every surface whose most recent present actually read `entity`, via the inverse index —
    /// the lookup this whole structure exists to make cheap instead of a scan of every live surface.
    pub fn dirty_surfaces_for(&self, entity: EntityId) -> impl Iterator<Item = ui_contract::SurfaceId> + '_ {
        self.entity_surfaces.get(&entity).into_iter().flatten().cloned()
    }

    /// 🔔️ Marks every surface currently reading `entity` dirty for the transaction in progress.
    /// Backed by a `HashSet`, so any number of calls — for the same entity repeatedly, or for
    /// several entities read by overlapping surfaces — coalesce to at most one dirty mark per
    /// surface: a bulk projection update still produces exactly one frame, not N.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn notify_entity(&mut self, entity: EntityId) {
        let surfaces: Vec<_> = self.dirty_surfaces_for(entity).collect();
        self.dirty.extend(surfaces);
    }

    /// 🧹️ Drains the coalesced dirty set. Call once per transaction, after every notification for
    /// that transaction has been recorded and before presenting, so each dirty surface presents
    /// exactly once this frame.
    pub fn drain_dirty(&mut self) -> impl Iterator<Item = ui_contract::SurfaceId> + '_ {
        self.dirty.drain()
    }
}
//#endregion 🕸️DependencyTracker

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn presenter_reading_a_not_b_wakes_only_on_a() {
        let mut tracker = DependencyTracker::default();
        let surface = ui_contract::SurfaceId::from("s");
        let a = EntityId(1);
        let b = EntityId(2);
        tracker.begin(surface.clone());
        tracker.record_read(a);
        tracker.finish(surface.clone());

        tracker.notify_entity(a);
        assert_eq!(tracker.drain_dirty().collect::<Vec<_>>(), vec![surface.clone()]);

        tracker.notify_entity(b);
        assert_eq!(tracker.drain_dirty().count(), 0);
    }

    #[test]
    fn stale_edge_disappears_after_next_present_without_the_read() {
        let mut tracker = DependencyTracker::default();
        let surface = ui_contract::SurfaceId::from("s");
        let a = EntityId(1);
        tracker.begin(surface.clone());
        tracker.record_read(a);
        tracker.finish(surface.clone());
        assert_eq!(tracker.dirty_surfaces_for(a).collect::<Vec<_>>(), vec![surface.clone()]);

        tracker.begin(surface.clone());
        tracker.finish(surface.clone());
        assert_eq!(tracker.dirty_surfaces_for(a).count(), 0);
    }

    #[test]
    fn n_notifications_of_one_surface_coalesce_to_one_dirty_mark() {
        let mut tracker = DependencyTracker::default();
        let surface = ui_contract::SurfaceId::from("s");
        let a = EntityId(1);
        let b = EntityId(2);
        tracker.begin(surface.clone());
        tracker.record_read(a);
        tracker.record_read(b);
        tracker.finish(surface.clone());

        tracker.notify_entity(a);
        tracker.notify_entity(b);
        tracker.notify_entity(a);

        assert_eq!(tracker.drain_dirty().collect::<Vec<_>>(), vec![surface]);
    }

    #[test]
    fn nested_present_scopes_attribute_reads_to_the_right_surface() {
        let mut tracker = DependencyTracker::default();
        let outer = ui_contract::SurfaceId::from("outer");
        let inner = ui_contract::SurfaceId::from("inner");
        let x = EntityId(1);
        let y = EntityId(2);
        let z = EntityId(3);

        tracker.begin(outer.clone());
        tracker.record_read(x);
        tracker.begin(inner.clone());
        tracker.record_read(y);
        tracker.finish(inner.clone());
        tracker.record_read(z);
        tracker.finish(outer.clone());

        assert_eq!(tracker.dirty_surfaces_for(x).collect::<Vec<_>>(), vec![outer.clone()]);
        assert_eq!(tracker.dirty_surfaces_for(y).collect::<Vec<_>>(), vec![inner]);
        assert_eq!(tracker.dirty_surfaces_for(z).collect::<Vec<_>>(), vec![outer]);
    }

    #[test]
    fn reads_outside_a_present_scope_are_not_recorded() {
        let mut tracker = DependencyTracker::default();
        let a = EntityId(1);
        tracker.record_read(a);
        assert_eq!(tracker.dirty_surfaces_for(a).count(), 0);
    }

    #[test]
    #[should_panic(expected = "present scope nesting mismatch")]
    fn finish_rejects_mismatched_surface() {
        let mut tracker = DependencyTracker::default();
        tracker.begin(ui_contract::SurfaceId::from("a"));
        tracker.finish(ui_contract::SurfaceId::from("b"));
    }
}
//#endregion 🧪️Tests

//#endregion 🔖️Tracking
