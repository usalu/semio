//! 🔢️ Borrowed GLB accessor sample locations and sorted sparse overrides.
use super::{Buffers, SceneResult, field, index, rows};
use super::super::{gltf_component_byte_size, gltf_normalize_components, gltf_read_component, json::Value};
use semio_framework_value::RetainedAllocationEffect;

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
struct Rows {
    buffer: usize,
    base: usize,
    stride: usize,
    component: u64,
    sample_size: usize,
}

impl Rows {
    fn layout(document: &Value, buffers: Buffers<'_>, view: usize, offset: usize, component: u64, components: usize, count: usize) -> SceneResult<Self> {
        let view = rows(document, "bufferViews").get(view).ok_or("glb: buffer view outside document")?;
        let buffer = index(view, "buffer").unwrap_or(0);
        let extent = buffers.buffer(buffer)?.len();
        let sample_size = gltf_component_byte_size(component)?;
        let width = sample_size * components;
        let base = index(view, "byteOffset").unwrap_or(0).checked_add(offset).ok_or("glb: accessor base overflow")?;
        let stride = index(view, "byteStride").unwrap_or(width);
        if extent == 0 { return Err("glb: accessor outside embedded buffer".into()); }
        if count != 0 {
            let span = (count - 1).checked_mul(stride).and_then(|last| last.checked_add(width)).ok_or("glb: accessor extent overflow")?;
            let declared = index(view, "byteLength").unwrap_or(usize::MAX);
            if base.checked_add(span).is_none_or(|end| end > extent)
                || offset.checked_add(span).is_none_or(|end| end > declared)
            {
                return Err("glb: accessor outside embedded buffer view".into());
            }
        }
        Ok(Self { buffer, base, stride, component, sample_size })
    }

    fn sample<const N: usize>(&self, buffers: Buffers<'_>, row: usize) -> SceneResult<[f64; N]> {
        let base = self.base + row * self.stride;
        let buffer = buffers.buffer(self.buffer)?;
        let mut scratch = [0u8; 12];
        let bytes = buffer.read(base, N * self.sample_size, &mut scratch)?;
        let mut sample = [0.0; N];
        for (i, value) in sample.iter_mut().enumerate() {
            *value = gltf_read_component(self.component, bytes, i * self.sample_size)?;
        }
        Ok(sample)
    }
}

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
struct Sparse {
    indices: Rows,
    values: Rows,
    count: usize,
}

impl Sparse {
    fn layout(document: &Value, buffers: Buffers<'_>, value: &Value, component: u64, components: usize, maximum: usize) -> SceneResult<Self> {
        let count = index(value, "count").filter(|count| *count <= maximum).ok_or("glb: invalid sparse accessor count")?;
        let indices = field(value, "indices").ok_or("glb: missing sparse indices")?;
        let values = field(value, "values").ok_or("glb: missing sparse values")?;
        let index_type = field(indices, "componentType").and_then(Value::as_u64)
            .filter(|value| matches!(*value, 5121 | 5123 | 5125)).ok_or("glb: invalid sparse index component type")?;
        let index_view = index(indices, "bufferView").ok_or("glb: missing sparse index view")?;
        let value_view = index(values, "bufferView").ok_or("glb: missing sparse value view")?;
        let index_offset = index(indices, "byteOffset").unwrap_or(0);
        let value_offset = index(values, "byteOffset").unwrap_or(0);
        let indices = Rows::layout(document, buffers, index_view, index_offset, index_type, 1, count)?;
        let values = Rows::layout(document, buffers, value_view, value_offset, component, components, count)?;
        Ok(Self { indices, values, count })
    }

    fn index(&self, buffers: Buffers<'_>, row: usize) -> SceneResult<usize> {
        Ok(self.indices.sample::<1>(buffers, row)?[0] as usize)
    }

    fn lookup(&self, buffers: Buffers<'_>, row: usize, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<Option<usize>> {
        let (mut start, mut end) = (0usize, self.count);
        while start < end {
            if !effect.performed(1).map_err(|error| error.to_string())? {
                return Err("glb: sparse lookup reached unavailable Work".into());
            }
            let mid = start + (end - start) / 2;
            match self.index(buffers, mid)?.cmp(&row) {
                std::cmp::Ordering::Less => start = mid + 1,
                std::cmp::Ordering::Equal => return Ok(Some(mid)),
                std::cmp::Ordering::Greater => end = mid,
            }
        }
        Ok(None)
    }
}

#[derive(Clone, Copy, semio_framework_value_derive::RetireOwned)]
pub(super) struct Accessor {
    base: Option<Rows>,
    sparse: Option<Sparse>,
    pub(super) count: usize,
    pub(super) component: u64,
    pub(super) normalized: bool,
}

impl Accessor {
    pub(super) fn layout(document: &Value, buffers: Buffers<'_>, accessor: usize, kind: &str, components: usize) -> SceneResult<Self> {
        let value = rows(document, "accessors").get(accessor).ok_or("glb: accessor index outside document")?;
        if field(value, "type").and_then(Value::as_str) != Some(kind) { return Err(format!("glb: expected {kind} accessor")); }
        let component = field(value, "componentType").and_then(Value::as_u64).ok_or("glb: missing accessor component type")?;
        let count = index(value, "count").ok_or("glb: missing accessor count")?;
        let normalized = field(value, "normalized").and_then(Value::as_bool).unwrap_or(false);
        let base = if let Some(view) = index(value, "bufferView") {
            let offset = index(value, "byteOffset").unwrap_or(0);
            let layout = Rows::layout(document, buffers, view, offset, component, components, count)?;
            Some(layout)
        } else { None };
        let sparse = if let Some(sparse) = field(value, "sparse") {
            let layout = Sparse::layout(document, buffers, sparse, component, components, count)?;
            Some(layout)
        } else { None };
        Ok(Self { base, sparse, count, component, normalized })
    }

    pub(super) fn sparse_count(&self) -> usize { self.sparse.as_ref().map_or(0, |state| state.count) }

    pub(super) fn sparse_index(&self, buffers: Buffers<'_>, row: usize) -> SceneResult<usize> {
        self.sparse.as_ref().ok_or("glb: missing sparse layout")?.index(buffers, row)
    }

    pub(super) fn lookup_work(&self) -> u64 {
        u64::from(usize::BITS - self.sparse_count().leading_zeros())
    }

    pub(super) fn sample<const N: usize>(&self, buffers: Buffers<'_>, row: usize, effect: &mut RetainedAllocationEffect<'_>) -> SceneResult<[f64; N]> {
        let selected = match self.sparse.as_ref() {
            Some(sparse) => sparse.lookup(buffers, row, effect)?.map(|index| (&sparse.values, index)),
            None => None,
        };
        let mut sample = selected.or(self.base.as_ref().map(|base| (base, row)))
            .map(|(source, index)| source.sample(buffers, index)).transpose()?.unwrap_or([0.0; N]);
        if self.normalized {
            gltf_normalize_components(self.component, &mut sample)?;
        }
        Ok(sample)
    }
}
