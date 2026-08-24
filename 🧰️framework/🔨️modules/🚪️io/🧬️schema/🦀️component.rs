// #region io-schema
//! 🧬️ Pure dialect/payload vocabulary for the io system — NO registry, NO `store::` dependency.
//! Ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task 1: this is the single
//! definition site for `StandardId`/`SubsetId`/`Dialect`/`ArtifactDialect`/`ArtifactKindId`/
//! `ArtifactRef` (moved here verbatim, byte-for-byte, from `🚪️io/🦀️component.rs`'s old
//! `🔖️Dialect`/`🔖️ArtifactRef` regions, which now `pub use` these names instead of defining
//! them — so `ArtifactDialect::to_coordinate`/`parse_coordinate`/`ArtifactRef::to_uri`/`parse_uri`
//! remain the ONE dialect-coordinate codec in the repo) plus the brand-new wire types the
//! `🔖️IoMechanism` region (same file, appended near the end) needs: `IoPayload`, `Confidence`,
//! `IoFidelity`, `IoError`/`IoOutcome`/`IoResult`, `IoEntryDescriptor`, `IoRoute`. These are
//! DELIBERATELY separate nominal types from the old file's own `IoPayload`/`Confidence` (which
//! keep their 3-variant/no-`None` shape so the old registry's exhaustive matches never change) —
//! see the W1-A report's "mount situation" section for why this is the correct, not merely
//! expedient, choice.
//!
//! Mounted ONCE — directly in the os-kernel crate glue (`os_io_schema`) — and re-exported by
//! `semio_framework` (`pub use semio_framework_os_kernel::os_io_schema as io_schema;`) rather than
//! remounted, because `semio-framework` already carries a real Cargo dependency on
//! `semio-framework-os-kernel` (see that crate's `extern crate semio_framework_os_kernel as
//! store;`/`as dsl;` aliases) — no cycle, unlike the `workflow` module's full-framework-surface
//! need documented in the os-kernel glue's own comment beside the (still double-mounted) `os_io`.

use dsl::Diagnostic;
use serde::{Deserialize, Serialize};

//#region 🔖️Dialect
/// 🏅️ A standard slug — the text after `🔖️` in `🏅️standards/🔖️<standard>/` (e.g. "2.0", "ap214", "1").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StandardId(pub &'static str);

/// 🪆️ A subset id — the text materialized as `🪆️subsets/✳️<dir>/`. `ANY` is the unconstrained base
/// subset every standard carries (dir `✳️any`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SubsetId(pub &'static str);

impl SubsetId {
    pub const ANY: SubsetId = SubsetId("*");
}

/// 🎯️ Fully-qualified dialect coordinate: which artifact, which standard, which subset.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Dialect {
    pub artifact_kind: &'static str,
    pub standard: StandardId,
    pub subset: SubsetId,
}

/// 🎯️ Owned serde twin of `Dialect` — the persisted/wire form; every dialect consumer outside a
/// `'static` compile-time registration (document envelopes, the hub's multi-user pin, WIT
/// `io-run`/`io-routes`, the io leaf generators) reads/writes THIS type.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactDialect {
    pub artifact_kind: String,
    pub standard: String,
    pub subset: String,
}

impl From<Dialect> for ArtifactDialect {
    fn from(d: Dialect) -> Self {
        ArtifactDialect { artifact_kind: d.artifact_kind.to_string(), standard: d.standard.0.to_string(), subset: d.subset.0.to_string() }
    }
}

impl ArtifactDialect {
    /// 🧵️ Canonical single-string coordinate form: `"s.stdio.gif@87a/*"`. The one format that
    /// crosses every boundary in the system — the only dialect-coordinate codec in the repo.
    // 🚫️async: E1 pure — `format!` only. See R9.
    pub fn to_coordinate(&self) -> String {
        format!("{}@{}/{}", self.artifact_kind, self.standard, self.subset)
    }

    /// 🧵️ Inverse of `to_coordinate`. `@` separates artifact_kind from standard/subset; the LAST
    /// `/` separates standard from subset.
    // 🚫️async: E1 pure — `split_once` only. See R9.
    pub fn parse_coordinate(s: &str) -> Result<Self, String> {
        let (kind, rest) = s.split_once('@').ok_or_else(|| format!("dialect coordinate {s:?} missing '@'"))?;
        let (standard, subset) = rest.rsplit_once('/').ok_or_else(|| format!("dialect coordinate {s:?} missing '/'"))?;
        if kind.is_empty() || standard.is_empty() || subset.is_empty() {
            return Err(format!("dialect coordinate {s:?} has an empty component"));
        }
        Ok(ArtifactDialect { artifact_kind: kind.to_string(), standard: standard.to_string(), subset: subset.to_string() })
    }
}
//#endregion 🔖️Dialect

//#region 🔖️ArtifactRef
/// 🪪️ Canonical artifact-kind id. Grammar: exactly three dot-separated ASCII segments,
/// `s.<plugin>.<artifact>` — the first segment is always the literal `s`, the remaining two are
/// lowercase-ASCII kebab (`[a-z0-9-]`, no leading/trailing/doubled hyphen).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ArtifactKindId(String);

impl ArtifactKindId {
    /// 🧵️ Parses and validates the canonical grammar, failing with a message that names which
    /// rule broke.
    pub fn parse(s: &str) -> Result<Self, String> {
        if !is_canonical_artifact_kind(s) {
            return Err(format!("artifact kind {s:?} is not canonical grammar `s.<plugin>.<artifact>` (three dot-separated ASCII segments, first literally `s`, the rest lowercase-kebab)"));
        }
        Ok(ArtifactKindId(s.to_string()))
    }

    /// 🔍️ Borrowed access to the full `s.<plugin>.<artifact>` string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 🔌️ Second segment — the owning plugin slug.
    pub fn plugin(&self) -> &str {
        self.0.split('.').nth(1).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }

    /// 🗿️ Third segment — the artifact slug within the plugin.
    pub fn artifact(&self) -> &str {
        self.0.split('.').nth(2).expect("ArtifactKindId invariant: exactly 3 dot-separated segments")
    }
}

impl std::fmt::Display for ArtifactKindId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// ✅️ Standalone canonical-grammar predicate behind `ArtifactKindId::parse`.
pub fn is_canonical_artifact_kind(kind: &str) -> bool {
    let mut segments = kind.split('.');
    let Some(first) = segments.next() else { return false };
    if first != "s" {
        return false;
    }
    let Some(plugin) = segments.next() else { return false };
    let Some(artifact) = segments.next() else { return false };
    if segments.next().is_some() {
        return false;
    }
    is_kebab_segment(plugin) && is_kebab_segment(artifact)
}

/// 🔡️ One canonical-grammar segment: non-empty lowercase-ASCII `[a-z0-9-]`, no leading/trailing
/// hyphen, no doubled hyphen.
fn is_kebab_segment(segment: &str) -> bool {
    if segment.is_empty() || segment.starts_with('-') || segment.ends_with('-') || segment.contains("--") {
        return false;
    }
    segment.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// 🔗️ A reference to one artifact: its id plus the dialect it is materialized in. Renders to/from
/// the wire URI `"<artifact_id>!<kind>@<standard>/<subset>"`.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub artifact_id: String,
    pub dialect: ArtifactDialect,
}

impl ArtifactRef {
    /// 🧵️ Canonical wire form: `"<artifact_id>!<kind>@<standard>/<subset>"`.
    // 🚫️async: E1 pure — canonical string formatting with no suspension point, consumed by sync
    // `DslField`/`DslVariants` trait impls that are language-barred from awaiting. See R9.
    pub fn to_uri(&self) -> String {
        format!("{}!{}", self.artifact_id, self.dialect.to_coordinate())
    }

    /// 🧵️ Inverse of `to_uri`. Splits on the FIRST `!`.
    // 🚫️async: E1 pure — string parsing only; same sync consumers as `to_uri`. See R9.
    pub fn parse_uri(s: &str) -> Result<Self, String> {
        let (artifact_id, coordinate) = s.split_once('!').ok_or_else(|| format!("artifact ref uri {s:?} missing '!'"))?;
        if artifact_id.is_empty() {
            return Err(format!("artifact ref uri {s:?} has an empty artifact id"));
        }
        let dialect = ArtifactDialect::parse_coordinate(coordinate)?;
        Ok(ArtifactRef { artifact_id: artifact_id.to_string(), dialect })
    }
}
//#endregion 🔖️ArtifactRef

//#region 🔖️Payload
/// 📦️ The one payload envelope the whole io mechanism moves. **Payload law**: the `IoPayload` of
/// dialect D is D's own *native* encoding — `Binary` = its pack, `Text` = its DSL — EXCEPT for the
/// two carrier dialects (`CARRIER_BINARY`, `CARRIER_TEXT`), whose native encoding IS the raw
/// external file content. So: **open a file** = `io_identify(bytes)` → `io_run(io_route(carrier →
/// D))`; **save a file** = `io_run(io_route(D → carrier))`. This is the rule that stops an export
/// writing pack bytes into a `.gif`/`.png` file.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum IoPayload {
    Text(String),
    Binary(Vec<u8>),
}

/// 🗄️ Carrier dialect for raw untyped bytes — the payload law's binary exception.
pub const CARRIER_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

/// 🗄️ Carrier dialect for raw untyped UTF-8 text — the payload law's text exception.
pub const CARRIER_TEXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };
//#endregion 🔖️Payload

//#region 🔖️Confidence
/// 🎚️ How sure an `io_identify` sniff is that a payload is dialect D. Distinct from the OLD
/// file's 3-variant `Confidence` (`High`/`Medium`/`Low`, no `None`) — that type stays exactly as
/// it is so the old registry's exhaustive matches never change; this 4-variant type is the new
/// mechanism's own, dropped entirely (not surfaced) by `io_identify` when the value is `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Confidence {
    None,
    Low,
    Medium,
    High,
}

impl Confidence {
    /// 📏️ Ordered strength: High > Medium > Low > None.
    pub async fn rank(self) -> u8 {
        match self {
            Self::None => 0,
            Self::Low => 1,
            Self::Medium => 2,
            Self::High => 3,
        }
    }
}
//#endregion 🔖️Confidence

//#region 🔖️IoFidelity
/// ⚖️ Declared strongest IO fidelity one hop of the new mechanism achieves. Distinct from the OLD
/// file's `IoFidelityClass` (same rank order, different name/type — that one stays a manifest
/// declaration field for the old subset-validator machinery).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum IoFidelity {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}

impl IoFidelity {
    /// 📏️ Ordered strength: Exact > Canonical > Semantic > Lossy — mirrors `IoFidelityClass::rank`.
    pub async fn rank(self) -> u8 {
        match self {
            Self::Exact => 3,
            Self::Canonical => 2,
            Self::Semantic => 1,
            Self::Lossy => 0,
        }
    }
}
//#endregion 🔖️IoFidelity

//#region 🔖️Result
/// 🚫️ A failed io operation: routing, running a hop, or (de)serializing one payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IoError {
    pub message: String,
    pub diagnostics: Vec<Diagnostic>,
}

/// 📦️ A successful io value plus every non-fatal diagnostic collected while obtaining it (e.g. a
/// `Deserializer::CONFORMANCE` check folded in after a successful deserialize) — same
/// value+diagnostics shape this file's own `CodecOutput<T>`/`CodecResult<T>` already establish for
/// the codec-contract layer, reused here for the io-mechanism layer.
#[derive(Clone, Debug, PartialEq)]
pub struct IoOutcome<T> {
    pub value: T,
    pub diagnostics: Vec<Diagnostic>,
}

impl<T> IoOutcome<T> {
    /// 🌱️ Wraps a bare value with no diagnostics — the common case for a clean hop.
    pub fn clean(value: T) -> Self {
        Self { value, diagnostics: Vec::new() }
    }
}

/// 🧩️ Common result boundary for every io-mechanism operation.
pub type IoResult<T> = Result<IoOutcome<T>, IoError>;
//#endregion 🔖️Result

//#region 🔖️Route
/// 📇️ One registered `IoEntry`, erased to owned/wire data — the shape the WIT `list-io-entries`
/// guest export and the TS `IoEntryDescriptor[]` mirror both use.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoEntryDescriptor {
    pub from: ArtifactDialect,
    pub into: ArtifactDialect,
    pub fidelity: IoFidelity,
    pub sniffs: bool,
}

/// 🗺️ A resolved, executable (or wire-transmissible) hop sequence from `io_route`. Pure data — no
/// `&'static IoEntry` pointers — so it can cross the WIT `io-routes` boundary; `io_run` re-resolves
/// each hop's `(from, into)` pair against the live registry.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoRoute {
    pub hops: Vec<IoEntryDescriptor>,
    pub fidelity: IoFidelity,
}
//#endregion 🔖️Route
// #endregion io-schema
