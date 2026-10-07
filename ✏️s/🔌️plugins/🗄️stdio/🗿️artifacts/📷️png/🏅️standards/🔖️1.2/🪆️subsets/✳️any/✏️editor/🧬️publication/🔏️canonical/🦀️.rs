//! 🔏️ Borrowed exact typed PNG mutation traversal for Store canonical sealing.
use crate::schema::{snapshot::*,mutations::PngMutation};
use store::{ArtifactCanonicalJsonValue as V,ArtifactCanonicalJsonNode as N,ArtifactCanonicalJsonArray as A,ArtifactCanonicalJsonObject as O};
fn text(value:&str)->V<'_>{V::Scalar(N::String(value))}
fn number<'a>(value:impl Into<u64>)->V<'a>{V::Scalar(N::U64(value.into()))}
fn boolean<'a>(value:bool)->V<'a>{V::Scalar(N::Bool(value))}
fn object<'a,const K:usize>(mut fields:[(&'a str,V<'a>);K])->V<'a>{fields.sort_unstable_by(|a,b|a.0.cmp(b.0));V::Object(O::new(fields.into_iter()))}
fn optional<'a,T>(value:Option<&'a T>,map:impl FnOnce(&'a T)->V<'a>)->V<'a>{value.map_or(V::Scalar(N::Null),map)}
fn array<'a,T:Sync+'a>(value:&'a [T],map:impl Fn(&'a T)->V<'a>+Send+'a)->V<'a>{V::Array(A::new(value.iter().map(map)))}
fn region(value:&PngRegion)->V<'_>{object([("x",number(value.x)),("y",number(value.y)),("width",number(value.width)),("height",number(value.height))])}
fn paint(value:&PngNativePaint)->V<'_>{object([("profile",text(match value.profile {PngNativeProfile::Indexed=>"indexed",PngNativeProfile::Grayscale=>"grayscale",PngNativeProfile::GrayscaleAlpha=>"grayscale-alpha",PngNativeProfile::Rgb=>"rgb",PngNativeProfile::Rgba=>"rgba"})),("first",number(value.first)),("second",number(value.second)),("third",number(value.third)),("fourth",number(value.fourth))])}
fn snapshot(value:&PngSnapshot)->V<'_>{object([("schema",text(&value.schema)),("image",image(&value.image))])}
fn image(value:&PngImage)->V<'_>{object([
("width",number(value.width)),("height",number(value.height)),("bitDepth",number(value.bit_depth)),("colorType",text(match value.color_type {PngColorType::Grayscale=>"grayscale",PngColorType::Rgb=>"rgb",PngColorType::Palette=>"palette",PngColorType::GrayscaleAlpha=>"grayscaleAlpha",PngColorType::Rgba=>"rgba"})),("interlace",boolean(value.interlace)),
("samples",array(&value.samples,|v|number(*v))),
("palette",optional(value.palette.as_ref(),|values|array(values,|v|object([("r",number(v.r)),("g",number(v.g)),("b",number(v.b))])))),
("transparency",optional(value.transparency.as_ref(),|v|match v {PngTransparency::Indexed{alpha}=>object([("colorType",text("indexed")),("alpha",array(alpha,|v|number(*v)))]),PngTransparency::Grayscale{gray}=>object([("colorType",text("grayscale")),("gray",number(*gray))]),PngTransparency::Rgb{r,g,b}=>object([("colorType",text("rgb")),("r",number(*r)),("g",number(*g)),("b",number(*b))])})),
("gamma",optional(value.gamma.as_ref(),|v|number(*v))),
("chromaticities",optional(value.chromaticities.as_ref(),|v|object([("whiteX",number(v.white_x)),("whiteY",number(v.white_y)),("redX",number(v.red_x)),("redY",number(v.red_y)),("greenX",number(v.green_x)),("greenY",number(v.green_y)),("blueX",number(v.blue_x)),("blueY",number(v.blue_y))]))),
("srgb",optional(value.srgb.as_ref(),|v|text(match v {PngSrgbIntent::Perceptual=>"perceptual",PngSrgbIntent::RelativeColorimetric=>"relativeColorimetric",PngSrgbIntent::Saturation=>"saturation",PngSrgbIntent::AbsoluteColorimetric=>"absoluteColorimetric"}))),
("physicalDims",optional(value.physical_dims.as_ref(),|v|object([("ppuX",number(v.ppu_x)),("ppuY",number(v.ppu_y)),("unitIsMeter",boolean(v.unit_is_meter))]))),
("timestamp",optional(value.timestamp.as_ref(),|v|object([("year",number(v.year)),("month",number(v.month)),("day",number(v.day)),("hour",number(v.hour)),("minute",number(v.minute)),("second",number(v.second))]))),
("background",optional(value.background.as_ref(),|v|match v {PngBackground::Indexed{index}=>object([("colorType",text("indexed")),("index",number(*index))]),PngBackground::Grayscale{gray}=>object([("colorType",text("grayscale")),("gray",number(*gray))]),PngBackground::Rgb{r,g,b}=>object([("colorType",text("rgb")),("r",number(*r)),("g",number(*g)),("b",number(*b))])})),
("textChunks",array(&value.text_chunks,|v|object([("keyword",text(&v.keyword)),("value",text(&v.value)),("compressed",boolean(v.compressed)),("kind",text(match v.kind {PngTextKind::Text=>"text",PngTextKind::ZText=>"zText",PngTextKind::IText=>"iText"})),("languageTag",text(&v.language_tag)),("translatedKeyword",text(&v.translated_keyword))]))),
("ancillaryChunks",array(&value.ancillary_chunks,|v|object([("kind",array(&v.kind,|v|number(*v))),("data",array(&v.data,|v|number(*v))),("afterRaster",boolean(v.after_raster))])))
])}
impl store::ArtifactCanonicalJson for PngMutation {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>,String>{
        let (kind,payload)=match self {
            Self::SetSnapshot(v)=>("set-snapshot",object([("snapshot",snapshot(&v.snapshot))])),
            Self::ChangeGamma(v)=>("change-gamma",object([("revision",text(&v.revision)),("gama",optional(v.gama.as_ref(),|v|number(*v)))])),
            Self::PatchPixels(v)=>("patch-pixels",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha))])),
            Self::PaintNativeSamples(v)=>("paint-native-samples",object([("revision",text(&v.revision)),("region",region(&v.region)),("paint",paint(&v.paint)),("result",snapshot(&v.result))])),
            Self::PatchSnapshot(_)=>return Err("png: path patch has no controlled canonical publication source".into()),
        };Ok(Some(object([("mutation",text(kind)),("payload",payload)])))
    }
}
