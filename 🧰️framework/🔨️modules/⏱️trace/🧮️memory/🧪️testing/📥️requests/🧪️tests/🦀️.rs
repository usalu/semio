//! ⚖️ Shared authored layout requests agree with System, HeapWitness, and SQLite references.
use super::*;
use std::alloc::System;
use semio_framework_trace::retained_heap_bytes_on_this_thread;

#[derive(Clone, Copy)]
enum Backend { System, Heap, Requested }
fn allocator(backend: Backend) -> &'static dyn GlobalAlloc {
    match backend { Backend::System => &System, Backend::Heap => &HeapWitness, Backend::Requested => &RequestedAllocator }
}
fn run(scope: &serde_json::Value, backend: Backend) -> usize {
    let mut pointer: *mut u8 = null_mut();
    let mut layout = Layout::from_size_align(0, 8).unwrap();
    let mut total = 0;
    for operation in scope["operations"].as_array().unwrap() {
        let op = operation["op"].as_str().unwrap();
        if op == "scope" { total += measure(operation, backend); continue; }
        let before = retained_heap_bytes_on_this_thread();
        let old = layout.size();
        let bytes = operation["bytes"].as_u64().unwrap_or(0) as usize;
        unsafe {
            match op {
                "allocate" | "zeroed" => {
                    assert!(pointer.is_null());
                    layout = Layout::from_size_align(bytes, 8).unwrap();
                    pointer = if op == "zeroed" { allocator(backend).alloc_zeroed(layout) } else { allocator(backend).alloc(layout) };
                    assert!(!pointer.is_null());
                    if op == "zeroed" { assert!(std::slice::from_raw_parts(pointer, bytes).iter().all(|byte| *byte == 0)); }
                    std::ptr::write_bytes(pointer, 0xa5, bytes);
                }
                "reallocate" => {
                    assert!(!pointer.is_null());
                    pointer = allocator(backend).realloc(pointer, layout, bytes);
                    assert!(!pointer.is_null());
                    assert!(std::slice::from_raw_parts(pointer, old.min(bytes)).iter().all(|byte| *byte == 0xa5));
                    layout = Layout::from_size_align(bytes, 8).unwrap();
                    std::ptr::write_bytes(pointer, 0xa5, bytes);
                }
                "deallocate" => { assert!(!pointer.is_null()); allocator(backend).dealloc(pointer, layout); pointer = null_mut(); }
                _ => panic!("unknown authored allocation operation"),
            }
        }
        if matches!(backend, Backend::System) { assert_eq!(retained_heap_bytes_on_this_thread(), before); }
        if matches!(backend, Backend::Heap) {
            let delta = retained_heap_bytes_on_this_thread() - before;
            let expected = match op { "reallocate" => bytes as isize - old as isize, "deallocate" => -(old as isize), _ => bytes as isize };
            assert_eq!(delta, expected);
        }
        if op != "deallocate" { total += bytes; }
    }
    assert!(pointer.is_null());
    total
}
fn measure(scope: &serde_json::Value, backend: Backend) -> usize {
    let total = if matches!(backend, Backend::Requested) {
        let (total, requested) = observe(|| run(scope, backend));
        assert_eq!(requested, total);
        requested
    } else { run(scope, backend) };
    assert_eq!(total, scope["expectedRequestedBytes"].as_u64().unwrap() as usize);
    total
}

#[test]
fn shared_requested_layout_scopes_preserve_totals_nesting_and_unwind() {
    let source = include_str!("../🧫️fixtures/🔣️.json");
    let fixture: serde_json::Value = serde_json::from_str(source).unwrap();
    let schema: serde_json::Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).unwrap();
    assert_eq!(schema["properties"]["cases"]["items"]["$ref"], "#/$defs/scope");
    for scope in fixture["cases"].as_array().unwrap() { for backend in [Backend::System, Backend::Heap, Backend::Requested] { measure(scope, backend); } }
    let interrupted = std::panic::catch_unwind(|| observe(|| { let _ = observe(|| panic!("authored observer unwind")); }));
    assert!(interrupted.is_err());
    let probe = fixture["unwindProbeBytes"].as_u64().unwrap() as usize;
    let (_, requested) = observe(|| { let layout = Layout::from_size_align(probe, 8).unwrap(); unsafe { let pointer = RequestedAllocator.alloc(layout); assert!(!pointer.is_null()); RequestedAllocator.dealloc(pointer, layout); } });
    assert_eq!(requested, probe);
    let reference = std::process::Command::new("bun").args(["-e", r#"import {Database} from 'bun:sqlite';const f=JSON.parse(await Bun.stdin.text());const db=new Database(':memory:');const totals=f.cases.map(scope=>db.query("SELECT SUM(CAST(value AS INTEGER)) AS total FROM json_tree(?) WHERE key='bytes'").get(JSON.stringify(scope)).total);await Bun.write(Bun.stdout,JSON.stringify(totals));db.close();"#]).stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).stderr(std::process::Stdio::piped()).spawn().unwrap();
    let mut reference = reference;
    std::io::Write::write_all(&mut reference.stdin.take().unwrap(), source.as_bytes()).unwrap();
    let output = reference.wait_with_output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let totals: Vec<usize> = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(totals, fixture["cases"].as_array().unwrap().iter().map(|scope| scope["expectedRequestedBytes"].as_u64().unwrap() as usize).collect::<Vec<_>>());
    eprintln!("[DEBUG] Shared requested allocation scopes retain layout totals, inclusive nesting, unwind restoration, System/HeapWitness and SQLite references");
}

#[test]
fn shared_requested_overflow_delegates_bounded_allocation_then_restores_before_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let probe = fixture["overflowProbeBytes"].as_u64().unwrap() as usize;
    let overflow = std::panic::catch_unwind(|| observe(|| {
        observe(|| {
            ACTIVE.with(|active| unsafe { (*active.get()).requested = usize::MAX; });
            let layout = Layout::from_size_align(probe, 8).unwrap();
            unsafe {
                let pointer = RequestedAllocator.alloc(layout);
                assert!(!pointer.is_null());
                RequestedAllocator.dealloc(pointer, layout);
            }
            ACTIVE.with(|active| unsafe {
                let scope = active.get();
                assert!((*scope).overflow);
                assert_eq!((*(*scope).parent).requested, probe);
                assert!(!(*(*scope).parent).overflow);
            });
        });
    }));
    assert!(overflow.is_err());
    assert!(ACTIVE.with(Cell::get).is_null());
    let (_, requested) = observe(|| {
        let layout = Layout::from_size_align(probe, 8).unwrap();
        unsafe {
            let pointer = RequestedAllocator.alloc(layout);
            assert!(!pointer.is_null());
            RequestedAllocator.dealloc(pointer, layout);
        }
    });
    assert_eq!(requested, probe);
    eprintln!("[DEBUG] Synthetic requested-counter overflow delegates a bounded allocation, counts its parent, and restores observation before refusal");
}
