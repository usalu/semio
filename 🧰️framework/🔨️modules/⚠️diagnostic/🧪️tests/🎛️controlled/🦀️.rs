use super::*;
use semio_framework_value::{NativeDecodeControl, NativeEncodeControl, Number};
use std::{io::Write, process::{Command, Stdio}};

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🎛️controlled/🔣️.json")).unwrap()
}

fn refusal(error: semio_framework_value::ValueError, operation: &str) {
    assert_eq!(error.kind.as_str(), fixture()["refusalKinds"][operation].as_str().unwrap());
}

fn intrinsic(value: &serde_json::Value) -> DslValue {
    match value {
        serde_json::Value::Null => DslValue::Null,
        serde_json::Value::Bool(value) => DslValue::Bool(*value),
        serde_json::Value::Number(value) => DslValue::Number(if let Some(value) = value.as_u64() { Number::UInt(value) } else if let Some(value) = value.as_i64() { Number::Int(value) } else { Number::Float(value.as_f64().unwrap()) }),
        serde_json::Value::String(value) => DslValue::String(value.clone()),
        serde_json::Value::Array(values) => DslValue::Array(values.iter().map(intrinsic).collect()),
        serde_json::Value::Object(values) => DslValue::Object(values.iter().map(|(key, value)| (key.clone(), intrinsic(value))).collect()),
    }
}

fn projection(value: &DslValue) -> serde_json::Value {
    match value {
        DslValue::Null => serde_json::Value::Null,
        DslValue::Bool(value) => (*value).into(),
        DslValue::Number(Number::UInt(value)) => (*value).into(),
        DslValue::Number(Number::Int(value)) => (*value).into(),
        DslValue::Number(Number::Float(value)) => (*value).into(),
        DslValue::String(value) => value.clone().into(),
        DslValue::Array(values) => values.iter().map(projection).collect(),
        DslValue::Object(values) => values.iter().map(|(key, value)| (key.clone(), projection(value))).collect::<serde_json::Map<_, _>>().into(),
        DslValue::Bytes(_) => panic!("diagnostic wire has no octets"),
    }
}

fn law<T: FromValue + ToValue + std::fmt::Debug + PartialEq>(row: &serde_json::Value) {
    let input = intrinsic(&row["input"]);
    let mut accept = |_| true;
    let mut control = NativeDecodeControl::new(fixture()["stress"]["maximumBytes"].as_u64().unwrap() as usize, &mut accept);
    let actual = T::from_value_controlled(&input, &mut control);
    if !row["accepted"].as_bool().unwrap() {
        refusal(actual.unwrap_err(), "malformed");
        return;
    }
    let actual = actual.unwrap();
    assert_eq!(projection(&actual.to_value()), row["expected"], "{}", row["id"]);
    let admitted = control.owned_bytes();
    let mut exact = NativeDecodeControl::new(admitted, &mut accept);
    assert_eq!(T::from_value_controlled(&input, &mut exact).unwrap(), actual);
    if admitted > 0 {
        let mut narrow = NativeDecodeControl::new(admitted - 1, &mut accept);
        refusal(T::from_value_controlled(&input, &mut narrow).unwrap_err(), "ownership");
        assert!(narrow.owned_bytes() < admitted);
    }
    let mut accept = |_| true;
    let mut control = NativeEncodeControl::new(fixture()["stress"]["maximumBytes"].as_u64().unwrap() as usize, &mut accept);
    let encoded = actual.to_value_controlled(&mut control).unwrap();
    assert_eq!(projection(&encoded), row["expected"], "{}", row["id"]);
    assert_eq!(encoded, actual.to_value());
    let admitted = control.owned_bytes();
    let mut exact = NativeEncodeControl::new(admitted, &mut accept);
    assert_eq!(actual.to_value_controlled(&mut exact).unwrap(), encoded);
    if admitted > 0 {
        let mut narrow = NativeEncodeControl::new(admitted - 1, &mut accept);
        refusal(actual.to_value_controlled(&mut narrow).unwrap_err(), "ownership");
        assert!(narrow.owned_bytes() < admitted);
    }
    let mut cancel = |_| false;
    refusal(T::from_value_controlled(&input, &mut NativeDecodeControl::new(0, &mut cancel)).unwrap_err(), "cancel");
    let mut cancel = |_| false;
    refusal(actual.to_value_controlled(&mut NativeEncodeControl::new(0, &mut cancel)).unwrap_err(), "cancel");
}

#[test]
fn controlled_diagnostic_all_owner_corpus_and_exact_admission() {
    for row in fixture()["cases"].as_array().unwrap() {
        match row["owner"].as_str().unwrap() {
            "TextSpan" => law::<TextSpan>(row), "TextError" => law::<TextError>(row),
            "FaultCode" => law::<FaultCode>(row), "Severity" => law::<Severity>(row),
            "FaultOrigin" => law::<FaultOrigin>(row), "ExpectedSet" => law::<ExpectedSet>(row),
            "Diagnostic" => law::<Diagnostic>(row), "FaultScope" => law::<FaultScope>(row),
            "FaultCause" => law::<FaultCause>(row), "FaultParams" => law::<FaultParams>(row), "Fault" => law::<Fault>(row),
            _ => panic!("closed diagnostic owner"),
        }
    }
    println!("[DEBUG] controlled Diagnostic all eleven owners preserve invalid, quota and cancellation kinds with exact admission");
}

#[test]
fn controlled_diagnostic_independent_ajv_schema_oracle() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🎛️controlled/🔣️.json")).unwrap();
    let value_schema: serde_json::Value = serde_json::from_str(include_str!("../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json")).unwrap();
    let script = "import Ajv from 'ajv/dist/2020.js';const x=JSON.parse(await Bun.stdin.text()),ajv=new Ajv({strict:true}).addSchema(x.valueSchema).addSchema(x.schema);const rows=x.fixture.cases.map(row=>{const valid=ajv.getSchema(x.schema.$id+'#/$defs/'+row.owner);if(valid(row.input)!==row.accepted)throw Error(row.id);if(row.accepted&&!valid(row.expected))throw Error(row.id+' output');return row.accepted;});await Bun.write(Bun.stdout,JSON.stringify(rows));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"valueSchema":value_schema,"fixture":fixture()}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let actual: Vec<bool> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(actual, fixture()["cases"].as_array().unwrap().iter().map(|row| row["accepted"].as_bool().unwrap()).collect::<Vec<_>>());
}

#[test]
fn controlled_diagnostic_duplicate_key_refusal() {
    let mut accept = |_| true;
    for row in fixture()["duplicateCases"].as_array().unwrap() {
        let value = DslValue::Object(row["entries"].as_array().unwrap().iter().map(|entry| (entry[0].as_str().unwrap().to_string(), intrinsic(&entry[1]))).collect());
        let mut control = NativeDecodeControl::new(4096, &mut accept);
        let rejected = match row["owner"].as_str().unwrap() {
            "FaultCause" => FaultCause::from_value_controlled(&value, &mut control).is_err(),
            "FaultScope" => FaultScope::from_value_controlled(&value, &mut control).is_err(),
            "TextSpan" => TextSpan::from_value_controlled(&value, &mut control).is_err(),
            "FaultParams" => FaultParams::from_value_controlled(&value, &mut control).is_err() && FaultParams::from_value(value.clone()).is_err(),
            _ => panic!("closed duplicate owner"),
        };
        assert!(rejected, "{}", row["id"]);
    }
}

#[test]
fn controlled_diagnostic_long_text_lists_and_interior_cancellation() {
    let fixture = fixture(); let stress = &fixture["stress"];
    let maximum = stress["maximumBytes"].as_u64().unwrap() as usize;
    let count = stress["listCount"].as_u64().unwrap() as usize;
    let cutoff = stress["cancelAt"].as_u64().unwrap() as usize;
    let message = stress["text"].as_str().unwrap().repeat(stress["textRepetitions"].as_u64().unwrap() as usize);
    let mut fault = Fault::new(FaultOrigin::Framework, "framework.controlled", message);
    fault.causes = (0..count).map(|index| FaultCause { message: index.to_string(), code: Some(FaultCode::new("cause")) }).collect();
    let value = fault.to_value(); let mut accept = |_| true;
    let decoded = Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut accept)).unwrap();
    assert_eq!(decoded, fault);
    let mut accept = |_| true;
    assert_eq!(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut accept)).unwrap(), value);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };
    refusal(Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_encoding::NativeEncodeProgress| { if p.completed >= cutoff && p.total == fault.message.len() { reached = true; false } else { true } };
    refusal(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_decoding::NativeDecodeProgress| { if p.completed == 256 && p.total == count { reached = true; false } else { true } };
    refusal(Fault::from_value_controlled(&value, &mut NativeDecodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    let mut reached = false;
    let mut cancel = |p: semio_framework_value::native_encoding::NativeEncodeProgress| { if p.completed == 256 && p.total == count { reached = true; false } else { true } };
    refusal(fault.to_value_controlled(&mut NativeEncodeControl::new(maximum, &mut cancel)).unwrap_err(), "cancel"); assert!(reached);
    println!("[DEBUG] controlled Diagnostic long UTF-8, 1024 causes and interior cancellation retain typed authority");
}
