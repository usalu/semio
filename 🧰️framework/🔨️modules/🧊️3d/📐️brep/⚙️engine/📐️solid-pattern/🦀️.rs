//! 📐️ Pattern publication with one native compound claim and private child topology.

use super::*;
use semio_framework_3d_brep_native_registry::{UnpublishedSolidPattern, primitive_handle};
pub use semio_framework_3d_brep_native_registry::SolidPatternRecipe;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetainedCloneStep as Step, RetirementDemand, ValueError, ValueRefusalKind};

/// 🧺️ Binds a group of prepared pattern solid roots to its original source and one live address.
pub struct SolidPatternPublication {
    owner: usize,
    revision: u64,
    source: SolidId,
    source_key: [u8; 64],
    label: PersistentLabel,
    key: [u8; 64],
    topology: UnpublishedSolidPattern,
    published: bool,
}

impl SolidPatternPublication {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.key).expect("digest is ASCII") }
    pub fn ready(&self) -> bool { self.topology.ready() }
    pub fn terminal_is_empty(&self) -> bool { self.topology.terminal_is_empty() }
}

impl Brep {
    /// 🎟️ Binds the pattern to one constructor-certified native solid.
    pub fn plan_solid_pattern(&mut self, source: &str, recipe: SolidPatternRecipe) -> Result<SolidPatternPublication, ValueError> {
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "pattern requires a constructor-certified solid");
        let Entity::Solid(id, Some(SolidProvenance::Primitive(provenance))) = self.native.original_query_entity(source)? else { return Err(unsupported()); };
        let id = *id;
        if !self.native.body.solids.contains(id) { return Err(ValueError::invariant("original pattern source disappeared")); }
        let topology = UnpublishedSolidPattern::new(recipe, provenance.primitive);
        let label = self.native.body.new_label();
        let source_key = source.as_bytes().try_into().map_err(|_| ValueError::invariant("invalid pattern source address"))?;
        Ok(SolidPatternPublication { owner: self.pattern_owner(), revision: self.native.shape_mutation_revision,
            source: id, source_key, label, key: primitive_handle(b"Compound:", label.0), topology, published: false })
    }

    fn pattern_owner(&self) -> usize { std::ptr::from_ref(&self.native) as usize }

    fn pattern_source_depth(&self, plan: &SolidPatternPublication) -> Result<usize, ValueError> {
        let key = std::str::from_utf8(&plan.source_key).expect("source is a canonical digest");
        if self.pattern_owner() != plan.owner || self.native.shape_mutation_revision != plan.revision
            || !matches!(self.native.original_query_entity(key)?, Entity::Solid(id, Some(SolidProvenance::Primitive(_))) if *id == plan.source)
        {
            return Err(ValueError::invariant("original pattern source changed"));
        }
        self.native.certified_solid_depth(key)
    }

    pub fn solid_pattern_preparation_demand(&self, plan: &SolidPatternPublication) -> Result<RetirementDemand, ValueError> {
        let mut demand = plan.topology.demand(&self.native.body)?;
        demand.depth = demand.depth.max(self.pattern_source_depth(plan)?);
        Ok(demand)
    }

    pub fn step_solid_pattern_preparation(&mut self, plan: &mut SolidPatternPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let demand = self.solid_pattern_preparation_demand(plan)?;
        let before = plan.topology.draft_count();
        let result = plan.topology.advance(&mut self.native.body, plan.owner, plan.source, demand, |label| primitive_handle(b"Solid:", label), effect);
        self.native.unpublished_topology += plan.topology.draft_count() - before;
        result
    }

    pub fn solid_pattern_publication_demand(&self, plan: &SolidPatternPublication, copy: usize) -> Result<RetirementDemand, ValueError> {
        let depth = self.pattern_source_depth(plan)?;
        let key = plan.handle();
        if !plan.ready() || self.native.live.contains_key(key) { return Err(ValueError::invariant("original pattern compound is not ready")); }
        Ok(RetirementDemand { capacity_bytes: self.native.live.next_insert_capacity_byte_demand(key, copy)?,
            depth: depth.max(self.native.live.next_insert_depth_demand(key)?), ..Default::default() })
    }

    pub fn reserve_solid_pattern(&mut self, plan: &SolidPatternPublication, grant: Grant) -> Result<Progress, ValueError> {
        self.solid_pattern_publication_demand(plan, grant.maximum_copy_bytes)?;
        self.native.live.reserve_insert_step(plan.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    pub fn publish_solid_pattern(&mut self, plan: &mut SolidPatternPublication, key: String, grant: Grant) -> Result<Progress, (ValueError, String)> {
        if key != plan.handle() { return Err((ValueError::invariant("original pattern address changed"), key)); }
        if let Err(error) = self.solid_pattern_publication_demand(plan, grant.maximum_copy_bytes) { return Err((error, key)); }
        let (previous, receipt) = self.native.live.insert_reserved(key, Entity::Compound(plan.topology.take_roots(), plan.label), grant)
            .map_err(|(error, key, entity)| {
                if let Entity::Compound(roots, _) = entity { plan.topology.restore_roots(roots); }
                (error, key)
            })?;
        assert!(previous.is_none());
        self.native.unpublished_topology -= plan.topology.draft_count();
        plan.topology.commit();
        plan.published = true;
        self.native.advance_live_revision();
        Ok(receipt)
    }

    pub fn solid_pattern_close_demand(&self, plan: &SolidPatternPublication) -> Result<RetirementDemand, ValueError> {
        if self.pattern_owner() != plan.owner { return Err(ValueError::invariant("original pattern owner changed")); }
        plan.topology.close_demand(&self.native.body)
    }

    pub fn close_solid_pattern(&mut self, plan: &mut SolidPatternPublication, effect: &mut RetainedAllocationEffect<'_>) -> Result<Step, ValueError> {
        self.solid_pattern_close_demand(plan)?;
        let before = plan.topology.draft_count();
        let result = plan.topology.close(&mut self.native.body, effect);
        if !plan.published { self.native.unpublished_topology -= before - plan.topology.draft_count(); }
        result
    }
}
