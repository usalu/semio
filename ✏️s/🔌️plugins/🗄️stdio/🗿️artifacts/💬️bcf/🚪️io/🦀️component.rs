//! IO stdio.bcf
//#region Register
pub fn register() {
    crate::artifacts::bcf::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::bcf::io::import::deserializers::artifacts::xml::register();
    crate::artifacts::bcf::io::export::serializers::artifacts::zip::register();
    crate::artifacts::bcf::io::export::serializers::artifacts::xml::register();
}
//#endregion Register
