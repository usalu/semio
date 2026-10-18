//! 🧾️ Conversions and registry: the point and vector conversions with their finite and positive argument checks, the entity labels and roots and the entity registry of the session. See [`super`].

use super::*;

// #region 🧮Convert
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn pnt(v: EVec3) -> Pnt3 {
    Pnt3::new(v[0], v[1], v[2])
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn evec(p: Pnt3) -> EVec3 {
    [p.x, p.y, p.z]
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn vec3(v: EVec3) -> NativeVec3 {
    NativeVec3::new(v[0], v[1], v[2])
}

/// 🧭️ Refuses a non-finite point or vector argument of an affine transform, by its name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn require_finite_vector(label: &str, value: EVec3) -> Result<(), BrepError> {
    if value.iter().all(|component| component.is_finite()) {
        return Ok(());
    }
    Err(BrepError::InvalidInput(format!("{label} must be finite, got {value:?}")))
}

/// 🧭️ Refuses a direction argument that has no direction (zero or non-finite length) instead of substituting one.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn require_direction(label: &str, value: EVec3) -> Result<(), BrepError> {
    require_finite_vector(label, value)?;
    if vec3(value).normalized().is_some() {
        return Ok(());
    }
    Err(BrepError::InvalidInput(format!("{label} must be a non-zero direction, got {value:?}")))
}

/// 🧭️ Refuses a non-finite scalar argument of an affine transform, by its name.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn require_finite_scalar(label: &str, value: f64) -> Result<(), BrepError> {
    if value.is_finite() {
        return Ok(());
    }
    Err(BrepError::InvalidInput(format!("{label} must be finite, got {value}")))
}

/// 📏️ Requires a finite positive size before a selected feature changes the body.
pub(super) fn require_positive_size(label: &str, value: f64) -> Result<(), BrepError> {
    require_finite_scalar(label, value)?;
    if value > 0.0 { return Ok(()); }
    Err(BrepError::InvalidInput(format!("{label} must be positive, got {value}")))
}

/// 🧭️ Refuses a uniform scale factor that collapses the shape (zero) or is not a number.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn require_scale_factor(label: &str, value: f64) -> Result<(), BrepError> {
    require_finite_scalar(label, value)?;
    if value != 0.0 {
        return Ok(());
    }
    Err(BrepError::InvalidInput(format!("{label} must be non-zero, got {value}")))
}
// #endregion 🧮Convert

// #region 🧮Registry
/// 🏷️ The [`PersistentLabel`] a live [`Entity`] resolves to — the identity [`Brep::mint`] hashes a
/// handle from. Vertex/Edge/Face/🐚️Shell/Solid read it back out of the `Body` arena entry they wrap
/// (stable for the entity's whole lifetime, independent of when/how often it's registered); the
/// other variants carry a label they were stamped with at registration time (see [`Entity`]'s own
/// docstring for why).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn label_of_entity(body: &Body, entity: &Entity) -> Option<PersistentLabel> {
    match entity {
        Entity::Vertex(id) => body.vertices.get(*id).map(|v| v.label),
        Entity::Edge(id) => body.edges.get(*id).map(|e| e.label),
        Entity::Face(id) => body.faces.get(*id).map(|f| f.label),
        Entity::Shell(id) => body.shells.get(*id).map(|s| s.label),
        Entity::Solid(id) => body.solids.get(*id).map(|s| s.label),
        Entity::Wire(_, label) | Entity::Compound(_, label) | Entity::Curve(_, label) | Entity::Surface(_, label) => Some(*label),
    }
}

impl Brep {
    /// 🏗️ Mints a handle deterministic in `(kind, entity's PersistentLabel)` — never a counter —
    /// so registering the *same* labelled entity again (e.g. two `deconstruct` calls on an
    /// untouched shape) always yields byte-identical handles. Panics only if `entity` carries no
    /// resolvable label, which would mean a caller minted against an id it never actually inserted
    /// into `self.body` — a caller bug, not a runtime condition to recover from.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn mint(&mut self, kind: GeometryKind, entity: Entity) -> GeometryHandle {
        let label = label_of_entity(&self.body, &entity).expect("entity must carry a resolvable PersistentLabel");
        let payload = format!("{kind:?}:{}", label.0);
        let handle = GeometryHandle(semio_framework_hash::hash_bytes(payload.as_bytes()));
        self.live.insert(handle.as_str().to_string(), entity);
        self.advance_live_revision();
        handle
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_solid(&mut self, solid: SolidId) -> GeometryHandle {
        self.mint(GeometryKind::Solid, Entity::Solid(solid))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_face(&mut self, face: FaceId) -> GeometryHandle {
        self.mint(GeometryKind::Face, Entity::Face(face))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_shell(&mut self, shell: ShellId) -> GeometryHandle {
        self.mint(GeometryKind::Shell, Entity::Shell(shell))
    }
    /// 🧩️ `solids` (already registered elsewhere) bundled behind one fresh-labelled compound
    /// handle — the compound's label is its own, not shared with any member solid's.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_compound(&mut self, solids: Vec<SolidId>) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Compound, Entity::Compound(solids, label))
    }
    /// 🧩️ A `Wire` bundles arena ids but has no [`Body`]-stored identity of its own, so it is
    /// stamped with a fresh label at registration time (see [`Entity`]'s own docstring).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_wire(&mut self, wire: Wire) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Wire, Entity::Wire(wire, label))
    }
    /// 🧩️ A bare `Curve3` built via one of the curve constructors never enters `body.curves3`
    /// (only a curve reached through an `Edge` does), so it too is stamped with a fresh label.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_curve(&mut self, curve: Curve3) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Curve, Entity::Curve(curve, label))
    }
    /// 🧩️ Mirror of [`Self::register_curve`] for a bare `Surface`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn register_surface(&mut self, surface: Surface) -> GeometryHandle {
        let label = self.body.new_label();
        self.mint(GeometryKind::Surface, Entity::Surface(surface, label))
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn entity(&self, handle: &GeometryHandle) -> Result<&Entity, BrepError> {
        self.live.get(handle.as_str()).ok_or_else(|| BrepError::MissingHandle(handle.as_str().to_string()))
    }

    /// ♻️ [`semio_framework_3d_brep_representation::topology::EntityRef`] roots for every entity currently kept alive by a live
    /// handle — the protection set [`Body::reachable_from`] walks before [`Body::compact`] frees
    /// anything, so `dispose`/`retain` never reclaim geometry a surviving handle still needs. A
    /// `Wire`'s member edges/vertices are included even though the wire itself isn't a `Body` root,
    /// since nothing else would otherwise keep them alive while the wire handle is live.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn live_roots(&self) -> Vec<EntityRef> {
        self.live.values().flat_map(entity_roots).collect()
    }

    /// ♻️ Frees every arena entity no surviving live handle still reaches — the GC step `dispose`
    /// and `retain` both run after touching `self.live`.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    /// ⚠️ A live entry whose entity no longer RESOLVES is dropped first, and that ordering is the
    /// whole point. A stale `(index, generation)` marks its own id into the protection set while the
    /// arena's CURRENT id for that index is a different generation — so the compactor, seeing the
    /// live id absent, frees a perfectly good entity, whose own handle then goes stale in turn. One
    /// stale entry therefore eats the store one compaction at a time; measured as a boolean operator
    /// answering `missing entity: solid solid-9-0` for a box whose handle every later evaluation
    /// still named, which emptied the preview of every example built on it
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️slider-reevaluation-correctness-2026-09-15.md`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn compact_unreachable(&mut self) {
        if self.pending_mutations>0 {return;}
        let body = &self.body;
        let mut changed=false;for slot in 0..self.live.slot_count() {if let Some(original)=self.live.extract_slot_if(slot,|_,entity|label_of_entity(body,entity).is_some()){drop(original);changed=true;}}if changed{self.advance_live_revision();}
        let roots = self.live_roots();
        let keep = self.body.reachable_from(&roots);
        self.body.compact(keep);
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn solid_id(&self, handle: &GeometryHandle) -> Result<SolidId, BrepError> {
        match self.entity(handle)? {
            Entity::Solid(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a solid", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn face_id(&self, handle: &GeometryHandle) -> Result<FaceId, BrepError> {
        match self.entity(handle)? {
            Entity::Face(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not a face", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn wire_ref(&self, handle: &GeometryHandle) -> Result<&Wire, BrepError> {
        match self.entity(handle)? {
            Entity::Wire(w, _) => Ok(w),
            _ => Err(BrepError::InvalidInput(format!("{} is not a wire", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn curve_ref(&self, handle: &GeometryHandle) -> Result<&Curve3, BrepError> {
        match self.entity(handle)? {
            Entity::Curve(c, _) => Ok(c),
            _ => Err(BrepError::InvalidInput(format!("{} is not a curve", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn surface_ref(&self, handle: &GeometryHandle) -> Result<&Surface, BrepError> {
        match self.entity(handle)? {
            Entity::Surface(s, _) => Ok(s),
            _ => Err(BrepError::InvalidInput(format!("{} is not a surface", handle.as_str()))),
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn edge_id(&self, handle: &GeometryHandle) -> Result<EdgeId, BrepError> {
        match self.entity(handle)? {
            Entity::Edge(id) => Ok(*id),
            _ => Err(BrepError::InvalidInput(format!("{} is not an edge", handle.as_str()))),
        }
    }
}

/// ♻️ The arena roots ONE live handle's entity keeps alive — the unit both the GC's protection set
/// ([`Brep::live_roots`]) and the per-shape validation scope ([`Brep::validate_gate_sync`]) are built
/// from, so a shape's reach means exactly the same thing to both.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn entity_roots(entity: &Entity) -> Vec<EntityRef> {
    match entity {
        Entity::Vertex(id) => vec![EntityRef::Vertex(*id)],
        Entity::Edge(id) => vec![EntityRef::Edge(*id)],
        Entity::Face(id) => vec![EntityRef::Face(*id)],
        Entity::Shell(id) => vec![EntityRef::Shell(*id)],
        Entity::Solid(id) => vec![EntityRef::Solid(*id)],
        Entity::Compound(solids, _) => solids.iter().map(|solid| EntityRef::Solid(*solid)).collect(),
        Entity::Wire(wire, _) => wire.members.iter().map(|(edge, _)| EntityRef::Edge(*edge)).chain(wire.vertices.iter().map(|vertex| EntityRef::Vertex(*vertex))).collect(),
        Entity::Curve(_, _) | Entity::Surface(_, _) => Vec::new(),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn entity_tag(e: &Entity) -> String {
    match e {
        Entity::Vertex(id) => format!("v{id}"),
        Entity::Edge(id) => format!("e{id}"),
        Entity::Wire(w, _) => format!("w{}", w.members.len()),
        Entity::Face(id) => format!("f{id}"),
        Entity::Shell(id) => format!("sh{id}"),
        Entity::Solid(id) => format!("s{id}"),
        Entity::Compound(solids, _) => format!("cp{}", solids.len()),
        Entity::Curve(_, _) => "c".into(),
        Entity::Surface(_, _) => "S".into(),
    }
}
// #endregion 🧮Registry
