//! ⚠️ Text errors, structured diagnostics, fault reporting, and parse limits.
// 🚫️async: E1 pure accessor consumed by external-trait impls (serde/Display) — see R9

pub use crate::span::TextSpan;
use semio_framework_value::{DslValue, FromValue, ToValue, ValueError, ValueRefusalKind};
#[path = "🎛️controlled/🦀️.rs"]
pub(crate) mod controlled;

//#region 🔖️Errors
/// 🚧️ Span-carrying parse/print failure — the one error type every DSL surface returns.
#[derive(Clone, Debug, PartialEq)]
pub struct TextError {
    pub kind: ValueRefusalKind,
    pub message: String,
    pub span: TextSpan,
    pub expected: Option<String>,
}

impl std::fmt::Display for TextError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} at {}:{}", self.message, self.span.line, self.span.column)
    }
}

impl std::error::Error for TextError {}

/// 🌉️ Hand-written, not derived — same DAG reason as `FaultCode`/`Severity` above. No
/// `#[serde(rename_all = …)]` on the original, so field names stay as declared.
impl ToValue for TextError {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_text_error(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("kind".to_string(), DslValue::String(self.kind.as_str().into())), ("message".to_string(), DslValue::String(self.message.clone())), ("span".to_string(), self.span.to_value())];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), DslValue::String(expected.clone())));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for TextError {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_text_error(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for TextError, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let kind = match find("kind") { Some(value) => controlled::refusal_kind(&value)?, None => return Err(ValueError::new(ValueRefusalKind::InvalidValue, "missing TextError.kind")) };
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.message, found {other:?}"))),
        };
        let span = match find("span") {
            Some(slot) => TextSpan::from_value(slot)?,
            None => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "missing TextError.span")),
        };
        let expected = match find("expected") {
            None | Some(DslValue::Null) => None,
            Some(DslValue::String(text)) => Some(text),
            Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for TextError.expected, found {other:?}"))),
        };
        Ok(TextError { kind, message, span, expected })
    }
}

impl TextError {
    pub fn new(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan) -> Self {
        Self { kind, message: message.into(), span, expected: None }
    }

    pub fn expected(kind: ValueRefusalKind, message: impl Into<String>, span: TextSpan, expected: impl Into<String>) -> Self {
        Self { kind, message: message.into(), span, expected: Some(expected.into()) }
    }

    /// 🧭️ Retains owned refusal authority at the caller-authored source position.
    pub fn from_value_error(error: ValueError, span: TextSpan) -> Self { Self::new(error.kind, error.message, span) }

    pub fn from_diagnostic(kind: ValueRefusalKind, diagnostic: Diagnostic) -> Self {
        diagnostic.into_text_error(kind)
    }
}

/// 🏷️ Stable dotted fault/diagnostic code (e.g. `module.pack.checksum-mismatch`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaultCode(pub String);

impl FaultCode {
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into())
    }
}

impl From<&'static str> for FaultCode {
    fn from(value: &'static str) -> Self {
        Self(value.to_string())
    }
}

/// 🌉️ Hand-written, not derived: `FaultCode` is `#[serde(transparent)]`, a shape
/// `#[derive(ToValue, FromValue)]` does not support (see its own module docstring).
impl ToValue for FaultCode {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_code(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.clone())
    }
}
impl FromValue for FaultCode {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_code(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => Ok(FaultCode(s)),
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCode, found {other:?}"))),
        }
    }
}

/// 🏷️ Stable, greppable diagnostic identifier, e.g. `"DSL0001"`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticCode(pub &'static str);

impl From<String> for FaultCode {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<DiagnosticCode> for FaultCode {
    fn from(value: DiagnosticCode) -> Self {
        FaultCode::new(value.0)
    }
}

/// 🚦️ Declaration order is the level order: `#[derive(PartialOrd, Ord)]` makes `Info < Warning <
/// Error < Fatal` a structural fact, not a hand-maintained comparator. `as_u8`/`from_u8` (0..3)
/// give a stable wire-compatible numeric mirror for TS/WIT twins.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Error,
    Fatal,
}

/// 🌉️ Hand-written, not derived: `Severity` is a plain unit-only "string enum" (`#[serde(rename_all
/// = "camelCase")]` with no `tag`, serde's default bare-string representation) —
/// `#[derive(ToValue, FromValue)]`'s enum path only supports internally-tagged representations.
impl ToValue for Severity {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_severity(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for Severity {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_severity(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "info" => Ok(Severity::Info),
                "warning" => Ok(Severity::Warning),
                "error" => Ok(Severity::Error),
                "fatal" => Ok(Severity::Fatal),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown Severity variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

impl Severity {
    /// 🔢️ Stable numeric mirror of declaration order, 0..3.
    pub fn as_u8(self) -> u8 {
        match self {
            Severity::Info => 0,
            Severity::Warning => 1,
            Severity::Error => 2,
            Severity::Fatal => 3,
        }
    }

    /// 🔢️ Inverse of [`as_u8`](Self::as_u8); `None` for any value outside 0..3.
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Severity::Info),
            1 => Some(Severity::Warning),
            2 => Some(Severity::Error),
            3 => Some(Severity::Fatal),
            _ => None,
        }
    }
}

/// 🧭️ What the parser would have accepted at the failure point — the raw material for
/// completions and for `TextError.expected`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExpectedSet {
    pub tokens: Vec<String>,
    pub keywords: Vec<String>,
    pub keys: Vec<String>,
}

impl ExpectedSet {
    pub fn describe(&self) -> String {
        let mut parts = Vec::new();
        if !self.keywords.is_empty() {
            parts.push(self.keywords.join("|"));
        }
        if !self.keys.is_empty() {
            parts.push(self.keys.iter().map(|k| format!("{k}=")).collect::<Vec<_>>().join("|"));
        }
        if !self.tokens.is_empty() {
            parts.push(self.tokens.join("|"));
        }
        parts.join(" or ")
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for ExpectedSet {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_expected(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(vec![
            ("tokens".to_string(), DslValue::Array(self.tokens.iter().cloned().map(DslValue::String).collect())),
            ("keywords".to_string(), DslValue::Array(self.keywords.iter().cloned().map(DslValue::String).collect())),
            ("keys".to_string(), DslValue::Array(self.keys.iter().cloned().map(DslValue::String).collect())),
        ])
    }
}
impl FromValue for ExpectedSet {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_expected(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for ExpectedSet, found {other:?}"))),
        };
        let strings = |key: &str| -> Result<Vec<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(Vec::new()),
                Some(DslValue::Array(items)) => items
                    .iter()
                    .map(|item| match item {
                        DslValue::String(text) => Ok(text.clone()),
                        other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string in ExpectedSet.{key}, found {other:?}"))),
                    })
                    .collect(),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for ExpectedSet.{key}, found {other:?}"))),
            }
        };
        Ok(ExpectedSet { tokens: strings("tokens")?, keywords: strings("keywords")?, keys: strings("keys")? })
    }
}

/// 🩺️ A structured diagnostic anchored to a span, with an optional `ExpectedSet` for
/// completions/fixes. Lowers into `TextError` with the caller's explicit refusal authority.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagnostic {
    pub code: FaultCode,
    pub severity: Severity,
    pub span: TextSpan,
    pub message: String,
    pub expected: Option<ExpectedSet>,
    pub scope: FaultScope,
}

impl Diagnostic {
    pub fn error(code: &'static str, span: TextSpan, message: impl Into<String>) -> Self {
        Self { code: FaultCode::new(code), severity: Severity::Error, span, message: message.into(), expected: None, scope: FaultScope::default() }
    }

    pub fn with_expected(mut self, expected: ExpectedSet) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn into_text_error(self, kind: ValueRefusalKind) -> TextError {
        let expected = self.expected.as_ref().map(ExpectedSet::describe);
        match expected {
            Some(expected) => TextError::expected(kind, self.message, self.span, expected),
            None => TextError::new(kind, self.message, self.span),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Diagnostic {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_diagnostic(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("span".to_string(), self.span.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
        ];
        if let Some(expected) = &self.expected {
            entries.push(("expected".to_string(), expected.to_value()));
        }
        entries.push(("scope".to_string(), self.scope.to_value()));
        DslValue::Object(entries)
    }
}
impl FromValue for Diagnostic {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_diagnostic(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Diagnostic, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Diagnostic.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Diagnostic.message, found {other:?}"))),
        };
        Ok(Diagnostic {
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            span: TextSpan::from_value(required("span")?)?,
            message,
            expected: match find("expected") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(ExpectedSet::from_value(slot)?),
            },
            scope: match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            },
        })
    }
}

//#region 🔖️Fault
/// 🧭️ Which layer of the os stack produced a {@link Fault}.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaultOrigin {
    Edge,
    Renderer,
    Os,
    Module,
    Plugin,
    App,
    Extension,
    /// 🚪️👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §2.3: origin for the
    /// five frozen `surface.*`/`viewer.*` fault codes (`AppRouter`/`OpeningResolver`/`VcsArtifactApp`
    /// role guard) — additive variant, no existing variant touched, no match site in this crate is
    /// exhaustive over it (verified with a repo-wide grep before adding).
    Framework,
}

/// 🎯️ Optional ids locating a fault/diagnostic to a plugin app surface.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct FaultScope {
    pub plugin_id: Option<String>,
    pub app_id: Option<String>,
    pub instance_id: Option<String>,
    pub module: Option<String>,
    pub body_key: Option<String>,
}

/// 🔗️ One hop in a {@link Fault} cause chain.
#[derive(Clone, Debug, PartialEq)]
pub struct FaultCause {
    pub message: String,
    pub code: Option<FaultCode>,
}

/// 🔣️ The named values a localized fault notice fills its `{name}` placeholders from (design §20.12): unique names in
/// authoring order, wire `{name: value}`. The only source of placeholder data — a shell never reads one from `message`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FaultParams(pub Vec<(String, String)>);

impl FaultParams {
    /// 🔎️ The value named `name`, if present.
    pub fn get(&self, name: &str) -> Option<&str> {
        self.0.iter().find(|(known, _)| known == name).map(|(_, value)| value.as_str())
    }
}

/// 🔤️ Whether `name` is a placeholder name (`[a-z][A-Za-z0-9]*`) — the grammar of `FaultParams` keys and of the `{name}`
/// placeholders a notice declares (schema `🧬️schema/🎛️controlled` `$defs.FaultParams`).
pub fn is_fault_param_name(name: &str) -> bool {
    let mut chars = name.chars();
    chars.next().is_some_and(|first| first.is_ascii_lowercase()) && chars.all(|rest| rest.is_ascii_alphanumeric())
}

/// 🧯️ Structured abort report crossing every os boundary.
/// Scope metadata and notice params own separate allocations to keep error return values compact.
#[derive(Clone, Debug, PartialEq)]
pub struct Fault {
    pub retained_progress:semio_framework_value::RetainedCloneProgress,
    pub origin: FaultOrigin,
    pub code: FaultCode,
    pub severity: Severity,
    pub message: String,
    pub scope: Box<FaultScope>,
    pub span: Option<TextSpan>,
    pub causes: Vec<FaultCause>,
    /// 🔣️ The placeholder values of this fault's localized notice; absent when it names none.
    pub params: Option<Box<FaultParams>>,
    pub retryable: bool,
}

/// 🌉️ Owned Value conversion preserves the canonical Diagnostic fault wire.
/// Literal camelCase keys and absent optional fields match the declared fault schema.
impl ToValue for FaultOrigin {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_origin(self, c) }
    fn to_value(&self) -> DslValue {
        let name = match self {
            FaultOrigin::Edge => "edge",
            FaultOrigin::Renderer => "renderer",
            FaultOrigin::Os => "os",
            FaultOrigin::Module => "module",
            FaultOrigin::Plugin => "plugin",
            FaultOrigin::App => "app",
            FaultOrigin::Extension => "extension",
            FaultOrigin::Framework => "framework",
        };
        DslValue::String(name.to_string())
    }
}
impl FromValue for FaultOrigin {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_origin(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        match value {
            DslValue::String(s) => match s.as_str() {
                "edge" => Ok(FaultOrigin::Edge),
                "renderer" => Ok(FaultOrigin::Renderer),
                "os" => Ok(FaultOrigin::Os),
                "module" => Ok(FaultOrigin::Module),
                "plugin" => Ok(FaultOrigin::Plugin),
                "app" => Ok(FaultOrigin::App),
                "extension" => Ok(FaultOrigin::Extension),
                "framework" => Ok(FaultOrigin::Framework),
                other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("unknown FaultOrigin variant `{other}`"))),
            },
            other => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string, found {other:?}"))),
        }
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultScope {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_scope(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries: Vec<(String, DslValue)> = Vec::new();
        for (key, slot) in [
            ("pluginId", &self.plugin_id),
            ("appId", &self.app_id),
            ("instanceId", &self.instance_id),
            ("module", &self.module),
            ("bodyKey", &self.body_key),
        ] {
            if let Some(text) = slot {
                entries.push((key.to_string(), DslValue::String(text.clone())));
            }
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultScope {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_scope(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn default_value_controlled(c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { c.checkpoint()?; Ok(Self::default()) }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultScope, found {other:?}"))),
        };
        let take = |key: &str| -> Result<Option<String>, ValueError> {
            match entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot) {
                None | Some(DslValue::Null) => Ok(None),
                Some(DslValue::String(text)) => Ok(Some(text.clone())),
                Some(other) => Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultScope.{key}, found {other:?}"))),
            }
        };
        Ok(FaultScope {
            plugin_id: take("pluginId")?,
            app_id: take("appId")?,
            instance_id: take("instanceId")?,
            module: take("module")?,
            body_key: take("bodyKey")?,
        })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for FaultCause {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_cause(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![("message".to_string(), DslValue::String(self.message.clone()))];
        if let Some(code) = &self.code {
            entries.push(("code".to_string(), code.to_value()));
        }
        DslValue::Object(entries)
    }
}
impl FromValue for FaultCause {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_cause(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for FaultCause, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let message = match find("message") {
            Some(DslValue::String(text)) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for FaultCause.message, found {other:?}"))),
        };
        let code = match find("code") {
            None | Some(DslValue::Null) => None,
            Some(slot) => Some(FaultCode::from_value(slot)?),
        };
        Ok(FaultCause { message, code })
    }
}

/// 🌉️ Hand-written — see `FaultOrigin` above.
impl ToValue for Fault {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_fault(self, c) }
    fn to_value(&self) -> DslValue {
        let mut entries = vec![
            ("origin".to_string(), self.origin.to_value()),
            ("code".to_string(), self.code.to_value()),
            ("severity".to_string(), self.severity.to_value()),
            ("message".to_string(), DslValue::String(self.message.clone())),
            ("scope".to_string(), self.scope.to_value()),
            ("retainedProgress".to_string(), self.retained_progress.to_value()),
        ];
        if let Some(span) = &self.span {
            entries.push(("span".to_string(), span.to_value()));
        }
        if !self.causes.is_empty() {
            entries.push(("causes".to_string(), DslValue::Array(self.causes.iter().map(ToValue::to_value).collect())));
        }
        if let Some(params) = self.params.as_deref().filter(|params| !params.0.is_empty()) {
            entries.push(("params".to_string(), params.to_value()));
        }
        entries.push(("retryable".to_string(), DslValue::Bool(self.retryable)));
        DslValue::Object(entries)
    }
}

/// 🌉️ `{name: value}` in authoring order — see [`FaultParams`].
impl ToValue for FaultParams {
    fn to_value_controlled(&self, c: &mut semio_framework_value::NativeEncodeControl<'_>) -> Result<DslValue, ValueError> { controlled::encode_params(self, c) }
    fn to_value(&self) -> DslValue {
        DslValue::Object(self.0.iter().map(|(name, value)| (name.clone(), DslValue::String(value.clone()))).collect())
    }
}
impl FromValue for FaultParams {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_params(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let DslValue::Object(entries) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "expected an object for FaultParams")) };
        let mut params = Vec::with_capacity(entries.len());
        for (name, value) in entries {
            if !is_fault_param_name(&name) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("invalid FaultParams name `{name}`"))) }
            if params.iter().any(|(known, _): &(String, String)| *known == name) { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("duplicate FaultParams name `{name}`"))) }
            let DslValue::String(value) = value else { return Err(ValueError::new(ValueRefusalKind::InvalidValue, format!("expected a string for FaultParams.{name}"))) };
            params.push((name, value));
        }
        Ok(Self(params))
    }
}
impl FromValue for Fault {
    fn from_value_controlled(value: &DslValue, c: &mut semio_framework_value::NativeDecodeControl<'_>) -> Result<Self, ValueError> { controlled::decode_fault(value, c) }
    fn retire_decoded(self) { controlled::retire(self); }
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = match value {
            DslValue::Object(entries) => entries,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an object for Fault, found {other:?}"))),
        };
        let find = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, slot)| slot.clone());
        let required = |key: &str| -> Result<DslValue, ValueError> {
            find(key).ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("missing Fault.{key}")))
        };
        let message = match required("message")? {
            DslValue::String(text) => text,
            other => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for Fault.message, found {other:?}"))),
        };
        Ok(Fault {
            retained_progress:semio_framework_value::RetainedCloneProgress::from_value(required("retainedProgress")?)?,
            origin: FaultOrigin::from_value(required("origin")?)?,
            code: FaultCode::from_value(required("code")?)?,
            severity: Severity::from_value(required("severity")?)?,
            message,
            scope: Box::new(match find("scope") {
                None | Some(DslValue::Null) => FaultScope::default(),
                Some(slot) => FaultScope::from_value(slot)?,
            }),
            span: match find("span") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(TextSpan::from_value(slot)?),
            },
            causes: match find("causes") {
                None | Some(DslValue::Null) => Vec::new(),
                Some(DslValue::Array(items)) => items.into_iter().map(FaultCause::from_value).collect::<Result<_, _>>()?,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected an array for Fault.causes, found {other:?}"))),
            },
            params: match find("params") {
                None | Some(DslValue::Null) => None,
                Some(slot) => Some(FaultParams::from_value(slot)?).filter(|params| !params.0.is_empty()).map(Box::new),
            },
            retryable: match find("retryable") {
                None | Some(DslValue::Null) => false,
                Some(DslValue::Bool(flag)) => flag,
                Some(other) => return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a bool for Fault.retryable, found {other:?}"))),
            },
        })
    }
}

impl From<&str> for Fault {
    fn from(value: &str) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl From<String> for Fault {
    fn from(value: String) -> Self {
        Fault::new(FaultOrigin::App, FaultCode::new("app.message"), value)
    }
}

impl Fault {
    /// 🏷️ Borrows an existing canonical Value refusal cause while preserving its original diagnostic receipt.
    pub fn value_refusal_kind(&self)->Option<ValueRefusalKind>{self.param("refusalKind").and_then(ValueRefusalKind::from_wire)}
    /// 🧾️ Reads the exact original failed producer receipt carried through this diagnostic.
    pub fn retained_progress(&self)->semio_framework_value::RetainedCloneProgress{self.retained_progress}
    /// 🫴️ Attaches the same original receipt before crossing a diagnostic boundary.
    pub fn with_retained_progress(mut self,progress:semio_framework_value::RetainedCloneProgress)->Self{self.retained_progress=progress;self}
    pub fn new(origin: FaultOrigin, code: impl Into<FaultCode>, message: impl Into<String>) -> Self {
        Self { retained_progress:Default::default(),origin, code: code.into(), severity: Severity::Error, message: message.into(), scope: Box::default(), span: None, causes: Vec::new(), params: None, retryable: false }
    }

    /// 🔣️ Names one placeholder value of this fault's localized notice (`{name}`, design §20.12); a later value for the
    /// same name replaces the earlier one.
    pub fn with_param(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let (name, value) = (name.into(), value.into());
        let params = &mut self.params.get_or_insert_with(Box::default).0;
        match params.iter_mut().find(|(known, _)| *known == name) {
            Some(slot) => slot.1 = value,
            None => params.push((name, value)),
        }
        self
    }

    /// 🔎️ The placeholder value named `name`, if this fault carries it.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.params.as_deref().and_then(|params| params.get(name))
    }

    pub fn with_scope(mut self, scope: FaultScope) -> Self {
        *self.scope = scope;
        self
    }

    pub fn with_retryable(mut self, retryable: bool) -> Self {
        self.retryable = retryable;
        self
    }

    /// 🗣️ Canonical one-line rendering of a fault for `String` error channels.
    ///
    /// `Fault` deliberately has no `Display`: a structured abort must not be silently interpolated
    /// into prose, and `to_string()` would hide the {@link FaultCode} every triage tool keys on.
    /// Every boundary that has to collapse a fault into text calls this instead, so the
    /// `code: message` shape is written once rather than re-derived per crate.
    pub fn describe(&self) -> String {
        format!("{}: {}", self.code.0, self.message)
    }
}

/// 🔁️ Maps a domain error enum into a {@link Fault} at a boundary.
pub trait FaultFrom {
    fn fault_origin(&self) -> FaultOrigin;
    fn fault_code(&self) -> FaultCode;
    fn fault_severity(&self) -> Severity;
    fn fault_message(&self) -> String;
    fn fault_scope(&self) -> FaultScope {
        FaultScope::default()
    }
    fn fault_span(&self) -> Option<TextSpan> {
        None
    }
    fn fault_causes(&self) -> Vec<FaultCause> {
        Vec::new()
    }
    fn fault_retryable(&self) -> bool {
        false
    }
    fn fault_params(&self) -> FaultParams {
        FaultParams::default()
    }

    /// 👁️ Builds a diagnostic while the original typed refusal remains with its owner.
    fn to_fault(&self) -> Fault {
        let params = Some(self.fault_params()).filter(|params| !params.0.is_empty()).map(Box::new);
        Fault { retained_progress:Default::default(),origin: self.fault_origin(), code: self.fault_code(), severity: self.fault_severity(), message: self.fault_message(), scope: Box::new(self.fault_scope()), span: self.fault_span(), causes: self.fault_causes(), params, retryable: self.fault_retryable() }
    }

    fn into_fault(self)->Fault where Self:Sized{self.to_fault()}
}

impl FaultFrom for TextError {
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Module
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("module.dsl.text")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.clone()
    }

    fn fault_span(&self) -> Option<TextSpan> {
        Some(self.span)
    }
}

impl FaultFrom for ValueError {
    fn to_fault(&self)->Fault{Fault::new(self.fault_origin(),self.fault_code(),self.fault_message()).with_param("refusalKind",self.kind.as_str()).with_retained_progress(self.retained_progress())}
    fn fault_origin(&self) -> FaultOrigin {
        FaultOrigin::Framework
    }

    fn fault_code(&self) -> FaultCode {
        FaultCode::new("value.refusal")
    }

    fn fault_severity(&self) -> Severity {
        Severity::Error
    }

    fn fault_message(&self) -> String {
        self.message.to_string()
    }

    fn fault_params(&self) -> FaultParams {
        FaultParams(vec![("refusalKind".to_string(), self.kind.as_str().to_string())])
    }
}

/// 📦️ JSON wire encoding for {@link Fault} crossing host/WIT boundaries.
pub fn encode_fault_bytes(fault: &Fault) -> Vec<u8> {
    serde_json::to_vec(&fault.to_value()).unwrap_or_else(|_| fault.message.as_bytes().to_vec())
}

fn truncate_fault_text(text: &mut String, maximum_bytes: usize) {
    if text.len() <= maximum_bytes {
        return;
    }
    let mut end = maximum_bytes;
    while end > 0 && !text.is_char_boundary(end) {
        end -= 1;
    }
    text.truncate(end);
}

/// 🧰️ Encodes one complete canonical Fault within a fixed carrier page, narrowing only oversized prose and
/// parameter values while preserving the typed code, source span, and named diagnostic locations.
pub fn encode_fault_bytes_bounded(fault: &Fault, maximum_bytes: usize) -> Option<Vec<u8>> {
    let exact = encode_fault_bytes(fault);
    if exact.len() <= maximum_bytes {
        return Some(exact);
    }
    let mut bounded = fault.clone();
    bounded.causes.clear();
    *bounded.scope = FaultScope::default();
    truncate_fault_text(&mut bounded.message, 192);
    if let Some(params) = bounded.params.as_mut() {
        params.0.truncate(8);
        for (_, value) in &mut params.0 {
            truncate_fault_text(value, 96);
        }
    }
    loop {
        let encoded = encode_fault_bytes(&bounded);
        if encoded.len() <= maximum_bytes {
            return Some(encoded);
        }
        if !bounded.message.is_empty() {
            bounded.message.pop();
            continue;
        }
        let Some(params) = bounded.params.as_mut() else { return None };
        let Some((_, value)) = params.0.iter_mut().max_by_key(|(_, value)| value.len()) else { return None };
        if value.is_empty() {
            return None;
        }
        value.pop();
    }
}

/// 🧬️ Decodes only an exact canonical Fault wire, leaving malformed or untyped bytes distinguishable to the carrier.
pub fn try_decode_fault_bytes(bytes: &[u8]) -> Option<Fault> {
    serde_json::from_slice::<DslValue>(bytes).ok().and_then(|value| Fault::from_value(value).ok())
}

/// 🌐️ Decodes a {@link Fault} from JSON wire bytes; falls back to an os-level message fault.
pub fn decode_fault_bytes(bytes: &[u8]) -> Fault {
    try_decode_fault_bytes(bytes).unwrap_or_else(|| Fault::new(FaultOrigin::Os, "os.fault.decode", String::from_utf8_lossy(bytes)))
}

/// 🔁️ Maps an error type into {@link Fault} with a stable dotted code namespace.
#[macro_export]
macro_rules! fault_from_error {
    ($ty:ty, $origin:expr, $prefix:literal) => {
        impl $crate::FaultFrom for $ty {
            fn fault_origin(&self) -> $crate::FaultOrigin {
                $origin
            }

            fn fault_code(&self) -> $crate::FaultCode {
                $crate::FaultCode::new($prefix)
            }

            fn fault_severity(&self) -> $crate::Severity {
                $crate::Severity::Error
            }

            fn fault_message(&self) -> String {
                self.to_string()
            }
        }
    };
}

//#endregion 🔖️Fault
//#endregion 🔖️Errors

//#region 🔖️Limits
/// 🛡️ Resource budgets threaded through every parse — exceeding one yields a budget
/// diagnostic (`DSL0100`), never a panic or unbounded recursion.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Limits {
    pub max_bytes: usize,
    pub max_tokens: usize,
    pub max_depth: usize,
    pub max_nodes: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self { max_bytes: 16 * 1024 * 1024, max_tokens: 1_000_000, max_depth: 64, max_nodes: 1_000_000 }
    }
}

pub const BUDGET_EXCEEDED_CODE: &str = "DSL0100";

impl Limits {
    pub fn check_bytes(&self, len: usize) -> Result<(), TextError> {
        if len > self.max_bytes {
            return Err(TextError::new(ValueRefusalKind::OwnershipLimit, format!("input exceeds max_bytes limit ({} > {})", len, self.max_bytes), TextSpan::at(1, 1)));
        }
        Ok(())
    }

    pub fn check_depth(&self, depth: usize, span: TextSpan) -> Result<(), TextError> {
        if depth > self.max_depth {
            return Err(TextError::new(ValueRefusalKind::DepthLimit, format!("nesting exceeds max_depth limit ({} > {})", depth, self.max_depth), span));
        }
        Ok(())
    }

    pub fn check_tokens(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_tokens {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("token count exceeds max_tokens limit ({} > {})", count, self.max_tokens), span));
        }
        Ok(())
    }

    pub fn check_nodes(&self, count: usize, span: TextSpan) -> Result<(), TextError> {
        if count > self.max_nodes {
            return Err(TextError::new(ValueRefusalKind::WorkLimit, format!("node count exceeds max_nodes limit ({} > {})", count, self.max_nodes), span));
        }
        Ok(())
    }
}
//#endregion 🔖️Limits

//#region 🔖️FaultDescribeTests
#[cfg(test)]
#[path = "🧪️tests/🔬️fault-describe/🦀️.rs"]
mod fault_describe_tests;
#[cfg(test)]
#[path = "🧪️tests/🎛️controlled/🦀️.rs"]
mod controlled_tests;
//#endregion 🔖️FaultDescribeTests

#[cfg(test)]
#[path = "🚧️text-error/🧪️tests/🦀️.rs"]
mod text_error_refusal_tests;

#[path="📦️close/🦀️.rs"]
mod owned_close;
pub use owned_close::{FaultCloseOwner,FaultCloseStep};

#[cfg(test)]
#[path="🧾️retained/🧪️tests/🦀️.rs"]
mod retained_fault_tests;

#[path="📤️wire/👣️cursor/🦀️.rs"]
mod fault_wire_cursor;
pub use fault_wire_cursor::FaultWireCursor;

#[cfg(test)]
#[path="📤️wire/👣️cursor/🧪️tests/🦀️.rs"]
mod fault_wire_cursor_tests;
