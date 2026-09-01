//! 🧪️ Raw wire pages must close with byte-exact progress before and after seal, including their backing allocation.

use super::*;
use semio_framework_job::InteractiveJobCloseStep as Step;

//#region 🧪️Retirement
#[test]
fn retained_wire_short_close_conserves_logical_bytes_and_physical_backing() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    let law = &fixture["shortClose"];
    let mut input = RetainedToolWireInput::try_new(8, 8).unwrap();
    input.admit_page(ToolWirePage::try_copy_from(&42u64.to_le_bytes()).unwrap()).unwrap();
    input.seal().unwrap();
    input.begin_close();
    let pointer = input.pages.as_ptr();
    let capacity = input.pages.capacity();
    let mut released = 0;
    for row in law["steps"].as_array().unwrap() {
        let items = usize::try_from(row["items"].as_u64().unwrap()).unwrap();
        let bytes = usize::try_from(row["bytes"].as_u64().unwrap()).unwrap();
        let count = usize::try_from(row["releasedBytes"].as_u64().unwrap()).unwrap();
        let expected = if row["blocked"].as_bool().unwrap() { Step::Blocked } else { Step::Pending { released_items: usize::try_from(row["releasedItems"].as_u64().unwrap()).unwrap(), released_bytes: count } };
        assert_eq!(input.close_step(items, bytes), expected);
        released += count;
        assert_eq!(input.admitted_bytes, usize::try_from(row["remaining"].as_u64().unwrap()).unwrap());
        assert_eq!(released + input.admitted_bytes, 8);
        assert_eq!(input.pages.as_ptr(), pointer);
        assert_eq!(input.pages.capacity(), capacity);
        assert!(!input.terminal_is_empty());
    }
    assert_eq!(input.close_step(1, 8), Step::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!(input.pages.capacity(), 0);
    assert!(input.terminal_is_empty());
    assert_eq!(input.close_step(1, 8), Step::Complete);
    eprintln!("[DEBUG] wire-short-close released={released} backing-capacity={capacity}->0 zero-grants-preserve=true");
}

#[test]
fn retained_wire_input_small_grants_retire_initialized_bytes_and_backing_allocation() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🧪️fixture/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() { for grant in fixture["grants"].as_array().unwrap() {
        let declared = row["declared"].as_u64().unwrap() as usize; let admitted = row["admitted"].as_u64().unwrap() as usize; let maximum_bytes = grant.as_u64().unwrap() as usize;
        let bytes: Vec<u8> = (0..admitted).map(|index| (index % 251) as u8).collect();
        let mut input = RetainedToolWireInput::try_new(declared, declared).unwrap();
        for page in bytes.chunks(TOOL_WIRE_PAGE_BYTES) { input.admit_page(ToolWirePage::try_copy_from(page).unwrap()).unwrap(); }
        if row["sealed"].as_bool().unwrap() { input.seal().unwrap(); assert!(input.page(0).is_some()); }
        input.begin_close(); let initial_capacity = input.pages.capacity();
        assert!(input.page(0).is_none());
        if initial_capacity > 0 { let before_pages = input.pages.len(); assert_eq!(input.close_step(0, 4096), Step::Blocked); assert!(!input.terminal_is_empty()); assert_eq!(input.pages.capacity(), initial_capacity); assert_eq!(input.pages.len(), before_pages); assert_eq!(input.admitted_bytes, admitted); }
        if maximum_bytes == 0 && admitted > 0 { assert_eq!(input.close_step(1, 0), Step::Blocked); assert_eq!(input.admitted_bytes, admitted); }
        let drain_bytes = if maximum_bytes == 0 && admitted > 0 { 1 } else { maximum_bytes }; let mut released = 0; let mut items = 0; let initial_pages = input.pages.len();
        for _ in 0..100_000 {
            let before = input.admitted_bytes; let before_capacity = input.pages.capacity(); let before_pages = input.pages.len();
            let step = input.close_step(1, drain_bytes);
            match step {
                Step::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1); assert!(released_bytes <= drain_bytes); assert_eq!(before - input.admitted_bytes, released_bytes); released += released_bytes; items += released_items;
                    assert_eq!(released_items, before_pages - input.pages.len() + usize::from(before_capacity > 0 && input.pages.capacity() == 0));
                    if before_capacity > 0 && input.pages.capacity() == 0 { assert_eq!(released_items, 1); assert_eq!(released_bytes, 0); }
                }
                Step::Blocked => panic!("positive raw close grant must progress"),
                Step::Complete => { assert!(input.terminal_is_empty()); break; }
            }
            assert_eq!(input.admitted_bytes, input.pages.iter().map(ToolWirePage::len).sum::<usize>());
        }
        assert_eq!(released, admitted); assert_eq!(items, initial_pages + usize::from(initial_capacity > 0)); assert!(input.terminal_is_empty()); assert_eq!(input.pages.capacity(), 0); assert_eq!(input.admitted_bytes, 0);
    } }
}
//#endregion 🧪️Retirement
