//! 🔏️ Borrowed exact typed BMP mutation traversal for Store canonical sealing, with the laws of all three canonical traversals.
use crate::schema::mutations::BmpMutation;
use crate::schema::snapshot::canonical::{image,number,object,rect_parts,text};
use store::{ArtifactCanonicalJsonValue as V};
#[cfg(test)]
use crate::schema::{snapshot::*,diff::BmpDiff};
impl store::ArtifactCanonicalJson for BmpMutation {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>, semio_framework_value::ValueError>{
        let (kind,payload)=match self {
            Self::ReplaceImage(v)=>("replace-image",object([("image",image(&v.image))])),
            Self::ReplaceSamples(v)=>("replace-samples",rect_parts(&v.region,&v.indices,&v.samples)),
            Self::PaintIndexedRegion(v)=>("paint-indexed-region",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("paletteIndex",number(v.palette_index))])),
            Self::PaintDirectRegion(v)=>("paint-direct-region",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha))])),
        };Ok(Some(object([("mutation",text(kind)),("payload",payload)])))
    }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
