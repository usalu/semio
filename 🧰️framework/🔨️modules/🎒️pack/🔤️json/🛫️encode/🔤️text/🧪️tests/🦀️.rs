//! 🔤️ Serde independently checks borrowed native text and exact cursor grants.
use super::*;
use semio_framework_value::{paged::PagedUtf8, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
fn grant(copy:usize)->RetainedCloneGrant {RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:1}}
fn close(cursor:&mut ArtifactCanonicalJsonTextCursor<'_>) {
    cursor.begin_close();
    let demand=cursor.next_demand();
    let (step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(grant(demand.copy_bytes)).unwrap());
    assert!(step.progress().fits(grant(demand.copy_bytes)));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    assert!(cursor.terminal_is_empty());
}
#[test]
fn canonical_native_paged_text_cursor_matches_serde_and_exact_granted_cancellation() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        for field in ["keyChunks","valueChunks"] {
            let text:String=row[field].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();
            let expected=serde_json::to_vec(&text).unwrap();
            let native=PagedUtf8::<{usize::MAX}>::from(text);
            for maximum in fixture["copyGrants"].as_array().unwrap() {
                let maximum=maximum.as_u64().unwrap() as usize;
                for stop in [0,1,3,17,129,usize::MAX] {
                    let original=ArtifactCanonicalJsonText::Native(&native);
                    let birth=ArtifactCanonicalJsonTextCursor::constructor_demand();
                    let (refusal,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactCanonicalJsonTextCursor::admit(original,grant(birth.copy_bytes-1)));
                    assert!(refusal.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    let (admitted,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ArtifactCanonicalJsonTextCursor::admit(original,grant(birth.copy_bytes)));
                    let (mut cursor,receipt)=admitted.unwrap();assert!(receipt.fits(grant(birth.copy_bytes)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    let mut byte=[0];let zero=RetainedCloneGrant{maximum_items:0,..grant(maximum)};
                    let (blocked,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(&mut byte,zero).unwrap());
                    assert_eq!(blocked.step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                    let mut output=Vec::new();
                    for turn in 0..100_000 {
                        if cursor.is_complete()||turn==stop {break;}
                        let (result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(&mut byte,grant(maximum)).unwrap());
                        assert!(result.step.progress().fits(grant(maximum)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                        output.extend_from_slice(&byte[..result.written_bytes]);
                    }
                    assert_eq!(output,expected[..output.len()]);
                    if stop==usize::MAX {assert!(cursor.is_complete());assert_eq!(output,expected);}
                    close(&mut cursor);
                    let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
                }
            }
        }
    }
    println!("[DEBUG] borrowed Pack native key/scalar text matches Serde Unicode/NUL/escaping; zero grants/admission below/exact/cancel return zero heap with original chunks retained");
}
