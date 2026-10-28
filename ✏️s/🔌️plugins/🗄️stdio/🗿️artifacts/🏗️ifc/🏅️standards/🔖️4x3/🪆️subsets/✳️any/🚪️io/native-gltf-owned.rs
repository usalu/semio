//! 🫴️ Static geometry exchange retains natural spans and every typed frontier under the original caller wallet.
use super::*;
use semio_framework_os_kernel::{NativeSnapshotBodyWallet, NativeSnapshotDecodeOwner};
use semio_framework_value::{NativeDecodeControl, RetainedCloneGrant, RetainedCloneProgress, RetireOwned, ValueError, ValueRefusalKind};
use std::mem::size_of;
#[path = "native-gltf-export.rs"]
mod export;
#[path = "native-gltf-graph.rs"]
mod graph;
#[path = "native-gltf-import.rs"]
mod import;
#[path = "native-gltf-json.rs"]
mod json;
#[path = "native-gltf-receive.rs"]
mod receive;
#[path = "native-gltf-write.rs"]
mod write;
const MAX_BYTES: usize = 64 * 1024 * 1024;
const MAX_TOKENS: usize = 2_000_000;
const MAX_MEMBERS: usize = 4096;
const MAX_DEPTH: usize = 64;
fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
fn limit(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::WorkLimit, message)
}
struct Port<'a, 'b, 'c> {
    native: &'a mut NativeDecodeControl<'b>,
    body: &'c mut NativeSnapshotBodyWallet,
}
impl Port<'_, '_, '_> {
    fn effect<T: Copy>(&mut self, bytes: usize, f: impl FnOnce() -> T) -> Result<T, ValueError> {
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_depth: 2, ..Default::default() })?;
        let value = f();
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })?;
        self.native.step()?;
        Ok(value)
    }
    fn vector<T>(&mut self, count: usize, out: &mut Vec<T>) -> Result<(), ValueError> {
        self.body.allocate_vec_into(self.native, count, out, 2)?;
        self.native.step()
    }
    fn text(&mut self, count: usize, out: &mut String) -> Result<(), ValueError> {
        if out.capacity() != 0 {
            return Err(invalid("natural glTF text frontier was already populated"));
        }
        self.native.checkpoint()?;
        self.body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<String>(), maximum_capacity_bytes: count, maximum_depth: 2, ..Default::default() })?;
        self.native.charge(count)?;
        out.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "natural glTF text allocation failed"))?;
        self.body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<String>(), retained_capacity_bytes: out.capacity(), ..Default::default() })?;
        self.native.step()
    }
}
#[derive(Clone, Copy, Default, RetireOwned)]
struct Natural {
    json: (usize, usize),
    bin: Option<(usize, usize)>,
    glb: bool,
}
fn u32_at(source: &[u8], at: usize, port: &mut Port<'_, '_, '_>) -> Result<u32, ValueError> {
    let bytes = source.get(at..at + 4).ok_or_else(|| invalid("natural GLB header is truncated"))?;
    port.effect(4, || u32::from_le_bytes(bytes.try_into().unwrap()))
}
fn framing(source: &[u8], port: &mut Port<'_, '_, '_>) -> Result<Natural, ValueError> {
    if source.len() > MAX_BYTES {
        return Err(limit("natural glTF source exceeds 64 MiB admission"));
    }
    if source.len() < 4 || !port.effect(4, || source[..4] == *b"glTF")? {
        return Ok(Natural { json: (0, source.len()), bin: None, glb: false });
    }
    if source.len() < 20 || u32_at(source, 4, port)? != 2 || u32_at(source, 8, port)? as usize != source.len() {
        return Err(invalid("natural GLB version or declared extent differs"));
    }
    let mut result = Natural { glb: true, ..Default::default() };
    let mut at = 12usize;
    let mut ordinal = 0;
    while at < source.len() {
        let length = u32_at(source, at, port)? as usize;
        let kind = u32_at(source, at + 4, port)?;
        let start = at.checked_add(8).ok_or_else(|| invalid("natural GLB chunk offset overflow"))?;
        let end = start.checked_add(length).filter(|end| *end <= source.len()).ok_or_else(|| invalid("natural GLB chunk exceeds original source"))?;
        if length % 4 != 0 {
            return Err(invalid("natural GLB chunk extent is not four-byte aligned"));
        }
        match kind {
            0x4e4f534a if ordinal == 0 => result.json = (start, end),
            0x4e4f534a => return Err(invalid("natural GLB contains a repeated or misplaced JSON chunk")),
            0x004e4942 if ordinal == 1 && result.bin.is_none() => result.bin = Some((start, end)),
            0x004e4942 => return Err(invalid("natural GLB contains a repeated or misplaced BIN chunk")),
            _ if ordinal == 0 => return Err(invalid("natural GLB first chunk is not JSON")),
            _ => {}
        }
        port.effect(size_of::<usize>(), || {
            at = end;
            ordinal += 1;
        })?;
    }
    if ordinal == 0 || result.json.0 == result.json.1 {
        return Err(invalid("natural GLB JSON chunk is empty"));
    }
    Ok(result)
}
#[derive(Default, RetireOwned)]
struct NaturalReceiving {
    catalog: json::Catalog,
    framing: Natural,
    snapshot: Option<GltfSnapshot>,
}
fn read_natural(source: &[u8], owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<GltfSnapshot, ValueError> {
    owner.receive::<NaturalReceiving, GltfSnapshot>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            body.admit_frontier(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<NaturalReceiving>(), maximum_depth: 2, ..Default::default() })?;
            *slot = Some(NaturalReceiving::default());
            body.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<NaturalReceiving>(), ..Default::default() })?;
            let state = slot.as_mut().unwrap();
            let mut port = Port { native, body };
            state.framing = framing(source, &mut port)?;
            state.catalog.receive(source, state.framing, &mut port)?;
            receive::snapshot(source, state, &mut port)?;
            port.effect(size_of::<GltfSnapshot>(), || ())?;
            Ok(state.snapshot.take().unwrap())
        })
    })
}
const IMPORT_LOSS: &str = "Static triangle positions, indices and authored world transforms are baked in SI metres/Z-up into new IFC tessellated proxies. Source glTF hierarchy, identities, materials, textures, normals, UVs, cameras, extras and non-geometric metadata are omitted. This geometry receiving route does not claim full foreign glTF validation or lossless source reconstruction. Animation, skins, morphs, sparse accessors, external buffers and selected extensions refuse atomically.";
const EXPORT_LOSS: &str = "Explicit IFC Body triangulated face sets, local placements and representation world coordinate systems are baked into glTF SI metres/Y-up with FLOAT coordinates and new mesh identities. IFC relationships, properties, materials, normals, source identities, presentation and analysis semantics are omitted. Mapped, swept, boolean, boundary, geographic and relational-void evaluation are unsupported and refuse atomically; this route does not claim complete IFC geometry or MVD coverage.";
fn warning(message: &'static str, owner: &mut NativeSnapshotDecodeOwner<'_, '_>) -> Result<Vec<Diagnostic>, ValueError> {
    owner.receive::<Vec<Diagnostic>, Vec<Diagnostic>>(|slot, native, body| {
        native.scoped_stage(|native| {
            native.begin_stage(0)?;
            let mut p = Port { native, body };
            p.effect(size_of::<Vec<Diagnostic>>(), || *slot = Some(Vec::new()))?;
            let out = slot.as_mut().unwrap();
            p.vector(1, out)?;
            p.effect(size_of::<Diagnostic>(), || {
                let mut d = Diagnostic::error("exchange.partial-geometry", TextSpan::default(), String::new());
                d.severity = Severity::Warning;
                out.push(d);
            })?;
            p.text(message.len(), &mut out[0].message)?;
            for chunk in message.as_bytes().chunks(256) {
                let chunk = std::str::from_utf8(chunk).unwrap();
                p.effect(chunk.len(), || out[0].message.push_str(chunk))?;
            }
            p.effect(size_of::<Vec<Diagnostic>>(), || ())?;
            Ok(slot.take().unwrap())
        })
    })
}
#[derive(Default, RetireOwned)]
struct Route {
    gltf: Option<GltfSnapshot>,
    graph: Option<IfcSnapshot>,
    wire: Option<Vec<u8>>,
    diagnostics: Vec<Diagnostic>,
}
fn header(control: &mut IoRunControl<'_, '_>, bytes: usize, encode: bool) -> Result<(), ValueError> {
    if encode {
        let mut owner = control.snapshot_encode()?;
        let g = owner.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < bytes {
            return Err(limit("geometry route output header exceeds original grant"));
        }
        owner.native().checkpoint()?;
        owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })
    } else {
        let mut owner = control.snapshot_decode()?;
        let g = owner.remaining_grant();
        if g.maximum_items == 0 || g.maximum_depth == 0 || g.maximum_copy_bytes < bytes {
            return Err(limit("geometry route receiving header exceeds original grant"));
        }
        owner.native().checkpoint()?;
        owner.record_progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() })
    }
}
pub(super) fn import_payload(source: &IoPayload, natural: bool, control: &mut IoRunControl<'_, '_>) -> IoResult<IfcSnapshot> {
    let IoPayload::Binary(bytes) = source else {
        return Err(IoError::from_value_error(invalid("registered glTF/GLB geometry import requires its declared original binary source")));
    };
    if natural && !bytes.starts_with(b"glTF") {
        return Err(IoError::from_value_error(invalid("natural GLB carrier does not have its declared GLB header")));
    }
    control
        .receive_nested::<Route, IoOutcome<IfcSnapshot>>(|slot, original| {
            header(original, size_of::<Route>(), false)?;
            *slot = Some(Route::default());
            let state = slot.as_mut().unwrap();
            state.gltf = Some(if natural { read_natural(bytes, &mut original.snapshot_decode()?)? } else { gltf_io::binary::snapshot::owned_pack::geometry::receive_geometry_pack(bytes, &mut original.snapshot_decode()?)? });
            state.graph = Some(import::receive(state.gltf.as_ref().unwrap(), &mut original.snapshot_decode()?)?);
            state.diagnostics = warning(IMPORT_LOSS, &mut original.snapshot_decode()?)?;
            header(original, size_of::<IoOutcome<IfcSnapshot>>(), false)?;
            Ok(IoOutcome { value: state.graph.take().unwrap(), diagnostics: std::mem::take(&mut state.diagnostics) })
        })
        .map_err(IoError::from_value_error)
}
pub(super) fn export_payload(source: &IfcSnapshot, natural: bool, control: &mut IoRunControl<'_, '_>) -> IoResult<IoPayload> {
    control
        .receive_nested::<Route, IoOutcome<IoPayload>>(|slot, original| {
            header(original, size_of::<Route>(), false)?;
            *slot = Some(Route::default());
            let state = slot.as_mut().unwrap();
            state.gltf = Some(export::receive(source, &mut original.snapshot_decode()?)?);
            state.wire = Some(if natural {
                write::encode(state.gltf.as_ref().unwrap(), &mut original.snapshot_encode()?)?
            } else {
                gltf_io::binary::snapshot::owned_pack::geometry::encode_geometry_pack(state.gltf.as_ref().unwrap(), &mut original.snapshot_encode()?)?
            });
            state.diagnostics = warning(EXPORT_LOSS, &mut original.snapshot_decode()?)?;
            header(original, size_of::<IoOutcome<IoPayload>>(), true)?;
            Ok(IoOutcome { value: IoPayload::Binary(state.wire.take().unwrap()), diagnostics: std::mem::take(&mut state.diagnostics) })
        })
        .map_err(IoError::from_value_error)
}
#[cfg(test)]
#[path = "native-gltf-owned-tests.rs"]
mod tests;
