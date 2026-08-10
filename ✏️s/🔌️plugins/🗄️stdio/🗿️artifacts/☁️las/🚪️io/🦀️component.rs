//! IO stdio.las
//#region Register
pub fn register() {
    crate::artifacts::las::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::las::io::export::serializers::artifacts::binary::register();
}
//#endregion Register
