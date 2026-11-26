//! 🪟️ Windows physical and commit memory counters.

#[repr(C)]
#[derive(Default)]
struct MemoryStatus {
    length: u32,
    load: u32,
    total_physical: u64,
    available_physical: u64,
    total_commit: u64,
    available_commit: u64,
    total_virtual: u64,
    available_virtual: u64,
    available_extended_virtual: u64,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GlobalMemoryStatusEx(status: *mut MemoryStatus) -> i32;
}

pub(super) fn host_memory() -> Option<(u64, u64)> {
    let mut memory = MemoryStatus { length: size_of::<MemoryStatus>() as u32, ..MemoryStatus::default() };
    if unsafe { GlobalMemoryStatusEx(&mut memory) } == 0 {
        return None;
    }
    Some((memory.total_physical, memory.available_physical.min(memory.available_commit)))
}
