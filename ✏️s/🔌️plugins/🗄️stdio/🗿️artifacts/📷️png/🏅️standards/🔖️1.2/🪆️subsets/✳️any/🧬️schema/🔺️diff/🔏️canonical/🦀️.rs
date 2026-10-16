//! 🔏️ Borrowed exact typed PNG diff traversal for Store canonical sealing.
use crate::schema::diff::PngDiff;
use crate::schema::snapshot::canonical::{array,image,number,object,optional,region};
use store::{ArtifactCanonicalJsonValue as V,ArtifactCanonicalJsonObject as O};
impl store::ArtifactCanonicalJson for PngDiff {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>, semio_framework_value::ValueError>{
        let fields=[
            self.gamma.as_ref().map(|value|("gamma",object([("gama",optional(value.gama.as_ref(),|v|number(*v)))]))),
            self.image.as_ref().map(|value|("image",image(value))),
            (!self.rects.is_empty()).then(||("rects",array(&self.rects,|rect|object([("region",region(&rect.region)),("samples",array(&rect.samples,|v|number(*v)))])))),
        ];
        Ok(Some(V::Object(O::new(fields.into_iter().flatten()))))
    }
}
