//! 📨️ Actual geometry snapshots expose borrowed native fields directly to the canonical original Pack writer.
use super::*;
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource, FieldProjectionView as V, projection_path_error};
use semio_framework_value::RetainedCloneProgress;
pub(super) struct Source<'a>(pub &'a GltfSnapshot);
impl FieldProjectionSource for Source<'_> {
    fn projection_identity(&self) -> usize {
        self.0 as *const GltfSnapshot as usize
    }
    fn projection_view(&self, path: &[usize]) -> Result<V<'_>, ValueError> {
        let Some((&root, tail)) = path.split_first() else {
            return Ok(V::Record(&[0, 1, 2, 3]));
        };
        match root {
            0 => DslField::projection_view(&self.0.schema, tail),
            1 => match tail {
                [4, mesh, 0, primitive, rest @ ..] => primitive_view(self.0.document.meshes.get(*mesh).and_then(|m| m.primitives.get(*primitive)).ok_or_else(projection_path_error)?, rest),
                _ => DslField::projection_view(&self.0.document, tail),
            },
            2 => match tail {
                [] => Ok(V::List(self.0.buffers.len())),
                [i] if *i < self.0.buffers.len() => Ok(V::Record(&[0])),
                [i, 0, rest @ ..] => DslField::projection_view(self.0.buffers.get(*i).ok_or_else(projection_path_error)?, rest),
                _ => Err(projection_path_error()),
            },
            3 => DslField::projection_view(&self.0.source_form, tail),
            _ => Err(projection_path_error()),
        }
    }
}
fn primitive_view<'a>(value: &'a GltfPrimitive, path: &[usize]) -> Result<V<'a>, ValueError> {
    let Some((&field, tail)) = path.split_first() else {
        return Ok(V::Record(&[0, 1, 2, 3, 4, 5, 6]));
    };
    match field {
        0 => match tail {
            [] => Ok(V::List(value.attributes.len())),
            [i] if *i < value.attributes.len() => Ok(V::Record(&[0, 1])),
            [i, 0, rest @ ..] => DslField::projection_view(&value.attributes.get(*i).ok_or_else(projection_path_error)?.0, rest),
            [i, 1, rest @ ..] => DslField::projection_view(&value.attributes.get(*i).ok_or_else(projection_path_error)?.1, rest),
            _ => Err(projection_path_error()),
        },
        1 => {
            if tail.is_empty() {
                Ok(value.indices.map_or(V::Absent, |i| V::UInt(i as u64)))
            } else {
                Err(projection_path_error())
            }
        }
        2 => {
            if tail.is_empty() {
                Ok(V::Absent)
            } else {
                Err(projection_path_error())
            }
        }
        3 => {
            if tail.is_empty() {
                Ok(value.mode.map_or(V::Absent, V::UInt))
            } else {
                Err(projection_path_error())
            }
        }
        4 => {
            if tail.is_empty() {
                Ok(V::List(0))
            } else {
                Err(projection_path_error())
            }
        }
        5 | 6 => {
            if tail.is_empty() {
                Ok(V::Absent)
            } else {
                Err(projection_path_error())
            }
        }
        _ => Err(projection_path_error()),
    }
}
fn work(owner: &mut NativeSnapshotEncodeOwner<'_, '_>, bytes: usize) -> Result<(), ValueError> {
    let g = owner.remaining_grant();
    if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < bytes {
        return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "glTF geometry publication inspection exceeds original grant"));
    }
    owner.native().checkpoint()?;
    owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
    owner.native().step()
}
fn omitted(present: bool) -> Result<(), ValueError> {
    if present { Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "geometry Pack publication requires an actual geometry-only snapshot; semantic fields need their own source-owned codec")) } else { Ok(()) }
}
pub(super) fn validate(value: &GltfSnapshot, owner: &mut NativeSnapshotEncodeOwner<'_, '_>) -> Result<(), ValueError> {
    owner.native().begin_stage(0)?;
    work(owner, size_of::<GltfSnapshot>())?;
    let d = &value.document;
    if value.schema != "stdio.gltf" || d.asset.version != "2.0" || d.asset.min_version.as_deref().is_some_and(|v| v != "2.0") {
        return Err(invalid("geometry Pack schema or supported glTF version differs"));
    }
    omitted(
        !d.materials.is_empty()
            || !d.textures.is_empty()
            || !d.images.is_empty()
            || !d.samplers.is_empty()
            || !d.skins.is_empty()
            || !d.animations.is_empty()
            || !d.cameras.is_empty()
            || !d.extensions_used.is_empty()
            || !d.extensions_required.is_empty()
            || d.extensions.is_some()
            || d.extras.is_some()
            || d.asset.extensions.is_some()
            || d.asset.extras.is_some(),
    )?;
    for row in &d.scenes {
        work(owner, size_of::<GltfScene>())?;
        omitted(row.extensions.is_some() || row.extras.is_some())?;
    }
    for row in &d.nodes {
        work(owner, size_of::<GltfNode>())?;
        omitted(row.camera.is_some() || row.skin.is_some() || !row.weights.is_empty() || row.extensions.is_some() || row.extras.is_some())?;
    }
    for row in &d.meshes {
        work(owner, size_of::<GltfMesh>())?;
        omitted(!row.weights.is_empty() || row.extensions.is_some() || row.extras.is_some())?;
        for primitive in &row.primitives {
            work(owner, size_of::<GltfPrimitive>())?;
            omitted(primitive.material.is_some() || !primitive.targets.is_empty() || primitive.extensions.is_some() || primitive.extras.is_some())?;
        }
    }
    for row in &d.accessors {
        work(owner, size_of::<GltfAccessor>())?;
        omitted(row.sparse.is_some() || row.extensions.is_some() || row.extras.is_some())?;
    }
    for row in &d.buffer_views {
        work(owner, size_of::<GltfBufferView>())?;
        omitted(row.extensions.is_some() || row.extras.is_some())?;
    }
    for row in &d.buffers {
        work(owner, size_of::<GltfBuffer>())?;
        omitted(row.extensions.is_some() || row.extras.is_some())?;
    }
    if d.buffers.len() != value.buffers.len() {
        return Err(invalid("geometry Pack declared and resolved buffer counts differ"));
    }
    let mut extent = 0usize;
    for (row, bytes) in d.buffers.iter().zip(&value.buffers) {
        work(owner, size_of::<Vec<u8>>())?;
        if row.byte_length != bytes.len() {
            return Err(invalid("geometry Pack buffer extent differs"));
        }
        extent = extent.checked_add(bytes.len()).filter(|n| *n <= 64 * 1024 * 1024).ok_or_else(|| ValueError::literal(ValueRefusalKind::WorkLimit, "geometry Pack resolved bytes exceed 64 MiB admission"))?;
    }
    Ok(())
}
