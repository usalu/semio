use super::*;
use std::cell::Cell;

#[derive(Clone,Copy)]
struct ObservedSource<'source>{bytes:&'source [u8],reads:&'source Cell<usize>}
impl JsonReadSource for ObservedSource<'_>{
    fn byte_len(&self)->usize{self.bytes.len()}
    fn byte_at(&self,index:usize)->Option<u8>{self.reads.set(self.reads.get()+1);self.bytes.get(index).copied()}
}

#[test]
fn original_source_constructor_captures_all_limits_without_reads_or_heap_birth(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let text=row["source"].as_str().unwrap();let policy=&row["limits"];
        let limits=JsonReadLimits{maximum_bytes:policy["maximumBytes"].as_u64().unwrap(),maximum_allocation_bytes:policy["maximumAllocationBytes"].as_u64().unwrap()as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap()as usize,maximum_items:policy["maximumItems"].as_u64().unwrap()};
        let reads=Cell::new(0);let source=ObservedSource{bytes:text.as_bytes(),reads:&reads};
        let(result,born,released)=test_allocation::observe_backing(||JsonSourceCursor::<_,Value>::new(source,JsonMemberPolicy::Reject,limits));
        assert_eq!((reads.get(),born,released),(0,0,0));
        assert_eq!(result.is_ok(),row["expected"]=="admitted");
        assert_eq!(result.is_ok(),text.len()as u128<=u128::from(limits.maximum_bytes));
        match result{
            Ok(cursor)=>{
                assert_eq!(cursor.position(),0);assert_eq!(cursor.source_ref().bytes.as_ptr(),text.as_ptr());
                let((restored,grammar),born,released)=test_allocation::observe_backing(||cursor.into_grammar());
                assert_eq!((reads.get(),born,released),(0,0,0));assert_eq!(grammar.limits,limits);
                assert_eq!(restored.bytes.as_ptr(),text.as_ptr());assert!(std::ptr::eq(restored.reads,&reads));
                let(_,born,released)=test_allocation::observe_backing(||drop(grammar));assert_eq!((born,released),(0,0));
            },
            Err(error)=>{
                assert_eq!(error.kind,ValueRefusalKind::WorkLimit);assert_eq!(error.retained_progress(),RetainedCloneProgress::default());
                let(_,born,released)=test_allocation::observe_backing(||drop(error));assert_eq!((born,released),(0,0));
            },
        }
        assert_eq!(reads.get(),0);assert!(serde_json::from_str::<serde_json::Value>(text).is_ok());
    }
    eprintln!("[DEBUG] JSON policy5 constructor preserves all four original limits and source pointer with zero octet reads, heap birth or release; independent Serde source validity agrees");
}
