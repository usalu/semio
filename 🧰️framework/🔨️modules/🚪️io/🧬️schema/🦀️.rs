//! 🧬️ Product-neutral I/O identities, payloads and route vocabulary.

use semio_framework_diagnostic::Diagnostic;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::serde::{Deserialize, Serialize};

#[path = "♻️retirement/🦀️.rs"]
mod retirement;

//#region 🔖️Dialect
/// 🏅️ A standard slug — the text after `🔖️` in `🏅️standards/🔖️<standard>/` (e.g. "2.0", "ap214", "1").
/// 🌱️ `'static`-only, compile-time registration data — never crosses a wire, so it carries no
/// `Serialize`/`Deserialize`/`ToValue`/`FromValue` at all (nothing in the repo (de)serializes it).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StandardId(pub &'static str);

/// 🪆️ A subset id — the text materialized as `🪆️subsets/✳️<dir>/`. `ANY` is the unconstrained base
/// subset every standard carries (dir `✳️any`). See `StandardId`'s doc comment for why this has no
/// wire-codec derive.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SubsetId(pub &'static str);

impl SubsetId {
    pub const ANY: SubsetId = SubsetId("*");
}

/// 🎯️ Fully-qualified dialect coordinate: which artifact, which standard, which subset. See
/// `StandardId`'s doc comment for why this has no wire-codec derive — `ArtifactDialect` below is
/// this type's owned/wire twin.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Dialect {
    pub artifact_kind: &'static str,
    pub standard: StandardId,
    pub subset: SubsetId,
}

/// 🎯️ Owned twin of `Dialect` — the persisted/wire form; every dialect consumer outside a
/// `'static` compile-time registration (document envelopes, the hub's multi-user pin, WIT
/// `io-run`/`io-routes`, the io leaf generators) reads/writes THIS type via `ToValue`/`FromValue`.
// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `Serialize`/
// `Deserialize` restored here ADDITIVELY, not removed — `🛂️manifest/🦀️.rs`'s `AppDefinition` and
// `WindowKindDefinition` are themselves serde-only (blocked on `ui_wgpu::LocalizedLabel`/
// `IconName`/`SurfaceKind`/`WindowOptions`, none owned by this pass) and embed `dialect: ArtifactDialect`
// resp. reach it transitively; `IoEntryDescriptor.owner`/`counterpart` and
// `ComposerEntryDescriptor.writes`/`reads` are dual-derived but still need the serde half because
// they are `referenced (directly or transitively) by a BLOCKED serde-only manifest type`. Revisit
// once `🖱️ui` gains `ToValue`/`FromValue` for those types.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(crate = "semio_framework_value::serde", rename_all = "camelCase")]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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
/// `<domain>.<plugin>.<artifact>`, with each segment in lowercase ASCII kebab form.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(transparent)]
pub struct ArtifactKindId(String);

impl ArtifactKindId {
    /// 🧵️ Parses and validates the canonical grammar, failing with a message that names which
    /// rule broke.
    pub fn parse(s: &str) -> Result<Self, String> {
        if !is_canonical_artifact_kind(s) {
            return Err(format!("artifact kind {s:?} must use `<domain>.<plugin>.<artifact>` with three lowercase ASCII kebab segments"));
        }
        Ok(ArtifactKindId(s.to_string()))
    }

    /// 🔍️ Borrows the complete canonical artifact kind.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// 🌐️ Returns the declaring domain namespace.
    pub fn domain(&self) -> &str {
        self.0.split('.').next().expect("ArtifactKindId invariant: three segments")
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
    let Some(plugin) = segments.next() else { return false };
    let Some(artifact) = segments.next() else { return false };
    if segments.next().is_some() {
        return false;
    }
    is_kebab_segment(first) && is_kebab_segment(plugin) && is_kebab_segment(artifact)
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
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

    /// 🚦️ Parses identity handles with bounded borrowed scanning and admitted ownership of their four strings.
    pub fn parse_uri_controlled(text:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,String>{
        control.scoped_stage(|control|{
            control.begin_stage(text.len())?;let mut bang=None;let mut at=None;let mut slash=None;let mut position=0;
            for chunk in text.as_bytes().chunks(256){for(byte_offset,byte)in chunk.iter().enumerate(){let offset=position+byte_offset;if bang.is_none(){if *byte==b'!'{bang=Some(offset);}}else if at.is_none(){if *byte==b'@'{at=Some(offset);}}else if *byte==b'/'{slash=Some(offset);}}position+=chunk.len();control.advance(chunk.len())?;}
            let bang=bang.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact reference requires '!'"))?;let at=at.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact dialect requires '@'"))?;let slash=slash.ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"artifact dialect requires '/'"))?;
            if bang==0||at==bang+1||slash==at+1||slash+1==text.len(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"artifact reference has an empty identity component"));}
            Ok(Self{artifact_id:control.copy_text(&text[..bang])?,dialect:ArtifactDialect{artifact_kind:control.copy_text(&text[bang+1..at])?,standard:control.copy_text(&text[at+1..slash])?,subset:control.copy_text(&text[slash+1..])?}})
        }).map_err(ValueError::into_message)
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
pub enum IoPayload {
    Text(String),
    Binary(Vec<u8>),
}

/// 🗄️ Carrier dialect for raw untyped bytes — the payload law's binary exception.
pub const CARRIER_BINARY: Dialect = Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") };

/// 🗄️ Carrier dialect for raw untyped UTF-8 text — the payload law's text exception.
pub const CARRIER_TEXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

/// 🪶️ Standalone SQLite container preserving one artifact's native snapshot and exact dialect.
pub const SQLITE_SNAPSHOT: Dialect = Dialect { artifact_kind: "s.framework.sqlite-snapshot", standard: StandardId("1"), subset: SubsetId("*") };
//#endregion 🔖️Payload

//#region 🔖️Confidence
/// 🎚️ How sure an `io_identify` sniff is that a payload is dialect D. Distinct from the OLD
/// file's 3-variant `Confidence` (`High`/`Medium`/`Low`, no `None`) — that type stays exactly as
/// it is so the old registry's exhaustive matches never change; this 4-variant type is the new
/// mechanism's own, dropped entirely (not surfaced) by `io_identify` when the value is `None`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum Confidence {
    None,
    Low,
    Medium,
    High,
}

impl Confidence {
    /// 📏️ Ordered strength: High > Medium > Low > None.
    pub const fn rank(self) -> u8 {
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
pub enum IoFidelity {
    Exact,
    Canonical,
    Semantic,
    Lossy,
}

impl IoFidelity {
    /// 📏️ Ordered strength: Exact > Canonical > Semantic > Lossy — mirrors `IoFidelityClass::rank`.
    pub const fn rank(self) -> u8 {
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
#[derive(Clone, Debug, PartialEq)]
pub struct IoError {
    pub cause: ValueError,
    pub diagnostics: Vec<Diagnostic>,
}

/// 📦️ A successful io value plus every non-fatal diagnostic collected while obtaining it (e.g. a
/// `Deserializer::CONFORMANCE` check folded in after a successful deserialize) — same
/// value+diagnostics shape this file's own `CodecOutput<T>`/`CodecResult<T>` already establish for
/// the codec-contract layer, reused here for the io-mechanism layer.
impl IoError {
    /// 🛫️ Projects the closed cause/diagnostic record through canonical owned controls.
    pub fn to_value_controlled(&self, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<semio_framework_value::DslValue, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(2)?;
            let mut fields = semio_framework_value::DslValue::object_encoding_controlled(2, control)?;
            let cause = self.cause.to_value_controlled(control)?;
            semio_framework_value::DslValue::push_encoding_controlled(fields.get_mut(), "cause", cause, control)?;
            control.step()?;
            let diagnostics = semio_framework_value::ToValue::to_value_controlled(&self.diagnostics, control)?;
            semio_framework_value::DslValue::push_encoding_controlled(fields.get_mut(), "diagnostics", diagnostics, control)?;
            control.step()?;
            Ok(semio_framework_value::DslValue::Object(fields.take()))
        })
    }

    /// 🛬️ Constructs exact owned causes and source diagnostics without an unchecked codec.
    pub fn from_value_controlled(value: &semio_framework_value::DslValue, control: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            control.checkpoint()?;
            let semio_framework_value::DslValue::Object(fields) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected IO error object")) };
            if fields.len() != 2 { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "IO error requires exactly cause and diagnostics")) }
            control.begin_stage(2)?;
            let (mut cause, mut diagnostics) = (None, None);
            for (key, value) in fields {
                match key.as_str() {
                    "cause" if cause.is_none() => cause = Some(value),
                    "diagnostics" if diagnostics.is_none() => diagnostics = Some(value),
                    _ => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "unknown or duplicate IO error field")),
                }
                control.step()?;
            }
            control.charge(std::mem::size_of::<Self>())?;
            let cause = ValueError::from_value_controlled(cause.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing IO error cause"))?, control)?;
            let cause = semio_framework_value::DecodedValue::new(cause, drop);
            let diagnostics = <Vec<Diagnostic> as semio_framework_value::FromValue>::from_value_controlled(diagnostics.ok_or_else(|| ValueError::new(ValueRefusalKind::InvalidValue, "missing IO error diagnostics"))?, control)?;
            let output = semio_framework_value::DecodedValue::new(Self { cause: cause.take(), diagnostics }, |error| <Vec<Diagnostic> as semio_framework_value::FromValue>::retire_decoded(error.diagnostics));
            control.checkpoint()?;
            Ok(output.take())
        })
    }

    /// 🚪️ Moves the complete owned Value refusal without text projection or allocation.
    pub fn from_value_error(cause: ValueError) -> Self { Self { cause, diagnostics: Vec::new() } }

    /// 📍️ Retains an authored text source through charged, cancelable diagnostic ownership.
    pub fn from_text_error_controlled(error: semio_framework_diagnostic::TextError, control: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<Self, ValueError> {
        control.scoped_stage(|control| {
            control.begin_stage(1)?;
            control.charge(std::mem::size_of::<Self>())?;
            let message = control.copy_text(&error.message)?;
            let mut diagnostics = control.allocate_vec::<Diagnostic>(1)?;
            let code = control.copy_text("io.text-refusal")?;
            let expected = match error.expected {
                Some(expected) => {
                    control.charge(std::mem::size_of::<semio_framework_diagnostic::ExpectedSet>())?;
                    let mut tokens = control.allocate_vec::<String>(1)?;
                    tokens.push(expected);
                    Some(semio_framework_diagnostic::ExpectedSet { tokens, keywords: Vec::new(), keys: Vec::new() })
                }
                None => None,
            };
            diagnostics.push(Diagnostic {
                code: semio_framework_diagnostic::FaultCode::new(code),
                severity: semio_framework_diagnostic::Severity::Error,
                span: error.span,
                message: error.message,
                expected,
                scope: semio_framework_diagnostic::FaultScope::default(),
            });
            control.step()?;
            Ok(Self { cause: ValueError::new(error.kind, message), diagnostics })
        })
    }
}

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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IoEntryDescriptor {
    pub from: ArtifactDialect,
    pub into: ArtifactDialect,
    pub fidelity: IoFidelity,
    pub sniffs: bool,
}

/// 🗺️ A resolved, executable (or wire-transmissible) hop sequence from `io_route`. Pure data — no
/// `&'static IoEntry` pointers — so it can cross the WIT `io-routes` boundary; `io_run` re-resolves
/// each hop's `(from, into)` pair against the live registry.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct IoRoute {
    pub hops: Vec<IoEntryDescriptor>,
    pub fidelity: IoFidelity,
}
//#endregion 🔖️Route

//#region 🔖️SchemaExports
const IO_SCHEMA_EXPORTS: [semio_framework_schema_registry::SchemaExport; 1] = [semio_framework_schema_registry::SchemaExport { id: "schema", leaves: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") } }];

/// 📌️ Registers the io vocabulary's own schema document (`framework/io/schema.json`: `ArtifactRef`, dialects, the io wire
/// types) as the `schema` export of the `framework.io` scope, so every contract that `$ref`s it resolves it.
// 🚫️async: E1 pure registration helper (no I/O) — see R9
pub fn register_io_schema_exports() -> Result<(), semio_framework_schema_registry::SchemaExportRegistryError> {
    semio_framework_schema_registry::register_scope_schema_exports(semio_framework_schema_registry::ScopeSchemaExports { scope: "framework.io", exports: &IO_SCHEMA_EXPORTS })
}
//#endregion 🔖️SchemaExports
// #endregion io-schema
