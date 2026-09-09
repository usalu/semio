//! 🖌️ `register-brush-mesh` command.

use crate::editor::puzzle3d::precompute::{decode_brush_mesh_page_values, Puzzle3dMeshUploadFault, PUZZLE3D_MESH_PAGE_VALUES};
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;
use semio_framework::kernel::Effect;

/// 🥽️ Real GLB geometry for one mesh id. Two admissible forms, and the id-only one is the normal case:
///
/// - `{url, digest?}` alone — the geometry is derived from the process-wide content-addressed mesh
///   store, so a mesh any document already uploaded costs nothing on the wire ever again. A supplied
///   `digest` is verified against the resident geometry, so a stale id never adopts foreign bytes. An
///   identity this guest instantiation cannot serve is not an error the client can only be told about:
///   it is recorded as a re-upload request and published on the world body — see [`request_reupload`].
/// - `{url, digest, page, pageCount, positionsB64?, indicesB64?}` — one page of a first upload. A whole
///   document-scale mesh never fitted the shared retained command's 8 192 raw bytes (the Nakagin
///   capsule `🧊️placeholder.glb` is 73 728 values, 64 KB as a JSON number array), so the client pages it:
///   [`PUZZLE3D_MESH_PAGE_VALUES`] values per page as base64 of little-endian `f32`/`u32` bytes,
///   accumulated in the precompute session's bounded staging area keyed by `(url, digest)` and
///   committed on the last page. Positions fill each page first; the indices stream continues in
///   whatever of the page's value budget is left.
///
/// The decode itself is the `"puzzle3d.mesh-decode"` engine kernel
/// (`✏️editor/⏳️precompute/🦀️.rs`), keyed by `(engine id, url, geometry)`, so one identity decodes once
/// per process and every open document reads the identical derived page.
pub fn register_brush_mesh(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    // 🖌️ A `HostOnly`-lane upload publishes to no store and paints nothing — `registerBrushMesh` is
    // declared `Puzzle3dScopeClass::Quiet` in the ONE scope table (`✏️editor/🦀️.rs`), so every accepting
    // exit of this arm, the shared-mesh fast path included, leaves the UI clean. A refused PAGE stays
    // visible through `Effect::Notify`, which the host applies before it ever consults the scope; a
    // refused IDENTITY widens the scope instead, so the world body republishes the re-upload request —
    // see `request_reupload`.
    let Some(url) = args.and_then(|value| value.get("url")).and_then(Value::as_str) else {
        return;
    };
    if url.len() > MAX_LEAF_BYTES {
        return;
    }
    let digest = args.and_then(|value| value.get("digest")).and_then(Value::as_str);
    let (Some(page), Some(page_count)) = (unsigned(args, "page"), unsigned(args, "pageCount")) else {
        if !ctx.app.precompute.borrow_mut().adopt_shared_mesh(url, digest) {
            request_reupload(ctx, url);
        }
        return;
    };
    let positions = page_values(args, "positionsB64", PUZZLE3D_MESH_PAGE_VALUES).map(|values| values.iter().map(|bytes| f32::from_le_bytes(*bytes)).collect::<Vec<f32>>());
    let indices = page_values(args, "indicesB64", PUZZLE3D_MESH_PAGE_VALUES.saturating_sub(positions.as_ref().map_or(0, Vec::len))).map(|values| values.iter().map(|bytes| u32::from_le_bytes(*bytes)).collect::<Vec<u32>>());
    if args.and_then(|value| value.get("positionsB64")).is_some() && positions.is_none() || args.and_then(|value| value.get("indicesB64")).is_some() && indices.is_none() {
        fault(ctx, url, Puzzle3dMeshUploadFault::Payload);
        return;
    }
    let staged = ctx.app.precompute.borrow_mut().stage_mesh_page(url, digest.unwrap_or_default(), page, page_count, &positions.unwrap_or_default(), &indices.unwrap_or_default());
    if let Err(rejection) = staged {
        fault(ctx, url, rejection);
    }
}

fn unsigned(args: Option<&Value>, key: &str) -> Option<u32> {
    args.and_then(|value| value.get(key)).and_then(Value::as_u64).and_then(|value| u32::try_from(value).ok())
}

fn page_values(args: Option<&Value>, key: &str, budget: usize) -> Option<Vec<[u8; 4]>> {
    decode_brush_mesh_page_values(args.and_then(|value| value.get(key)).and_then(Value::as_str)?, budget)
}

/// 🚨️ A refused page is never a silent drop: without the geometry the brush utility has no collision
/// body at all, so the shell says which identity failed and why instead of leaving placement running
/// against nothing.
fn fault(ctx: &mut Puzzle3dActionCtx<'_>, url: &str, rejection: Puzzle3dMeshUploadFault) {
    ctx.effects.push(Effect::Notify { message: format!("{}: {url}", rejection.code()) });
}

/// 🚚️ A refused *identity-only* announcement is a different animal from a refused page: it means the
/// client's own bookkeeping outlived the guest instantiation that justified it — the classic case is a
/// restored actor, whose `brush_mesh_store` is empty while the tab's module singleton still says
/// "uploaded". A notice would be a dead end (nothing on the host reads a notification's text, so the
/// brush utility would simply stay without a collision body until a full page reload). Recording the
/// identity instead turns the refusal into a request the world body publishes as
/// `interactionJson.meshReuploadUrls`, which the client answers with the page run.
///
/// The scope is widened from the action's declared [`Puzzle3dScopeClass::Quiet`] to the viewport scope
/// for exactly this exit: the request is worthless until the world body carrying it is republished,
/// and a refusal is rare by construction — every one of them ends in the bytes arriving.
///
/// [`Puzzle3dScopeClass::Quiet`]: crate::editor::puzzle3d::Puzzle3dScopeClass::Quiet
fn request_reupload(ctx: &mut Puzzle3dActionCtx<'_>, url: &str) {
    ctx.app.precompute.borrow_mut().request_mesh_reupload(url);
    *ctx.ui_scope = crate::editor::puzzle3d::puzzle3d_viewport_scope();
}

const MAX_LEAF_BYTES: usize = 4 * 1024;
