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
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let policy=&rows["retainedPolicy"];
    let grant=RetainedCloneGrant{maximum_items:policy["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:policy["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:policy["maximumDepth"].as_u64().unwrap()as usize};
    assert!(demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth);
    grant
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
            let grant=admitted(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});
            let((mut source,receipt),heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(root,(),grant).unwrap_or_else(|(error,_,_)|panic!("native source birth: {error}")));
            assert!(receipt.fits(grant));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));
            let projection_grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:source.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()};let(projection,heap)=observe_heap_allocations_on_this_thread(||source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,projection_grant).unwrap().0);
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();
            let original_grant=admitted(demand);
            let((mut cursor,receipt),heap)=observe_heap_allocations_on_this_thread(||ArtifactCanonicalJsonTreeCursor::admit(projection,original_grant).unwrap_or_else(|(error,_)|panic!("native traversal birth: {error}")));
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
    let grant=admitted(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});
    let((mut source,receipt),heap)=observe_heap_allocations_on_this_thread(||RetainedCloneSource::admit_owned(mutation,(),grant).unwrap_or_else(|(error,_,_)|panic!("derived native source: {error}")));
    assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,0));
    let(_,heap)=observe_heap_allocations_on_this_thread(||{
     let root=source.borrow();let DerivedMutation::MoveNode(payload)=root.get()else{unreachable!()};
     let child=root.get().canonical_tree_child(1).unwrap();assert_eq!(child as*const dyn ArtifactCanonicalJsonTree as*const (),&payload.id as*const PagedUtf8<{usize::MAX}> as*const ());
    });assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    let projection=source.project_owned(0,|owner|owner as&dyn ArtifactCanonicalJsonTree,admitted(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;let demand=ArtifactCanonicalJsonTreeCursor::constructor_demand();
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

#[test]
fn canonical_native_scalar_initializes_only_real_prefix_and_preserves_immutable_authority() {
 let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
 for row in rows["scalarCases"].as_array().unwrap(){
  let root=match row["value"].as_bool(){Some(flag)=>NativeValue::Flag(flag),None=>NativeValue::Number(row["value"].as_u64().unwrap())};
  let oracle=serde_json::to_vec(&row["value"]).unwrap();assert_eq!(oracle,row["expected"].as_str().unwrap().as_bytes());
  let birth=RetainedCloneSource::<NativeValue>::owned_constructor_demand::<()>();let policy=admitted(RetirementDemand{capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});
  let(mut source,_)=RetainedCloneSource::admit_owned(root,(),policy).unwrap_or_else(|(error,_,_)|panic!("scalar original source birth: {error}"));
  let projection=source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,admitted(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,policy).unwrap_or_else(|(error,_)|panic!("scalar original cursor admission: {error}"));
  let mut actual=Vec::new();let mut inspections=0;
  for turn in 0..10000{
   if cursor.terminal_is_empty(){break;}
   let inspect=cursor.frames.last().is_some_and(|frame|frame.phase==Phase::Inspect);
   if inspect{
    let denied=RetainedCloneGrant{maximum_copy_bytes:0,..policy};let mut output=[0xa5];
    let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut output,denied).unwrap());assert_eq!(step.ownership.progress(),RetainedCloneProgress::default());assert_eq!(step.written_bytes,0);assert_eq!(output,[0xa5]);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
   }
   let mut output=[0xa5];let(step,heap)=observe_heap_allocations_on_this_thread(||cursor.advance(&mut output,policy).unwrap());let progress=step.ownership.progress();assert!(progress.fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));
   if inspect{inspections+=1;assert_eq!(progress.copied_bytes,oracle.len());assert_eq!(step.written_bytes,0);assert_eq!(output,[0xa5]);}
   actual.extend_from_slice(&output[..step.written_bytes]);assert!(turn<9999||cursor.terminal_is_empty());
  }
  assert_eq!(inspections,1);assert_eq!(actual,oracle);drop(cursor);
  for turn in 0..10000{if source.terminal_is_empty(){break;}let(step,heap)=observe_heap_allocations_on_this_thread(||source.close_step(policy).unwrap());assert!(step.progress().fits(policy));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(turn<9999||source.terminal_is_empty());}assert!(source.terminal_is_empty());drop(source);
  println!("[DEBUG] original canonical scalar prefix={} initialized once; independent Serde, denied original copy, exact System births/releases and fixed plain grant",oracle.len());
 }
}

#[derive(semio_framework_value::RetireOwned)]
struct Leaves{pair:(i32,String),triple:(u8,bool,Vec<(u16,f32)>),dsl:semio_framework_value::DslValue,f32_:f32,i128_:i128,i16_:i16,i8_:i8,isize_:isize,number:semio_framework_value::Number,u128_:u128,u16_:u16,u32_:u32,unit:(),value_type:semio_framework_value::ValueType}
impl ArtifactCanonicalJsonTree for Leaves {
    fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(14))}
    fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn ArtifactCanonicalJsonTree,ValueError>{Ok(match ordinal{0=>&self.dsl as&dyn ArtifactCanonicalJsonTree,1=>&self.f32_,2=>&self.i128_,3=>&self.i16_,4=>&self.i8_,5=>&self.isize_,6=>&self.number,7=>&self.pair,8=>&self.triple,9=>&self.u128_,10=>&self.u16_,11=>&self.u32_,12=>&self.unit,13=>&self.value_type,_=>return Err(refusal("leaf fixture ordinal is absent"))})}
    fn canonical_tree_key(&self,ordinal:usize)->Result<ArtifactCanonicalJsonText<'_>,ValueError>{["dsl","f32","i128","i16","i8","isize","number","pair","triple","u128","u16","u32","unit","value_type"].get(ordinal).map(|key|ArtifactCanonicalJsonText::from(*key)).ok_or_else(||refusal("leaf fixture key is absent"))}
}

#[test]
fn canonical_native_leaf_impls_match_independent_serde_json_output() {
    use semio_framework_value::{DslValue,Number};
    let dsl=DslValue::Object(vec![("a".into(),DslValue::Array(vec![DslValue::Null,DslValue::Bool(true),DslValue::Number(Number::UInt(7)),DslValue::Number(Number::Int(-3)),DslValue::Number(Number::Float(1.5)),DslValue::String("é🧬\"x\n".into()),DslValue::Bytes(vec![1,2,255])])),("b".into(),DslValue::Object(vec![("c".into(),DslValue::Null)]))]);
    let leaves=Leaves{pair:(-4,"p\"q".into()),triple:(9,true,vec![(1,0.5),(2,0.1)]),dsl:dsl.clone(),f32_:0.1,i128_:-(1i128<<100),i16_:-300,i8_:-7,isize_:-9,number:Number::Float(-0.25),u128_:(1u128<<100)+5,u16_:65535,u32_:4_000_000_000,unit:(),value_type:semio_framework_value::ValueType::List(Box::new(semio_framework_value::ValueType::Schema("row".into())))};
    let oracle=format!("{{\"dsl\":{},\"f32\":{},\"i128\":{},\"i16\":{},\"i8\":{},\"isize\":{},\"number\":{},\"pair\":{},\"triple\":{},\"u128\":{},\"u16\":{},\"u32\":{},\"unit\":{},\"value_type\":{}}}",serde_json::to_string(&serde_json::Value::from(&dsl)).unwrap(),serde_json::to_string(&0.1f32).unwrap(),serde_json::to_string(&leaves.i128_).unwrap(),serde_json::to_string(&leaves.i16_).unwrap(),serde_json::to_string(&leaves.i8_).unwrap(),serde_json::to_string(&leaves.isize_).unwrap(),serde_json::to_string(&-0.25f64).unwrap(),serde_json::to_string(&leaves.pair).unwrap(),serde_json::to_string(&leaves.triple).unwrap(),serde_json::to_string(&leaves.u128_).unwrap(),serde_json::to_string(&leaves.u16_).unwrap(),serde_json::to_string(&leaves.u32_).unwrap(),serde_json::to_string(&()).unwrap(),serde_json::to_string(&serde_json::Value::from(semio_framework_value::ToValue::to_value(&leaves.value_type))).unwrap());
    let birth=RetainedCloneSource::<Leaves>::owned_constructor_demand::<()>();
    let grant=admitted(RetirementDemand{copy_bytes:RetainedCloneSource::<Leaves>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});
    let(mut source,_)=RetainedCloneSource::admit_owned(leaves,(),grant).unwrap_or_else(|(error,_,_)|panic!("leaf fixture source birth: {error}"));
    let projection=source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,admitted(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;
    let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,admitted(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("leaf fixture cursor admission: {error}"));
    let mut output=Vec::new();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty(){break;}
        assert!(turn<999_999,"leaf canonical traversal did not settle");
        let mut byte=[0xa5];let step=observe_step(&mut cursor,&mut byte);output.extend_from_slice(&byte[..step.written_bytes]);
    }
    assert!(cursor.is_complete());drop(cursor);
    assert_eq!(String::from_utf8(output).unwrap(),oracle);
    for _ in 0..100_000 {
        if source.terminal_is_empty(){break;}
        let copy=source.next_close_copy_byte_demand().unwrap();
        let grant=admitted(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
        assert!(source.close_step(grant).unwrap().progress().fits(grant));
    }
    assert!(source.terminal_is_empty());
    println!("[DEBUG] canonical leaf impls u16/u32/u128/i8/i16/i128/isize/f32/unit/Number/DslValue equal serde_json: {oracle}");
}

fn canonical_bytes<T:ArtifactCanonicalJsonTree+semio_framework_value::retirement::RetireOwned>(root:T)->Vec<u8>{
    let birth=RetainedCloneSource::<T>::owned_constructor_demand::<()>();
    let grant=admitted(RetirementDemand{copy_bytes:RetainedCloneSource::<T>::constructor_copy_bytes(),capacity_bytes:birth.capacity_bytes,depth:birth.depth,..Default::default()});
    let(mut source,_)=RetainedCloneSource::admit_owned(root,(),grant).unwrap_or_else(|(error,_,_)|panic!("map fixture source birth: {error}"));
    let projection=source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,admitted(RetirementDemand{copy_bytes:source.borrow().binding_copy_bytes(),depth:1,..Default::default()})).unwrap().0;
    let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,admitted(ArtifactCanonicalJsonTreeCursor::constructor_demand())).unwrap_or_else(|(error,_)|panic!("map fixture cursor admission: {error}"));
    let mut output=Vec::new();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty(){break;}
        assert!(turn<999_999,"map canonical traversal did not settle");
        let mut byte=[0xa5];let step=observe_step(&mut cursor,&mut byte);output.extend_from_slice(&byte[..step.written_bytes]);
    }
    assert!(cursor.is_complete());drop(cursor);
    for _ in 0..100_000 {
        if source.terminal_is_empty(){break;}
        let copy=source.next_close_copy_byte_demand().unwrap();
        let grant=admitted(RetirementDemand{copy_bytes:copy,capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});
        assert!(source.close_step(grant).unwrap().progress().fits(grant));
    }
    assert!(source.terminal_is_empty());
    output
}

#[test]
fn canonical_native_ordered_and_hashed_maps_match_independent_serde_json_output() {
    let names=["zeta","alpha","é","Beta","","a\"q","mid dle","α","10","9"];
    let ordered:std::collections::BTreeMap<String,f64>=names.iter().enumerate().map(|(index,name)|(name.to_string(),index as f64*0.5-1.0)).collect();
    let oracle=serde_json::to_string(&ordered).unwrap();
    assert_eq!(String::from_utf8(canonical_bytes(ordered)).unwrap(),oracle);
    let hashed:std::collections::HashMap<String,Vec<u8>>=names.iter().enumerate().map(|(index,name)|(name.to_string(),vec![index as u8;index%3])).collect();
    let sorted:std::collections::BTreeMap<&String,&Vec<u8>>=hashed.iter().collect();
    let oracle=serde_json::to_string(&sorted).unwrap();
    let actual=String::from_utf8(canonical_bytes(hashed.clone())).unwrap();
    assert_eq!(actual,oracle);
    assert_eq!(serde_json::from_str::<serde_json::Value>(&actual).unwrap(),serde_json::to_value(&hashed).unwrap());
    assert_eq!(String::from_utf8(canonical_bytes(std::collections::BTreeMap::<String,f64>::new())).unwrap(),"{}");
    println!("[DEBUG] canonical BTreeMap/HashMap objects equal serde_json (sorted keys): {actual}");
}

#[test]
fn canonical_native_ordered_roots_match_serde_json_and_their_to_value_wire() {
    use semio_framework_value::{ToValue, ordered::{OrderedMap, OrderedSet}};
    let names=["zeta","alpha","é","Beta","","a\"q","mid dle","α","10","9"];
    let mut ordered=OrderedMap::<f64>::new();
    for(index,name)in names.iter().enumerate(){assert!(ordered.insert(name.to_string(),index as f64*0.5-1.0).is_none());}
    let oracle:std::collections::BTreeMap<String,f64>=names.iter().enumerate().map(|(index,name)|(name.to_string(),index as f64*0.5-1.0)).collect();
    let expected=serde_json::to_string(&oracle).unwrap();
    let wire=serde_json::to_string(&serde_json::Value::from(&ordered.to_value())).unwrap();
    assert_eq!(wire,expected);
    assert_eq!(String::from_utf8(canonical_bytes(ordered)).unwrap(),expected);
    let set:OrderedSet=names.iter().map(|name|name.to_string()).collect();
    let expected=serde_json::to_string(&oracle.keys().collect::<Vec<_>>()).unwrap();
    assert_eq!(serde_json::to_string(&serde_json::Value::from(&set.to_value())).unwrap(),expected);
    assert_eq!(String::from_utf8(canonical_bytes(set)).unwrap(),expected);
    assert_eq!(String::from_utf8(canonical_bytes(OrderedMap::<f64>::new())).unwrap(),"{}");
    assert_eq!(String::from_utf8(canonical_bytes(OrderedSet::new())).unwrap(),"[]");
    println!("[DEBUG] canonical OrderedMap/OrderedSet equal serde_json and ToValue wire: {expected}");
}

#[test]
fn canonical_native_decimal_u64_text_matches_serde_json_string_output() {
    use crate::ArtifactCanonicalDecimalU64;
    for value in [0u64, 1, 9, 10, (1 << 53) + 1, u64::MAX] {
        let expected = serde_json::to_string(&serde_json::Value::String(value.to_string())).unwrap();
        assert_eq!(String::from_utf8(canonical_bytes(ArtifactCanonicalDecimalU64(value))).unwrap(), expected);
        assert_eq!(expected, format!("\"{value}\""));
    }
    let list = vec![ArtifactCanonicalDecimalU64(7), ArtifactCanonicalDecimalU64(u64::MAX)];
    assert_eq!(String::from_utf8(canonical_bytes(list)).unwrap(), "[\"7\",\"18446744073709551615\"]");
    use semio_framework_value::{FromValue, ToValue};
    let wire = ArtifactCanonicalDecimalU64(u64::MAX).to_value();
    assert_eq!(ArtifactCanonicalDecimalU64::from_value(wire).unwrap(), ArtifactCanonicalDecimalU64(u64::MAX));
    assert!(ArtifactCanonicalDecimalU64::from_value(semio_framework_value::DslValue::String("007".into())).is_err());
    assert!(ArtifactCanonicalDecimalU64::from_value(semio_framework_value::DslValue::String("18446744073709551616".into())).is_err());
    println!("[DEBUG] canonical decimal u64 text equals serde_json string output");
}

fn is_zero_byte(value:&u8)->bool{*value==0}
fn is_zero(value:&u64)->bool{*value==0}
fn decimal_to_value(value:&u64)->semio_framework_value::DslValue{semio_framework_value::DslValue::String(value.to_string())}
fn decimal_to_serde<S:semio_framework_value::serde::Serializer>(value:&u64,serializer:S)->Result<S::Ok,S::Error>{serializer.collect_str(value)}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
enum DerivedQuote {Double,Single}
impl DerivedQuote {fn is_double(&self)->bool{matches!(self,Self::Double)}}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct DerivedRoles {
 #[value(skip_serializing_if="is_zero_byte")]
 #[serde(skip_serializing_if="is_zero_byte")]
 pad_byte:u8,
 #[value(skip_serializing_if="DerivedQuote::is_double")]
 #[serde(skip_serializing_if="DerivedQuote::is_double")]
 quote:DerivedQuote,
 #[value(skip_serializing_if="is_zero",serialize_with="decimal_to_value")]
 #[canonical_json(decimal_string)]
 #[serde(skip_serializing_if="is_zero",serialize_with="decimal_to_serde")]
 position:u64,
 #[value(with="semio_framework_value::bytes",serialize_controlled_with="semio_framework_value::bytes::to_value_controlled")]
 octets:Vec<u8>,
 #[value(with="semio_framework_value::bytes::optional",serialize_controlled_with="semio_framework_value::bytes::optional::to_value_controlled",skip_serializing_if="Option::is_none")]
 #[serde(skip_serializing_if="Option::is_none")]
 tail:Option<Vec<u8>>,
 #[value(with="semio_framework_value::bytes::optional")]
 maybe:Option<Vec<u8>>,
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
#[serde(crate="semio_framework_value::serde",tag="kind",rename_all="camelCase",rename_all_fields="camelCase")]
enum DerivedMarker {
 Mark {
  #[value(skip_serializing_if="is_zero",serialize_with="decimal_to_value")]
  #[canonical_json(decimal_string)]
  #[serde(skip_serializing_if="is_zero",serialize_with="decimal_to_serde")]
  prolog_position:u64,
  #[value(with="pack::value::bytes",serialize_controlled_with="pack::value::bytes::to_value_controlled")]
  payload:Vec<u8>,
  #[value(skip_serializing_if="is_zero_byte")]
  #[serde(skip_serializing_if="is_zero_byte")]
  pad_byte:u8,
 },
 Bare,
}

#[test]
fn canonical_native_field_roles_match_serde_json_and_to_value_wire() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json")).unwrap();
    let octets=|value:&serde_json::Value|->Vec<u8>{value.as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()};
    let optional=|value:&serde_json::Value|(!value.is_null()).then(||octets(value));
    for row in rows["roles"].as_array().unwrap() {
        let roles=DerivedRoles{
            pad_byte:row["padByte"].as_u64().unwrap()as u8,
            quote:if row["quote"]=="double"{DerivedQuote::Double}else{DerivedQuote::Single},
            position:row["position"].as_str().unwrap().parse().unwrap(),
            octets:octets(&row["octets"]),
            tail:optional(&row["tail"]),
            maybe:optional(&row["maybe"]),
        };
        let expected=row["expected"].as_str().unwrap();
        assert_eq!(serde_json::to_string(&roles).unwrap(),expected);
        use semio_framework_value::ToValue;
        assert_eq!(serde_json::Value::from(&roles.to_value()),serde_json::from_str::<serde_json::Value>(expected).unwrap());
        assert_eq!(String::from_utf8(canonical_bytes(roles)).unwrap(),expected);
        println!("[DEBUG] derived field roles equal serde_json and ToValue wire: {expected}");
    }
}

#[test]
fn canonical_native_field_roles_project_original_fields_in_place() {
    let roles=DerivedRoles{pad_byte:3,quote:DerivedQuote::Single,position:u64::MAX,octets:vec![1,2],tail:Some(vec![3]),maybe:None};
    assert!(matches!(roles.canonical_tree_node().unwrap(),Node::Object(6)));
    let child=roles.canonical_tree_child(2).unwrap();
    assert_eq!(child as*const dyn ArtifactCanonicalJsonTree as*const (),&roles.position as*const u64 as*const ());
    assert!(matches!(child.canonical_tree_node().unwrap(),Node::U64Text(u64::MAX)));
    assert_eq!(roles.canonical_tree_child(3).unwrap() as*const dyn ArtifactCanonicalJsonTree as*const (),&roles.octets as*const Vec<u8> as*const ());
    let skipped=DerivedRoles{pad_byte:0,quote:DerivedQuote::Double,position:0,octets:Vec::new(),tail:None,maybe:None};
    assert!(matches!(skipped.canonical_tree_node().unwrap(),Node::Object(2)));
    assert!(matches!(skipped.canonical_tree_key(0).unwrap(),ArtifactCanonicalJsonText::Contiguous("octets")));
}

#[test]
fn canonical_native_variant_field_roles_match_serde_json() {
    for(position,payload,pad_byte)in [(0u64,vec![],0u8),(1,vec![0,255],9),(u64::MAX,vec![7],0),((1u64<<53)+1,vec![],200)] {
        let marker=DerivedMarker::Mark{prolog_position:position,payload,pad_byte};
        let oracle=serde_json::to_string(&marker).unwrap();
        assert_eq!(String::from_utf8(canonical_bytes(marker)).unwrap(),oracle);
        println!("[DEBUG] derived variant field roles equal serde_json: {oracle}");
    }
    assert_eq!(String::from_utf8(canonical_bytes(DerivedMarker::Bare)).unwrap(),serde_json::to_string(&DerivedMarker::Bare).unwrap());
}

fn word_text(value:&f32)->String{format!("{:08x}",value.to_bits())}
fn serde_word<S:semio_framework_value::serde::Serializer>(value:&f32,serializer:S)->Result<S::Ok,S::Error>{serializer.serialize_str(&word_text(value))}
fn serde_array<const N:usize,S:semio_framework_value::serde::Serializer>(value:&[f32;N],serializer:S)->Result<S::Ok,S::Error>{serializer.collect_seq(value.iter().map(word_text))}
fn serde_list<S:semio_framework_value::serde::Serializer>(value:&Vec<f32>,serializer:S)->Result<S::Ok,S::Error>{serializer.collect_seq(value.iter().map(word_text))}
fn serde_optional_array<const N:usize,S:semio_framework_value::serde::Serializer>(value:&Option<[f32;N]>,serializer:S)->Result<S::Ok,S::Error>{match value{Some(value)=>serde_array(value,serializer),None=>serializer.serialize_none()}}
fn serde_optional_word<S:semio_framework_value::serde::Serializer>(value:&Option<f32>,serializer:S)->Result<S::Ok,S::Error>{match value{Some(value)=>serde_word(value,serializer),None=>serializer.serialize_none()}}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct DerivedWords {
 #[canonical_json(hex_word)]
 #[serde(serialize_with="serde_array")]
 position:[f32;3],
 #[canonical_json(hex_word)]
 #[value(skip_serializing_if="Option::is_none")]
 #[serde(skip_serializing_if="Option::is_none",serialize_with="serde_optional_array")]
 normal:Option<[f32;3]>,
 halfedge:Option<u32>,
 #[canonical_json(hex_word)]
 #[serde(serialize_with="serde_array")]
 uv:[f32;2],
 #[canonical_json(hex_word)]
 #[serde(serialize_with="serde_word")]
 weight:f32,
 #[canonical_json(hex_word)]
 #[value(skip_serializing_if="Vec::is_empty")]
 #[serde(skip_serializing_if="Vec::is_empty",serialize_with="serde_list")]
 samples:Vec<f32>,
 #[canonical_json(hex_word)]
 #[serde(serialize_with="serde_optional_word")]
 scale:Option<f32>,
}

#[test]
fn canonical_native_hex_words_match_the_managed_mesh_word_wire_and_serde_json() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json")).unwrap();
    let floats=|value:&serde_json::Value|->Vec<f32>{value.as_array().unwrap().iter().map(|value|value.as_f64().unwrap()as f32).collect()};
    for row in rows["hexWords"].as_array().unwrap() {
        for scale in [None,Some(f32::from_bits(0x7fc0_0001)),Some(f32::NEG_INFINITY),Some(-0.0)] {
            let words=DerivedWords{
                position:floats(&row["position"]).try_into().unwrap(),
                normal:(!row["normal"].is_null()).then(||floats(&row["normal"]).try_into().unwrap()),
                halfedge:row["halfedge"].as_u64().map(|value|value as u32),
                uv:floats(&row["uv"]).try_into().unwrap(),
                weight:row["weight"].as_f64().unwrap()as f32,
                samples:floats(&row["samples"]),
                scale,
            };
            let mut expected=row["expected"].as_str().unwrap().to_string();
            expected.pop();
            expected.push_str(&format!(",\"scale\":{}}}",scale.map_or("null".into(),|value|format!("\"{}\"",word_text(&value)))));
            assert_eq!(serde_json::to_string(&words).unwrap(),expected);
            assert_eq!(String::from_utf8(canonical_bytes(words)).unwrap(),expected);
            println!("[DEBUG] derived hex words equal serde_json and the managed mesh word wire: {expected}");
        }
    }
}

#[test]
fn canonical_native_hex_words_project_original_fields_in_place() {
    let words=DerivedWords{position:[1.5,-0.0,f32::INFINITY],normal:None,halfedge:Some(2),uv:[0.0,1.0],weight:2.0,samples:vec![0.5],scale:None};
    assert!(matches!(words.canonical_tree_node().unwrap(),Node::Object(6)));
    let position=words.canonical_tree_child(0).unwrap();
    assert_eq!(position as*const dyn ArtifactCanonicalJsonTree as*const (),&words.position as*const [f32;3] as*const ());
    assert!(matches!(position.canonical_tree_node().unwrap(),Node::Array(3)));
    let element=position.canonical_tree_child(2).unwrap();
    assert_eq!(element as*const dyn ArtifactCanonicalJsonTree as*const (),&words.position[2] as*const f32 as*const ());
    assert!(matches!(element.canonical_tree_node().unwrap(),Node::F32HexWord(value) if value.to_bits()==0x7f80_0000));
    assert!(position.canonical_tree_child(3).is_err());
    assert!(matches!(words.canonical_tree_child(5).unwrap().canonical_tree_node().unwrap(),Node::Null));
    use crate::ArtifactCanonicalHexWordF32;
    use semio_framework_value::{FromValue,ToValue};
    for bits in [0u32,1,0x3f80_0000,0x7fc0_0001,0xffff_ffff] {
        let value=ArtifactCanonicalHexWordF32(f32::from_bits(bits));
        assert_eq!(ArtifactCanonicalHexWordF32::from_value(value.to_value()).unwrap().0.to_bits(),bits);
    }
    for text in ["3F800000","3f80000","3f8000000","+f800000",""] {assert!(ArtifactCanonicalHexWordF32::from_value(semio_framework_value::DslValue::String(text.into())).is_err(),"{text}");}
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[serde(crate="semio_framework_value::serde")]
struct TreeInner {count:u32,label:String}
fn inner_value(inner:&TreeInner)->semio_framework_value::DslValue{
    use semio_framework_value::{DslValue,Number};
    DslValue::Object(vec![("count".into(),DslValue::Number(Number::UInt(u64::from(inner.count)))),("label".into(),DslValue::String(inner.label.clone()))])
}
mod inner_json {
    pub fn to_value(inner:&super::TreeInner)->semio_framework_value::DslValue{super::inner_value(inner)}
    pub fn to_value_controlled(inner:&super::TreeInner,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{control.step()?;Ok(super::inner_value(inner))}
    pub mod optional {
        pub fn to_value(inner:&Option<super::super::TreeInner>)->semio_framework_value::DslValue{inner.as_ref().map_or(semio_framework_value::DslValue::Null,super::to_value)}
        pub fn to_value_controlled(inner:&Option<super::super::TreeInner>,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_value::DslValue,semio_framework_value::ValueError>{match inner{Some(inner)=>super::to_value_controlled(inner,control),None=>{control.step()?;Ok(semio_framework_value::DslValue::Null)}}}
    }
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct DerivedTreeRoles {
 #[value(with="inner_json",serialize_controlled_with="inner_json::to_value_controlled")]
 #[canonical_json(tree)]
 inner:TreeInner,
 #[value(with="inner_json::optional",serialize_controlled_with="inner_json::optional::to_value_controlled")]
 #[canonical_json(tree)]
 maybe_inner:Option<TreeInner>,
}

#[test]
fn canonical_native_tree_role_matches_the_custom_value_serializer_and_serde_json() {
    use semio_framework_value::ToValue;
    for maybe_inner in [None,Some(TreeInner{count:7,label:"é\"🧩".into()})] {
        let roles=DerivedTreeRoles{inner:TreeInner{count:u32::MAX,label:"a".into()},maybe_inner};
        let oracle=serde_json::to_string(&roles).unwrap();
        assert_eq!(serde_json::Value::from(&roles.to_value()),serde_json::from_str::<serde_json::Value>(&oracle).unwrap());
        let child=roles.canonical_tree_child(0).unwrap();
        assert_eq!(child as*const dyn ArtifactCanonicalJsonTree as*const (),&roles.inner as*const TreeInner as*const ());
        assert_eq!(String::from_utf8(canonical_bytes(roles)).unwrap(),oracle);
        println!("[DEBUG] derived tree role equals serde_json and the custom serializer wire: {oracle}");
    }
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct FlatHeader {
 id:String,
 #[value(skip_serializing_if="Option::is_none")]
 #[serde(skip_serializing_if="Option::is_none")]
 note:Option<String>,
}
#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct FlatNested {
 #[value(flatten)]
 #[serde(flatten)]
 header:FlatHeader,
 level:u32,
}
#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct FlatExtra {extra_count:u8}
#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct FlatRoot {
 name:String,
 #[value(flatten)]
 #[serde(flatten)]
 nested:FlatNested,
 #[value(flatten)]
 #[serde(flatten)]
 extra:Option<FlatExtra>,
 #[value(skip_serializing_if="Option::is_none")]
 #[serde(skip_serializing_if="Option::is_none")]
 tail:Option<u32>,
}

#[test]
fn canonical_native_flatten_splices_at_field_position_like_serde_json_and_to_value() {
    use semio_framework_value::ToValue;
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json")).unwrap();
    for row in rows["flatten"].as_array().unwrap() {
        let root=FlatRoot{
            name:row["name"].as_str().unwrap().into(),
            nested:FlatNested{header:FlatHeader{id:row["id"].as_str().unwrap().into(),note:row["note"].as_str().map(Into::into)},level:row["level"].as_u64().unwrap()as u32},
            extra:row["extraCount"].as_u64().map(|extra_count|FlatExtra{extra_count:extra_count as u8}),
            tail:row["tail"].as_u64().map(|tail|tail as u32),
        };
        let expected=row["expected"].as_str().unwrap();
        assert_eq!(serde_json::to_string(&root).unwrap(),expected);
        let wire=root.to_value();
        let semio_framework_value::DslValue::Object(entries)=&wire else{panic!("object")};
        assert_eq!(serde_json::Value::from(&wire),serde_json::from_str::<serde_json::Value>(expected).unwrap());
        let node_length=match root.canonical_tree_node().unwrap(){Node::Object(length)=>length,_=>panic!("object")};
        assert_eq!(node_length,entries.len());
        assert!(root.canonical_tree_child(node_length).is_err()&&root.canonical_tree_key(node_length).is_err());
        let keys:Vec<String>=(0..node_length).map(|ordinal|match root.canonical_tree_key(ordinal).unwrap(){ArtifactCanonicalJsonText::Contiguous(key)=>key.to_string(),_=>panic!("contiguous key")}).collect();
        assert_eq!(keys,entries.iter().map(|(key,_)|key.clone()).collect::<Vec<_>>());
        let first_header=root.canonical_tree_child(1).unwrap();
        assert_eq!(first_header as*const dyn ArtifactCanonicalJsonTree as*const (),&root.nested.header.id as*const String as*const ());
        assert_eq!(String::from_utf8(canonical_bytes(root)).unwrap(),expected);
        println!("[DEBUG] derived flatten equals serde_json, ToValue member order and member count: {expected}");
    }
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase",rename_all_fields="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase",rename_all_fields="camelCase")]
enum ExternalEdge {Solid,Dashed{on_length:f64,off_length:f64}}
#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::ToValue,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase",rename_all_fields="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase",rename_all_fields="camelCase")]
enum ExternalShape {
 Point,
 Circle(f64),
 Rect {
  width:f64,
  height:f64,
  #[value(skip_serializing_if="Option::is_none")]
  #[serde(skip_serializing_if="Option::is_none")]
  label:Option<String>,
 },
 Frame {outer_edge:ExternalEdge,count:u32},
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase",rename_all_fields="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase",rename_all_fields="camelCase")]
enum ExternalRich {
 Placed {
  #[value(skip_serializing_if="is_zero",serialize_with="decimal_to_value")]
  #[canonical_json(decimal_string)]
  #[serde(skip_serializing_if="is_zero",serialize_with="decimal_to_serde")]
  serial_number:u64,
  #[value(flatten)]
  #[serde(flatten)]
  header:FlatHeader,
  #[canonical_json(hex_word)]
  #[serde(serialize_with="serde_word")]
  weight:f32,
  #[value(with="pack::value::bytes")]
  payload:Vec<u8>,
  #[value(skip)]
  #[serde(skip)]
  scratch:u8,
 },
 Hollow {},
 Bare,
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[serde(crate="semio_framework_value::serde")]
enum ExternalGeneric<T> {Wrap{inner:T,again:Option<T>},Empty}

#[test]
fn canonical_native_externally_tagged_named_variants_match_serde_json_and_to_value() {
    use semio_framework_value::ToValue;
    let rows:serde_json::Value=serde_json::from_str(include_str!("../../../../../🌱️value/✨️derive/🧵️canonical/🧫️fixtures/🔣️.json")).unwrap();
    for row in rows["externalVariants"].as_array().unwrap() {
        let number=|key:&str|row[key].as_f64().unwrap();
        let shape=match row["variant"].as_str().unwrap() {
            "point"=>ExternalShape::Point,
            "circle"=>ExternalShape::Circle(number("radius")),
            "rect"=>ExternalShape::Rect{width:number("width"),height:number("height"),label:row["label"].as_str().map(Into::into)},
            _=>ExternalShape::Frame{outer_edge:if row["edge"]=="solid"{ExternalEdge::Solid}else{ExternalEdge::Dashed{on_length:number("onLength"),off_length:number("offLength")}},count:row["count"].as_u64().unwrap()as u32},
        };
        let expected=row["expected"].as_str().unwrap();
        assert_eq!(serde_json::to_string(&shape).unwrap(),expected);
        assert_eq!(serde_json::Value::from(&shape.to_value()),serde_json::from_str::<serde_json::Value>(expected).unwrap());
        assert_eq!(String::from_utf8(canonical_bytes(shape)).unwrap(),expected);
        println!("[DEBUG] derived externally tagged variant equals serde_json and ToValue: {expected}");
    }
}

#[test]
fn canonical_native_externally_tagged_variants_support_every_field_role_and_generics() {
    for(serial_number,note)in [(0u64,None),(u64::MAX,Some("n")),((1u64<<53)+1,None)] {
        let rich=ExternalRich::Placed{serial_number,header:FlatHeader{id:"h".into(),note:note.map(Into::into)},weight:f32::from_bits(0x3fc0_0000),payload:vec![0,255],scratch:9};
        let oracle=serde_json::to_string(&rich).unwrap();
        assert_eq!(String::from_utf8(canonical_bytes(rich)).unwrap(),oracle);
        println!("[DEBUG] derived externally tagged rich variant equals serde_json: {oracle}");
    }
    for rich in [ExternalRich::Hollow{},ExternalRich::Bare] {
        let oracle=serde_json::to_string(&rich).unwrap();
        assert_eq!(String::from_utf8(canonical_bytes(rich)).unwrap(),oracle);
    }
    for generic in [ExternalGeneric::Wrap{inner:7u32,again:None},ExternalGeneric::Wrap{inner:u32::MAX,again:Some(1)},ExternalGeneric::Empty] {
        let oracle=serde_json::to_string(&generic).unwrap();
        assert_eq!(String::from_utf8(canonical_bytes(generic)).unwrap(),oracle);
    }
    let shape=ExternalShape::Rect{width:1.5,height:2.5,label:None};
    assert!(matches!(shape.canonical_tree_node().unwrap(),Node::Object(1)));
    assert!(matches!(shape.canonical_tree_key(0).unwrap(),ArtifactCanonicalJsonText::Contiguous("rect")));
    assert!(shape.canonical_tree_key(1).is_err()&&shape.canonical_tree_child(1).is_err());
    let view=shape.canonical_tree_child(0).unwrap();
    assert_eq!(view as*const dyn ArtifactCanonicalJsonTree as*const (),&shape as*const ExternalShape as*const ());
    assert!(matches!(view.canonical_tree_node().unwrap(),Node::Object(2)));
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct BulkTexel { red:u32, green:u32, blue:u32, alpha:u32, reserved:u32 }

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned,semio_framework_value::serde::Serialize)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
#[serde(crate="semio_framework_value::serde",rename_all="camelCase")]
struct BulkDocument {
 name:String,
 samples:Vec<u16>,
 octets:Vec<u8>,
 texels:Vec<BulkTexel>,
 tags:Vec<String>,
 maybe:Option<Vec<u16>>,
 nested:Vec<Vec<u16>>,
 empty:Vec<u16>,
 pair:(u8,bool),
}

#[derive(semio_framework_value::CanonicalJsonTree,semio_framework_value::RetireOwned)]
#[canonical_json(owner=crate)]
#[value(rename_all="camelCase")]
struct BulkFloats { weights:[f32;4], list:Vec<f32>, wide:Vec<i64>, large:Vec<u64>, flags:Vec<bool>, nothing:Vec<()> }

fn bulk_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:1<<16,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:64}}

fn encode_with_window<R:ArtifactCanonicalJsonTree+semio_framework_value::retirement::RetireOwned>(root:R,window:usize)->(Vec<u8>,usize) {
 let grant=bulk_grant();
 let(mut source,_)=RetainedCloneSource::admit_owned(root,(),grant).unwrap_or_else(|(error,_,_)|panic!("bulk source birth: {error}"));
 let projection=source.project_owned(0,|value|value as&dyn ArtifactCanonicalJsonTree,grant).unwrap().0;
 let(mut cursor,_)=ArtifactCanonicalJsonTreeCursor::admit(projection,grant).unwrap_or_else(|(error,_)|panic!("bulk traversal birth: {error}"));
 let(mut output,mut turns,mut buffer)=(Vec::new(),0usize,vec![0u8;window]);
 while !cursor.terminal_is_empty(){
  assert!(turns<20_000_000,"bulk canonical traversal did not settle");
  let step=cursor.advance(&mut buffer,grant).unwrap();
  assert!(step.ownership.progress().fits(grant));
  assert!(step.written_bytes<=window);
  if window>=FLAT_MINIMUM_BYTES&&step.written_bytes>1{assert_eq!(step.ownership.progress().copied_bytes,step.written_bytes,"a bulk turn charges exactly the bytes it wrote");}
  output.extend_from_slice(&buffer[..step.written_bytes]);
  turns+=1;
 }
 drop(cursor);
 for _ in 0..1_000_000{
  if source.terminal_is_empty(){break;}
  let copy=source.next_close_copy_byte_demand().unwrap();
  let close=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:source.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:source.next_close_release_byte_demand().unwrap(),maximum_depth:source.next_close_depth_demand().unwrap()};
  assert!(source.close_step(close).unwrap().progress().fits(close));
 }
 assert!(source.terminal_is_empty());
 (output,turns)
}

fn texts()->Vec<String>{vec![String::new(),"plain".into(),"quote\" slash\\ tab\t newline\n bell\u{7} del\u{7f}".into(),"Ä🧩 \u{0}\u{1f}".into(),"x".repeat(3000)]}

fn bulk_document(scale:usize)->BulkDocument {
 BulkDocument{
  name:"bulk \"document\"\n".into(),
  samples:(0..scale).map(|index|((index*40503+12345)%65536)as u16).collect(),
  octets:(0..scale).map(|index|(index%251)as u8).collect(),
  texels:(0..scale/8).map(|index|BulkTexel{red:index as u32,green:(index*3)as u32,blue:u32::MAX-index as u32,alpha:255,reserved:0}).collect(),
  tags:texts(),
  maybe:Some((0..scale/4).map(|index|index as u16).collect()),
  nested:(0..40).map(|row|(0..row*7).map(|column|(row*column)as u16).collect()).collect(),
  empty:Vec::new(),
  pair:(7,true),
 }
}

#[test]
fn bulk_canonical_walk_is_byte_identical_to_the_one_event_walk_and_to_serde() {
 for scale in [0usize,1,5,64,1000,2500] {
  let document=bulk_document(scale);
  let oracle=serde_json::to_vec(&document).unwrap();
  let(legacy,legacy_turns)=encode_with_window(bulk_document(scale),1);
  assert_eq!(legacy,oracle,"scale {scale}: the one-event walk matches serde");
  for window in [FLAT_MINIMUM_BYTES,33,64,255,256,4096,1<<16] {
   let(bulk,bulk_turns)=encode_with_window(bulk_document(scale),window);
   assert_eq!(bulk,legacy,"scale {scale} window {window}: bulk bytes equal the one-event walk");
   if window==4096 {println!("[DEBUG] bulk canonical walk scale={scale} window=4096 bytes={} bulkTurns={bulk_turns} oneEventTurns={legacy_turns}",bulk.len());}
   if window==4096&&scale>=1000 {assert!(bulk_turns*40<legacy_turns,"scale {scale}: {bulk_turns} bulk turns vs {legacy_turns} one-event turns");}
  }
  println!("[DEBUG] bulk canonical walk scale={scale} bytes={} oneEventTurns={legacy_turns}",legacy.len());
 }
}

#[test]
fn bulk_canonical_walk_matches_the_one_event_walk_for_floats_wide_integers_and_deep_nests() {
 let floats=||BulkFloats{weights:[0.0,-1.5,3.25e10,f32::MIN_POSITIVE],list:(0..300).map(|index|index as f32*0.37-11.0).collect(),wide:(0..300).map(|index|i64::MIN+index*1_000_003).collect(),large:(0..300).map(|index|u64::MAX-index).collect(),flags:(0..300).map(|index|index%3==0).collect(),nothing:vec![();40]};
 let(legacy,_)=encode_with_window(floats(),1);
 for window in [FLAT_MINIMUM_BYTES,100,4096] {assert_eq!(encode_with_window(floats(),window).0,legacy,"window {window}");}
 let deep=|depth:usize|{let mut value=NativeValue::Number(7);for _ in 0..depth{value=NativeValue::Array([value,NativeValue::Flag(false)].into_iter().collect());}value};
 for depth in [1usize,5,FLAT_DEPTH,FLAT_DEPTH+1,FLAT_DEPTH+30] {
  let(legacy,_)=encode_with_window(deep(depth),1);
  for window in [FLAT_MINIMUM_BYTES,4096] {assert_eq!(encode_with_window(deep(depth),window).0,legacy,"depth {depth} window {window}");}
 }
 let text=NativeValue::Text("native \"paged\" text\n".into());
 assert_eq!(encode_with_window(text,4096).0,b"\"native \\\"paged\\\" text\\n\"");
}
