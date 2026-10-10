use super::*;
use crate::{DslValue, FromValue, ToValue, NativeDecodeControl, NativeEncodeControl};
use std::{io::Write, process::{Command, Stdio}};
use crate::retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneSource, RetainedCloneProgress, RetainedCloneGrant};
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn kind(value: &str) -> ValueRefusalKind {
    match value { "invalidValue" => ValueRefusalKind::InvalidValue, "canceled" => ValueRefusalKind::Canceled, "ownershipLimit" => ValueRefusalKind::OwnershipLimit, "allocationFailed" => ValueRefusalKind::AllocationFailed, "workLimit" => ValueRefusalKind::WorkLimit, "depthLimit" => ValueRefusalKind::DepthLimit, "unsupportedOwner" => ValueRefusalKind::UnsupportedOwner, "invariantViolated" => ValueRefusalKind::InvariantViolated, _ => panic!("closed refusal kind") }
}
struct Uncontrolled;
impl ToValue for Uncontrolled { fn to_value(&self) -> DslValue { DslValue::Null } }
impl FromValue for Uncontrolled { fn from_value(_: DslValue) -> Result<Self, ValueError> { Ok(Self) } }
fn run(row: &serde_json::Value) -> ValueError {
    let mut accept_decode = |_| true; let mut accept_encode = |_| true;
    let mut cancel_decode = |_| false; let mut cancel_encode = |_| false;
    let mut decode = NativeDecodeControl::new(0, &mut accept_decode);
    let mut encode = NativeEncodeControl::new(0, &mut accept_encode);
    let error = match row["operation"].as_str().unwrap() {
        "construct" => ValueError::new(kind(row["kind"].as_str().unwrap()), row["message"].as_str().unwrap()),
        "decodeCancel" => NativeDecodeControl::new(0, &mut cancel_decode).checkpoint().unwrap_err(),
        "encodeCancel" => NativeEncodeControl::new(0, &mut cancel_encode).checkpoint().unwrap_err(),
        "decodeOwnership" => decode.charge(1).unwrap_err(), "encodeOwnership" => encode.charge(1).unwrap_err(),
        "decodeCollectionOverflow" => decode.allocate_vec::<u8>(usize::MAX).unwrap_err(), "encodeCollectionOverflow" => encode.allocate_vec::<u8>(usize::MAX).unwrap_err(),
        "decodeWork" => { decode.begin_stage(1).unwrap(); decode.advance(2).unwrap_err() },
        "encodeWork" => { encode.begin_stage(1).unwrap(); encode.advance(2).unwrap_err() },
        "decodeWorkOverflow" => { decode.begin_stage(0).unwrap(); decode.advance(usize::MAX).unwrap(); decode.advance(1).unwrap_err() },
        "encodeWorkOverflow" => { encode.begin_stage(0).unwrap(); encode.advance(usize::MAX).unwrap(); encode.advance(1).unwrap_err() },
        "decodeDepth" => decode.scoped_depth(0, |_| Ok::<(), ValueError>(())).unwrap_err(),
        "encodeDepth" => encode.scoped_depth(0, |_| Ok::<(), ValueError>(())).unwrap_err(),
        "decodeUtf8" => decode.borrow_text(&[255]).unwrap_err(),
        "decodeUnsupported" => Uncontrolled::from_value_controlled(&DslValue::Null, &mut decode).err().unwrap(),
        "encodeUnsupported" => Uncontrolled.to_value_controlled(&mut encode).unwrap_err(),
        "cloneLeaseChanged" => { let first = RetainedCloneSource::from_owner(1u32); let second = RetainedCloneSource::from_owner(1u32); let mut binding = None; first.borrow().bind(&mut binding,crate::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:first.borrow().binding_copy_bytes(),maximum_depth:1,..Default::default()}).unwrap(); let error=second.borrow().bind(&mut binding,Default::default()).unwrap_err(); for _ in 0..10000 {if binding.is_none(){break;} let close=crate::retained_clone::RetainedCloneClose::default(); let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:3,maximum_capacity_bytes:close.next_owner_capacity_with_binding::<()>(false,3,&binding).unwrap(),maximum_release_bytes:close.next_release_with_binding(&binding).unwrap(),maximum_depth:64};assert!(crate::retained_clone::RetainedCloneBinding::close_one(&mut binding,grant).unwrap().progress().fits(grant));} assert!(binding.is_none()); error },
        "cloneItemOverflow" => RetainedCloneProgress { copied_items: usize::MAX, ..Default::default() }.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() }).unwrap_err(),
        "cloneCapacityOverflow" => RetainedCloneProgress { retained_capacity_bytes: usize::MAX, ..Default::default() }.checked_add(RetainedCloneProgress { retained_capacity_bytes: 1, ..Default::default() }).unwrap_err(),
        "cloneGrantOverrun" => crate::retained_clone::admit_retained_clone_progress(RetainedCloneGrant::default(), RetainedCloneProgress { copied_items: 1, ..Default::default() }, "refusal corpus").unwrap_err(),
        "cloneDepth" => { let source = RetainedCloneSource::from_owner(Box::new(1u32)); let mut cursor = Box::<u32>::retained_clone_cursor(); let error = cursor.advance(source.borrow(), RetainedCloneGrant { maximum_items: 16, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_depth: 0, maximum_release_bytes: 65536 }).unwrap_err(); cursor.begin_close(); for _ in 0..16 { if cursor.terminal_is_empty() { break; } cursor.close_step(RetainedCloneGrant{maximum_items:16,maximum_copy_bytes:65536,maximum_capacity_bytes:cursor.next_close_capacity_byte_demand(65536).unwrap(),maximum_release_bytes:cursor.next_close_release_byte_demand().unwrap(),maximum_depth:64}).unwrap(); } assert!(cursor.terminal_is_empty()); error },
        "orderedCancel" | "orderedOwnership" => { let owner = crate::ordered::OrderedMap::<u32>::default(); let mut cursor = owner.begin_set("key".into(), 1); let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: cursor.next_capacity_byte_demand().unwrap(), maximum_release_bytes: 0, maximum_depth: cursor.next_depth_demand() }; let error = if row["operation"] == "orderedCancel" { cursor.advance_insert_controlled(grant, &mut NativeDecodeControl::new(65536, &mut cancel_decode)).unwrap_err() } else { cursor.advance_insert_controlled(grant, &mut decode).unwrap_err() }; cursor.begin_close(); for _ in 0..16 { if cursor.terminal_is_empty() { break; } let physical=RetainedCloneGrant {maximum_items:16,maximum_copy_bytes:65536,maximum_capacity_bytes:0,maximum_release_bytes:cursor.next_close_byte_demand().unwrap(),maximum_depth:cursor.next_close_depth_demand()};assert!(!matches!(cursor.close_step(physical),crate::ordered::RetirementStep::Failure(_))); } assert!(cursor.terminal_is_empty()); error },
        _ => panic!("closed actual refusal operation"),
    };
    row["path"].as_array().unwrap().iter().rev().fold(error, |error, segment| match segment.as_str() { Some(segment) => error.under(segment), None => error.under(segment.as_u64().unwrap()) })
}
#[test]
fn controlled_value_refusal_actual_kind_and_path_corpus() {
    for row in fixture()["cases"].as_array().unwrap() {
        let error = run(row);
        assert_eq!(error.kind, kind(row["expected"]["kind"].as_str().unwrap()), "{}", row["id"]);
        assert_eq!(error.kind.as_str(), row["expected"]["kind"].as_str().unwrap());
        assert_eq!(error.to_string(), row["expected"]["display"].as_str().unwrap(), "{}", row["id"]);
    }
    println!("[DEBUG] controlled Value refusal actual owners preserve eight kinds through dotted paths, including spoofed prose");
}
#[test]
fn controlled_value_refusal_independent_ajv_sqlite_path_oracle() {
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    let script = "import Ajv from 'ajv/dist/2020.js';import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const valid=new Ajv({strict:true}).compile(x.schema);const db=new Database(':memory:');db.run('CREATE TABLE path(position INTEGER PRIMARY KEY,segment TEXT NOT NULL)');const output=x.fixture.cases.map(row=>{db.run('DELETE FROM path');row.path.forEach((segment,index)=>db.run('INSERT INTO path VALUES(?,?)',[index,String(segment)]));const prefix=db.query(\"SELECT GROUP_CONCAT(segment,'.') AS prefix FROM(SELECT segment FROM path ORDER BY position)\").get().prefix;const display=db.query(\"SELECT COALESCE(? || '.', '') || ? AS display\").get(prefix,row.message).display;const result={kind:row.kind,display};if(!valid(result))throw Error(row.id);return result;});db.close();await Bun.write(Bun.stdout,JSON.stringify(output));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"schema":schema,"fixture":fixture()}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let rows: Vec<serde_json::Value> = serde_json::from_slice(&output.stdout).unwrap();
    for (row, reference) in fixture()["cases"].as_array().unwrap().iter().zip(rows) { let error = run(row); let actual = serde_json::json!({"kind":error.kind.as_str(),"display":error.to_string()}); assert_eq!(actual, reference); assert_eq!(actual, row["expected"]); }
}
