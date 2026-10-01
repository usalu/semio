use super::*;
use std::alloc::{GlobalAlloc, Layout, System};

struct ObservedDslAllocator;
std::thread_local! {
    static ALLOCATION_ENABLED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static ALLOCATED_BYTES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}
fn record_allocation(bytes: usize) {
    if ALLOCATION_ENABLED.try_with(|enabled| enabled.get()).unwrap_or(false) { let _ = ALLOCATED_BYTES.try_with(|count| count.set(count.get().saturating_add(bytes))); }
}
unsafe impl GlobalAlloc for ObservedDslAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 { record_allocation(layout.size()); unsafe { System.alloc(layout) } }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { record_allocation(layout.size()); unsafe { System.alloc_zeroed(layout) } }
    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, size: usize) -> *mut u8 { record_allocation(size); unsafe { System.realloc(pointer, layout, size) } }
    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) { unsafe { System.dealloc(pointer, layout) } }
}
#[global_allocator]
static DSL_TEST_ALLOCATOR: ObservedDslAllocator = ObservedDslAllocator;

#[test]
fn sqlite_snapshot_native_encoding_borrowed_object_sort_keeps_payload_allocations_bounded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let bytes = fixture["payloadBytes"].as_u64().unwrap() as usize;
    let depth = fixture["depth"].as_u64().unwrap() as usize;
    let wide = fixture["unsigned"].as_str().unwrap().parse::<u64>().unwrap();
    let bits = u64::from_str_radix(fixture["binary64Bits"].as_str().unwrap(), 16).unwrap();
    let mut value = DslValue::Object(vec![("zPayload".into(), DslValue::String("x".repeat(bytes))), ("wide".into(), DslValue::uint(wide)), ("exceptional".into(), DslValue::Number(Number::Float(f64::from_bits(bits))))]);
    for _ in 0..depth { value = DslValue::Object(vec![("z".into(), DslValue::Bool(true)), ("a".into(), value)]); }
    let spec = RecordSpec::new(None, RecordLayout::Lines, vec![FieldSpec::new(1, "value", Shape::Value)]);
    let mut record = RecordValue::default(); record.fields.insert(1, FieldValue::Value(value));
    ALLOCATED_BYTES.with(|count| count.set(0)); ALLOCATION_ENABLED.with(|enabled| enabled.set(true));
    let text = print(&record, &spec, JoinMode::Document);
    ALLOCATION_ENABLED.with(|enabled| enabled.set(false));
    let allocated = ALLOCATED_BYTES.with(|count| count.get());
    assert!(allocated <= bytes * fixture["allocationMultiplier"].as_u64().unwrap() as usize, "printing allocated {allocated} bytes for a {bytes}-byte payload at depth {depth}");
    assert!(text.find("a=").unwrap() < text.find("z=").unwrap());
    let restored = parse(&text, &spec, &ParseOptions { limits: Limits { max_bytes: bytes * 2, ..Limits::default() }, mode: SourceMode::Document }).unwrap();
    let FieldValue::Value(node) = restored.get(1).unwrap() else { panic!("owned value"); }; let mut node = node;
    for _ in 0..depth { node = node.get("a").unwrap(); }
    assert_eq!(node.get("wide").unwrap().as_u64(), Some(wide));
    assert_eq!(node.get("exceptional").unwrap().as_f64().unwrap().to_bits(), bits);
    assert_eq!(node.get("zPayload").unwrap().as_str().unwrap().len(), bytes);
    let output = std::process::Command::new("bun").args(["-e", "import {Database} from 'bun:sqlite';const db=new Database(':memory:');db.run('CREATE TABLE keys(value TEXT)');for(const key of ['zPayload','wide','exceptional'])db.run('INSERT INTO keys VALUES(?)',[key]);await Bun.write(Bun.stdout,JSON.stringify(db.query('SELECT value FROM keys ORDER BY value COLLATE BINARY').all().map(row=>row.value)));db.close();"]).output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let ordered: Vec<String> = serde_json::from_slice(&output.stdout).unwrap();
    let positions = ordered.iter().map(|key| text.find(&format!("{key}=")).unwrap()).collect::<Vec<_>>();
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(parse("value={ a=", &spec, &ParseOptions::default()).is_err());
}
