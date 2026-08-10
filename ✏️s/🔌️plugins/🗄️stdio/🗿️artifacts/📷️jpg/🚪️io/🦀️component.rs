//! IO stdio.jpg
//#region Register
pub fn register() {
    crate::artifacts::jpg::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::jpg::io::export::serializers::artifacts::binary::register();
}
//#endregion Register
