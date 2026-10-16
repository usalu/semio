//! 🦴️ GLB decode: JSON lexer, schema structs and cursor, instance plan, math, outline and materialize cursors for renderer mesh assets.
use super::*;

//#region 📡️RendererAssetAuthority
const GLB_JSON_ATOM_BYTES: usize = 64;
const GLB_SCHEMA_ITEM_CAPACITY: usize = 512;
const GLB_SCHEMA_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy)]
struct GlbJsonAtom {
    bytes: [u8; GLB_JSON_ATOM_BYTES],
    len: u8,
    total: u32,
    overflow: bool,
}

impl GlbJsonAtom {
    fn new() -> Self {
        Self { bytes: [0; GLB_JSON_ATOM_BYTES], len: 0, total: 0, overflow: false }
    }

    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        self.total = self.total.checked_add(1).ok_or("GLB JSON atom length overflowed")?;
        if usize::from(self.len) < self.bytes.len() {
            self.bytes[usize::from(self.len)] = byte;
            self.len += 1;
        } else {
            self.overflow = true;
        }
        Ok(())
    }

    fn equals(&self, expected: &str) -> bool {
        !self.overflow && usize::from(self.len) == expected.len() && &self.bytes[..usize::from(self.len)] == expected.as_bytes()
    }

    fn unsigned(&self) -> Result<u64, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON integer exceeded fixed atom credits");
        }
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse().ok()).ok_or("GLB JSON integer was invalid")
    }

    fn float(&self) -> Result<f32, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON number exceeded fixed atom credits");
        }
        let value = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse::<f32>().ok()).ok_or("GLB JSON number was invalid")?;
        value.is_finite().then_some(value).ok_or("GLB JSON number was not finite")
    }
}

enum GlbJsonToken {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String(GlbJsonAtom),
    Number(GlbJsonAtom),
    Literal(Option<bool>),
}

enum GlbJsonLexPhase {
    Idle,
    String { atom: GlbJsonAtom, escape: bool, unicode_remaining: u8 },
    Number(GlbJsonAtom),
    Literal { expected: &'static [u8], index: u8, value: Option<bool> },
}

struct GlbJsonTokenCursor {
    phase: GlbJsonLexPhase,
    replay: Option<u8>,
}

impl GlbJsonTokenCursor {
    fn new() -> Self {
        Self { phase: GlbJsonLexPhase::Idle, replay: None }
    }

    fn feed(&mut self, byte: u8) -> Result<Option<GlbJsonToken>, &'static str> {
        let byte = self.replay.take().unwrap_or(byte);
        match &mut self.phase {
            GlbJsonLexPhase::Idle => match byte {
                b' ' | b'\n' | b'\r' | b'\t' => Ok(None),
                b'{' => Ok(Some(GlbJsonToken::ObjectStart)),
                b'}' => Ok(Some(GlbJsonToken::ObjectEnd)),
                b'[' => Ok(Some(GlbJsonToken::ArrayStart)),
                b']' => Ok(Some(GlbJsonToken::ArrayEnd)),
                b':' => Ok(Some(GlbJsonToken::Colon)),
                b',' => Ok(Some(GlbJsonToken::Comma)),
                b'"' => {
                    self.phase = GlbJsonLexPhase::String { atom: GlbJsonAtom::new(), escape: false, unicode_remaining: 0 };
                    Ok(None)
                }
                b'-' | b'0'..=b'9' => {
                    let mut atom = GlbJsonAtom::new();
                    atom.push(byte)?;
                    self.phase = GlbJsonLexPhase::Number(atom);
                    Ok(None)
                }
                b't' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"true", index: 1, value: Some(true) };
                    Ok(None)
                }
                b'f' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"false", index: 1, value: Some(false) };
                    Ok(None)
                }
                b'n' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"null", index: 1, value: None };
                    Ok(None)
                }
                _ => Err("GLB JSON token was invalid"),
            },
            GlbJsonLexPhase::String { atom, escape, unicode_remaining } => {
                if *unicode_remaining != 0 {
                    if !byte.is_ascii_hexdigit() {
                        return Err("GLB JSON unicode escape was invalid");
                    }
                    *unicode_remaining -= 1;
                    if *unicode_remaining == 0 {
                        atom.push(b'?')?;
                    }
                    return Ok(None);
                }
                if *escape {
                    *escape = false;
                    match byte {
                        b'"' | b'\\' | b'/' => atom.push(byte)?,
                        b'b' => atom.push(8)?,
                        b'f' => atom.push(12)?,
                        b'n' => atom.push(b'\n')?,
                        b'r' => atom.push(b'\r')?,
                        b't' => atom.push(b'\t')?,
                        b'u' => *unicode_remaining = 4,
                        _ => return Err("GLB JSON escape was invalid"),
                    }
                    return Ok(None);
                }
                match byte {
                    b'\\' => {
                        *escape = true;
                        Ok(None)
                    }
                    b'"' => {
                        let atom = *atom;
                        self.phase = GlbJsonLexPhase::Idle;
                        Ok(Some(GlbJsonToken::String(atom)))
                    }
                    0..=31 => Err("GLB JSON string contained a control byte"),
                    _ => {
                        atom.push(byte)?;
                        Ok(None)
                    }
                }
            }
            GlbJsonLexPhase::Number(atom) => {
                if matches!(byte, b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                    atom.push(byte)?;
                    Ok(None)
                } else {
                    let atom = *atom;
                    self.phase = GlbJsonLexPhase::Idle;
                    self.replay = Some(byte);
                    Ok(Some(GlbJsonToken::Number(atom)))
                }
            }
            GlbJsonLexPhase::Literal { expected, index, value } => {
                if expected.get(usize::from(*index)).copied() != Some(byte) {
                    return Err("GLB JSON literal was invalid");
                }
                *index += 1;
                if usize::from(*index) == expected.len() {
                    let value = *value;
                    self.phase = GlbJsonLexPhase::Idle;
                    Ok(Some(GlbJsonToken::Literal(value)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn replay(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        let Some(byte) = self.replay else { return Ok(None) };
        self.feed(byte)
    }

    fn finish(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        match std::mem::replace(&mut self.phase, GlbJsonLexPhase::Idle) {
            GlbJsonLexPhase::Idle => self.replay(),
            GlbJsonLexPhase::Number(atom) => Ok(Some(GlbJsonToken::Number(atom))),
            _ => Err("GLB JSON token ended before terminal"),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbAccessorSchema {
    view: u16,
    byte_offset: u32,
    component: u16,
    count: u32,
    kind: u8,
    view_set: bool,
    normalized: bool,
}

#[derive(Clone, Copy, Default)]
struct GlbViewSchema {
    byte_offset: u32,
    byte_length: u32,
    byte_stride: u16,
}

/// 🎯️ The custom glTF attribute that carries one face id per vertex; a triangle's face id is its first corner's. Pinned by
/// `🧫️fixtures/📏️world3d-modelling/🔣️.json` (`glbSubElementIds`).
const GLB_FACE_ID_ATTRIBUTE: &str = "_FACE_ID";
/// 🎯️ The custom glTF attribute that carries one vertex id per vertex.
const GLB_VERTEX_ID_ATTRIBUTE: &str = "_VERTEX_ID";

#[derive(Clone, Copy)]
struct GlbPrimitiveSchema {
    position: u16,
    normal: Option<u16>,
    uv_sets: [Option<u16>;64],
    tangent:Option<u16>,
    color: Option<u16>,
    indices: Option<u16>,
    material: Option<u16>,
    mode: u8,
    position_set: bool,
    face_id: Option<u16>,
    vertex_id: Option<u16>,
}

#[derive(Clone, Copy)]
struct GlbMaterialSchema {
    base_color: [f32; 4],
    emissive: [f32; 3],
    metalness: f32,
    roughness: f32,
    alpha_mode: u8,
    alpha_cutoff: f32,
    double_sided: bool,
    base_color_texture: Option<u16>,
    additional_textures: [Option<u16>;4],
    texture_coordinates:[u8;5],
    normal_scale:f32,
    occlusion_strength:f32,
}

impl Default for GlbMaterialSchema {
    fn default() -> Self {
        Self { base_color: [1.0; 4], emissive: [0.0; 3], metalness: 1.0, roughness: 1.0, alpha_mode: 0, alpha_cutoff: 0.5, double_sided: false, base_color_texture: None, additional_textures: [None;4],texture_coordinates:[0;5],normal_scale:1.0,occlusion_strength:1.0 }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbTextureSchema {
    sampler: Option<u16>,
    source: Option<u16>,
}

#[derive(Clone, Copy, Default)]
struct GlbImageSchema {
    view: Option<u16>,
    mime: u8,
}

#[derive(Clone, Copy)]
struct GlbSamplerSchema {
    wrap_s: u16,
    wrap_t: u16,
    mag_filter: Option<u16>,
    min_filter: Option<u16>,
}

impl Default for GlbSamplerSchema {
    fn default() -> Self {
        Self { wrap_s: 10497, wrap_t: 10497, mag_filter: None, min_filter: None }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbMeshSchema {
    primitive_start: u16,
    primitive_len: u16,
}

#[derive(Clone, Copy)]
struct GlbNodeSchema {
    mesh: Option<u16>,
    child_start: u16,
    child_len: u16,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
    matrix: Option<[f32; 16]>,
}

impl Default for GlbNodeSchema {
    fn default() -> Self {
        Self { mesh: None, child_start: 0, child_len: 0, translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: [1.0; 3], matrix: None }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbSceneSchema {
    node_start: u16,
    node_len: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GlbNumericArrayKind {
    NodeChildren,
    NodeTranslation,
    NodeRotation,
    NodeScale,
    NodeMatrix,
    SceneNodes,
    MaterialBaseColor,
    MaterialEmissive,
}

struct GlbNumericArray {
    kind: GlbNumericArrayKind,
    depth: u16,
    start: u16,
    values: [f32; 16],
    len: u16,
}

impl GlbNumericArray {
    fn new(kind: GlbNumericArrayKind, depth: u16, start: u16) -> Self {
        Self { kind, depth, start, values: [0.0; 16], len: 0 }
    }

    fn push(&mut self, value: f32) -> Result<(), &'static str> {
        let slot = usize::from(self.len);
        if slot == self.values.len() {
            return Err("GLB node or scene array exceeded fixed item credits");
        }
        self.values[slot] = value;
        self.len += 1;
        Ok(())
    }
}

impl Default for GlbPrimitiveSchema {
    fn default() -> Self {
        Self { position: 0, normal: None, uv_sets: [None;64],tangent:None, color: None, indices: None, material: None, mode: 4, position_set: false, face_id: None, vertex_id: None }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GlbSchemaSection {
    None,
    Accessors,
    Views,
    Meshes,
    Nodes,
    Scenes,
    Materials,
    Textures,
    Images,
    Samplers,
}

struct GlbSchemaOutput {
    accessors: Box<[Option<GlbAccessorSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    views: Box<[Option<GlbViewSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    primitives: Box<[Option<GlbPrimitiveSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    materials: Box<[Option<GlbMaterialSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    textures: Box<[Option<GlbTextureSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    images: Box<[Option<GlbImageSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    samplers: Box<[Option<GlbSamplerSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    meshes: Box<[Option<GlbMeshSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    nodes: Box<[Option<GlbNodeSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    scenes: Box<[Option<GlbSceneSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    node_edges: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    scene_roots: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    accessor_len: u16,
    view_len: u16,
    primitive_len: u16,
    material_len: u16,
    texture_len: u16,
    image_len: u16,
    sampler_len: u16,
    mesh_len: u16,
    node_len: u16,
    scene_len: u16,
    node_edge_len: u16,
    scene_root_len: u16,
    default_scene: Option<u16>,
    json_bytes: u32,
    output_bytes: usize,
}

impl GlbSchemaOutput {
    fn new() -> Self {
        Self {
            accessors: semio_framework_async::boxed_fixed_slots(|| None),
            views: semio_framework_async::boxed_fixed_slots(|| None),
            primitives: semio_framework_async::boxed_fixed_slots(|| None),
            materials: semio_framework_async::boxed_fixed_slots(|| None),
            textures: semio_framework_async::boxed_fixed_slots(|| None),
            images: semio_framework_async::boxed_fixed_slots(|| None),
            samplers: semio_framework_async::boxed_fixed_slots(|| None),
            meshes: semio_framework_async::boxed_fixed_slots(|| None),
            nodes: semio_framework_async::boxed_fixed_slots(|| None),
            scenes: semio_framework_async::boxed_fixed_slots(|| None),
            node_edges: semio_framework_async::boxed_fixed_slots(|| None),
            scene_roots: semio_framework_async::boxed_fixed_slots(|| None),
            accessor_len: 0,
            view_len: 0,
            primitive_len: 0,
            material_len: 0,
            texture_len: 0,
            image_len: 0,
            sampler_len: 0,
            mesh_len: 0,
            node_len: 0,
            scene_len: 0,
            node_edge_len: 0,
            scene_root_len: 0,
            default_scene: None,
            json_bytes: 0,
            output_bytes: 0,
        }
    }

    fn validate(&mut self) -> Result<(), &'static str> {
        if let Some(scene) = self.default_scene {
            if usize::from(scene) >= usize::from(self.scene_len) {
                return Err("GLB default scene referenced a missing scene");
            }
        }
        for node in self.nodes[..usize::from(self.node_len)].iter().flatten() {
            if node.mesh.is_some_and(|mesh| usize::from(mesh) >= usize::from(self.mesh_len)) {
                return Err("GLB node referenced a missing mesh");
            }
            let start = usize::from(node.child_start);
            let end = start.checked_add(usize::from(node.child_len)).ok_or("GLB child range overflowed")?;
            if end > usize::from(self.node_edge_len) || self.node_edges[start..end].iter().flatten().any(|child| usize::from(*child) >= usize::from(self.node_len)) {
                return Err("GLB node referenced a missing child");
            }
        }
        for scene in self.scenes[..usize::from(self.scene_len)].iter().flatten() {
            let start = usize::from(scene.node_start);
            let end = start.checked_add(usize::from(scene.node_len)).ok_or("GLB scene root range overflowed")?;
            if end > usize::from(self.scene_root_len) || self.scene_roots[start..end].iter().flatten().any(|node| usize::from(*node) >= usize::from(self.node_len)) {
                return Err("GLB scene referenced a missing root node");
            }
        }
        let mut output_bytes = 0usize;
        for primitive in self.primitives[..usize::from(self.primitive_len)].iter().flatten() {
            if !matches!(primitive.mode, 4..=6) {
                continue;
            }
            let position = self.accessor(primitive.position)?;
            if primitive.material.is_some_and(|material| material >= self.material_len) {
                return Err("GLB primitive referenced a missing material");
            }
            if let Some(material)=primitive.material.and_then(|index|self.materials[usize::from(index)].as_ref()) {
                for (role,reference) in std::iter::once(material.base_color_texture).chain(material.additional_textures).enumerate() {
                    if reference.is_some() && primitive.uv_sets[usize::from(material.texture_coordinates[role])].is_none(){return Err("GLB texture references an absent UV set");}
                }
            }
            if position.component != 5126 || position.kind != 3 {
                return Err("GLB POSITION accessor was not FLOAT VEC3");
            }
            self.validate_accessor_span(position)?;
            let position_bytes = usize::try_from(position.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB vertex output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB vertex output bytes overflowed")?;
            if let Some(normal) = primitive.normal {
                let normal = self.accessor(normal)?;
                if !matches!(normal.component, 5126 | 5120 | 5122) || (normal.component != 5126 && !normal.normalized) || normal.kind != 3 || normal.count != position.count {
                    return Err("GLB NORMAL accessor did not match POSITION");
                }
                self.validate_accessor_span(normal)?;
                let normal_bytes = usize::try_from(normal.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB normal output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(normal_bytes).ok_or("GLB normal output bytes overflowed")?;
            } else {
                output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB generated normal bytes overflowed")?;
            }
            if let Some(tangent)=primitive.tangent {let tangent=self.accessor(tangent)?;if tangent.component!=5126 || tangent.kind!=4 || tangent.count!=position.count || primitive.normal.is_none(){return Err("GLB TANGENT must be FLOAT VEC4 aligned with POSITION and NORMAL");}self.validate_accessor_span(tangent)?;output_bytes=output_bytes.checked_add(tangent.count as usize*16).ok_or("GLB tangent output bytes overflowed")?;}
            for uv in primitive.uv_sets.into_iter().flatten() {
                let uv = self.accessor(uv)?;
                if !matches!(uv.component, 5126 | 5121 | 5123) || (uv.component != 5126 && !uv.normalized) || uv.kind != 2 || uv.count != position.count {
                    return Err("GLB TEXCOORD accessor did not match POSITION");
                }
                self.validate_accessor_span(uv)?;
                let uv_bytes = usize::try_from(uv.count).ok().and_then(|count| count.checked_mul(2)).and_then(|count| count.checked_mul(size_of::<f32>())).ok_or("GLB UV output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(uv_bytes).ok_or("GLB UV output bytes overflowed")?;
            }
            if let Some(color) = primitive.color {
                let color = self.accessor(color)?;
                if !matches!(color.kind, 3 | 4) || !matches!(color.component, 5121 | 5123 | 5126) || (color.component != 5126 && !color.normalized) || color.count != position.count {
                    return Err("GLB COLOR_0 accessor had an unsupported shape");
                }
                self.validate_accessor_span(color)?;
            }
            for (id, detail) in [(primitive.face_id, "GLB _FACE_ID accessor must be SCALAR UNSIGNED_INT aligned with POSITION"), (primitive.vertex_id, "GLB _VERTEX_ID accessor must be SCALAR UNSIGNED_INT aligned with POSITION")] {
                let Some(id) = id else { continue };
                let id = self.accessor(id)?;
                if id.component != 5125 || id.kind != 1 || id.normalized || id.count != position.count {
                    return Err(detail);
                }
                self.validate_accessor_span(id)?;
            }
            let source_indices = match primitive.indices {
                Some(indices) => {
                    let indices = self.accessor(indices)?;
                    if !matches!(indices.component, 5121 | 5123 | 5125) || indices.kind != 1 || indices.normalized {
                        return Err("GLB index accessor used an unsupported component or shape");
                    }
                    self.validate_accessor_span(indices)?;
                    indices.count
                }
                None => position.count,
            };
            let triangle_indices = match primitive.mode {
                4 => source_indices,
                5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
                _ => 0,
            };
            if triangle_indices == 0 || !triangle_indices.is_multiple_of(3) {
                return Err("GLB triangle primitive had no complete triangles");
            }
            let index_bytes = usize::try_from(triangle_indices).ok().and_then(|count| count.checked_mul(size_of::<u32>())).ok_or("GLB index output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(index_bytes).ok_or("GLB index output bytes overflowed")?;
            let id_bytes = (if primitive.face_id.is_some() { usize::try_from(triangle_indices / 3).ok() } else { Some(0) }).and_then(|triangles| if primitive.vertex_id.is_some() { usize::try_from(position.count).ok().and_then(|vertices| vertices.checked_add(triangles)) } else { Some(triangles) }).and_then(|count| count.checked_mul(size_of::<u32>())).ok_or("GLB sub-element id output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(id_bytes).ok_or("GLB sub-element id output bytes overflowed")?;
            if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
                return Err("GLB semantic output exceeded fixed byte credits");
            }
        }
        for material in self.materials[..usize::from(self.material_len)].iter().flatten() {
            if material.base_color.iter().chain(material.emissive.iter()).any(|value| !value.is_finite()) || !material.metalness.is_finite() || !material.roughness.is_finite() || !material.alpha_cutoff.is_finite() || !material.normal_scale.is_finite() || !material.occlusion_strength.is_finite() || !(0.0..=1.0).contains(&material.occlusion_strength) {
                return Err("GLB material contained a non-finite scalar");
            }
            if std::iter::once(material.base_color_texture).chain(material.additional_textures).flatten().any(|texture|texture >= self.texture_len) {
                return Err("GLB material referenced a missing texture");
            }
        }
        for texture in self.textures[..usize::from(self.texture_len)].iter().flatten() {
            if texture.source.is_none_or(|image| image >= self.image_len) || texture.sampler.is_some_and(|sampler| sampler >= self.sampler_len) {
                return Err("GLB texture referenced a missing image or sampler");
            }
        }
        for image in self.images[..usize::from(self.image_len)].iter().flatten() {
            if image.view.is_none_or(|view| view >= self.view_len) || image.mime == 0 {
                return Err("GLB image omitted an embedded bufferView or supported MIME type");
            }
        }
        for sampler in self.samplers[..usize::from(self.sampler_len)].iter().flatten() {
            glb_texture_wrap(sampler.wrap_s)?;
            glb_texture_wrap(sampler.wrap_t)?;
            glb_mag_filter(sampler.mag_filter)?;
            glb_min_filter(sampler.min_filter)?;
        }
        if output_bytes == 0 {
            return Err("GLB schema contained no triangle primitive output");
        }
        self.output_bytes = output_bytes;
        Ok(())
    }

    fn accessor(&self, index: u16) -> Result<&GlbAccessorSchema, &'static str> {
        self.accessors.get(usize::from(index)).and_then(Option::as_ref).ok_or("GLB primitive referenced a missing accessor")
    }

    fn validate_accessor_span(&self, accessor: &GlbAccessorSchema) -> Result<(), &'static str> {
        if !accessor.view_set {
            return Err("GLB sparse or viewless accessor was unsupported");
        }
        let view = self.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor referenced a missing bufferView")?;
        let components = usize::from(accessor.kind);
        let component_bytes = match accessor.component {
            5120 | 5121 => 1,
            5122 | 5123 => 2,
            5125 | 5126 => 4,
            _ => return Err("GLB accessor component type was unsupported"),
        };
        let item_bytes = components.checked_mul(component_bytes).ok_or("GLB accessor item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        if stride < item_bytes {
            return Err("GLB bufferView stride was smaller than its accessor item");
        }
        let span = usize::try_from(accessor.count.saturating_sub(1)).ok().and_then(|count| count.checked_mul(stride)).and_then(|bytes| bytes.checked_add(item_bytes)).and_then(|bytes| bytes.checked_add(accessor.byte_offset as usize));
        if span.is_none_or(|span| span > view.byte_length as usize) {
            return Err("GLB accessor exceeded its bufferView");
        }
        Ok(())
    }
}

pub struct GlbSchemaCursor {
    page: RendererAssetPageCursor,
    header: [u8; 20],
    header_len: u8,
    json_remaining: u32,
    lexer: GlbJsonTokenCursor,
    depth: u16,
    section: GlbSchemaSection,
    last_string: Option<GlbJsonAtom>,
    pending_key: Option<GlbJsonAtom>,
    expecting_value: bool,
    current_accessor: Option<GlbAccessorSchema>,
    current_view: Option<GlbViewSchema>,
    current_primitive: Option<GlbPrimitiveSchema>,
    current_mesh: Option<GlbMeshSchema>,
    current_node: Option<GlbNodeSchema>,
    current_scene: Option<GlbSceneSchema>,
    current_material: Option<GlbMaterialSchema>,
    current_texture: Option<GlbTextureSchema>,
    current_image: Option<GlbImageSchema>,
    current_sampler: Option<GlbSamplerSchema>,
    numeric_array: Option<GlbNumericArray>,
    primitives_depth: Option<u16>,
    attributes_depth: Option<u16>,
    pbr_depth: Option<u16>,
    base_color_texture_depth: Option<u16>,
    material_texture_role: usize,
    pub output: GlbSchemaOutput,
    terminal: bool,
}

impl GlbSchemaCursor {
    pub fn new() -> Self {
        Self {
            page: RendererAssetPageCursor::new(),
            header: [0; 20],
            header_len: 0,
            json_remaining: 0,
            lexer: GlbJsonTokenCursor::new(),
            depth: 0,
            section: GlbSchemaSection::None,
            last_string: None,
            pending_key: None,
            expecting_value: false,
            current_accessor: None,
            current_view: None,
            current_primitive: None,
            current_mesh: None,
            current_node: None,
            current_scene: None,
            current_material: None,
            current_texture: None,
            current_image: None,
            current_sampler: None,
            numeric_array: None,
            primitives_depth: None,
            attributes_depth: None,
            pbr_depth: None,
            base_color_texture_depth: None,
            material_texture_role: 0,
            output: GlbSchemaOutput::new(),
            terminal: false,
        }
    }

    pub fn step(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.header_len < 20 {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB semantic header was truncated") };
            self.header[usize::from(self.header_len)] = byte;
            self.header_len += 1;
            if self.header_len == 20 {
                if &self.header[..4] != b"glTF" || &self.header[16..20] != b"JSON" {
                    return Err("GLB semantic cursor did not find the JSON chunk");
                }
                self.json_remaining = u32::from_le_bytes(self.header[12..16].try_into().expect("fixed GLB JSON length"));
                self.output.json_bytes = self.json_remaining;
                if self.json_remaining == 0 || self.json_remaining as usize > owner.received_bytes().saturating_sub(20) {
                    return Err("GLB JSON chunk exceeded sealed input credits");
                }
            }
            return Ok(false);
        }
        if let Some(token) = self.lexer.replay()? {
            self.apply_token(token)?;
            return Ok(false);
        }
        if self.json_remaining == 0 {
            if let Some(token) = self.lexer.finish()? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.depth != 0
                || self.current_accessor.is_some()
                || self.current_view.is_some()
                || self.current_primitive.is_some()
                || self.current_mesh.is_some()
                || self.current_node.is_some()
                || self.current_scene.is_some()
                || self.current_material.is_some()
                || self.current_texture.is_some()
                || self.current_image.is_some()
                || self.current_sampler.is_some()
                || self.numeric_array.is_some()
            {
                return Err("GLB JSON schema ended with live nested ownership");
            }
            self.output.validate()?;
            self.terminal = true;
            return Ok(true);
        }
        for _ in 0..RENDERER_ASSET_PARSE_BLOCK_BYTES {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB JSON chunk was truncated") };
            self.json_remaining -= 1;
            if let Some(token) = self.lexer.feed(byte)? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.json_remaining == 0 {
                return Ok(false);
            }
        }
        Ok(false)
    }

    fn apply_token(&mut self, token: GlbJsonToken) -> Result<(), &'static str> {
        match token {
            GlbJsonToken::ObjectStart => {
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.section == GlbSchemaSection::Accessors && self.depth == 2 {
                    self.current_accessor = Some(GlbAccessorSchema::default());
                } else if self.section == GlbSchemaSection::Views && self.depth == 2 {
                    self.current_view = Some(GlbViewSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 2 {
                    self.current_mesh = Some(GlbMeshSchema { primitive_start: self.output.primitive_len, primitive_len: 0 });
                } else if self.section == GlbSchemaSection::Meshes && self.primitives_depth == Some(self.depth) {
                    self.current_primitive = Some(GlbPrimitiveSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 5 && self.pending_key.as_ref().is_some_and(|key| key.equals("attributes")) {
                    self.attributes_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 2 {
                    self.current_node = Some(GlbNodeSchema::default());
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 2 {
                    self.current_scene = Some(GlbSceneSchema::default());
                } else if self.section == GlbSchemaSection::Materials && self.depth == 2 {
                    self.current_material = Some(GlbMaterialSchema::default());
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && self.pending_key.as_ref().is_some_and(|key| key.equals("pbrMetallicRoughness")) {
                    self.pbr_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Materials && self.pending_key.as_ref().is_some_and(|key| key.equals("baseColorTexture") || key.equals("metallicRoughnessTexture") || key.equals("normalTexture") || key.equals("occlusionTexture") || key.equals("emissiveTexture")) {
                    let key = self.pending_key.as_ref().unwrap();
                    self.material_texture_role = if key.equals("baseColorTexture") {0} else if key.equals("metallicRoughnessTexture") {1} else if key.equals("normalTexture") {2} else if key.equals("occlusionTexture") {3} else {4};
                    self.base_color_texture_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Textures && self.depth == 2 {
                    self.current_texture = Some(GlbTextureSchema::default());
                } else if self.section == GlbSchemaSection::Images && self.depth == 2 {
                    self.current_image = Some(GlbImageSchema::default());
                } else if self.section == GlbSchemaSection::Samplers && self.depth == 2 {
                    self.current_sampler = Some(GlbSamplerSchema::default());
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ObjectEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON object depth underflowed");
                }
                if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
                    self.finish_accessor()?;
                } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
                    self.finish_view()?;
                } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && self.primitives_depth == Some(self.depth - 1) {
                    self.finish_primitive()?;
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 {
                    self.finish_mesh()?;
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.finish_node()?;
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 {
                    self.finish_scene()?;
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 {
                    self.finish_material()?;
                } else if self.section == GlbSchemaSection::Textures && self.depth == 3 {
                    self.finish_texture()?;
                } else if self.section == GlbSchemaSection::Images && self.depth == 3 {
                    self.finish_image()?;
                } else if self.section == GlbSchemaSection::Samplers && self.depth == 3 {
                    self.finish_sampler()?;
                }
                if self.attributes_depth == Some(self.depth) {
                    self.attributes_depth = None;
                }
                if self.base_color_texture_depth == Some(self.depth) {
                    self.base_color_texture_depth = None;
                }
                if self.pbr_depth == Some(self.depth) {
                    self.pbr_depth = None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::ArrayStart => {
                let key = self.pending_key;
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.depth == 1 {
                    self.section = if key.as_ref().is_some_and(|key| key.equals("accessors")) {
                        GlbSchemaSection::Accessors
                    } else if key.as_ref().is_some_and(|key| key.equals("bufferViews")) {
                        GlbSchemaSection::Views
                    } else if key.as_ref().is_some_and(|key| key.equals("meshes")) {
                        GlbSchemaSection::Meshes
                    } else if key.as_ref().is_some_and(|key| key.equals("nodes")) {
                        GlbSchemaSection::Nodes
                    } else if key.as_ref().is_some_and(|key| key.equals("scenes")) {
                        GlbSchemaSection::Scenes
                    } else if key.as_ref().is_some_and(|key| key.equals("materials")) {
                        GlbSchemaSection::Materials
                    } else if key.as_ref().is_some_and(|key| key.equals("textures")) {
                        GlbSchemaSection::Textures
                    } else if key.as_ref().is_some_and(|key| key.equals("images")) {
                        GlbSchemaSection::Images
                    } else if key.as_ref().is_some_and(|key| key.equals("samplers")) {
                        GlbSchemaSection::Samplers
                    } else {
                        GlbSchemaSection::None
                    };
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("primitives")) {
                    self.primitives_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.numeric_array = key.as_ref().and_then(|key| {
                        if key.equals("children") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeChildren, next_depth, self.output.node_edge_len))
                        } else if key.equals("translation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeTranslation, next_depth, 0))
                        } else if key.equals("rotation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeRotation, next_depth, 0))
                        } else if key.equals("scale") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeScale, next_depth, 0))
                        } else if key.equals("matrix") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeMatrix, next_depth, 0))
                        } else {
                            None
                        }
                    });
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("nodes")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::SceneNodes, next_depth, self.output.scene_root_len));
                } else if self.section == GlbSchemaSection::Materials && self.pbr_depth == Some(self.depth) && key.as_ref().is_some_and(|key| key.equals("baseColorFactor")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::MaterialBaseColor, next_depth, 0));
                } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("emissiveFactor")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::MaterialEmissive, next_depth, 0));
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ArrayEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON array depth underflowed");
                }
                if self.primitives_depth == Some(self.depth) {
                    self.primitives_depth = None;
                }
                if self.numeric_array.as_ref().is_some_and(|array| array.depth == self.depth) {
                    self.finish_numeric_array()?;
                }
                if self.depth == 2 {
                    self.section = GlbSchemaSection::None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::Colon => {
                self.pending_key = self.last_string.take();
                self.expecting_value = true;
            }
            GlbJsonToken::Comma => {
                self.last_string = None;
            }
            GlbJsonToken::String(atom) => {
                if self.expecting_value {
                    self.apply_string(atom)?;
                    self.consume_value();
                } else {
                    self.last_string = Some(atom);
                }
            }
            GlbJsonToken::Number(atom) => {
                if self.numeric_array.is_some() {
                    self.push_numeric_array(atom.float()?)?;
                } else if self.expecting_value {
                    self.apply_unsigned(atom)?;
                }
                self.consume_value();
            }
            GlbJsonToken::Literal(value) => {
                if self.expecting_value {
                    self.apply_literal(value)?;
                }
                self.consume_value();
            }
        }
        Ok(())
    }

    fn apply_unsigned(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
            let value = value.unsigned()?;
            let accessor = self.current_accessor.as_mut().ok_or("GLB accessor scalar arrived without an owner")?;
            if key.equals("bufferView") {
                accessor.view = u16::try_from(value).map_err(|_| "GLB accessor bufferView exceeded fixed index credits")?;
                accessor.view_set = true;
            } else if key.equals("byteOffset") {
                accessor.byte_offset = u32::try_from(value).map_err(|_| "GLB accessor byteOffset overflowed")?;
            } else if key.equals("componentType") {
                accessor.component = u16::try_from(value).map_err(|_| "GLB accessor componentType overflowed")?;
            } else if key.equals("count") {
                accessor.count = u32::try_from(value).map_err(|_| "GLB accessor count overflowed")?;
            }
        } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
            let value = value.unsigned()?;
            let view = self.current_view.as_mut().ok_or("GLB bufferView scalar arrived without an owner")?;
            if key.equals("buffer") && value != 0 {
                return Err("GLB external buffer index was unsupported");
            } else if key.equals("byteOffset") {
                view.byte_offset = u32::try_from(value).map_err(|_| "GLB bufferView byteOffset overflowed")?;
            } else if key.equals("byteLength") {
                view.byte_length = u32::try_from(value).map_err(|_| "GLB bufferView byteLength overflowed")?;
            } else if key.equals("byteStride") {
                view.byte_stride = u16::try_from(value).map_err(|_| "GLB bufferView byteStride exceeded fixed credits")?;
            }
        } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && (self.attributes_depth == Some(self.depth) || self.depth == 5) {
            let value = value.unsigned()?;
            let primitive = self.current_primitive.as_mut().expect("checked GLB primitive owner");
            if self.attributes_depth == Some(self.depth) {
                let index = u16::try_from(value).map_err(|_| "GLB primitive attribute index exceeded fixed credits")?;
                if key.equals("POSITION") {
                    primitive.position = index;
                    primitive.position_set = true;
                } else if key.equals("NORMAL") {
                    primitive.normal = Some(index);
                } else if let Some(set)=std::str::from_utf8(&key.bytes[..usize::from(key.len)]).ok().and_then(|key|key.strip_prefix("TEXCOORD_")).and_then(|set|set.parse::<usize>().ok()) {
                    if set>=64{return Err("GLB UV set exceeded declared surface credits");}primitive.uv_sets[set] = Some(index);
                } else if key.equals("TANGENT") { primitive.tangent=Some(index);
                } else if key.equals("COLOR_0") {
                    primitive.color = Some(index);
                } else if key.equals(GLB_FACE_ID_ATTRIBUTE) {
                    primitive.face_id = Some(index);
                } else if key.equals(GLB_VERTEX_ID_ATTRIBUTE) {
                    primitive.vertex_id = Some(index);
                }
            } else if self.depth == 5 {
                if key.equals("indices") {
                    primitive.indices = Some(u16::try_from(value).map_err(|_| "GLB primitive index accessor exceeded fixed credits")?);
                } else if key.equals("mode") {
                    primitive.mode = u8::try_from(value).map_err(|_| "GLB primitive mode overflowed")?;
                } else if key.equals("material") {
                    primitive.material = Some(u16::try_from(value).map_err(|_| "GLB primitive material index exceeded fixed credits")?);
                }
            }
        } else if self.section == GlbSchemaSection::Materials {
            let material = self.current_material.as_mut().ok_or("GLB material scalar arrived without an owner")?;
            if self.pbr_depth == Some(self.depth) && key.equals("metallicFactor") {
                material.metalness = value.float()?;
            } else if self.pbr_depth == Some(self.depth) && key.equals("roughnessFactor") {
                material.roughness = value.float()?;
            } else if self.base_color_texture_depth == Some(self.depth) && key.equals("index") {
                let index = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB material texture index exceeded fixed credits")?);
                if self.material_texture_role == 0 {material.base_color_texture=index;} else {material.additional_textures[self.material_texture_role-1]=index;}
            } else if self.base_color_texture_depth == Some(self.depth) && key.equals("texCoord") {
                let set=value.unsigned()?;if set>=64{return Err("GLB UV selector exceeded declared surface credits");}material.texture_coordinates[self.material_texture_role]=set as u8;
            } else if self.base_color_texture_depth == Some(self.depth) && self.material_texture_role==2 && key.equals("scale") {material.normal_scale=value.float()?;
            } else if self.base_color_texture_depth == Some(self.depth) && self.material_texture_role==3 && key.equals("strength") {material.occlusion_strength=value.float()?;
            } else if self.depth == 3 && key.equals("alphaCutoff") {
                material.alpha_cutoff = value.float()?;
            }
        } else if self.section == GlbSchemaSection::Textures && self.depth == 3 {
            let value = u16::try_from(value.unsigned()?).map_err(|_| "GLB texture index exceeded fixed credits")?;
            let texture = self.current_texture.as_mut().ok_or("GLB texture scalar arrived without an owner")?;
            if key.equals("sampler") {
                texture.sampler = Some(value);
            } else if key.equals("source") {
                texture.source = Some(value);
            }
        } else if self.section == GlbSchemaSection::Images && self.depth == 3 && key.equals("bufferView") {
            self.current_image.as_mut().ok_or("GLB image bufferView arrived without an owner")?.view = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB image bufferView exceeded fixed credits")?);
        } else if self.section == GlbSchemaSection::Samplers && self.depth == 3 {
            let value = u16::try_from(value.unsigned()?).map_err(|_| "GLB sampler value overflowed")?;
            let sampler = self.current_sampler.as_mut().ok_or("GLB sampler scalar arrived without an owner")?;
            if key.equals("wrapS") {
                sampler.wrap_s = value;
            } else if key.equals("wrapT") {
                sampler.wrap_t = value;
            } else if key.equals("magFilter") {
                sampler.mag_filter = Some(value);
            } else if key.equals("minFilter") {
                sampler.min_filter = Some(value);
            }
        } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 && key.equals("mesh") {
            self.current_node.as_mut().ok_or("GLB node mesh arrived without an owner")?.mesh = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB node mesh index exceeded fixed credits")?);
        } else if self.section == GlbSchemaSection::None && self.depth == 1 && key.equals("scene") {
            self.output.default_scene = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB default scene index exceeded fixed credits")?);
        }
        Ok(())
    }

    fn apply_literal(&mut self, value: Option<bool>) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("normalized") {
            self.current_accessor.as_mut().ok_or("GLB accessor normalized flag arrived without an owner")?.normalized = value.ok_or("GLB accessor normalized flag was null")?;
        } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.equals("doubleSided") {
            self.current_material.as_mut().ok_or("GLB material side flag arrived without an owner")?.double_sided = value.ok_or("GLB material side flag was null")?;
        }
        Ok(())
    }

    fn apply_string(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("type") {
            self.current_accessor.as_mut().ok_or("GLB accessor type arrived without an owner")?.kind = if value.equals("SCALAR") {
                1
            } else if value.equals("VEC2") {
                2
            } else if value.equals("VEC3") {
                3
            } else if value.equals("VEC4") {
                4
            } else {
                return Err("GLB accessor type was unsupported");
            };
        } else if self.section == GlbSchemaSection::Materials && self.depth == 3 && key.equals("alphaMode") {
            self.current_material.as_mut().ok_or("GLB material alpha mode arrived without an owner")?.alpha_mode = if value.equals("OPAQUE") { 0 } else if value.equals("MASK") { 1 } else if value.equals("BLEND") { 2 } else { return Err("GLB material alpha mode was unsupported") };
        } else if self.section == GlbSchemaSection::Images && self.depth == 3 && key.equals("mimeType") {
            self.current_image.as_mut().ok_or("GLB image MIME arrived without an owner")?.mime = if value.equals("image/png") { 1 } else if value.equals("image/jpeg") { 2 } else { return Err("GLB embedded image MIME type was unsupported") };
        }
        Ok(())
    }

    fn consume_value(&mut self) {
        self.pending_key = None;
        self.expecting_value = false;
    }

    fn finish_accessor(&mut self) -> Result<(), &'static str> {
        let accessor = self.current_accessor.take().ok_or("GLB accessor owner was missing")?;
        if accessor.component == 0 || accessor.count == 0 || accessor.kind == 0 {
            return Err("GLB accessor omitted required fixed fields");
        }
        let slot = usize::from(self.output.accessor_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB accessor count exceeded fixed item credits");
        }
        self.output.accessors[slot] = Some(accessor);
        self.output.accessor_len += 1;
        Ok(())
    }

    fn finish_view(&mut self) -> Result<(), &'static str> {
        let view = self.current_view.take().ok_or("GLB bufferView owner was missing")?;
        if view.byte_length == 0 {
            return Err("GLB bufferView omitted its byte length");
        }
        let slot = usize::from(self.output.view_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB bufferView count exceeded fixed item credits");
        }
        self.output.views[slot] = Some(view);
        self.output.view_len += 1;
        Ok(())
    }

    fn finish_primitive(&mut self) -> Result<(), &'static str> {
        let primitive = self.current_primitive.take().ok_or("GLB primitive owner was missing")?;
        if !primitive.position_set {
            return Err("GLB primitive omitted POSITION");
        }
        let slot = usize::from(self.output.primitive_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB primitive count exceeded fixed item credits");
        }
        self.output.primitives[slot] = Some(primitive);
        self.output.primitive_len += 1;
        let mesh = self.current_mesh.as_mut().ok_or("GLB primitive completed without a mesh owner")?;
        mesh.primitive_len = mesh.primitive_len.checked_add(1).ok_or("GLB mesh primitive count overflowed")?;
        Ok(())
    }

    fn finish_mesh(&mut self) -> Result<(), &'static str> {
        let mesh = self.current_mesh.take().ok_or("GLB mesh owner was missing")?;
        if mesh.primitive_len == 0 {
            return Err("GLB mesh contained no primitive");
        }
        let slot = usize::from(self.output.mesh_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB mesh count exceeded fixed item credits");
        }
        self.output.meshes[slot] = Some(mesh);
        self.output.mesh_len += 1;
        Ok(())
    }

    fn finish_node(&mut self) -> Result<(), &'static str> {
        let node = self.current_node.take().ok_or("GLB node owner was missing")?;
        let slot = usize::from(self.output.node_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node count exceeded fixed item credits");
        }
        self.output.nodes[slot] = Some(node);
        self.output.node_len += 1;
        Ok(())
    }

    fn finish_scene(&mut self) -> Result<(), &'static str> {
        let scene = self.current_scene.take().ok_or("GLB scene owner was missing")?;
        let slot = usize::from(self.output.scene_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB scene count exceeded fixed item credits");
        }
        self.output.scenes[slot] = Some(scene);
        self.output.scene_len += 1;
        Ok(())
    }

    fn finish_material(&mut self) -> Result<(), &'static str> {
        let material = self.current_material.take().ok_or("GLB material owner was missing")?;
        let slot = usize::from(self.output.material_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB material count exceeded fixed item credits") }
        self.output.materials[slot] = Some(material);
        self.output.material_len += 1;
        Ok(())
    }

    fn finish_texture(&mut self) -> Result<(), &'static str> {
        let texture = self.current_texture.take().ok_or("GLB texture owner was missing")?;
        let slot = usize::from(self.output.texture_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB texture count exceeded fixed item credits") }
        self.output.textures[slot] = Some(texture);
        self.output.texture_len += 1;
        Ok(())
    }

    fn finish_image(&mut self) -> Result<(), &'static str> {
        let image = self.current_image.take().ok_or("GLB image owner was missing")?;
        let slot = usize::from(self.output.image_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB image count exceeded fixed item credits") }
        self.output.images[slot] = Some(image);
        self.output.image_len += 1;
        Ok(())
    }

    fn finish_sampler(&mut self) -> Result<(), &'static str> {
        let sampler = self.current_sampler.take().ok_or("GLB sampler owner was missing")?;
        let slot = usize::from(self.output.sampler_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY { return Err("GLB sampler count exceeded fixed item credits") }
        self.output.samplers[slot] = Some(sampler);
        self.output.sampler_len += 1;
        Ok(())
    }

    fn finish_numeric_array(&mut self) -> Result<(), &'static str> {
        let array = self.numeric_array.take().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let node = self.current_node.as_mut().ok_or("GLB children completed without a node owner")?;
                node.child_start = array.start;
                node.child_len = array.len;
            }
            GlbNumericArrayKind::NodeTranslation => {
                if array.len != 3 {
                    return Err("GLB node translation did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB translation completed without a node owner")?.translation.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeRotation => {
                if array.len != 4 {
                    return Err("GLB node rotation did not contain four values");
                }
                self.current_node.as_mut().ok_or("GLB rotation completed without a node owner")?.rotation.copy_from_slice(&array.values[..4]);
            }
            GlbNumericArrayKind::NodeScale => {
                if array.len != 3 {
                    return Err("GLB node scale did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB scale completed without a node owner")?.scale.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeMatrix => {
                if array.len != 16 {
                    return Err("GLB node matrix did not contain sixteen values");
                }
                self.current_node.as_mut().ok_or("GLB matrix completed without a node owner")?.matrix = Some(array.values);
            }
            GlbNumericArrayKind::SceneNodes => {
                let scene = self.current_scene.as_mut().ok_or("GLB roots completed without a scene owner")?;
                scene.node_start = array.start;
                scene.node_len = array.len;
            }
            GlbNumericArrayKind::MaterialBaseColor => {
                if array.len != 4 { return Err("GLB material base color did not contain four values") }
                self.current_material.as_mut().ok_or("GLB base color completed without a material owner")?.base_color.copy_from_slice(&array.values[..4]);
            }
            GlbNumericArrayKind::MaterialEmissive => {
                if array.len != 3 { return Err("GLB material emissive did not contain three values") }
                self.current_material.as_mut().ok_or("GLB emissive completed without a material owner")?.emissive.copy_from_slice(&array.values[..3]);
            }
        }
        Ok(())
    }

    fn push_numeric_array(&mut self, value: f32) -> Result<(), &'static str> {
        let array = self.numeric_array.as_mut().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let slot = usize::from(self.output.node_edge_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB child edges exceeded fixed item credits");
                }
                self.output.node_edges[slot] = Some(exact_u16(value, "GLB child index was not an admitted integer")?);
                self.output.node_edge_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB child count overflowed")?;
            }
            GlbNumericArrayKind::SceneNodes => {
                let slot = usize::from(self.output.scene_root_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB scene roots exceeded fixed item credits");
                }
                self.output.scene_roots[slot] = Some(exact_u16(value, "GLB scene node index was not an admitted integer")?);
                self.output.scene_root_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB scene root count overflowed")?;
            }
            _ => array.push(value)?,
        }
        Ok(())
    }
}

fn exact_u16(value: f32, detail: &'static str) -> Result<u16, &'static str> {
    if value < 0.0 || value > u16::MAX as f32 || value.fract() != 0.0 {
        return Err(detail);
    }
    Ok(value as u16)
}

pub type GlbMatrix = [[f32; 4]; 4];

#[derive(Clone, Copy)]
struct GlbInstanceOutput {
    primitive: u16,
    matrix: GlbMatrix,
    vertex_base: u32,
    index_base: u32,
    vertex_count: u32,
    index_count: u32,
    explicit_normals: bool,
}

#[derive(Clone, Copy)]
struct GlbNodePlanFrame {
    node: u16,
    parent: GlbMatrix,
    phase: u8,
    item: u16,
}

enum GlbPlanMode {
    Roots { scene: Option<u16>, index: u16 },
    Nodes { scene: Option<u16>, next_root: u16 },
    Fallback { primitive: u16 },
    Terminal,
}

struct GlbInstancePlanCursor {
    instances: Box<[Option<GlbInstanceOutput>; GLB_SCHEMA_ITEM_CAPACITY]>,
    instance_len: u16,
    stack: Box<[Option<GlbNodePlanFrame>; GLB_SCHEMA_ITEM_CAPACITY]>,
    stack_len: u16,
    mode: GlbPlanMode,
    vertex_count: u32,
    index_count: u32,
    has_uvs: bool,
    has_tangents:bool,
    has_colors: bool,
    face_id_instances: u16,
    vertex_id_instances: u16,
    output_bytes: usize,
}

impl GlbInstancePlanCursor {
    fn new(schema: &GlbSchemaOutput) -> Self {
        let scene = schema.default_scene.or((schema.scene_len != 0).then_some(0));
        let mode = if schema.node_len == 0 { GlbPlanMode::Fallback { primitive: 0 } } else { GlbPlanMode::Roots { scene, index: 0 } };
        Self { instances: semio_framework_async::boxed_fixed_slots(|| None), instance_len: 0, stack: semio_framework_async::boxed_fixed_slots(|| None), stack_len: 0, mode, vertex_count: 0, index_count: 0, has_uvs: false,has_tangents:false, has_colors: false, face_id_instances: 0, vertex_id_instances: 0, output_bytes: 0 }
    }

    /// 🎯️ Whether EVERY instantiated primitive carries `_FACE_ID`; a table with gaps would mix producer ids with anonymous ones, so a mixed bank publishes none.
    fn has_face_ids(&self) -> bool {
        self.instance_len != 0 && self.face_id_instances == self.instance_len
    }

    /// 🎯️ Whether EVERY instantiated primitive carries `_VERTEX_ID`.
    fn has_vertex_ids(&self) -> bool {
        self.instance_len != 0 && self.vertex_id_instances == self.instance_len
    }

    fn step(&mut self, schema: &GlbSchemaOutput) -> Result<bool, &'static str> {
        match self.mode {
            GlbPlanMode::Roots { scene, index } => {
                let root = if let Some(scene) = scene {
                    let scene = schema.scenes.get(usize::from(scene)).and_then(Option::as_ref).ok_or("GLB plan lost its scene")?;
                    if index >= scene.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    schema.scene_roots.get(usize::from(scene.node_start) + usize::from(index)).and_then(Option::as_ref).copied().ok_or("GLB plan lost a scene root")?
                } else {
                    if index >= schema.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    index
                };
                self.push_node(root, glb_world_frame())?;
                self.mode = GlbPlanMode::Nodes { scene, next_root: index + 1 };
                Ok(false)
            }
            GlbPlanMode::Nodes { scene, next_root } => {
                if self.stack_len == 0 {
                    self.mode = GlbPlanMode::Roots { scene, index: next_root };
                    return Ok(false);
                }
                let slot = usize::from(self.stack_len - 1);
                let mut frame = self.stack[slot].take().ok_or("GLB node traversal lost its retained frame")?;
                let node = *schema.nodes.get(usize::from(frame.node)).and_then(Option::as_ref).ok_or("GLB node traversal referenced a missing node")?;
                let local = node.matrix.map(glb_matrix_from_array).unwrap_or_else(|| glb_trs(node.translation, node.rotation, node.scale));
                let world = glb_matrix_mul(frame.parent, local);
                if frame.phase == 0 {
                    frame.phase = 1;
                    frame.item = 0;
                }
                if frame.phase == 1 {
                    if let Some(mesh) = node.mesh {
                        let mesh = schema.meshes.get(usize::from(mesh)).and_then(Option::as_ref).ok_or("GLB node traversal lost a mesh")?;
                        if frame.item < mesh.primitive_len {
                            let primitive = mesh.primitive_start.checked_add(frame.item).ok_or("GLB primitive range overflowed")?;
                            frame.item += 1;
                            self.stack[slot] = Some(frame);
                            self.admit_instance(schema, primitive, world)?;
                            return Ok(false);
                        }
                    }
                    frame.phase = 2;
                    frame.item = 0;
                }
                if frame.item < node.child_len {
                    let child = schema.node_edges.get(usize::from(node.child_start) + usize::from(frame.item)).and_then(Option::as_ref).copied().ok_or("GLB node traversal lost a child edge")?;
                    frame.item += 1;
                    self.stack[slot] = Some(frame);
                    self.push_node(child, world)?;
                    return Ok(false);
                }
                self.stack_len -= 1;
                Ok(false)
            }
            GlbPlanMode::Fallback { primitive } => {
                if primitive >= schema.primitive_len {
                    self.mode = GlbPlanMode::Terminal;
                    return Ok(false);
                }
                self.admit_instance(schema, primitive, glb_world_frame())?;
                self.mode = GlbPlanMode::Fallback { primitive: primitive + 1 };
                Ok(false)
            }
            GlbPlanMode::Terminal => Ok(true),
        }
    }

    fn push_node(&mut self, node: u16, parent: GlbMatrix) -> Result<(), &'static str> {
        let slot = usize::from(self.stack_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node traversal exceeded fixed depth credits");
        }
        if self.stack[..slot].iter().flatten().any(|frame| frame.node == node) {
            return Err("GLB node traversal contained a cycle");
        }
        self.stack[slot] = Some(GlbNodePlanFrame { node, parent, phase: 0, item: 0 });
        self.stack_len += 1;
        Ok(())
    }

    fn admit_instance(&mut self, schema: &GlbSchemaOutput, primitive: u16, matrix: GlbMatrix) -> Result<(), &'static str> {
        let primitive_schema = schema.primitives.get(usize::from(primitive)).and_then(Option::as_ref).ok_or("GLB plan referenced a missing primitive")?;
        if !matches!(primitive_schema.mode, 4..=6) {
            return Ok(());
        }
        let position = schema.accessor(primitive_schema.position)?;
        let source_indices = primitive_schema.indices.map(|index| schema.accessor(index).map(|accessor| accessor.count)).transpose()?.unwrap_or(position.count);
        let index_count = match primitive_schema.mode {
            4 => source_indices,
            5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
            _ => 0,
        };
        if index_count == 0 || !index_count.is_multiple_of(3) {
            return Err("GLB instance had no complete triangle output");
        }
        let vertex_base = self.vertex_count;
        let index_base = self.index_count;
        let vertex_count = vertex_base.checked_add(position.count).ok_or("GLB instantiated vertex count overflowed")?;
        let output_index_count = index_base.checked_add(index_count).ok_or("GLB instantiated index count overflowed")?;
        let has_uvs = self.has_uvs || primitive_schema.uv_sets.iter().any(Option::is_some);
        let has_tangents=self.has_tangents || primitive_schema.tangent.is_some();
        let has_colors = self.has_colors || primitive_schema.color.is_some();
        let has_face_ids = self.face_id_instances != 0 || primitive_schema.face_id.is_some();
        let has_vertex_ids = self.vertex_id_instances != 0 || primitive_schema.vertex_id.is_some();
        let vertex_stride = 24usize.checked_add(if has_uvs { 40 } else { 0 }).and_then(|stride|stride.checked_add(if has_tangents{16}else{0})).and_then(|stride| stride.checked_add(if has_colors { 16 } else { 0 })).and_then(|stride| stride.checked_add(if has_vertex_ids { 4 } else { 0 })).ok_or("GLB instantiated vertex stride overflowed")?;
        let vertex_bytes = usize::try_from(vertex_count).ok().and_then(|count| count.checked_mul(vertex_stride)).ok_or("GLB instantiated vertex bytes overflowed")?;
        let index_bytes = usize::try_from(output_index_count).ok().and_then(|count| count.checked_mul(4).and_then(|bytes| bytes.checked_add(if has_face_ids { count / 3 * 4 } else { 0 }))).ok_or("GLB instantiated index bytes overflowed")?;
        let output_bytes = vertex_bytes.checked_add(index_bytes).ok_or("GLB instantiated output bytes overflowed")?;
        if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
            return Err("GLB instantiated output exceeded fixed byte credits");
        }
        let slot = usize::from(self.instance_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB instantiated primitive count exceeded fixed item credits");
        }
        self.instances[slot] = Some(GlbInstanceOutput { primitive, matrix, vertex_base, index_base, vertex_count: position.count, index_count, explicit_normals: primitive_schema.normal.is_some() });
        self.instance_len += 1;
        self.face_id_instances += u16::from(primitive_schema.face_id.is_some());
        self.vertex_id_instances += u16::from(primitive_schema.vertex_id.is_some());
        self.vertex_count = vertex_count;
        self.index_count = output_index_count;
        self.has_uvs = has_uvs;self.has_tangents=has_tangents;
        self.has_colors = has_colors;
        self.output_bytes = output_bytes;
        Ok(())
    }
}

/// 🧊️🌍️ The frame every GLB scene root is mounted under: a +90° rotation about X, which carries
/// glTF's Y-up convention (https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#coordinate-system-and-units)
/// into the world's Z-up one. Column-major, so `(x, y, z)` becomes `(x, -z, y)`.
///
/// This is `GLB_MESH_FRAME_ROTATION_X` (`♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`), which React applies as the
/// `<group rotation={[π/2, 0, 0]}>` wrapping every `GlbInstanceMesh` — and, crucially, also inside
/// `extractGlbCollisionMesh`, so the collision bodies an app registers for these meshes are already
/// in this frame. Baking it into the decoded mesh (rather than into each instance model) keeps the
/// wire's INLINE geometry untouched, exactly the split React makes: only a GLB gets the frame.
fn glb_world_frame() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, -1.0, 0.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn glb_matrix_from_array(values: [f32; 16]) -> GlbMatrix {
    [values[0..4].try_into().expect("fixed GLB matrix column"), values[4..8].try_into().expect("fixed GLB matrix column"), values[8..12].try_into().expect("fixed GLB matrix column"), values[12..16].try_into().expect("fixed GLB matrix column")]
}

fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            for axis in 0..4 {
                result[column][row] += left[axis][row] * right[column][axis];
            }
        }
    }
    result
}

fn glb_trs(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> GlbMatrix {
    let [x, y, z, w] = rotation;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    [
        [(1.0 - (yy + zz)) * scale[0], (xy + wz) * scale[0], (xz - wy) * scale[0], 0.0],
        [(xy - wz) * scale[1], (1.0 - (xx + zz)) * scale[1], (yz + wx) * scale[1], 0.0],
        [(xz + wy) * scale[2], (yz - wx) * scale[2], (1.0 - (xx + yy)) * scale[2], 0.0],
        [translation[0], translation[1], translation[2], 1.0],
    ]
}

fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    glb_normalize(transformed)
}

fn glb_normalize(value: [f32; 3]) -> [f32; 3] {
    let length = value.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        value
    } else {
        value.map(|value| value / length)
    }
}

pub const GLB_OUTLINE_SCALE: f32 = 1.001;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct GlbOutlineEdgeKey {
    instance: u16,
    from: [i64; 3],
    to: [i64; 3],
}

#[derive(Clone, Copy)]
struct GlbOutlineEdgeRecord {
    points: [[f32; 3]; 2],
    matrix: GlbMatrix,
    normal: [f32; 3],
    live: bool,
}

pub struct GlbOutlineAccumulator {
    lookup: HashMap<GlbOutlineEdgeKey, usize>,
    records: Vec<GlbOutlineEdgeRecord>,
    pub segments: Vec<[[f32; 3]; 2]>,
    maximum_edges: usize,
    flush: usize,
}

impl GlbOutlineAccumulator {
    pub fn new(maximum_edges: usize) -> Result<Self, &'static str> {
        let mut lookup = HashMap::new();
        let mut records = Vec::new();
        let mut segments = Vec::new();
        lookup.try_reserve(maximum_edges).map_err(|_| "GLB outline lookup exceeded allocation credits")?;
        records.try_reserve_exact(maximum_edges).map_err(|_| "GLB outline records exceeded allocation credits")?;
        segments.try_reserve_exact(maximum_edges).map_err(|_| "GLB outline segments exceeded allocation credits")?;
        Ok(Self { lookup, records, segments, maximum_edges, flush: 0 })
    }

    fn point_key(point: [f32; 3]) -> Result<[i64; 3], &'static str> {
        if !point.iter().all(|value| value.is_finite()) {
            return Err("GLB outline position was not finite");
        }
        let scaled = point.map(|value| (f64::from(value) * 10_000.0 + 0.5).floor());
        if !scaled.iter().all(|value| *value >= i64::MIN as f64 && *value <= i64::MAX as f64) {
            return Err("GLB outline position exceeded weld-key credits");
        }
        Ok(scaled.map(|value| value as i64))
    }

    fn transformed(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
        glb_transform_point(matrix, point.map(|value| value * GLB_OUTLINE_SCALE))
    }

    fn push_segment(&mut self, matrix: GlbMatrix, points: [[f32; 3]; 2]) -> Result<(), &'static str> {
        if self.segments.len() == self.maximum_edges {
            return Err("GLB outline output exceeded fixed edge credits");
        }
        self.segments.push([Self::transformed(matrix, points[0]), Self::transformed(matrix, points[1])]);
        Ok(())
    }

    pub fn push_triangle(&mut self, instance: u16, matrix: GlbMatrix, points: [[f32; 3]; 3]) -> Result<(), &'static str> {
        let keys = [Self::point_key(points[0])?, Self::point_key(points[1])?, Self::point_key(points[2])?];
        if keys[0] == keys[1] || keys[1] == keys[2] || keys[2] == keys[0] {
            return Ok(());
        }
        let a = [points[1][0] - points[0][0], points[1][1] - points[0][1], points[1][2] - points[0][2]];
        let b = [points[2][0] - points[0][0], points[2][1] - points[0][1], points[2][2] - points[0][2]];
        let normal = glb_normalize([a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]);
        let threshold = 1.0_f32.to_radians().cos();
        for edge in 0..3 {
            let next = (edge + 1) % 3;
            let key = GlbOutlineEdgeKey { instance, from: keys[edge], to: keys[next] };
            let reverse = GlbOutlineEdgeKey { instance, from: keys[next], to: keys[edge] };
            if let Some(index) = self.lookup.get(&reverse).copied().filter(|index| self.records[*index].live) {
                if normal.iter().zip(self.records[index].normal.iter()).map(|(left, right)| *left * *right).sum::<f32>() <= threshold {
                    self.push_segment(matrix, [points[edge], points[next]])?;
                }
                self.records[index].live = false;
            } else if !self.lookup.contains_key(&key) {
                if self.records.len() == self.maximum_edges {
                    return Err("GLB outline topology exceeded fixed edge credits");
                }
                let index = self.records.len();
                self.records.push(GlbOutlineEdgeRecord { points: [points[edge], points[next]], matrix, normal, live: true });
                self.lookup.insert(key, index);
            }
        }
        Ok(())
    }

    pub fn flush_step(&mut self) -> Result<bool, &'static str> {
        let Some(record) = self.records.get(self.flush).copied() else { return Ok(true) };
        self.flush += 1;
        if record.live {
            self.push_segment(record.matrix, record.points)?;
        }
        Ok(self.flush == self.records.len())
    }
}

#[derive(Clone, Copy)]
pub enum GlbMaterializePhase {
    Plan,
    Header,
    TextureBytes,
    TextureDecode,
    Outlines,
    Allocate,
    Positions,
    Normals,
    Tangents,
    Uvs,
    Colors,
    Indices,
    FaceIds,
    VertexIds,
    GenerateNormals,
    NormalizeNormals,
    OutlineEdges,
    Seal,
    Ready,
    Closing,
}

pub struct GlbMaterializeCursor {
    schema: GlbSchemaOutput,
    plan: GlbInstancePlanCursor,
    write: Option<Mesh3dWriteToken>,
    pub lease: Option<Mesh3dLease>,
    pub appearance: Option<infinite_world::world::World3dMeshAppearance>,
    textures: Vec<(String, SceneRasterLease)>,
    texture_bytes: Vec<u8>,
    decoded_texture: Option<DecodedReferenceImage>,
    texture_hash:semio_framework_hash::Sha256,
    texture_digest:[u64;2],
    texture_png:Option<semio_framework_pixels::png_decoding::PngDecodeJob>,
    texture_submission:Option<semio_framework_async::Job>,
    texture_worker:Option<Arc<Mutex<Option<Option<semio_framework_pixels::RasterImage>>>>>,
    texture_cancelled:Arc<AtomicBool>,
    texture: u16,
    uv_role:usize,
    texture_role: u8,
    outline: Option<GlbOutlineAccumulator>,
    pub phase: GlbMaterializePhase,
    bin_start: usize,
    bin_bytes: usize,
    instance: u16,
    item: u32,
    normal_substep: u8,
    normal_indices: [u32; 3],
    normal_positions: [[f32; 3]; 3],
    normal_face: [f32; 3],
}

impl GlbMaterializeCursor {
    pub fn new(schema: GlbSchemaOutput) -> Self {
        let plan = GlbInstancePlanCursor::new(&schema);
        Self { schema, plan, write: None, lease: None, appearance: None, textures: Vec::new(), texture_bytes: Vec::new(), decoded_texture: None,texture_hash:Default::default(),texture_digest:[0;2],texture_png:None,texture_submission:None,texture_worker:None,texture_cancelled:Arc::new(AtomicBool::new(false)), texture: 0, uv_role:0, texture_role: 0, outline: None, phase: GlbMaterializePhase::Plan, bin_start: 0, bin_bytes: 0, instance: 0, item: 0, normal_substep: 0, normal_indices: [0; 3], normal_positions: [[0.0; 3]; 3], normal_face: [0.0; 3] }
    }

    pub fn step(&mut self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex) -> Result<bool, &'static str> {
        match self.phase {
            GlbMaterializePhase::Plan => {
                if self.plan.step(&self.schema)? {
                    if self.plan.output_bytes == 0 {
                        return Err("GLB plan produced no reachable triangle output");
                    }
                    let maximum_edges = usize::try_from(self.plan.index_count).map_err(|_| "GLB outline edge count overflowed")?;
                    let maximum_outline_bytes = maximum_edges.checked_mul(24).ok_or("GLB outline byte count overflowed")?;
                    if self.plan.output_bytes.checked_add(maximum_outline_bytes).is_none_or(|bytes| bytes > GLB_SCHEMA_OUTPUT_BYTES) {
                        return Err("GLB mesh and outline exceeded fixed semantic output credits");
                    }
                    self.outline = Some(GlbOutlineAccumulator::new(maximum_edges)?);
                    self.phase = GlbMaterializePhase::Header;
                }
                Ok(false)
            }
            GlbMaterializePhase::Header => {
                let header_at = 20usize.checked_add(self.schema.json_bytes as usize).ok_or("GLB BIN header offset overflowed")?;
                let header = pages.read::<8>(owner, header_at)?;
                if &header[4..] != b"BIN\0" {
                    return Err("GLB semantic materializer did not find BIN");
                }
                self.bin_bytes = u32::from_le_bytes(header[..4].try_into().expect("fixed GLB BIN length")) as usize;
                self.bin_start = header_at.checked_add(8).ok_or("GLB BIN payload offset overflowed")?;
                if self.bin_bytes == 0 || self.bin_start.checked_add(self.bin_bytes).is_none_or(|end| end > pages.total) {
                    return Err("GLB BIN payload exceeded sealed page credits");
                }
                self.validate_bin_spans()?;
                self.phase = if self.schema.texture_len == 0 { GlbMaterializePhase::Outlines } else { GlbMaterializePhase::TextureBytes };
                Ok(false)
            }
            GlbMaterializePhase::TextureBytes => {
                if self.texture == self.schema.texture_len {
                    self.advance_phase(GlbMaterializePhase::Outlines);
                    return Ok(false);
                }
                let required=self.schema.materials[..usize::from(self.schema.material_len)].iter().flatten().any(|material|if self.texture_role==0 {material.base_color_texture==Some(self.texture)||material.additional_textures[3]==Some(self.texture)}else {material.additional_textures[..3].contains(&Some(self.texture))});
                if !required {self.texture_role+=1;if self.texture_role==2{self.texture_role=0;self.texture+=1;}return Ok(false)}
                let texture = self.schema.textures.get(usize::from(self.texture)).and_then(Option::as_ref).ok_or("GLB materializer lost a texture schema")?;
                let image = self
                    .schema
                    .images
                    .get(usize::from(texture.source.ok_or("GLB texture lost its image source")?))
                    .and_then(Option::as_ref)
                    .ok_or("GLB materializer lost an image schema")?;
                let view = self
                    .schema
                    .views
                    .get(usize::from(image.view.ok_or("GLB image lost its embedded bufferView")?))
                    .and_then(Option::as_ref)
                    .ok_or("GLB materializer lost an image bufferView")?;
                let expected = usize::try_from(view.byte_length).map_err(|_| "GLB image byte count exceeded address credits")?;
                if self.item == 0 {
                    self.texture_bytes.clear();
                    self.texture_bytes.try_reserve_exact(expected).map_err(|_| "GLB image exceeded allocation credits")?;
                }
                let cursor = usize::try_from(self.item).map_err(|_| "GLB image cursor exceeded address credits")?;
                if cursor == expected {
                    let digest=std::mem::take(&mut self.texture_hash).finalize();self.texture_digest=[u64::from_be_bytes(digest[..8].try_into().unwrap()),u64::from_be_bytes(digest[8..16].try_into().unwrap())];
                    let bytes=std::mem::take(&mut self.texture_bytes);
                    if image.mime==1 {self.texture_png=Some(semio_framework_pixels::png_decoding::PngDecodeJob::new(semio_framework_pixels::png_decoding::PngDecodeInput{data:Arc::new(bytes),max_pixels:16_777_216,max_bytes:16_000_000,max_chunks:65_536}).map_err(|_|"GLB owned PNG texture decoder refused its input")?);}
                    else {let worker=Arc::new(Mutex::new(None));let output=worker.clone();let canceled=self.texture_cancelled.clone();self.texture_submission=Some(Box::new(move||{let image=if canceled.load(Ordering::Acquire){None}else{infinite_world::world::decode_mesh_surface_image_bytes(&bytes)};*output.lock().unwrap_or_else(|poisoned|poisoned.into_inner())=Some(image);}));self.texture_worker=Some(worker);}
                    self.phase = GlbMaterializePhase::TextureDecode;
                    self.item = 0;
                    return Ok(false);
                }
                let take = (expected - cursor).min(RENDERER_ASSET_PARSE_BLOCK_BYTES);
                let absolute = self
                    .bin_start
                    .checked_add(usize::try_from(view.byte_offset).map_err(|_| "GLB image byte offset exceeded address credits")?)
                    .and_then(|offset| offset.checked_add(cursor))
                    .ok_or("GLB image address overflowed")?;
                for at in absolute..absolute + take {
                    self.texture_bytes.push(pages.read::<1>(owner, at)?[0]);
                }
                self.texture_hash.update(&self.texture_bytes[cursor..cursor+take]);
                self.item = self.item.checked_add(u32::try_from(take).map_err(|_| "GLB image block exceeded cursor credits")?).ok_or("GLB image cursor overflowed")?;
                Ok(false)
            }
            GlbMaterializePhase::TextureDecode => {
                if self.decoded_texture.is_none(){
                    if let Some(submission)=self.texture_submission.take(){if let Err(refusal)=renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance,submission){self.texture_submission=Some(refusal.into_job());}return Ok(false)}
                    let image=if let Some(job)=self.texture_png.as_mut(){if !job.advance(4096).map_err(|_|"GLB owned PNG decode failed")?.done{return Ok(false)}self.texture_png.take().unwrap().into_result().map_err(|_|"GLB owned PNG decode failed")?}
                    else {let worker=self.texture_worker.as_ref().ok_or("GLB texture decoder owner was lost")?;let image=worker.lock().map_err(|_|"GLB texture decoder lock failed")?.take();let Some(image)=image else{return Ok(false)};self.texture_worker=None;image.ok_or("GLB owned JPEG decode failed")?};
                    self.decoded_texture=Some(DecodedReferenceImage{width:image.width,height:image.height,source_digest:self.texture_digest,pixels:image.pixels});return Ok(false)
                }
                let decoded = self.decoded_texture.take().ok_or("GLB materializer lost its decoded texture")?;
                let descriptor = SceneRasterDescriptor {
                    width: decoded.width,
                    height: decoded.height,
                    source_digest: decoded.source_digest,
                    source_revision: 1,
                    profile: if self.texture_role == 0 {SceneRasterProfile::MeshBaseColorSrgb} else {SceneRasterProfile::MeshNumericLinear},
                    mesh: None,
                };
                let pool = infinite_world::world::world_scene_raster_pool();
                let raster_owner = decoded.source_digest[0].wrapping_add(decoded.source_digest[1]).max(1);
                let lease = match pool.begin(descriptor, raster_owner, SceneRasterWriteMode::Moved { capacity_bytes: decoded.pixels.capacity() }) {
                    SceneRasterBegin::Reused(lease) => lease,
                    SceneRasterBegin::Writer(writer) => {
                        let DecodedReferenceImage { width, height, source_digest, pixels } = decoded;
                        match pool.prepare_moved(writer, pixels).and_then(|prepared| pool.seal_prepared_moved(prepared)) {
                            Ok(lease) => lease,
                            Err((pixels, detail)) => {
                                self.decoded_texture = Some(DecodedReferenceImage { width, height, source_digest, pixels });
                                return Err(detail);
                            }
                        }
                    }
                    SceneRasterBegin::Backpressure(_) => {
                        self.decoded_texture = Some(decoded);
                        return Err("GLB texture residency admission is full");
                    }
                    SceneRasterBegin::Refused(detail) => {
                        self.decoded_texture = Some(decoded);
                        return Err(detail);
                    }
                };
                self.textures.push((self.texture_key(owner), lease));
                self.texture_role += 1;
                if self.texture_role == 2 {self.texture_role = 0;self.texture += 1;}
                self.texture_bytes.clear();
                self.phase = GlbMaterializePhase::TextureBytes;
                Ok(false)
            }
            GlbMaterializePhase::Outlines => {
                if self.instance == self.plan.instance_len {
                    let outline = self.outline.as_mut().ok_or("GLB materializer lost its outline owner")?;
                    if outline.flush_step()? {
                        let edges = u32::try_from(outline.segments.len()).map_err(|_| "GLB outline output count overflowed")?;
                        let schema = Mesh3dSchema { vertices: self.plan.vertex_count, indices: self.plan.index_count, face_ids: if self.plan.has_face_ids() { self.plan.index_count / 3 } else { 0 }, vertex_ids: if self.plan.has_vertex_ids() { self.plan.vertex_count } else { 0 }, edges, edge_ids: 0, uvs: if self.plan.has_uvs { self.plan.vertex_count } else { 0 }, colors: if self.plan.has_colors { self.plan.vertex_count } else { 0 } ,surface_uvs:if self.plan.has_uvs {[self.plan.vertex_count;4]}else{[0;4]},tangents:if self.plan.has_tangents {self.plan.vertex_count}else{0},};
                        self.write = Some(mesh3d_begin(owner.generation(), owner.revision(), schema).map_err(glb_mesh_fault)?);
                        self.phase = GlbMaterializePhase::Allocate;
                    }
                    return Ok(false);
                }
                let instance = self.instance()?;
                let triangle_count = instance.index_count / 3;
                if self.item == triangle_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let mut points = [[0.0; 3]; 3];
                for component in 0..3u32 {
                    let index = self.read_output_index(owner, pages, primitive, self.item * 3 + component, instance.vertex_count)?;
                    let value = self.read_vec(owner, pages, primitive.position, index, 3)?;
                    points[component as usize] = [value[0], value[1], value[2]];
                }
                self.outline.as_mut().ok_or("GLB materializer lost its outline owner")?.push_triangle(self.instance, instance.matrix, points)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Allocate => {
                if mesh3d_allocate_step(self.write_token()?).map_err(glb_mesh_fault)? {
                    self.advance_phase(GlbMaterializePhase::Positions);
                }
                Ok(false)
            }
            GlbMaterializePhase::Positions => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Normals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = self.read_vec(owner, pages, primitive.position, self.item, 3)?;
                let value = glb_transform_point(instance.matrix, [value[0], value[1], value[2]]);
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Positions, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Normals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Tangents);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let value = if instance.explicit_normals {
                    let primitive = self.primitive(instance)?;
                    let accessor = primitive.normal.ok_or("GLB explicit normal stage lost its accessor")?;
                    let value = self.read_vec(owner, pages, accessor, self.item, 3)?;
                    glb_transform_normal(instance.matrix, [value[0], value[1], value[2]])
                } else {
                    [0.0; 3]
                };
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Normals, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Tangents=>{
                if !self.plan.has_tangents || self.instance==self.plan.instance_len {self.advance_phase(GlbMaterializePhase::Uvs);return Ok(false);}
                let instance=self.instance()?;if self.item==instance.vertex_count {self.next_instance();return Ok(false);}
                let value=if let Some(accessor)=self.primitive(instance)?.tangent {let value=self.read_vec(owner,pages,accessor,self.item,4)?;if value[..3].iter().all(|value|*value==0.0) || value[3].abs()!=1.0 {return Err("GLB TANGENT requires a nonzero direction and signed handedness");}let matrix=instance.matrix;let direction=glb_normalize(std::array::from_fn(|axis|matrix[0][axis]*value[0]+matrix[1][axis]*value[1]+matrix[2][axis]*value[2]));let determinant=matrix[0][0]*(matrix[1][1]*matrix[2][2]-matrix[2][1]*matrix[1][2])-matrix[1][0]*(matrix[0][1]*matrix[2][2]-matrix[2][1]*matrix[0][2])+matrix[2][0]*(matrix[0][1]*matrix[1][2]-matrix[1][1]*matrix[0][2]);[direction[0],direction[1],direction[2],value[3]*if determinant<0.0{-1.0}else{1.0}]}else{[0.0;4]};
                mesh3d_write_vec4(self.write_token()?,Mesh3dField::Tangents,value).map_err(glb_mesh_fault)?;self.item+=1;Ok(false)
            }
            GlbMaterializePhase::Uvs => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Colors);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if !self.plan.has_uvs {
                    self.advance_phase(GlbMaterializePhase::Colors);
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let material=primitive.material.and_then(|index|self.schema.materials[usize::from(index)].as_ref());
                let role=self.uv_role;let set=material.map_or(0,|material|usize::from(material.texture_coordinates[role]));
                let value = if let Some(accessor) = primitive.uv_sets[set] {
                    let value = self.read_vec(owner, pages, accessor, self.item, 2)?;
                    [value[0], value[1]]
                } else {
                    [0.0; 2]
                };
                mesh3d_write_vec2(self.write_token()?, [Mesh3dField::Uvs,Mesh3dField::UvsMetallicRoughness,Mesh3dField::UvsNormal,Mesh3dField::UvsOcclusion,Mesh3dField::UvsEmissive][role], value).map_err(glb_mesh_fault)?;
                self.uv_role+=1;if self.uv_role==5{self.uv_role=0;self.item += 1;}
                Ok(false)
            }
            GlbMaterializePhase::Colors => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if !self.plan.has_colors {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = if let Some(accessor) = primitive.color {
                    let accessor_schema = self.schema.accessor(accessor)?;
                    let value = self.read_vec(owner, pages, accessor, self.item, usize::from(accessor_schema.kind))?;
                    [value[0], value[1], value[2], if accessor_schema.kind == 4 { value[3] } else { 1.0 }]
                } else {
                    [1.0; 4]
                };
                mesh3d_write_vec4(self.write_token()?, Mesh3dField::Colors, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Indices => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::FaceIds);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.index_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let index = self.read_output_index(owner, pages, primitive, self.item, instance.vertex_count)?;
                let index = instance.vertex_base.checked_add(index).ok_or("GLB instantiated index overflowed")?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::FaceIds => {
                if self.instance == self.plan.instance_len || !self.plan.has_face_ids() {
                    self.advance_phase(GlbMaterializePhase::VertexIds);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.index_count / 3 {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let accessor = primitive.face_id.ok_or("GLB face id plan lost its accessor")?;
                let corner = self.read_output_index(owner, pages, primitive, self.item * 3, instance.vertex_count)?;
                let id = self.read_unsigned_id(owner, pages, accessor, corner)?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::FaceIds, id).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::VertexIds => {
                if self.instance == self.plan.instance_len || !self.plan.has_vertex_ids() {
                    self.advance_phase(GlbMaterializePhase::GenerateNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let accessor = primitive.vertex_id.ok_or("GLB vertex id plan lost its accessor")?;
                let id = self.read_unsigned_id(owner, pages, accessor, self.item)?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::VertexIds, id).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::GenerateNormals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::NormalizeNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                let triangle_count = instance.index_count / 3;
                if self.item == triangle_count {
                    self.next_instance();
                    return Ok(false);
                }
                let token = self.write_token()?;
                match self.normal_substep {
                    0..=2 => {
                        let component = usize::from(self.normal_substep);
                        let index = instance.index_base.checked_add(self.item * 3).and_then(|index| index.checked_add(component as u32)).ok_or("GLB generated-normal index address overflowed")?;
                        self.normal_indices[component] = mesh3d_read_write_u32(token, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    3..=5 => {
                        let component = usize::from(self.normal_substep - 3);
                        self.normal_positions[component] = mesh3d_read_write_vec3(token, Mesh3dField::Positions, self.normal_indices[component]).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    6 => {
                        let a = [self.normal_positions[1][0] - self.normal_positions[0][0], self.normal_positions[1][1] - self.normal_positions[0][1], self.normal_positions[1][2] - self.normal_positions[0][2]];
                        let b = [self.normal_positions[2][0] - self.normal_positions[0][0], self.normal_positions[2][1] - self.normal_positions[0][1], self.normal_positions[2][2] - self.normal_positions[0][2]];
                        self.normal_face = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
                        self.normal_substep += 1;
                    }
                    7..=9 => {
                        let component = usize::from(self.normal_substep - 7);
                        let index = self.normal_indices[component];
                        let prior = mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?;
                        mesh3d_update_vec3(token, Mesh3dField::Normals, index, [prior[0] + self.normal_face[0], prior[1] + self.normal_face[1], prior[2] + self.normal_face[2]]).map_err(glb_mesh_fault)?;
                        if self.normal_substep == 9 {
                            self.normal_substep = 0;
                            self.item += 1;
                        } else {
                            self.normal_substep += 1;
                        }
                    }
                    _ => return Err("GLB generated-normal cursor exceeded its fixed phase"),
                }
                Ok(false)
            }
            GlbMaterializePhase::NormalizeNormals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::OutlineEdges);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let index = instance.vertex_base.checked_add(self.item).ok_or("GLB normal output index overflowed")?;
                let token = self.write_token()?;
                let normal = glb_normalize(mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?);
                mesh3d_update_vec3(token, Mesh3dField::Normals, index, normal).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::OutlineEdges => {
                let outline = self.outline.as_ref().ok_or("GLB materializer lost its outline owner")?;
                let Some(edge) = outline.segments.get(self.item as usize).copied() else {
                    self.phase = GlbMaterializePhase::Seal;
                    return Ok(false);
                };
                mesh3d_write_edge(self.write_token()?, edge).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Seal => {
                let token = self.write.take().ok_or("GLB materializer lost its mesh write claim")?;
                match mesh3d_seal(token) {
                    Ok(lease) => {
                        let appearance = self.build_appearance(owner)?;
                        self.lease = Some(lease);
                        self.appearance = Some(appearance);
                        self.phase = GlbMaterializePhase::Ready;
                        Ok(true)
                    }
                    Err(fault) => {
                        self.write = Some(token);
                        Err(glb_mesh_fault(fault))
                    }
                }
            }
            GlbMaterializePhase::Ready => Ok(true),
            GlbMaterializePhase::Closing => Ok(false),
        }
    }

    fn validate_bin_spans(&self) -> Result<(), &'static str> {
        for accessor in self.schema.accessors[..usize::from(self.schema.accessor_len)].iter().flatten() {
            let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB materializer lost an accessor view")?;
            let end = (view.byte_offset as usize).checked_add(view.byte_length as usize).ok_or("GLB bufferView BIN span overflowed")?;
            if end > self.bin_bytes {
                return Err("GLB bufferView exceeded the BIN payload");
            }
        }
        for image in self.schema.images[..usize::from(self.schema.image_len)].iter().flatten() {
            let view = self.schema.views.get(usize::from(image.view.ok_or("GLB image lost its embedded bufferView")?)).and_then(Option::as_ref).ok_or("GLB image referenced a missing bufferView")?;
            let end = (view.byte_offset as usize).checked_add(view.byte_length as usize).ok_or("GLB image bufferView BIN span overflowed")?;
            if end > self.bin_bytes {
                return Err("GLB image bufferView exceeded the BIN payload");
            }
        }
        Ok(())
    }

    fn texture_key(&self, owner: &RendererAssetFetchOwner) -> String {
        format!("glb:{}:texture:{}:{}", owner.generation(), self.texture, if self.texture_role == 0 {"srgb"} else {"linear"})
    }

    fn build_appearance(&mut self, owner: &RendererAssetFetchOwner) -> Result<World3dMeshAppearance, &'static str> {
        let mut primitives = Vec::new();
        primitives.try_reserve_exact(usize::from(self.plan.instance_len)).map_err(|_| "GLB material appearance exceeded allocation credits")?;
        for instance in self.plan.instances[..usize::from(self.plan.instance_len)].iter().flatten() {
            let primitive = self.schema.primitives.get(usize::from(instance.primitive)).and_then(Option::as_ref).ok_or("GLB materializer lost a primitive material")?;
            let material = primitive
                .material
                .and_then(|index| self.schema.materials.get(usize::from(index)).and_then(Option::as_ref).copied())
                .unwrap_or_default();
            let base_color_texture = material.base_color_texture.map(|texture| format!("glb:{}:texture:{texture}:srgb", owner.generation()));
            let sampler_for = |reference:Option<u16>| -> Result<ui_wgpu::wgpu::SceneTextureSampler3d, &'static str> {Ok(if let Some(texture) = reference {
                let texture = self.schema.textures.get(usize::from(texture)).and_then(Option::as_ref).ok_or("GLB materializer lost a texture sampler owner")?;
                let sampler = texture.sampler.and_then(|index| self.schema.samplers.get(usize::from(index)).and_then(Option::as_ref).copied()).unwrap_or_default();
                ui_wgpu::wgpu::SceneTextureSampler3d {
                    wrap_u: glb_texture_wrap(sampler.wrap_s)?,
                    wrap_v: glb_texture_wrap(sampler.wrap_t)?,
                    mag_filter: glb_mag_filter(sampler.mag_filter)?,
                    min_filter: glb_min_filter(sampler.min_filter)?,
                }
            } else {
                Default::default()
            })};
            let texture_sampler = sampler_for(material.base_color_texture)?;
            let additional_texture_samplers = [sampler_for(material.additional_textures[0])?,sampler_for(material.additional_textures[1])?,sampler_for(material.additional_textures[2])?,sampler_for(material.additional_textures[3])?];
            let reference = |role:usize| material.additional_textures[role].map(|texture|format!("glb:{}:texture:{texture}:{}",owner.generation(),if role==3 {"srgb"} else {"linear"}));
            primitives.push(World3dPrimitiveMaterial {
                first_index: instance.index_base,
                index_count: instance.index_count,
                material: ui_wgpu::wgpu::SceneAuthoredMaterial3d {
                    base_color: material.base_color,
                    emissive: material.emissive,
                    metalness: material.metalness,
                    roughness: material.roughness,
                    alpha: match material.alpha_mode {
                        0 => ui_wgpu::wgpu::SceneMaterialAlpha3d::Opaque,
                        1 => ui_wgpu::wgpu::SceneMaterialAlpha3d::Mask,
                        2 => ui_wgpu::wgpu::SceneMaterialAlpha3d::Blend,
                        _ => return Err("GLB materializer reached an unsupported alpha mode"),
                    },
                    alpha_cutoff: material.alpha_cutoff,
                    double_sided: material.double_sided,
                    preserve_vertex_color: primitive.color.is_some(),
                    base_color_texture,
                    texture_sampler,
                    metallic_roughness_texture:reference(0),normal_texture:reference(1),occlusion_texture:reference(2),emissive_texture:reference(3),additional_texture_samplers,
                normal_scale:[material.normal_scale,if primitive.tangent.is_some(){material.normal_scale}else{-material.normal_scale}],occlusion_strength:material.occlusion_strength,},
            });
        }
        let textures = std::mem::take(&mut self.textures);
        World3dMeshAppearance::new(primitives, textures).map_err(|(_primitives, textures)| {
            self.textures = textures;
            "GLB material appearance exceeded fixed owner credits"
        })
    }

    fn read_vec(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: u16, item: u32, components: usize) -> Result<[f32; 4], &'static str> {
        let schema = self.schema.accessor(accessor)?;
        if usize::from(schema.kind) != components || item >= schema.count {
            return Err("GLB accessor semantic read exceeded its shape");
        }
        let mut result = [0.0; 4];
        for (component, output) in result[..components].iter_mut().enumerate() {
            *output = self.read_component(owner, pages, schema, item, component)?;
        }
        Ok(result)
    }

    fn read_component(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: &GlbAccessorSchema, item: u32, component: usize) -> Result<f32, &'static str> {
        let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor semantic read lost its bufferView")?;
        let component_bytes = glb_component_bytes(accessor.component)?;
        let item_bytes = usize::from(accessor.kind).checked_mul(component_bytes).ok_or("GLB accessor semantic item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        let relative = (view.byte_offset as usize)
            .checked_add(accessor.byte_offset as usize)
            .and_then(|offset| usize::try_from(item).ok().and_then(|item| item.checked_mul(stride)).and_then(|item| offset.checked_add(item)))
            .and_then(|offset| offset.checked_add(component * component_bytes))
            .ok_or("GLB accessor semantic address overflowed")?;
        let absolute = self.bin_start.checked_add(relative).ok_or("GLB accessor absolute address overflowed")?;
        glb_read_component(owner, pages, absolute, accessor.component, accessor.normalized)
    }

    /// 🎯️ One exact `u32` sub-element id; `read_component` widens through `f32` and would corrupt ids past 2^24.
    fn read_unsigned_id(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: u16, item: u32) -> Result<u32, &'static str> {
        let schema = self.schema.accessor(accessor)?;
        if schema.component != 5125 || schema.kind != 1 || item >= schema.count {
            return Err("GLB sub-element id read exceeded its shape");
        }
        let view = self.schema.views.get(usize::from(schema.view)).and_then(Option::as_ref).ok_or("GLB sub-element id read lost its bufferView")?;
        let stride = if view.byte_stride == 0 { size_of::<u32>() } else { usize::from(view.byte_stride) };
        let relative = (view.byte_offset as usize)
            .checked_add(schema.byte_offset as usize)
            .and_then(|offset| usize::try_from(item).ok().and_then(|item| item.checked_mul(stride)).and_then(|item| offset.checked_add(item)))
            .ok_or("GLB sub-element id address overflowed")?;
        let absolute = self.bin_start.checked_add(relative).ok_or("GLB sub-element id absolute address overflowed")?;
        Ok(u32::from_le_bytes(pages.read::<4>(owner, absolute)?))
    }

    fn read_output_index(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, primitive: &GlbPrimitiveSchema, output: u32, vertex_count: u32) -> Result<u32, &'static str> {
        let triangle = output / 3;
        let component = output % 3;
        let source = match (primitive.mode, component) {
            (4, component) => triangle * 3 + component,
            (5, component) if triangle.is_multiple_of(2) => triangle + component,
            (5, 0) => triangle + 1,
            (5, 1) => triangle,
            (5, 2) => triangle + 2,
            (6, 0) => 0,
            (6, 1) => triangle + 1,
            (6, 2) => triangle + 2,
            _ => return Err("GLB materializer reached an unsupported primitive mode"),
        };
        let result = if let Some(accessor) = primitive.indices {
            let accessor = self.schema.accessor(accessor)?;
            let value = self.read_component(owner, pages, accessor, source, 0)?;
            if value < 0.0 || value > u32::MAX as f32 || value.fract() != 0.0 {
                return Err("GLB index value was not an admitted integer");
            }
            value as u32
        } else {
            source
        };
        (result < vertex_count).then_some(result).ok_or("GLB index exceeded POSITION count")
    }

    fn instance(&self) -> Result<GlbInstanceOutput, &'static str> {
        self.plan.instances.get(usize::from(self.instance)).and_then(Option::as_ref).copied().ok_or("GLB materializer lost an instance plan")
    }

    fn primitive(&self, instance: GlbInstanceOutput) -> Result<&GlbPrimitiveSchema, &'static str> {
        self.schema.primitives.get(usize::from(instance.primitive)).and_then(Option::as_ref).ok_or("GLB materializer lost a primitive schema")
    }

    fn write_token(&self) -> Result<Mesh3dWriteToken, &'static str> {
        self.write.ok_or("GLB materializer lost its mesh write claim")
    }

    pub fn take_asset(&mut self) -> Option<World3dMeshAsset> {
        Some(World3dMeshAsset { mesh: self.lease.take()?, appearance: self.appearance.take()? })
    }

    pub fn restore_asset(&mut self, asset: World3dMeshAsset) {
        assert!(self.lease.replace(asset.mesh).is_none(), "GLB publication restores exactly one retained mesh lease");
        assert!(self.appearance.replace(asset.appearance).is_none(), "GLB publication restores exactly one retained appearance owner");
    }

    fn next_instance(&mut self) {
        self.instance += 1;
        self.item = 0;
        self.normal_substep = 0;
    }

    fn advance_phase(&mut self, phase: GlbMaterializePhase) {
        self.phase = phase;
        self.instance = 0;
        self.item = 0;
        self.normal_substep = 0;
    }

    pub fn begin_close(&mut self) {
        self.texture_cancelled.store(true,Ordering::Release);
        self.phase = GlbMaterializePhase::Closing;
    }

    pub fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.texture_submission.take().is_some(){self.texture_worker=None;return false}
        if let Some(worker)=self.texture_worker.as_ref(){let Ok(mut output)=worker.try_lock()else{return false};let Some(image)=output.take()else{return false};self.decoded_texture=image.map(|image|DecodedReferenceImage{width:image.width,height:image.height,pixels:image.pixels,source_digest:self.texture_digest});drop(output);self.texture_worker=None;return false}
        if let Some(mut job)=self.texture_png.take(){job.cancel();return false}
        if let Some(image)=self.decoded_texture.as_mut(){if !image.pixels.is_empty(){image.pixels.truncate(image.pixels.len().saturating_sub(4096));return false}self.decoded_texture=None;return false}
        if !self.texture_bytes.is_empty(){self.texture_bytes.truncate(self.texture_bytes.len().saturating_sub(4096));return false}
        if let Some(appearance)=self.appearance.as_mut(){if !appearance.close_step(){return false}self.appearance=None;return false}
        if self.textures.pop().is_some(){return false}
        if let Some(token) = self.write {
            match mesh3d_abort(token) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.write = None;
                    return self.lease.is_none() && self.appearance.is_none() && self.textures.is_empty();
                }
                Err(_) => return false,
            }
            match mesh3d_abort_step(token) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.write = None,
                Ok(false) | Err(_) => return false,
            }
            return self.lease.is_none() && self.appearance.is_none() && self.textures.is_empty();
        }
        if let Some(lease) = self.lease {
            match mesh3d_begin_close(lease) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.lease = None;
                    return true;
                }
                Err(_) => return false,
            }
            match mesh3d_close_step(lease) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.lease = None,
                Ok(false) | Err(_) => return false,
            }
        }
        true
    }
}

fn glb_texture_wrap(value: u16) -> Result<ui_wgpu::wgpu::SceneTextureWrap3d, &'static str> {
    match value {
        33071 => Ok(ui_wgpu::wgpu::SceneTextureWrap3d::ClampToEdge),
        10497 => Ok(ui_wgpu::wgpu::SceneTextureWrap3d::Repeat),
        33648 => Ok(ui_wgpu::wgpu::SceneTextureWrap3d::MirrorRepeat),
        _ => Err("GLB texture sampler used an unsupported wrap mode"),
    }
}

fn glb_mag_filter(value: Option<u16>) -> Result<ui_wgpu::wgpu::SceneTextureFilter3d, &'static str> {
    match value {
        None | Some(9729) => Ok(ui_wgpu::wgpu::SceneTextureFilter3d::Linear),
        Some(9728) => Ok(ui_wgpu::wgpu::SceneTextureFilter3d::Nearest),
        Some(_) => Err("GLB texture sampler used an unsupported magnification filter"),
    }
}

fn glb_min_filter(value: Option<u16>) -> Result<ui_wgpu::wgpu::SceneTextureFilter3d, &'static str> {
    match value {
        None => Ok(ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapLinear),
        Some(9729) => Ok(ui_wgpu::wgpu::SceneTextureFilter3d::Linear),
        Some(9985)=>Ok(ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapNearest),
        Some(9987)=>Ok(ui_wgpu::wgpu::SceneTextureFilter3d::LinearMipmapLinear),
        Some(9984)=>Ok(ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapNearest),
        Some(9986)=>Ok(ui_wgpu::wgpu::SceneTextureFilter3d::NearestMipmapLinear),
        Some(9728) => Ok(ui_wgpu::wgpu::SceneTextureFilter3d::Nearest),
        Some(_) => Err("GLB texture sampler used an unsupported minification filter"),
    }
}

fn glb_mesh_fault(fault: Mesh3dFault) -> &'static str {
    match fault {
        Mesh3dFault::Closing => "GLB paged mesh authority was closing",
        Mesh3dFault::ItemCapacity => "GLB paged mesh authority exhausted fixed owner slots",
        Mesh3dFault::PageCapacity => "GLB paged mesh authority exhausted aggregate page credits",
        Mesh3dFault::ByteCapacity => "GLB paged mesh exceeded fixed byte credits",
        Mesh3dFault::Schema => "GLB paged mesh schema was invalid",
        Mesh3dFault::Stale => "GLB paged mesh generation/revision witness was stale",
        Mesh3dFault::Order => "GLB paged mesh field order was invalid",
        Mesh3dFault::Incomplete => "GLB paged mesh was not terminally complete",
    }
}

fn glb_component_bytes(component: u16) -> Result<usize, &'static str> {
    match component {
        5120 | 5121 => Ok(1),
        5122 | 5123 => Ok(2),
        5125 | 5126 => Ok(4),
        _ => Err("GLB component type was unsupported"),
    }
}

fn glb_read_component(owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, absolute: usize, component: u16, normalized: bool) -> Result<f32, &'static str> {
    let value = match component {
        5120 => {
            let value = i8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 127.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5121 => {
            let value = u8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                f32::from(value) / 255.0
            } else {
                f32::from(value)
            }
        }
        5122 => {
            let value = i16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 32767.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5123 => {
            let value = u16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                f32::from(value) / 65535.0
            } else {
                f32::from(value)
            }
        }
        5125 => {
            let value = u32::from_le_bytes(pages.read::<4>(owner, absolute)?);
            if normalized {
                value as f32 / u32::MAX as f32
            } else {
                value as f32
            }
        }
        5126 if !normalized => f32::from_le_bytes(pages.read::<4>(owner, absolute)?),
        5126 => return Err("GLB FLOAT accessor could not be normalized"),
        _ => return Err("GLB semantic component type was unsupported"),
    };
    value.is_finite().then_some(value).ok_or("GLB semantic component was not finite")
}
//#endregion 📡️RendererAssetAuthority
