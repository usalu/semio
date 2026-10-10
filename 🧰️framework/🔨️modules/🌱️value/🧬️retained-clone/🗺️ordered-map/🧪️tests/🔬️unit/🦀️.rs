use super::*;
use crate::retained_clone::RetainedCloneSource;
use serde::Deserialize;
use std::collections::BTreeMap;

fn physical_grant(work:usize)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:work.max(65536),maximum_capacity_bytes:4096,maximum_release_bytes:65536,maximum_depth:64}}

#[test]
fn ordered_map_clone_advance_quotes_balance_original_heap_and_refused_currencies() {
    use crate::retirement::controlled::ControlledRetirement;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️clone/🔣️.json")).unwrap();
    assert_eq!(fixture["pageCapacity"].as_u64().unwrap()as usize,RETAINED_ORDERED_MAP_PAGE_CAPACITY);
    let body=fixture["workBytes"].as_u64().unwrap()as usize;
    let quote=|d:crate::RetirementDemand|RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
    for row in fixture["cases"].as_array().unwrap(){
        let count=row["entryCount"].as_u64().unwrap();
        let expected:Vec<u64>=serde_json::from_value(row["expectedKeys"].clone()).unwrap();
        for stop in std::iter::once(None).chain(fixture["interruptAfter"].as_array().unwrap().iter().map(|x|Some(x.as_u64().unwrap()as usize))){
            let(map,original)=crate::value::observe_retirement_allocations(||(0..count).map(|key|(key,())).collect::<RetainedOrderedMap<u64,()>>());
            assert_eq!(original.1,0);
            let birth=RetainedCloneSource::<RetainedOrderedMap<u64,()>>::owned_constructor_capacity_bytes::<()>();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedCloneSource::<RetainedOrderedMap<u64,()>>::constructor_copy_bytes(),maximum_capacity_bytes:birth,maximum_depth:1,..Default::default()};
            let(admitted,heap)=crate::value::observe_retirement_allocations(||RetainedCloneSource::admit_owned(map,(),grant));
            let(mut source,receipt)=admitted.unwrap_or_else(|_|panic!("original numeric clone source admission"));
            assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));
            let mut born=heap.0;let mut freed=heap.1;let mut output=None;let mut cursor=RetainedOrderedMap::<u64,()>::retained_clone_cursor();
            for turn in 0..10000{
                if stop.is_some_and(|stop|turn>=stop){break;}
                let demand=cursor.advance_demands(source.borrow(),body).unwrap();let grant=quote(demand);
                for axis in fixture["deniedAxes"].as_array().unwrap(){
                    let mut denied=grant;
                    match axis.as_str().unwrap(){"items"=>denied.maximum_items=0,"copy" if demand.copy_bytes>0=>denied.maximum_copy_bytes=demand.copy_bytes-1,"capacity" if demand.capacity_bytes>0=>denied.maximum_capacity_bytes=demand.capacity_bytes-1,"release" if demand.release_bytes>0=>denied.maximum_release_bytes=demand.release_bytes-1,"depth" if demand.depth>0=>denied.maximum_depth=demand.depth-1,_=>continue}
                    let(result,heap)=crate::value::observe_retirement_allocations(||cursor.advance(source.borrow(),denied));
                    assert_eq!(heap,(0,0));match result{Ok(step)=>assert_eq!(step.progress(),Default::default()),Err(error)=>assert!(matches!(error.kind,crate::ValueRefusalKind::DepthLimit|crate::ValueRefusalKind::WorkLimit|crate::ValueRefusalKind::OwnershipLimit))};assert_eq!(cursor.advance_demands(source.borrow(),body).unwrap(),demand);
                }
                let(step,heap)=crate::value::observe_retirement_allocations(||cursor.advance(source.borrow(),grant).unwrap());let receipt=step.progress();
                assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;
                if matches!(step,RetainedCloneStep::Complete(_)){output=cursor.take();break;}
            }
            if stop.is_none(){let output=output.as_ref().expect("funded clone completed");assert_eq!(output.iter().map(|(key,())|*key).collect::<Vec<_>>(),expected);}
            assert_eq!(source.borrow().get().iter().map(|(key,())|*key).collect::<Vec<_>>(),expected);
            let mut cursor=ControlledRetirement::new(cursor).unwrap_or_else(|_|panic!("original clone cursor controlled authority"));
            for _ in 0..10000{if cursor.terminal_is_empty(){break;}let grant=quote(crate::RetirementDemand{copy_bytes:cursor.next_copy_byte_demand().unwrap().max(body),capacity_bytes:cursor.next_capacity_byte_demand(body).unwrap(),release_bytes:cursor.next_release_byte_demand().unwrap(),depth:cursor.next_depth_demand().unwrap()});let(step,heap)=crate::value::observe_retirement_allocations(||cursor.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}
            assert!(cursor.terminal_is_empty());
            for _ in 0..10000{if source.terminal_is_empty(){break;}let grant=quote(crate::RetirementDemand{copy_bytes:source.next_close_copy_byte_demand().unwrap().max(body),capacity_bytes:source.next_close_capacity_byte_demand(body).unwrap(),release_bytes:source.next_close_release_byte_demand().unwrap(),depth:source.next_close_depth_demand().unwrap()});let(step,heap)=crate::value::observe_retirement_allocations(||source.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}
            assert!(source.terminal_is_empty());
            if let Some(output)=output{let mut owner=ControlledRetirement::new(output).unwrap_or_else(|_|panic!("numeric map controlled authority"));for _ in 0..10000{if owner.terminal_is_empty(){break;}let grant=quote(crate::RetirementDemand{copy_bytes:owner.next_copy_byte_demand().unwrap().max(body),capacity_bytes:owner.next_capacity_byte_demand(body).unwrap(),release_bytes:owner.next_release_byte_demand().unwrap(),depth:owner.next_depth_demand().unwrap()});let(step,heap)=crate::value::observe_retirement_allocations(||owner.step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(heap,(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.0;freed+=heap.1;}assert!(owner.terminal_is_empty());}
            let(_,heap)=crate::value::observe_retirement_allocations(||{drop(cursor);drop(source);});assert_eq!(heap,(0,0));assert_eq!(original.0+born,freed);
            println!("[DEBUG] Ordered numeric clone count={count} stop={stop:?} original={} born={born} physical={freed}",original.0);
        }
    }
}
fn insert(map:RetainedOrderedMap<String,String>,key:String,value:String)->RetainedOrderedMapInsertCursor<String,String>{
    let bytes=RetainedOrderedMapInsertCursor::<String,String>::constructor_capacity_bytes();
    let (cursor,receipt)=RetainedOrderedMapInsertCursor::admit(map,key,value,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedOrderedMapInsertCursor::<String,String>::constructor_copy_bytes(),maximum_capacity_bytes:bytes,maximum_depth:1,..Default::default()}).unwrap_or_else(|_|panic!("test insertion admission"));
    assert_eq!(receipt.retained_capacity_bytes,bytes);cursor
}
fn close_insert(cursor:&mut RetainedOrderedMapInsertCursor<String,String>){
    cursor.begin_close();for _ in 0..100000{if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};assert!(cursor.close_step(grant).unwrap().progress().fits(grant));}assert!(cursor.terminal_is_empty());
}
fn close_comparator<K:BoundedOrd>(cursor:&mut K::Cursor){
    cursor.begin_close();for _ in 0..100000{if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};assert!(cursor.close_step(grant).unwrap().progress().fits(grant));}assert!(cursor.terminal_is_empty());
}
fn close_lookup<K:BoundedOrd>(cursor:&mut RetainedOrderedMapLookupCursor<K>){
    cursor.begin_close();for _ in 0..100000{if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};assert!(cursor.close_step(grant).unwrap().progress().fits(grant));}assert!(cursor.terminal_is_empty());
}


#[test]
fn ordered_comparison_custody_refuses_unadmitted_constructor_and_closes_original_aliases() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../♻️custody/🧫️fixtures/🔣️.json")).unwrap();
    let mut oracle=std::process::Command::new("bun").args(["-e","const x=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(x.fixture.keys.map(x=>[x,...new TextEncoder().encode(x)]))); "]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;
    oracle.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture}).to_string().as_bytes()).unwrap();
    let output=oracle.wait_with_output().unwrap();assert!(output.status.success());
    let rows:Vec<serde_json::Value>=serde_json::from_slice(&output.stdout).unwrap();
    for (key,row) in fixture["keys"].as_array().unwrap().iter().zip(rows) {
        let key=key.as_str().unwrap();assert_eq!(row.as_array().unwrap()[1..],key.as_bytes().iter().map(|x|serde_json::json!(*x)).collect::<Vec<_>>());
        let map=RetainedOrderedMap::<String,String>::default();let key=key.to_owned();let pointer=key.as_ptr();
        let value=String::from("value");let original=key.capacity()+value.capacity();
        let (refused,heap)=crate::value::observe_retirement_allocations(||RetainedOrderedMapInsertCursor::admit(map,key,value,Default::default()));
        assert_eq!(heap,(0,0));let (error,map,key,value)=refused.err().unwrap();
        assert_eq!(error.kind,crate::ValueRefusalKind::WorkLimit);assert_eq!(key.as_ptr(),pointer);
        let bytes=RetainedOrderedMapInsertCursor::<String,String>::constructor_capacity_bytes();
        let (admitted,heap)=crate::value::observe_retirement_allocations(||RetainedOrderedMapInsertCursor::admit(map,key,value,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:RetainedOrderedMapInsertCursor::<String,String>::constructor_copy_bytes(),maximum_depth:1,maximum_capacity_bytes:bytes,..Default::default()}));
        let (mut cursor,receipt)=admitted.unwrap_or_else(|_|panic!("admitted insertion constructor refused"));
        assert_eq!(receipt.retained_capacity_bytes,bytes);assert_eq!(heap,(bytes,0));cursor.begin_close();let mut born=bytes;let mut freed=0;
        for _ in 0..10000 {if cursor.terminal_is_empty(){break;}let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(3).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand().unwrap()};let (step,heap)=crate::value::observe_retirement_allocations(||cursor.close_step(grant).unwrap());let p=step.progress();assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));born+=p.retained_capacity_bytes;freed+=p.released_bytes;}
        assert!(cursor.terminal_is_empty());assert_eq!(original+born,freed);println!("[DEBUG] Ordered insertion authority original={original} born={born} physical={freed}");
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    page_capacity: usize,
    entry_count: usize,
    key_prefix: String,
    value_prefix: String,
    long_key_byte_length: usize,
    comparison_grant: ComparisonGrant,
    progress_channels: ProgressChannels,
    repeated_growth: RepeatedGrowth,
    immutable_lookup: ImmutableLookup,
    operations: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RepeatedGrowth {
    entry_count: usize,
    insertions: usize,
    key_prefix: String,
    value_prefix: String,
    expected_entry_count: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ImmutableLookup {
    captured_target: String,
    external_after_capture: String,
    expected_ordinal: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ComparisonGrant {
    maximum_items: usize,
    maximum_bytes: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProgressChannels {
    comparison_only: bool,
    capacity_only: bool,
    minimum_moved_items: usize,
}

#[derive(Deserialize)]
struct Operation {
    kind: String,
    key: String,
    value: Option<String>,
    expected: Expected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Expected {
    found: bool,
    ordinal: usize,
    entry_count: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📦️paging/🔣️.json")).expect("retained ordered-map fixture")
}

fn oracle(fixture: &Fixture) -> BTreeMap<String, String> {
    (0..fixture.entry_count).map(|ordinal| (format!("{}{:04}", fixture.key_prefix, ordinal), format!("{}{}", fixture.value_prefix, ordinal))).collect()
}

fn retained_map(source: &RetainedOrderedMap<String, String>) -> RetainedOrderedMap<String, String> {
    let source = RetainedCloneSource::from_owner(source.clone());
    let grant = RetainedCloneGrant { maximum_items: 2, maximum_copy_bytes: 64, maximum_capacity_bytes: 65_536, maximum_depth: 64, maximum_release_bytes: 65_536 };
    let mut cursor = RetainedOrderedMap::<String, String>::retained_clone_cursor();
    let output = loop {
        let step = cursor.advance(source.borrow(), grant).expect("retained ordered-map clone");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("retained ordered-map output");
        }
    };
    cursor.begin_close();
    while !cursor.terminal_is_empty() {
        let bytes = cursor.next_close_release_byte_demand().unwrap();
        assert!(bytes <= grant.maximum_release_bytes);
        let step = cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:bytes,maximum_depth:64}).expect("retained ordered-map cursor close");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(cursor.terminal_is_empty());
        }
    }
    output
}

fn lookup(map: &RetainedOrderedMap<String, String>, key: &String, fixture: &Fixture) -> RetainedOrderedMapLookup {
    let map = RetainedCloneSource::from_owner(map.clone());
    let key = RetainedCloneSource::from_owner(key.clone());
    let grant = BoundedOrdGrant { maximum_items: fixture.comparison_grant.maximum_items, maximum_bytes: fixture.comparison_grant.maximum_bytes };
    let mut cursor = RetainedOrderedMapLookupCursor::default();
    loop {
        let (result, progress, retirement) = cursor.advance(map.borrow(), key.borrow(), grant,physical_grant(7)).expect("retained ordered-map lookup");
        assert!(progress.fits(grant));
        assert!(retirement.fits(physical_grant(7)));
        if let Some(result) = result {
            close_lookup(&mut cursor);return result;
        }
    }
}

#[test]
fn fixed_page_clone_matches_btree_and_serde_oracles() {
    let fixture = fixture();
    assert_eq!(fixture.page_capacity, RETAINED_ORDERED_MAP_PAGE_CAPACITY);
    let oracle = oracle(&fixture);
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("ordered source");
    let copied = retained_map(&source);
    assert_eq!(copied.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    assert_eq!(serde_json::to_value(&copied).expect("retained ordered-map JSON"), serde_json::to_value(&oracle).expect("BTreeMap JSON"));
    assert_eq!(copied.page_count(), fixture.entry_count.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
}

#[test]
fn bounded_lookup_insert_and_duplicate_refusal_match_btree_oracle() {
    let fixture = fixture();
    let mut oracle = oracle(&fixture);
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("ordered source");
    let mut map = retained_map(&source);
    for operation in &fixture.operations {
        let before = lookup(&map, &operation.key, &fixture);
        match operation.kind.as_str() {
            "lookup" => {
                assert_eq!(matches!(before, RetainedOrderedMapLookup::Found(_)), operation.expected.found);
                assert_eq!(
                    match before {
                        RetainedOrderedMapLookup::Found(ordinal) | RetainedOrderedMapLookup::Missing(ordinal) => ordinal,
                    },
                    operation.expected.ordinal
                );
            }
            "insert" => {
                assert!(matches!(before, RetainedOrderedMapLookup::Missing(ordinal) if ordinal == operation.expected.ordinal));
                let value = operation.value.clone().expect("insert value");
                let mut cursor = insert(map, operation.key.clone(), value.clone());
                let grant = RetainedOrderedMapInsertGrant { retirement:physical_grant(7), comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
                let ordinal = loop {
                    match cursor.advance(grant).expect("bounded insertion") {
                        RetainedOrderedMapInsertStep::Progress(progress) => assert!(progress.fits(grant)),
                        RetainedOrderedMapInsertStep::Complete { ordinal, progress } => {
                            assert!(progress.fits(grant));
                            break ordinal;
                        }
                    }
                };
                assert!(cursor.output_ready());
                map = cursor.take().expect("bounded insertion output");
                assert!(!cursor.output_ready());
                assert_eq!(ordinal, operation.expected.ordinal);
                oracle.insert(operation.key.clone(), value);
                assert!(cursor.begin_close());
                close_insert(&mut cursor);assert!(cursor.terminal_is_empty());
            }
            "duplicate" => {
                assert!(matches!(before, RetainedOrderedMapLookup::Found(ordinal) if ordinal == operation.expected.ordinal));
                assert!(operation.expected.found);
                let before_json = serde_json::to_value(&map).expect("map before duplicate");
                let mut cursor = insert(map, operation.key.clone(), operation.value.clone().expect("duplicate value"));
                let grant = RetainedOrderedMapInsertGrant { retirement:physical_grant(7), comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 1, maximum_moved_bytes: 7, maximum_capacity_bytes: 65_536 };
                let error = loop {
                    match cursor.advance(grant) {
                        Ok(RetainedOrderedMapInsertStep::Progress(progress)) => assert!(progress.fits(grant)),
                        Ok(RetainedOrderedMapInsertStep::Complete { .. }) => panic!("duplicate insertion completed"),
                        Err(error) => break error,
                    }
                };
                assert!(cursor.refused_workspace_ready());
                map = cursor.take_refused_workspace().expect("duplicate refusal workspace");
                assert!(!cursor.refused_workspace_ready());
                assert!(error.message.contains("duplicate"));
                assert_eq!(serde_json::to_value(&map).expect("map after duplicate"), before_json);
                assert!(cursor.begin_close());
                while !cursor.terminal_is_empty() {
                    let bytes = cursor.next_close_release_byte_demand().unwrap();
                    assert!(bytes <= 65_536);
                    let step = cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:bytes,maximum_depth:64}).expect("duplicate cursor close");
                    if matches!(step, RetainedCloneStep::Complete(_)) {
                        assert!(cursor.terminal_is_empty());
                    }
                }
            }
            other => panic!("unexpected fixture operation {other}"),
        }
        assert_eq!(map.len(), operation.expected.entry_count);
        assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    }
}

#[test]
fn long_string_comparison_is_byte_paged_and_shape_pinned() {
    let fixture = fixture();
    let left = "k".repeat(fixture.long_key_byte_length);
    let right = format!("{left}z");
    let grant = BoundedOrdGrant { maximum_items: fixture.comparison_grant.maximum_items, maximum_bytes: fixture.comparison_grant.maximum_bytes };
    let mut cursor = String::bounded_ord_cursor();
    let left_source = RetainedCloneSource::from_owner(left.clone());
    let right_source = RetainedCloneSource::from_owner(right.clone());
    let mut turns = 0usize;
    loop {
        turns += 1;
        match cursor.compare(left_source.borrow(), right_source.borrow(), grant,physical_grant(7)).expect("bounded string comparison") {
            BoundedOrdStep::Authority(progress) => assert!(progress.fits(physical_grant(7))),
            BoundedOrdStep::Progress(progress) => assert!(progress.fits(grant)),
            BoundedOrdStep::Complete { ordering, progress } => {
                assert!(progress.fits(grant));
                assert_eq!(ordering, Ordering::Less);
                break;
            }
        }
    }
    assert!(turns > fixture.long_key_byte_length / fixture.comparison_grant.maximum_bytes);
    close_comparator::<String>(&mut cursor);

    let changed = RetainedCloneSource::from_owner(left.clone());
    let replacement = RetainedCloneSource::from_owner(format!("{}x", &left[..left.len() - 1]));
    let right = RetainedCloneSource::from_owner(right);
    let mut cursor = String::bounded_ord_cursor();
    cursor.compare(changed.borrow(), right.borrow(), grant,physical_grant(7)).expect("comparison initialization");
    assert!(cursor.compare(replacement.borrow(), right.borrow(), grant,physical_grant(7)).expect_err("changed comparison source must fail").message.contains("projected path changed"));
    close_comparator::<String>(&mut cursor);
}

#[test]
fn immutable_lookup_target_and_repeated_directory_growth_match_btree_oracle() {
    let fixture = fixture();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle(&fixture).into_iter().collect()).expect("lookup source");
    let source = RetainedCloneSource::from_owner(source);
    let mut external_target = fixture.immutable_lookup.captured_target.clone();
    let target = RetainedCloneSource::from_owner(external_target.clone());
    let grant = BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 };
    let mut lookup = RetainedOrderedMapLookupCursor::default();
    lookup.advance(source.borrow(), target.borrow(), grant,physical_grant(7)).expect("lookup initialization");
    external_target = fixture.immutable_lookup.external_after_capture.clone();
    let result = loop {
        if let (Some(result), progress, retirement) = lookup.advance(source.borrow(), target.borrow(), grant,physical_grant(7)).expect("captured lookup") {
            assert!(progress.fits(grant));
            break result;
        }
    };
    let changed=RetainedCloneSource::from_owner(external_target.clone());
    assert!(lookup.advance(source.borrow(),changed.borrow(),grant,physical_grant(7)).expect_err("cached lookup must retain the captured target authority").message.contains("projected path changed"));
    close_lookup(&mut lookup);
    assert_eq!(result, RetainedOrderedMapLookup::Missing(fixture.immutable_lookup.expected_ordinal));
    assert_ne!(external_target, fixture.immutable_lookup.captured_target);

    let growth = &fixture.repeated_growth;
    let mut oracle = (0..growth.entry_count).map(|ordinal| (format!("{}{:04}", growth.key_prefix, ordinal * 2), format!("{}{}", growth.value_prefix, ordinal))).collect::<BTreeMap<_, _>>();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle.clone().into_iter().collect()).expect("growth source");
    let mut map = retained_map(&source);
    let grant = RetainedOrderedMapInsertGrant { retirement:physical_grant(7), comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
    for ordinal in 0..growth.insertions {
        let key = format!("{}{:04}", growth.key_prefix, ordinal * 2 + 1);
        let value = format!("{}insert-{ordinal}", growth.value_prefix);
        let mut cursor = insert(map, key.clone(), value.clone());
        loop {
            match cursor.advance(grant).expect("repeated map growth") {
                RetainedOrderedMapInsertStep::Progress(progress) => assert!(progress.fits(grant)),
                RetainedOrderedMapInsertStep::Complete { progress, .. } => {
                    assert!(progress.fits(grant));
                    break;
                }
            }
        }
        map = cursor.take().expect("grown map output");
        oracle.insert(key, value);
        assert!(cursor.begin_close());
        while !cursor.terminal_is_empty() {
            let bytes = cursor.next_close_release_byte_demand().unwrap();
                    assert!(bytes <= 65_536);
                    let step = cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:bytes,maximum_depth:64}).expect("grown insertion close");
            if matches!(step, RetainedCloneStep::Complete(_)) {
                assert!(cursor.terminal_is_empty());
            }
        }
    }
    assert_eq!(map.len(), growth.expected_entry_count);
    assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
}

#[test]
fn insertion_reports_comparison_movement_and_capacity_independently() {
    let fixture = fixture();
    let prefix = "k".repeat(fixture.long_key_byte_length);
    let entries = (0..RETAINED_ORDERED_MAP_PAGE_CAPACITY).map(|ordinal| (format!("{prefix}-{:04}", ordinal * 2), format!("value-{ordinal}"))).collect::<Vec<_>>();
    let oracle = entries.iter().cloned().collect::<BTreeMap<_, _>>();
    let map = RetainedOrderedMap::from_sorted_entries_for_test(entries).expect("long-key insertion source");
    let key = format!("{prefix}-0001");
    let value = "inserted-β".to_string();
    let grant = RetainedOrderedMapInsertGrant { retirement:physical_grant(7),
        comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: fixture.comparison_grant.maximum_bytes },
        maximum_moved_items: RETAINED_ORDERED_MAP_PAGE_CAPACITY + 1,
        maximum_moved_bytes: (RETAINED_ORDERED_MAP_PAGE_CAPACITY + 1) * std::mem::size_of::<(String, String)>(),
        maximum_capacity_bytes: 65_536,
    };
    let mut cursor = insert(map, key.clone(), value.clone());
    let mut compared_bytes = 0usize;
    let mut moved_bytes = 0usize;
    let mut retained_capacity_bytes = 0usize;
    loop {
        let step = cursor.advance(grant).expect("long-key bounded insertion");
        let (progress, complete) = match step {
            RetainedOrderedMapInsertStep::Progress(progress) => (progress, false),
            RetainedOrderedMapInsertStep::Complete { progress, .. } => (progress, true),
        };
        assert!(progress.fits(grant));
        assert!(!fixture.progress_channels.comparison_only || progress.comparison.compared_bytes == 0 || (progress.moved_items == 0 && progress.moved_bytes == 0 && progress.retained_capacity_bytes == 0));
        assert!(!fixture.progress_channels.capacity_only || progress.retained_capacity_bytes == 0 || (progress.comparison == BoundedOrdProgress::default() && progress.moved_items == 0 && progress.moved_bytes == 0));
        compared_bytes += progress.comparison.compared_bytes;
        moved_bytes += progress.moved_bytes;
        retained_capacity_bytes += progress.retained_capacity_bytes;
        if complete {
            break;
        }
    }
    assert!(compared_bytes >= fixture.long_key_byte_length);
    assert!(moved_bytes >= fixture.progress_channels.minimum_moved_items * std::mem::size_of::<(String, String)>());
    assert!(retained_capacity_bytes >= RETAINED_ORDERED_MAP_PAGE_CAPACITY * std::mem::size_of::<(String, String)>());
    let map = cursor.take().expect("long-key insertion output");
    let mut oracle = oracle;
    oracle.insert(key, value);
    assert_eq!(map.iter().map(|(key, value)| (key.clone(), value.clone())).collect::<BTreeMap<_, _>>(), oracle);
    assert!(cursor.begin_close());
    close_insert(&mut cursor);assert!(cursor.terminal_is_empty());
}

#[test]
fn cancelled_partial_insertion_retires_candidate_and_shifted_workspace() {
    let fixture = fixture();
    let source = RetainedOrderedMap::from_sorted_entries_for_test(oracle(&fixture).into_iter().collect()).expect("ordered source");
    let workspace = retained_map(&source);
    let original_pages = workspace.pages.iter().map(Vec::len).collect::<Vec<_>>();
    let mut cursor = insert(workspace, "key-0000a".to_string(), "cancelled-β".to_string());
    let grant = RetainedOrderedMapInsertGrant { retirement:physical_grant(7), comparison: BoundedOrdGrant { maximum_items: 1, maximum_bytes: 7 }, maximum_moved_items: 32, maximum_moved_bytes: 4_096, maximum_capacity_bytes: 65_536 };
    let mut turns = 0usize;
    while cursor.state.map.as_ref().is_none_or(|map|map.pages.iter().map(Vec::len).eq(original_pages.iter().copied())) {
        assert!(matches!(cursor.advance(grant).expect("partial insertion"), RetainedOrderedMapInsertStep::Progress(_)));
        turns += 1;
        assert!(turns < 1_000);
    }
    assert_eq!(cursor.state.map.as_ref().expect("shifted workspace").len(), fixture.entry_count);

    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        let bytes = cursor.next_close_release_byte_demand().unwrap();
                    assert!(bytes <= 65_536);
                    let step = cursor.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(7).unwrap(),maximum_release_bytes:bytes,maximum_depth:64}).expect("cancelled insertion cursor close");
        if matches!(step, RetainedCloneStep::Complete(_)) {
            assert!(cursor.terminal_is_empty());
        }
    }
}

#[test]
fn cold_clone_terminal_frame_denies_partial_funding_and_reports_actual_same_turn_release() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️close/🔣️.json")).unwrap();
    let admission = fixture["maximumAdmissionBytes"].as_u64().unwrap() as usize;
    let mut mismatches = 0;
    for row in fixture["cases"].as_array().unwrap() {
        let mut text = String::with_capacity(row["capacity"].as_u64().unwrap() as usize);
        text.push_str(row["text"].as_str().unwrap());
        let mut close = RetainedCloneClose::default();
        let mut text=Some(text);
        let birth=crate::owned_retirement_birth_bytes::<String>();
        assert!(birth<=admission);
        close.begin_granted(&mut text,RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:birth,maximum_depth:1,..Default::default()}).unwrap();
        assert!(text.is_none());
        for _ in 0..10_000 {
            if close.retirement.as_ref().unwrap().terminal_is_empty() { break; }
            let capacity=close.next_capacity_byte_demand(7).unwrap();
            let release=close.next_release_byte_demand().unwrap();
            assert!(capacity<=admission&&release<=admission);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:7,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:close.next_depth_demand().unwrap()};
            let (step,events)=crate::value::observe_retirement_allocations(||close.step_granted(grant).unwrap());
            assert!(step.progress().fits(grant));
            assert_eq!(events,(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        }
        assert!(close.retirement.as_ref().unwrap().terminal_is_empty());
        let pointer = close.retirement.as_ref().unwrap().as_ref() as *const dyn crate::ErasedSnapshotRetirement;
        let (physical, query_events) = crate::value::observe_retirement_allocations(|| close.next_release_byte_demand().unwrap());
        assert_eq!(query_events, (0, 0));
        assert!(physical > 0 && physical <= admission);
        for (items, bytes) in [(0, physical), (1, physical - 1)] {
            let (step, events) = crate::value::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:items,maximum_release_bytes:bytes,maximum_depth:1,..Default::default()}).unwrap());
            let retained = close.retirement.as_ref().is_some_and(|owner| std::ptr::addr_eq(pointer, owner.as_ref() as *const dyn crate::ErasedSnapshotRetirement));
            if step != (RetainedCloneStep::Progress(Default::default())) || events != (0, 0) || !retained {
                mismatches += 1;
                println!("[DEBUG] cold terminal denied items={items} bytes={bytes} query={physical} born={} actual-free={} retained={retained}", events.0, events.1);
            }
            if close.is_empty() { break; }
        }
        if !close.is_empty() {
            let (step, events) = crate::value::observe_retirement_allocations(|| close.step_granted(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:physical,maximum_depth:1,..Default::default()}).unwrap());
            if step != (RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:1,released_bytes:physical,..Default::default()})) || events != (0, physical) { mismatches += 1; }
        }
        assert!(close.is_empty());
        assert_eq!(close.step_granted(Default::default()).unwrap(), RetainedCloneStep::Complete(Default::default()));
        println!("[DEBUG] cold terminal original case={} exact-frame={physical} original-admission={admission}", row["id"]);
    }
    assert_eq!(mismatches, 0, "cold terminal retirement Box requires its complete same-turn physical grant");
}

#[test]
fn ordered_wire_cold_context_map_matches_serde_and_bun() {
    use crate::{FromValue,ToValue,retirement::controlled::ControlledRetirement};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📋️wire/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let entries:Vec<(String,String)>=serde_json::from_value(case["entries"].clone()).unwrap();
        let map:RetainedOrderedMap<String,String>=entries.into_iter().collect();
        assert_eq!(serde_json::to_value(&map).unwrap(),case["expected"]);
        let from_serde:RetainedOrderedMap<String,String>=serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(map,from_serde);
        let from_native=RetainedOrderedMap::<String,String>::from_value(map.to_value()).unwrap();
        assert_eq!(map,from_native);
        let keys:Vec<_>=map.iter().map(|(key,_)|key.clone()).collect();
        assert!(keys.windows(2).all(|pair|pair[0]<pair[1]));
        for(key,value)in map.iter(){assert_eq!(map.get(key.as_str()),Some(value));}
    }
    assert!(serde_json::from_str::<RetainedOrderedMap<String,String>>(fixture["rejectDuplicateJson"].as_str().unwrap()).is_err());
    let count=fixture["pageBoundaryEntries"].as_u64().unwrap()as usize;
    let map:RetainedOrderedMap<String,String>=(0..count).rev().map(|index|(format!("{index:03}"),format!("value-{index}"))).collect();
    assert_eq!(map.page_count(),3);
    let original=map.pages.capacity()*size_of::<Vec<(String,String)>>()+map.pages.iter().map(|page|page.capacity()*size_of::<(String,String)>()).sum::<usize>()+map.iter().map(|(key,value)|key.capacity()+value.capacity()).sum::<usize>();
    let birth=0;
    let (admitted,heap)=crate::value::observe_retirement_allocations(||ControlledRetirement::new(map));
    let mut retirement=admitted.unwrap_or_else(|_|panic!("actual paged map admission"));
    assert_eq!(heap,(birth,0));
    let mut allocated=birth;let mut released=0;
    for _ in 0..10000 {
        if retirement.terminal_is_empty(){break;}
        let copy=retirement.next_copy_byte_demand().unwrap().min(7);
        let release=retirement.next_release_byte_demand().unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:retirement.next_depth_demand().unwrap()};
        let (step,heap)=crate::value::observe_retirement_allocations(||retirement.step(grant).unwrap());
        let progress=step.progress();assert!(progress.fits(grant));assert_eq!(heap,(progress.retained_capacity_bytes,progress.released_bytes));allocated+=heap.0;released+=heap.1;
    }
    assert!(retirement.terminal_is_empty());assert_eq!(original+allocated,released);
    let mut oracle=std::process::Command::new("bun").args(["-e","const f=JSON.parse(await Bun.stdin.text());console.log(JSON.stringify(f.cases.map(c=>Object.fromEntries(c.entries))));"]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().unwrap();
    use std::io::Write;oracle.stdin.take().unwrap().write_all(fixture.to_string().as_bytes()).unwrap();
    let result=oracle.wait_with_output().unwrap();assert!(result.status.success());
    let independent:Vec<serde_json::Value>=serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(independent,fixture["cases"].as_array().unwrap().iter().map(|case|case["expected"].clone()).collect::<Vec<_>>());
    println!("[DEBUG] Ordered cold map wire cases=5 original={original} born={allocated} released={released}");
}
