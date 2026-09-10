//! 🧮️ The guest's linear-memory budget and the witness that weighs it.
//!
//! 🧊️ A plugin component runs in ONE fixed wasm linear memory with ONE allocator and no threads, so
//! an owner nothing ever frees is invisible in a native suite and a hard `rust_oom` → `unreachable`
//! trap in a browser tab. [`GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`] is that memory's declared `maximum`
//! and [`GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT`] the share of it a boot + contributions-install +
//! first-mesh sequence may peak at, both pinned to `🧮️memory/🧬️schema/🔣️.json`.
//!
//! 🩺️ [`HeapWitness`] is the measuring instrument: a `GlobalAlloc` wrapper a TEST binary installs to
//! read retained and peak bytes for the whole process. Production guests never install it — on wasm
//! the same reading is free through [`guest_linear_memory_bytes`], which is one `memory.size`.

use std::sync::atomic::{AtomicIsize, Ordering};

//#region 📏️Budget
/// 📏️ Largest linear memory a plugin guest may grow to — `maximumBytes`. Linked in as
/// `wasm-ld --max-memory` by `.cargo/config.toml`'s `[target.wasm32-wasip2]` rustflags and read back
/// off the staged component's memory section by the law in `🧪️tests/🔬️memory`.
pub const GUEST_LINEAR_MEMORY_MAXIMUM_BYTES: usize = 536_870_912;

/// 📏️ Shadow stack reserved inside that memory — `stackBytes`, linked in as `wasm-ld -zstack-size`
/// by the dev plugin build (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`'s `PLUGIN_WASM_STACK_BYTES`).
pub const GUEST_LINEAR_MEMORY_STACK_BYTES: usize = 8_388_608;

/// 📏️ Share of [`GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`], in percent, the boot + contributions-install +
/// first-mesh sequence's retained-heap peak must stay under — `installPeakPercent`. Headroom for the
/// evaluation, tessellation and render work that runs AFTER the sequence a law can bound.
pub const GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT: usize = 60;

/// 📏️ [`GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT`] of the budget, in bytes — the number a law
/// compares a measured peak against, so no call site re-derives the percentage.
pub const fn guest_linear_memory_install_peak_ceiling_bytes() -> usize {
    (GUEST_LINEAR_MEMORY_MAXIMUM_BYTES as u64 * GUEST_LINEAR_MEMORY_INSTALL_PEAK_PERCENT as u64 / 100) as usize
}

/// 📏️ Largest CONTIGUOUS block a routine, per-command or per-turn guest path may request —
/// `contiguousRequestCeilingBytes`, one wasm page.
///
/// 🧊️ The guest runs on one linear memory that grows and never shrinks, served by `dlmalloc` with a
/// 64 KiB `granularity` (its `memory.grow` unit; `dlmalloc-rs` `Dlmalloc::granularity: 64 * 1024`).
/// A request at or under one granularity unit is served from a small bin, a split of `dv`/`top`, or
/// one page of growth. A larger request needs either a pre-existing large free chunk or a multi-page
/// grow, so it is the FIRST request a fragmented or nearly-full guest refuses — which is exactly how
/// build #29 of ticket 26/09/02 died: the command-ingress prologue asked for 262 272 B per command
/// at 4 Hz and answered `plugin.command-page-allocation` thirty-two times before the guest trapped.
/// A path that runs once per command or once per turn must therefore stay at or under this.
pub const GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES: usize = 65_536;

/// 📏️ Largest ASSEMBLED answer the host may deliver into a guest for ONE outstanding request —
/// `hostAnswerCeilingBytes`.
///
/// 🧨️ [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`] bounds what a SINGLE lowering asks the guest
/// allocator for; this bounds what the pages add up to. Both are needed, and for different reasons:
/// a lowering the allocator refuses is not a fault the actor can report — `cabi_realloc` answers a
/// null with `handle_alloc_error` → `abort_internal` → `unreachable`, an unrecoverable actor trap
/// raised BEFORE one instruction of guest code runs (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, boot
/// #12: every extension result trapped `procedural#1` in an endless restore loop). Past THIS
/// ceiling the guest still owns the decision and answers the request with a typed fault.
pub const GUEST_HOST_ANSWER_CEILING_BYTES: usize = 8_388_608;

/// 📄️ How many [`GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES`] pages an answer of `bytes` occupies — the
/// count a host-side pager and a guest-side law both derive from the one budget.
pub const fn guest_host_answer_pages(bytes: usize) -> usize {
    bytes.div_ceil(GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES)
}
//#endregion 📏️Budget

//#region 🧮️HeapWitness
/// 🧮️ Bytes this process has allocated and not yet freed. Signed: a `realloc` shrink and a free of
/// memory allocated before the witness was installed both legitimately push it down, so a law reads
/// DIFFERENCES of it and never its absolute value.
static RETAINED_BYTES: AtomicIsize = AtomicIsize::new(0);

/// 🏔️ Largest [`RETAINED_BYTES`] reading since the last [`reset_heap_peak`].
static PEAK_BYTES: AtomicIsize = AtomicIsize::new(0);

/// 🧮️ A `GlobalAlloc` that forwards to [`std::alloc::System`] and weighs what it hands out.
///
/// 🧵️ Process-wide, deliberately NOT per-thread: natively a mounted worker session allocates its
/// owners on a pool thread and frees them on the caller's, so a per-thread counter reads a growing
/// leak as a NEGATIVE number on the reader's own thread and reports the exact opposite of the truth.
///
/// 🚫️ Never installed by a production target: two relaxed atomics per allocation is a measurement
/// cost, and the guest this measures reads its own footprint for free with [`guest_linear_memory_bytes`].
pub struct HeapWitness;

fn record(delta: isize) {
    let retained = RETAINED_BYTES.fetch_add(delta, Ordering::Relaxed).saturating_add(delta);
    if delta > 0 {
        PEAK_BYTES.fetch_max(retained, Ordering::Relaxed);
    }
}

/// 🧮️ Retained bytes right now — the reading a growth law differences.
pub fn retained_heap_bytes() -> isize {
    RETAINED_BYTES.load(Ordering::Relaxed)
}

/// 🏔️ Peak retained bytes since the last [`reset_heap_peak`] — the reading a ceiling law compares.
pub fn peak_heap_bytes() -> isize {
    PEAK_BYTES.load(Ordering::Relaxed)
}

/// 🔄️ Arms the peak at the current retained reading, so the next measurement window starts empty.
pub fn reset_heap_peak() {
    PEAK_BYTES.store(RETAINED_BYTES.load(Ordering::Relaxed), Ordering::Relaxed);
}

unsafe impl std::alloc::GlobalAlloc for HeapWitness {
    unsafe fn alloc(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pointer = unsafe { std::alloc::GlobalAlloc::alloc(&std::alloc::System, layout) };
        if !pointer.is_null() {
            record(layout.size() as isize);
        }
        pointer
    }

    unsafe fn alloc_zeroed(&self, layout: std::alloc::Layout) -> *mut u8 {
        let pointer = unsafe { std::alloc::GlobalAlloc::alloc_zeroed(&std::alloc::System, layout) };
        if !pointer.is_null() {
            record(layout.size() as isize);
        }
        pointer
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: std::alloc::Layout) {
        record(-(layout.size() as isize));
        unsafe { std::alloc::GlobalAlloc::dealloc(&std::alloc::System, pointer, layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: std::alloc::Layout, new_size: usize) -> *mut u8 {
        let grown = unsafe { std::alloc::GlobalAlloc::realloc(&std::alloc::System, pointer, layout, new_size) };
        if !grown.is_null() {
            record(new_size as isize - layout.size() as isize);
        }
        grown
    }
}
//#endregion 🧮️HeapWitness

//#region 🌐️GuestReading
/// 🌐️ The guest's own linear memory size in bytes — one `memory.size`, free enough to read at every
/// turn boundary. `None` off wasm, where a process has no single linear memory to weigh and the
/// measurement runs through [`HeapWitness`] instead.
#[cfg(target_arch = "wasm32")]
pub fn guest_linear_memory_bytes() -> Option<usize> {
    Some(core::arch::wasm32::memory_size(0) * 65_536)
}

/// 🖥️ Off wasm there is no single linear memory: see the wasm arm above.
#[cfg(not(target_arch = "wasm32"))]
pub fn guest_linear_memory_bytes() -> Option<usize> {
    None
}

/// 📊️ Share of [`GUEST_LINEAR_MEMORY_MAXIMUM_BYTES`], in percent, a reading occupies — what a
/// diagnostics trace prints so a boot log says how close the guest is to trapping.
pub fn guest_linear_memory_percent(bytes: usize) -> usize {
    (bytes as u64 * 100 / GUEST_LINEAR_MEMORY_MAXIMUM_BYTES.max(1) as u64) as usize
}
//#endregion 🌐️GuestReading

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️memory/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
