//! 🔤️ Json lexer and schema types: the GLB JSON atom, token and lexer cursor and the accessor, view, primitive, material, texture, image, sampler, mesh, node and scene schema records with the numeric array. See [`super`].

use super::*;

//#region 📡️RendererAssetAuthority
const GLB_JSON_ATOM_BYTES: usize = 64;

pub(super) const GLB_SCHEMA_ITEM_CAPACITY: usize = 512;

pub(super) const GLB_SCHEMA_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy)]
pub(super) struct GlbJsonAtom {
    pub(super) bytes: [u8; GLB_JSON_ATOM_BYTES],
    pub(super) len: u8,
    pub(super) total: u32,
    overflow: bool,
}

impl GlbJsonAtom {
    pub(super) fn new() -> Self {
        Self { bytes: [0; GLB_JSON_ATOM_BYTES], len: 0, total: 0, overflow: false }
    }

    pub(super) fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        self.total = self.total.checked_add(1).ok_or("GLB JSON atom length overflowed")?;
        if usize::from(self.len) < self.bytes.len() {
            self.bytes[usize::from(self.len)] = byte;
            self.len += 1;
        } else {
            self.overflow = true;
        }
        Ok(())
    }

    pub(super) fn equals(&self, expected: &str) -> bool {
        !self.overflow && usize::from(self.len) == expected.len() && &self.bytes[..usize::from(self.len)] == expected.as_bytes()
    }

    pub(super) fn unsigned(&self) -> Result<u64, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON integer exceeded fixed atom credits");
        }
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse().ok()).ok_or("GLB JSON integer was invalid")
    }

    pub(super) fn float(&self) -> Result<f32, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON number exceeded fixed atom credits");
        }
        let value = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse::<f32>().ok()).ok_or("GLB JSON number was invalid")?;
        value.is_finite().then_some(value).ok_or("GLB JSON number was not finite")
    }
}

pub(super) enum GlbJsonToken {
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

pub(super) struct GlbJsonTokenCursor {
    pub(super) phase: GlbJsonLexPhase,
    pub(super) replay: Option<u8>,
}

impl GlbJsonTokenCursor {
    pub(super) fn new() -> Self {
        Self { phase: GlbJsonLexPhase::Idle, replay: None }
    }

    pub(super) fn feed(&mut self, byte: u8) -> Result<Option<GlbJsonToken>, &'static str> {
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

    pub(super) fn replay(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        let Some(byte) = self.replay else { return Ok(None) };
        self.feed(byte)
    }

    pub(super) fn finish(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        match std::mem::replace(&mut self.phase, GlbJsonLexPhase::Idle) {
            GlbJsonLexPhase::Idle => self.replay(),
            GlbJsonLexPhase::Number(atom) => Ok(Some(GlbJsonToken::Number(atom))),
            _ => Err("GLB JSON token ended before terminal"),
        }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbAccessorSchema {
    pub(super) view: u16,
    pub(super) byte_offset: u32,
    pub(super) component: u16,
    pub(super) count: u32,
    pub(super) kind: u8,
    pub(super) view_set: bool,
    pub(super) normalized: bool,
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbViewSchema {
    pub(super) byte_offset: u32,
    pub(super) byte_length: u32,
    pub(super) byte_stride: u16,
}

/// 🎯️ The custom glTF attribute that carries one face id per vertex; a triangle's face id is its first corner's. Pinned by
/// `🧫️fixtures/📏️world3d-modelling/🔣️.json` (`glbSubElementIds`).
pub(super) const GLB_FACE_ID_ATTRIBUTE: &str = "_FACE_ID";

/// 🎯️ The custom glTF attribute that carries one vertex id per vertex.
pub(super) const GLB_VERTEX_ID_ATTRIBUTE: &str = "_VERTEX_ID";

#[derive(Clone, Copy)]
pub(super) struct GlbPrimitiveSchema {
    pub(super) position: u16,
    pub(super) normal: Option<u16>,
    pub(super) uv_sets: [Option<u16>;64],
    pub(super) tangent:Option<u16>,
    pub(super) color: Option<u16>,
    pub(super) indices: Option<u16>,
    pub(super) material: Option<u16>,
    pub(super) mode: u8,
    pub(super) position_set: bool,
    pub(super) face_id: Option<u16>,
    pub(super) vertex_id: Option<u16>,
}

#[derive(Clone, Copy)]
pub(super) struct GlbMaterialSchema {
    pub(super) base_color: [f32; 4],
    pub(super) emissive: [f32; 3],
    pub(super) metalness: f32,
    pub(super) roughness: f32,
    pub(super) alpha_mode: u8,
    pub(super) alpha_cutoff: f32,
    pub(super) double_sided: bool,
    pub(super) base_color_texture: Option<u16>,
    pub(super) additional_textures: [Option<u16>;4],
    pub(super) texture_coordinates:[u8;5],
    pub(super) normal_scale:f32,
    pub(super) occlusion_strength:f32,
}

impl Default for GlbMaterialSchema {
    fn default() -> Self {
        Self { base_color: [1.0; 4], emissive: [0.0; 3], metalness: 1.0, roughness: 1.0, alpha_mode: 0, alpha_cutoff: 0.5, double_sided: false, base_color_texture: None, additional_textures: [None;4],texture_coordinates:[0;5],normal_scale:1.0,occlusion_strength:1.0 }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbTextureSchema {
    pub(super) sampler: Option<u16>,
    pub(super) source: Option<u16>,
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbImageSchema {
    pub(super) view: Option<u16>,
    pub(super) mime: u8,
}

#[derive(Clone, Copy)]
pub(super) struct GlbSamplerSchema {
    pub(super) wrap_s: u16,
    pub(super) wrap_t: u16,
    pub(super) mag_filter: Option<u16>,
    pub(super) min_filter: Option<u16>,
}

impl Default for GlbSamplerSchema {
    fn default() -> Self {
        Self { wrap_s: 10497, wrap_t: 10497, mag_filter: None, min_filter: None }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbMeshSchema {
    pub(super) primitive_start: u16,
    pub(super) primitive_len: u16,
}

#[derive(Clone, Copy)]
pub(super) struct GlbNodeSchema {
    pub(super) mesh: Option<u16>,
    pub(super) child_start: u16,
    pub(super) child_len: u16,
    pub(super) translation: [f32; 3],
    pub(super) rotation: [f32; 4],
    pub(super) scale: [f32; 3],
    pub(super) matrix: Option<[f32; 16]>,
}

impl Default for GlbNodeSchema {
    fn default() -> Self {
        Self { mesh: None, child_start: 0, child_len: 0, translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: [1.0; 3], matrix: None }
    }
}

#[derive(Clone, Copy, Default)]
pub(super) struct GlbSceneSchema {
    pub(super) node_start: u16,
    pub(super) node_len: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum GlbNumericArrayKind {
    NodeChildren,
    NodeTranslation,
    NodeRotation,
    NodeScale,
    NodeMatrix,
    SceneNodes,
    MaterialBaseColor,
    MaterialEmissive,
}

pub(super) struct GlbNumericArray {
    pub(super) kind: GlbNumericArrayKind,
    pub(super) depth: u16,
    pub(super) start: u16,
    pub(super) values: [f32; 16],
    pub(super) len: u16,
}

impl GlbNumericArray {
    pub(super) fn new(kind: GlbNumericArrayKind, depth: u16, start: u16) -> Self {
        Self { kind, depth, start, values: [0.0; 16], len: 0 }
    }

    pub(super) fn push(&mut self, value: f32) -> Result<(), &'static str> {
        let slot = usize::from(self.len);
        if slot == self.values.len() {
            return Err("GLB node or scene array exceeded fixed item credits");
        }
        self.values[slot] = value;
        self.len += 1;
        Ok(())
    }
}
//#endregion 📡️RendererAssetAuthority
