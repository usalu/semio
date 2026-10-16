//! 🔏️ Borrowed exact typed BMP snapshot traversal for Store canonical sealing; its value builders are shared by the diff and mutation traversals.
use crate::schema::snapshot::*;
use store::{ArtifactCanonicalJsonValue as V,ArtifactCanonicalJsonNode as N,ArtifactCanonicalJsonArray as A,ArtifactCanonicalJsonObject as O};
pub fn text(value:&str)->V<'_>{V::Scalar(N::String(value))}
pub fn number<'a>(value:impl Into<u64>)->V<'a>{V::Scalar(N::U64(value.into()))}
fn signed<'a>(value:i32)->V<'a>{V::Scalar(N::I64(value.into()))}
pub fn object<'a,const K:usize>(mut fields:[(&'a str,V<'a>);K])->V<'a>{fields.sort_unstable_by(|a,b|a.0.cmp(b.0));V::Object(O::new(fields.into_iter()))}
pub fn array<'a,T:Sync+'a>(value:&'a [T],map:impl Fn(&'a T)->V<'a>+Send+'a)->V<'a>{V::Array(A::new(value.iter().map(map)))}
fn region(value:&BmpRegion)->V<'_>{object([("x",number(value.x)),("y",number(value.y)),("width",number(value.width)),("height",number(value.height))])}
fn sample(value:&BmpNativeSample)->V<'_>{object([("red",number(value.red)),("green",number(value.green)),("blue",number(value.blue)),("alpha",number(value.alpha)),("reserved",number(value.reserved))])}
pub fn rect(value:&BmpSampleRect)->V<'_>{rect_parts(&value.region,&value.indices,&value.samples)}
pub fn rect_parts<'a>(at:&'a BmpRegion,indices:&'a [u8],samples:&'a [BmpNativeSample])->V<'a>{
    let mut fields:Vec<(&str,V<'a>)>=vec![("region",region(at))];
    if !indices.is_empty(){fields.push(("indices",array(indices,|v|number(*v))));}
    if !samples.is_empty(){fields.push(("samples",array(samples,sample)));}
    fields.sort_unstable_by(|a,b|a.0.cmp(b.0));V::Object(O::new(fields.into_iter()))
}
fn snapshot(value:&BmpSnapshot)->V<'_>{object([("schema",text(&value.schema)),("image",image(&value.image))])}
pub fn image(value:&BmpImage)->V<'_>{object([
("width",number(value.width)),("height",number(value.height)),("rowOrder",text(match value.row_order {BmpRowOrder::BottomUp=>"bottomUp",BmpRowOrder::TopDown=>"topDown"})),("profile",text(value.profile.id())),("masks",array(&value.masks,|v|number(*v))),
("palette",array(&value.palette,|v|object([("r",number(v.r)),("g",number(v.g)),("b",number(v.b)),("reserved",number(v.reserved))]))),
("pixels",match &value.pixels {BmpPixels::Indexed{indices}=>object([("storage",text("indexed")),("indices",array(indices,|v|number(*v)))]),BmpPixels::Direct{samples}=>object([("storage",text("direct")),("samples",array(samples,|v|object([("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha)),("reserved",number(v.reserved))])))] )}),
("xPixelsPerMeter",signed(value.x_pixels_per_meter)),("yPixelsPerMeter",signed(value.y_pixels_per_meter)),("colorsUsed",number(value.colors_used)),("colorsImportant",number(value.colors_important)),("reserved1",number(value.reserved_1)),("reserved2",number(value.reserved_2)),("opaqueGap",array(&value.opaque_gap,|v|number(*v))),("opaqueTrailer",array(&value.opaque_trailer,|v|number(*v)))
])}
impl store::ArtifactCanonicalJson for BmpSnapshot {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>, semio_framework_value::ValueError>{Ok(Some(snapshot(self)))}
}
