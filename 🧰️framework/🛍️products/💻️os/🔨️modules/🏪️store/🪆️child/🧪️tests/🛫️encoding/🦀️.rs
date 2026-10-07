use semio_framework_dsl_record::DslField;
use semio_framework_dsl_record::FieldValue;
use semio_framework_value::NativeEncodeControl;
use {semio_framework_artifact_reference::ArtifactRef};
use crate::os_store::ArtifactChild;
use protocol::value::ToValue;

#[test]
fn sqlite_snapshot_native_child_metadata_owns_literal_fields_under_both_native_controls() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🏭️schema/🔣️.json")).unwrap();
    let maximum = fixture["maximumBytes"].as_u64().unwrap() as usize;
    let tiny = fixture["tinyBytes"].as_u64().unwrap() as usize;
    let semio_framework_dsl_record::Shape::Record(producer) = <ArtifactChild<()> as DslField>::shape() else { panic!("declared child record") };
    let mut admitted = |_| true;
    let mut encoding = semio_framework_value::NativeEncodeControl::new(maximum, &mut admitted);
    let encoded = producer.encode(&mut encoding).unwrap();
    let exact = encoding.owned_bytes();
    assert!(exact > 0);
    let mut admitted = |_| true;
    let mut decoding = semio_framework_value::NativeDecodeControl::new(maximum, &mut admitted);
    let decoded = producer.decode(&mut decoding).unwrap();
    for record in [encoded, decoded] {
        let fields: Vec<_> = record.fields.iter().map(|field| serde_json::json!([field.id, field.key, field.optional])).collect();
        assert_eq!(serde_json::json!(fields), fixture["fields"]);
        assert!(matches!(record.fields[1].shape, semio_framework_dsl_record::Shape::Record(_)));
    }
    assert!(producer.encode(&mut NativeEncodeControl::new(exact, &mut |_| true)).is_ok());
    assert!(producer.encode(&mut NativeEncodeControl::new(exact - 1, &mut |_| true)).is_err());
    let mut admitted = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(tiny, &mut admitted);
    assert!(producer.encode(&mut control).is_err());
    assert_eq!(control.owned_bytes(), 0);
    assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(tiny, &mut |_| true)).is_err());
    assert!(producer.encode(&mut NativeEncodeControl::new(maximum, &mut |_| false)).is_err());
    assert!(producer.decode(&mut semio_framework_value::NativeDecodeControl::new(maximum, &mut |_| false)).is_err());
}

fn fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()
}
fn child(value: &serde_json::Value) -> ArtifactChild<()> {
    let t = &value["target"];
    let d = &t["dialect"];
    ArtifactChild::new(
        value["childId"].as_str().unwrap().into(),
        ArtifactRef {
            artifact_id: t["artifactId"].as_str().unwrap().into(),
            dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: d["artifactKind"].as_str().unwrap().into(), standard: d["standard"].as_str().unwrap().into(), subset: d["subset"].as_str().unwrap().into() },
        },
    )
}

fn independent_literal_oracle(value: &serde_json::Value) {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };
    
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const p=x.fixture.fieldPolicy,b=JSON.stringify(x.fixture.cases[p.caseIndex]);if(JSON.parse(b.slice(0,-1)+','+JSON.stringify(p.replacementKey)+':'+JSON.stringify(p.replacementValue)+'}').childId!==p.replacementValue)throw Error('last key policy');const db=new Database(':memory:');db.run('CREATE TABLE child(id INTEGER PRIMARY KEY,child_id TEXT NOT NULL,artifact_id TEXT NOT NULL,artifact_kind TEXT NOT NULL,standard TEXT NOT NULL,subset TEXT NOT NULL)');for(const[c,v]of x.fixture.cases.entries())db.run('INSERT INTO child VALUES(?,?,?,?,?,?)',c,v.childId,v.target.artifactId,v.target.dialect.artifactKind,v.target.dialect.standard,v.target.dialect.subset);await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT * FROM child ORDER BY id').all().map(r=>({childId:r.child_id,target:{artifactId:r.artifact_id,dialect:{artifactKind:r.artifact_kind,standard:r.standard,subset:r.subset}}}))));db.close();";
    let mut process = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    process.stdin.take().unwrap().write_all(serde_json::json!({"fixture":value}).to_string().as_bytes()).unwrap();
    let output = process.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(serde_json::from_slice::<serde_json::Value>(&output.stdout).unwrap(), value["cases"]);
}

#[test]
fn sqlite_snapshot_native_child_literal_controlled_record_preserves_every_component_without_local_materialization() {
    let value = fixture();
    independent_literal_oracle(&value);
    for case in value["cases"].as_array().unwrap() {
        let owner = std::sync::Arc::new(String::from("local materialization"));
        let child = child(case).with_local_owner(owner.clone());
        let maximum = value["control"]["maximumBytes"].as_u64().unwrap() as usize;
        let record = <ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut semio_framework_value::NativeEncodeControl::new(maximum, &mut |_| true)).unwrap();
        assert_eq!(FieldValue::Record(record), <ArtifactChild<()> as DslField>::to_value(&child));
        assert_eq!(std::sync::Arc::strong_count(&owner), 2);
        let projected = <ArtifactChild<()> as DslField>::to_value_controlled(&child, &mut semio_framework_value::NativeEncodeControl::new(maximum, &mut |_| true)).unwrap();
        assert_eq!(projected, <ArtifactChild<()> as DslField>::to_value(&child));
        assert_eq!(std::sync::Arc::strong_count(&owner), 2);
    }
}

#[test]
fn sqlite_snapshot_native_child_record_output_admits_exact_cumulative_slots_and_refuses_before_ownership() {
    let value = fixture();
    let child = child(&value["cases"][1]);
    let maximum = value["control"]["maximumBytes"].as_u64().unwrap() as usize;
    let mut admitted = |_| true;
    let mut control = semio_framework_value::NativeEncodeControl::new(maximum, &mut admitted);
    <ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut control).unwrap();
    let exact = control.owned_bytes();
    assert!(exact > child.child_id.len());
    assert!(<ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut NativeEncodeControl::new(exact, &mut |_| true)).is_ok());
    assert!(<ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut NativeEncodeControl::new(exact - 1, &mut |_| true)).is_err());
    let mut admitted = |_| true;
    let mut tiny = semio_framework_value::NativeEncodeControl::new(1, &mut admitted);
    assert!(<ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut tiny).is_err());
    assert_eq!(tiny.owned_bytes(), 0);
    assert!(<ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut NativeEncodeControl::new(maximum, &mut |_| false)).is_err());
}

#[test]
fn sqlite_snapshot_native_child_output_cancels_inside_each_literal_utf8_field() {
    let value = fixture();
    let control = &value["control"];
    let large = control["seed"].as_str().unwrap().repeat(control["repeatCount"].as_u64().unwrap() as usize);
    let maximum = control["maximumBytes"].as_u64().unwrap() as usize;
    let cancel_at = control["cancelAt"].as_u64().unwrap() as usize;
    for field in 0..5 {
        let mut child = child(&value["cases"][0]);
        match field {
            0 => child.child_id = large.clone(),
            1 => child.target.artifact_id = large.clone(),
            2 => child.target.dialect.artifact_kind = large.clone(),
            3 => child.target.dialect.standard = large.clone(),
            4 => child.target.dialect.subset = large.clone(),
            _ => unreachable!(),
        }
        let mut reached = false;
        let mut callback = |event: protocol::value::native_encoding::NativeEncodeProgress| {
            if event.total == large.len() && event.completed >= cancel_at && event.completed < event.total {
                reached = true;
                false
            } else {
                true
            }
        };
        assert!(<ArtifactChild<()> as DslField>::to_record_controlled(&child, &mut NativeEncodeControl::new(maximum, &mut callback)).is_err());
        assert!(reached, "literal field {field} must cancel during genuine copying");
    }
}

#[test]
fn sqlite_snapshot_native_child_intrinsic_output_retains_literal_value_shape_and_cumulative_controls() {
    let value = fixture();
    independent_literal_oracle(&value);
    let maximum = value["control"]["maximumBytes"].as_u64().unwrap() as usize;
    for case in value["cases"].as_array().unwrap() {
        let child = child(case);
        let mut admitted = |_| true;
        let mut control = semio_framework_value::NativeEncodeControl::new(maximum, &mut admitted);
        let output = ToValue::to_value_controlled(&child, &mut control).unwrap();
        assert_eq!(output, ToValue::to_value(&child));
        let exact = control.owned_bytes();
        assert!(ToValue::to_value_controlled(&child, &mut NativeEncodeControl::new(exact, &mut |_| true)).is_ok());
        assert!(ToValue::to_value_controlled(&child, &mut NativeEncodeControl::new(exact - 1, &mut |_| true)).is_err());
    }
}

#[test]
fn sqlite_snapshot_native_child_intrinsic_input_retains_literal_fields_without_materialization() {
    use protocol::value::{FromValue, NativeDecodeControl};
    let fixture = fixture();
    independent_literal_oracle(&fixture);
    let maximum = fixture["control"]["maximumBytes"].as_u64().unwrap() as usize;
    for case in fixture["cases"].as_array().unwrap() {
        let owner = std::sync::Arc::new(String::from("local materialization"));
        let source = child(case).with_local_owner(owner.clone());
        let intrinsic = ToValue::to_value(&source);
        let mut accepted = |_| true;
        let mut control = NativeDecodeControl::new(maximum, &mut accepted);
        let actual = <ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut control).unwrap();
        assert_eq!(actual, source);
        assert!(actual.local_owner::<String>().is_none());
        assert_eq!(std::sync::Arc::strong_count(&owner), 2);
        let exact = control.owned_bytes();
        assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut NativeDecodeControl::new(exact, &mut |_| true)).is_ok());
        if exact > 0 {
            assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut NativeDecodeControl::new(exact - 1, &mut |_| true)).is_err());
        }
    }
}
#[test]
fn sqlite_snapshot_native_child_intrinsic_input_cancels_within_each_borrowed_utf8_field() {
    use protocol::value::{FromValue, NativeDecodeControl};
    let fixture = fixture();
    let settings = &fixture["control"];
    let text = settings["seed"].as_str().unwrap().repeat(settings["repeatCount"].as_u64().unwrap() as usize);
    let maximum = settings["maximumBytes"].as_u64().unwrap() as usize;
    let cancel = settings["cancelAt"].as_u64().unwrap() as usize;
    for field in 0..5 {
        let mut source = child(&fixture["cases"][0]);
        match field {
            0 => source.child_id = text.clone(),
            1 => source.target.artifact_id = text.clone(),
            2 => source.target.dialect.artifact_kind = text.clone(),
            3 => source.target.dialect.standard = text.clone(),
            _ => source.target.dialect.subset = text.clone(),
        };
        let intrinsic = ToValue::to_value(&source);
        let mut hit = false;
        let mut callback = |p: protocol::value::native_decoding::NativeDecodeProgress| {
            if p.total == text.len() && p.completed >= cancel && p.completed < p.total {
                hit = true;
                false
            } else {
                true
            }
        };
        assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut NativeDecodeControl::new(maximum, &mut callback)).is_err());
        assert!(hit, "child literal field {field}");
        assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut NativeDecodeControl::new(1, &mut |_| true)).is_err());
        assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&intrinsic, &mut NativeDecodeControl::new(maximum, &mut |_| false)).is_err());
    }
}
#[test]
fn sqlite_snapshot_native_child_intrinsic_input_preserves_authored_field_policy() {
    use protocol::value::{FromValue, NativeDecodeControl};
    let fixture = fixture();
    independent_literal_oracle(&fixture);
    let maximum = fixture["control"]["maximumBytes"].as_u64().unwrap() as usize;
    let policy = &fixture["fieldPolicy"];
    let source = child(&fixture["cases"][policy["caseIndex"].as_u64().unwrap() as usize]);
    let protocol::value::DslValue::Object(mut fields) = ToValue::to_value(&source) else { panic!("child object") };
    fields.push((policy["replacementKey"].as_str().unwrap().into(), protocol::value::DslValue::String(policy["replacementValue"].as_str().unwrap().into())));
    let input = protocol::value::DslValue::Object(fields.clone());
    let expected = <ArtifactChild<()> as FromValue>::from_value(input.clone()).unwrap();
    let actual = <ArtifactChild<()> as FromValue>::from_value_controlled(&input, &mut NativeDecodeControl::new(maximum, &mut |_| true)).unwrap();
    assert_eq!(actual, expected);
    assert_eq!(actual.child_id, policy["replacementValue"].as_str().unwrap());
    fields.push((policy["unknownKey"].as_str().unwrap().into(), protocol::value::DslValue::Null));
    assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&protocol::value::DslValue::Object(fields), &mut NativeDecodeControl::new(maximum, &mut |_| true)).is_err());
    let mut input = ToValue::to_value(&source);
    if let protocol::value::DslValue::Object(fields) = &mut input {
        fields.retain(|(key, _)| key != policy["missingKey"].as_str().unwrap());
    }
    assert!(<ArtifactChild<()> as FromValue>::from_value_controlled(&input, &mut NativeDecodeControl::new(maximum, &mut |_| true)).is_err());
}
