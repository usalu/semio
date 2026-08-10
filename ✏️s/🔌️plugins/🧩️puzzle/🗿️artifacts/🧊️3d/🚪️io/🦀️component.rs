//! puzzle3d IO stdio matrix
pub fn register() {
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::dwg::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::glb::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::gltf::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::json::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::las::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::obj::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::ply::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::png::register();
    crate::artifacts::puzzle3d::io::import::deserializers::artifacts::stl::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::dwg::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::glb::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::gltf::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::json::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::las::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::obj::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::ply::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::png::register();
    crate::artifacts::puzzle3d::io::export::serializers::artifacts::stl::register();
}
pub fn import_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"]
}
pub fn export_stdio_kinds() -> &'static [&'static str] {
    &["stdio.dwg", "stdio.glb", "stdio.gltf", "stdio.json", "stdio.las", "stdio.obj", "stdio.ply", "stdio.png", "stdio.stl"]
}
