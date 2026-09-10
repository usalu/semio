//! 📏️ The guest linear-memory budget, pinned against the language-neutral schema and against the
//! two link arguments that actually put it into the shipped component — `.cargo/config.toml`'s
//! `--max-memory` and the dev plugin build's `-zstack-size`. Neither may drift from
//! `🧮️memory/🧬️schema/🔣️.json`, because the number in that schema is the one every law reasons with
//! and the ones in the link arguments are the ones the browser enforces.

use super::{
    guest_host_answer_pages, guest_linear_memory_bytes, guest_linear_memory_install_peak_ceiling_bytes, guest_linear_memory_percent, peak_heap_bytes, reset_heap_peak, retained_heap_bytes,
    GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, GUEST_HOST_ANSWER_CEILING_BYTES, GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT, GUEST_LINEAR_MEMORY_MAXIMUM_BYTES, GUEST_LINEAR_MEMORY_STACK_BYTES,
};

fn schema() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧬️schema/🔣️.json")).unwrap()
}

#[test]
fn the_budget_matches_the_neutral_schema() {
    let schema = schema();
    let properties = &schema["properties"];
    assert_eq!(properties["maximumBytes"]["const"].as_u64().unwrap() as usize, GUEST_LINEAR_MEMORY_MAXIMUM_BYTES);
    assert_eq!(properties["stackBytes"]["const"].as_u64().unwrap() as usize, GUEST_LINEAR_MEMORY_STACK_BYTES);
    assert_eq!(properties["installPeakPercent"]["const"].as_u64().unwrap() as usize, GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT);
    assert_eq!(properties["contiguousRequestCeilingBytes"]["const"].as_u64().unwrap() as usize, GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
    assert_eq!(properties["hostAnswerCeilingBytes"]["const"].as_u64().unwrap() as usize, GUEST_HOST_ANSWER_CEILING_BYTES);
}

/// ⚖️ LAW: the host-answer ceiling is a whole number of contiguous-request pages, strictly above one
/// page (else paging would be pointless) and far enough under the install-peak ceiling that a
/// maximal answer cannot on its own push a settled guest past it.
#[test]
fn the_host_answer_ceiling_is_a_whole_number_of_pages_inside_the_install_headroom() {
    assert_eq!(GUEST_HOST_ANSWER_CEILING_BYTES % GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, 0);
    assert!(GUEST_HOST_ANSWER_CEILING_BYTES > GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
    assert!(GUEST_HOST_ANSWER_CEILING_BYTES < guest_linear_memory_install_peak_ceiling_bytes() / 8, "a single answer may never claim an eighth of the install-peak headroom");
    assert_eq!(guest_host_answer_pages(0), 0);
    assert_eq!(guest_host_answer_pages(1), 1);
    assert_eq!(guest_host_answer_pages(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES), 1);
    assert_eq!(guest_host_answer_pages(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES + 1), 2);
    assert_eq!(guest_host_answer_pages(GUEST_HOST_ANSWER_CEILING_BYTES), GUEST_HOST_ANSWER_CEILING_BYTES / GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
}

/// ⚖️ LAW: paging an answer must not move the oversized allocation somewhere else. A host lowers the
/// pages as one EVENT LIST, and that list is itself one contiguous `cabi_realloc` of
/// `events × <flattened record stride>` — measured at 80 B/event on `wasm32-wasip2`, so a 384 MiB
/// answer's 6 144 pages ask for 491 520 B in one block and the trap simply moves
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `🗑️generated/realloc-probe-4.txt`). At a generous
/// 256 B/event the maximal answer's list must still fit ONE page, which is what the host-answer
/// ceiling buys: every contiguous request on the whole delivery path stays inside the one bound.
#[test]
fn a_maximal_answers_own_event_list_still_fits_one_page() {
    const GENEROUS_EVENT_STRIDE_BYTES: usize = 256;
    let events = guest_host_answer_pages(GUEST_HOST_ANSWER_CEILING_BYTES) + 1;
    assert!(
        events * GENEROUS_EVENT_STRIDE_BYTES <= GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES,
        "{events} events × {GENEROUS_EVENT_STRIDE_BYTES} B exceeds the {GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES}-byte contiguous-request ceiling"
    );
}

/// ⚖️ LAW: the contiguous-request ceiling is one guest allocator growth unit — big enough that no
/// ordinary owner trips it, small enough that it is genuinely a bound on the request class a
/// fragmented, non-shrinkable linear memory refuses first.
#[test]
fn the_contiguous_request_ceiling_is_one_guest_growth_unit() {
    assert_eq!(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES, 64 * 1024, "the ceiling is dlmalloc's wasm granularity, the unit `memory.grow` moves in");
    assert!(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES < GUEST_LINEAR_MEMORY_STACK_BYTES);
    assert!(GUEST_LINEAR_MEMORY_MAXIMUM_BYTES % GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES == 0, "the budget is a whole number of guest pages");
}

/// ⚖️ LAW: the budget is what the linker is actually told. `--max-memory` is uniform across every
/// `wasm32-wasip2` plugin in `.cargo/config.toml` (per-plugin rustflags would churn cargo
/// fingerprints across the shared dep graph), and `-zstack-size` rides the dev plugin build's
/// `cargo rustc --` tail; a change to either without the schema is the drift this catches.
#[test]
fn the_link_arguments_carry_the_declared_budget() {
    let cargo_config = include_str!("../../../../../../.cargo/config.toml");
    assert!(
        cargo_config.contains(&format!("link-arg=--max-memory={GUEST_LINEAR_MEMORY_MAXIMUM_BYTES}")),
        "`.cargo/config.toml` must link wasm32-wasip2 plugin guests with --max-memory={GUEST_LINEAR_MEMORY_MAXIMUM_BYTES}"
    );
    let dev_script = include_str!("../../../../../🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts");
    assert!(
        dev_script.contains(&format!("const PLUGIN_WASM_STACK_BYTES = {} * 1024 * 1024;", GUEST_LINEAR_MEMORY_STACK_BYTES / (1024 * 1024))),
        "the dev plugin build must carve the declared shadow stack out of the guest budget"
    );
}

/// ⚖️ LAW: the install-peak ceiling is a real fraction of the budget with headroom left over, not a
/// rubber stamp that permits the whole memory.
#[test]
fn the_install_peak_ceiling_leaves_headroom_inside_the_budget() {
    assert!(GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT > 0 && GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT < 100);
    let ceiling = guest_linear_memory_install_peak_ceiling_bytes();
    assert!(ceiling < GUEST_LINEAR_MEMORY_MAXIMUM_BYTES);
    assert!(ceiling > GUEST_LINEAR_MEMORY_STACK_BYTES, "a ceiling under the shadow stack would bound nothing");
    assert_eq!(ceiling as u64, GUEST_LINEAR_MEMORY_MAXIMUM_BYTES as u64 * GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT as u64 / 100);
    assert_eq!(guest_linear_memory_percent(ceiling + 1), GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT);
    assert_eq!(guest_linear_memory_percent(GUEST_LINEAR_MEMORY_MAXIMUM_BYTES), 100);
}

/// ⚖️ LAW: the witness weighs what it is handed. Reading the counters without the allocator
/// installed must still be sound (both are plain atomics), and a reset must arm the peak at the
/// current retained reading rather than at zero.
#[test]
fn the_heap_witness_counters_are_readable_and_resettable() {
    reset_heap_peak();
    let armed = peak_heap_bytes();
    assert_eq!(armed, retained_heap_bytes());
    let held: Vec<u8> = vec![7; 1 << 20];
    assert!(!held.is_empty());
    drop(held);
    assert!(peak_heap_bytes() >= armed);
}

/// ⚖️ LAW: the guest reading exists exactly where a single linear memory does.
#[test]
fn the_guest_reading_is_wasm_only() {
    assert_eq!(guest_linear_memory_bytes().is_some(), cfg!(target_arch = "wasm32"));
}
