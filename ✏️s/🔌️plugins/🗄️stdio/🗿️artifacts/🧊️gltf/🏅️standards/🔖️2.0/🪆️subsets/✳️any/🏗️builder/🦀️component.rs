//! 🏗️ GltfBuilder — local ArtifactBuilder until SDK Wave 3, plus typed glTF 2.0 document
//! constructors (accessor/bufferView/mesh/node plumbing). Ticket
//! ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D2 ground rules: "typed builder
//! constructors with accessor/bufferView plumbing in the standard-level builder" -- these are the
//! reconstruction primitives the metabolism fixture's analyzer→builder round-trip test drives.

use semio_framework_plugin::ArtifactBuilder;
use crate::artifacts::gltf::engine::{GltfAccessorType, GltfComponentType};
use crate::artifacts::gltf::{GltfDiff, GltfMutation, GltfSnapshot};
use serde_json::{json, Value};

//#region 🔖️Builder
/// 🏗️ Builds a `stdio.gltf` snapshot.
#[derive(Clone, Debug, Default)]
pub struct GltfBuilder {
    snapshot: GltfSnapshot,
    diagnostics: Vec<dsl::Diagnostic>,
}

impl ArtifactBuilder for GltfBuilder {
    type Snapshot = GltfSnapshot;
    type Mutation = GltfMutation;
    type Diff = GltfDiff;
    fn empty() -> Self {
        Self { snapshot: GltfSnapshot::default(), diagnostics: Vec::new() }
    }
    fn from_snapshot(snapshot: Self::Snapshot) -> Self {
        Self { snapshot, diagnostics: Vec::new() }
    }
    fn from_text(text: &str) -> Result<Self, store::TextError> {
        Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactDsl>::parse_dsl(text)?))
    }
    fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
        Ok(Self::from_snapshot(<GltfSnapshot as store::ArtifactPack>::decode_pack(bytes)?))
    }
    fn mutate(mut self, mutation: Self::Mutation) -> Self {
        crate::artifacts::gltf::schema::mutations::apply_gltf_mutation(&mut self.snapshot, &mutation);
        self
    }
    fn absorb(mut self, diff: Self::Diff) -> Self {
        self.snapshot = <GltfDiff as protocol::MutationDiff<GltfSnapshot>>::apply(&diff, &self.snapshot);
        self
    }
    fn build(self) -> Result<Self::Snapshot, Vec<dsl::Diagnostic>> {
        if self.diagnostics.is_empty() { Ok(self.snapshot) } else { Err(self.diagnostics) }
    }
}
//#endregion 🔖️Builder

//#region 🔖️AccessorSpec
/// 📐️ Everything `add_accessor` needs beyond the buffer-view-and-offset plumbing common to every
/// accessor. Built via `new` + chained `with_*` setters (values, not consuming-`Self` document
/// mutation -- this is a plain value type, not the builder itself).
#[derive(Clone, Debug)]
pub struct GltfAccessorSpec {
    pub buffer_view: Option<usize>,
    pub byte_offset: usize,
    pub component_type: GltfComponentType,
    pub accessor_type: GltfAccessorType,
    pub count: usize,
    pub normalized: bool,
    pub min: Option<Vec<f64>>,
    pub max: Option<Vec<f64>>,
}

impl GltfAccessorSpec {
    pub fn new(component_type: GltfComponentType, accessor_type: GltfAccessorType, count: usize) -> Self {
        Self { buffer_view: None, byte_offset: 0, component_type, accessor_type, count, normalized: false, min: None, max: None }
    }
    pub fn with_buffer_view(mut self, buffer_view: usize, byte_offset: usize) -> Self {
        self.buffer_view = Some(buffer_view);
        self.byte_offset = byte_offset;
        self
    }
    pub fn with_normalized(mut self, normalized: bool) -> Self {
        self.normalized = normalized;
        self
    }
    pub fn with_min_max(mut self, min: Vec<f64>, max: Vec<f64>) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }
}
//#endregion 🔖️AccessorSpec

//#region 🔖️DocumentConstructors
fn ensure_array<'a>(document: &'a mut Value, key: &str) -> &'a mut Vec<Value> {
    if !document.get(key).map(Value::is_array).unwrap_or(false) {
        document[key] = Value::Array(Vec::new());
    }
    document[key].as_array_mut().expect("just ensured array")
}

impl GltfBuilder {
    /// 🌱 Sets `asset.version` (the one glTF-mandatory field).
    pub fn set_asset_version(&mut self, version: &str) -> &mut Self {
        self.snapshot.document["asset"] = json!({ "version": version });
        self
    }

    /// 📦️ Appends a buffer, storing its real bytes on the snapshot's `buffers` (index-aligned with
    /// `document.buffers`) and recording `byteLength` in the document. Returns the new index.
    pub fn add_buffer(&mut self, bytes: Vec<u8>) -> usize {
        let byte_length = bytes.len();
        let idx = self.snapshot.buffers.len();
        self.snapshot.buffers.push(bytes);
        ensure_array(&mut self.snapshot.document, "buffers").push(json!({ "byteLength": byte_length }));
        idx
    }

    /// 🪟️ Appends a `bufferView` (buffer index, byte offset/length, optional `byteStride` for
    /// interleaved data, optional `target` -- 34962 `ARRAY_BUFFER` / 34963 `ELEMENT_ARRAY_BUFFER`).
    /// Returns the new index.
    pub fn add_buffer_view(&mut self, buffer: usize, byte_offset: usize, byte_length: usize, byte_stride: Option<usize>, target: Option<u64>) -> usize {
        let mut entry = json!({ "buffer": buffer, "byteLength": byte_length });
        if byte_offset != 0 {
            entry["byteOffset"] = json!(byte_offset);
        }
        if let Some(stride) = byte_stride {
            entry["byteStride"] = json!(stride);
        }
        if let Some(t) = target {
            entry["target"] = json!(t);
        }
        let views = ensure_array(&mut self.snapshot.document, "bufferViews");
        let idx = views.len();
        views.push(entry);
        idx
    }

    /// 🔢️ Appends an `accessor` from a typed [`GltfAccessorSpec`]. Returns the new index.
    pub fn add_accessor(&mut self, spec: GltfAccessorSpec) -> usize {
        let mut entry = json!({
            "componentType": spec.component_type.code(),
            "type": spec.accessor_type.as_str(),
            "count": spec.count,
        });
        if let Some(bv) = spec.buffer_view {
            entry["bufferView"] = json!(bv);
            if spec.byte_offset != 0 {
                entry["byteOffset"] = json!(spec.byte_offset);
            }
        }
        if spec.normalized {
            entry["normalized"] = json!(true);
        }
        if let Some(min) = &spec.min {
            entry["min"] = json!(min);
        }
        if let Some(max) = &spec.max {
            entry["max"] = json!(max);
        }
        let accessors = ensure_array(&mut self.snapshot.document, "accessors");
        let idx = accessors.len();
        accessors.push(entry);
        idx
    }

    /// 🎨️ Appends a `material` (kept as raw JSON -- PBR material shape has too many optional
    /// KHR-extension-carrying corners to justify a bespoke typed constructor in this wave; the
    /// caller already has the real material `Value`, e.g. read straight off another document via
    /// the analyzer, and this just re-inserts it verbatim). Returns the new index.
    pub fn add_material(&mut self, material: Value) -> usize {
        let materials = ensure_array(&mut self.snapshot.document, "materials");
        let idx = materials.len();
        materials.push(material);
        idx
    }

    /// 🕸️ Appends an empty `mesh` (primitives added via [`Self::add_mesh_primitive`]). Returns the
    /// new index.
    pub fn add_mesh(&mut self) -> usize {
        let meshes = ensure_array(&mut self.snapshot.document, "meshes");
        let idx = meshes.len();
        meshes.push(json!({ "primitives": [] }));
        idx
    }

    /// 🔺️ Appends a primitive to `meshes[mesh]` -- `attributes` are `(semantic, accessor index)`
    /// pairs (e.g. `("POSITION", 0)`), `indices`/`material` are optional accessor/material
    /// indices, `mode` is the primitive topology (defaults to `4` TRIANGLES per spec when unset).
    pub fn add_mesh_primitive(&mut self, mesh: usize, attributes: &[(&str, usize)], indices: Option<usize>, material: Option<usize>, mode: Option<u64>) {
        let mut attrs = serde_json::Map::new();
        for (name, idx) in attributes {
            attrs.insert((*name).to_string(), json!(idx));
        }
        let mut primitive = json!({ "attributes": Value::Object(attrs) });
        if let Some(i) = indices {
            primitive["indices"] = json!(i);
        }
        if let Some(m) = material {
            primitive["material"] = json!(m);
        }
        if let Some(m) = mode {
            primitive["mode"] = json!(m);
        }
        let meshes = self.snapshot.document["meshes"].as_array_mut().expect("mesh array must exist -- call add_mesh first");
        let mesh_entry = meshes.get_mut(mesh).expect("mesh index out of range");
        mesh_entry["primitives"].as_array_mut().expect("mesh.primitives array").push(primitive);
    }

    /// 🧍️ Appends a `node`, optionally referencing a mesh. Returns the new index.
    pub fn add_node(&mut self, mesh: Option<usize>) -> usize {
        let mut entry = json!({});
        if let Some(m) = mesh {
            entry["mesh"] = json!(m);
        }
        let nodes = ensure_array(&mut self.snapshot.document, "nodes");
        let idx = nodes.len();
        nodes.push(entry);
        idx
    }

    /// 🎬️ Appends a `scene` referencing `nodes` (root node indices), with an optional passthrough
    /// `extensions` object (real documents sometimes carry a declared-but-empty `{}` here).
    /// Returns the new index.
    pub fn add_scene(&mut self, nodes: Vec<usize>, extensions: Option<Value>) -> usize {
        let mut entry = json!({ "nodes": nodes });
        if let Some(ext) = extensions {
            entry["extensions"] = ext;
        }
        let scenes = ensure_array(&mut self.snapshot.document, "scenes");
        let idx = scenes.len();
        scenes.push(entry);
        idx
    }

    /// 🎬️ Sets the document's default `scene` index.
    pub fn set_default_scene(&mut self, scene: usize) -> &mut Self {
        self.snapshot.document["scene"] = json!(scene);
        self
    }

    /// 🧩️ Sets `extensionsUsed` (declared, not necessarily applied -- mirrors real-world documents
    /// that declare an extension namespace without every element using it).
    pub fn set_extensions_used(&mut self, names: Vec<String>) -> &mut Self {
        self.snapshot.document["extensionsUsed"] = json!(names);
        self
    }

    /// 📸️ Peeks the in-progress document -- used by tests/callers that need to inspect state
    /// mid-construction without consuming the builder via `build()`.
    pub fn document(&self) -> &Value {
        &self.snapshot.document
    }

    /// 📦️ Peeks the in-progress resolved buffer bytes.
    pub fn buffers(&self) -> &[Vec<u8>] {
        &self.snapshot.buffers
    }
}
//#endregion 🔖️DocumentConstructors

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn typed_constructors_build_a_decodable_triangle() {
        let mut b = GltfBuilder::empty();
        b.set_asset_version("2.0");
        let mut bytes = Vec::new();
        let verts: [[f32; 3]; 3] = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        for v in verts { for c in v { bytes.extend_from_slice(&c.to_le_bytes()); } }
        let buf = b.add_buffer(bytes);
        let bv = b.add_buffer_view(buf, 0, 36, None, Some(34962));
        let acc = b.add_accessor(GltfAccessorSpec::new(GltfComponentType::Float, GltfAccessorType::Vec3, 3).with_buffer_view(bv, 0).with_min_max(vec![0.0, 0.0, 0.0], vec![1.0, 1.0, 0.0]));
        let mat = b.add_material(json!({ "pbrMetallicRoughness": { "baseColorFactor": [1.0, 0.0, 0.0, 1.0] } }));
        let mesh = b.add_mesh();
        b.add_mesh_primitive(mesh, &[("POSITION", acc)], None, Some(mat), None);
        let node = b.add_node(Some(mesh));
        let scene = b.add_scene(vec![node], None);
        b.set_default_scene(scene);
        let snapshot = b.build().expect("build");

        assert_eq!(snapshot.document["asset"]["version"], "2.0");
        let decoded = crate::artifacts::gltf::engine::decode_accessor(&snapshot.document, &snapshot.buffers, acc).expect("decode");
        assert_eq!(decoded.components, vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
    }
}
//#endregion 🧪️Tests
