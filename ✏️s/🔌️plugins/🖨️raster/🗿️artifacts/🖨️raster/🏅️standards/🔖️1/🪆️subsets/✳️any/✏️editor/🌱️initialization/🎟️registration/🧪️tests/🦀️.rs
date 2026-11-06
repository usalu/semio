//! 🧪️ The concrete type recipe agrees with the neutral ownership declaration and genuine app-associated member family.
use super::*;
use semio_plugin::WindowTransientOwner;
#[test]
fn raster_native_registration_keeps_its_actual_lane_and_window_types(){
 let fixture:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../fixture.json")).unwrap();
 fn registration<R:semio_plugin::original_instance_registration::OriginalInstanceRegistration<EditorApp<RasterPlayApp>,<RasterPlayApp as ArtifactEditor>::Members>>(){}
 registration::<RasterNativeRegistration>();
 fn presence<D:OriginalInstanceDeclarations<EditorApp<RasterPlayApp>,Ephemeral=crate::app::original_ephemeral::RasterEphemeralDeclaration>>(){}
 presence::<RasterInstanceDeclarations>();
 assert!(fixture["persistedWindowOwners"].as_array().unwrap().is_empty());assert_eq!(fixture["transientWindowOwners"][0],RasterCompositeWindowTransientOwner::WINDOW_KIND_ID);
 assert_eq!(std::any::TypeId::of::<<RasterPlayApp as ArtifactEditor>::Members>(),std::any::TypeId::of::<semio_s_artifact_stdio_semio::SemioMembers>());
}
