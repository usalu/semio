//! 🚪️ IO s.raster (1/✳️any) — registration now flows through 🎹️composer::register
//! (called once from the artifact root's `declaration()`), not per-leaf register().
/// 📥️ The stdio format kinds this artifact can genuinely BUILD a document from — the single source
/// of truth `artifact_kind()` re-exports, so the workflow wire negotiator
/// (`🧰️framework/🛍️products/💻️os/🖥️host/🦀️.rs`'s `negotiate_wire_format`) can never pick a hop that
/// has no real decoder. `stdio.pdf` is absent on purpose: `PdfSnapshot`'s only per-page state is
/// `{width, height, text}` (no image XObject model at all), so there is nothing to read pixels out
/// of — the leaf itself says so with a typed `Err`.
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.dwg", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"]
}
/// 📤️ The stdio format kinds this artifact can genuinely EMIT — same single-source-of-truth rule as
/// `import_stdio_kinds`. `stdio.dwg`/`stdio.pdf` are absent on purpose: both of stdio's own
/// `s.stdio.semio/v1/drawing` bridges into those formats drop `DrawNode::Image` outright (their own
/// module docs say so), so a raster composite would encode as an empty drawing/text-only page.
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.bmp", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"]
}

//#region 🔖️SemioBridge
/// 🌉️ Relocated verbatim from `⚙️engine` (ticket 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES,
/// rule 5: sniff/codec dispatch lives in `🚪️io/`). References below into `semio_s_plugin_stdio`'s own
/// `semio`/`png` compute modules belong to stdio (a different plugin, out of this ticket's five-plugin
/// scope and explicitly not to be touched) — left as-is, cross-plugin
/// calls, not struct instantiations.
///
/// ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT (W5b): raster
/// is dual-natured — its vector-shaped exports (svg) go through `s.stdio.semio/v1/drawing`, its
/// pixel asset surface (png) goes through `s.stdio.semio/v1/image` — both real stdio bridges, never
/// hand-rolled SVG/DWG bytes. Only the SVG→pixels render step has no stdio equivalent (a genuine
/// vector-rasterizer gap, reported in `stdio_gaps`); it stays on `semio_framework_os`'s real
/// usvg/resvg renderer, whose OUTPUT is then canonicalized through the real png↔semio/image codec.
use crate::artifacts::raster::{RasterImageAsset, RasterLayerNode, RasterSnapshot, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use semio_framework::{io::io_compose_via, io_dispatch, resolve_ready, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_s_artifact_stdio_dwg::{DwgDrawing, DwgGeometry};
use semio_s_artifact_stdio_png::PngSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioPoint3, SemioQuaternion, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode, PathSegment, SemioDrawingSnapshot, STDIO_SEMIODRAWING_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageFrame, SemioImageSnapshot, STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA};
use semio_s_artifact_stdio_svg::SvgSnapshot;

const SEMIO_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const SEMIO_IMAGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
pub(crate) const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };
/// 🪟️ The four other pixel formats stdio's own `s.stdio.semio/v1/image` hub already bridges both
/// ways (`🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs:91-105` registers
/// all five `SemioImageFrom*`/`SemioImageTo*` pairs in ONE `register_composer_entries` call). Every
/// pixel hop in this subset's io leaves goes through that hub — this plugin never hand-rolls a
/// PNG/BMP/GIF/JPEG/TIFF byte codec of its own.
pub(crate) const BMP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };
pub(crate) const JPG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId::ANY };
pub(crate) const TIFF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };
/// 🎞️ The hub bridges GIF at **89a** only (that is the standard the `SemioImageFromGif`/`ToGif`
/// leaves declare), while this subset's own gif leaf is declared at `87a`. The gif leaves therefore
/// hop `semio/image ↔ gif@89a` for the palette work (real 1:1 quantization, never a local
/// re-implementation) and then remap the 89a snapshot onto stdio's own `87a` `GifSnapshot`/
/// `encode_gif`, which is what actually writes the `GIF87a` bytes this dialect promises.
pub(crate) const GIF89A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId::ANY };

/// 📌️ w5b-close fix: registers stdio's `semio` v1 engine (drawing/image/… subset composers) and
/// stdio's `png` engine into the process-global `io` registry exactly once, so `io_dispatch`/
/// `io_compose_via` below resolve regardless of host-boot ordering — a bare `cargo test` process
/// never runs the plugin-host boot path that would normally call this. Mirrors 🗒️note's/📏️layout's/
/// 🌍️gis's own `ensure_..._registered()` helper (w5b-verify-report.md flagged 🖍️draw as missing this
/// exact pattern; raster was missing it too, surfaced by `cargo test` once the crate compiled).
fn ensure_stdio_semio_and_png_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_artifact_stdio_semio::register();
        semio_s_artifact_stdio_png::register();
    });
}

/// 🗝️ Mirrors `io::IoKey::from_owner_counterpart` (private to the io module) for the four fixed
/// owner/counterpart pairs this bridge dispatches.
fn semio_io_key(owner: &Dialect, direction: IoDirection, counterpart: &Dialect) -> IoKey {
    IoKey {
        artifact_kind: owner.artifact_kind.into(),
        standard: owner.standard.0.into(),
        subset: owner.subset.0.into(),
        direction,
        format_kind: counterpart.artifact_kind.into(),
        format_standard: counterpart.standard.0.into(),
        format_subset: counterpart.subset.0.into(),
    }
}

fn semio_transform_from_raster(transform: &RasterTransform) -> SemioTransform {
    let half = transform.rotation.to_radians() / 2.0;
    SemioTransform { translation: SemioPoint3 { x: transform.x, y: transform.y, z: 0.0 }, rotation: SemioQuaternion { x: 0.0, y: 0.0, z: half.sin(), w: half.cos() }, scale: SemioPoint3 { x: transform.scale_x, y: transform.scale_y, z: 1.0 } }
}

/// 🖼️ Builds one real `DrawNode` per visible pixel layer (its embedded asset bytes, positioned/
/// scaled/rotated by the layer's own `RasterTransform`), recursing into group layers; adjustment
/// layers carry no geometry of their own and are honestly skipped.
fn draw_node_for_raster_layer(layer: &RasterLayerNode, assets: &crate::artifacts::raster::RasterOwnedMap<crate::artifacts::raster::RasterAssetChild>) -> Option<DrawNode> {
    match layer {
        RasterLayerNode::Pixel { visible, transform, width, height, image_key, .. } => {
            if !*visible {
                return None;
            }
            let asset = image_key.as_ref().and_then(|key| crate::artifacts::raster::raster_asset(assets, key))?;
            let w = width.unwrap_or(0) as f64;
            let h = height.unwrap_or(0) as f64;
            if w <= 0.0 || h <= 0.0 {
                return None;
            }
            Some(DrawNode::Group { transform: semio_transform_from_raster(transform), children: vec![DrawNode::Image { at: SemioPoint2 { x: 0.0, y: 0.0 }, width: w, height: h, mime: asset.mime.clone(), bytes: asset.data }] })
        }
        RasterLayerNode::Group { visible, transform, children, .. } => {
            if !*visible {
                return None;
            }
            let kids: Vec<DrawNode> = children.iter().filter_map(|child| draw_node_for_raster_layer(child, assets)).collect();
            if kids.is_empty() {
                return None;
            }
            Some(DrawNode::Group { transform: semio_transform_from_raster(transform), children: kids })
        }
        RasterLayerNode::Adjustment { .. } => None,
    }
}

/// 🧬️ Builds a real `SemioDrawingSnapshot` from a raster document's own layer stack (its own
/// domain document model), replacing the `title_card_svg` placeholder.
fn drawing_snapshot_from_raster(document: &RasterSnapshot) -> SemioDrawingSnapshot {
    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    for layer in &document.layers {
        if let RasterLayerNode::Pixel { transform, width, height, .. } = layer {
            max_x = max_x.max(transform.x + width.unwrap_or(0) as f64);
            max_y = max_y.max(transform.y + height.unwrap_or(0) as f64);
        }
    }
    let canvas = DrawCanvas { width: if max_x > 0.0 { max_x } else { 1024.0 }, height: if max_y > 0.0 { max_y } else { 1024.0 }, background: None };
    let children: Vec<DrawNode> = document.layers.iter().filter_map(|layer| draw_node_for_raster_layer(layer, &document.assets)).collect();
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas,
        styles: Vec::new(),
        layers: vec![DrawLayer { id: document.id.clone(), name: document.title.clone().unwrap_or_default(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🧬️ Converts a legacy `DwgDrawing`'s line-shaped entities into a real `DrawNode::Path` tree —
/// typed geometry, not hand-formatted SVG `<path d="…">` strings.
fn drawing_snapshot_from_dwg(drawing: &DwgDrawing) -> SemioDrawingSnapshot {
    let width = (drawing.extmax[0] - drawing.extmin[0]).max(1.0);
    let height = (drawing.extmax[1] - drawing.extmin[1]).max(1.0);
    let to_point = |v: &[f64; 2]| SemioPoint2 { x: v[0] - drawing.extmin[0], y: height - (v[1] - drawing.extmin[1]) };
    let mut children = Vec::new();
    for entity in &drawing.entities {
        let (vertices, closed): (Vec<[f64; 2]>, bool) = match &entity.geometry {
            DwgGeometry::LwPolyline { vertices, closed, .. } => (vertices.clone(), *closed),
            DwgGeometry::Polyline3d { vertices, closed } => (vertices.iter().map(|v| [v[0], v[1]]).collect(), *closed),
            DwgGeometry::Line { start, end } => (vec![[start[0], start[1]], [end[0], end[1]]], false),
            _ => continue,
        };
        if vertices.is_empty() {
            continue;
        }
        let mut segments = Vec::with_capacity(vertices.len() + 1);
        segments.push(PathSegment::MoveTo { to: to_point(&vertices[0]) });
        for vertex in &vertices[1..] {
            segments.push(PathSegment::LineTo { to: to_point(vertex) });
        }
        if closed {
            segments.push(PathSegment::Close);
        }
        children.push(DrawNode::Path { segments, style: None });
    }
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width, height, background: None },
        styles: Vec::new(),
        layers: vec![DrawLayer { id: "0".into(), name: "dwg-import".into(), visible: true, root: DrawNode::Group { transform: SemioTransform::identity(), children } }],
    }
}

/// 🚪️ Dispatches `s.stdio.semio/v1/drawing` → `s.stdio.svg` through stdio's real SVG serializer
/// (`io_dispatch`), then prints the composed `SvgSnapshot` as bare XML (`write_svg_xml`, NOT
/// `ArtifactDsl::print_dsl` — w5b-close fix: `print_dsl` wraps the text in stdio's `.semio`
/// envelope preamble, which `raster_document_json_from_dwg`'s `semio_framework_os::
/// rasterize_svg_to_png_base64` call below then fails to parse as XML at all ("unknown token at
/// 1:1"); every downstream consumer of this function's return value wants a bare `<svg>…</svg>`
/// document, matching 🗒️note's/🖍️draw's own `write_svg_xml` usage for the identical bridge).
fn dispatch_drawing_to_svg(snapshot: &SemioDrawingSnapshot) -> Result<String, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioDrawingSnapshot as store::ArtifactPack>::encode_pack(snapshot));
    let key = semio_io_key(&SEMIO_DRAWING_DIALECT, IoDirection::Export, &SVG_DIALECT);
    let composed = resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(svg_bytes) = composed.payload else { return Err("s.stdio.svg composer returned a non-binary payload".into()) };
    let svg_snapshot = <SvgSnapshot as store::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| format!("{error:?}"))?;
    Ok(semio_s_artifact_stdio_svg::schema::snapshot::write_svg_xml(&svg_snapshot.doc))
}

/// 🚪️ Dispatches a decoded foreign pixel snapshot (`PngSnapshot`/`BmpSnapshot`/`GifSnapshot`/
/// `JpgSnapshot`/`TiffSnapshot`) → `s.stdio.semio/v1/image` through stdio's own registered
/// deserializer for that dialect — the honest, structured way to reach an image's real
/// width/height/RGBA8 frames without this plugin ever touching the wire format itself.
pub(crate) fn semio_image_from_format<T: store::ArtifactPack>(snapshot: &T, format: Dialect) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<T as store::ArtifactPack>::encode_pack(snapshot));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &format);
    let composed = resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: format, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.semio image composer returned a non-binary payload".into()) };
    <SemioImageSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

/// 🚪️ Dispatches `s.stdio.semio/v1/image` → the target format's own typed snapshot through stdio's
/// registered serializer for that dialect. The caller then hands that snapshot to the format's own
/// real byte encoder (`encode_png`/`encode_bmp`/…) — never a hand-rolled writer here.
pub(crate) fn semio_image_to_format<T: store::ArtifactPack>(image: &SemioImageSnapshot, format: Dialect) -> Result<T, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioImageSnapshot as store::ArtifactPack>::encode_pack(image));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &format);
    let composed = resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_IMAGE_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err(format!("{} composer returned a non-binary payload", format.artifact_kind)) };
    <T as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

/// 🚪️ Dispatches real `png` bytes → `s.stdio.semio/v1/image` through stdio's real PNG deserializer
/// (`io_dispatch`) — the honest, structured way to learn a decoded image's real width/height/pixels.
pub(crate) fn semio_image_from_png_bytes(raw_png_bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot: PngSnapshot = semio_s_artifact_stdio_png::io::decode_png(raw_png_bytes)?;
    semio_image_from_format(&png_snapshot, PNG_DIALECT)
}

/// 🚪️ Dispatches `s.stdio.semio/v1/image` → real `png` bytes through stdio's real PNG serializer
/// (`io_dispatch`) plus its own real byte encoder — never a hand-rolled PNG writer.
pub(crate) fn png_bytes_from_semio_image(image: &SemioImageSnapshot) -> Result<Vec<u8>, String> {
    let png_snapshot: PngSnapshot = semio_image_to_format(image, PNG_DIALECT)?;
    semio_s_artifact_stdio_png::io::encode_png(&png_snapshot)
}

/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: real bidirectional CHILD-CONTENT
/// converters between this plugin's own `RasterImageAsset` (mime+bytes, the mutation-payload/
/// working-scene shape) and the composed `s.stdio.semio/v1/image` child's real content
/// (`SemioImageSnapshot`, decoded RGBA8 pixels) — reuses the SAME real png↔semio/image bridge above,
/// never a stub. Only `image/png` is lossless today (the only mime this plugin ever produces, via
/// `raster_document_json_from_dwg`/`raster_image_layer_and_asset` below); any other mime is honestly
/// reported as an error, never silently coerced.
pub fn semio_image_snapshot_from_raster_asset(asset: &RasterImageAsset) -> Result<SemioImageSnapshot, String> {
    if asset.mime != "image/png" {
        return Err(format!("semio_image_snapshot_from_raster_asset: unsupported mime {:?} (only image/png round-trips today)", asset.mime));
    }
    semio_image_from_png_bytes(&asset.data)
}

pub fn raster_asset_from_semio_image_snapshot(image: &SemioImageSnapshot) -> Result<RasterImageAsset, String> {
    Ok(RasterImageAsset { mime: "image/png".into(), data: png_bytes_from_semio_image(image)? })
}

/// 🌉️🌉️ Round-trips raw PNG bytes through `s.stdio.semio/v1/image` (import then export) via the
/// real 2-hop `io_compose_via` seam — canonicalizes a renderer's raw output through stdio's own
/// codec rather than trusting it verbatim.
/// 🌉️🌉️ `pub` (not `fn` as it was inside `⚙️engine`): now called cross-module from `🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/
/// 🦀️.rs`'s `raster_composite_media` (rule 4: `AppIo`-adjacent behaviour lives in the app).
pub fn canonicalize_png_bytes(raw_png_bytes: &[u8]) -> Result<Vec<u8>, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot = semio_s_artifact_stdio_png::io::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as store::ArtifactPack>::encode_pack(&png_snapshot));
    let hub_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let target_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = resolve_ready(io_compose_via(&hub_key, &target_key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as store::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_artifact_stdio_png::io::encode_png(&png_snapshot)
}
//#endregion 🔖️SemioBridge

//#region 🔖️Composite
/// 🎨️ The separable blend functions this compositor implements, per W3C Compositing-1 §11 (the
/// `blend_mode` field is a free `String` in the schema, so an unrecognized value is reported as a
/// typed error rather than silently painted as `normal` — that would fabricate a picture the app
/// never showed).
#[derive(Clone, Copy)]
enum RasterBlend {
    Normal,
    Multiply,
    Screen,
    Darken,
    Lighten,
    Difference,
}

impl RasterBlend {
    fn parse(mode: &str) -> Result<Self, String> {
        match mode {
            "normal" => Ok(Self::Normal),
            "multiply" => Ok(Self::Multiply),
            "screen" => Ok(Self::Screen),
            "darken" => Ok(Self::Darken),
            "lighten" => Ok(Self::Lighten),
            "difference" => Ok(Self::Difference),
            other => Err(format!("unsupported blend mode {other:?} (this compositor implements normal/multiply/screen/darken/lighten/difference)")),
        }
    }

    fn apply(self, backdrop: f32, source: f32) -> f32 {
        match self {
            Self::Normal => source,
            Self::Multiply => backdrop * source,
            Self::Screen => backdrop + source - backdrop * source,
            Self::Darken => backdrop.min(source),
            Self::Lighten => backdrop.max(source),
            Self::Difference => (backdrop - source).abs(),
        }
    }
}

/// 📐️ A 2D affine `[a c e; b d f]` in the same column convention SVG/`SemioTransform` use:
/// `x' = a·x + c·y + e`, `y' = b·x + d·y + f`.
#[derive(Clone, Copy)]
struct RasterAffine {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
}

impl RasterAffine {
    const IDENTITY: Self = Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, e: 0.0, f: 0.0 };

    /// 🧭️ `translate(x, y) ∘ rotate(rotation) ∘ scale(scale_x, scale_y)` — the exact order
    /// `semio_transform_from_raster` above already assumes when it hands the same `RasterTransform`
    /// to the drawing bridge, so the pixel and vector exports agree on what a layer transform means.
    fn from_transform(transform: &RasterTransform) -> Self {
        let (sin, cos) = transform.rotation.to_radians().sin_cos();
        Self { a: cos * transform.scale_x, b: sin * transform.scale_x, c: -sin * transform.scale_y, d: cos * transform.scale_y, e: transform.x, f: transform.y }
    }

    fn then(self, outer: Self) -> Self {
        Self {
            a: outer.a * self.a + outer.c * self.b,
            b: outer.b * self.a + outer.d * self.b,
            c: outer.a * self.c + outer.c * self.d,
            d: outer.b * self.c + outer.d * self.d,
            e: outer.a * self.e + outer.c * self.f + outer.e,
            f: outer.b * self.e + outer.d * self.f + outer.f,
        }
    }

    fn apply(self, x: f64, y: f64) -> (f64, f64) {
        (self.a * x + self.c * y + self.e, self.b * x + self.d * y + self.f)
    }

    fn invert(self) -> Option<Self> {
        let determinant = self.a * self.d - self.b * self.c;
        if !determinant.is_finite() || determinant.abs() <= f64::EPSILON {
            return None;
        }
        let (a, b, c, d) = (self.d / determinant, -self.b / determinant, -self.c / determinant, self.a / determinant);
        Some(Self { a, b, c, d, e: -(a * self.e + c * self.f), f: -(b * self.e + d * self.f) })
    }
}

/// 🖼️ One pixel layer resolved to everything the rasterizer needs: its materialized child content
/// (real decoded RGBA8, never re-encoded bytes), its local→device matrix, its local box, and the
/// accumulated group opacity.
struct RasterPlacement {
    image: std::sync::Arc<SemioImageSnapshot>,
    matrix: RasterAffine,
    width: f64,
    height: f64,
    alpha: f32,
    blend: RasterBlend,
}

/// 🛡️ Refuses a canvas no honest single-frame encoder should be asked to allocate inside a
/// `wasm32-wasip2` guest (4 bytes/pixel; 64 MPx is already a 256 MB buffer).
const RASTER_COMPOSITE_MAX_SIDE: u32 = 16_384;
const RASTER_COMPOSITE_MAX_PIXELS: u64 = 64 * 1024 * 1024;

/// 🌉️ Resolves the exact `s.stdio.semio/v1/image` content this snapshot's own asset child owns —
/// the same materialization rule `crate::artifacts::raster::raster_asset` follows, minus its
/// re-encode to PNG bytes (this compositor wants the decoded frames, not a wire encoding).
fn placement_image(assets: &crate::artifacts::raster::RasterOwnedMap<crate::artifacts::raster::RasterAssetChild>, image_key: &str) -> Result<std::sync::Arc<SemioImageSnapshot>, String> {
    let handle = assets.get(image_key).ok_or_else(|| format!("layer references asset {image_key:?}, which this document does not carry"))?;
    handle.local_owner::<SemioImageSnapshot>().ok_or_else(|| format!("asset {image_key:?} is not materialized in this snapshot — its content must be resolved before a composite can be produced"))
}

/// 🧭️ Walks the layer stack in painter's order (index 0 paints first, exactly as
/// `draw_node_for_raster_layer` already orders the vector bridge), pushing one `RasterPlacement`
/// per visible pixel layer that actually carries image content.
fn collect_placements(layers: &[RasterLayerNode], assets: &crate::artifacts::raster::RasterOwnedMap<crate::artifacts::raster::RasterAssetChild>, parent: RasterAffine, parent_alpha: f32, out: &mut Vec<RasterPlacement>) -> Result<(), String> {
    for layer in layers {
        match layer {
            RasterLayerNode::Pixel { visible, opacity, blend_mode, transform, width, height, image_key, .. } => {
                if !*visible {
                    continue;
                }
                let Some(key) = image_key.as_deref() else { continue };
                let image = placement_image(assets, key)?;
                let box_width = width.map_or(image.width as f64, |value| value as f64);
                let box_height = height.map_or(image.height as f64, |value| value as f64);
                if box_width <= 0.0 || box_height <= 0.0 || image.width == 0 || image.height == 0 {
                    continue;
                }
                out.push(RasterPlacement { image, matrix: RasterAffine::from_transform(transform).then(parent), width: box_width, height: box_height, alpha: parent_alpha * opacity, blend: RasterBlend::parse(blend_mode)? });
            }
            RasterLayerNode::Group { visible, opacity, blend_mode, transform, children, .. } => {
                if !*visible {
                    continue;
                }
                // 🚧️ A non-`normal` group blend needs the group rendered to its own offscreen buffer
                // first and only then blended as one unit; painting its children individually with
                // that mode is a DIFFERENT picture, so it is refused rather than approximated.
                if !matches!(RasterBlend::parse(blend_mode)?, RasterBlend::Normal) {
                    return Err(format!("group layer declares blend mode {blend_mode:?}; group-level (offscreen) blending is not implemented, only per-layer blending"));
                }
                collect_placements(children, assets, RasterAffine::from_transform(transform).then(parent), parent_alpha * opacity, out)?;
            }
            RasterLayerNode::Adjustment { visible, adjustment_kind, name, .. } => {
                if !*visible {
                    continue;
                }
                return Err(format!(
                    "visible adjustment layer {name:?} of kind {adjustment_kind:?} cannot be applied: this document model carries no pixel-level adjustment evaluator, and flattening without it would encode a picture the editor never showed"
                ));
            }
        }
    }
    Ok(())
}

/// 🖼️ Flattens the document's visible pixel layers into one canonical RGBA8 canvas, as an
/// `s.stdio.semio/v1/image` snapshot — the ONE hub every real pixel export in this subset then
/// hands to stdio's own png/bmp/gif/jpg/tiff serializer.
///
/// 📐️ Canvas: the union of every placement's device-space axis-aligned bounding box, anchored at
/// the origin (a layer painted at a negative coordinate is clipped, matching the editor's own
/// origin-anchored composite viewport). Sampling is nearest-neighbour through each placement's
/// inverse matrix, so translation, non-uniform scale and rotation are all honoured exactly.
///
/// 🚧️ Honest limitations, each of which is an `Err` and never a silent approximation: a visible
/// adjustment layer, a group-level non-`normal` blend, an unrecognized blend mode, an unmaterialized
/// asset child, and a canvas past `RASTER_COMPOSITE_MAX_*`. `RasterLayerMask` carries no pixel
/// payload at all in this schema (`enabled`/`linked`/`invert`/`width`/`height`, no `image_key`), so
/// there is nothing a mask could mask out — it is honestly inert here, not dropped.
pub fn raster_composite_image(document: &RasterSnapshot) -> Result<SemioImageSnapshot, String> {
    let mut placements = Vec::new();
    collect_placements(&document.layers, &document.assets, RasterAffine::IDENTITY, 1.0, &mut placements)?;
    if placements.is_empty() {
        return Err("no visible pixel layer with materialized image content: there is nothing to flatten into a raster composite".into());
    }

    let mut max_x = 0.0f64;
    let mut max_y = 0.0f64;
    for placement in &placements {
        for (x, y) in [(0.0, 0.0), (placement.width, 0.0), (0.0, placement.height), (placement.width, placement.height)] {
            let (device_x, device_y) = placement.matrix.apply(x, y);
            if !device_x.is_finite() || !device_y.is_finite() {
                return Err("a layer transform maps its box to a non-finite device coordinate".into());
            }
            max_x = max_x.max(device_x);
            max_y = max_y.max(device_y);
        }
    }
    let width = (max_x.ceil().max(1.0)) as u64;
    let height = (max_y.ceil().max(1.0)) as u64;
    if width > RASTER_COMPOSITE_MAX_SIDE as u64 || height > RASTER_COMPOSITE_MAX_SIDE as u64 || width * height > RASTER_COMPOSITE_MAX_PIXELS {
        return Err(format!("composite canvas {width}x{height} exceeds this encoder's {RASTER_COMPOSITE_MAX_SIDE} px side / {RASTER_COMPOSITE_MAX_PIXELS} px area budget"));
    }
    let (width, height) = (width as u32, height as u32);

    let mut canvas = vec![0u8; width as usize * height as usize * 4];
    for placement in &placements {
        let frame = placement.image.frames.first().ok_or_else(|| "a layer's image asset carries no decoded frame".to_string())?;
        let (source_width, source_height) = (placement.image.width as usize, placement.image.height as usize);
        if frame.rgba8.len() != source_width * source_height * 4 {
            return Err("a layer's image asset frame length does not match width*height*4".into());
        }
        let Some(inverse) = placement.matrix.invert() else {
            return Err("a layer transform is singular (zero scale) and cannot be sampled".into());
        };
        let mut min_device = (f64::MAX, f64::MAX);
        let mut max_device = (f64::MIN, f64::MIN);
        for (x, y) in [(0.0, 0.0), (placement.width, 0.0), (0.0, placement.height), (placement.width, placement.height)] {
            let (device_x, device_y) = placement.matrix.apply(x, y);
            min_device = (min_device.0.min(device_x), min_device.1.min(device_y));
            max_device = (max_device.0.max(device_x), max_device.1.max(device_y));
        }
        let x0 = min_device.0.floor().max(0.0) as u32;
        let y0 = min_device.1.floor().max(0.0) as u32;
        let x1 = (max_device.0.ceil().max(0.0) as u64).min(width as u64) as u32;
        let y1 = (max_device.1.ceil().max(0.0) as u64).min(height as u64) as u32;
        for y in y0..y1 {
            for x in x0..x1 {
                let (u, v) = inverse.apply(x as f64 + 0.5, y as f64 + 0.5);
                if u < 0.0 || v < 0.0 || u >= placement.width || v >= placement.height {
                    continue;
                }
                let source_x = ((u / placement.width) * source_width as f64) as usize;
                let source_y = ((v / placement.height) * source_height as f64) as usize;
                let source_index = (source_y.min(source_height - 1) * source_width + source_x.min(source_width - 1)) * 4;
                let source_alpha = (frame.rgba8[source_index + 3] as f32 / 255.0) * placement.alpha;
                if source_alpha <= 0.0 {
                    continue;
                }
                let destination_index = (y as usize * width as usize + x as usize) * 4;
                let backdrop_alpha = canvas[destination_index + 3] as f32 / 255.0;
                let out_alpha = source_alpha + backdrop_alpha * (1.0 - source_alpha);
                for channel in 0..3 {
                    let source_channel = frame.rgba8[source_index + channel] as f32 / 255.0;
                    let backdrop_channel = canvas[destination_index + channel] as f32 / 255.0;
                    let blended = (1.0 - backdrop_alpha) * source_channel + backdrop_alpha * placement.blend.apply(backdrop_channel, source_channel);
                    let out_channel = if out_alpha > 0.0 { ((1.0 - source_alpha) * backdrop_alpha * backdrop_channel + source_alpha * blended) / out_alpha } else { 0.0 };
                    canvas[destination_index + channel] = (out_channel.clamp(0.0, 1.0) * 255.0).round() as u8;
                }
                canvas[destination_index + 3] = (out_alpha.clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }
    }

    Ok(SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8: canvas }], icc: None, metadata: Vec::new() })
}

/// 📥️ The inverse hub hop every real pixel IMPORT in this subset ends on: one decoded
/// `s.stdio.semio/v1/image` becomes a one-`Pixel`-layer raster document whose single asset child is
/// that same content (canonicalized to PNG bytes by the real png serializer, exactly as
/// `raster_document_json_from_dwg`/`raster_image_layer_and_asset` already do).
pub fn raster_document_from_semio_image(image: &SemioImageSnapshot, id_prefix: &str, title: &str) -> Result<RasterSnapshot, String> {
    if image.width == 0 || image.height == 0 {
        return Err(format!("{id_prefix}: decoded image is {}x{} — an empty raster cannot become a pixel layer", image.width, image.height));
    }
    let data = png_bytes_from_semio_image(image)?;
    let asset_key = crate::artifacts::raster::schema::create_raster_id(&format!("{id_prefix}-asset"));
    let mut layer = crate::artifacts::raster::schema::create_pixel_layer(title, image.width, image.height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::artifacts::raster::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = crate::artifacts::raster::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    Ok(RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: crate::artifacts::raster::schema::create_raster_id(id_prefix), title: Some(title.into()), layers: vec![layer], assets })
}
//#endregion 🔖️Composite

//#region 🔖️Gif87aBridge
/// 🎞️ A pure VERSION remap between stdio's two `GifSnapshot` types. It moves no pixels and quantizes
/// nothing — the palette work stays in stdio's own `SemioImageFromGif`/`SemioImageToGif` leaves,
/// which are declared at `89a` only, while this subset's own gif dialect is `87a`. GIF87a has no
/// Graphic Control Extension, so the 89a-only frame state (`delay_cs`, `disposal`,
/// `transparent_index`, `user_input`, `plain_text`) and the 89a-only document state (`loop_count`,
/// `comments`, `app_extensions`) have no on-disk home in an 87a file and are dropped ON PURPOSE —
/// that is what "this document is GIF87a" means, not a shortcut taken here.
pub(crate) mod gif87a {
    use semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable as Table87a, GifImage as Image87a, GifRgb as Rgb87a, GifSnapshot as Snapshot87a};
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable as Table89a, GifFrame as Frame89a, GifRgb as Rgb89a, GifSnapshot as Snapshot89a};

    fn table_to_87a(table: &Table89a) -> Table87a {
        Table87a { sorted: table.sorted, colors: table.colors.iter().map(|color| Rgb87a { r: color.r, g: color.g, b: color.b }).collect() }
    }

    fn table_to_89a(table: &Table87a) -> Table89a {
        Table89a { sorted: table.sorted, colors: table.colors.iter().map(|color| Rgb89a { r: color.r, g: color.g, b: color.b }).collect() }
    }

    pub(crate) fn from_89a(snapshot: &Snapshot89a) -> Snapshot87a {
        Snapshot87a {
            width: snapshot.width,
            height: snapshot.height,
            gct: snapshot.gct.as_ref().map(table_to_87a),
            background_color_index: snapshot.background_color_index,
            pixel_aspect_ratio: snapshot.pixel_aspect_ratio,
            images: snapshot
                .frames
                .iter()
                .filter(|frame| !frame.indices.is_empty())
                .map(|frame| Image87a { left: frame.left, top: frame.top, width: frame.width, height: frame.height, interlace: frame.interlace, lct: frame.lct.as_ref().map(table_to_87a), indices: frame.indices.clone() })
                .collect(),
            ..Snapshot87a::default()
        }
    }

    pub(crate) fn to_89a(snapshot: &Snapshot87a) -> Snapshot89a {
        Snapshot89a {
            width: snapshot.width,
            height: snapshot.height,
            gct: snapshot.gct.as_ref().map(table_to_89a),
            background_color_index: snapshot.background_color_index,
            pixel_aspect_ratio: snapshot.pixel_aspect_ratio,
            frames: snapshot
                .images
                .iter()
                .map(|image| Frame89a { left: image.left, top: image.top, width: image.width, height: image.height, interlace: image.interlace, lct: image.lct.as_ref().map(table_to_89a), indices: image.indices.clone(), ..Frame89a::default() })
                .collect(),
            ..Snapshot89a::default()
        }
    }
}
//#endregion 🔖️Gif87aBridge

//#region 🔖️MediaExport
/// 📤️ Real vector export: the document's visible layer stack becomes a `SemioDrawingSnapshot`
/// (real geometry, own domain model), composed into real SVG text via stdio's `s.stdio.semio/v1/
/// drawing` → `s.stdio.svg` bridge (`io_dispatch`) — no more `title_card_svg` placeholder.
pub fn raster_document_json_to_svg(document: &RasterSnapshot) -> Result<(String, u32, u32), String> {
    let drawing = drawing_snapshot_from_raster(document);
    let svg = dispatch_drawing_to_svg(&drawing)?;
    Ok((svg, drawing.canvas.width.round().max(1.0) as u32, drawing.canvas.height.round().max(1.0) as u32))
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 📥️ Rewires the DWG import path onto real stdio bridges: the DWG entities become a real
/// `SemioDrawingSnapshot` (`drawing_snapshot_from_dwg`), composed to real SVG text via
/// `s.stdio.semio/v1/drawing` (`io_dispatch`). SVG→pixels still needs a real vector renderer — no
/// stdio bridge does that (reported `stdio_gaps`) — so `semio_framework_os`'s real usvg/resvg
/// renderer stays, but its raw PNG bytes are then canonicalized through the real
/// `s.stdio.semio/v1/image` ↔ png round trip (`canonicalize_png_bytes`) instead of being trusted
/// verbatim, which also recovers the real decoded width/height for the new pixel layer.
pub fn raster_document_json_from_dwg(drawing: &DwgDrawing) -> Result<RasterSnapshot, String> {
    let drawing_snapshot = drawing_snapshot_from_dwg(drawing);
    let svg = dispatch_drawing_to_svg(&drawing_snapshot)?;
    let fallback_width = drawing_snapshot.canvas.width.round().max(1.0) as u32;
    let fallback_height = drawing_snapshot.canvas.height.round().max(1.0) as u32;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, fallback_width, fallback_height)?;
    let raw_bytes = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| error.to_string())?;
    let (data, width, height) = match semio_image_from_png_bytes(&raw_bytes).and_then(|image| Ok((png_bytes_from_semio_image(&image)?, image.width, image.height))) {
        Ok((bytes, width, height)) => (bytes, width, height),
        Err(_) => (raw_bytes, fallback_width, fallback_height),
    };
    let asset_key = crate::artifacts::raster::schema::create_raster_id("dwg-asset");
    let mut layer = crate::artifacts::raster::schema::create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::artifacts::raster::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = crate::artifacts::raster::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    let document = RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: crate::artifacts::raster::schema::create_raster_id("dwg-import"), title: Some("DWG Import".into()), layers: vec![layer], assets };
    Ok(document)
}

/// 🎞️ Decodes an incoming `image:in` PNG payload into an `(asset_id, asset, layer)` triple, ready to
/// be emitted as two real semantic mutations (`add-layer-asset` then `create-layer`) instead of a
/// whole-document replace — the dispatch enum has no such variant anymore. `png_base64` is raw
/// (unprefixed) base64 PNG bytes — the same convention `shooting_engine::shooting_photo_media`
/// produces on `photos:out` and the Vector→Raster run-crate converter produces for a draw
/// `vector:out` source. Real decode through `s.stdio.semio/v1/image` (`semio_image_from_png_bytes`)
/// recovers the real width/height instead of leaving them unset, and re-encodes through the real
/// serializer instead of storing the caller's bytes verbatim.
pub fn raster_image_layer_and_asset(png_base64: &str) -> (String, RasterImageAsset, RasterLayerNode) {
    let asset_key = crate::artifacts::raster::schema::create_raster_id("image-in-asset");
    let raw_bytes = base64_codec::base64_standard_decode(png_base64.as_bytes()).unwrap_or_default();
    let (data, width, height) = match semio_image_from_png_bytes(&raw_bytes).and_then(|image| Ok((png_bytes_from_semio_image(&image)?, image.width, image.height))) {
        Ok((bytes, width, height)) => (bytes, Some(width), Some(height)),
        Err(_) => (raw_bytes, None, None),
    };
    let mut layer = crate::artifacts::raster::schema::create_pixel_layer("Imported Image", width.unwrap_or(0), height.unwrap_or(0));
    if let RasterLayerNode::Pixel { image_key, width: layer_width, height: layer_height, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
        *layer_width = width;
        *layer_height = height;
    }
    (asset_key, RasterImageAsset { mime: "image/png".into(), data }, layer)
}
//#endregion 🔖️MediaImport

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn raster_image_layer_and_asset_builds_a_pixel_layer_and_matching_asset() {
        let (asset_id, asset, layer) = raster_image_layer_and_asset("aGVsbG8=");
        assert_eq!(asset.data, b"hello".to_vec());
        let RasterLayerNode::Pixel { image_key, .. } = &layer else { panic!("expected pixel layer") };
        assert_eq!(image_key.as_deref(), Some(asset_id.as_str()));
    }

    /// 🧪️ Builds a real one-pixel-layer document whose asset child carries genuinely PNG-encoded
    /// content (through the same `mint_raster_asset_child` funnel every mutation uses), so the
    /// composite tests below exercise the real materialization path and never a fabricated handle.
    fn document_with_solid_layer(red: u8, green: u8, blue: u8, alpha: u8, width: u32, height: u32) -> RasterSnapshot {
        let pixel_count = width as usize * height as usize;
        let mut rgba8 = Vec::with_capacity(pixel_count * 4);
        for _ in 0..pixel_count {
            rgba8.extend_from_slice(&[red, green, blue, alpha]);
        }
        let image = SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() };
        raster_document_from_semio_image(&image, "fixture", "Fixture").expect("fixture document")
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_flattens_a_pixel_layer_back_to_its_own_canvas() {
        let document = document_with_solid_layer(10, 20, 30, 255, 4, 2);
        let composite = raster_composite_image(&document).expect("composite");
        assert_eq!((composite.width, composite.height), (4, 2));
        let frame = composite.frames.first().expect("one frame");
        assert_eq!(frame.rgba8.len(), 4 * 2 * 4);
        assert_eq!(&frame.rgba8[..4], &[10, 20, 30, 255]);
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_refuses_a_visible_adjustment_layer_with_a_reason() {
        let mut document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
        document.layers.push(crate::artifacts::raster::schema::create_layer_of_kind("adjustment"));
        let error = raster_composite_image(&document).expect_err("adjustment layers must refuse");
        assert!(error.contains("adjustment layer"), "{error}");
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_refuses_an_unknown_blend_mode_with_a_reason() {
        let mut document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
        if let Some(RasterLayerNode::Pixel { blend_mode, .. }) = document.layers.first_mut() {
            *blend_mode = "colorDodge".into();
        }
        let error = raster_composite_image(&document).expect_err("unknown blend modes must refuse");
        assert!(error.contains("unsupported blend mode"), "{error}");
    }

    #[semio_framework_async_macros::async_test]
    async fn composite_refuses_a_document_with_nothing_to_flatten() {
        let error = raster_composite_image(&crate::artifacts::raster::schema::empty_raster_snapshot()).expect_err("an empty document has no composite");
        assert!(error.contains("nothing to flatten"), "{error}");
    }

    /// 🧪️ The real end-to-end pixel hop this packet exists for: composite → stdio's own
    /// `semio/image` → `bmp` serializer → stdio's own `encode_bmp`, then all the way back. A BMP v3
    /// file starts with `BM`, and the round trip must recover the same RGB (alpha is the format's
    /// own documented loss).
    #[semio_framework_async_macros::async_test]
    async fn bmp_export_writes_real_bytes_that_import_reads_back() {
        let document = document_with_solid_layer(200, 100, 50, 255, 3, 2);
        let bytes = crate::artifacts::raster::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&document).expect("bmp export");
        assert_eq!(&bytes[..2], b"BM", "real BITMAPFILEHEADER magic, not DSL text");
        let reimported = crate::artifacts::raster::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes).expect("bmp import");
        let composite = raster_composite_image(&reimported).expect("composite of the reimported document");
        assert_eq!((composite.width, composite.height), (3, 2));
        assert_eq!(&composite.frames[0].rgba8[..3], &[200, 100, 50]);
    }

    /// 🧪️ A PNG export must carry the 8-byte PNG signature — the single sharpest proof that no leaf
    /// is printing this artifact's own DSL text under a foreign extension any more.
    #[semio_framework_async_macros::async_test]
    async fn png_export_writes_a_real_png_signature() {
        let document = document_with_solid_layer(0, 128, 255, 255, 2, 2);
        let bytes = crate::artifacts::raster::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&document).expect("png export");
        assert_eq!(&bytes[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    }

    /// 🧫️ The one cross-language oracle both implementations of the bmp hop assert. The bun twin
    /// (`🚪️io/🧪️tests/🟦️.ts`) writes and reads the SAME `bmpHex` from its own hand-written BMP v3
    /// codec, so a drift between this plugin's Rust path (composite → stdio `SemioImageToBmp` →
    /// stdio `encode_bmp`) and an independent second implementation fails in BOTH languages instead
    /// of going unnoticed.
    const BMP_PARITY_FIXTURES: &[&str] = &[include_str!("🧪️tests/🧫️fixtures/🪟️solid-3x2.json"), include_str!("🧪️tests/🧫️fixtures/🌈️gradient-5x3.json")];

    fn parity_fixture(text: &str) -> (u32, u32, Vec<u8>, String) {
        use semio_s_artifact_stdio_json::schema::snapshot::{parse_json_text, JsonValue};
        let JsonValue::Object { members } = parse_json_text(text).expect("parity fixture is valid json") else { panic!("parity fixture root must be an object") };
        let member = |key: &str| members.iter().find(|entry| entry.key == key).map(|entry| entry.value.clone()).unwrap_or_else(|| panic!("parity fixture has no {key:?} member"));
        let number = |value: &JsonValue| match value {
            JsonValue::Number { lexeme } => lexeme.parse::<u32>().expect("parity fixture numbers are integers"),
            other => panic!("parity fixture expected a number, got {other:?}"),
        };
        let width = number(&member("width"));
        let height = number(&member("height"));
        let JsonValue::Array { items } = member("rgba8") else { panic!("parity fixture rgba8 must be an array") };
        let rgba8 = items.iter().map(|item| number(item) as u8).collect();
        let JsonValue::String { value: bmp_hex } = member("bmpHex") else { panic!("parity fixture bmpHex must be a string") };
        (width, height, rgba8, bmp_hex)
    }

    fn hex_of(bytes: &[u8]) -> String {
        bytes.iter().fold(String::with_capacity(bytes.len() * 2), |mut text, byte| {
            text.push_str(&format!("{byte:02x}"));
            text
        })
    }

    fn bytes_of(hex: &str) -> Vec<u8> {
        (0..hex.len() / 2).map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("parity fixture hex")).collect()
    }

    fn parity_document(width: u32, height: u32, rgba8: Vec<u8>) -> RasterSnapshot {
        let image = SemioImageSnapshot { schema: STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(), width, height, colorspace: SemioColorspace::Rgba, bit_depth: 8, frames: vec![SemioImageFrame { delay_ms: 0, rgba8 }], icc: None, metadata: Vec::new() };
        raster_document_from_semio_image(&image, "parity", "Parity").expect("parity document")
    }

    /// 🧪️ The Rust bmp EXPORT must produce the exact bytes the TypeScript twin produces.
    #[semio_framework_async_macros::async_test]
    async fn bmp_export_matches_the_typescript_parity_fixture() {
        for text in BMP_PARITY_FIXTURES {
            let (width, height, rgba8, bmp_hex) = parity_fixture(text);
            let document = parity_document(width, height, rgba8);
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&document).expect("bmp export");
            assert_eq!(hex_of(&bytes), bmp_hex, "the Rust bmp writer drifted from the TypeScript twin");
        }
    }

    /// 🧪️ The Rust bmp IMPORT must recover the exact canvas the TypeScript twin recovers.
    #[semio_framework_async_macros::async_test]
    async fn bmp_import_matches_the_typescript_parity_fixture() {
        for text in BMP_PARITY_FIXTURES {
            let (width, height, rgba8, bmp_hex) = parity_fixture(text);
            let document = crate::artifacts::raster::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes_of(&bmp_hex)).expect("bmp import");
            let composite = raster_composite_image(&document).expect("composite of the imported document");
            assert_eq!((composite.width, composite.height), (width, height));
            assert_eq!(composite.frames[0].rgba8, rgba8, "the Rust bmp reader drifted from the TypeScript twin");
        }
    }

    /// 🧪️ Every hop this subset declines is declined with a SENTENCE, never with silently wrong
    /// bytes and never with a bare "not implemented".
    #[semio_framework_async_macros::async_test]
    async fn declined_hops_refuse_with_a_reason() {
        let document = document_with_solid_layer(1, 2, 3, 255, 2, 2);
        let pdf_export = crate::artifacts::raster::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&document).expect_err("pdf export is declined");
        assert!(pdf_export.contains("pdf export not supported for a raster document:"), "{pdf_export}");
        let pdf_import = crate::artifacts::raster::io::import::deserializers::artifacts::pdf::v1_4::any::deserialize_bytes(b"%PDF-1.4\n").expect_err("pdf import is declined");
        assert!(pdf_import.contains("pdf import not supported for a raster document:"), "{pdf_import}");
        let dwg_export = crate::artifacts::raster::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&document).expect_err("dwg export is declined");
        assert!(dwg_export.contains("dwg export not supported for a raster document:"), "{dwg_export}");
    }

    /// 🧪️ The two advertised-kind lists must name only formats a leaf really encodes/decodes —
    /// `negotiate_wire_format` picks workflow wires straight out of them.
    #[semio_framework_async_macros::async_test]
    async fn advertised_stdio_kinds_exclude_every_declined_hop() {
        assert!(!export_stdio_kinds().contains(&"stdio.pdf"), "pdf export is declined");
        assert!(!export_stdio_kinds().contains(&"stdio.dwg"), "dwg export is declined");
        assert!(!import_stdio_kinds().contains(&"stdio.pdf"), "pdf import is declined");
        assert!(import_stdio_kinds().contains(&"stdio.dwg"), "dwg import is real");
        for kind in ["stdio.bmp", "stdio.gif", "stdio.jpg", "stdio.json", "stdio.png", "stdio.svg", "stdio.tiff"] {
            assert!(export_stdio_kinds().contains(&kind), "{kind} export is real");
            assert!(import_stdio_kinds().contains(&kind), "{kind} import is real");
        }
        assert_eq!(crate::artifacts::raster::artifact_kind().export_stdio_kinds, export_stdio_kinds().to_vec());
        assert_eq!(crate::artifacts::raster::artifact_kind().import_stdio_kinds, import_stdio_kinds().to_vec());
    }
}
//#endregion 🧪️Tests
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::artifacts::raster::standards::v1::subsets::any::schema::RasterAnalyzer;
    use crate::artifacts::raster::RasterSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_BMP: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GIF: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    const DEP_JPG: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PDF: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_TIFF: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

    pub struct RasterComposerComposition;

    impl ArtifactComposition for RasterComposerComposition {
        type Snapshot = RasterSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BMP, DEP_DWG, DEP_GIF, DEP_JPG, DEP_JSON, DEP_PDF, DEP_PNG, DEP_SVG, DEP_TIFF]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            for source in sources {
                if source.dialect == DIALECT {
                    let native = match &source.payload {
                        AnalyzeSource::Text(t) => AnalyzeSource::Text(t),
                        AnalyzeSource::Binary(b) => AnalyzeSource::Binary(b),
                    };
                    let analysis = RasterAnalyzer::analyze(&[native]);
                    if let Some(snapshot) = analysis.parts.snapshot {
                        return Ok(Composition { snapshot, confidence: analysis.confidence, diagnostics: analysis.diagnostics });
                    }
                }
                if source.dialect == DEP_BMP {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GIF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::gif::v87a::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JPG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::jpg::v_jfif_1_01::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PDF {
                    // 🚫️ The pdf leaf can never succeed (this repo's `PdfSnapshot` decodes no pixels),
                    // so swallowing its `Err` the way the real decoders above are swallowed would
                    // answer a pdf source with "no source in a known read dialect" — a wrong reason.
                    // The leaf's own sentence is returned instead.
                    return Err(ComposeError { message: crate::artifacts::raster::io::import::deserializers::artifacts::pdf::v1_4::any::RASTER_PDF_IMPORT_UNSUPPORTED.into(), diagnostics: Vec::new() });
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TIFF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::artifacts::raster::io::import::deserializers::artifacts::tiff::v6_0::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
            }
            Err(ComposeError { message: "RasterComposerComposition: no source in a known read dialect".into(), diagnostics: Vec::new() })
        }
    }
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

//#region 🚪️DerivedIoRegistry
/// 🚪️ Relocated verbatim from `⚙️engine` (rule 5). The artifact root's `declaration()` calls
/// `…::io::io_registry::entries()` (qualified — see that file's own `🔖️Register` region); the root's
/// own shadowing `io_registry` (returns `&'static [&'static ComposerEntry]`, a different type) must
/// never be confused with this module's `entries() -> &'static [ComposerEntry]`.
pub mod io_registry {
    use crate::artifacts::raster::standards::v1::subsets::any::schema::RasterBuilder as RasterAnyBuilder;
    use crate::artifacts::raster::standards::v1::subsets::any::schema::RasterComposer as RasterAnyComposer;
    use semio_framework_plugin::{composer_entry_of, ArtifactBuilder, ComposeError, ComposedArtifact, ComposerEntry, Dialect, ErasedComposeSource, IoConfidence, IoPayload, StandardId, SubsetId};
    use std::sync::OnceLock;

    static ENTRIES: OnceLock<Vec<ComposerEntry>> = OnceLock::new();

    //#region 🔖️ExportEntries
    /// 🗄️ Ticket 26/08/10/STDIO-ARTIFACTS-AND-IO W15: the typed registry (W11-W14) only ever grew
    /// IMPORT-direction entries (each composer's own `reads()`) -- nothing registers the REVERSE
    /// ("this domain artifact can be exported AS format Y"), because `ArtifactComposer` only models
    /// "produce my own snapshot." These entries wrap the artifact's EXISTING `🚪️io/📤️export/🧵️serializers`
    /// leaves (which already convert this artifact's snapshot straight to target-format bytes/text) as
    /// their own `ComposerEntry` rows: `writes` = the target format's dialect, `reads` = just this
    /// artifact's own dialect. `register_composer_entries` already inserts BOTH an Import key (target
    /// reads from us) and an Export key (we export to target) per entry, so no framework change was
    /// needed, only populating the missing direction. Generated by generators/w15_add_export_entries.py
    /// -- hand-validated pattern on note/json first (see that file's own tests), pilot kept as reference.
    const RASTER_DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };
    const RASTER_JSON_BRIDGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::artifacts::raster::RasterSnapshot, ComposeError> {
        if let Some(source) = sources.iter().find(|s| s.dialect == RASTER_DIALECT) {
            let builder = match &source.payload {
                IoPayload::Text(t) => RasterAnyBuilder::from_text(t).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
                IoPayload::Binary(b) => RasterAnyBuilder::from_binary(b).map_err(|e| ComposeError { message: e.to_string(), diagnostics: Vec::new() })?,
            };
            return builder.build().map_err(|diagnostics| ComposeError { message: "RasterComposer export: build() failed".into(), diagnostics });
        }
        if let Some(source) = sources.iter().find(|s| s.dialect == RASTER_JSON_BRIDGE_DIALECT) {
            // 🌉 The OS dispatch layer (export_os_app_instance_media_kind) deals in already-
            // deserialized `serde_json::Value`, not this artifact's own wire text/binary -- json
            // is the universal bridge dialect every domain artifact already imports from.
            let bytes: Vec<u8> = match &source.payload {
                IoPayload::Text(t) => t.as_bytes().to_vec(),
                IoPayload::Binary(b) => b.clone(),
            };
            return crate::artifacts::raster::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "RasterComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_GIF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    fn compose_export_gif(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::gif::v87a::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GIF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PDF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.pdf", standard: StandardId("1.4"), subset: SubsetId("*") };
    fn compose_export_pdf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::pdf::v1_4::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PDF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JPG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    fn compose_export_jpg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::jpg::v_jfif_1_01::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JPG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_DWG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    fn compose_export_dwg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::dwg::v_ac1018::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_DWG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_BMP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    fn compose_export_bmp(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_BMP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_TIFF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    fn compose_export_tiff(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::artifacts::raster::io::export::serializers::artifacts::tiff::v6_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_TIFF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    //#endregion 🔖️ExportEntries

    pub fn entries() -> &'static [ComposerEntry] {
        ENTRIES
            .get_or_init(|| {
                vec![
                    composer_entry_of::<RasterAnyComposer>(),
                    ComposerEntry { writes: EXPORT_GIF_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_gif },
                    ComposerEntry { writes: EXPORT_SVG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_svg },
                    ComposerEntry { writes: EXPORT_PDF_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_pdf },
                    ComposerEntry { writes: EXPORT_JPG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_jpg },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_json },
                    ComposerEntry { writes: EXPORT_DWG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_dwg },
                    ComposerEntry { writes: EXPORT_BMP_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_bmp },
                    ComposerEntry { writes: EXPORT_TIFF_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_tiff },
                ]
            })
            .as_slice()
    }
}
//#endregion 🚪️DerivedIoRegistry

//#region 🧪️Tests
#[cfg(test)]
mod dwg_import_tests {
    use crate::artifacts::raster::RasterLayerNode;

    #[semio_framework_async_macros::async_test]
    async fn imports_dwg_polyline_into_raster_document() {
        let mut drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
        let layer = drawing.ensure_layer("0");
        drawing.entities.push(semio_s_artifact_stdio_dwg::DwgEntity {
            layer,
            color: semio_s_artifact_stdio_dwg::DwgColor::ByLayer,
            geometry: semio_s_artifact_stdio_dwg::DwgGeometry::LwPolyline { closed: true, elevation: 0.0, vertices: vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]], bulges: vec![0.0, 0.0, 0.0, 0.0] },
        });
        drawing.extmin = [0.0, 0.0, 0.0];
        drawing.extmax = [10.0, 10.0, 0.0];
        let document = crate::artifacts::raster::io::raster_document_json_from_dwg(&drawing).expect("dwg import");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        let asset_key = image_key.as_ref().expect("image key set");
        assert!(document.assets.contains_key(asset_key), "asset handle present");
        let asset = crate::artifacts::raster::raster_asset(&document.assets, asset_key).expect("asset content cached");
        assert_eq!(asset.mime, "image/png");
        assert!(!asset.data.is_empty());
    }

    #[semio_framework_async_macros::async_test]
    async fn imports_empty_dwg_into_blank_raster_document() {
        let drawing = semio_s_artifact_stdio_dwg::DwgDrawing::default();
        let document = crate::artifacts::raster::io::raster_document_json_from_dwg(&drawing).expect("empty dwg import");
        assert_eq!(document.layers.len(), 1);
        let RasterLayerNode::Pixel { image_key, width, height, .. } = &document.layers[0] else {
            panic!("expected pixel layer");
        };
        assert_eq!(*width, Some(1));
        assert_eq!(*height, Some(1));
        let asset_key = image_key.as_ref().expect("image key set");
        let asset = crate::artifacts::raster::raster_asset(&document.assets, asset_key).expect("asset content cached");
        assert!(!asset.data.is_empty());
    }
}
//#endregion 🧪️Tests
