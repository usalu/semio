//! 🎟️ Raster's embedding admits one reserved RGBA backing while initialization and copying remain paged.
use semio_plugin::MountedOwnerPolicyV1;
/// 🖼️ This per-turn policy lends no lifetime stock and never increases the original caller's currencies.
pub const fn raster_mounted_owner_policy()->MountedOwnerPolicyV1{
 let mut policy=MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
 let bytes=semio_framework_pixels::editing::MAX_IMAGE_PIXELS*4;
 policy.preparation.maximum_items=1024;
 policy.preparation.maximum_copy_bytes=1<<20;
 policy.preparation.maximum_capacity_bytes=bytes;
 policy.preparation.maximum_release_bytes=bytes;
 policy.maintenance.maximum_capacity_bytes=bytes;
 policy.maintenance.maximum_release_bytes=bytes;
 policy.close.maximum_capacity_bytes=bytes;
 policy.close.maximum_release_bytes=bytes;
 policy
}
#[cfg(test)]
mod tests{
 use super::*;
 #[test]
 fn raster_mounted_pixel_policy_matches_closed_neutral_phases_without_increasing_caller_stock(){
  let law:serde_json::Value=serde_json::from_str(include_str!("fixture.json")).unwrap();
  let policy=raster_mounted_owner_policy();
  let n=|value:&serde_json::Value,key:&str|value[key].as_u64().unwrap()as usize;
  assert_eq!(semio_framework_pixels::editing::MAX_IMAGE_PIXELS,n(&law,"maximumPixels"));
  for (grant,bounds) in [(policy.preparation,&law["preparation"]),(policy.maintenance,&law["maintenanceAndClose"]),(policy.close,&law["maintenanceAndClose"])]{
   assert_eq!(grant.maximum_items,n(bounds,"maximumItems"));
   assert_eq!(grant.maximum_copy_bytes,n(bounds,"maximumCopyBytes"));
   assert_eq!(grant.maximum_capacity_bytes,n(&law,"maximumCapacityBytes"));
   assert_eq!(grant.maximum_release_bytes,n(&law,"maximumReleaseBytes"));
   assert_eq!(grant.maximum_depth,n(&law,"maximumDepth"));
   for row in law["callerIntersections"].as_array().unwrap(){
    assert_eq!(grant.maximum_capacity_bytes.min(n(row,"callerCapacity")),n(row,"expectedCapacity"));
    assert_eq!(grant.maximum_release_bytes.min(n(row,"callerRelease")),n(row,"expectedRelease"));
   }
  }
  for width in law["widths"].as_array().unwrap(){
   for phase in law["phases"].as_array().unwrap(){
    let bytes=n(&law,"maximumPixels")*n(phase,"bytesPerPixel")+n(phase,"arcHeaders")*(n(width,"imageHeaderBytes")+2*n(width,"pointerBytes"));
    assert!(bytes<=n(&law,"maximumCapacityBytes"));
   }
  }
  assert!(!law["sourceAllocationInitializesPixels"].as_bool().unwrap());
  assert_eq!(law["maximumInitializationBytes"],law["maintenanceAndClose"]["maximumCopyBytes"]);
 }
}
