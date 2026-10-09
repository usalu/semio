//! 🧪️ Native arbitrary-depth traversal and partial cancellation conserve original paged owners.
use super::*;
use semio_framework_value::retained_clone::RetainedCloneSource;
use semio_framework_trace::observe_heap_allocations_on_this_thread;

#[derive(semio_framework_value::RetireOwned)]
enum NativeValue { Text(PagedUtf8<{usize::MAX}>), Array(PagedList<NativeValue,{usize::MAX}>), Object(PagedMap<NativeValue,{usize::MAX}>), Flag(bool), Number(u64) }
impl ArtifactCanonicalJsonTree for NativeValue {
    fn canonical_tree_node(&self) -> Result<Node<'_>,ValueError> {
        Ok(match self { Self::Text(text)=>Node::Text(ArtifactCanonicalJsonText::Native(text)),Self::Array(values)=>Node::Array(values.len()),Self::Object(values)=>Node::Object(values.len()),Self::Flag(value)=>Node::Bool(*value),Self::Number(value)=>Node::U64(*value) })
    }
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{
        match self {Self::Array(values)=>values.canonical_tree_child(ordinal),Self::Object(values)=>values.canonical_tree_child(ordinal),_=>Err(refusal("fixture scalar has no child"))}
    }
    fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{
        match self {Self::Object(values)=>values.canonical_tree_key(ordinal),_=>Err(refusal("fixture scalar has no key"))}
    }
}

fn fixture(text:&str,depth:usize)->NativeValue {
    let mut map=PagedMap::default();
    map.insert("a",NativeValue::Text(text.into()));
    map.insert("z",NativeValue::Array([NativeValue::Flag(true),NativeValue::Number(42)].into_iter().collect()));
    let mut value=NativeValue::Object(map);
    for _ in 0..depth {value=NativeValue::Array([value].into_iter().collect());}
    value
}
fn admitted(demand:RetirementDemand)->RetainedCloneGrant {
    assert!(demand.copy_bytes+demand.capacity_bytes<=4096);
    RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth}
}
fn observe_step(cursor:&mut ArtifactCanonicalJsonTreeCursor,output:&mut[u8])->ArtifactCanonicalJsonTreeStep {
    let(demand,pure)=observe_heap_allocations_on_this_thread(||cursor.next_demand().unwrap());
    assert_eq!((pure.requested_bytes,pure.released_bytes),(0,0));
    let grant=admitted(demand);
    let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(output,grant).unwrap());
    let progress=step.ownership.progress();
    assert!(progress.fits(grant));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
    if grant.maximum_release_bytes==0 {assert_eq!(heap.released_bytes,0);}
    assert!(step.written_bytes<=output.len().min(1));
    step
}

#[test]
fn canonical_native_paged_tree_arbitrary_depth_and_cancellation_conserve_actual_heap() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let text=rows["text"].as_str().unwrap();
    let leaf=serde_json::to_string(&serde_json::json!({"a":text,"z":[true,42]})).unwrap();
    for depth in rows["depths"].as_array().unwrap() {
        let depth=depth.as_u64().unwrap()as usize;
        let oracle=format!("{}{}{}","[".repeat(depth),leaf,"]".repeat(depth)).into_bytes();
        let mut pauses:Vec<Option<usize>>=rows["pauses"].as_array().unwrap().iter().map(|value|Some(value.as_u64().unwrap()as usize)).collect();
        pauses.push(None);
        for pause in pauses {
            let root=fixture(text,depth);
            let birth=RetainedCloneSource::<NativeValue>::owned_constructor_demand::<()>();
            assert!(birth.capacity_bytes<=4096);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};
            let((mut source,receipt),heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(root,(),grant).unwrap_or_else(|(error,_,_)|panic!("native source birth: {error}")));
            assert!(receipt.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));
            let(projection,heap)=observe_heap_allocations_on_this_thread(||source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();
            let((mut cursor,receipt),heap)=observe_heap_allocations_on_this_thread(||ArtifactCanonicalJsonTreeCursor::admit(projection,admitted(demand)).unwrap_or_else(|(error,_)|panic!("native traversal birth: {error}")));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert_eq!(receipt.copied_bytes,demand.copy_bytes);
            let(zero,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut[0],RetainedCloneGrant::default()).unwrap());
            assert_eq!(zero.ownership.progress(),RetainedCloneProgress::default());
            assert_eq!(zero.written_bytes,0);
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let mut output=Vec::new();
            let mut turns=0;
            while !cursor.terminal_is_empty() {
                assert!(turns<100_000,"native canonical traversal did not settle");
                if pause==Some(turns) {cursor.begin_close();}
                let mut byte=[0xa5];
                let step=observe_step(&mut cursor,&mut byte);
                output.extend_from_slice(&byte[..step.written_bytes]);
                turns+=1;
            }
            if pause.is_none(){assert!(cursor.is_complete());assert_eq!(output,oracle);}
            else {assert_eq!(output,oracle[..output.len()]);}
            let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            for turn in 0..100_000 {
                if source.terminal_is_empty(){break;}
                let copy=source.next_close_copy_byte_demand().unwrap();
                let grant=admitted(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
                let(step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(grant).unwrap());
                assert!(step.progress().fits(grant));
                assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
                assert!(turn<99_999||source.terminal_is_empty());
            }
            assert!(source.terminal_is_empty());
            let(_,heap)=observe_heap_allocations_on_this_thread(||drop(source));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            println!("[DEBUG] native canonical tree depth={depth} pause={pause:?} turns={turns}; exact Serde prefix, original projection aliases, 4096 copy/capacity, separate physical release, terminal drop zero heap");
        }
    }
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct DerivedMove {
 id:PagedUtf8<{usize::MAX}>,
 new_x:f64,
 new_y:f64,
 #[value(skip_serializing_if="Option::is_none")]
 #[serde(skip_serializing_if="Option::is_none")]
 visible:Option<bool>,
}
#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(tag="mutation",rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",tag="mutation",rename_all="camelCase")]
enum DerivedMutation {MoveNode(DerivedMove),Empty}

#[test]
fn canonical_native_derived_mutation_fields_match_serde_and_original_projection_ownership(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json")).unwrap();
 for visible in [None,Some(false),Some(true)]{
  for repeat in [1,1024]{
   let mut pauses:Vec<Option<usize>>=[0,1,3,17,129].into_iter().map(Some).collect();pauses.push(None);
   for pause in pauses {
    let mutation=DerivedMutation::MoveNode(DerivedMove{id:fixture["values"]["id"].as_str().unwrap().repeat(repeat).as_str().into(),new_x:fixture["values"]["newX"].as_f64().unwrap(),new_y:fixture["values"]["newY"].as_f64().unwrap(),visible});
    let oracle=serde_json::to_vec(&mutation).unwrap();if repeat==1&&visible.is_none(){assert_eq!(oracle,fixture["expected"].as_str().unwrap().as_bytes());}
    let birth=RetainedCloneSource::<DerivedMutation>::owned_constructor_demand::<()>();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth.capacity_bytes,maximum_depth:birth.depth,..Default::default()};
    let((mut source,receipt),heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(mutation,(),grant).unwrap_or_else(|(error,_,_)|panic!("derived native source: {error}")));
    assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));
    let(_,heap)=observe_heap_allocations_on_this_thread(||{
     let root=source.borrow();let DerivedMutation::MoveNode(payload)=root.get()else{unreachable!()};
     let child=root.get().canonical_tree_child(1).unwrap();assert_eq!(child as*const dyn ArtifactCanonicalJsonTree as*const (),&payload.id as*const PagedUtf8<{usize::MAX}> as*const ());
    });assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let projection=source.project_owned(0,|owner|owner as&dyn ArtifactCanonicalJsonTree);let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();
    let(mut cursor,receipt)=ArtifactCanonicalJsonTreeCursor::admit(projection,admitted(demand)).unwrap_or_else(|(error,_)|panic!("derived native tree: {error}"));assert_eq!(receipt.copied_bytes,demand.copy_bytes);
    let mut output=Vec::new();let mut turns=0;
    while !cursor.terminal_is_empty(){assert!(turns<100000);if pause==Some(turns){cursor.begin_close();}let mut byte=[0xa5];let step=observe_step(&mut cursor,&mut byte);output.extend_from_slice(&byte[..step.written_bytes]);turns+=1;}
    if pause.is_none(){assert_eq!(output,oracle);}else{assert_eq!(output,oracle[..output.len()]);}
    let(_,heap)=observe_heap_allocations_on_this_thread(||drop(cursor));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    for turn in 0..100000{
     if source.terminal_is_empty(){break;}
     let copy=source.next_close_copy_byte_demand().unwrap();let grant=admitted(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
     let(step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<99999||source.terminal_is_empty());
    }
    assert!(source.terminal_is_empty());let(_,heap)=observe_heap_allocations_on_this_thread(||drop(source));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    println!("[DEBUG] derived native mutation visible={visible:?} repeat={repeat} pause={pause:?} turns={turns}; actual original payload field projection, independent Serde bytes, 4096 perturn and exact source/alias closure");
   }
  }
 }
 let empty=DerivedMutation::Empty;assert_eq!(serde_json::to_vec(&empty).unwrap(),b"{\"mutation\":\"empty\"}");assert!(matches!(empty.canonical_tree_node().unwrap(),Node::Object(1)));assert!(matches!(empty.canonical_tree_child(0).unwrap().canonical_tree_node().unwrap(),Node::String("empty")));
}
