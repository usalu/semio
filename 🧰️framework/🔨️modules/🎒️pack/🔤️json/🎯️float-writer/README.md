# Design Notes

## Preserved Source Rationale

<a id="source-notes-110"></a>

### 🦀️.rs:110

#region 🔖️FloatFormat
🎯️ Exact-arithmetic correctly-rounded decimal digit generation for `f64`, used to reconcile
[`write_float`]'s digit string with `serde_json`'s (`zmij`'s) round-half-to-even tie-break at
the last significant digit — Rust's own shortest-round-trip `{:e}` formatter picks the
mathematically-correct-length, correctly-magnituded digit string, but at an exact
halfway-point tie (the true binary value sits precisely between two adjacent same-length
decimal strings, both of which round-trip) it does not consistently pick the even one the way
IEEE 754 and `zmij`'s Schubfach-based writer do. A "does a neighboring digit also round-trip"
probe was tried first and found unsound for large-magnitude values, where the round-trip basin
can hold more than two adjacent minimal-length candidates — this module instead recomputes the
digits directly from the value's exact rational form, `mantissa * 2^binary_exponent`, using a
small first-party big unsigned integer (no external bignum crate).

<a id="source-notes-350"></a>

### 🦀️.rs:350

✅️ The correctly-rounded (round-half-to-even) `digit_count`-digit decimal significand for
`magnitude`, given the leading-digit decimal exponent `decimal_exponent` — both already
established by Rust's own shortest-round-trip `{:e}` formatter, which is trusted for LENGTH
and MAGNITUDE (every case in this module's own differential sweep confirms both are already
correct); only the exact DIGITS at that fixed precision are recomputed here. Returns
`(digits, exponent_adjust)`, where `exponent_adjust` is `1` when rounding carries all the
way through (e.g. `"999"` rounds up to a truncated `"100"` with the exponent bumped), `0`
otherwise.
