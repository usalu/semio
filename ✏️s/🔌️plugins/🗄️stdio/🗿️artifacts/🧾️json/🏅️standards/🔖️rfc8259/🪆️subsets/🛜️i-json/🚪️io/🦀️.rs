//! 🚪️ IO stdio.json (rfc8259/🛜️i-json) — reuses the ✳️any subset's `txt` raw-codec DAG leaf
//! rather than duplicating it (same `JsonSnapshot` type, same catalog DAG edges). Registration
//! flows through `🎹️composer::register` (the `ComposerEntry` via the standard-level aggregator,
//! and the `SubsetValidator` directly), not per-leaf `register()` — same pattern `✳️any/🚪️io`
//! already established for this artifact.
//#region 🎹️DerivedComposition
pub mod derived_composition {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::base::io::JsonComposer as JsonAnyComposer;
    use crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance;
    use semio_framework_diagnostic::Diagnostic;
use crate::apply_mutation;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::register_subset_validator,semio_framework_plugin::io::subset_validator_entry_of,semio_framework_plugin::ArtifactComposition,semio_framework_plugin::io::ComposeError,semio_framework_plugin::io::ComposeSource,semio_framework_plugin::io::Composition,semio_framework_artifact_reference::Dialect,semio_framework_plugin::io::IoPayload,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId,semio_framework_plugin::io::SubsetValidator,semio_framework_plugin::io::SubsetValidatorEntry};
    use std::sync::OnceLock;

    const DIALECT_I_JSON: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };
    const DIALECT_ANY: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("*") };
    const DEP_TXT: Dialect = Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") };

    //#region 🔖️Composer
    pub struct JsonIJsonComposerComposition;

    impl ArtifactComposition for JsonIJsonComposerComposition {
        type Snapshot = JsonSnapshot;
        const WRITES: Dialect = DIALECT_I_JSON;

        fn reads() -> &'static [Dialect] {
            &[DIALECT_ANY, DIALECT_I_JSON, DEP_TXT]
        }

        fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> {
            let inner = JsonAnyComposer::compose(sources)?;
            let checks = check_i_json_conformance(&inner.snapshot);
            let (hard, soft): (Vec<Diagnostic>, Vec<Diagnostic>) = checks.into_iter().partition(|d| matches!(d.severity, Severity::Error | Severity::Fatal));
            if !hard.is_empty() {
                let mut all = hard.clone();
                all.extend(soft);
                return Err(ComposeError { message: format!("I-JSON (RFC 7493) conformance violated: {} hard issue(s) -- not stamping the i-json dialect", hard.len()), diagnostics: all });
            }
            let mut diagnostics = inner.diagnostics;
            diagnostics.extend(soft);
            Ok(Composition { snapshot: inner.snapshot, confidence: inner.confidence, diagnostics })
        }
    }
    //#endregion 🔖️Composer

    //#region 🔖️SubsetValidator
    /// 🛡️ The registered `SubsetValidator` for `rfc8259/i-json`.
    pub struct JsonIJsonValidator;

    impl SubsetValidator for JsonIJsonValidator {
        const DIALECT: Dialect = DIALECT_I_JSON;

        async fn validate(payload: &IoPayload) -> Vec<Diagnostic> {
            let decoded = match payload {
                IoPayload::Binary(bytes) => <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes).ok(),
                IoPayload::Text(text) => <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text).ok(),
            };
            match decoded {
                Some(snapshot) => check_i_json_conformance(&snapshot),
                None => vec![Diagnostic {
                    code: FaultCode::new("stdio.json.i-json.validate-decode-failed"),
                    severity: Severity::Warning,
                    span: TextSpan::at(1, 1),
                    message: "I-JSON SubsetValidator: payload did not decode as a JsonSnapshot -- skipped".into(),
                    expected: None,
                    scope: semio_framework_diagnostic::FaultScope::default(),
                }],
            }
        }
    }

    static VALIDATOR_ENTRY: OnceLock<SubsetValidatorEntry> = OnceLock::new();

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn validator_entry() -> &'static SubsetValidatorEntry {
        VALIDATOR_ENTRY.get_or_init(subset_validator_entry_of::<JsonIJsonValidator>)
    }

    /// 📌️ Registers this subset's `SubsetValidator` with the generic io registry (D5's
    /// validate-on-build hook). Called from the rfc8259 standard's own `⚙️engine::register()`. The
    /// `ComposerEntry` itself is registered separately by the standard-level composer aggregator
    /// (`crate::standards::v_rfc8259::subsets::base::io::io_registry::entries()`).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn register() {
        register_subset_validator(validator_entry()).expect("static Stdio registration must be available and conflict-free");
    }
    //#endregion 🔖️SubsetValidator

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-composition-unit/🦀️.rs");
}
pub use derived_composition::*;
//#endregion 🎹️DerivedComposition

pub mod derived_construction {
    use crate::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::i_json::io::check_i_json_conformance;
    use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::{JsonIJsonMutation};
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::Severity;
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    /// 🏗️ Holds the snapshot directly rather than wrapping the ✳️any builder, because this subset's
    /// `Mutation` is its OWN `JsonIJsonMutation` (see `mutations/🦀️.rs`) and not the ✳️any
    /// sibling's -- the same shape `ZipIso21320BuilderConstruction` already uses for the same reason.
    /// `from_text`/`from_binary` stay exactly the ✳️any codec (`ArtifactDsl`/`ArtifactPack` on the
    /// SHARED `JsonSnapshot`); only the mutation vocabulary and the RFC 7493 build gate are this
    /// subset's own.
    #[derive(Clone, Debug, Default)]
    pub struct JsonIJsonBuilderConstruction {
        snapshot: JsonSnapshot,
    }

    impl ArtifactBuilder for JsonIJsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonIJsonMutation;
        type Diff = JsonDiff;

        fn empty() -> Self {
            Self { snapshot: JsonSnapshot::default() }
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self { snapshot }
        }

        fn from_text(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes)? })
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = apply_mutation(&mut self.snapshot, &mutation);
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = protocol::apply_diff(&diff, &self.snapshot)?;
            Ok(self)
        }

        /// 🛡️ The real construction gate: however the inner snapshot got here, a hard RFC 7493
        /// violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`. Two doors now
        /// lead to the same clauses: this gate rejects a violating STATE, and `JsonIJsonMutation`'s
        /// own `diff()` rejects a violating EDIT before it can produce one.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let hard: Vec<Diagnostic> = check_i_json_conformance(&self.snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(self.snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;

pub mod derived_analysis {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
    use crate::standards::v_rfc8259::subsets::base::io::JsonAnalyzer as JsonAnyAnalyzer;
    pub use crate::standards::v_rfc8259::subsets::base::io::JsonParts;
    use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
    use {semio_framework_plugin::io::Analysis,semio_framework_plugin::io::AnalyzeSource,semio_framework_plugin::ArtifactAnalysis,semio_framework_artifact_reference::Dialect,semio_framework_artifact_reference::StandardId,semio_framework_artifact_reference::SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };

    //#region 🔖️Conformance
    pub const CODE_DUPLICATE_MEMBER: &str = "stdio.json.i-json.duplicate-member-name";
    pub const CODE_UNSAFE_INTEGER: &str = "stdio.json.i-json.unsafe-integer";
    pub const CODE_INVALID_NUMBER_LEXEME: &str = "stdio.json.i-json.invalid-number-lexeme";
    pub const CODE_NUMBER_NOT_BINARY64: &str = "stdio.json.i-json.number-not-binary64";
    pub const CODE_TOP_LEVEL_SCALAR: &str = "stdio.json.i-json.top-level-scalar";
    pub const CODE_STRING_NONCHARACTER: &str = "stdio.json.i-json.string-noncharacter";

    /// ± the largest integer magnitude exactly representable as an IEEE-754 double (2^53 - 1).
    const MAX_SAFE_INTEGER_MAGNITUDE: i128 = 9_007_199_254_740_991;

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🔢️ Is this number lexeme an integer (no fractional part, no exponent)? Per RFC8259's grammar,
    /// `.`/`e`/`E` only ever appear in the fraction/exponent parts.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_integer_lexeme(lexeme: &str) -> bool {
        !lexeme.contains('.') && !lexeme.contains('e') && !lexeme.contains('E')
    }

    /// 🔁️ Recursive scan: every integer number's magnitude against RFC 7493 §2.2's ±(2^53-1) safe
    /// bound, using the ORIGINAL LEXEME (never a lossy `f64` parse) so arbitrary-precision integers
    /// are checked exactly.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_unsafe_integers(value: &JsonValue, out: &mut Vec<Diagnostic>) {
        match value {
            JsonValue::Number { lexeme } if is_integer_lexeme(lexeme) => match lexeme.parse::<i128>() {
                Ok(n) if n.unsigned_abs() > MAX_SAFE_INTEGER_MAGNITUDE as u128 => {
                    out.push(hard(CODE_UNSAFE_INTEGER, format!("integer {lexeme} exceeds ±(2^53-1) = ±{MAX_SAFE_INTEGER_MAGNITUDE} and is not exactly representable as an IEEE-754 double -- RFC 7493 §2.2 forbids this for I-JSON")));
                }
                Ok(_) => {}
                Err(_) => {
                    // Too large even for i128 -- definitely exceeds the much smaller 2^53-1 bound.
                    out.push(hard(CODE_UNSAFE_INTEGER, format!("integer {lexeme} is far larger than ±(2^53-1) and is not exactly representable as an IEEE-754 double -- RFC 7493 §2.2 forbids this for I-JSON")));
                }
            },
            JsonValue::Number { .. } => {}
            _ => {}
        }
    }

    /// 🚫️ A Unicode noncharacter per the Unicode Standard: the last two code points of every plane
    /// (`cp & 0xFFFE == 0xFFFE` covers U+FFFE/U+FFFF, U+1FFFE/U+1FFFF, ..., U+10FFFE/U+10FFFF) plus
    /// the reserved BMP range U+FDD0-U+FDEF.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn is_unicode_noncharacter(c: char) -> bool {
        let cp = c as u32;
        (cp & 0xFFFE) == 0xFFFE || (0xFDD0..=0xFDEF).contains(&cp)
    }

    /// 🛡️ Real RFC 7493 I-JSON conformance checks against one already-decoded `JsonSnapshot`. Shared
    /// single source of truth: `JsonIJsonComposer::compose` hard-gates on this (pre-serialization,
    /// authoritative), `JsonIJsonBuilder::build` hard-gates on this too, and the registered
    /// `SubsetValidator` re-runs it post-hoc against the wire payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    /// 🛡️ Applies the same I-JSON rules with bounded typed traversal and transfer cancellation.
    pub fn check_i_json_conformance_controlled(snapshot:&JsonSnapshot,control:&mut semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl<'_>)->Result<Vec<Diagnostic>,semio_framework_os_kernel::sqlite_snapshot::ValueError>{
        use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotPhase, ValueError, ValueRefusalKind};
        enum Work<'a>{Value(&'a JsonValue),Object(std::slice::Iter<'a,crate::schema::snapshot::JsonMember>,std::collections::HashSet<&'a str>),Array(std::slice::Iter<'a,JsonValue>)}
        let mut out=Vec::new();let mut count=0usize;control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;
        if !matches!(snapshot.value,JsonValue::Object{..}|JsonValue::Array{..}){out.push(soft(CODE_TOP_LEVEL_SCALAR,"top-level value is neither an object nor an array -- RFC 7493 §2.1 recommends against a bare top-level scalar for interop".into()));}
        for pass in 0..3{let mut rows=0usize;let mut pending=vec![Work::Value(&snapshot.value)];while let Some(work)=pending.pop(){if matches!(&work,Work::Value(_)){rows=rows.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "I-JSON conformance row count overflow"))?;control.check_rows(rows)?;}count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "I-JSON validation unit count overflow"))?;if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}
            match work{
                Work::Value(JsonValue::Object{members})=>pending.push(Work::Object(members.iter(),std::collections::HashSet::new())),
                Work::Value(JsonValue::Array{items})=>pending.push(Work::Array(items.iter())),
                Work::Object(mut members,mut seen)=>{if let Some(member)=members.next(){if pass==0&&!seen.insert(member.key.as_str()){if member.key.len()>65536{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}out.push(hard(CODE_DUPLICATE_MEMBER,format!("object member name '{}' appears more than once -- RFC 7493 §2.3 forbids duplicate member names within one object",member.key)));}pending.push(Work::Object(members,seen));pending.push(Work::Value(&member.value));}},
                Work::Array(mut items)=>{if let Some(value)=items.next(){pending.push(Work::Array(items));pending.push(Work::Value(value));}},
                Work::Value(value@JsonValue::Number{lexeme}) if pass==1=>{let meaning=crate::schema::snapshot::number::meaning(lexeme,&mut ||control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0))?;if !meaning.valid{out.push(hard(CODE_INVALID_NUMBER_LEXEME,"a number requires an RFC8259 lexeme".into()));}else if meaning.numeric.is_none(){out.push(hard(CODE_NUMBER_NOT_BINARY64,"a number exceeds finite IEEE754 binary64 representation -- RFC7493 §2.2".into()));}else{scan_unsafe_integers(value,&mut out);}},
                Work::Value(JsonValue::String{value}) if pass==2=>{let mut noncharacter=false;for c in value.chars(){count=count.checked_add(1).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "I-JSON validation unit count overflow"))?;noncharacter|=is_unicode_noncharacter(c);if count%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,0)?;}}if noncharacter{out.push(soft(CODE_STRING_NONCHARACTER,format!("string {value:?} contains a Unicode noncharacter (U+FFFE/U+FFFF, U+FDD0-U+FDEF, or a per-plane equivalent) -- RFC 7493 §2.3 advises against these in I-JSON text")));}},
                _=>{}
            }
        }}control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,count,count)?;Ok(out)
    }

    pub fn check_i_json_conformance(snapshot: &JsonSnapshot) -> Vec<Diagnostic> {
        let mut proceed=|_|true;
        let mut control=semio_framework_os_kernel::sqlite_snapshot::SqliteSnapshotControl::new(&mut proceed,semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits{max_rows:usize::MAX,..semio_framework_os_kernel::sqlite_snapshot::SqliteDatabaseLimits::default()});
        check_i_json_conformance_controlled(snapshot,&mut control).expect("borrowed I-JSON conformance without cancellation")
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.json` (rfc8259/🛜️i-json): delegates the real parse to the ✳️any subset's
    /// analyzer (same `JsonSnapshot`), then folds real I-JSON conformance diagnostics on top.
    pub struct JsonIJsonAnalyzerAnalysis;

    impl ArtifactAnalysis for JsonIJsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> semio_framework_plugin::io::Confidence {
            JsonAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JsonAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_i_json_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = semio_framework_plugin::io::Confidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("../🧬️schema/🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;

semio_framework_plugin::derive_artifact_facets!(
    pub spec JsonIJsonBuilderFacets {
        construction: JsonIJsonBuilderConstruction,
        analysis: JsonIJsonAnalyzerAnalysis,
        composition: crate::standards::v_rfc8259::subsets::i_json::io::derived_composition::JsonIJsonComposerComposition,
    }
    builder: JsonIJsonBuilder,
    analyzer: JsonIJsonAnalyzer,
    composer: JsonIJsonComposer,
);
