//! IO stdio.tiff
//#region Register
pub fn register() {
    crate::artifacts::tiff::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::tiff::io::export::serializers::artifacts::binary::register();
}
//#endregion Register
