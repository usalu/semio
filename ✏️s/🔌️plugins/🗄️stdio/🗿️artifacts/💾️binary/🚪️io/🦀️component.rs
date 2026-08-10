//! IO stdio.binary
//#region Register
pub fn register() {
    crate::artifacts::binary::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::binary::io::export::serializers::artifacts::binary::register();
}
//#endregion Register
