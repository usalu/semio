//! ♻️ Image read leases retain the codec's original allocation through funded close.
use crate::RasterImage;
use semio_framework_value::retirement::{RetireOwned,RetirementCursor,arc_custody::SharedControlledRetirement};
use std::{sync::Arc,ops::Deref,mem::size_of};
#[derive(Clone,Debug)]
pub struct RasterLease(pub Arc<RasterImage>);
impl Deref for RasterLease {type Target=RasterImage;fn deref(&self)->&RasterImage{&self.0}}
impl RetireOwned for RasterLease {
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(SharedControlledRetirement::lease(self.0))}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<RasterImage>>())}
 fn controlled_retirement_supported()->bool{true}
}
