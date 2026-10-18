//! 🌉️ The semio image and drawing bridge: dialects, io keys, format conversions and png canonicalization.

use super::*;

//#region 🔖️SemioBridge

const SEMIO_DRAWING_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("drawing") };
const SEMIO_IMAGE_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.semio", standard: StandardId("v1"), subset: SubsetId("image") };
const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
pub const PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId::ANY };
/// 🪟️ The four other pixel formats stdio's own `s.stdio.semio/v1/image` hub already bridges both
/// ways (`🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs:91-105` registers
/// all five `SemioImageFrom*`/`SemioImageTo*` pairs in ONE `register_composer_entries` call). Every
/// pixel hop in this subset's io leaves goes through that hub — this plugin never hand-rolls a
/// PNG/BMP/GIF/JPEG/TIFF byte codec of its own.
pub const BMP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId::ANY };
pub const JPG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId::ANY };
pub const TIFF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId::ANY };
/// 🎞️ The hub bridges GIF at **89a** only (that is the standard the `SemioImageFromGif`/`ToGif`
/// leaves declare), while this subset's own gif leaf is declared at `87a`. The gif leaves therefore
/// hop `semio/image ↔ gif@89a` for the palette work (real 1:1 quantization, never a local
/// re-implementation) and then remap the 89a snapshot onto stdio's own `87a` `GifSnapshot`/
/// `encode_gif`, which is what actually writes the `GIF87a` bytes this dialect promises.
pub const GIF89A_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("89a"), subset: SubsetId::ANY };

/// 📌️ w5b-close fix: registers stdio's `semio` v1 engine (drawing/image/… subset composers) and
/// stdio's `png` engine into the process-global `io` registry exactly once, so `io_dispatch`/
/// `io_compose_via` below resolve regardless of host-boot ordering — a bare `cargo test` process
/// never runs the plugin-host boot path that would normally call this. Mirrors 🗒️note's/📏️layout's/
/// 🌍️gis's own `ensure_..._registered()` helper (w5b-verify-report.md flagged 🖍️draw as missing this
/// exact pattern; raster was missing it too, surfaced by `cargo test` once the crate compiled).
fn ensure_stdio_semio_and_png_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        semio_s_artifact_stdio_semio::register(&[semio_s_artifact_stdio_semio_drawing_import::CONVERSION, semio_s_artifact_stdio_semio_drawing_export::CONVERSION, semio_s_artifact_stdio_semio_image_import::CONVERSION, semio_s_artifact_stdio_semio_image_export::CONVERSION]);
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

/// 🌐️ World-space bounds `[min_x, min_y, max_x, max_y]` of everything `world` paints (group
/// transforms applied, arcs and curves flattened, text anchors included); `None` when it paints nothing.
fn world_drawing_bounds(world: &SemioDrawingSnapshot) -> Option<[f64; 4]> {
    fn walk(node: &DrawNode, matrix: [f64; 6], bounds: &mut Option<[f64; 4]>) {
        let mut include = |p: [f64; 2]| {
            let (x, y) = (matrix[0] * p[0] + matrix[2] * p[1] + matrix[4], matrix[1] * p[0] + matrix[3] * p[1] + matrix[5]);
            let b = bounds.get_or_insert([x, y, x, y]);
            *b = [b[0].min(x), b[1].min(y), b[2].max(x), b[3].max(y)];
        };
        match node {
            DrawNode::Group { transform, children } => {
                let inner = compose_affine(&matrix, &semio_transform_affine(transform));
                children.iter().for_each(|child| walk(child, inner, bounds));
            }
            DrawNode::Path { segments, .. } => flatten_segments(segments, 16.0).iter().flat_map(|(points, _)| points).for_each(|p| include(*p)),
            DrawNode::Text { at, .. } => include([at.x, at.y]),
            DrawNode::Image { at, width, height, .. } => {
                include([at.x, at.y]);
                include([at.x + width, at.y + height]);
            }
        }
    }
    let mut bounds = None;
    world.layers.iter().for_each(|layer| walk(&layer.root, [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], &mut bounds));
    bounds
}

/// 🗺️ `node` baked through `matrix` (group transforms folded in, arcs become cubics under the mirror).
fn page_node(node: &DrawNode, matrix: [f64; 6]) -> DrawNode {
    let apply = |p: &SemioPoint2| SemioPoint2 { x: matrix[0] * p.x + matrix[2] * p.y + matrix[4], y: matrix[1] * p.x + matrix[3] * p.y + matrix[5] };
    match node {
        DrawNode::Group { transform, children } => {
            let inner = compose_affine(&matrix, &semio_transform_affine(transform));
            DrawNode::Group { transform: SemioTransform::identity(), children: children.iter().map(|child| page_node(child, inner)).collect() }
        }
        DrawNode::Path { segments, style } => DrawNode::Path { segments: transformed_segments(segments, &matrix), style: style.clone() },
        DrawNode::Text { value, at, style } => DrawNode::Text { value: value.clone(), at: apply(at), style: style.clone() },
        DrawNode::Image { at, width, height, mime, bytes } => DrawNode::Image { at: apply(&SemioPoint2 { x: at.x, y: at.y + height }), width: *width, height: *height, mime: mime.clone(), bytes: bytes.clone() },
    }
}

/// 📄️ A WORLD drawing (y up, a CAD file's model space) fitted onto a page (y down): the page is the
/// painted bounds (at least 1×1 unit), every vertex moves by `(x − min_x, max_y − y)`. An empty drawing
/// becomes an empty 1×1 page.
pub fn page_drawing_from_world(world: &SemioDrawingSnapshot) -> SemioDrawingSnapshot {
    let [min_x, min_y, max_x, max_y] = world_drawing_bounds(world).unwrap_or([0.0; 4]);
    let matrix = [1.0, 0.0, 0.0, -1.0, -min_x, max_y];
    SemioDrawingSnapshot {
        schema: STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas: DrawCanvas { width: (max_x - min_x).max(1.0), height: (max_y - min_y).max(1.0), background: None },
        styles: world.styles.clone(),
        layers: world.layers.iter().map(|layer| DrawLayer { id: layer.id.clone(), name: layer.name.clone(), visible: layer.visible, root: page_node(&layer.root, matrix) }).collect(),
    }
}

/// 🚪️ Dispatches `s.stdio.semio/v1/drawing` → `s.stdio.svg` through stdio's real SVG serializer
/// (`io_dispatch`), then prints the composed `SvgSnapshot` as bare XML (`write_svg_xml`, NOT
/// `ArtifactDsl::print_dsl` — w5b-close fix: `print_dsl` wraps the text in stdio's `.semio`
/// envelope preamble, which `raster_document_from_dwg_drawing`'s `semio_framework_os::
/// rasterize_svg_to_png_base64` call below then fails to parse as XML at all ("unknown token at
/// 1:1"); every downstream consumer of this function's return value wants a bare `<svg>…</svg>`
/// document, matching 🗒️note's/🖍️draw's own `write_svg_xml` usage for the identical bridge).
pub fn dispatch_drawing_to_svg(snapshot: &SemioDrawingSnapshot) -> Result<String, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioDrawingSnapshot as pack::snapshot::ArtifactPack>::encode_pack(snapshot));
    let key = semio_io_key(&SEMIO_DRAWING_DIALECT, IoDirection::Export, &SVG_DIALECT);
    let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_DRAWING_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(svg_bytes) = composed.payload else { return Err("s.stdio.svg composer returned a non-binary payload".into()) };
    let svg_snapshot = <SvgSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&svg_bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::write_svg_xml(&svg_snapshot.doc)
}

/// 🚪️ Dispatches a decoded foreign pixel snapshot (`PngSnapshot`/`BmpSnapshot`/`GifSnapshot`/
/// `JpgSnapshot`/`TiffSnapshot`) → `s.stdio.semio/v1/image` through stdio's own registered
/// deserializer for that dialect — the honest, structured way to reach an image's real
/// width/height/RGBA8 frames without this plugin ever touching the wire format itself.
pub fn semio_image_from_format<T: pack::snapshot::ArtifactPack>(snapshot: &T, format: Dialect) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<T as pack::snapshot::ArtifactPack>::encode_pack(snapshot));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &format);
    let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: format, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.semio image composer returned a non-binary payload".into()) };
    <SemioImageSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

/// 🚪️ Dispatches `s.stdio.semio/v1/image` → the target format's own typed snapshot through stdio's
/// registered serializer for that dialect. The caller then hands that snapshot to the format's own
/// real byte encoder (`encode_png`/`encode_bmp`/…) — never a hand-rolled writer here.
pub fn semio_image_to_format<T: pack::snapshot::ArtifactPack>(image: &SemioImageSnapshot, format: Dialect) -> Result<T, String> {
    ensure_stdio_semio_and_png_registered();
    let payload = IoPayload::Binary(<SemioImageSnapshot as pack::snapshot::ArtifactPack>::encode_pack(image));
    let key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &format);
    let composed = ::semio_framework_async::poll::resolve_ready(io_dispatch(&key, &[ErasedComposeSource { dialect: SEMIO_IMAGE_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err(format!("{} composer returned a non-binary payload", format.artifact_kind)) };
    <T as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))
}

/// 🚪️ Dispatches real `png` bytes → `s.stdio.semio/v1/image` through stdio's real PNG deserializer
/// (`io_dispatch`) — the honest, structured way to learn a decoded image's real width/height/pixels.
pub fn semio_image_from_png_bytes(raw_png_bytes: &[u8]) -> Result<SemioImageSnapshot, String> {
    ensure_stdio_semio_and_png_registered();
    let png_snapshot: PngSnapshot = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::decode_png(raw_png_bytes)?;
    semio_image_from_format(&png_snapshot, PNG_DIALECT)
}

/// 🚪️ Dispatches `s.stdio.semio/v1/image` → real `png` bytes through stdio's real PNG serializer
/// (`io_dispatch`) plus its own real byte encoder — never a hand-rolled PNG writer.
pub fn png_bytes_from_semio_image(image: &SemioImageSnapshot) -> Result<Vec<u8>, String> {
    let png_snapshot: PngSnapshot = semio_image_to_format(image, PNG_DIALECT)?;
    semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&png_snapshot)
}

/// 🧩️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`: real bidirectional CHILD-CONTENT
/// converters between this plugin's own `RasterImageAsset` (mime+bytes, the mutation-payload/
/// working-scene shape) and the composed `s.stdio.semio/v1/image` child's real content
/// (`SemioImageSnapshot`, decoded RGBA8 pixels) — reuses the SAME real png↔semio/image bridge above,
/// never a stub. Only `image/png` is lossless today (the only mime this plugin ever produces, via
/// `raster_document_from_dwg_drawing`/`raster_image_layer_and_asset` below); any other mime is honestly
/// reported as an error, never silently coerced.
pub fn semio_image_snapshot_from_raster_asset(asset: &RasterImageAsset) -> Result<SemioImageSnapshot, String> {
    match asset.mime.as_str() {
        "image/png" => semio_image_from_png_bytes(&asset.data),
        RASTER_IMAGE_PACK_MIME => <SemioImageSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&asset.data).map_err(|error| format!("semio_image_snapshot_from_raster_asset: the image pack does not decode: {error}")),
        other => Err(format!("semio_image_snapshot_from_raster_asset: unsupported mime {other:?} (image/png and {RASTER_IMAGE_PACK_MIME} round-trip)")),
    }
}

/// 🧊️ The media type of an asset whose bytes are the `s.stdio.semio/v1/image` pack of its decoded image: the lossless
/// carrier a replayed `paint-stroke` mints its painted image through and its inverse restores the prior image through
/// — every pixel, profile and metadata entry survives the round trip and no replay ever encodes a PNG.
pub const RASTER_IMAGE_PACK_MIME: &str = "application/x-semio-image";

/// 🧊️ `image` as an asset in the lossless [`RASTER_IMAGE_PACK_MIME`] carrier.
pub fn raster_image_pack_asset(image: &SemioImageSnapshot) -> RasterImageAsset {
    RasterImageAsset { mime: RASTER_IMAGE_PACK_MIME.into(), data: <SemioImageSnapshot as pack::snapshot::ArtifactPack>::encode_pack(image) }
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
    let png_snapshot = semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::decode_png(raw_png_bytes)?;
    let payload = IoPayload::Binary(<PngSnapshot as pack::snapshot::ArtifactPack>::encode_pack(&png_snapshot));
    let hub_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Import, &PNG_DIALECT);
    let target_key = semio_io_key(&SEMIO_IMAGE_DIALECT, IoDirection::Export, &PNG_DIALECT);
    let composed = ::semio_framework_async::poll::resolve_ready(io_compose_via(&hub_key, &target_key, &[ErasedComposeSource { dialect: PNG_DIALECT, payload }])).map_err(|error| error.message)?;
    let IoPayload::Binary(bytes) = composed.payload else { return Err("s.stdio.png composer returned a non-binary payload".into()) };
    let png_snapshot = <PngSnapshot as pack::snapshot::ArtifactPack>::decode_pack(&bytes).map_err(|error| format!("{error:?}"))?;
    semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::encode_png(&png_snapshot)
}
//#endregion 🔖️SemioBridge
