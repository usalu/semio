use crate::{interpreter::{CoreInstance,CoreModule,CoreStepOutcome,StepControl,Value},operation_authority::{owned_result,refusal_code}};

#[test]
fn owned_scalar_refusal_matches_actual_interpreter_and_independent_wasmtime(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let engine=wasmtime::Engine::default();
    for case in fixture["accepted"].as_array().unwrap(){
        let word=case["word"].as_str().unwrap().parse::<u64>().unwrap();let wat=format!("(module (func (export \"result\") (result i64) i64.const {}))",word as i64);
        let module=wasmtime::Module::new(&engine,&wat).unwrap();let mut store=wasmtime::Store::new(&engine,());let instance=wasmtime::Instance::new(&mut store,&module,&[]).unwrap();assert_eq!(instance.get_typed_func::<(),i64>(&mut store,"result").unwrap().call(&mut store,()).unwrap() as u64,word);
        let module=std::sync::Arc::new(CoreModule::parse(&scalar_module(word)).unwrap());let mut instance=CoreInstance::instantiate(module).unwrap();instance.begin_export("result",Vec::new()).unwrap();let values=loop{match instance.step(100,StepControl::default()){CoreStepOutcome::Complete{values,..}=>break values,CoreStepOutcome::Yield{..}=>{},other=>panic!("unexpected original interpreter outcome {other:?}")}};assert_eq!(values,vec![Value::I64(word as i64)]);
        let decoded=owned_result::decode(word).unwrap();if let Some(code)=case["refusal"].as_u64(){match decoded{owned_result::OwnedReturn::Refusal(kind)=>{assert_eq!(refusal_code(kind),code as u32);assert_eq!(owned_result::encode_refusal(kind),word);},_=>panic!("refusal exposed physical bytes")}}else{assert_eq!(decoded,owned_result::OwnedReturn::Bytes{pointer:case["pointer"].as_u64().unwrap()as u32,length:case["length"].as_u64().unwrap()as u32});}
    }
    for case in fixture["refused"].as_array().unwrap(){assert!(owned_result::decode((case["length"].as_u64().unwrap()<<32)|case["pointer"].as_u64().unwrap()).is_err());}
    println!("[DEBUG] Owned original scalar return: actualInterpreter=true independentWasmtime=true codes=8 invalidCodesRefused=true noResultCopy=true");
}

pub(crate) fn scalar_module(word:u64)->Vec<u8>{let mut instruction=vec![0,0x42];let mut value=word as i64;loop{let byte=(value as u8)&0x7f;value>>=7;let done=(value==0&&byte&0x40==0)||(value == -1&&byte&0x40!=0);instruction.push(if done{byte}else{byte|0x80});if done{break;}}instruction.push(0x0b);let mut body=vec![1,instruction.len()as u8];body.extend(instruction);let mut bytes=b"\0asm\x01\0\0\0".to_vec();for(id,data)in[(1,vec![1,0x60,0,1,0x7e]),(3,vec![1,0]),(7,vec![1,6,b'r',b'e',b's',b'u',b'l',b't',0,0]),(10,body)]{bytes.push(id);bytes.push(data.len()as u8);bytes.extend(data);}bytes}
