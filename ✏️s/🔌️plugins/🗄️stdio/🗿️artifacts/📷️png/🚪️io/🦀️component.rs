//! IO stdio.png
//#region Register
pub fn register() {
    crate::artifacts::png::io::import::deserializers::artifacts::binary::register();
    crate::artifacts::png::io::import::deserializers::artifacts::deflate::register();
    crate::artifacts::png::io::export::serializers::artifacts::binary::register();
    crate::artifacts::png::io::export::serializers::artifacts::deflate::register();
}
//#endregion Register
