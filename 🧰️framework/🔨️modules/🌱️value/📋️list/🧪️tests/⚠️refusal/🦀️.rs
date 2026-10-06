use super::*;
use std::{io::Write, process::{Command, Stdio}};

struct RefusedAllocation;
impl PageAllocation for RefusedAllocation {
    fn reserve<T>(_: &mut Vec<T>, _: usize) -> Result<(), std::collections::TryReserveError> {
        Vec::<u8>::new().try_reserve_exact(usize::MAX)
    }
}
struct ExcessAllocation;
impl PageAllocation for ExcessAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize) -> Result<(), std::collections::TryReserveError> {
        owner.try_reserve_exact(slots * 2)
    }
}
fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧫️fixtures/⚠️refusal/🔣️.json")).unwrap() }
fn run(operation: &str) -> PagedListError {
    if operation == "nextEmpty" { return PagedList::<u64, 0>::default().next_allocation_bytes().unwrap_err(); }
    let mut list = PagedList::<u64, 1>::default();
    if operation == "capacity" { return list.next_capacity_allocation_bytes(2).unwrap_err(); }
    if operation == "extraRetireEmpty" { return list.truncate_retired_last().unwrap_err(); }
    if operation == "extraRelease" || operation == "extraReleaseNext" {
        list.reserve_full().unwrap(); list.push_reserved(7).unwrap();
        let refusal = if operation == "extraRelease" { list.release_empty_page(usize::MAX).unwrap_err() } else { list.next_release_allocation_bytes().unwrap_err() };
        assert_eq!(list.pop(), Some(7));
        while !list.terminal_is_empty() { let bytes = list.next_release_allocation_bytes().unwrap(); assert!(list.release_empty_page(bytes).unwrap().progressed); }
        return refusal;
    }
    if operation.ends_with("Payload") { list.reserve_one(list.next_allocation_bytes().unwrap()).unwrap(); }
    let requested = list.next_allocation_bytes().unwrap();
    let physical = list.allocated_bytes();
    let error = match operation {
        "counter" => { list.allocated = isize::MAX as usize; list.reserve_one(requested).unwrap_err() },
        "failMetadata" | "failPayload" => list.reserve_page_using::<RefusedAllocation>(requested).unwrap_err(),
        "extraMetadata" | "extraPayload" => list.reserve_page_using::<ExcessAllocation>(requested).unwrap_err(),
        "signedMetadata" | "signedPayload" => { list.allocated = isize::MAX as usize - requested; list.reserve_page_using::<ExcessAllocation>(requested * 2).unwrap_err() },
        _ => panic!("closed paged refusal operation"),
    };
    list.allocated = physical + error.allocated_bytes;
    let refusal = error.refusal();
    while !list.terminal_is_empty() {
        let bytes = list.next_release_allocation_bytes().unwrap();
        assert!(list.release_empty_page(bytes).unwrap().progressed);
    }
    assert_eq!(list.allocated_bytes(), 0);
    refusal
}

#[test]
fn controlled_value_paged_refusal_actual_allocation_kind_corpus() {
    for row in fixture()["cases"].as_array().unwrap() {
        let refusal = run(row["operation"].as_str().unwrap());
        assert_eq!(refusal.kind.as_str(), row["expected"]["kind"].as_str().unwrap(), "{}", row["id"]);
        assert_eq!(refusal.reason, row["expected"]["reason"].as_str().unwrap(), "{}", row["id"]);
    }
    println!("[DEBUG] PagedList preserves quota allocator and admission identity through actual allocation refusals");
}

#[test]
fn controlled_value_paged_refusal_independent_ajv_sqlite_oracle() {
    
    let script = "import{Database}from'bun:sqlite';const x=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');const q=db.query(\"SELECT CASE WHEN ? LIKE 'fail%' THEN 'allocationFailed' WHEN ? LIKE 'extra%' THEN 'invariantViolated' ELSE 'ownershipLimit' END AS kind\");const rows=x.fixture.cases.map(row=>q.get(row.operation,row.operation).kind);db.close();await Bun.write(Bun.stdout,JSON.stringify(rows));";
    let mut child = Command::new("bun").args(["-e", script]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();
    child.stdin.take().unwrap().write_all(serde_json::json!({"fixture":fixture()}).to_string().as_bytes()).unwrap();
    let output = child.wait_with_output().unwrap(); assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let reference: Vec<String> = serde_json::from_slice(&output.stdout).unwrap();
    for (row, kind) in fixture()["cases"].as_array().unwrap().iter().zip(reference) { assert_eq!(run(row["operation"].as_str().unwrap()).kind.as_str(), kind); }
}
