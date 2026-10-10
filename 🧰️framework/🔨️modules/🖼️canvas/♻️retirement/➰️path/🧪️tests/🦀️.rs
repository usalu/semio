use super::*;
use crate::test_allocation::observe_backing;

#[test]
fn original_canvas_path_retains_all_cancelled_backings_and_exact_native_receipts() {
    let example: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let policy: RetainedCloneGrant = serde_json::from_value(example["grant"].clone()).unwrap();
    assert_eq!(size_of::<PathEl>(), 56);
    for row in example["cases"].as_array().unwrap() {
        let stream = row["path"][1].as_array().unwrap();
        let mut expected = Vec::new();
        let mut oracle = kurbo::BezPath::new();
        let mut index = 0;
        while index < stream.len() {
            let opcode = stream[index].as_u64().unwrap(); index += 1;
            let mut point = || { let point = geometry::Point::new(stream[index].as_f64().unwrap(), stream[index + 1].as_f64().unwrap()); index += 2; point };
            let element = match opcode { 0 => PathEl::MoveTo(point()), 1 => PathEl::LineTo(point()), 2 => PathEl::QuadTo(point(), point()), 3 => PathEl::CurveTo(point(), point(), point()), 4 => PathEl::ClosePath, _ => unreachable!() };
            let p = |point: geometry::Point| kurbo::Point::new(point.x, point.y);
            oracle.push(match element { PathEl::MoveTo(a) => kurbo::PathEl::MoveTo(p(a)), PathEl::LineTo(a) => kurbo::PathEl::LineTo(p(a)), PathEl::QuadTo(a,b) => kurbo::PathEl::QuadTo(p(a),p(b)), PathEl::CurveTo(a,b,c) => kurbo::PathEl::CurveTo(p(a),p(b),p(c)), PathEl::ClosePath => kurbo::PathEl::ClosePath });
            expected.push(element);
        }
        assert_eq!(expected.len(), oracle.elements().len());
        let actual_bounds = BezPath::from_elements(expected.clone()).bounding_box();
        let oracle_bounds = kurbo::Shape::bounding_box(&oracle);
        for (actual, expected) in [(actual_bounds.x0(), oracle_bounds.x0), (actual_bounds.y0(), oracle_bounds.y0), (actual_bounds.x1(), oracle_bounds.x1), (actual_bounds.y1(), oracle_bounds.y1)] {
            assert!((actual - expected).abs() < 1e-9, "original path bounds differ from Kurbo");
        }
        for cut in example["cuts"].as_array().unwrap() {
            let (elements, original_birth, released) = observe_backing(|| {
                let mut elements = Vec::with_capacity(row["capacity"].as_u64().unwrap() as usize);
                elements.extend_from_slice(&expected);
                elements
            });
            assert_eq!(released, 0);
            let pointer = elements.as_ptr(); let capacity = elements.capacity();
            let mut path = BezPath::from_elements(elements);
            assert_eq!(path.elements_slice(), expected.as_slice());
            let demand = CanvasPathRetirement::birth_demand();
            for denied in [RetainedCloneGrant { maximum_items: 0, ..policy }, RetainedCloneGrant { maximum_depth: 0, ..policy }, RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..policy }, RetainedCloneGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..policy }] {
                let (result, allocated, released) = observe_backing(|| CanvasPathRetirement::admit(path, denied));
                let (_, original) = result.err().unwrap();
                path = original;
                assert_eq!((allocated,released),(0,0));
                assert_eq!(path.elements_slice().as_ptr(),pointer);
                assert_eq!(path.elements_slice(),expected.as_slice());
            }
            let ((owner, birth), allocated, released) = observe_backing(|| CanvasPathRetirement::admit(path, policy).unwrap_or_else(|(error,_)| panic!("{error}")));
            assert_eq!((allocated,released),(birth.retained_capacity_bytes,0));
            assert_eq!(birth.retained_capacity_bytes,size_of::<CanvasPathRetirement>());
            let mut slot = Some(owner); let mut freed = 0; let mut born = original_birth + allocated;
            let original_length = slot.as_ref().unwrap().original_elements().len();
            if original_length != 0 {
                assert_eq!(example["elementCopyBytes"].as_u64().unwrap() as usize, size_of::<PathEl>());
                for denied in [RetainedCloneGrant { maximum_copy_bytes: 0, ..policy }, RetainedCloneGrant { maximum_copy_bytes: size_of::<PathEl>() - 1, ..policy }] {
                    let (step, allocated, released) = observe_backing(|| semio_framework_value::close_factory_ticket(&mut slot, denied).unwrap());
                    assert_eq!(step.progress(), RetainedCloneProgress::default(), "underfunded original typed element must remain retained");
                    assert_eq!((allocated, released), (0, 0));
                    let retained = slot.as_ref().unwrap();
                    assert_eq!(retained.original_elements().as_ptr(), pointer);
                    assert_eq!(retained.original_elements(), &expected[..original_length]);
                    assert_eq!(retained.original_capacity(), capacity);
                }
                assert_eq!(slot.as_ref().unwrap().next_copy_byte_demand().unwrap(), size_of::<PathEl>());
            }
            for _ in 0..cut.as_u64().unwrap() {
                if slot.as_ref().unwrap().original_elements().is_empty() { break; }
                let (step, allocated, released) = observe_backing(|| semio_framework_value::close_factory_ticket(&mut slot,policy).unwrap());
                assert!(step.progress().fits(policy)); assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
                freed += released; born += allocated;
            }
            assert_eq!(slot.as_ref().unwrap().original_elements().as_ptr(),pointer);
            assert_eq!(slot.as_ref().unwrap().original_capacity(),capacity);
            for _ in 0..expected.len()+4 {
                if slot.is_none() { break; }
                let owner = slot.as_ref().unwrap();
                if owner.terminal_is_empty() {
                    let shell_pointer = owner.as_ref() as *const CanvasPathRetirement;
                    for denied in [RetainedCloneGrant { maximum_release_bytes: size_of::<CanvasPathRetirement>() - 1, ..policy }, RetainedCloneGrant { maximum_copy_bytes: size_of::<Option<Box<CanvasPathRetirement>>>() - 1, ..policy }, RetainedCloneGrant { maximum_items: 0, ..policy }] {
                        let (step, allocated, released) = observe_backing(|| semio_framework_value::close_factory_ticket(&mut slot, denied).unwrap());
                        assert_eq!(step.progress(), Default::default());
                        assert_eq!((allocated, released), (0, 0));
                        assert_eq!(slot.as_ref().unwrap().as_ref() as *const CanvasPathRetirement, shell_pointer);
                    }
                }
                if slot.as_ref().unwrap().original_elements().is_empty() && !slot.as_ref().unwrap().terminal_is_empty() {
                    let extent = slot.as_ref().unwrap().next_release_byte_demand().unwrap();
                    for denied in [RetainedCloneGrant { maximum_release_bytes: extent - 1, ..policy },RetainedCloneGrant { maximum_copy_bytes: 0, ..policy }] {
                        let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,denied).unwrap());
                        assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(slot.as_ref().unwrap().original_elements().as_ptr(),pointer);
                    }
                }
                let original_element = !slot.as_ref().unwrap().original_elements().is_empty();
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,policy).unwrap());
                if original_element { assert_eq!(step.progress().copied_bytes, size_of::<PathEl>()); }
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));freed+=released;born+=allocated;
            }
            assert!(slot.is_none());assert_eq!(freed,born);
            let (_,allocated,released)=observe_backing(||drop(slot));assert_eq!((allocated,released),(0,0));
            println!("[DEBUG] Original Canvas path={} cancelledCut={} pointer/capacity/order/Kurbo=true exactSystemBirthFree={born} retainedEmptyCapacity=true terminalFrameSeparate=true terminalDrop0",row["id"],cut);
        }
    }
}
