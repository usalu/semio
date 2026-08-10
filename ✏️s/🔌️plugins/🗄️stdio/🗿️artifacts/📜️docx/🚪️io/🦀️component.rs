//! IO stdio.docx
//#region Register
pub fn register() {
    crate::artifacts::docx::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::docx::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::docx::io::export::serializers::artifacts::zip::register();
    crate::artifacts::docx::io::export::serializers::artifacts::xml::register();
}
//#endregion Register
