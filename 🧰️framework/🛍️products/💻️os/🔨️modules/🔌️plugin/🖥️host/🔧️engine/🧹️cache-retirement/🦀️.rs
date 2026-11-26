//! 🧹️ Retirement of the compiled Wasmtime cache: an age limit and a size cap with least-recently-used
//! order, run at engine start and then rate-limited by a stamp file in the cache root.
//! @see ⚙️configuration/🔣️.json (values) and 🧬️schema/🔣️.json (meaning)

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const SHIPPED_POLICY: &str = include_str!("⚙️configuration/🔣️.json");

/// 📐️ The declared limits of the compiled cache; every value comes from the configuration JSON.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheRetirementPolicy {
    pub maximum_bytes: u64,
    pub maximum_age_ms: u64,
    pub minimum_idle_ms: u64,
    pub scratch_grace_ms: u64,
    pub minimum_interval_ms: u64,
    pub touch_interval_ms: u64,
    pub maximum_entries: usize,
    pub stamp_file: String,
}

fn json_number(text: &str, key: &str) -> Option<u64> {
    let needle = format!("\"{key}\"");
    let rest = text[text.find(&needle)? + needle.len()..].trim_start().strip_prefix(':')?.trim_start();
    rest.chars().take_while(char::is_ascii_digit).collect::<String>().parse().ok()
}

fn json_string(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let rest = text[text.find(&needle)? + needle.len()..].trim_start().strip_prefix(':')?.trim_start().strip_prefix('"')?;
    Some(rest[..rest.find('"')?].to_string())
}

impl CacheRetirementPolicy {
    /// 📖️ Reads one configuration document; a missing or malformed key is a refusal naming it.
    pub fn parse(text: &str) -> Result<Self, String> {
        let number = |key: &str| json_number(text, key).ok_or_else(|| format!("compiled cache retirement: `{key}` is missing or not a number / `{key}` fehlt oder ist keine Zahl"));
        let seconds = |key: &str| number(key).map(|value| value.saturating_mul(1000));
        let stamp_file = json_string(text, "stampFile").filter(|name| !name.is_empty() && !name.contains(['/', '\\'])).ok_or("compiled cache retirement: `stampFile` is missing or not a file name / `stampFile` fehlt oder ist kein Dateiname")?;
        if number("version")? != 1 {
            return Err("compiled cache retirement: version 1 required / Version 1 erforderlich".to_string());
        }
        Ok(Self {
            maximum_bytes: number("maximumBytes")?,
            maximum_age_ms: seconds("maximumAgeSeconds")?,
            minimum_idle_ms: seconds("minimumIdleSeconds")?,
            scratch_grace_ms: seconds("scratchGraceSeconds")?,
            minimum_interval_ms: seconds("minimumIntervalSeconds")?,
            touch_interval_ms: seconds("touchIntervalSeconds")?,
            maximum_entries: usize::try_from(number("maximumEntries")?).unwrap_or(usize::MAX),
            stamp_file,
        })
    }

    /// 🏷️ The limits this build ships with.
    pub fn shipped() -> Self {
        Self::parse(SHIPPED_POLICY).expect("the shipped compiled cache retirement configuration is valid")
    }
}

/// 📄️ One file of the cache as the retirement sees it: where it is, its bytes, when it was last used (milliseconds since the epoch) and whether it is a writer's staging file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CacheEntry {
    pub path: PathBuf,
    pub bytes: u64,
    pub used_ms: u64,
    pub scratch: bool,
}

/// ⚖️ Chooses what to retire, as indices into `entries`: staging files past their grace, complete files past the age limit, then the least recently used complete files (ties by path) until the rest fits the cap. A complete file used within `minimum_idle_ms` is never chosen. Pure.
pub fn select_retired(entries: &[CacheEntry], now_ms: u64, policy: &CacheRetirementPolicy) -> Vec<usize> {
    let idle = |entry: &CacheEntry| now_ms.saturating_sub(entry.used_ms);
    let mut retired: Vec<usize> = Vec::new();
    let mut kept_bytes: u64 = 0;
    let mut candidates: Vec<usize> = Vec::new();
    for (index, entry) in entries.iter().enumerate() {
        if entry.scratch {
            if idle(entry) > policy.scratch_grace_ms {
                retired.push(index);
            }
        } else if idle(entry) >= policy.minimum_idle_ms && idle(entry) > policy.maximum_age_ms {
            retired.push(index);
        } else {
            kept_bytes = kept_bytes.saturating_add(entry.bytes);
            if idle(entry) >= policy.minimum_idle_ms {
                candidates.push(index);
            }
        }
    }
    candidates.sort_by(|left, right| idle(&entries[*right]).cmp(&idle(&entries[*left])).then_with(|| entries[*left].path.cmp(&entries[*right].path)));
    for index in candidates {
        if kept_bytes <= policy.maximum_bytes {
            break;
        }
        kept_bytes -= entries[index].bytes;
        retired.push(index);
    }
    retired.sort_unstable();
    retired
}

/// 📊️ What one run found and did.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CacheRetirement {
    pub scanned: usize,
    pub removed_files: usize,
    pub removed_bytes: u64,
    pub failed_files: usize,
}

fn epoch_ms(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH).map_or(0, |elapsed| u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX))
}

/// 🔭️ Lists the `.cwasm` and `.scratch` files two levels below the cache root (`<engine-config-hash>/<file>`), at most `limit` of them; anything else (the cold compile lock, the stamp) is not the retirement's.
pub fn scan_compiled_cache(root: &Path, limit: usize) -> Vec<CacheEntry> {
    let mut entries = Vec::new();
    let Ok(groups) = fs::read_dir(root) else { return entries };
    for group in groups.flatten() {
        let Ok(files) = fs::read_dir(group.path()) else { continue };
        for file in files.flatten() {
            if entries.len() >= limit {
                return entries;
            }
            let name = file.file_name().to_string_lossy().into_owned();
            let scratch = name.ends_with(".scratch");
            if !scratch && !name.ends_with(".cwasm") {
                continue;
            }
            let Ok(info) = file.metadata() else { continue };
            if !info.is_file() {
                continue;
            }
            entries.push(CacheEntry { path: file.path(), bytes: info.len(), used_ms: info.modified().map_or(0, epoch_ms), scratch });
        }
    }
    entries
}

/// 🧹️ Retires what the policy selects from one cache root now; each file goes by one `remove_file`, so a file a reader still maps (Windows) stays for the next run.
pub fn retire_compiled_cache(root: &Path, now_ms: u64, policy: &CacheRetirementPolicy) -> CacheRetirement {
    let entries = scan_compiled_cache(root, policy.maximum_entries);
    let mut outcome = CacheRetirement { scanned: entries.len(), ..CacheRetirement::default() };
    for index in select_retired(&entries, now_ms, policy) {
        match fs::remove_file(&entries[index].path) {
            Ok(()) => {
                outcome.removed_files += 1;
                outcome.removed_bytes += entries[index].bytes;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => outcome.failed_files += 1,
        }
    }
    outcome
}

/// ⏱️ Runs the retirement unless the stamp file of the root says another run happened within the minimum interval (any process of the machine); the stamp is renewed before the scan so concurrent engine starts do not repeat it. `None` when it was not due.
pub fn retire_compiled_cache_if_due(root: &Path, now_ms: u64, policy: &CacheRetirementPolicy) -> Option<CacheRetirement> {
    let stamp = root.join(&policy.stamp_file);
    if let Ok(info) = fs::metadata(&stamp) {
        let last = info.modified().map_or(0, epoch_ms);
        if last <= now_ms && now_ms - last < policy.minimum_interval_ms {
            return None;
        }
    }
    if !root.is_dir() {
        return None;
    }
    fs::write(&stamp, now_ms.to_string()).ok()?;
    Some(retire_compiled_cache(root, now_ms, policy))
}

/// 👆️ Records a use of a cache entry by refreshing its modification time (at most every `touch_interval_ms`), the recency the retirement orders by; a failure only makes the entry look older.
pub fn touch_compiled_cache_entry(path: &Path, now_ms: u64, policy: &CacheRetirementPolicy) {
    let Ok(info) = fs::metadata(path) else { return };
    if now_ms.saturating_sub(info.modified().map_or(0, epoch_ms)) < policy.touch_interval_ms {
        return;
    }
    if let Ok(file) = fs::OpenOptions::new().write(true).open(path) {
        let _ = file.set_modified(SystemTime::now());
    }
}

/// 🕰️ The wall clock in milliseconds since the epoch.
pub fn now_epoch_ms() -> u64 {
    epoch_ms(SystemTime::now())
}
