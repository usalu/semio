
use super::*;

#[test]
fn neutral_interaction_interfaces_agree_with_independent_serde_wires(){
    fn agree<T:ToValue+serde::Serialize>(value:&T,expected:&serde_json::Value){assert_eq!(serde_json::to_value(value).unwrap(),*expected);assert_eq!(serde_json::to_value(value.to_value()).unwrap(),*expected);}
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/🔌️ports/🔣️.json")).unwrap();
    let selection=DomainSelection{granularity:"node".into(),ids:vec!["α".into(),"🧬".into()],anchor_id:Some("α".into())};agree(&selection,&fixture["selection"]);
    let hover=DomainHover{channel:"pointer".into(),ids:vec!["🧬".into()]};agree(&hover,&fixture["hover"]);
    for(method,expected)in[SelectionMethod::Pick,SelectionMethod::Rectangle,SelectionMethod::Lasso].iter().zip(fixture["methods"].as_array().unwrap()){agree(method,expected);}
    agree(&Viewport2d{x:-12.5,y:8.0,zoom:1.25},&fixture["viewport"]);
    println!("[DEBUG] neutral Surface interaction and viewport ports preserve their defining package wire contracts against independent Serde");
}

