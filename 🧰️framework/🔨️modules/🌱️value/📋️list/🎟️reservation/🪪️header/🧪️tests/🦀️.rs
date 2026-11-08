//! 🧪️ One admitted root header yields source-free work bounds for later bounded selection.
use super::*;
#[test]
fn selection_header_gates_zero_work_and_preserves_physical_ownership(){
 let owner=PagedList::<u64,4096>::empty();
 let(turn,heap)=crate::observe_retirement_allocations(||owner.inspect_selection_header(0));assert_eq!(turn.work,0);assert!(turn.result.unwrap().is_none());assert_eq!(heap,(0,0));
 let(turn,heap)=crate::observe_retirement_allocations(||owner.inspect_selection_header(1));assert_eq!(turn.work,1);assert_eq!(heap,(0,0));let header=turn.result.unwrap().unwrap();assert!(header.reservation_selection_work_demand(1)<PagedList::<u64,4096>::reservation_selection_work_demand());assert!(header.release_selection_work_demand()<PagedList::<u64,4096>::release_selection_work_demand());
}
#[test]
fn stale_selection_header_refuses_before_page_inspection(){
 let mut owner=PagedList::<u64,4>::empty();let header=owner.inspect_selection_header(1).result.unwrap().unwrap();let bytes=owner.next_allocation_bytes().unwrap();owner.reserve_one_granted(bytes,&mut None).unwrap();
 let(turn,heap)=crate::observe_retirement_allocations(||owner.select_reservation_from_header(&header,1,header.reservation_selection_work_demand(1)));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap_err().kind,PagedListRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));
 let(turn,heap)=crate::observe_retirement_allocations(||owner.select_release_from_header(&header,header.release_selection_work_demand()));assert_eq!(turn.work,1);assert_eq!(turn.result.unwrap_err().kind,PagedListRefusalKind::InvariantViolated);assert_eq!(heap,(0,0));while !owner.terminal_is_empty(){owner.release_empty_page_granted(usize::MAX).unwrap();}
}
