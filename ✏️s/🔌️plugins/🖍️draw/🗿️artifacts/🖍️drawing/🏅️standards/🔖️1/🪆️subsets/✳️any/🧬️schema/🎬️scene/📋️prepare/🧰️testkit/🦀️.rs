//! 🧰️ Physical-close macros shared with the drawing writer laws (behind the `testkit` feature).

#[macro_export]
macro_rules! draw_scene_physical_turn{($owner:expr,$items:expr)=>{{let owner=&mut $owner;let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:$items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};assert_eq!(owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());let step=owner.close_step(grant).unwrap();assert!(step.progress().fits(grant));step}};}
#[macro_export]
macro_rules! draw_scene_physical_close{($owner:expr,$items:expr)=>{{let mut turns=0;while !$owner.terminal_is_empty(){crate::draw_scene_physical_turn!($owner,$items);turns+=1;assert!(turns<2_000_000);}turns}};}
