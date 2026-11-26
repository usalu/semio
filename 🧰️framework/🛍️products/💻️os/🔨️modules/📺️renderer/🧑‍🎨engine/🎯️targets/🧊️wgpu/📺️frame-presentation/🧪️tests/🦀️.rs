//! 📺️ Unit tests of frame-presentation; the work package moves the tests of its sources here.
use crate::{media_input_step,media_holds_input_barrier,MediaInputStep};
#[test]
fn pending_media_input_releases_loans_in_bounded_turns(){
 let(mut loans,mut released,mut queued,mut first)=(3,false,2,None);
 for turn in 0..16{let pending=queued>0;if !media_holds_input_barrier(released,false)&&pending{queued-=1;first.get_or_insert(turn);continue}
  match media_input_step(pending,&mut released,false){MediaInputStep::Release=>{loans-=1;released=loans==0}MediaInputStep::Hold=>unreachable!(),MediaInputStep::Collect=>loans=3}}
 assert!(first.unwrap()<=3&&queued==0&&loans==3&&!released);let mut r=true;assert_eq!(media_input_step(true,&mut r,true),MediaInputStep::Collect);assert!(!r&&!media_holds_input_barrier(false,true));
}
