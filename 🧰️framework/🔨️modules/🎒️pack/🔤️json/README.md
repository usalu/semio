# JSON


## Original JSON Ownership Notes

### ✍️text-writer/🦀️.rs

#region 🔖️Writer
📝️ Writes `value` as compact JSON.
🚫️async: R9 pure in-memory format, no I/O.
🪐️ Writes `value` as 2-space-indented JSON, matching `serde_json::to_string_pretty`'s own
layout (`": "` after object keys, one array/object member per line, no trailing newline) — the
`serde_json::to_string_pretty` replacement every human-facing JSON view (an example-document
viewer, an exported fixture, a rule inspector) still needs even after its call site stops
depending on `serde_json` for everything else.
🚫️async: R9 pure in-memory format, no I/O.
🪞️ `value.to_string()`/`format!("{value}")`, mirroring `serde_json::Value`'s own `Display` —
every plugin call site that used to write `serde_json::json!({...}).to_string()` keeps
compiling unchanged against `pack::json!({...}).to_string()`.
🎛️ Escapes `"`, `\`, and control characters (`\b \f \n \r \t` shorthands, `\u00XX` otherwise);
everything else — including non-ASCII — passes through unescaped, matching `serde_json`'s
default (`ensure_ascii`-off) writer.
🔧️ `serde_json`'s own (`zmij`-based) fixed/exponential split for `f64`: fixed notation for
`-5 <= exponent <= 15` on the leading-digit decimal exponent, exponential otherwise — traced
by hand from `zmij 1.0.21`'s `write<Float>` (`~/.cargo/registry/…/zmij-1.0.21/src/lib.rs`,
the `(-5..=15).contains(&dec_exp)` guard) and confirmed against the pinned resolved version in
this workspace's own `Cargo.lock`. Not ECMA-262's `-6 <= e < 21` (that was this writer's prior,
wrong rule — ties in the last significant digit alone hid the boundary-case fanout it caused).
A whole-number float always gets an explicit `.0` in fixed notation so it never collapses onto
its integer twin on the wire. See `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`.
#endregion 🔖️Writer

### 🌉️dsl-value-bridge/🦀️.rs

#region 🔖️DslValueBridge
🌱️ Structural conversion from `semio_framework_value::DslValue` (the in-memory tree
`ToValue`/`FromValue` target onto — see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/
📓️serde-replacement-surface.md` §"pack::json::Value ↔ DslValue conversion") into this crate's
own JSON-text-oriented [`Value`] — the two are sibling shapes with no shared type, so a
`Mutation`/`MutationDiff` payload that needs literal JSON **text** (a wire byte string, not
just an in-memory value) walks through here on the way to [`to_string`]/[`parse`].
Maps `semio_framework_value::Number`'s `UInt`/`Int`/`Float` variants onto this crate's own
identically-shaped [`Number`] one-for-one — an integer stays an integer across the bridge
instead of being widened to `f64` and printed back with a spurious `.0`.
⏪️ The reverse of [`from_dsl_value`] — maps numbers variant-for-variant, so an integer stays an
integer across the bridge instead of being widened to `f64` and printed back as `1.0`.
#endregion 🔖️DslValueBridge

### 🎛️controlled-and-borrowed-bridge/🦀️.rs

#region 🔖️ToFromValueBridge
🔣️ `serde_json::to_string`/`from_str` analogs over [`ToValue`]/[`FromValue`] instead of
`Serialize`/`DeserializeOwned` — layered on the structural [`from_dsl_value`]/[`to_dsl_value`]
walk above (`//#region 🔖️DslValueBridge`) rather than a second one: a concurrent session
landed that walk in this same file while this one was in flight, so this region only adds the
generic string convenience pair it didn't have, instead of a duplicate `DslValue <-> Value`
conversion. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: the
pair every plugin routes JSON text through once it stops deriving
`serde::Serialize`/`Deserialize` in favor of `ToValue`/`FromValue`.
🔤️ `serde_json::from_str` analog over [`FromValue`] instead of `DeserializeOwned` — a parse
failure and a decode failure both collapse onto [`ValueError`], matching `from_value`'s own.
🚦️ Parses and binds JSON under the caller's cumulative native ownership control.
🛫️ Writes ordered JSON under the caller's cumulative native ownership control.
🔎️ A canonical JSON scalar or collection borrowed directly from the retained source owner.
🌱️ Source owners provide ordinal views without projecting or copying their payload first.
🧵️ Retains physical writer state while borrowing the unchanged semantic owner per grant.
▶️ Starts without copying or scanning a source owner.
📍️ Returns the current canonical output byte count and measure/write phase.
⚖️ Borrows the complete measured extent before output storage is admitted.
🔄️ Reuses only paid measure frames after their previous source walk reached its frontier.
⏱️ Advances at most the supplied structural or scalar-character transitions.
📏️ Quotes one actual scalar write, natural frame birth or final output allocation.
🌳️ Owns its semantic source and the canonical borrowed writer state.
#endregion 🔖️ToFromValueBridge

### 📥️decode/🧳️receiving/🦀️.rs

🛡️ The installed wire projection determines canonical operation bytes; its original decoder ledger remains fixed.
📐️ Captures only original wire custody, codec source and issuer aliases before reading any bytes.
🎟️ A denied constructor hands the same wire projection back without decoding or materialization.
📏️ The next physical frontier is quoted from the same original parser, receiver or canonical source.
⏯️ Each granted action preserves the same decode ledger and original byte source.
📤️ The returned native operation projection keeps its original typed source and issuer alive.
🫴️ The same unique handoff continues after its first funded alias-close step.
📐️ Quotes one unique original handoff action after canonical validation, preserving all wire cleanup owners.
📤️ Transfers the same uniquely retained native value while retaining its parser, bytes and actual issuer for closure.
📏️ Moving the actual received value closes canonical and projection aliases before unique source transfer.
📤️ Returns the same received operation and exact installed issuer without a second payload clone.
🛑️ Cancellation retains parser, typed result, canonical cursor and every original source alias.
📪️ Cleanup quotes its defining typed owners instead of retiring payloads through a cold fallback.

### 📥️decode/🧳️receiving/🧪️tests/🦀️.rs

📤️ The owned terminal returns the same original native value and issuer only after all output aliases close.

### 📥️decode/🧵️operation/🦀️.rs

🛂️ Caller-authored total parse and cleanup authority accompanies the complete source limits.
🪪️ One retained operation conserves its original source, control, receipts and partial candidate.
🎟️ Binds the complete caller policy and original control without scanning or allocating.
🪟️ Borrows the original admitted result without moving its storage or cleanup authority.
🚨️ Borrows the original diagnostic and duplicate-name backing until physical cleanup admission.
⏱️ Intersects one caller turn with the unchanged remaining total parse allowance.
♻️ Debits actual cleanup receipts while retaining original ownership on every refusal.

### 📥️decode/🫳️borrowed/🦀️.rs

🔣️ Binds the retained grammar to one exact immutable source borrow until result or retirement handoff.
🪟️ Retains an immutable first-party source view by value alongside its original grammar owner.
🛂️ Captures the complete original caller limits before any source read or destination allocation.
🪪️ Transfers the original immutable source identity with its native grammar owner.
🪆️ Restores the same source and grammar after a refused ownership handoff.
⏱️ Forwards the exact selected grammar work, including a refused operation.
🎟️ Admits the native grammar retirement frame before transferring its original owner.
📏️ One immutable finite source supplies original octets for every retained parse step.
🧬️ Closed first-party semantic destinations move each original admitted JSON cell exactly once.
📖️ Original caller policy bounds the complete source, cumulative storage and each declared collection extent.

### 🔢️number/🦀️.rs

#region 🔖️Number
👁️ A JSON number, keeping the writer's-eye distinction JSON itself does not: an integer
literal (`UInt`/`Int`) round-trips without a decimal point, a `Float` always carries one (or an
exponent) so `42` and `42.0` are never confused on the wire.
🔎️ Widens to `f64` regardless of variant — lossy for `u64`/`i64` magnitudes beyond 2^53.
🎯️ Exact `i64`, only for the `Int` variant and `UInt` values that fit.
❎️ Exact `u64`, only for the `UInt` variant and non-negative `Int` values.
📍️ Whether this literal was written without a decimal point or exponent.
#endregion 🔖️Number

### 🔤️lexer/🦀️.rs

#region 🔖️Lexer
🪙️ One structural token — the streaming layer everything else is built on. A future
chunked-`Read` streaming API would produce these incrementally across buffer refills; today's
`Lexer` already tokenizes without materializing the whole document as a DOM first, it just still
requires the whole input as one contiguous `&str` (every framework consumer today hands over a
short handcrafted schema leaf, never something worth reading in chunks).
🔤️ Token-at-a-time reader over a `&str` — zero-copy for structural bytes, allocating
only for string/number token payloads.
🔣️ Retains the original allocation-free RFC8259 numeric scanner under caller-authored turns.
🌱️ Begins one exact numeric TEXT field without scanning, copying, or allocating.
📍️ Reports the actual original numeric source position.
🧮️ Performs one original numeric grammar transition into the retained scalar owner.
🔎️ Current byte offset into the input.
🔢️ Reads one JSON number literal per RFC 8259 §6: `-? int frac? exp?`, no leading zeros.
Falls back to `Number::Float` whenever a fractional part, exponent, or `u64`/`i64` overflow
is present — identical to `serde_json` with its `float_roundtrip` oracle configuration and
without `arbitrary_precision` (the only configuration this repo ever builds with).
🧵️ Reads the 4 hex digits of one `\uXXXX` escape (already past the `u`).
💯️ Reads a full JSON string literal (opening quote must be at `self.pos`). Handles every
short escape, `\uXXXX`, and UTF-16 surrogate pairs for supplementary-plane characters —
rejects a lone (unpaired) surrogate rather than silently producing an invalid `char`.
📤️ Reads the next structural or scalar token, or `None` at end of input.
#endregion 🔖️Lexer

### 🦀️.rs

`serde_json` inside framework crates (see `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️20/
INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-9-RUNTIME-DEPENDENCY-REMOVAL/📓️p9b-owned-serialization.md`).

Whole-document parsing drains the same retained [`JsonParseCursor`] used by interactive owners.
The cursor borrows unchanged source text on each grant and keeps partial candidate ownership;
[`JsonValueProjection`] moves that candidate into domain-neutral values without payload clones.
The token-at-a-time [`Lexer`] shares the same string and number grammar. Arbitrary-precision
integers are excluded (JSON numbers outside `[i64::MIN, u64::MAX]` fall back to `f64`, exactly like
`serde_json` without its `arbitrary_precision` feature — the only configuration this repo ever
built with).

Number formatting is byte-identical to `serde_json`'s own (`zmij`-based) writer for every
`f64`: the fixed/exponential split follows `zmij`'s rule (`-5 <= exponent <= 15` stays fixed),
not ECMA-262's `Number::toString` (`-6 <= e < 21`) — the two differ, and only `zmij`'s matches
this repo's actual `serde_json` dependency — and the digits themselves are recomputed by exact
round-half-to-even arithmetic so a genuine last-digit tie always resolves the same way `zmij`
resolves it (proof: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/
RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/🔍️research/📓️float-format-parity.md`).
A float and an integer of the same magnitude are still never spelled the same way (`42.0`
always keeps its `.0`). NaN/±Infinity — not representable in JSON — encode as `null`, matching
`serde_json`'s own behaviour (verified by the differential tests below).
#region 🔖️Parser
#endregion 🔖️Parser
#region 🔖️ToFromValueBridge
#endregion 🔖️ToFromValueBridge
#region 🔖️Macro
🧩️ `serde_json::json!` replacement — an object/array literal builder over [`Value`], expanded
via the standard TT-muncher recursion (see `json_object_internal!`/`json_array_internal!`,
`#[doc(hidden)]`, exported only so this macro's own expansion can call them from any crate).
Object keys are string literals (`"key": value`); leaf expressions are borrowed and encoded
through [`ToValue`], preserving access to records and fields after constructing the JSON tree.
#endregion 🔖️Macro

### 🧩️members/🦀️.rs

🧩️ Explicit authority for repeated decoded JSON object member names.

### 🧮️parser-and-grammar-cursor/🦀️.rs

#region 🔖️Parser
🌳️ Parses one whole JSON document from `input`, rejecting trailing non-whitespace bytes.
🚫️async: R9 pure in-memory parse, no I/O.
📖️ [`parse`] over raw bytes — errors with [`JsonError::InvalidUtf8`] if `input` is not UTF-8.
🧵️ Retains the canonical JSON grammar and admitted candidates while borrowing unchanged source.
🖱️ One retained grammar moves admitted semantic cells directly into its declared first-party output.
🌱️ Starts parsing without copying, scanning, or allocating for the source.
📍️ Returns the measured source byte offset.
🧭️ Identifies the existing grammar or physical candidate frontier.
⏱️ Advances admitted grammar transitions under this operation's cumulative decode authority.
⏱️ Actual selected UTF-8 or grammar work remains visible when the operation refuses.
📏️ Quotes the actual next original semantic or physical frontier before any effect.
🎒️ Moves a parsed JSON candidate into admitted canonical values without payload clones.
🎚️ Takes ownership of the existing parsed candidate.
🧬️ Projects the same candidate with canonical member order for dependency identity.
📥️ Exposes the current admitted value or consumed input frontier.
🪜️ Moves admitted values and drains consumed input under the same work and decode controls.
👣️ Reads actual admitted output birth and original input backing release independently.
#endregion 🔖️Parser

### 🧱️object-and-value/🦀️.rs

#region 🔖️Value
🗂️ An insertion-order-preserving JSON object. Re-inserting an existing key overwrites
its value in place (last-value-wins) rather than moving it to the end — the same externally
observable behaviour as `serde_json::Map`'s default `BTreeMap` backing, just order-preserving
for the common no-duplicate case instead of key-sorted.
🗑️ Removes `key` and returns its value, preserving the insertion order of the rest.

Mirrors `serde_json::Map::remove` so a recursive key-at-a-time retirement walk can be
expressed against this type without reaching for serde_json.
✏️ Mutable iteration in insertion order, for in-place rewrites of nested values.
🪪️ Borrows the last owned member and its actual key allocation before a bounded retirement grant.
🌿️ Borrows the last descendant without transferring or cloning its owner.
🫴️ Transfers the last member's real key and value allocations in constant time.
📏️ The actual retained member-vector backing, excluding independently owned descendants.
⛔️ Transfers the actual owned member storage in insertion order without copying keys or descendants.
🌳️ An owned JSON value tree — the `serde_json::Value` replacement. Every framework
consumer of dynamically-shaped JSON (schema leaves, protocol probes) reads/writes this type.
🖊️ `serde_json::Value::as_array`'s own signature (`Option<&Vec<Value>>`, not a bare slice) —
on purpose: `Vec<Value>: Clone` lets `.and_then(Value::as_array).cloned()` call sites that
used to target `serde_json::Value` keep compiling unchanged (`[Value]` alone is unsized and
has no `Clone`).
🔎️ Object-field lookup, mirroring `serde_json::Value::get` — `None` on any non-object.
📇️ Array-element lookup by position, mirroring `serde_json::Value::get(usize)` —
`None` on any non-array or out-of-bounds index.
🧭️ RFC 6901 JSON Pointer lookup, mirroring `serde_json::Value::pointer` — the empty string
resolves to `self`; a non-empty pointer must start with `/`, and each `/`-separated segment
is unescaped (`~1` -> `/`, `~0` -> `~`) before being tried as an object key, then as an
array index. `None` on a malformed pointer, a missing key, or an out-of-range index.
🗝️ `value["key"]`, mirroring `serde_json::Value`'s own `Index<&str>` — panics if `self` is
not an object, returns [`Value::Null`] for a missing key (never panics on a missing key,
matching `serde_json`'s own permissive lookup semantics for assertions/fixtures).
🔢️ `value[index]`, mirroring `serde_json::Value`'s own `Index<usize>` — panics if `self` is
not an array, returns [`Value::Null`] for an out-of-bounds index.
🪞️ Cross-type equality against Rust primitives, mirroring `serde_json::Value`'s own
`impl_value_eq!` family — lets `assert_eq!(value["key"], "literal")`/`value == 3.0` read
exactly like the `serde_json` call sites they replace, with no `.as_str()`/`.as_f64()` unwrap
noise at the assertion site.
🏗️ Builds a [`Value::Array`] from any iterator of values.
🔨️ Builds a [`Value::Object`] from any iterator of `(key, value)` pairs.
⚖️ Structural equality that ignores object key order — [`Object`]'s derived `PartialEq`
(insertion-order `Vec`-backed) is order-sensitive, unlike `serde_json::Map`'s default
key-sorted `BTreeMap` backing; a decode→re-encode round trip through [`crate::json::to_json_string`]
naturally reorders fields to Rust struct declaration order, so a "committed JSON is already
canonical" style assertion needs this, not `==`, to match the old serde-era test semantics.
#endregion 🔖️Value

### 🚨️json-error/🦀️.rs

#region 🔖️Errors
🚨️ Every parse failure this crate can produce, with a byte offset into the input.
🧭️ Retains parse semantics at the owned value refusal boundary.
🌱️ Carries parse refusal into typed native construction.
🛡️ Recursion ceiling for nested arrays/objects — matches `serde_json`'s own default
(128), the value this repo's fixtures were authored against.
#endregion 🔖️Errors

### 🛫️encode/🔣️scalar/🦀️.rs

🧬️ A borrowed typed JSON node; callers cannot supply encoded bytes or a digest.
🔤️ An unsigned 64-bit integer spelled as its canonical decimal JSON string, for wires that carry ids beyond 2^53.
📖️ A signed 64-bit integer spelled as its canonical decimal JSON string, for wires that carry exact signed64 values.
🔢️ A binary32 spelled as its IEEE-754 bit pattern in eight lowercase hexadecimal digits inside a JSON string, for wires that must carry exact floats.
🔓️ Every arm but `F32` is now serde-free: `null`/`bool`/plain-decimal integers have a
single unambiguous JSON spelling (no shortest-round-trip question the way floats have), so
they are written directly; `F64` routes through the allocation-free `pack::json::write_float_to`, proven
byte-identical to `serde_json`'s own `f64` writer for every value (`.🧬semio/🦑️repo/
🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
🔍️research/📓️float-format-parity.md`). `F32` stays on `serde_json` — that proof covers only
`f64`, and `zmij`'s `f32` path uses a materially different threshold/precision budget this
ticket did not verify.
✍️ Initializes only the real scalar prefix inside its already retained inline backing.

### 🛫️encode/🔤️text/🦀️.rs

🔡️ Projects one canonical JSON escape byte without materializing an escaped string.
🔣️ Writes the same escape projection into a caller-owned six-byte scratch cell.

### 🛫️encode/🧭️tree/📋️persistent/🧪️tests/🦀️.rs

📋️ Real persistent and sealed native trees borrow original rows and encode exact canonical arrays through cancellation.

### 🛫️encode/🧭️tree/📨️wire/🦀️.rs

🧾️ Physical receipt accompanies every optional output, including denied or metadata-only turns.
🧳️ The same typed root is measured and written without a value projection or payload clone.
📥️ Transfers only an existing source header into inline custody; no backing is allocated here.
🫴️ Borrows the exact typed source while its original authority remains live.
📏️ Quotes the identical next selected body, allocator, alias or full-close event.
✍️ Performs one bounded event; any refusal leaves every remaining original owner available for paid close.
♻️ Closes the exact aliases, typed body and partial wire backing under the supplied actual grant.

### 🛫️encode/🧭️tree/🦀️.rs

🌲️ Direct native child ordinals preserve original record fields and collection order.
🔤️ A `u64` whose canonical JSON wire is its decimal string (`"18446744073709551615"`), for ids beyond 2^53.
🔗️ Views a native `u64` field in place as its decimal-string role, so `#[canonical_json(decimal_string)]` projects the original field without a copy.
🖊️ A `i64` whose canonical JSON wire is its decimal string (`"-9223372036854775808"`), for exact signed64 values.
🖋️ Views a native `i64` field in place as its decimal-string role, so `#[canonical_json(decimal_string)]` projects the original field without a copy.
🔢️ An `f32` whose canonical JSON wire is its IEEE-754 bit pattern as eight lowercase hexadecimal digits in a string (`"3fc00000"`), for exact binary32 words.
🔡️ Views a native `f32` field in place as its hex-word role, so `#[canonical_json(hex_word)]` projects the original field without a copy.
🧮️ A fixed `[f32; N]` whose canonical JSON wire is an array of hex-word strings.
🗃️ Views a native `[f32; N]` field in place as its hex-word array role.
📋️ A `Vec<f32>` whose canonical JSON wire is an array of hex-word strings.
🗒️ Views a native `Vec<f32>` field in place as its hex-word list role.
🗝️ String-like ordered keys render as JSON object members in byte order; ordinal access walks the live ordered map.
🔐️ First-party retained dictionaries lend their original ranked key and value without allocating a projection.
🗺️ Ordered immutable maps render as JSON objects in their own rank order, matching their `ToValue` wire; ordinal access is one bounded ranked visit.
🔃️ Ordered immutable string sets render as JSON string arrays in rank order, matching their `ToValue` wire.
🗄️ Hashed maps render in canonical byte-ordered key order, independent of their hasher, by ranking each key without allocating.
🚚️ The smallest output window in which the bulk path is attempted; smaller windows keep the one-event-per-turn walk.
📏️ The deepest subtree the bulk path encodes inside one turn.
📤️ Encodes one whole subtree into `output` with exactly the bytes the one-event walk emits, or returns `None` when it does not fit or exceeds the bulk depth, leaving the caller on the walk.
⏩️ Moves the frame forward by whole subtrees inside one turn: a node that fits the window completes at once, and an array resumes as a run of children, so contiguous scalar runs cost one turn per window instead of one per element.
📊️ One admitted native action reports only its initialized byte prefix and common ownership receipt.
🌳️ Owns actual immutable-root aliases in a genuinely paged traversal frontier without path rescans.

