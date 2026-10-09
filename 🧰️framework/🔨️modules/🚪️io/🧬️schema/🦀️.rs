//! 🚪️ Product-neutral I/O payloads and route vocabulary over semantic artifact identities.

use semio_framework_diagnostic::Diagnostic;
use semio_framework_value_derive::{FromValue, ToValue};
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::serde::{Deserialize, Serialize};


use semio_framework_artifact_reference::{ArtifactDialect,ArtifactRef,Dialect,StandardId,SubsetId};
#[path="♻️retirement/🦀️.rs"]mod retirement;

//#region 🔖️Payload
#[path="../📦️payload/🧬️schema/🦀️.rs"]
mod payload;
pub use payload::IoPayload;

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

impl From<ValueError> for IoError {
    fn from(cause:ValueError)->Self{Self::from_value_error(cause)}
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
    semio_framework_artifact_reference::register_artifact_reference_schema_exports()?;
    semio_framework_schema_registry::register_scope_schema_exports(semio_framework_schema_registry::ScopeSchemaExports { scope: "framework.io", exports: &IO_SCHEMA_EXPORTS })
}
//#endregion 🔖️SchemaExports
// #endregion io-schema
