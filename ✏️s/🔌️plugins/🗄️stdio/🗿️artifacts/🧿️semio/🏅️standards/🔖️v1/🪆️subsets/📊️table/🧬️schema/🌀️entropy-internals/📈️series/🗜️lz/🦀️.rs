//! 🗜️ Lempel-Ziv complexity family: the Kaspar & Schuster (1987) incremental-parsing LZ76
//! complexity measure over arbitrary discrete symbol streams, a from-scratch LZ78 dictionary
//! [`Compressor`], and the compressor-agnostic normalized compression distance ([`ncd`]) built on
//! top of it. Zero external dependencies — no `flate2`, no `zstd`, nothing beyond `std`.

use std::collections::HashMap;

use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::EntropyError;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Estimate;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::LogBase;
use semio_s_artifact_stdio_semio_table_entropy::schema::entropy_internals::Warning;

// #region 🔖️Lz76
/// 🗜️ Kaspar & Schuster (1987) incremental-parsing complexity `c(n)`, generalized from binary
/// strings to an arbitrary `u32`-symbol alphabet. Counts the number of distinct "new" phrases a
/// greedy left-to-right parse needs to reproduce `s`, where a phrase may self-referentially copy
/// from *inside* the phrase currently being grown (not just from history already committed).
///
/// Ported verbatim (0-indexed) from the classic incremental-parsing pseudocode; the loop body is
/// fiddly and was **not** simplified. The two indices `s[i + k - 1]` / `s[l + k - 1]` are read
/// through [`slice::get`] rather than direct indexing: an out-of-range read is treated as "not
/// equal" so the mismatch branch fires instead of panicking, which is the only change from the
/// textbook pseudocode (needed because the naive index-safety invariant does not actually hold
/// for short inputs — confirmed by exhaustive brute-force cross-validation, see
/// `tests::exhaustive`).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn lz76_complexity(s: &[u32]) -> usize {
    let n = s.len();
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    let (mut i, mut k, mut l) = (0usize, 1usize, 1usize);
    let mut c = 1usize;
    let mut k_max = 1usize;
    loop {
        let equal = match (s.get(i + k - 1), s.get(l + k - 1)) {
            (Some(a), Some(b)) => a == b,
            _ => false,
        };
        if equal {
            k += 1;
            if l + k > n {
                c += 1;
                break;
            }
        } else {
            if k > k_max {
                k_max = k;
            }
            i += 1;
            if i == l {
                c += 1;
                l += k_max;
                if l + 1 > n {
                    break;
                }
                i = 0;
                k = 1;
                k_max = 1;
            } else {
                k = 1;
            }
        }
    }
    c
}

/// 🗜️ Lempel-Ziv complexity of a discrete symbol sequence. Raw mode reports the incremental-parse
/// phrase count `c(n)` directly; `normalized` mode reports `c(n) * log_alpha(n) / n` (`alpha` =
/// occupied alphabet size), which converges to `1` for a maximally complex sequence over that
/// alphabet as `n -> infinity`.
///
/// `base` on the returned [`Estimate`] is always [`LogBase::Nats`] as an inert placeholder — LZ
/// complexity is a phrase count / normalized ratio, not a log-base-dependent entropy, so
/// [`Estimate::in_base`] is meaningless here and should not be called on this result.
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn lempel_ziv_complexity(symbols: &[u32], normalized: bool) -> Result<Estimate, EntropyError> {
    if symbols.is_empty() {
        return Err(EntropyError::EmptyInput { what: "symbols" });
    }
    let n = symbols.len();
    let c = lz76_complexity(symbols);

    let mut alphabet: Vec<u32> = symbols.to_vec();
    alphabet.sort_unstable();
    alphabet.dedup();
    let alpha = alphabet.len();

    let value = if normalized {
        if alpha <= 1 {
            0.0
        } else {
            let log_alpha_n = (n as f64).ln() / (alpha as f64).ln();
            c as f64 * log_alpha_n / n as f64
        }
    } else {
        c as f64
    };

    let mut warnings = Vec::new();
    if n < 100 {
        warnings.push(Warning::SmallSample { n, recommended: 100 });
    }

    Ok(Estimate { value, base: LogBase::Nats, method: "lempel_ziv", n, n_effective: n as f64, std_error: None, ci: None, warnings, diagnostics: vec![("alphabet_size", alpha as f64), ("raw_complexity", c as f64)] })
}
// #endregion 🔖️Lz76

// #region 🔖️Compressor
/// 🗜️ A byte-stream compressor exposing only the one number [`ncd`] needs: how many bytes `data`
/// would take to represent. Kept behind a trait so [`ncd`] never depends on a specific codec's
/// implementation details.
pub trait Compressor {
    fn compressed_len(&self, data: &[u8]) -> usize;
}

/// 🗜️ Textbook LZ78 dictionary compressor (Ziv & Lempel, 1978), implemented from scratch as a
/// proxy [`Compressor`] for [`ncd`]. Not a byte-for-byte codec (there is no matching decoder) —
/// only [`Compressor::compressed_len`]'s *size estimate* is produced.
pub struct Lz78Compressor;

impl Compressor for Lz78Compressor {
    /// 🗜️ Greedily extends the current phrase while it remains a dictionary hit; each time a new
    /// (dictionary-index, literal-byte) pair is emitted, the phrase is added to the dictionary and
    /// the running bit cost grows by `ceil(log2(dictionary_size + 2))` (index) `+ 8` (literal).
    /// The dictionary is capped at 65535 entries (`u16` index space, index `0` reserved for the
    /// implicit empty root phrase); once full, matching against existing entries continues but no
    /// new phrases are memorized (standard LZW/LZ78 dictionary-full behavior). Returns the total
    /// emitted bit cost rounded up to whole bytes.
    fn compressed_len(&self, data: &[u8]) -> usize {
        if data.is_empty() {
            return 0;
        }
        const MAX_ENTRIES: usize = 65_535;
        let mut dict: HashMap<Vec<u8>, u16> = HashMap::new();
        let mut phrase: Vec<u8> = Vec::new();
        let mut total_bits: f64 = 0.0;

        let index_bits = |dict_len: usize| -> f64 {
            let addressable = dict_len as f64 + 1.0; // 🗜️ +1 for the implicit empty-phrase index 0
            (addressable + 1.0).log2().ceil().max(1.0)
        };

        for &byte in data {
            phrase.push(byte);
            if !dict.contains_key(&phrase) {
                total_bits += index_bits(dict.len()) + 8.0;
                if dict.len() < MAX_ENTRIES {
                    dict.insert(phrase.clone(), dict.len() as u16 + 1);
                }
                phrase.clear();
            }
        }
        if !phrase.is_empty() {
            // 🗜️ Trailing phrase matched an existing entry but the input ended before a new
            // extension was discovered; emit one final index-only reference (no literal).
            total_bits += index_bits(dict.len());
        }
        ((total_bits / 8.0).ceil() as usize).max(1)
    }
}
// #endregion 🔖️Compressor

// #region 🔖️Ncd
/// 🗜️ Normalized compression distance (Cilibrasi & Vitanyi, 2005):
/// `(C(xy) - min(C(x), C(y))) / max(C(x), C(y))`, a compressor-driven approximation to
/// normalized information distance in `[0, ~1]`. `compressor` decides what "compressed" means;
/// [`Lz78Compressor`] is a reasonable zero-dependency default. Generic over the compressor (O1 —
/// R11(a): a borrowed-reference parameter is trivially generic).
// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
pub fn ncd<C: Compressor>(x: &[u8], y: &[u8], compressor: &C) -> Result<f64, EntropyError> {
    if x.is_empty() {
        return Err(EntropyError::EmptyInput { what: "x" });
    }
    if y.is_empty() {
        return Err(EntropyError::EmptyInput { what: "y" });
    }
    let cx = compressor.compressed_len(x) as f64;
    let cy = compressor.compressed_len(y) as f64;
    let mut xy = Vec::with_capacity(x.len() + y.len());
    xy.extend_from_slice(x);
    xy.extend_from_slice(y);
    let cxy = compressor.compressed_len(&xy) as f64;

    let denom = cx.max(cy);
    if denom == 0.0 {
        return Ok(0.0);
    }
    Ok((cxy - cx.min(cy)) / denom)
}
// #endregion 🔖️Ncd

// #region 🔖️Tests
#[cfg(test)]
include!("🧪️tests/🔬️unit/🦀️.rs");
// #endregion 🔖️Tests
