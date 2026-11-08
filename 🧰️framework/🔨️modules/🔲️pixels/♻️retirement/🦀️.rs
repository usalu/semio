//! ♻️ Raster and mask read leases retain original allocations through explicitly funded close.
pub use semio_framework_pixels_codecs::retirement::RasterLease;
use semio_framework_value::retirement::{RetireOwned,arc_custody::SharedControlledRetirement};
use std::{sync::Arc,ops::{Deref,DerefMut}};
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct MaskBytes(pub Vec<u8>);
impl Deref for MaskBytes {type Target=[u8];fn deref(&self)->&[u8]{&self.0}}
impl DerefMut for MaskBytes {fn deref_mut(&mut self)->&mut[u8]{&mut self.0}}
#[derive(Clone,Debug)]
pub struct MaskLease(pub Arc<MaskBytes>);
impl Deref for MaskLease {type Target=[u8];fn deref(&self)->&[u8]{&self.0}}
impl From<Vec<u8>> for MaskLease {fn from(value:Vec<u8>)->Self{Self(Arc::new(MaskBytes(value)))}}
impl RetireOwned for MaskLease {
 type RetirementCursor=SharedControlledRetirement<MaskBytes>;
 fn retirement_seed(self)->Self::RetirementCursor{SharedControlledRetirement::lease(self.0)}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<SharedControlledRetirement<MaskBytes>>())}
 fn controlled_retirement_supported()->bool{true}
}
