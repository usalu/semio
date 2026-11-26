//! 🍎️ Darwin memory pressure and host VM counters.

unsafe extern "C" {
    fn sysctlbyname(name: *const std::ffi::c_char, value: *mut std::ffi::c_void, size: *mut usize, new: *const std::ffi::c_void, new_size: usize) -> std::ffi::c_int;
    fn host_statistics64(host: u32, flavor: i32, values: *mut i32, count: *mut u32) -> i32;
    fn mach_host_self() -> u32;
    fn mach_port_deallocate(task: u32, port: u32) -> i32;
    static mach_task_self_: u32;
    fn getpagesize() -> std::ffi::c_int;
}

fn sysctl<T: Default>(name: &std::ffi::CStr) -> Option<T> {
    let mut value = T::default();
    let mut size = size_of::<T>();
    let result = unsafe { sysctlbyname(name.as_ptr(), (&mut value as *mut T).cast(), &mut size, std::ptr::null(), 0) };
    (result == 0 && size == size_of::<T>()).then_some(value)
}

pub(super) fn host_memory() -> Option<(u64, u64)> {
    if sysctl::<u32>(c"kern.memorystatus_vm_pressure_level")? != 1 {
        return None;
    }
    let total = sysctl::<u64>(c"hw.memsize")?;
    let mut statistics = [0u64; 64];
    let words = statistics.as_mut_ptr().cast::<i32>();
    let mut count = (size_of_val(&statistics) / size_of::<i32>()) as u32;
    let host = unsafe { mach_host_self() };
    let result = unsafe { host_statistics64(host, 4, words, &mut count) };
    unsafe { mach_port_deallocate(mach_task_self_, host) };
    if result != 0 || !(35..=128).contains(&count) {
        return None;
    }
    let pages = unsafe { std::slice::from_raw_parts(words.cast::<u32>(), count as usize) };
    let speculative = u64::from(pages[23]);
    let free = u64::from(pages[0]).saturating_sub(speculative);
    let cooling = u64::from(pages[2]) + speculative;
    let cache = u64::from(pages[34]) + u64::from(pages[22]);
    let available = (free + cooling.max(cache)).checked_mul(u64::try_from(unsafe { getpagesize() }).ok()?)?;
    Some((total, available.min(total)))
}
