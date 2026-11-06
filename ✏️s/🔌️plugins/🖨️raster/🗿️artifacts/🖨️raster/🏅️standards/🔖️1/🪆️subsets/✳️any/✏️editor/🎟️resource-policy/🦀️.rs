//! 🎟️ Raster's embedding admits one reserved RGBA backing while initialization and copying remain paged.
use semio_plugin::MountedOwnerPolicyV1;
/// 🖼️ This per-turn policy lends no lifetime stock and never increases the original caller's currencies.
pub const fn raster_mounted_owner_policy()->MountedOwnerPolicyV1{
 let mut policy=MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;let bytes=semio_framework_pixels::editing::MAX_IMAGE_PIXELS*4;policy.preparation.maximum_capacity_bytes=bytes;policy.preparation.maximum_release_bytes=bytes;policy.maintenance.maximum_capacity_bytes=bytes;policy.maintenance.maximum_release_bytes=bytes;policy.close.maximum_capacity_bytes=bytes;policy.close.maximum_release_bytes=bytes;policy
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn raster_mounted_pixel_policy_matches_closed_neutral_phases_without_increasing_caller_stock(){
  let law:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();let policy=raster_mounted_owner_policy();assert_eq!(semio_framework_pixels::editing::MAX_IMAGE_PIXELS,law["maximumPixels"].as_u64().unwrap()as usize);for grant in[policy.preparation,policy.maintenance,policy.close]{assert_eq!(grant.maximum_items,law["maximumItems"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_copy_bytes,law["maximumCopyBytes"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_capacity_bytes,law["maximumCapacityBytes"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_release_bytes,law["maximumReleaseBytes"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_depth,law["maximumDepth"].as_u64().unwrap()as usize);for row in law["callerIntersections"].as_array().unwrap(){assert_eq!(grant.maximum_capacity_bytes.min(row["callerCapacity"].as_u64().unwrap()as usize),row["expectedCapacity"].as_u64().unwrap()as usize);assert_eq!(grant.maximum_release_bytes.min(row["callerRelease"].as_u64().unwrap()as usize),row["expectedRelease"].as_u64().unwrap()as usize);}}for width in law["widths"].as_array().unwrap(){for phase in law["phases"].as_array().unwrap(){let bytes=law["maximumPixels"].as_u64().unwrap()*phase["bytesPerPixel"].as_u64().unwrap()+phase["arcHeaders"].as_u64().unwrap()*(width["imageHeaderBytes"].as_u64().unwrap()+2*width["pointerBytes"].as_u64().unwrap());assert!(bytes<=law["maximumCapacityBytes"].as_u64().unwrap());}}assert!(!law["sourceAllocationInitializesPixels"].as_bool().unwrap());assert_eq!(law["maximumInitializationBytes"],law["maximumCopyBytes"]);
 }
}
