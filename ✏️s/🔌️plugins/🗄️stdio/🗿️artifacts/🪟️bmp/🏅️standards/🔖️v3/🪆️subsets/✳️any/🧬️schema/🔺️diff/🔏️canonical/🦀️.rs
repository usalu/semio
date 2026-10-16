//! 🔏️ Borrowed exact typed BMP diff traversal for Store canonical sealing.
use crate::schema::diff::BmpDiff;
use crate::schema::snapshot::canonical::{array,image,rect};
use store::{ArtifactCanonicalJsonValue as V,ArtifactCanonicalJsonObject as O};
impl store::ArtifactCanonicalJson for BmpDiff {
    fn canonical_json_borrowed_root(&self)->Result<Option<V<'_>>, semio_framework_value::ValueError>{Ok(Some(V::Object(O::new(self.image.as_ref().map(|value|("image",image(value))).into_iter().chain((!self.rects.is_empty()).then(||("rects",array(&self.rects,rect))))))))}
}
