//! ♻️ Block3d document retirement owns every semantic field and its materialized kit catalog.

use crate::{
    Block3dMutation, Block3dSnapshot, Block3dVortexKind, Block3dVortexKindExtra, Block3dVortexTemplate, BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule,
    BlockKindIdentity, BlockMeta, BlockRepresentation,
};
use semio_s_artifact_stdio_semio::standards::v1::subsets::{
    base::schema::geometry::SemioTransform,
    kit::schema::snapshot::{SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitSnapshot, SemioKitType},
};
use std::{mem::ManuallyDrop, sync::Arc};
use store::retirement::{OwnedValueRetirementFactory, RetireOwned, RetirementCursor, RetirementStep, SharedValueRetirementFactory};

struct KindIdentity(BlockKindIdentity);
impl RetireOwned for KindIdentity {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockKindIdentity { id, name, label, variant, description, icon, unit } = self.0;
        store::artifact_retirement_sequence![id, name, label, variant, description, icon, unit]
    }
}

struct Attribute(BlockAttribute);
impl RetireOwned for Attribute {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockAttribute { key, value, definition } = self.0;
        store::artifact_retirement_sequence![key, value, definition]
    }
}

struct Author(BlockAuthor);
impl RetireOwned for Author {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockAuthor { id, name, email } = self.0;
        store::artifact_retirement_sequence![id, name, email]
    }
}

struct Compatibility(BlockCompatibilityRule);
impl RetireOwned for Compatibility {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockCompatibilityRule { id, source, target, bidirectional } = self.0;
        store::artifact_retirement_sequence![id, source, target, bidirectional]
    }
}

struct Representation(BlockRepresentation);
impl RetireOwned for Representation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockRepresentation { id, name, mesh_url, tags, lod, description, attributes } = self.0;
        store::artifact_retirement_sequence![id, name, mesh_url, tags, lod, description, attributes.into_iter().map(Attribute).collect::<Vec<_>>()]
    }
}

struct Camera(BlockCamera3d);
impl RetireOwned for Camera {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let BlockCamera3d { position, target, zoom } = self.0;
        store::artifact_retirement_sequence![position.into_iter().collect::<Vec<_>>(), target.into_iter().collect::<Vec<_>>(), zoom]
    }
}

struct Meta(BlockMeta);
impl RetireOwned for Meta {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        self.0.description.retirement()
    }
}

struct VortexKindExtra(Block3dVortexKindExtra);
impl RetireOwned for VortexKindExtra {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Block3dVortexKindExtra { id, name, label, color, default_cable_kind } = self.0;
        store::artifact_retirement_sequence![id, name, label, color, default_cable_kind]
    }
}

struct VortexKind(Block3dVortexKind);
impl RetireOwned for VortexKind {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Block3dVortexKind { id, name, label, color, default_cable_kind } = self.0;
        store::artifact_retirement_sequence![id, name, label, color, default_cable_kind]
    }
}

struct Vortex(Block3dVortexTemplate);
impl RetireOwned for Vortex {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let Block3dVortexTemplate { id, vortex_kind, position, direction, radius, label } = self.0;
        store::artifact_retirement_sequence![id, vortex_kind, position.into_iter().collect::<Vec<_>>(), direction.into_iter().collect::<Vec<_>>(), radius, label]
    }
}

struct KitType(SemioKitType);
impl RetireOwned for KitType {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioKitType { id, name, category } = self.0;
        store::artifact_retirement_sequence![id, name, category]
    }
}

struct Transform(SemioTransform);
impl RetireOwned for Transform {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioTransform { translation, rotation, scale } = self.0;
        store::artifact_retirement_sequence![
            vec![translation.x, translation.y, translation.z],
            vec![rotation.x, rotation.y, rotation.z, rotation.w],
            vec![scale.x, scale.y, scale.z],
        ]
    }
}

struct KitPiece(SemioKitPiece);
impl RetireOwned for KitPiece {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioKitPiece { id, type_id, transform } = self.0;
        store::artifact_retirement_sequence![id, type_id, Transform(transform)]
    }
}

struct KitConnection(SemioKitConnection);
impl RetireOwned for KitConnection {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioKitConnection { id, connecting_piece_id, connecting_port, connected_piece_id, connected_port } = self.0;
        store::artifact_retirement_sequence![id, connecting_piece_id, connecting_port, connected_piece_id, connected_port]
    }
}

struct KitDesign(SemioKitDesign);
impl RetireOwned for KitDesign {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioKitDesign { id, name, pieces, connections } = self.0;
        store::artifact_retirement_sequence![
            id,
            name,
            pieces.into_iter().map(KitPiece).collect::<Vec<_>>(),
            connections.into_iter().map(KitConnection).collect::<Vec<_>>(),
        ]
    }
}

struct KitSnapshot(SemioKitSnapshot);
impl RetireOwned for KitSnapshot {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        let SemioKitSnapshot { schema, types, designs, objects, models, properties, representations } = self.0;
        store::artifact_retirement_sequence![
            schema,
            types.into_iter().map(KitType).collect::<Vec<_>>(),
            designs.into_iter().map(KitDesign).collect::<Vec<_>>(),
            objects,
            models,
            properties,
            representations,
        ]
    }
}

struct CatalogRoot(ManuallyDrop<Option<Arc<SemioKitSnapshot>>>);
impl RetirementCursor for CatalogRoot {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        self.0.take().and_then(Arc::into_inner).map_or(RetirementStep::Complete, |value| RetirementStep::Child(KitSnapshot(value).retirement()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl Drop for CatalogRoot {
    fn drop(&mut self) {
        assert!(self.0.is_none(), "Block3d catalog root retired before terminal-empty");
    }
}

struct SnapshotRetirement(ManuallyDrop<Option<Block3dSnapshot>>);
impl RetirementCursor for SnapshotRetirement {
    fn close_step(&mut self, _: usize) -> RetirementStep {
        let Some(value) = self.0.as_mut() else { return RetirementStep::Complete };
        let catalog_owner = match value.catalog.take_local_owner::<SemioKitSnapshot>() {
            Ok(owner) => owner,
            Err(_) => return RetirementStep::BudgetExhausted,
        };
        let Block3dSnapshot { schema, object_kind, representations, catalog, vortex_kind_extra, vortices, compatibility, attributes, authors, camera3d, meta } =
            self.0.take().expect("exact Block3d snapshot remains owned");
        RetirementStep::Child(store::retirement::sequence(vec![
            schema.retirement(),
            KindIdentity(object_kind).retirement(),
            representations.into_iter().map(Representation).collect::<Vec<_>>().retirement(),
            catalog.retirement(),
            vortex_kind_extra.into_iter().map(VortexKindExtra).collect::<Vec<_>>().retirement(),
            vortices.into_iter().map(Vortex).collect::<Vec<_>>().retirement(),
            compatibility.into_iter().map(Compatibility).collect::<Vec<_>>().retirement(),
            attributes.into_iter().map(Attribute).collect::<Vec<_>>().retirement(),
            authors.into_iter().map(Author).collect::<Vec<_>>().retirement(),
            Camera(camera3d).retirement(),
            Meta(meta).retirement(),
            Box::new(CatalogRoot(ManuallyDrop::new(catalog_owner))),
        ]))
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}
impl Drop for SnapshotRetirement {
    fn drop(&mut self) {
        assert!(self.0.is_none(), "Block3d snapshot retired before terminal-empty");
    }
}
impl RetireOwned for Block3dSnapshot {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(SnapshotRetirement(ManuallyDrop::new(Some(self))))
    }
}

impl RetireOwned for Block3dMutation {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        match self {
            Self::RenameObjectKind(value) => value.new_name.retirement(),
            Self::ChangeObjectKindLabel(value) => value.new_label.retirement(),
            Self::ChangeObjectKindVariant(value) => value.new_variant.retirement(),
            Self::ChangeObjectKindDescription(value) => value.new_description.retirement(),
            Self::ChangeObjectKindIcon(value) => value.new_icon.retirement(),
            Self::ChangeObjectKindUnit(value) => value.new_unit.retirement(),
            Self::CreateRepresentation(value) => Representation(value.representation).retirement(),
            Self::DeleteRepresentation(value) => value.id.retirement(),
            Self::RenameRepresentation(value) => (value.id, value.new_name).retirement(),
            Self::ChangeRepresentationMeshUrl(value) => (value.id, value.new_mesh_url).retirement(),
            Self::ChangeRepresentationLod(value) => (value.id, value.new_lod).retirement(),
            Self::ChangeRepresentationDescription(value) => (value.id, value.new_description).retirement(),
            Self::AddRepresentationTag(value) => (value.id, value.tag).retirement(),
            Self::RemoveRepresentationTag(value) => (value.id, value.tag).retirement(),
            Self::AddRepresentationAttribute(value) => store::artifact_retirement_sequence![value.id, Attribute(value.attribute)],
            Self::RemoveRepresentationAttribute(value) => (value.id, value.key).retirement(),
            Self::CreateVortexKind(value) => VortexKind(value.vortex_kind).retirement(),
            Self::DeleteVortexKind(value) => value.id.retirement(),
            Self::RenameVortexKind(value) => (value.id, value.new_name).retirement(),
            Self::ChangeVortexKindLabel(value) => (value.id, value.new_label).retirement(),
            Self::ChangeVortexKindColor(value) => (value.id, value.new_color).retirement(),
            Self::ChangeVortexKindDefaultCableKind(value) => (value.id, value.new_default_cable_kind).retirement(),
            Self::CreateVortex(value) => Vortex(value.vortex).retirement(),
            Self::DeleteVortex(value) => value.id.retirement(),
            Self::MoveVortex(value) => store::artifact_retirement_sequence![value.id, value.new_position.into_iter().collect::<Vec<_>>(), value.new_direction.into_iter().collect::<Vec<_>>()],
            Self::ResizeVortex(value) => (value.id, value.new_radius).retirement(),
            Self::ChangeVortexVortexKind(value) => (value.id, value.new_vortex_kind).retirement(),
            Self::ChangeVortexLabel(value) => (value.id, value.new_label).retirement(),
            Self::AddCompatibilityRule(value) => Compatibility(value.rule).retirement(),
            Self::RemoveCompatibilityRule(value) => value.id.retirement(),
            Self::AddAttribute(value) => Attribute(value.attribute).retirement(),
            Self::RemoveAttribute(value) => value.key.retirement(),
            Self::AddAuthor(value) => Author(value.author).retirement(),
            Self::RemoveAuthor(value) => value.id.retirement(),
            Self::MoveCamera3d(value) => store::artifact_retirement_sequence![value.new_position.into_iter().collect::<Vec<_>>(), value.new_target.into_iter().collect::<Vec<_>>()],
            Self::ScaleCamera3d(value) => value.new_zoom.retirement(),
            Self::ChangeMetaDescription(value) => value.new_description.retirement(),
        }
    }
}

/// 🗃️ Installs Block3d's exact document root and retained mutation retirement authorities.
pub fn document_store_owners() -> store::MemberStoreOwners<Block3dSnapshot, Block3dMutation> {
    store::MemberStoreOwners::new(
        Arc::new(SharedValueRetirementFactory::<Block3dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block3dSnapshot>::default()),
        Arc::new(OwnedValueRetirementFactory::<Block3dMutation>::default()),
        Box::new(store::ArtifactStoreCursorDisposer::<Block3dSnapshot, Block3dMutation>::new()),
    )
}
