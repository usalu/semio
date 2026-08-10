//! IO stdio.pptx
//#region Register
pub fn register() {
    crate::artifacts::pptx::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::pptx::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::pptx::io::export::serializers::artifacts::zip::register();
    crate::artifacts::pptx::io::export::serializers::artifacts::xml::register();
}
//#endregion Register
