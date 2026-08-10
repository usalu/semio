//! IO stdio.obj
pub fn register() {
    crate::artifacts::obj::io::import::deserializers::artifacts::txt::register();
    crate::artifacts::obj::io::export::serializers::artifacts::txt::register();
}
