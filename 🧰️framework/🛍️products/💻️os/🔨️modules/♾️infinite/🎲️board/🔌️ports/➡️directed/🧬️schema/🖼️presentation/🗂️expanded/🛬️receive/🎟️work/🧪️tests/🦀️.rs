use super::*;
use semio_framework_value::DslValue;
#[path="../../../../../../../../../../../../../🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
#[test]fn actual_expanded_paths_sort_and_deduplicate_through_the_new_receiver_protocol(){
    let f:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
    for case in f["cases"].as_array().unwrap(){
        let input:Vec<String>=serde_json::from_value(case["input"].clone()).unwrap();
        let oracle=input.iter().cloned().collect::<std::collections::BTreeSet<_>>().into_iter().collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(&oracle).unwrap(),case["expected"]);
        for cut in std::iter::once(None).chain(f["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()))){
            let mut bank=driver::bank();
            let(source,_,_)=semio_framework_value::testing::admit_owned_source(DslValue::Array(input.iter().cloned().map(DslValue::String).collect()),());
            let received=driver::lifecycle(<DagExpandedPaths as NativeReceive<DslValue>>::receiver(),source,&mut bank,4096,cut,|output:&DagExpandedPaths|assert_eq!(output.iter().collect::<Vec<_>>(),oracle.iter().collect::<Vec<_>>()));
            match cut{None=>assert_eq!(received,Ok(true),"expanded paths"),Some(_)=>assert!(matches!(received,Ok(false)|Err(_)),"a cancelled receive publishes nothing: {received:?}")}
        }
    }
}
