//! 🪪️ RFC 7493 I-JSON conformance of an owned `JsonSnapshot`, the single source of truth for the composer, builder, analyzer and the SQLite snapshot guard.
use crate::standards::v_rfc8259::subsets::base::schema::snapshot::{JsonSnapshot, JsonValue};
use semio_framework_diagnostic::{Diagnostic, FaultCode, FaultScope, Severity, TextSpan};

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

//#region 🧩️ArtifactItems
pub mod artifact_items {
/// 🏷️ Document schema / DSL envelope id.
pub const STDIO_JSON_DOCUMENT_SCHEMA: &str = "stdio.json";
}
//#endregion 🧩️ArtifactItems
