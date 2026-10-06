use super::*;
use semio_framework_value::{DslValue, FromValue, NativeDecodeControl, NativeEncodeControl, ToValue, ValueError, ValueRefusalKind};
use std::{io::Write, process::{Command, Stdio}};

fn text_refusal_fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn text_refusal_kind(value: &str) -> ValueRefusalKind {
    match value { "invalidValue" => ValueRefusalKind::InvalidValue, "canceled" => ValueRefusalKind::Canceled, "ownershipLimit" => ValueRefusalKind::OwnershipLimit, "allocationFailed" => ValueRefusalKind::AllocationFailed, "workLimit" => ValueRefusalKind::WorkLimit, "depthLimit" => ValueRefusalKind::DepthLimit, "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner, "invariantViolated" => ValueRefusalKind::InvariantViolated, _ => panic!("closed kind") }
}
fn text_refusal_span(value: &serde_json::Value) -> TextSpan { TextSpan { line: u32::try_from(value["line"].as_u64().unwrap()).unwrap(), column: u32::try_from(value["column"].as_u64().unwrap()).unwrap(), length: u32::try_from(value["length"].as_u64().unwrap()).unwrap() } }

#[test]
fn controlled_diagnostic_text_error_retains_explicit_kind_and_span_corpus() {
    let fixture = text_refusal_fixture();
    let mut outputs = Vec::new();
    for row in fixture["cases"].as_array().unwrap() {
        let kind = text_refusal_kind(row["kind"].as_str().unwrap()); let span = text_refusal_span(&row["span"]); let message = row["message"].as_str().unwrap();
        let error = match row["operation"].as_str().unwrap() {
            "new" => TextError::new(kind, message, span),
            "expected" => TextError::expected(kind, message, span, row["expectedWire"]["expected"].as_str().unwrap()),
            "fromValueError" => TextError::from_value_error(ValueError::new(kind, message), span),
            "fromDiagnostic" => { let mut diagnostic = Diagnostic::error("canceled", span, message).with_expected(ExpectedSet { tokens: vec!["=".into()], keywords: vec!["name".into()], keys: vec!["type".into()] }); diagnostic.severity = Severity::Fatal; assert_eq!(TextError::from_diagnostic(kind, diagnostic.clone()), diagnostic.clone().into_text_error(kind)); diagnostic.into_text_error(kind) },
            _ => panic!("closed operation"),
        };
        assert_eq!(error.kind, kind, "{}", row["id"]); assert_eq!(error.span, span); assert_eq!(error.to_string(), row["expectedDisplay"].as_str().unwrap());
        let mut accept = |_| true; let mut encode = NativeEncodeControl::new(1_000_000, &mut accept);
        let wire = error.to_value_controlled(&mut encode).unwrap(); let projected = serde_json::Value::from(&wire);
        assert_eq!(projected, row["expectedWire"], "{}", row["id"]);
        let mut accept = |_| true; assert_eq!(TextError::from_value_controlled(&wire, &mut NativeDecodeControl::new(1_000_000, &mut accept)).unwrap(), error);
        outputs.push(serde_json::json!({"wire":projected,"display":error.to_string()}));
    }
    for row in fixture["wireCases"].as_array().unwrap() {
        let input = DslValue::from(&row["input"]); let mut accept = |_| true;
        let error = TextError::from_value_controlled(&input, &mut NativeDecodeControl::new(1_000_000, &mut accept)).unwrap_err();
        assert_eq!(error.kind, ValueRefusalKind::InvalidValue, "{}", row["id"]);
    }
    let limits = Limits { max_bytes: 0, max_tokens: 0, max_depth: 0, max_nodes: 0 };
    for row in fixture["limits"].as_array().unwrap() {
        let source_span = TextSpan::at(7, 3);
        let error = match row["operation"].as_str().unwrap() { "bytes" => limits.check_bytes(1), "depth" => limits.check_depth(1, source_span), "tokens" => limits.check_tokens(1, source_span), "nodes" => limits.check_nodes(1, source_span), _ => panic!("closed limit") }.unwrap_err();
        assert_eq!(error.kind.as_str(), row["kind"].as_str().unwrap()); assert_eq!(error.span, text_refusal_span(&row["expectedSpan"]));
    }
    
    let diagnostic_schema: serde_json::Value = serde_json::from_str(include_str!("../../🧬️schema/🎛️controlled/🔣️.json")).unwrap();
    let value_schema: serde_json::Value = serde_json::from_str(include_str!("../../../🌱️value/⚠️refusal/🧬️schema/🔣️.json")).unwrap();
    let script = "import Ajv from'ajv/dist/2020.js';import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const a=new Ajv({strict:true}).addSchema(x.valueSchema).addSchema(x.diagnosticSchema);const v=a.getSchema(x.diagnosticSchema.$id+'#/$defs/TextError');for(const r of x.fixture.wireCases)if(v(r.input)!==r.accepted)throw Error(r.id);const db=new Database(':memory:');const q=db.query(\"SELECT ? || ' at ' || ? || ':' || ? AS display\");const output=x.fixture.cases.map(r=>({wire:r.expectedWire,display:q.get(r.message,r.span.line,r.span.column).display}));db.close();await Bun.write(Bun.stdout,JSON.stringify(output));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"diagnosticSchema":diagnostic_schema,"valueSchema":value_schema,"fixture":fixture}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr)); assert_eq!(serde_json::from_slice::<Vec<serde_json::Value>>(&output.stdout).unwrap(), outputs);
    println!("[DEBUG] TextError preserves explicit eight-kind authority and actual spans through constructors, Diagnostic conversion and controlled wire against Ajv and SQLite");
}
