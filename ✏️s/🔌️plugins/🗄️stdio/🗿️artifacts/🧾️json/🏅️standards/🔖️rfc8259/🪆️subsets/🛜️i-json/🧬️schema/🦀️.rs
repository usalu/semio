//! 🧬️ JsonSnapshot schema (rfc8259/🛜️i-json) — reuses the ✳️any subset's `JsonSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.json` schema id). RFC 7493 I-JSON is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/🛜️i-json/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. The underlying `JsonValue::Object(Vec<JsonMember>)` shape
//! (see the ✳️any schema) is what makes duplicate member names genuinely representable/checkable
//! here -- a `serde_json::Value`-style `Map` would have silently collapsed them on parse.

pub use crate::standards::v_rfc8259::subsets::base::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::standards::v_rfc8259::subsets::base::schema::diff::JsonDiff;
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot;
    use crate::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    use crate::standards::v_rfc8259::subsets::i_json::schema::mutations::{apply_json_i_json_mutation, JsonIJsonMutation};
    use dsl::{Diagnostic, Severity};
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

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactDsl>::parse_dsl(text)? })
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self { snapshot: <JsonSnapshot as store::ArtifactPack>::decode_pack(bytes)? })
        }

        fn mutate(mut self, mutation: Self::Mutation) -> (Self, protocol::MutationOutcome<Self::Diff>) {
            let outcome = apply_json_i_json_mutation(&mut self.snapshot, &mutation);
            (self, outcome)
        }

        fn absorb(mut self, diff: Self::Diff) -> protocol::MutationApplyResult<Self> {
            self.snapshot = <JsonDiff as protocol::MutationDiff<JsonSnapshot>>::apply(&diff, &self.snapshot)?;
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
    include!("🧪️tests/🔬️derived-construction-unit/🦀️.rs");
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
    use crate::standards::v_rfc8259::subsets::base::schema::JsonAnalyzer as JsonAnyAnalyzer;
    pub use crate::standards::v_rfc8259::subsets::base::schema::JsonParts;
    use dsl::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};
    use semio_framework_plugin::{Analysis, AnalyzeSource, ArtifactAnalysis, Dialect, IoConfidence, StandardId, SubsetId};

    /// 🎯️ This subset's dialect coordinate.
    pub const DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.json", standard: StandardId("rfc8259"), subset: SubsetId("i-json") };

    //#region 🔖️Conformance
    pub const CODE_DUPLICATE_MEMBER: &str = "stdio.json.i-json.duplicate-member-name";
    pub const CODE_UNSAFE_INTEGER: &str = "stdio.json.i-json.unsafe-integer";
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

    /// 🔁️ Recursive scan: every object's member names, checked for duplicates independently at each
    /// nesting level (a duplicate at a nested object doesn't affect its ancestors' own uniqueness).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_duplicate_members(value: &JsonValue, out: &mut Vec<Diagnostic>) {
        match value {
            JsonValue::Object { members } => {
                let mut seen: std::collections::HashSet<&str> = std::collections::HashSet::new();
                for member in members {
                    if !seen.insert(member.key.as_str()) {
                        out.push(hard(CODE_DUPLICATE_MEMBER, format!("object member name '{}' appears more than once -- RFC 7493 §2.3 forbids duplicate member names within one object", member.key)));
                    }
                    scan_duplicate_members(&member.value, out);
                }
            }
            JsonValue::Array { items } => {
                for item in items {
                    scan_duplicate_members(item, out);
                }
            }
            _ => {}
        }
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
            JsonValue::Object { members } => {
                for member in members {
                    scan_unsafe_integers(&member.value, out);
                }
            }
            JsonValue::Array { items } => {
                for item in items {
                    scan_unsafe_integers(item, out);
                }
            }
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

    /// 🔁️ Recursive scan: every string value for embedded Unicode noncharacters.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn scan_noncharacter_strings(value: &JsonValue, out: &mut Vec<Diagnostic>) {
        match value {
            JsonValue::String { value: s } => {
                if s.chars().any(is_unicode_noncharacter) {
                    out.push(soft(CODE_STRING_NONCHARACTER, format!("string {s:?} contains a Unicode noncharacter (U+FFFE/U+FFFF, U+FDD0-U+FDEF, or a per-plane equivalent) -- RFC 7493 §2.3 advises against these in I-JSON text")));
                }
            }
            JsonValue::Object { members } => {
                for member in members {
                    scan_noncharacter_strings(&member.value, out);
                }
            }
            JsonValue::Array { items } => {
                for item in items {
                    scan_noncharacter_strings(item, out);
                }
            }
            _ => {}
        }
    }

    /// 🛡️ Real RFC 7493 I-JSON conformance checks against one already-decoded `JsonSnapshot`. Shared
    /// single source of truth: `JsonIJsonComposer::compose` hard-gates on this (pre-serialization,
    /// authoritative), `JsonIJsonBuilder::build` hard-gates on this too, and the registered
    /// `SubsetValidator` re-runs it post-hoc against the wire payload for the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_i_json_conformance(snapshot: &JsonSnapshot) -> Vec<Diagnostic> {
        let mut out = Vec::new();
        if !matches!(snapshot.value, JsonValue::Object { .. } | JsonValue::Array { .. }) {
            out.push(soft(CODE_TOP_LEVEL_SCALAR, "top-level value is neither an object nor an array -- RFC 7493 §2.1 recommends against a bare top-level scalar for interop".into()));
        }
        scan_duplicate_members(&snapshot.value, &mut out);
        scan_unsafe_integers(&snapshot.value, &mut out);
        scan_noncharacter_strings(&snapshot.value, &mut out);
        out
    }
    //#endregion 🔖️Conformance

    //#region 🔖️Analyzer
    /// 🧐️ Analyzes `stdio.json` (rfc8259/🛜️i-json): delegates the real parse to the ✳️any subset's
    /// analyzer (same `JsonSnapshot`), then folds real I-JSON conformance diagnostics on top.
    pub struct JsonIJsonAnalyzerAnalysis;

    impl ArtifactAnalysis for JsonIJsonAnalyzerAnalysis {
        type Parts = JsonParts;
        const DIALECT: Dialect = DIALECT;

        fn sniff(source: &AnalyzeSource<'_>) -> IoConfidence {
            JsonAnyAnalyzer::sniff(source)
        }

        fn analyze(sources: &[AnalyzeSource<'_>]) -> Analysis<Self::Parts> {
            let inner = JsonAnyAnalyzer::analyze(sources);
            let mut diagnostics = inner.diagnostics.clone();
            let mut confidence = inner.confidence;
            if let Some(snapshot) = &inner.parts.snapshot {
                let checks = check_i_json_conformance(snapshot);
                if checks.iter().any(|d| matches!(d.severity, Severity::Error | Severity::Fatal)) {
                    confidence = IoConfidence::Low;
                }
                diagnostics.extend(checks);
            }
            Analysis { parts: inner.parts, dialect: DIALECT, confidence, diagnostics }
        }
    }
    //#endregion 🔖️Analyzer

    #[cfg(test)]
    include!("🧪️tests/🔬️derived-analysis-unit/🦀️.rs");
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
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
//#endregion 🧬️DerivedArtifactFacets
