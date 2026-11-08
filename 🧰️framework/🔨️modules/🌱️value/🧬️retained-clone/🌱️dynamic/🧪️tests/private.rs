//! 🔒️ Every real fixture denial preserves the private source-binding state.
use super::*;
use super::protocol::*;
use crate::observe_retirement_allocations as observe;
#[test]
fn denied_dynamic_clone_preserves_original_binding_state(){for(name,encoded)in cases(){for copy in [1,3,64]{for cancelled in [None,Some(0),Some(1),Some(3),Some(8),Some(17)]{private_case(&name,&encoded,copy,cancelled)}}}}
fn private_case(name:&str,encoded:&serde_json::Value,copy:usize,cancelled:Option<usize>){
    let(owner,heap)=observe(||decode(encoded));let(mut born,mut freed)=heap;let(mut source,heap)=admit_source(owner);born+=heap.0;freed+=heap.1;let(mut cursor,heap)=observe(||Harness::new(&source));assert_eq!(heap,(0,0));let mut bound=false;
    let actual=advance_original(&mut source,&mut cursor,cancelled,|cursor,before|{if before{bound=cursor.state.source.is_some();}else{assert_eq!(cursor.state.source.is_some(),bound);}});born+=actual.born;freed+=actual.freed;assert!(actual.complete||cancelled==Some(actual.turns));close_cursor(&mut source,&mut cursor,copy,&mut born,&mut freed);cursor.release(&mut source);assert_eq!(observe(||drop(cursor)).1,(0,0));close_source(&mut source,copy,&mut born,&mut freed);assert_eq!(observe(||drop(source)).1,(0,0));
}
