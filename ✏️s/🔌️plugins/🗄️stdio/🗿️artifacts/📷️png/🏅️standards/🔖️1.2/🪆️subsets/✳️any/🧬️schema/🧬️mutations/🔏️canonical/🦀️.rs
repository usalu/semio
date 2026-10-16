//! 🔏️ Borrowed exact typed PNG mutation traversal for Store canonical sealing, with the laws of all three canonical traversals.
use crate::schema::mutations::PngMutation;
use crate::schema::snapshot::canonical::{array,image,number,object,optional,paint,region,text};
use store::{ArtifactCanonicalJsonValue as V};
#[cfg(test)]
use crate::schema::{snapshot::*,diff::PngDiff};
impl store::ArtifactCanonicalJson for PngMutation {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>, semio_framework_value::ValueError>{
        let (kind,payload)=match self {
            Self::ReplaceImage(v)=>("replace-image",object([("image",image(&v.image))])),
            Self::ChangeGamma(v)=>("change-gamma",object([("revision",text(&v.revision)),("gama",optional(v.gama.as_ref(),|v|number(*v)))])),
            Self::PatchPixels(v)=>("patch-pixels",object([("revision",text(&v.revision)),("x",number(v.x)),("y",number(v.y)),("width",number(v.width)),("height",number(v.height)),("red",number(v.red)),("green",number(v.green)),("blue",number(v.blue)),("alpha",number(v.alpha))])),
            Self::PaintNativeSamples(v)=>("paint-native-samples",object([("revision",text(&v.revision)),("region",region(&v.region)),("paint",paint(&v.paint))])),
            Self::SetGamma(v)=>("set-gamma",object([("gama",optional(v.gama.as_ref(),|v|number(*v)))])),
            Self::ReplaceSamples(v)=>("replace-samples",object([("region",region(&v.region)),("samples",array(&v.samples,|v|number(*v)))])),
        };Ok(Some(object([("mutation",text(kind)),("payload",payload)])))
    }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
