use super::*;
use crate::{Cap, test_allocation::observe_backing};

#[test]
fn original_canvas_stroke_preserves_every_cancelled_dash_and_physical_receipt() {
    let example: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let policy: RetainedCloneGrant = serde_json::from_value(example["grant"].clone()).unwrap();
    let cap = |value: u64| match value { 0 => Cap::Butt, 1 => Cap::Round, 2 => Cap::Square, _ => unreachable!() };
    for row in example["cases"].as_array().unwrap() {
        let expected: Vec<f64> = row["dashes"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect();
        let mut oracle = kurbo::Stroke::new(row["width"].as_f64().unwrap());
        oracle.start_cap = match cap(row["caps"][0].as_u64().unwrap()) { Cap::Butt => kurbo::Cap::Butt, Cap::Round => kurbo::Cap::Round, Cap::Square => kurbo::Cap::Square };
        oracle.end_cap = match cap(row["caps"][1].as_u64().unwrap()) { Cap::Butt => kurbo::Cap::Butt, Cap::Round => kurbo::Cap::Round, Cap::Square => kurbo::Cap::Square };
        oracle.dash_pattern = expected.iter().copied().collect();
        for cut in example["cuts"].as_array().unwrap() {
            let (mut original, original_birth, original_free) = observe_backing(|| {
                let mut values = Vec::with_capacity(row["capacity"].as_u64().unwrap() as usize); values.extend_from_slice(&expected);
                let mut original = Stroke::new(row["width"].as_f64().unwrap()); original.set_start_cap(cap(row["caps"][0].as_u64().unwrap())); original.set_end_cap(cap(row["caps"][1].as_u64().unwrap())); original.set_dash_pattern(values); original
            });
            assert_eq!(original_free, 0);
            assert_eq!(original.to_kurbo(), oracle);
            let pointer = original.dash_pattern().as_ptr(); let capacity = original.dash_capacity();
            let birth = CanvasStrokeRetirement::birth_demand();
            for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_depth: 0, ..policy }, RetainedCloneGrant { maximum_copy_bytes: birth.copy_bytes-1, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: birth.capacity_bytes-1, ..policy }] {
                let (result, allocated, released) = observe_backing(|| CanvasStrokeRetirement::admit(original, denied));
                original = result.err().unwrap().1; assert_eq!((allocated,released),(0,0)); assert_eq!(original.dash_pattern().as_ptr(),pointer); assert_eq!(original.dash_capacity(),capacity); assert_eq!(original.dash_pattern(),expected); assert_eq!(original.to_kurbo(),oracle);
            }
            let ((owner, progress), allocated, released) = observe_backing(|| CanvasStrokeRetirement::admit(original,policy).unwrap_or_else(|(error,_)|panic!("{error}")));
            assert_eq!((allocated,released),(progress.retained_capacity_bytes,0)); assert!(progress.fits(policy));
            let mut slot = Some(owner); let mut born = original_birth+allocated; let mut freed = 0;
            let first_copy = slot.as_ref().unwrap().next_copy_byte_demand().unwrap();
            for denied in [RetainedCloneGrant { maximum_copy_bytes: 0, ..policy }, RetainedCloneGrant { maximum_copy_bytes: first_copy-1, ..policy }] {
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,denied).unwrap());
                assert_eq!(step.progress(),Default::default()); assert_eq!((allocated,released),(0,0)); let retained=slot.as_ref().unwrap().original_stroke().unwrap(); assert_eq!(retained.dash_pattern().as_ptr(),pointer); assert_eq!(retained.dash_capacity(),capacity); assert_eq!(retained.dash_pattern(),expected);
            }
            for _ in 0..cut.as_u64().unwrap() {
                if slot.is_none(){break;}
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,policy).unwrap());
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=allocated;freed+=released;
            }
            if let Some(owner)=slot.as_ref() {
                if let Some(values)=owner.controlled_original() {assert_eq!(values.as_ptr(),pointer);assert_eq!(values.capacity(),capacity);assert_eq!(values.as_slice(),expected);}
                let shell=owner.as_ref() as *const CanvasStrokeRetirement;
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant{maximum_items:0,..policy}).unwrap());
                assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(slot.as_ref().unwrap().as_ref() as *const CanvasStrokeRetirement,shell);
            }
            for _ in 0..128 {
                if slot.is_none(){break;}
                if slot.as_ref().unwrap().next_copy_byte_demand().unwrap()==size_of::<f64>() {
                    for copy in [0,size_of::<f64>()-1] {
                        let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant{maximum_copy_bytes:copy,..policy}).unwrap());
                        assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));
                    }
                }
                let owner=slot.as_ref().unwrap();let depth=owner.next_depth_demand().unwrap();
                if depth!=0 {
                    let (result,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant{maximum_depth:depth-1,..policy}));
                    assert!(result.is_err());assert_eq!((allocated,released),(0,0));
                }
                let release=slot.as_ref().unwrap().next_release_byte_demand().unwrap();
                if release!=0 {
                    let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant{maximum_release_bytes:release-1,..policy}).unwrap());
                    assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));
                }
                if slot.as_ref().unwrap().terminal_is_empty() {
                    let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,RetainedCloneGrant{maximum_release_bytes:size_of::<CanvasStrokeRetirement>()-1,..policy}).unwrap());
                    assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));
                }
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,policy).unwrap());
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert_ne!(step.progress(),Default::default());born+=allocated;freed+=released;
            }
            assert!(slot.is_none());assert_eq!(born,freed);let(_,allocated,released)=observe_backing(||drop(slot));assert_eq!((allocated,released),(0,0));
            println!("[DEBUG] Original Canvas Stroke={} cancelledCut={} originalPointerCapacityOrder=true Kurbo=true everyTurnSystem=true bornFreed={born} suppliedPolicy=true terminalShellSeparate=true Drop0",row["id"],cut);
        }
    }
}
