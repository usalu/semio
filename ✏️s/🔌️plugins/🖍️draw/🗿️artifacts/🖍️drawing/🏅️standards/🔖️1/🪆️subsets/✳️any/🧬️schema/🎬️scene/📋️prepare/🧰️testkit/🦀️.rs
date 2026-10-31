//! 🧰️ Physical-close macros shared with the drawing writer laws (behind the `testkit` feature).

#[macro_export]
macro_rules! draw_scene_physical_turn{($owner:expr,$items:expr)=>{{let owner=&mut $owner;let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:$items,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};assert_eq!(owner.close_step(semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:0,..grant}).unwrap().progress(),Default::default());let step=owner.close_step(grant).unwrap();assert!(step.progress().fits(grant));step}};}
#[macro_export]
macro_rules! draw_scene_physical_close{($owner:expr,$items:expr)=>{{let mut turns=0;while !$owner.terminal_is_empty(){crate::draw_scene_physical_turn!($owner,$items);turns+=1;assert!(turns<2_000_000);}turns}};}

/// 🚦️ Finite fixture authority is never renewed when a real producer cannot perform another turn.
#[macro_export]
macro_rules! draw_scene_funded_turn{($job:expr,$items:expr,$wallet:expr)=>{{let job=&mut $job;$wallet.turn($items,|work|{let before=work.receipt();let p=job.advance($items,work).unwrap();assert!(p.done||work.receipt()!=before||job.last_turn_work()>0,"finite scene fixture exhausted at {:?}",job.next_work_demand());p})}};}

/// 🧾️ The exact same finite fixture stock pays actual close progress without deriving new authority from demand.
#[macro_export]
macro_rules! draw_scene_funded_close{($owner:expr,$wallet:expr,$items:expr)=>{{let owner=&mut $owner;let wallet=&mut $wallet;let mut turns=0;let mut work=0;while !owner.terminal_is_empty(){for _ in 0..$items{if owner.terminal_is_empty(){break}let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();let d=semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()};let p=wallet.close_turn(d,|g|owner.close_step(g)).unwrap().progress();assert_ne!(p,Default::default());work+=p.copied_items as u64;turns+=1;assert!(turns<2_000_000);}}work}};}
