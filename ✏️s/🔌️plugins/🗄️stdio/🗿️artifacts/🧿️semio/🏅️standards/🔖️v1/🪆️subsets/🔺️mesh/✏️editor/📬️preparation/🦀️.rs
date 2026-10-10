//! 📬️ Paged structural preparation for one mesh vertex move.

use super::*;
use crate::{
    retained_native_preparation::{StructuralCopyStep, StructuralMutationCopy, StructuralPreparationFactory},
    standards::v1::subsets::{
        base::schema::geometry::{SemioPoint3, SemioRgba, SemioUv},
        mesh::schema::{
            mutations::move_vertex,
            snapshot::{SemioMaterial, SemioMesh, SemioPrimitive, SemioTexture, SemioTopology},
        },
    },
    SemioMutationRetirementFactory, SemioSnapshotRetirementFactory,
};
use semio_framework_plugin::plugin_app_close_prelude::store as app_store;
use semio_s_artifact_stdio_contract::editing::{NativeEditPreparationRoute, RetainedBytesCopy, RetainedTextCopy};
use std::{marker::PhantomData, mem::size_of, sync::Arc};

const PREFIX: &str = "stdio-semio-mesh-set-vertex";

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<SemioMeshSnapshot, SemioMeshMutation>> {
    Some(NativeEditPreparationRoute::new(
        recognizes,
        Arc::new(StructuralPreparationFactory::new(
            PREFIX,
            recognizes,
            preflight,
            || Box::<MeshStructuralCopy>::default(),
            Arc::new(SemioMutationRetirementFactory::<SemioMeshMutation>(PhantomData)),
            Arc::new(SemioSnapshotRetirementFactory::<SemioMeshSnapshot>(PhantomData)),
        )),
    ))
}

fn recognizes(mutation: &SemioMeshMutation) -> bool {
    matches!(mutation, SemioMeshMutation::MoveVertex(_))
}

fn preflight(mutation: &SemioMeshMutation) -> Result<usize, String> {
    let SemioMeshMutation::MoveVertex(value) = mutation else { return Err(format!("{PREFIX}-mutation")) };
    value.mesh_id.len().checked_add(value.primitive_id.len()).and_then(|bytes| bytes.checked_add(size_of::<usize>() + size_of::<SemioPoint3>())).ok_or_else(|| format!("{PREFIX}-payload-overflow"))
}

#[derive(Default)]
struct PodCopy<T> {
    values: Vec<T>,
    reserved: bool,
    complete: bool,
}

impl<T: Copy> PodCopy<T> {
    fn advance(&mut self, source: &[T], maximum_items: usize, maximum_bytes: usize) -> Result<(usize, usize), String> {
        if !self.reserved {
            self.values.try_reserve_exact(source.len()).map_err(|_| format!("{PREFIX}-vector-allocation"))?;
            self.reserved = true;
            self.complete = source.is_empty();
            return Ok((1, 0));
        }
        if self.complete {
            return Ok((0, 0));
        }
        let element_bytes = size_of::<T>().max(1);
        let count = maximum_items.min(maximum_bytes / element_bytes).min(source.len().saturating_sub(self.values.len()));
        if count == 0 {
            return Ok((0, 0));
        }
        let start = self.values.len();
        self.values.extend_from_slice(&source[start..start + count]);
        self.complete = self.values.len() == source.len();
        Ok((count, count.saturating_mul(element_bytes)))
    }

    fn take(&mut self) -> Option<Vec<T>> {
        self.complete.then(|| {
            self.complete = false;
            self.reserved = false;
            std::mem::take(&mut self.values)
        })
    }

    fn take_partial(&mut self) -> Vec<T> {
        self.complete = false;
        self.reserved = false;
        std::mem::take(&mut self.values)
    }

    fn terminal_is_empty(&self) -> bool {
        self.values.is_empty() && self.values.capacity() == 0 && !self.reserved && !self.complete
    }
}

fn text_step(copy: &mut RetainedTextCopy, source: &str, maximum_bytes: usize) -> Result<(usize, Option<String>), String> {
    let bytes = copy.advance(source, maximum_bytes)?.unwrap_or(0);
    let value = copy.is_complete().then(|| copy.take()).flatten();
    Ok((bytes, value))
}

fn text_close(copy: &mut RetainedTextCopy, grant: app_store::ArtifactStoreOneItemGrant) -> app_store::SnapshotRetirementStep {
    match copy.close_step(grant.maximum_items, grant.maximum_bytes) {
        semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => app_store::SnapshotRetirementStep::Pending { released_items, released_bytes },
        semio_framework_job::InteractiveJobCloseStep::Blocked => app_store::SnapshotRetirementStep::Blocked,
        semio_framework_job::InteractiveJobCloseStep::Complete => app_store::SnapshotRetirementStep::Complete,
    }
}

#[derive(Default)]
struct PrimitiveCopy {
    phase: u8,
    id_copy: RetainedTextCopy,
    id: Option<String>,
    positions: PodCopy<SemioPoint3>,
    normals: PodCopy<SemioPoint3>,
    uvs: PodCopy<SemioUv>,
    colors: PodCopy<SemioRgba>,
    indices: PodCopy<u32>,
    material_copy: RetainedTextCopy,
    material_id: Option<String>,
    material_present: bool,
    output: Option<SemioPrimitive>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl PrimitiveCopy {
    fn advance(&mut self, source: &SemioPrimitive, replacement: Option<(usize, SemioPoint3)>, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        match self.phase {
            0 => {
                let (bytes, value) = text_step(&mut self.id_copy, &source.id, maximum_bytes)?;
                if let Some(value) = value {
                    self.id = Some(value);
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => {
                let (items, bytes) = self.positions.advance(&source.positions, maximum_items, maximum_bytes)?;
                if self.positions.complete {
                    if let Some((index, point)) = replacement {
                        *self.positions.values.get_mut(index).ok_or_else(|| format!("{PREFIX}-vertex-index"))? = point;
                    }
                    self.phase = 2;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            2 => {
                let (items, bytes) = self.normals.advance(&source.normals, maximum_items, maximum_bytes)?;
                if self.normals.complete {
                    self.phase = 3;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            3 => {
                let (items, bytes) = self.uvs.advance(&source.uvs, maximum_items, maximum_bytes)?;
                if self.uvs.complete {
                    self.phase = 4;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            4 => {
                let (items, bytes) = self.colors.advance(&source.colors, maximum_items, maximum_bytes)?;
                if self.colors.complete {
                    self.phase = 5;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            5 => {
                let (items, bytes) = self.indices.advance(&source.indices, maximum_items, maximum_bytes)?;
                if self.indices.complete {
                    self.phase = 6;
                }
                Ok(StructuralCopyStep::Progress { items, bytes })
            }
            6 => match source.material_id.as_deref() {
                Some(material) => {
                    let (bytes, value) = text_step(&mut self.material_copy, material, maximum_bytes)?;
                    if let Some(value) = value {
                        self.material_id = Some(value);
                        self.material_present = true;
                        self.phase = 7;
                    }
                    Ok(StructuralCopyStep::Progress { items: 1, bytes })
                }
                None => {
                    self.phase = 7;
                    Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                }
            },
            7 => {
                self.output = Some(SemioPrimitive {
                    id: self.id.take().ok_or_else(|| format!("{PREFIX}-primitive-id"))?,
                    topology: source.topology,
                    positions: self.positions.take().ok_or_else(|| format!("{PREFIX}-positions"))?,
                    normals: self.normals.take().ok_or_else(|| format!("{PREFIX}-normals"))?,
                    uvs: self.uvs.take().ok_or_else(|| format!("{PREFIX}-uvs"))?,
                    colors: self.colors.take().ok_or_else(|| format!("{PREFIX}-colors"))?,
                    indices: self.indices.take().ok_or_else(|| format!("{PREFIX}-indices"))?,
                    material_id: self.material_present.then(|| self.material_id.take()).flatten(),
                });
                self.phase = 8;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take(&mut self) -> Option<SemioPrimitive> {
        self.output.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        for copy in [&mut self.id_copy, &mut self.material_copy] {
            let step = text_close(copy, grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-primitive-retirement-witness")));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if self.output.is_some()
            || self.id.is_some()
            || !self.positions.values.is_empty()
            || !self.normals.values.is_empty()
            || !self.uvs.values.is_empty()
            || !self.colors.values.is_empty()
            || !self.indices.values.is_empty()
            || self.material_id.is_some()
        {
            let partial = self.output.take().unwrap_or_else(|| SemioPrimitive {
                id: self.id.take().unwrap_or_default(),
                topology: SemioTopology::default(),
                positions: self.positions.take_partial(),
                normals: self.normals.take_partial(),
                uvs: self.uvs.take_partial(),
                colors: self.colors.take_partial(),
                indices: self.indices.take_partial(),
                material_id: self.material_id.take(),
            });
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(partial));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.positions.take_partial();
        self.normals.take_partial();
        self.uvs.take_partial();
        self.colors.take_partial();
        self.indices.take_partial();
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.id_copy.terminal_is_empty()
            && self.material_copy.terminal_is_empty()
            && self.id.is_none()
            && self.material_id.is_none()
            && self.output.is_none()
            && self.retirement.is_none()
            && self.positions.terminal_is_empty()
            && self.normals.terminal_is_empty()
            && self.uvs.terminal_is_empty()
            && self.colors.terminal_is_empty()
            && self.indices.terminal_is_empty()
    }
}

#[derive(Default)]
struct MeshCopy {
    phase: u8,
    id_copy: RetainedTextCopy,
    id: Option<String>,
    primitives: Vec<SemioPrimitive>,
    primitive_index: usize,
    primitive: Option<PrimitiveCopy>,
    output: Option<SemioMesh>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl MeshCopy {
    fn advance(&mut self, source: &SemioMesh, target_primitive: Option<(&str, usize, SemioPoint3)>, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        if self.phase == 0 {
            let (bytes, value) = text_step(&mut self.id_copy, &source.id, maximum_bytes)?;
            if let Some(value) = value {
                self.id = Some(value);
                self.phase = 1;
            }
            return Ok(StructuralCopyStep::Progress { items: 1, bytes });
        }
        if self.phase == 1 {
            let Some(source_primitive) = source.primitives.get(self.primitive_index) else {
                self.phase = 2;
                return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
            };
            let replacement = target_primitive.and_then(|(id, index, point)| (source_primitive.id == id).then_some((index, point)));
            let cursor = self.primitive.get_or_insert_with(PrimitiveCopy::default);
            match cursor.advance(source_primitive, replacement, maximum_items, maximum_bytes)? {
                StructuralCopyStep::Progress { items, bytes } => return Ok(StructuralCopyStep::Progress { items, bytes }),
                StructuralCopyStep::Complete => {
                    self.primitives.push(cursor.take().ok_or_else(|| format!("{PREFIX}-primitive-result"))?);
                    self.primitive = None;
                    self.primitive_index += 1;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                }
            }
        }
        if self.phase == 2 {
            self.output = Some(SemioMesh { id: self.id.take().ok_or_else(|| format!("{PREFIX}-mesh-id"))?, primitives: std::mem::take(&mut self.primitives) });
            self.phase = 3;
        }
        Ok(StructuralCopyStep::Complete)
    }

    fn take(&mut self) -> Option<SemioMesh> {
        self.output.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(primitive) = self.primitive.as_mut() {
            primitive.begin_close();
        }
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(primitive) = self.primitive.as_mut() {
            let step = primitive.close_step(grant)?;
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !primitive.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-primitive-copy-witness")));
            }
            self.primitive = None;
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let text = text_close(&mut self.id_copy, grant);
        if text != app_store::SnapshotRetirementStep::Complete {
            return Ok(text);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-mesh-retirement-witness")));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if self.output.is_some() || self.id.is_some() || !self.primitives.is_empty() {
            let partial = self.output.take().unwrap_or_else(|| SemioMesh { id: self.id.take().unwrap_or_default(), primitives: std::mem::take(&mut self.primitives) });
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(partial));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.id_copy.terminal_is_empty() && self.id.is_none() && self.primitives.is_empty() && self.primitive.is_none() && self.output.is_none() && self.retirement.is_none()
    }
}

#[derive(Default)]
struct MaterialCopy {
    phase: usize,
    texture_copies: [RetainedTextCopy; 5],
    id_copy: RetainedTextCopy,
    output: Option<SemioMaterial>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl MaterialCopy {
    fn advance(&mut self, source: &SemioMaterial, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        if self.phase == 0 {
            let (bytes, value) = text_step(&mut self.id_copy, &source.id, maximum_bytes)?;
            if let Some(id) = value {
                self.output = Some(SemioMaterial { id, base_color: source.base_color, metallic: source.metallic, roughness: source.roughness, ..Default::default() });
                self.phase = 1;
            }
            return Ok(StructuralCopyStep::Progress { items: 1, bytes });
        }
        if self.phase > 5 { return Ok(StructuralCopyStep::Complete); }
        let references = [&source.base_color_texture, &source.metallic_roughness_texture, &source.normal_texture, &source.occlusion_texture, &source.emissive_texture];
        let index = self.phase - 1;
        let mut bytes = 0;
        if let Some(reference) = references[index] {
            let (copied, value) = text_step(&mut self.texture_copies[index], reference, maximum_bytes)?;
            bytes = copied;
            if let Some(value) = value {
                let output = self.output.as_mut().ok_or_else(|| format!("{PREFIX}-material-output"))?;
                match index {
                    0 => output.base_color_texture = Some(value),
                    1 => output.metallic_roughness_texture = Some(value),
                    2 => output.normal_texture = Some(value),
                    3 => output.occlusion_texture = Some(value),
                    4 => output.emissive_texture = Some(value),
                    _ => unreachable!(),
                }
                self.phase += 1;
            }
        } else { self.phase += 1; }
        Ok(StructuralCopyStep::Progress { items: 1, bytes })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        let step = text_close(&mut self.id_copy, grant);
        if step != app_store::SnapshotRetirementStep::Complete {
            return Ok(step);
        }
        for copy in &mut self.texture_copies {
            let step = text_close(copy, grant);
            if step != app_store::SnapshotRetirementStep::Complete { return Ok(step); }
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-material-retirement-witness")));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(value) = self.output.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.id_copy.terminal_is_empty() && self.texture_copies.iter().all(RetainedTextCopy::terminal_is_empty) && self.output.is_none() && self.retirement.is_none()
    }
}

#[derive(Default)]
struct TextureCopy {
    phase: u8,
    id_copy: RetainedTextCopy,
    mime_copy: RetainedTextCopy,
    id: Option<String>,
    mime: Option<String>,
    bytes: RetainedBytesCopy,
    output: Option<SemioTexture>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl TextureCopy {
    fn advance(&mut self, source: &SemioTexture, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        match self.phase {
            0 => {
                let (bytes, value) = text_step(&mut self.id_copy, &source.id, maximum_bytes)?;
                if let Some(value) = value {
                    self.id = Some(value);
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => {
                let (bytes, value) = text_step(&mut self.mime_copy, &source.mime, maximum_bytes)?;
                if let Some(value) = value {
                    self.mime = Some(value);
                    self.phase = 2;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            2 => {
                let bytes = self.bytes.advance(&source.bytes, maximum_bytes)?.unwrap_or(0);
                if self.bytes.is_complete() {
                    self.phase = 3;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            3 => {
                self.output = Some(SemioTexture {
                    id: self.id.take().ok_or_else(|| format!("{PREFIX}-texture-id"))?,
                    mime: self.mime.take().ok_or_else(|| format!("{PREFIX}-texture-mime"))?,
                    bytes: self.bytes.take().ok_or_else(|| format!("{PREFIX}-texture-bytes"))?,
                });
                self.phase = 4;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        for copy in [&mut self.id_copy, &mut self.mime_copy] {
            let step = text_close(copy, grant);
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
        }
        match self.bytes.close_step(grant.maximum_items, grant.maximum_bytes) {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items, released_bytes } => return Ok(app_store::SnapshotRetirementStep::Pending { released_items, released_bytes }),
            semio_framework_job::InteractiveJobCloseStep::Blocked => return Ok(app_store::SnapshotRetirementStep::Blocked),
            semio_framework_job::InteractiveJobCloseStep::Complete => {}
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-texture-retirement-witness")));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if self.output.is_some() || self.id.is_some() || self.mime.is_some() {
            let value = self.output.take().unwrap_or_else(|| SemioTexture { id: self.id.take().unwrap_or_default(), mime: self.mime.take().unwrap_or_default(), bytes: self.bytes.take_partial() });
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(value));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.id_copy.terminal_is_empty() && self.mime_copy.terminal_is_empty() && self.bytes.terminal_is_empty() && self.id.is_none() && self.mime.is_none() && self.output.is_none() && self.retirement.is_none()
    }
}

#[derive(Default)]
struct MeshStructuralCopy {
    phase: u8,
    schema_copy: RetainedTextCopy,
    schema: Option<String>,
    meshes: Vec<SemioMesh>,
    mesh_index: usize,
    mesh: Option<MeshCopy>,
    materials: Vec<SemioMaterial>,
    material_index: usize,
    material: Option<MaterialCopy>,
    textures: Vec<SemioTexture>,
    texture_index: usize,
    texture: Option<TextureCopy>,
    mesh_matches: usize,
    primitive_matches: usize,
    old_point: Option<SemioPoint3>,
    result: Option<(SemioMeshSnapshot, SemioMeshMutation)>,
    orphan_inverse: Option<SemioMeshMutation>,
    retirement: Option<Box<dyn app_store::ErasedSnapshotRetirement>>,
    closing: bool,
}

impl MeshStructuralCopy {
    fn target(mutation: &SemioMeshMutation) -> Result<&move_vertex::MoveVertex, String> {
        let SemioMeshMutation::MoveVertex(value) = mutation else { return Err(format!("{PREFIX}-mutation")) };
        Ok(value)
    }
}

impl StructuralMutationCopy<SemioMeshSnapshot, SemioMeshMutation> for MeshStructuralCopy {
    fn advance(&mut self, base: &SemioMeshSnapshot, mutation: &SemioMeshMutation, maximum_items: usize, maximum_bytes: usize) -> Result<StructuralCopyStep, String> {
        let target = Self::target(mutation)?;
        match self.phase {
            0 => {
                let (bytes, value) = text_step(&mut self.schema_copy, &base.schema, maximum_bytes)?;
                if let Some(value) = value {
                    self.schema = Some(value);
                    self.phase = 1;
                }
                Ok(StructuralCopyStep::Progress { items: 1, bytes })
            }
            1 => {
                let Some(source) = base.meshes.get(self.mesh_index) else {
                    self.phase = 2;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                if self.mesh.is_none() && source.id == target.mesh_id {
                    self.mesh_matches += 1;
                }
                if source.id == target.mesh_id {
                    let primitive_index = self.mesh.as_ref().map_or(0, |cursor| cursor.primitive_index);
                    let primitive_starts = self.mesh.as_ref().is_some_and(|cursor| cursor.phase == 1 && cursor.primitive.is_none());
                    if primitive_starts {
                        if let Some(primitive) = source.primitives.get(primitive_index).filter(|primitive| primitive.id == target.primitive_id) {
                            let point = *primitive.positions.get(target.vertex_index).ok_or_else(|| format!("{PREFIX}-vertex-index"))?;
                            self.primitive_matches += 1;
                            self.old_point = Some(point);
                        }
                    }
                }
                let primitive_target = (source.id == target.mesh_id).then_some((target.primitive_id.as_str(), target.vertex_index, target.new_point));
                let cursor = self.mesh.get_or_insert_with(MeshCopy::default);
                match cursor.advance(source, primitive_target, maximum_items, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.meshes.push(cursor.take().ok_or_else(|| format!("{PREFIX}-mesh-result"))?);
                        self.mesh = None;
                        self.mesh_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            2 => {
                let Some(source) = base.materials.get(self.material_index) else {
                    self.phase = 3;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.material.get_or_insert_with(MaterialCopy::default);
                match cursor.advance(source, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.materials.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-material-result"))?);
                        self.material = None;
                        self.material_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            3 => {
                let Some(source) = base.textures.get(self.texture_index) else {
                    self.phase = 4;
                    return Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 });
                };
                let cursor = self.texture.get_or_insert_with(TextureCopy::default);
                match cursor.advance(source, maximum_bytes)? {
                    StructuralCopyStep::Progress { items, bytes } => Ok(StructuralCopyStep::Progress { items, bytes }),
                    StructuralCopyStep::Complete => {
                        self.textures.push(cursor.output.take().ok_or_else(|| format!("{PREFIX}-texture-result"))?);
                        self.texture = None;
                        self.texture_index += 1;
                        Ok(StructuralCopyStep::Progress { items: 1, bytes: 0 })
                    }
                }
            }
            4 => {
                if self.mesh_matches != 1 || self.primitive_matches != 1 {
                    return Err(format!("{PREFIX}-target-count-{}-{}", self.mesh_matches, self.primitive_matches));
                }
                let inverse = SemioMeshMutation::MoveVertex(move_vertex::MoveVertex {
                    mesh_id: target.mesh_id.clone(),
                    primitive_id: target.primitive_id.clone(),
                    vertex_index: target.vertex_index,
                    new_point: self.old_point.ok_or_else(|| format!("{PREFIX}-inverse"))?,
                });
                self.result = Some((
                    SemioMeshSnapshot { schema: self.schema.take().ok_or_else(|| format!("{PREFIX}-schema"))?, meshes: std::mem::take(&mut self.meshes), materials: std::mem::take(&mut self.materials), textures: std::mem::take(&mut self.textures) },
                    inverse,
                ));
                self.phase = 5;
                Ok(StructuralCopyStep::Complete)
            }
            _ => Ok(StructuralCopyStep::Complete),
        }
    }

    fn take_result(&mut self) -> Option<(SemioMeshSnapshot, SemioMeshMutation)> {
        self.result.take()
    }

    fn begin_close(&mut self) {
        self.closing = true;
        if let Some(mesh) = self.mesh.as_mut() {
            mesh.begin_close();
        }
        if let Some(material) = self.material.as_mut() {
            material.begin_close();
        }
        if let Some(texture) = self.texture.as_mut() {
            texture.begin_close();
        }
    }

    fn close_step(&mut self, grant: app_store::ArtifactStoreOneItemGrant) -> Result<app_store::SnapshotRetirementStep, semio_framework_value::ValueError> {
        if !self.closing {
            return Ok(app_store::SnapshotRetirementStep::Blocked);
        }
        if let Some(mesh) = self.mesh.as_mut() {
            let step = mesh.close_step(grant)?;
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !mesh.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-mesh-copy-witness")));
            }
            self.mesh = None;
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(material) = self.material.as_mut() {
            let step = material.close_step(grant)?;
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !material.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-material-copy-witness")));
            }
            self.material = None;
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(texture) = self.texture.as_mut() {
            let step = texture.close_step(grant)?;
            if step != app_store::SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !texture.terminal_is_empty() {
                return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-texture-copy-witness")));
            }
            self.texture = None;
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        let text = text_close(&mut self.schema_copy, grant);
        if text != app_store::SnapshotRetirementStep::Complete {
            return Ok(text);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            let step = retirement.close_step(grant.maximum_items.min(1), grant.maximum_bytes)?;
            if step == app_store::SnapshotRetirementStep::Complete {
                if !retirement.terminal_is_empty() {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,format!("{PREFIX}-snapshot-retirement-witness")));
                }
                self.retirement = None;
                return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(inverse) = self.orphan_inverse.take() {
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(inverse));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.result.is_some() || self.schema.is_some() || !self.meshes.is_empty() || !self.materials.is_empty() || !self.textures.is_empty() {
            let snapshot = if let Some((snapshot, inverse)) = self.result.take() {
                self.orphan_inverse = Some(inverse);
                snapshot
            } else {
                SemioMeshSnapshot { schema: self.schema.take().unwrap_or_default(), meshes: std::mem::take(&mut self.meshes), materials: std::mem::take(&mut self.materials), textures: std::mem::take(&mut self.textures) }
            };
            self.retirement = Some(semio_framework_value::retirement::owned_retirement(snapshot));
            return Ok(app_store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(app_store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.schema_copy.terminal_is_empty()
            && self.schema.is_none()
            && self.meshes.is_empty()
            && self.mesh.is_none()
            && self.materials.is_empty()
            && self.material.is_none()
            && self.textures.is_empty()
            && self.texture.is_none()
            && self.result.is_none()
            && self.orphan_inverse.is_none()
            && self.retirement.is_none()
    }
}

fn canonical_scalar(value: app_store::ArtifactCanonicalJsonNode<'_>) -> app_store::ArtifactCanonicalJsonValue<'_> {
    app_store::ArtifactCanonicalJsonValue::Scalar(value)
}

fn canonical_object<'a, const N: usize>(mut fields: [(&'a str, app_store::ArtifactCanonicalJsonValue<'a>); N]) -> app_store::ArtifactCanonicalJsonValue<'a> {
    fields.sort_unstable_by(|left, right| left.0.cmp(right.0));
    app_store::ArtifactCanonicalJsonValue::Object(app_store::ArtifactCanonicalJsonObject::new(fields.into_iter()))
}

impl app_store::ArtifactCanonicalJson for SemioMeshMutation {
    fn canonical_json_borrowed_root(&self) -> Result<Option<app_store::ArtifactCanonicalJsonValue<'_>>, semio_framework_value::ValueError> {
        let SemioMeshMutation::MoveVertex(value) = self else { return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue, "stdio-semio-mesh-set-vertex-canonical-mutation")) };
        let point = canonical_object([
            ("x", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.x))),
            ("y", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.y))),
            ("z", canonical_scalar(app_store::ArtifactCanonicalJsonNode::F64(value.new_point.z))),
        ]);
        Ok(Some(canonical_object([
            ("kind", canonical_scalar(app_store::ArtifactCanonicalJsonNode::String("moveVertex"))),
            ("meshId", canonical_scalar(app_store::ArtifactCanonicalJsonNode::String(&value.mesh_id))),
            ("newPoint", point),
            ("primitiveId", canonical_scalar(app_store::ArtifactCanonicalJsonNode::String(&value.primitive_id))),
            ("vertexIndex", canonical_scalar(app_store::ArtifactCanonicalJsonNode::U64(value.vertex_index as u64))),
        ])))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../📇️registry/🧬️contract/✏️editing/🧫️fixtures/🧵️retained-native/🔣️.json"))).expect("retained native fixture")
    }

    fn large_snapshot() -> SemioMeshSnapshot {
        let values = fixture();
        let values = &values["structuralCopy"];
        let vertex_count = values["meshVertexCount"].as_u64().expect("mesh vertex count") as usize;
        let texture_bytes = values["textureByteLength"].as_u64().expect("texture byte length") as usize;
        SemioMeshSnapshot {
            schema: SEMIO_MESH_DOCUMENT_SCHEMA.into(),
            meshes: vec![SemioMesh { id: "mesh".into(), primitives: vec![SemioPrimitive { id: "primitive".into(), positions: (0..vertex_count).map(|index| SemioPoint3 { x: index as f64, y: 1.0, z: 2.0 }).collect(), ..Default::default() }] }],
            materials: vec![SemioMaterial { id: "material".into(), ..Default::default() }],
            textures: vec![SemioTexture { id: "texture".into(), mime: "application/octet-stream".into(), bytes: (0..texture_bytes).map(|index| (index % 251) as u8).collect() }],
        }
    }

    fn mutation(snapshot: &SemioMeshSnapshot) -> SemioMeshMutation {
        SemioMeshMutation::MoveVertex(move_vertex::MoveVertex { mesh_id: "mesh".into(), primitive_id: "primitive".into(), vertex_index: snapshot.meshes[0].primitives[0].positions.len() - 1, new_point: SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 } })
    }

    #[test]
    fn structural_copy_pages_large_mesh_and_preserves_untouched_payloads() {
        let source = large_snapshot();
        let mutation = mutation(&source);
        let mut copy = MeshStructuralCopy::default();
        let mut turns = 0;
        loop {
            match copy.advance(&source, &mutation, 32, 4_096).expect("mesh structural copy") {
                StructuralCopyStep::Progress { items, bytes } => {
                    assert!(items <= 32);
                    assert!(bytes <= 4_096);
                }
                StructuralCopyStep::Complete => break,
            }
            turns += 1;
            assert!(turns < 10_000);
        }
        let (post, inverse) = copy.take_result().expect("completed mesh result");
        assert!(turns > 512);
        assert_eq!(post.textures, source.textures);
        assert_eq!(post.materials, source.materials);
        assert_eq!(post.meshes[0].primitives[0].positions.len(), source.meshes[0].primitives[0].positions.len());
        assert_eq!(post.meshes[0].primitives[0].positions.last(), Some(&SemioPoint3 { x: -1.0, y: -2.0, z: -3.0 }));
        let SemioMeshMutation::MoveVertex(inverse) = inverse else { panic!("mesh inverse") };
        assert_eq!(inverse.new_point, *source.meshes[0].primitives[0].positions.last().expect("source target"));
        copy.begin_close();
        assert_eq!(copy.close_step(app_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }).expect("empty close"), app_store::SnapshotRetirementStep::Complete);
        assert!(copy.terminal_is_empty());
    }

    #[test]
    fn structural_copy_cancellation_retires_partial_vertex_buffers() {
        let source = large_snapshot();
        let mutation = mutation(&source);
        let mut copy = MeshStructuralCopy::default();
        for _ in 0..4_096 {
            if matches!(copy.advance(&source, &mutation, 1, 4_096).expect("mesh structural copy"), StructuralCopyStep::Complete) {
                break;
            }
        }
        copy.begin_close();
        let mut close_turns = 0;
        while !copy.terminal_is_empty() {
            copy.close_step(app_store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4_096 }).expect("bounded mesh close");
            close_turns += 1;
            assert!(close_turns < 20_000);
        }
        assert!(close_turns > 1);
    }
}
