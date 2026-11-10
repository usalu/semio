use super::*;
use semio_framework_value::DslValue;
#[path="../../../../../../../../🧰️framework/🔨️modules/🌱️value/🧪️testing/📥️admission/receiver-driver.rs"]mod driver;
#[test]
fn original_erased_part21_sources_keep_binary_admission_and_funded_closure(){
    let law:serde_json::Value=serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    for row in law["binary"].as_array().unwrap(){
        let text=row["value"].as_str().unwrap();
        let input=DslValue::Object(vec![("kind".into(),DslValue::String("binary".into())),("value".into(),DslValue::String(text.into()))]);
        let mut bank=driver::bank();
        let(source,_,_)=semio_framework_value::testing::admit_owned_source(input,());
        let projected=source.borrow().project(0,|value|value as &dyn NativeValueSource);
        let mut receiver=<Part21Value as NativeReceive<dyn NativeValueSource>>::receiver();
        let received=driver::receive(&mut receiver,projected,&mut bank,4096);
        assert_eq!(received.is_ok(),row["admitted"].as_bool().unwrap());
        if let Ok(value)=received{assert!(matches!(&value,Part21Value::Binary(actual)if actual==text));std::mem::forget(value);}
        if receiver.output_ready()||!receiver.terminal_is_empty(){driver::close(&mut receiver,&mut bank,4096);}
        assert!(receiver.terminal_is_empty());
        std::mem::forget(receiver);std::mem::forget(source);
    }
}
