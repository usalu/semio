//! 🧬️ JsonSnapshot schema (rfc8259/✳️i-json) — reuses the ✳️any subset's `JsonSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.json` schema id). RFC 7493 I-JSON is a validation-gated
//! dialect STAMP on top of that existing schema, not a new one -- see D4's Tier-1 "same snapshot
//! type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This leaf exists so
//! `🪆️subsets/✳️i-json/🧬️schema/` is present per `🔣️taxonomy.json`'s `subsetChildDirs`, without
//! duplicating the schema definition. The underlying `JsonValue::Object(Vec<JsonMember>)` shape
//! (see the ✳️any schema) is what makes duplicate member names genuinely representable/checkable
//! here -- a `serde_json::Value`-style `Map` would have silently collapsed them on parse.

pub use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::*;
//#region 🏗️DerivedConstruction
pub mod derived_construction {
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::diff::JsonDiff;
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::mutations::JsonMutation;
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::JsonSnapshot;
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonBuilder as JsonAnyBuilder;
    use crate::artifacts::json::standards::v_rfc8259::subsets::i_json::schema::check_i_json_conformance;
    use dsl::{Diagnostic, Severity};
    use semio_framework_plugin::ArtifactBuilder;

    //#region 🔖️Builder
    #[derive(Clone, Debug, Default)]
    pub struct JsonIJsonBuilderConstruction(JsonAnyBuilder);

    impl ArtifactBuilder for JsonIJsonBuilderConstruction {
        type Snapshot = JsonSnapshot;
        type Mutation = JsonMutation;
        type Diff = JsonDiff;

        fn empty() -> Self {
            Self(JsonAnyBuilder::empty())
        }

        fn from_snapshot(snapshot: Self::Snapshot) -> Self {
            Self(JsonAnyBuilder::from_snapshot(snapshot))
        }

        fn from_text(text: &str) -> Result<Self, store::TextError> {
            Ok(Self(JsonAnyBuilder::from_text(text)?))
        }

        fn from_binary(bytes: &[u8]) -> Result<Self, store::PackError> {
            Ok(Self(JsonAnyBuilder::from_binary(bytes)?))
        }

        fn mutate(self, mutation: Self::Mutation) -> (Self, Self::Diff) {
            let (inner, diff) = self.0.mutate(mutation);
            (Self(inner), diff)
        }

        fn absorb(self, diff: Self::Diff) -> Self {
            Self(self.0.absorb(diff))
        }

        /// 🛡️ The real construction gate: however `self.0`'s inner snapshot got here, a hard RFC 7493
        /// violation fails `build()` -- soft/advisory diagnostics pass through as `Ok`.
        fn build(self) -> Result<Self::Snapshot, Vec<Diagnostic>> {
            let snapshot = self.0.build()?;
            let hard: Vec<Diagnostic> = check_i_json_conformance(&snapshot).into_iter().filter(|d| matches!(d.severity, Severity::Error | Severity::Fatal)).collect();
            if hard.is_empty() {
                Ok(snapshot)
            } else {
                Err(hard)
            }
        }
    }
    //#endregion 🔖️Builder

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn conforming_snapshot_builds_clean() {
            let snapshot = JsonIJsonBuilderConstruction::from_text("{\"a\":1}").expect("parses").build().expect("conforming construction must build");
            assert!(matches!(snapshot.value, crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::JsonValue::Object { .. }));
        }

        #[test]
        fn duplicate_member_name_fails_build() {
            let err = JsonIJsonBuilderConstruction::from_text("{\"a\":1,\"a\":2}").expect("parses").build().expect_err("a duplicate member name must fail build()");
            assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.duplicate-member-name"));
        }

        #[test]
        fn unsafe_integer_injected_via_raw_mutate_still_fails_build() {
            use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::{JsonMember, JsonValue};
            let violating = JsonSnapshot { value: JsonValue::Object { members: vec![JsonMember { key: "n".into(), value: JsonValue::Number { lexeme: "9007199254740993".into() } }] }, ..JsonSnapshot::default() };
            let (mutated, _diff) = JsonIJsonBuilderConstruction::from_snapshot(JsonSnapshot::default()).mutate(JsonMutation::SetSnapshot { snapshot: violating });
            let err = mutated.build().expect_err("an unsafe integer must fail build()");
            assert!(err.iter().any(|d| d.code.0 == "stdio.json.i-json.unsafe-integer"));
        }
    }
}
pub use derived_construction::*;
//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis
pub mod derived_analysis {
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::{JsonSnapshot, JsonValue};
    use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonAnalyzer as JsonAnyAnalyzer;
    pub use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::JsonParts;
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

    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🔁️ Recursive scan: every object's member names, checked for duplicates independently at each
    /// nesting level (a duplicate at a nested object doesn't affect its ancestors' own uniqueness).
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
    fn is_integer_lexeme(lexeme: &str) -> bool {
        !lexeme.contains('.') && !lexeme.contains('e') && !lexeme.contains('E')
    }

    /// 🔁️ Recursive scan: every integer number's magnitude against RFC 7493 §2.2's ±(2^53-1) safe
    /// bound, using the ORIGINAL LEXEME (never a lossy `f64` parse) so arbitrary-precision integers
    /// are checked exactly.
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
    fn is_unicode_noncharacter(c: char) -> bool {
        let cp = c as u32;
        (cp & 0xFFFE) == 0xFFFE || (0xFDD0..=0xFDEF).contains(&cp)
    }

    /// 🔁️ Recursive scan: every string value for embedded Unicode noncharacters.
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
    /// 🧐️ Analyzes `stdio.json` (rfc8259/✳️i-json): delegates the real parse to the ✳️any subset's
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
    mod tests {
        use super::*;
        use crate::artifacts::json::standards::v_rfc8259::subsets::any::schema::snapshot::JsonMember;

        fn obj(pairs: Vec<(&str, JsonValue)>) -> JsonValue {
            JsonValue::Object { members: pairs.into_iter().map(|(k, v)| JsonMember { key: k.into(), value: v }).collect() }
        }

        fn snapshot(value: JsonValue) -> JsonSnapshot {
            JsonSnapshot { value, ..JsonSnapshot::default() }
        }

        #[test]
        fn conforming_object_reports_nothing() {
            let value = obj(vec![("a", JsonValue::Number { lexeme: "1".into() }), ("b", JsonValue::String { value: "hi".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.is_empty(), "got {diagnostics:?}");
        }

        #[test]
        fn duplicate_member_name_is_hard() {
            let value = JsonValue::Object { members: vec![JsonMember { key: "a".into(), value: JsonValue::Number { lexeme: "1".into() } }, JsonMember { key: "a".into(), value: JsonValue::Number { lexeme: "2".into() } }] };
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DUPLICATE_MEMBER && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn nested_duplicate_member_name_is_detected_recursively() {
            let inner = JsonValue::Object { members: vec![JsonMember { key: "x".into(), value: JsonValue::Null }, JsonMember { key: "x".into(), value: JsonValue::Null }] };
            let value = obj(vec![("outer", inner)]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_DUPLICATE_MEMBER), "got {diagnostics:?}");
        }

        #[test]
        fn integer_within_safe_bound_is_clean() {
            let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740991".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_UNSAFE_INTEGER), "got {diagnostics:?}");
        }

        #[test]
        fn integer_beyond_safe_bound_is_hard() {
            let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740993".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn negative_integer_beyond_safe_bound_is_hard() {
            let value = obj(vec![("n", JsonValue::Number { lexeme: "-9007199254740993".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn astronomically_large_integer_is_hard() {
            let value = obj(vec![("n", JsonValue::Number { lexeme: "100000000000000000000000000000".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_UNSAFE_INTEGER && d.severity == Severity::Error), "got {diagnostics:?}");
        }

        #[test]
        fn fractional_number_is_never_flagged_as_unsafe_integer() {
            let value = obj(vec![("n", JsonValue::Number { lexeme: "9007199254740993.5".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_UNSAFE_INTEGER), "got {diagnostics:?}");
        }

        #[test]
        fn top_level_scalar_is_soft() {
            let diagnostics = check_i_json_conformance(&snapshot(JsonValue::String { value: "hi".into() }));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_TOP_LEVEL_SCALAR && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn top_level_array_is_clean_on_that_check() {
            let diagnostics = check_i_json_conformance(&snapshot(JsonValue::Array { items: vec![] }));
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_TOP_LEVEL_SCALAR), "got {diagnostics:?}");
        }

        #[test]
        fn noncharacter_in_string_is_soft() {
            let value = obj(vec![("s", JsonValue::String { value: "abc\u{FFFE}def".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRING_NONCHARACTER && d.severity == Severity::Warning), "got {diagnostics:?}");
        }

        #[test]
        fn reserved_bmp_noncharacter_range_is_detected() {
            let value = obj(vec![("s", JsonValue::String { value: "\u{FDD5}".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().any(|d| d.code.0 == CODE_STRING_NONCHARACTER), "got {diagnostics:?}");
        }

        #[test]
        fn ordinary_string_has_no_noncharacter_diagnostic() {
            let value = obj(vec![("s", JsonValue::String { value: "hello world".into() })]);
            let diagnostics = check_i_json_conformance(&snapshot(value));
            assert!(diagnostics.iter().all(|d| d.code.0 != CODE_STRING_NONCHARACTER), "got {diagnostics:?}");
        }
    }
}
pub use derived_analysis::*;
//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets
semio_framework_plugin::derive_artifact_facets!(
    pub spec JsonIJsonBuilderFacets {
        construction: derived_construction::JsonIJsonBuilderConstruction,
        analysis: derived_analysis::JsonIJsonAnalyzerAnalysis,
        composition: crate::artifacts::json::standards::v_rfc8259::subsets::i_json::io::derived_composition::JsonIJsonComposerComposition,
    }
    builder: JsonIJsonBuilder,
    analyzer: JsonIJsonAnalyzer,
    composer: JsonIJsonComposer,
);
//#endregion 🧬️DerivedArtifactFacets
