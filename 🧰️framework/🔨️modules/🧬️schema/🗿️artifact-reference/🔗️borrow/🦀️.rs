//! 🔗️ Allocation-free artifact reference fields and canonical kind grammar.

/// 🪪️ One exact borrowed target identity independent of value codecs and registries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BorrowedArtifactRef<'a> {
    pub artifact_id: &'a str,
    pub artifact_kind: &'a str,
    pub standard: &'a str,
    pub subset: &'a str,
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
