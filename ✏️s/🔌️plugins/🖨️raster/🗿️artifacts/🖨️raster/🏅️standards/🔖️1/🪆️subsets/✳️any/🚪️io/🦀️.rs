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
use crate::{RasterImageAsset, RasterLayerNode, RasterSnapshot, RasterTransform, RASTER_DOCUMENT_SCHEMA};
use semio_framework::{io::io_compose_via, io_dispatch, resolve_ready, Dialect, ErasedComposeSource, IoDirection, IoKey, IoPayload, StandardId, SubsetId};
use semio_s_artifact_stdio_png::PngSnapshot;
use semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::geometry::{SemioPoint2, SemioTransform};
use semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::io::export::serializers::artifacts::png::v1_2::any::{compose_affine, flatten_segments, semio_transform_affine, transformed_segments};
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
fn page_drawing_from_world(world: &SemioDrawingSnapshot) -> SemioDrawingSnapshot {
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
/// `raster_document_from_dwg_drawing`/`raster_image_layer_and_asset` below); any other mime is honestly
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
use semio_framework_pixels::compositing::layers::{RasterStackContent,RasterStackInput,RasterStackJob,RasterStackLayer,RasterStackMask,RasterStackTransform};
use semio_framework_pixels::{editing::validate_extent,RasterImage};
use std::{collections::BTreeMap,sync::Arc};

fn stack_transform(value:&RasterTransform)->RasterStackTransform {
    RasterStackTransform {x:value.x,y:value.y,scale_x:value.scale_x,scale_y:value.scale_y,rotation:value.rotation}
}

/// 🌉️ Resolves materialized asset children once; geometry and mask coverage belong to the shared stack job.
struct RasterStackAssets<'a> {
    assets:&'a crate::RasterOwnedMap<crate::RasterAssetChild>,
    images:BTreeMap<String,Arc<RasterImage>>,
    nodes:usize,
}
impl RasterStackAssets<'_> {
    fn image(&mut self,key:&str)->Result<(),String>{
        if self.images.contains_key(key){return Ok(());}
        let handle=self.assets.get(key).ok_or_else(||format!("layer references missing asset {key:?}"))?;
        let image=handle.local_owner::<SemioImageSnapshot>().ok_or_else(||format!("asset {key:?} must be materialized before rendering"))?;
        let count=validate_extent(image.width,image.height).map_err(|error|error.to_string())?;
        let frame=image.frames.first().ok_or("a layer's image asset carries no decoded frame")?;
        if frame.rgba8.len()!=count*4{return Err("a layer's image asset frame length does not match width*height*4".into());}
        self.images.insert(key.to_owned(),Arc::new(RasterImage {width:image.width,height:image.height,pixels:frame.rgba8.clone()}));Ok(())
    }
    fn mask(&mut self,value:&Option<crate::RasterLayerMask>)->Result<Option<RasterStackMask>,String>{
        let Some(mask)=value else{return Ok(None);};
        if mask.enabled{if let Some(key)=&mask.image_key{self.image(key)?;}}
        Ok(Some(RasterStackMask {enabled:mask.enabled,linked:mask.linked,invert:mask.invert,width:mask.width,height:mask.height,image_key:mask.image_key.clone(),transform:stack_transform(&mask.transform)}))
    }
    fn layers(&mut self,layers:&[RasterLayerNode],depth:usize)->Result<Vec<RasterStackLayer>,String>{
        use crate::standards::v1::subsets::any::schema::{layer_node_id,layer_visible,layer_opacity,layer_blend_mode,layer_transform};
        if depth>32{return Err("layer nesting exceeds compositor budget".into());}
        let mut output=Vec::with_capacity(layers.len());
        for layer in layers{
            self.nodes+=1;if self.nodes>1024{return Err("layer count exceeds compositor budget".into());}
            let (content,mask)=match layer{
                RasterLayerNode::Pixel {image_key,width,height,mask,..}=>{
                    if let Some(key)=image_key{self.image(key)?;}
                    (RasterStackContent::Pixel {width:*width,height:*height,image_key:image_key.clone()},self.mask(mask)?)
                }
                RasterLayerNode::Group {children,mask,..}=>(RasterStackContent::Group(self.layers(children,depth+1)?),self.mask(mask)?),
                RasterLayerNode::Adjustment {adjustment_kind,params,..}=>{
                    if adjustment_kind!="brightnessContrast"{return Err(format!("unsupported adjustment layer kind {adjustment_kind:?}"));}
                    let parameter=|key:&str|->Result<f64,String>{match params.get(key){None=>Ok(0.0),Some(value)=>value.as_f64().ok_or_else(||format!("adjustment parameter {key:?} must be numeric"))}};
                    (RasterStackContent::BrightnessContrast {brightness:parameter("brightness")?,contrast:parameter("contrast")?},None)
                }
            };
            output.push(RasterStackLayer {id:layer_node_id(layer).to_owned(),visible:layer_visible(layer),opacity:f64::from(layer_opacity(layer)),blend_mode:layer_blend_mode(layer).parse().map_err(|_|format!("unsupported blend mode {:?}",layer_blend_mode(layer)))?,transform:stack_transform(layer_transform(layer)),mask,content});
        }
        Ok(output)
    }
}

/// 🧱️ Prepares exports with the same bounded mask and layer compositor used by interactive surfaces.
pub fn raster_composite_job(document:&RasterSnapshot)->Result<RasterStackJob,String>{
    let mut assets=RasterStackAssets {assets:&document.assets,images:BTreeMap::new(),nodes:0};
    let layers=assets.layers(&document.layers,0)?;
    RasterStackJob::new(RasterStackInput {layers,images:assets.images}).map_err(|error|error.to_string())
}

/// 🖼️ Flattens visible layers through the shared compositor into the image export hub.
pub fn raster_composite_image(document:&RasterSnapshot)->Result<SemioImageSnapshot,String> {
    let mut job=raster_composite_job(document)?;
    while !job.advance(65536).map_err(|e|e.to_string())?.done {}
    let output=job.into_result().map_err(|e|e.to_string())?;
    if output.empty{return Err("no visible pixel layer: there is nothing to flatten into a raster composite".into());}
    let image=output.image;
    Ok(SemioImageSnapshot {schema:STDIO_SEMIOIMAGE_DOCUMENT_SCHEMA.into(),width:image.width,height:image.height,colorspace:SemioColorspace::Rgba,bit_depth:8,frames:vec![SemioImageFrame {delay_ms:0,rgba8:image.pixels}],icc:None,metadata:Vec::new()})
}

/// 📥️ The inverse hub hop every real pixel IMPORT in this subset ends on: one decoded
/// `s.stdio.semio/v1/image` becomes a one-`Pixel`-layer raster document whose single asset child is
/// that same content (canonicalized to PNG bytes by the real png serializer, exactly as
/// `raster_document_from_dwg_drawing`/`raster_image_layer_and_asset` already do).
pub fn raster_document_from_semio_image(image: &SemioImageSnapshot, id_prefix: &str, title: &str) -> Result<RasterSnapshot, String> {
    if image.width == 0 || image.height == 0 {
        return Err(format!("{id_prefix}: decoded image is {}x{} — an empty raster cannot become a pixel layer", image.width, image.height));
    }
    let data = png_bytes_from_semio_image(image)?;
    let asset_key = crate::standards::v1::subsets::any::schema::create_raster_id(&format!("{id_prefix}-asset"));
    let mut layer = crate::standards::v1::subsets::any::schema::create_pixel_layer(title, image.width, image.height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = crate::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    Ok(RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: crate::standards::v1::subsets::any::schema::create_raster_id(id_prefix), title: Some(title.into()), layers: vec![layer], assets })
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
/// 📤️ Embeds the canonical raster composite in SVG using the shared drawing serializer.
pub fn raster_document_json_to_svg(document: &RasterSnapshot) -> Result<(String, u32, u32), String> {
    let image=raster_composite_image(document)?;
    let drawing=SemioDrawingSnapshot {
        schema:STDIO_SEMIODRAWING_DOCUMENT_SCHEMA.into(),
        canvas:DrawCanvas {width:f64::from(image.width),height:f64::from(image.height),background:None},
        styles:Vec::new(),
        layers:vec![DrawLayer {id:document.id.clone(),name:document.title.clone().unwrap_or_default(),visible:true,root:DrawNode::Image {at:SemioPoint2 {x:0.0,y:0.0},width:f64::from(image.width),height:f64::from(image.height),mime:"image/png".into(),bytes:png_bytes_from_semio_image(&image)?}}],
    };
    Ok((dispatch_drawing_to_svg(&drawing)?,image.width,image.height))
}
//#endregion 🔖️MediaExport

//#region 🔖️MediaImport
/// 📥️ A WORLD drawing (what `s.stdio.semio/v1/drawing`'s own dwg import leaf reads out of a DWG
/// file) becomes a one-`Pixel`-layer raster document: fitted onto a page (`page_drawing_from_world`),
/// composed to SVG through stdio's `s.stdio.semio/v1/drawing` → `s.stdio.svg` bridge (`io_dispatch`),
/// rendered by `semio_framework_os`'s usvg/resvg renderer (no stdio bridge rasterizes vectors — a
/// reported `stdio_gaps` entry), and canonicalized through the `s.stdio.semio/v1/image` ↔ png codec,
/// which also yields the real decoded width/height.
pub fn raster_document_from_dwg_drawing(world: &SemioDrawingSnapshot) -> Result<RasterSnapshot, String> {
    let page = page_drawing_from_world(world);
    let svg = dispatch_drawing_to_svg(&page)?;
    let rendered = semio_framework_os::rasterize_svg_to_png_base64(&svg, page.canvas.width.round().max(1.0) as u32, page.canvas.height.round().max(1.0) as u32)?;
    let raw_bytes = base64_codec::base64_standard_decode(rendered.as_bytes()).map_err(|error| error.to_string())?;
    let image = semio_image_from_png_bytes(&raw_bytes)?;
    let (data, width, height) = (png_bytes_from_semio_image(&image)?, image.width, image.height);
    let asset_key = crate::standards::v1::subsets::any::schema::create_raster_id("dwg-asset");
    let mut layer = crate::standards::v1::subsets::any::schema::create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = crate::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    let document = RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: crate::standards::v1::subsets::any::schema::create_raster_id("dwg-import"), title: Some("DWG Import".into()), layers: vec![layer], assets };
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
    let asset_key = crate::standards::v1::subsets::any::schema::create_raster_id("image-in-asset");
    let raw_bytes = base64_codec::base64_standard_decode(png_base64.as_bytes()).unwrap_or_default();
    let (data, width, height) = match semio_image_from_png_bytes(&raw_bytes).and_then(|image| Ok((png_bytes_from_semio_image(&image)?, image.width, image.height))) {
        Ok((bytes, width, height)) => (bytes, Some(width), Some(height)),
        Err(_) => (raw_bytes, None, None),
    };
    let mut layer = crate::standards::v1::subsets::any::schema::create_pixel_layer("Imported Image", width.unwrap_or(0), height.unwrap_or(0));
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
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v1::subsets::any::schema::RasterAnalyzer;
    use crate::RasterSnapshot;
    use semio_framework_plugin::{AnalyzeSource, ArtifactComposition, ComposeError, ComposeSource, Composition, Dialect, StandardId, SubsetId};

    const DIALECT: Dialect = Dialect { artifact_kind: "s.raster.raster", standard: StandardId("1"), subset: SubsetId("*") };
    const DEP_BMP: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    const DEP_DWG: Dialect = Dialect { artifact_kind: "s.stdio.dwg", standard: StandardId("ac1018"), subset: SubsetId("*") };
    const DEP_GIF: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    const DEP_JPG: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    const DEP_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_PNG: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    const DEP_SVG: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    const DEP_TIFF: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };

    pub struct RasterComposerComposition;

    impl ArtifactComposition for RasterComposerComposition {
        type Snapshot = RasterSnapshot;
        const WRITES: Dialect = DIALECT;

        fn reads() -> &'static [Dialect] {
            &[DIALECT, DEP_BMP, DEP_DWG, DEP_GIF, DEP_JPG, DEP_JSON, DEP_PNG, DEP_SVG, DEP_TIFF]
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
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::bmp::v_v3::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_DWG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::dwg::v_ac1018::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_GIF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::gif::v87a::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JPG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::jpg::v_jfif_1_01::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_JSON {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_PNG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::png::v1_2::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_SVG {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::svg::v1_1::any::deserialize_bytes(&bytes) {
                        return Ok(Composition { snapshot, confidence: semio_framework_plugin::IoConfidence::Medium, diagnostics: Vec::new() });
                    }
                }
                if source.dialect == DEP_TIFF {
                    let bytes: Vec<u8> = match &source.payload {
                        AnalyzeSource::Text(t) => t.as_bytes().to_vec(),
                        AnalyzeSource::Binary(b) => b.to_vec(),
                    };
                    if let Ok(snapshot) = crate::io::import::deserializers::artifacts::tiff::v6_0::any::deserialize_bytes(&bytes) {
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
    use crate::standards::v1::subsets::any::schema::RasterBuilder as RasterAnyBuilder;
    use crate::standards::v1::subsets::any::schema::RasterComposer as RasterAnyComposer;
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

    fn rebuild_native_snapshot(sources: &[ErasedComposeSource]) -> Result<crate::RasterSnapshot, ComposeError> {
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
            return crate::io::import::deserializers::artifacts::json::v_rfc8259::any::deserialize_bytes(&bytes).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() });
        }
        Err(ComposeError { message: "RasterComposer export: no native or json-bridge source provided".into(), diagnostics: Vec::new() })
    }

    const EXPORT_GIF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.gif", standard: StandardId("87a"), subset: SubsetId("*") };
    fn compose_export_gif(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::gif::v87a::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_GIF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId("*") };
    fn compose_export_svg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::svg::v1_1::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_SVG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JPG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.jpg", standard: StandardId("jfif-1.01"), subset: SubsetId("*") };
    fn compose_export_jpg(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::jpg::v_jfif_1_01::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JPG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_PNG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.png", standard: StandardId("1.2"), subset: SubsetId("*") };
    fn compose_export_png(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::png::v1_2::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_PNG_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_JSON_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    fn compose_export_json(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::json::v_rfc8259::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_JSON_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_BMP_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.bmp", standard: StandardId("v3"), subset: SubsetId("*") };
    fn compose_export_bmp(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::bmp::v_v3::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
            Ok(ComposedArtifact { dialect: EXPORT_BMP_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidence::Medium })
        })
    }
    const EXPORT_TIFF_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.tiff", standard: StandardId("6.0"), subset: SubsetId("*") };
    fn compose_export_tiff(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
        Box::pin(async move {
            let snapshot = rebuild_native_snapshot(sources)?;
            let bytes = crate::io::export::serializers::artifacts::tiff::v6_0::any::serialize_bytes(&snapshot).map_err(|e| ComposeError { message: e, diagnostics: Vec::new() })?;
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
                    ComposerEntry { writes: EXPORT_JPG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_jpg },
                    ComposerEntry { writes: EXPORT_PNG_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_png },
                    ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[RASTER_DIALECT], compose: compose_export_json },
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
#[path = "🧪️tests/🔬️dwg-import/🦀️.rs"]
mod dwg_import_tests;
//#endregion 🧪️Tests
