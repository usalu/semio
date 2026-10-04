//! 🚦️ Neutral refusal identity survives private controlled Pack work and public text terminals.
use super::*;
#[test]
fn pack_output_refusals_preserve_private_categories_and_terminal_messages(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🚦️refusals/🔣️.json")).unwrap();
    let record=RecordValue::default();
    let mut deny=|_|false;let mut control=NativeEncodeControl::new(usize::MAX,&mut deny);let mut symbols=Symbols{entries:Vec::new()};
    let canceled=symbols.record(None,&record,0,64,&mut control).unwrap_err();
    let mut allow=|_|true;let mut control=NativeEncodeControl::new(0,&mut allow);
    let ownership=symbols.note("literal",false,&mut control).unwrap_err();
    let depth=symbols.record(None,&record,65,64,&mut control).unwrap_err();
    for(error,case)in[canceled,ownership,depth].into_iter().zip(fixture["cases"].as_array().unwrap()){
        let PackRefusal::ValueRefusal(refusal)=error else{panic!("private control lost refusal authority")};
        assert_eq!(refusal.kind.as_str(),case["kind"].as_str().unwrap());
        let kind=refusal.kind;let message=refusal.message.clone();let wrapped=PackRefusal::from(refusal);let PackRefusal::ValueRefusal(refusal)=&wrapped else{panic!("typed refusal lost at Pack boundary")};let source=std::error::Error::source(&wrapped).unwrap().downcast_ref::<ValueError>().unwrap();assert!(std::ptr::eq(source,refusal));let projected=wrapped.into_value_error();assert_eq!(projected.kind,kind);assert_eq!(projected.message,message);
    }
    let mut allow=|_|true;let mut control=NativeEncodeControl::new(usize::MAX,&mut allow);let mut symbols=Symbols{entries:Vec::new()};
    for text in fixture["symbols"].as_array().unwrap(){symbols.note(text.as_str().unwrap(),false,&mut control).unwrap();}
    symbols.finish(&mut control).unwrap();
    let actual:Vec<_>=symbols.entries.iter().map(|symbol|symbol.text).collect();let expected:Vec<_>=fixture["orderedSymbols"].as_array().unwrap().iter().map(|text|text.as_str().unwrap()).collect();assert_eq!(actual,expected);
    let independent=serde_json::to_string(&expected).unwrap();assert_eq!(serde_json::from_str::<Vec<String>>(&independent).unwrap(),actual);
}
