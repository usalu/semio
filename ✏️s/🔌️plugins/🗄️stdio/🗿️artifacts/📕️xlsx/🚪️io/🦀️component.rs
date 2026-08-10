//! IO stdio.xlsx
//#region Register
pub fn register() {
    crate::artifacts::xlsx::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::xlsx::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::xlsx::io::export::serializers::artifacts::zip::register();
    crate::artifacts::xlsx::io::export::serializers::artifacts::xml::register();
}
//#endregion Register
