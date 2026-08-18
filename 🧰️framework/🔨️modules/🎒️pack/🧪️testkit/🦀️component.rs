//! 🧪️ Container corruption fuzzers — truncation and bit-flip sweeps over raw `.spk` bytes.
//! DSL-free on purpose: any decoder can be swept, whatever schema it speaks.

//#region 🔖️Corrupt
/// @emoji ⏱️ How exhaustively [`fuzz_truncation`]/[`fuzz_bit_flips`] sample the corruption space —
/// mirrors the repo-wide `quick`/`long`/`exhaustive` leveled-test convention.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CorruptionLevel {
    Quick,
    Long,
    Exhaustive,
}

/// @emoji 🩹️ Outcome of a corruption-fuzz run. `cases_panicked` must be empty for a correct
/// decoder — a corrupted input is allowed to be rejected (`cases_errored`) or, rarely, to still
/// happen to decode (neither counted, since a coincidentally-valid truncation/bit-flip isn't a
/// bug), but it must never panic or abort the process.
#[derive(Clone, Debug, Default)]
pub struct CorruptionReport {
    pub cases_run: u64,
    pub cases_errored: u64,
    pub cases_panicked: Vec<String>,
}

/// @emoji 📐️ Picks up to `cap` roughly-evenly-spaced indices from `[0, total)`, always including
/// the very first and last index once `total > cap`. Shared sampling core for both the
/// truncation-length and bit-flip-position candidate lists below.
fn sample_evenly(total: usize, cap: usize) -> Vec<usize> {
    if total == 0 {
        return Vec::new();
    }
    if total <= cap {
        return (0..total).collect();
    }
    let step = total as f64 / cap as f64;
    let mut out: Vec<usize> = (0..cap).map(|i| ((i as f64 * step) as usize).min(total - 1)).collect();
    out.sort_unstable();
    out.dedup();
    out
}

fn truncation_candidates(len: usize, level: CorruptionLevel) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    match level {
        CorruptionLevel::Exhaustive => (0..len).collect(),
        CorruptionLevel::Long => sample_evenly(len, 128),
        CorruptionLevel::Quick => sample_evenly(len, 16),
    }
}

fn bit_flip_candidates(len: usize, level: CorruptionLevel) -> Vec<(usize, u8)> {
    if len == 0 {
        return Vec::new();
    }
    let total_bits = len * 8;
    let cap = match level {
        CorruptionLevel::Exhaustive => total_bits,
        CorruptionLevel::Long => 128,
        CorruptionLevel::Quick => 16,
    };
    sample_evenly(total_bits, cap).into_iter().map(|bit| (bit / 8, (bit % 8) as u8)).collect()
}

/// @emoji 💬️ Best-effort human-readable message from a `catch_unwind` payload.
fn panic_payload_message(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "non-string panic payload".to_string()
    }
}

/// @emoji 🛡️ Runs `decode` over every `(label, bytes)` case inside `catch_unwind`, silencing the
/// default panic hook for the duration (a fuzz run intentionally trips dozens of panics when it
/// finds a bug; letting the default hook print each one to stderr would drown the actual test
/// output). Restores the previous hook before returning, including on an unexpected early return.
fn run_corruption_cases(cases: impl Iterator<Item = (String, Vec<u8>)>, decode: &impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let mut report = CorruptionReport::default();
    let previous_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for (label, bytes) in cases {
        report.cases_run += 1;
        let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decode(&bytes)));
        match outcome {
            Ok(Ok(())) => {}
            Ok(Err(_)) => report.cases_errored += 1,
            Err(payload) => report.cases_panicked.push(format!("{label}: {}", panic_payload_message(payload.as_ref()))),
        }
    }
    std::panic::set_hook(previous_hook);
    report
}

/// @emoji ✂️ Truncates `valid_pack` at a sampled set of lengths (density per `level`) and calls
/// `decode` on each — proves a decoder never panics on a merely-shorter-than-expected input.
pub fn fuzz_truncation(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let lengths = truncation_candidates(valid_pack.len(), level);
    let cases = lengths.into_iter().map(|len| (format!("truncate_to_len_{len}"), valid_pack[..len].to_vec()));
    run_corruption_cases(cases, &decode)
}

/// @emoji 🔀️ Flips one bit of `valid_pack` at a sampled set of byte/bit positions (density per
/// `level`) and calls `decode` on each — proves a decoder never panics on single-bit corruption
/// (the failure mode CRC/blake3 verification exists to catch, not to crash on).
pub fn fuzz_bit_flips(valid_pack: &[u8], level: CorruptionLevel, decode: impl Fn(&[u8]) -> Result<(), String>) -> CorruptionReport {
    let positions = bit_flip_candidates(valid_pack.len(), level);
    let cases = positions.into_iter().map(|(byte_idx, bit_idx)| {
        let mut corrupted = valid_pack.to_vec();
        corrupted[byte_idx] ^= 1 << bit_idx;
        (format!("flip_byte_{byte_idx}_bit_{bit_idx}"), corrupted)
    });
    run_corruption_cases(cases, &decode)
}
//#endregion 🔖️Corrupt

