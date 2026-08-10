//! IO stdio.pdf
//#region Register
pub fn register() {
    crate::artifacts::pdf::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::pdf::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::pdf::io::export::serializers::artifacts::binary::register();
    crate::artifacts::pdf::io::export::serializers::artifacts::deflate::register();
}
//#endregion Register
