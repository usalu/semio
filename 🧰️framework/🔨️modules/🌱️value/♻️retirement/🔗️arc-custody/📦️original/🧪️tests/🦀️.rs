//! 🧵️ Actual initialized allocations elect one owner and retain partial fields until funded release.
use super::*;
use crate::observe_retirement_allocations as observe;
use std::sync::atomic::AtomicUsize;
struct Probe(Arc<AtomicUsize>);
impl Drop for Probe{fn drop(&mut self){self.0.fetch_add(1,Ordering::SeqCst);}}
#[test]
fn concurrent_alias_retirement_elects_one_original_without_moving_or_dropping_it(){
 let drops=Arc::new(AtomicUsize::new(0));let storage=SharedOriginalStorage::new(Probe(Arc::clone(&drops)));let pointer=storage.identity();let peer=storage.duplicate().unwrap();let left=std::thread::spawn(move||storage.retire());let right=std::thread::spawn(move||peer.retire());let mut original=None;let mut aliases=0;for result in[left.join().unwrap(),right.join().unwrap()]{match result{SharedOriginalRelease::Alias=>aliases+=1,SharedOriginalRelease::Original(owner)=>{assert!(original.replace(owner).is_none());}}}assert_eq!(aliases,1);let mut owner=original.unwrap();assert_eq!(owner.identity(),pointer);assert_eq!(drops.load(Ordering::SeqCst),0);drop(owner.take_value().unwrap());assert_eq!(drops.load(Ordering::SeqCst),1);assert!(owner.terminal_is_empty());
}
#[test]
fn selected_field_retirement_retains_untouched_originals_and_funds_final_storage_release(){
 struct Pair{left:Probe,right:Probe}
 let drops=Arc::new(AtomicUsize::new(0));let(storage,heap)=observe(||SharedOriginalStorage::new(Pair{left:Probe(Arc::clone(&drops)),right:Probe(Arc::clone(&drops))}));assert_eq!(heap,(SharedOriginalStorage::<Pair>::allocation_bytes(),0));let pointer=storage.identity();let SharedOriginalRelease::Original(mut owner)=storage.retire()else{panic!("one original owner")};assert_eq!(owner.identity(),pointer);let failed:Result<(),_>=Err(owner);owner=failed.unwrap_err();assert_eq!(owner.identity(),pointer);assert_eq!(drops.load(Ordering::SeqCst),0);let left=unsafe{owner.take_field(|root|std::ptr::addr_of_mut!((*root).left))};drop(left);assert_eq!(drops.load(Ordering::SeqCst),1);let right=unsafe{owner.take_field(|root|std::ptr::addr_of_mut!((*root).right))};drop(right);assert_eq!(drops.load(Ordering::SeqCst),2);let below=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:SharedOriginal::<Pair>::allocation_bytes()-1,maximum_depth:1,..Default::default()};let(receipt,heap)=observe(||unsafe{owner.release_empty(below)});assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));assert!(!owner.terminal_is_empty());let issued=RetainedCloneGrant{maximum_release_bytes:SharedOriginal::<Pair>::allocation_bytes(),..below};let(receipt,heap)=observe(||unsafe{owner.release_empty(issued)});assert!(receipt.fits(issued));assert_eq!(heap,(0,receipt.released_bytes));assert!(owner.terminal_is_empty());assert_eq!(observe(||drop(owner)).1,(0,0));
}

#[test]
fn delayed_physical_alias_drop_cannot_free_initialized_or_unfunded_storage(){
 let storage=SharedOriginalStorage::new(7u64);let peer=storage.duplicate().unwrap();let delayed=Arc::clone(peer.source.as_ref().unwrap());assert!(matches!(peer.retire(),SharedOriginalRelease::Alias));let SharedOriginalRelease::Original(mut original)=storage.retire()else{panic!("one elected original")};assert!(!original.physical_is_unique());let issued=RetainedCloneGrant{maximum_items:1,maximum_release_bytes:SharedOriginal::<u64>::allocation_bytes(),maximum_depth:1,..Default::default()};let value=unsafe{original.take_field(|root|root)};assert_eq!(value,7);let(receipt,heap)=observe(||unsafe{original.release_empty(issued)});assert_eq!(receipt,Default::default());assert_eq!(heap,(0,0));let(_,heap)=observe(||drop(delayed));assert_eq!(heap,(0,0));assert!(original.physical_is_unique());let(receipt,heap)=observe(||unsafe{original.release_empty(issued)});assert_eq!(heap,(0,receipt.released_bytes));assert_eq!(receipt.released_bytes,SharedOriginal::<u64>::allocation_bytes());assert!(original.terminal_is_empty());
}
