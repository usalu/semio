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
                if owner.original_elements().is_empty() && !owner.terminal_is_empty() {
                    let extent = owner.next_release_byte_demand().unwrap();
                    for denied in [RetainedCloneGrant { maximum_release_bytes: extent - 1, ..policy },RetainedCloneGrant { maximum_copy_bytes: 0, ..policy }] {
                        let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,denied).unwrap());
                        assert_eq!(step.progress(),Default::default());assert_eq!((allocated,released),(0,0));assert_eq!(slot.as_ref().unwrap().original_elements().as_ptr(),pointer);
                    }
                }
                let (step,allocated,released)=observe_backing(||semio_framework_value::close_factory_ticket(&mut slot,policy).unwrap());
                assert!(step.progress().fits(policy));assert_eq!((allocated,released),(step.progress().retained_capacity_bytes,step.progress().released_bytes));freed+=released;born+=allocated;
            }
            assert!(slot.is_none());assert_eq!(freed,born);
            let (_,allocated,released)=observe_backing(||drop(slot));assert_eq!((allocated,released),(0,0));
            println!("[DEBUG] Original Canvas path={} cancelledCut={} pointer/capacity/order/Kurbo=true exactSystemBirthFree={born} retainedEmptyCapacity=true terminalFrameSeparate=true terminalDrop0",row["id"],cut);
        }
    }
}
