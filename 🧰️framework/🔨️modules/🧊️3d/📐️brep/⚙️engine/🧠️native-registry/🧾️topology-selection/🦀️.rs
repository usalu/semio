//! 🧾️ Stable zero-copy views of a solid's shells or a compound's members.

use super::*;
use super::analytic_publication::primitive_handle;
use semio_framework_value::{RetainedCloneGrant as Grant, RetainedCloneProgress as Progress, RetirementDemand};

#[derive(Clone, Copy)]
pub enum TopologyList {
    Shells,
    Solids,
}

#[derive(Clone, Copy)]
enum SelectionRoot {
    Solid(SolidId),
    Compound([u8; 64]),
}

#[derive(Clone, Copy)]
pub struct TopologySelection {
    owner: usize,
    revision: u64,
    root: SelectionRoot,
    count: usize,
}
semio_framework_value::artifact_retire_leaf!(TopologySelection);

#[derive(Clone, Copy)]
enum AliasRoot {
    Vertex(VertexId),
    Edge(EdgeId),
    Face(FaceId),
    Shell(ShellId),
    Solid(SolidId),
}

#[derive(Clone, Copy)]
pub struct TopologyAlias {
    owner: usize,
    revision: u64,
    root: AliasRoot,
    handle: [u8; 64],
    label: u64,
}
semio_framework_value::artifact_retire_leaf!(TopologyAlias);

impl TopologySelection {
    pub fn count(&self) -> usize { self.count }
    pub fn kind(&self) -> TopologyList {
        match self.root {
            SelectionRoot::Solid(_) => TopologyList::Shells,
            SelectionRoot::Compound(_) => TopologyList::Solids,
        }
    }
}

impl TopologyAlias {
    pub fn handle(&self) -> &str { std::str::from_utf8(&self.handle).expect("topology handle is lowercase hexadecimal") }

    pub fn label(&self) -> u64 { self.label }

    fn check(&self, kernel: &BrepRegistry) -> Result<(), ValueError> {
        if self.owner != std::ptr::from_ref(kernel) as usize || self.revision != kernel.shape_mutation_revision {
            return Err(ValueError::invariant("original topology selection changed"));
        }
        let present = match self.root {
            AliasRoot::Vertex(id) => kernel.body.vertices.contains(id),
            AliasRoot::Edge(id) => kernel.body.edges.contains(id),
            AliasRoot::Face(id) => kernel.body.faces.contains(id),
            AliasRoot::Shell(id) => kernel.body.shells.contains(id),
            AliasRoot::Solid(id) => kernel.body.solids.contains(id),
        };
        if !present { return Err(ValueError::invariant("original topology member is missing")); }
        Ok(())
    }
}

impl AliasRoot {
    fn matches(self, entity: &Entity) -> bool {
        match (self, entity) {
            (Self::Vertex(expected), Entity::Vertex(id)) => expected == *id,
            (Self::Edge(expected), Entity::Edge(id)) => expected == *id,
            (Self::Face(expected), Entity::Face(id)) => expected == *id,
            (Self::Shell(expected), Entity::Shell(id)) => expected == *id,
            (Self::Solid(expected), Entity::Solid(id, _)) => expected == *id,
            _ => false,
        }
    }

    fn entity(self) -> Entity {
        match self {
            Self::Vertex(id) => Entity::Vertex(id),
            Self::Edge(id) => Entity::Edge(id),
            Self::Face(id) => Entity::Face(id),
            Self::Shell(id) => Entity::Shell(id),
            Self::Solid(id) => Entity::Solid(id, None),
        }
    }

    fn depth(self, body: &Body) -> Result<usize, ValueError> {
        match self {
            Self::Vertex(id) => body.vertices.next_remove_depth_demand(id),
            Self::Edge(id) => body.edges.next_remove_depth_demand(id),
            Self::Face(id) => body.faces.next_remove_depth_demand(id),
            Self::Shell(id) => body.shells.next_remove_depth_demand(id),
            Self::Solid(id) => body.solids.next_remove_depth_demand(id),
        }
    }
}

impl BrepRegistry {
    /// 🧾️ Admits one ordered topology list without copying the backing.
    pub fn plan_topology_selection(&self, handle: &str, kind: TopologyList) -> Result<TopologySelection, ValueError> {
        let entity = self.original_query_entity(handle)?;
        let (root, count) = match (kind, entity) {
            (TopologyList::Shells, Entity::Solid(id, _)) => {
                let solid = self.body.solids.get(*id).ok_or_else(|| ValueError::invariant("original solid is missing"))?;
                (SelectionRoot::Solid(*id), solid.inners.len().saturating_add(1))
            }
            (TopologyList::Solids, Entity::Compound(members, _)) => {
                let source = handle.as_bytes().try_into().map_err(|_| ValueError::invariant("compound handle is not canonical"))?;
                (SelectionRoot::Compound(source), members.len())
            }
            (TopologyList::Shells, _) => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "shell selection requires a solid")),
            (TopologyList::Solids, _) => return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "explode requires a compound")),
        };
        if count > 65_536 {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "topology selection exceeds 65,536 items"));
        }
        Ok(TopologySelection { owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, root, count })
    }

    fn selected_topology(&self, plan: &TopologySelection, index: usize) -> Result<AliasRoot, ValueError> {
        if plan.owner != std::ptr::from_ref(self) as usize || plan.revision != self.shape_mutation_revision {
            return Err(ValueError::invariant("original topology selection changed"));
        }
        let missing = || ValueError::invariant("original topology index is missing");
        match &plan.root {
            SelectionRoot::Solid(id) => {
                let solid = self.body.solids.get(*id).ok_or_else(|| ValueError::invariant("original solid is missing"))?;
                if index == 0 { return Ok(AliasRoot::Shell(solid.outer)); }
                solid.inners.get(index - 1).copied().map(AliasRoot::Shell).ok_or_else(missing)
            }
            SelectionRoot::Compound(key) => {
                let source = std::str::from_utf8(key).expect("compound handle is ASCII");
                let Entity::Compound(members, _) = self.original_query_entity(source)? else {
                    return Err(ValueError::invariant("original compound is missing"));
                };
                if members.len() != plan.count { return Err(ValueError::invariant("original compound membership changed")); }
                members.get(index).copied().map(AliasRoot::Solid).ok_or_else(missing)
            }
        }
    }

    /// 📐️ Quotes access to a source and one of its native member labels.
    pub fn topology_member_depth(&self, plan: &TopologySelection, index: usize) -> Result<usize, ValueError> {
        let member = self.selected_topology(plan, index)?;
        let root = match &plan.root {
            SelectionRoot::Solid(id) => self.body.solids.next_remove_depth_demand(*id)?,
            SelectionRoot::Compound(key) => self.live.next_contains_depth_demand(std::str::from_utf8(key).expect("compound handle is ASCII"))?,
        };
        Ok(root.max(member.depth(&self.body)?))
    }

    /// 🏷️ Derives the same stable handle as synchronously registering the original member.
    pub fn topology_member(&self, plan: &TopologySelection, index: usize) -> Result<TopologyAlias, ValueError> {
        self.make_topology_alias(self.selected_topology(plan, index)?)
    }

    /// 🪪️ Makes an alias for one reached original component.
    pub fn reached_topology_alias(&self, member: EntityRef) -> Result<TopologyAlias, ValueError> {
        self.make_topology_alias(Self::reached_alias_root(member)?)
    }

    pub fn reached_topology_alias_depth(&self, member: EntityRef) -> Result<usize, ValueError> {
        Self::reached_alias_root(member)?.depth(&self.body)
    }

    fn reached_alias_root(member: EntityRef) -> Result<AliasRoot, ValueError> {
        match member {
            EntityRef::Vertex(id) => Ok(AliasRoot::Vertex(id)),
            EntityRef::Edge(id) => Ok(AliasRoot::Edge(id)),
            EntityRef::Face(id) => Ok(AliasRoot::Face(id)),
            EntityRef::Shell(id) => Ok(AliasRoot::Shell(id)),
            _ => Err(ValueError::invariant("unsupported reached topology alias")),
        }
    }

    fn make_topology_alias(&self, root: AliasRoot) -> Result<TopologyAlias, ValueError> {
        let kind: &[u8] = match root {
            AliasRoot::Vertex(_) => b"Vertex:",
            AliasRoot::Edge(_) => b"Edge:",
            AliasRoot::Face(_) => b"Face:",
            AliasRoot::Shell(_) => b"Shell:",
            AliasRoot::Solid(_) => b"Solid:",
        };
        let label = label_of_entity(&self.body, &root.entity()).ok_or_else(|| ValueError::invariant("original topology member is missing"))?;
        Ok(TopologyAlias { owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, root, handle: primitive_handle(kind, label.0), label: label.0 })
    }

    /// 🔎️ Quotes a deterministic alias insertion or reuse of the existing original member.
    pub fn topology_alias_demand(&self, alias: &TopologyAlias, copy: usize) -> Result<RetirementDemand, ValueError> {
        alias.check(self)?;
        let key = alias.handle();
        let (capacity_bytes, depth) = match self.live.get(key) {
            Some(entity) if alias.root.matches(entity) => (0, self.live.next_contains_depth_demand(key)?),
            Some(_) => return Err(ValueError::invariant("topology alias points to another entity")),
            None => (self.live.next_insert_capacity_byte_demand(key, copy)?, self.live.next_insert_depth_demand(key)?),
        };
        Ok(RetirementDemand { capacity_bytes, depth: depth.max(alias.root.depth(&self.body)?), ..Default::default() })
    }

    /// 🗄️ Reserves the index backing for one previously unregistered alias.
    pub fn reserve_topology_alias(&mut self, alias: &TopologyAlias, grant: Grant) -> Result<Progress, ValueError> {
        self.topology_alias_demand(alias, grant.maximum_copy_bytes)?;
        self.live.reserve_insert_step(alias.handle(), grant).map_err(|(error, receipt)| error.with_retained_progress(receipt))
    }

    /// ♻️ Publishes the original member reference, preserving any already live equivalent.
    pub fn publish_topology_alias(&mut self, alias: &TopologyAlias, key: &mut String, grant: Grant) -> Result<Progress, ValueError> {
        if key.as_str() != alias.handle() { return Err(ValueError::invariant("original topology alias key changed")); }
        self.topology_alias_demand(alias, grant.maximum_copy_bytes)?;
        if self.live.contains_key(key.as_str()) { return Ok(Progress { copied_items: 1, ..Default::default() }); }
        let (previous, receipt) = self.live.insert_reserved(std::mem::take(key), alias.root.entity(), grant)
            .map_err(|(error, original, _)| { *key = original; error })?;
        assert!(previous.is_none());
        self.advance_live_revision();
        Ok(receipt)
    }
}
