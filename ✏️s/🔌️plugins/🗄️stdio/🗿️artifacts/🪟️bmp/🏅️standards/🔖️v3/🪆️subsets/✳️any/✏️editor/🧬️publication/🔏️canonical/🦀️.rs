//! 🔏️ Borrowed exact typed BMP mutation traversal for Store canonical sealing.
use crate::schema::{snapshot::*,mutations::BmpMutation};
use store::{ArtifactCanonicalJsonValue as V,ArtifactCanonicalJsonNode as N,ArtifactCanonicalJsonArray as A,ArtifactCanonicalJsonObject as O};
fn text(value:&str)->V<'_>{V::Scalar(N::String(value))}
fn number<'a>(value:impl Into<u64>)->V<'a>{V::Scalar(N::U64(value.into()))}
fn signed<'a>(value:i32)->V<'a>{V::Scalar(N::I64(value.into()))}
fn object<'a,const K:usize>(mut fields:[(&'a str,V<'a>);K])->V<'a>{fields.sort_unstable_by(|a,b|a.0.cmp(b.0));V::Object(O::new(fields.into_iter()))}
fn array<'a,T:Sync+'a>(value:&'a [T],map:impl Fn(&'a T)->V<'a>+Send+'a)->V<'a>{V::Array(A::new(value.iter().map(map)))}
fn snapshot(value:&BmpSnapshot)->V<'_>{object([("schema",text(&value.schema)),("image",image(&value.image))])}
fn image(value:&BmpImage)->V<'_>{object([
("width",number(value.width)),("height",number(value.height)),("rowOrder",text(match value.row_order {BmpRowOrder::BottomUp=>"bottomUp",BmpRowOrder::TopDown=>"topDown"})),("profile",text(value.profile.id())),("masks",array(&value.masks,|v|number(*v))),
("palette",array(&value.palette,|v|object([("r",number(v.r)),("g",number(v.g)),("b",number(v.b)),("reserved",number(v.reserved))]))),
("pixels",match &value.pixels {BmpPixels::Indexed{indices}=>object([("storage",text("indexed")),("indices",array(indices,|v|number(*v)))]),BmpPixels::Direct{samples}=>object([("storage",text("direct")),("samples",array(samples,|v|object([("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha)),("reserved",number(v.reserved))])))] )}),
("xPixelsPerMeter",signed(value.x_pixels_per_meter)),("yPixelsPerMeter",signed(value.y_pixels_per_meter)),("colorsUsed",number(value.colors_used)),("colorsImportant",number(value.colors_important)),("reserved1",number(value.reserved_1)),("reserved2",number(value.reserved_2)),("opaqueGap",array(&value.opaque_gap,|v|number(*v))),("opaqueTrailer",array(&value.opaque_trailer,|v|number(*v)))
])}
impl store::ArtifactCanonicalJson for BmpMutation {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>,String>{
        let (kind,payload)=match self {
            Self::SetSnapshot(v)=>("set-snapshot",object([("snapshot",snapshot(&v.snapshot))])),
            Self::PaintIndexedRegion(v)=>("paint-indexed-region",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("paletteIndex",number(v.palette_index))])),
            Self::PaintDirectRegion(v)=>("paint-direct-region",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha))])),
            Self::PatchSnapshot(_)=>return Err("bmp: path patch has no controlled canonical publication source".into()),
        };Ok(Some(object([("mutation",text(kind)),("payload",payload)])))
    }
}
