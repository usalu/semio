//! 📥️ Borrowed GLB container scanning and caller-funded JSON document intake.

use super::{json, MeshData};
use super::glb_scene::{SceneCursor, SceneStep};
use semio_framework_value::{RetainedAllocationEffect, retained_clone::RetainedCloneProgress};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeContinuation, NativeDecodeControl};

const MAXIMUM_DOCUMENT_ALLOCATED_BYTES: usize = 256 * 1024 * 1024;

#[derive(Default, semio_framework_value_derive::RetireOwned)]
pub(super) struct GlbChunks {
    length: Option<usize>,
    at: usize,
    json: Option<[usize; 2]>,
    bin: Option<[usize; 2]>,
}

impl GlbChunks {
    pub(super) fn step(&mut self, bytes: &[u8]) -> Result<bool, String> {
        let Some(length) = self.length else {
            let header = bytes.get(..12).filter(|header| header.starts_with(b"glTF")).ok_or("glb: bad magic, expected 'glTF'")?;
            let version = u32::from_le_bytes(header[4..8].try_into().unwrap());
            if version != 2 {
                return Err(format!("glb: unsupported version {version}, only 2 is supported"));
            }
            self.length = Some(u32::from_le_bytes(header[8..12].try_into().unwrap()) as usize);
            self.at = 12;
            return Ok(false);
        };
        let header_end = self.at.checked_add(8).ok_or("glb: invalid chunk header")?;
        if self.at >= length || header_end > bytes.len() {
            self.json(bytes)?;
            return Ok(true);
        }
        let header = &bytes[self.at..header_end];
        let size = u32::from_le_bytes(header[..4].try_into().unwrap()) as usize;
        let end = header_end.checked_add(size).filter(|end| *end <= bytes.len()).ok_or("glb: chunk length exceeds buffer")?;
        match &header[4..8] {
            b"JSON" if self.json.is_none() => self.json = Some([header_end, end]),
            b"BIN\0" if self.bin.is_none() => self.bin = Some([header_end, end]),
            _ => {}
        }
        self.at = end;
        Ok(false)
    }

    pub(super) fn json<'a>(&self, bytes: &'a [u8]) -> Result<&'a [u8], String> {
        self.json.and_then(|[start, end]| bytes.get(start..end)).ok_or_else(|| "glb: missing JSON chunk".to_owned())
    }

    pub(super) fn bin<'a>(&self, bytes: &'a [u8]) -> Option<&'a [u8]> {
        self.bin.and_then(|[start, end]| bytes.get(start..end))
    }
}

/// 🧵️ Owns the parsed scene document and partial grammar storage through original retirement.
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
pub struct GlbDocumentReader {
    chunks: GlbChunks,
    scanning: bool,
    parser: json::JsonParseCursor,
    continuation: Option<NativeDecodeContinuation>,
    document: Option<json::Value>,
    scene: Option<SceneCursor>,
    receipt: [usize; 4],
}

impl Default for GlbDocumentReader {
    fn default() -> Self {
        Self {
            chunks: GlbChunks::default(),
            scanning: true,
            parser: json::JsonParseCursor::new(json::JsonMemberPolicy::Reject),
            continuation: None,
            document: None,
            scene: None,
            receipt: [0; 4],
        }
    }
}

impl GlbDocumentReader {
    /// 📍️ Labels the current physical intake frontier.
    pub fn phase(&self) -> &'static str {
        if self.scanning { "mesh-glb-container" }
        else if self.document.is_none() { "mesh-glb-json" }
        else { self.scene.as_ref().map_or("mesh-glb-metadata", SceneCursor::phase) }
    }

    pub fn document_ready(&self) -> bool { self.document.is_some() }

    /// 📜️ Reports what the most recent intake turn copied, owned or released.
    pub fn take_progress(&mut self) -> RetainedCloneProgress {
        let [copied_items, copied_bytes, retained_capacity_bytes, released_bytes] = std::mem::take(&mut self.receipt);
        RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes }
    }

    /// ⏱️ Advances one bounded container or UTF-8/JSON grammar frontier.
    pub fn parse_step(&mut self, bytes: &[u8], effect: &mut RetainedAllocationEffect<'_>) -> Result<bool, String> {
        self.receipt = [0; 4];
        if self.document.is_some() {
            return Ok(true);
        }
        let grant = effect.grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || effect.remaining_work_units().map_err(|error| error.to_string())? == 0 {
            return Ok(false);
        }
        if self.scanning {
            if !effect.performed(1).map_err(|error| error.to_string())? {
                return Ok(false);
            }
            self.receipt[0] = 1;
            self.scanning = !self.chunks.step(bytes)?;
            return Ok(false);
        }
        let input = self.chunks.json(bytes)?;
        let demand = self.parser.normal_step_demands(input).map_err(|error| error.to_string())?;
        let mut accepted = |_| true;
        let mut control = match self.continuation.take() {
            Some(receipt) => NativeDecodeControl::resume(receipt, &mut accepted).map_err(|(error, receipt)| {
                self.continuation = Some(receipt);
                error.to_string()
            })?,
            None => NativeDecodeControl::new_retained(&mut accepted),
        };
        if control.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|bytes| bytes > MAXIMUM_DOCUMENT_ALLOCATED_BYTES) {
            self.continuation = Some(control.pause().map_err(|(error, _)| error.to_string())?);
            return Err("GLB document exceeds its allocation bound".into());
        }
        let result = self.parser.step_source(input, 1, &mut control, effect, false);
        let progress = self.parser.normal_step_progress();
        self.receipt = [progress.copied_items, progress.copied_bytes, progress.retained_capacity_bytes, progress.released_bytes];
        self.continuation = Some(control.pause().map_err(|(error, _)| error.to_string())?);
        if let Some(document) = result.map_err(|error| format!("gltf json parse error: {error}"))? {
            self.document = Some(document);
            return Ok(true);
        }
        Ok(false)
    }

    /// 📤️ Advances a funded dense scene or moves its completed mesh to the caller.
    pub fn project_step(&mut self, bytes: &[u8], maximum_triangles: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<Option<MeshData>, String> {
        self.receipt = [0; 4];
        let document = self.document.as_mut().ok_or("GLB JSON document is not ready")?;
        let scene = self.scene.get_or_insert_with(SceneCursor::default);
        let result = scene.step(document, self.chunks.bin(bytes), maximum_triangles, effect);
        let progress = scene.take_progress();
        self.receipt = [progress.copied_items, progress.copied_bytes, progress.retained_capacity_bytes, progress.released_bytes];
        result.map(|step| match step {
            SceneStep::Complete(mesh) => Some(mesh),
            SceneStep::Working => None,
        })
    }
}
