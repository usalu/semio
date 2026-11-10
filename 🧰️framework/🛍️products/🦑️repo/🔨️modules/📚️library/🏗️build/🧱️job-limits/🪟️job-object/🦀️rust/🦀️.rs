//! 🪟️ semio-job-limit: std-only Windows Job Object limiter. `--attach <pid> --memory-mb <n> [--pids <n>] [--poll-ms <n>]`
//! puts the process in a job with a job memory limit and a kill-on-close flag, polls the peak and terminates the job at the limit.
//! NDJSON frames on stdout: ready, usage, breach; EOF on stdin releases the job.
use std::io::{Read, Write};
use std::time::Duration;

#[cfg(windows)]
mod job {
    use std::ffi::c_void;
    pub type Handle = *mut c_void;
    #[repr(C)]
    pub struct Basic { pub per_process_user_time: i64, pub per_job_user_time: i64, pub flags: u32, pub minimum_working_set: usize, pub maximum_working_set: usize, pub active_process_limit: u32, pub affinity: usize, pub priority_class: u32, pub scheduling_class: u32 }
    #[repr(C)]
    pub struct Io { pub counters: [u64; 6] }
    #[repr(C)]
    pub struct Extended { pub basic: Basic, pub io: Io, pub process_memory_limit: usize, pub job_memory_limit: usize, pub peak_process_memory_used: usize, pub peak_job_memory_used: usize }
    pub const LIMIT_ACTIVE_PROCESS: u32 = 0x8;
    pub const LIMIT_JOB_MEMORY: u32 = 0x200;
    pub const LIMIT_KILL_ON_JOB_CLOSE: u32 = 0x2000;
    pub const EXTENDED_LIMIT_INFORMATION: i32 = 9;
    pub const PROCESS_SET_QUOTA_TERMINATE: u32 = 0x0100 | 0x0001;
    extern "system" {
        pub fn CreateJobObjectW(attributes: *mut c_void, name: *const u16) -> Handle;
        pub fn SetInformationJobObject(job: Handle, class: i32, information: *mut c_void, length: u32) -> i32;
        pub fn QueryInformationJobObject(job: Handle, class: i32, information: *mut c_void, length: u32, returned: *mut u32) -> i32;
        pub fn AssignProcessToJobObject(job: Handle, process: Handle) -> i32;
        pub fn TerminateJobObject(job: Handle, code: u32) -> i32;
        pub fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
        pub fn CloseHandle(handle: Handle) -> i32;
    }
}

fn argument(name: &str) -> Option<String> {
    let arguments: Vec<String> = std::env::args().collect();
    arguments.iter().position(|row| row == name).and_then(|index| arguments.get(index + 1).cloned())
}

fn emit(line: &str) {
    let mut out = std::io::stdout().lock();
    let _ = writeln!(out, "{}", line);
    let _ = out.flush();
}

#[cfg(windows)]
fn main() {
    use job::*;
    use std::mem::size_of;
    let pid: u32 = argument("--attach").and_then(|text| text.parse().ok()).unwrap_or_else(|| std::process::exit(2));
    let megabytes: usize = argument("--memory-mb").and_then(|text| text.parse().ok()).unwrap_or_else(|| std::process::exit(2));
    let pids: u32 = argument("--pids").and_then(|text| text.parse().ok()).unwrap_or(0);
    let poll: u64 = argument("--poll-ms").and_then(|text| text.parse().ok()).unwrap_or(50);
    let limit = megabytes * 1048576;
    unsafe {
        let handle = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
        if handle.is_null() { std::process::exit(3); }
        let mut information: Extended = std::mem::zeroed();
        information.basic.flags = LIMIT_JOB_MEMORY | LIMIT_KILL_ON_JOB_CLOSE | if pids > 0 { LIMIT_ACTIVE_PROCESS } else { 0 };
        information.basic.active_process_limit = pids;
        information.job_memory_limit = limit;
        if SetInformationJobObject(handle, EXTENDED_LIMIT_INFORMATION, &mut information as *mut Extended as *mut _, size_of::<Extended>() as u32) == 0 { std::process::exit(4); }
        let process = OpenProcess(PROCESS_SET_QUOTA_TERMINATE, 0, pid);
        if process.is_null() || AssignProcessToJobObject(handle, process) == 0 { std::process::exit(5); }
        CloseHandle(process);
        let (sender, receiver) = std::sync::mpsc::channel::<()>();
        std::thread::spawn(move || { let mut sink = Vec::new(); let _ = std::io::stdin().read_to_end(&mut sink); let _ = sender.send(()); });
        emit("{\"event\":\"ready\"}");
        loop {
            if receiver.recv_timeout(Duration::from_millis(poll)).is_ok() { break; }
            let mut now: Extended = std::mem::zeroed();
            let mut returned = 0u32;
            if QueryInformationJobObject(handle, EXTENDED_LIMIT_INFORMATION, &mut now as *mut Extended as *mut _, size_of::<Extended>() as u32, &mut returned) == 0 { break; }
            let peak = (now.peak_job_memory_used + 1048575) / 1048576;
            emit(&format!("{{\"event\":\"usage\",\"peakMegabytes\":{}}}", peak));
            if now.peak_job_memory_used >= limit {
                TerminateJobObject(handle, 1);
                emit(&format!("{{\"event\":\"breach\",\"peakMegabytes\":{}}}", peak));
                break;
            }
        }
        CloseHandle(handle);
    }
}

#[cfg(not(windows))]
fn main() {
    let _ = (argument("--attach"), Duration::from_millis(1), std::io::stdin().read(&mut []));
    emit("{\"event\":\"unsupported\"}");
    std::process::exit(6);
}
