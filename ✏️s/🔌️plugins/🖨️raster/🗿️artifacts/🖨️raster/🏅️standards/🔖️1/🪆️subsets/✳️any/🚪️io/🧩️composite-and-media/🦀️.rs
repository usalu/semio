//! 🧩️ Composite rendering, the gif87a bridge and the media export and import ports.

use super::*;

//#region 🔖️Composite

fn stack_transform(value:&RasterTransform)->RasterStackTransform {
    RasterStackTransform {x:value.x,y:value.y,a:value.a,b:value.b,c:value.c,d:value.d}
}

/// 🌉️ Resolves materialized asset children once; geometry and mask coverage belong to the shared stack job.
struct RasterStackAssets<'a> {
    pub assets:&'a semio_s_artifact_raster_raster_model::RasterOwnedMap<semio_s_artifact_raster_raster_model::RasterAssetChild>,
    pub images:BTreeMap<String,Arc<RasterImage>>,
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
        self.images.insert(key.to_owned(),Arc::new(RasterImage {width:image.width,height:image.height,pixels:Vec::with_capacity(count*4)}));Ok(())
    }
    fn mask(&mut self,value:&Option<semio_s_artifact_raster_raster_model::RasterLayerMask>)->Result<Option<RasterStackMask>,String>{
        let Some(mask)=value else{return Ok(None);};
        if mask.enabled{if let Some(key)=&mask.image_key{self.image(key)?;}}
        Ok(Some(RasterStackMask {enabled:mask.enabled,linked:mask.linked,invert:mask.invert,width:mask.width,height:mask.height,image_key:mask.image_key.clone(),transform:stack_transform(&mask.transform)}))
    }
    fn layers(&mut self,layers:&[RasterLayerNode],depth:usize)->Result<Vec<RasterStackLayer>,String>{
        use semio_s_artifact_raster_raster_model::{layer_opacity, layer_blend_mode, layer_transform};
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

/// 🧵️ Copies each unique source image in bounded grants from one captured immutable document.
pub struct RasterStackPreparation {
    pub layers:Vec<RasterStackLayer>,
    pub images:BTreeMap<String,Arc<RasterImage>>,
    keys:Vec<String>,
    pub index:usize,
    cancelled:bool,
}
impl RasterStackPreparation {
    pub fn new(document:&RasterSnapshot)->Result<Self,String> {Self::from_layers(document,&document.layers)}
    pub fn from_layers(document:&RasterSnapshot,source:&[RasterLayerNode])->Result<Self,String> {
        let mut assets=RasterStackAssets {assets:&document.assets,images:BTreeMap::new(),nodes:0};
        let layers=assets.layers(source,0)?;
        let keys=assets.images.keys().cloned().collect();
        Ok(Self {layers,images:assets.images,keys,index:0,cancelled:false})
    }
    pub fn advance(&mut self,document:&RasterSnapshot,maximum:usize)->Result<bool,String> {
        if self.cancelled {return Err("Composite preparation cancelled".into());}
        let mut remaining=maximum.min(32768);
        while remaining>0&&self.index<self.keys.len() {
            let key=&self.keys[self.index];
            let source=document.assets.get(key).and_then(|asset|asset.local_owner::<SemioImageSnapshot>()).ok_or("Composite source image is unavailable")?;
            let image=Arc::get_mut(self.images.get_mut(key).unwrap()).ok_or("Composite preparation was shared before completion")?;
            let frame=source.frames.first().ok_or("Composite source image has no frame")?;
            let total=image.width as usize*image.height as usize;
            if source.width!=image.width||source.height!=image.height||frame.rgba8.len()!=total*4 {return Err("Composite source extent changed".into());}
            let start=image.pixels.len()/4;let end=total.min(start+remaining);
            image.pixels.extend_from_slice(&frame.rgba8[start*4..end*4]);remaining-=end-start;
            if end==total {self.index+=1;}
        }
        Ok(self.index==self.keys.len())
    }
    pub fn into_job(self)->Result<RasterStackJob,String> {
        if self.cancelled||self.index!=self.keys.len() {return Err("Composite preparation is incomplete or cancelled".into());}
        RasterStackJob::new(RasterStackInput {layers:self.layers,images:self.images}).map_err(|error|error.to_string())
    }
    pub fn cancel(&mut self) {self.cancelled=true;self.layers.clear();self.images.clear();self.keys.clear();}
}

/// 🧱️ Prepares exports with the same mask and layer compositor used by interactive surfaces.
pub fn raster_composite_job(document:&RasterSnapshot)->Result<RasterStackJob,String>{
    let mut preparation=RasterStackPreparation::new(document)?;
    while !preparation.advance(document,32768)? {}
    preparation.into_job()
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
    let asset_key = semio_s_artifact_raster_raster_model::create_raster_id(&format!("{id_prefix}-asset"));
    let mut layer = semio_s_artifact_raster_raster_model::create_pixel_layer(title, image.width, image.height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = semio_s_artifact_raster_raster_model::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    Ok(RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: semio_s_artifact_raster_raster_model::create_raster_id(id_prefix), title: Some(title.into()), layers: vec![layer], assets })
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
pub mod gif87a {
    use semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::snapshot::{GifColorTable as Table87a, GifImage as Image87a, GifRgb as Rgb87a, GifSnapshot as Snapshot87a};
    use semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::snapshot::{GifColorTable as Table89a, GifFrame as Frame89a, GifRgb as Rgb89a, GifSnapshot as Snapshot89a};

    fn table_to_87a(table: &Table89a) -> Table87a {
        Table87a { sorted: table.sorted, colors: table.colors.iter().map(|color| Rgb87a { r: color.r, g: color.g, b: color.b }).collect() }
    }

    fn table_to_89a(table: &Table87a) -> Table89a {
        Table89a { sorted: table.sorted, colors: table.colors.iter().map(|color| Rgb89a { r: color.r, g: color.g, b: color.b }).collect() }
    }

    pub fn from_89a(snapshot: &Snapshot89a) -> Snapshot87a {
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

    pub fn to_89a(snapshot: &Snapshot87a) -> Snapshot89a {
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
    let asset_key = semio_s_artifact_raster_raster_model::create_raster_id("dwg-asset");
    let mut layer = semio_s_artifact_raster_raster_model::create_pixel_layer("DWG Import", width, height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    let asset = RasterImageAsset { mime: "image/png".into(), data };
    let handle = crate::mint_raster_asset_child(&asset_key, &asset);
    let mut assets = semio_s_artifact_raster_raster_model::RasterOwnedMap::new();
    assets.insert(asset_key, handle).map_err(|rejected| rejected.reason.to_string())?;
    let document = RasterSnapshot { schema: RASTER_DOCUMENT_SCHEMA.into(), id: semio_s_artifact_raster_raster_model::create_raster_id("dwg-import"), title: Some("DWG Import".into()), layers: vec![layer], assets };
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
pub fn raster_image_layer_and_asset(png_base64: &str) -> Result<(String, SemioImageSnapshot, RasterLayerNode), String> {
    let bytes = base64_codec::base64_standard_decode(png_base64.as_bytes()).map_err(|error| error.to_string())?;
    let image = semio_image_from_png_bytes(&bytes)?;
    let asset_key = semio_s_artifact_raster_raster_model::create_raster_id("image-in-asset");
    let mut layer = semio_s_artifact_raster_raster_model::create_pixel_layer("Imported Image", image.width, image.height);
    if let RasterLayerNode::Pixel { image_key, .. } = &mut layer {
        *image_key = Some(asset_key.clone());
    }
    Ok((asset_key, image, layer))
}
//#endregion 🔖️MediaImport
