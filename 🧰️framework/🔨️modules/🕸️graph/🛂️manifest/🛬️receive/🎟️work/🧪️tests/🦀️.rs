use super::*;
use semio_framework_value::{DslValue,Number};
#[path="../../../../../🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
#[test]fn actual_graph_original_tree_sorts_bounded_keys_rejects_duplicates_and_keeps_cancelled_custody(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../📐️.json")).unwrap();
    for case in law["cases"].as_array().unwrap(){
        for cut in std::iter::once(None).chain(law["cancelCuts"].as_array().unwrap().iter().map(|n|Some(n.as_u64().unwrap()))){
            let rows=case["rows"].as_array().unwrap();
            let original=DslValue::Object(rows.iter().map(|r|(r[0].as_str().unwrap().to_owned(),DslValue::Number(Number::UInt(r[1].as_u64().unwrap())))).collect());
            let mut bank=driver::bank();
            let(source,_,_)=semio_framework_value::testing::admit_owned_source(original,());
            let oracle:std::collections::BTreeMap<String,u64>=rows.iter().map(|r|(r[0].as_str().unwrap().to_owned(),r[1].as_u64().unwrap())).collect();
            let received=driver::lifecycle(<PropertyBag as NativeReceive<DslValue>>::receiver(),source,&mut bank,4096,cut,|output:&PropertyBag|{
                assert_eq!(serde_json::to_value(output.keys().collect::<Vec<_>>()).unwrap(),case["keys"]);
                assert_eq!(output.iter().map(|(k,v)|(k.clone(),v.as_f64().unwrap()as u64)).collect::<std::collections::BTreeMap<_,_>>(),oracle);
            });
            match(case["error"].as_str(),cut,&received){
                (Some(expected),None,Err(message))=>assert!(message.contains(expected),"expected refusal {expected}, got {message}"),
                (Some(expected),None,Ok(_))=>panic!("expected refusal {expected}"),
                (None,None,received)=>assert_eq!(received,&Ok(true)),
                (_,Some(_),received)=>assert!(matches!(received,Ok(false)|Err(_)),"a cancelled receive publishes nothing: {received:?}"),
            }
        }
    }
}
