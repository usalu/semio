//! 🐧️ Linux physical and cgroup memory counters.

use std::path::{Component, Path};

fn kib(text: &str, field: &str) -> Option<u64> {
    let value = text.lines().find_map(|line| line.strip_prefix(field))?;
    value.trim().strip_suffix(" kB")?.parse::<u64>().ok()?.checked_mul(1024)
}

pub(super) fn host_memory() -> Option<(u64, u64)> {
    let text = std::fs::read_to_string("/proc/meminfo").ok()?;
    let total = kib(&text, "MemTotal:")?;
    let available = kib(&text, "MemAvailable:")?.min(total);
    let groups = std::fs::read_to_string("/proc/self/cgroup").ok()?;
    let mut unified = None;
    for group in groups.lines() {
        let mut fields = group.splitn(3, ':');
        let hierarchy = fields.next()?;
        let controllers = fields.next()?;
        let path = fields.next()?;
        if hierarchy == "0" && controllers.is_empty() {
            unified = Some(path);
        }
        if controllers.split(',').any(|controller| controller == "memory") {
            return container_memory(Path::new("/sys/fs/cgroup/memory"), path, ("memory.limit_in_bytes", "memory.usage_in_bytes"), total, available);
        }
    }
    match unified {
        Some(path) => container_memory(Path::new("/sys/fs/cgroup"), path, ("memory.max", "memory.current"), total, available),
        None => Some((total, available)),
    }
}

fn container_memory(root: &Path, path: &str, files: (&str, &str), mut total: u64, mut available: u64) -> Option<(u64, u64)> {
    let path = Path::new(path.strip_prefix('/')?);
    if path.components().any(|component| !matches!(component, Component::Normal(_))) {
        return None;
    }
    if !root.is_dir() {
        return None;
    }
    let mut group = root.join(path);
    loop {
        match std::fs::read_to_string(group.join(files.0)) {
            Ok(limit) if limit.trim() == "max" => {}
            Ok(limit) => {
                let limit = limit.trim().parse::<u64>().ok()?;
                let usage = std::fs::read_to_string(group.join(files.1)).ok()?.trim().parse::<u64>().ok()?;
                total = total.min(limit);
                available = available.min(limit.saturating_sub(usage));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && (group == root || group.is_dir()) => {}
            Err(_) => return None,
        }
        if group == root {
            return Some((total, available));
        }
        group.pop();
    }
}
