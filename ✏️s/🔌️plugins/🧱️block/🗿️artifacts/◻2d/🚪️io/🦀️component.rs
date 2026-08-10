//! block2d IO stdio matrix
pub fn register() {
    crate::artifacts::block2d::io::import::deserializers::artifacts::glb::register();
    crate::artifacts::block2d::io::import::deserializers::artifacts::json::register();
    crate::artifacts::block2d::io::import::deserializers::artifacts::obj::register();
    crate::artifacts::block2d::io::import::deserializers::artifacts::png::register();
    crate::artifacts::block2d::io::import::deserializers::artifacts::stl::register();
    crate::artifacts::block2d::io::import::deserializers::artifacts::zip::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::glb::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::json::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::obj::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::png::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::stl::register();
    crate::artifacts::block2d::io::export::serializers::artifacts::zip::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.glb", "stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.glb", "stdio.json", "stdio.obj", "stdio.png", "stdio.stl", "stdio.zip"]
}
