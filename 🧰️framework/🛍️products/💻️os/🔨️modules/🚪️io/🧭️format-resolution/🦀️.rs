//! 🧭️ Format resolution: the one canonical precedence turning an explicit key, a file name or a MIME type into a registered format.
//!
//! Precedence is frozen and total: (1) an explicit kind, short id or alias is authoritative and never falls through;
//! (2) the longest registered extension suffix of the file name; (3) the exact MIME claim, parameters and case ignored;
//! (4) unresolved. There is no implicit fallback format: a caller that wants the raw bytes asks for the binary format by key.

use super::*;

/// 🏷️ The facts a caller knows about one source, any of which may be absent.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct FormatQuery<'a> {
    pub explicit: Option<&'a str>,
    pub file_name: Option<&'a str>,
    pub mime: Option<&'a str>,
}

/// 🪪️ Which claim decided a resolution.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormatResolutionBasis {
    Explicit,
    Extension,
    Mime,
}

/// ✅️ A resolved format with the decisive claim and the format the MIME claim alone would have chosen when it disagrees.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FormatResolution {
    pub descriptor: FormatDescriptor,
    pub basis: FormatResolutionBasis,
    pub mime_disagrees_with: Option<String>,
}

/// 🚫️ A resolution that must not guess.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FormatResolutionError {
    UnknownExplicit { input: String },
    Ambiguous { claim: String, kind_ids: Vec<String> },
    Unavailable(FormatRegistryError),
}

impl std::fmt::Display for FormatResolutionError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownExplicit { input } => write!(formatter, "unknown explicit format {input}"),
            Self::Ambiguous { claim, kind_ids } => write!(formatter, "claim {claim} is held by {}", kind_ids.join(", ")),
            Self::Unavailable(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for FormatResolutionError {}

fn extension_candidates(file_name: &str) -> Vec<String> {
    let base = file_name.rsplit(['/', '\\']).next().unwrap_or(file_name).trim().to_ascii_lowercase();
    base.char_indices().filter(|(_, character)| *character == '.').map(|(index, _)| base[index..].to_string()).filter(|extension| extension.len() > 1).collect()
}

fn normalized_mime(mime: &str) -> String {
    mime.split(';').next().unwrap_or_default().trim().to_ascii_lowercase()
}

fn holders<'a>(descriptors: &'a [FormatDescriptor], claim_of: impl Fn(&'a FormatDescriptor) -> bool) -> Vec<&'a FormatDescriptor> {
    let mut found: Vec<&FormatDescriptor> = Vec::new();
    for descriptor in descriptors.iter().filter(|descriptor| claim_of(descriptor)) {
        if !found.iter().any(|existing| existing.kind_id == descriptor.kind_id) {
            found.push(descriptor);
        }
    }
    found
}

fn single<'a>(claim: &str, found: Vec<&'a FormatDescriptor>) -> Result<Option<&'a FormatDescriptor>, FormatResolutionError> {
    match found.as_slice() {
        [] => Ok(None),
        [only] => Ok(Some(only)),
        many => Err(FormatResolutionError::Ambiguous { claim: claim.to_string(), kind_ids: many.iter().map(|descriptor| descriptor.kind_id.clone()).collect() }),
    }
}

/// 🧮️ Resolves one query against an explicit descriptor list without touching the global catalog.
pub fn resolve_format_in(descriptors: &[FormatDescriptor], query: &FormatQuery<'_>) -> Result<Option<FormatResolution>, FormatResolutionError> {
    if let Some(explicit) = query.explicit {
        let key = explicit.trim();
        let found = holders(descriptors, |descriptor| format_descriptor_keys(descriptor).any(|candidate| candidate == key));
        let descriptor = single(key, found)?.ok_or_else(|| FormatResolutionError::UnknownExplicit { input: key.to_string() })?;
        return Ok(Some(FormatResolution { descriptor: descriptor.clone(), basis: FormatResolutionBasis::Explicit, mime_disagrees_with: None }));
    }
    let mut by_extension = None;
    for extension in query.file_name.map(extension_candidates).unwrap_or_default() {
        if let Some(found) = single(&extension, holders(descriptors, |descriptor| descriptor.extensions.iter().any(|claim| *claim == extension)))? {
            by_extension = Some(found);
            break;
        }
    }
    let by_mime = || -> Result<Option<&FormatDescriptor>, FormatResolutionError> {
        match query.mime.map(normalized_mime).filter(|mime| !mime.is_empty()) {
            Some(mime) => single(&mime, holders(descriptors, |descriptor| format_mimes(descriptor).any(|claim| claim == mime))),
            None => Ok(None),
        }
    };
    Ok(match by_extension {
        Some(found) => Some(FormatResolution { descriptor: found.clone(), basis: FormatResolutionBasis::Extension, mime_disagrees_with: by_mime().ok().flatten().filter(|other| other.kind_id != found.kind_id).map(|other| other.kind_id.clone()) }),
        None => by_mime()?.map(|found| FormatResolution { descriptor: found.clone(), basis: FormatResolutionBasis::Mime, mime_disagrees_with: None }),
    })
}

/// 🗂️ Resolves one query against the registered global format catalog.
pub fn resolve_format(query: &FormatQuery<'_>) -> Result<Option<FormatResolution>, FormatResolutionError> {
    let descriptors: Vec<FormatDescriptor> = format_catalog().read().map_err(|_| FormatResolutionError::Unavailable(FormatRegistryError::Unavailable(IoRegistryUnavailable { registry: "format-catalog" })))?.values().cloned().collect();
    resolve_format_in(&descriptors, query)
}

/// 🔬️ One revision's declared content check among the revisions that share a claim set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RevisionSniff {
    pub kind_id: String,
    pub window: usize,
    pub contains: Vec<String>,
}

/// 🎯️ Picks the revision whose declared needle occurs in the first `window` bytes of `head`; no match falls back to the declared owner, so a file no revision recognises still opens in the most complete one.
pub fn pick_revision<'a>(owner_kind_id: &'a str, revisions: &'a [RevisionSniff], head: &[u8]) -> &'a str {
    revisions
        .iter()
        .find(|revision| {
            let bounded = &head[..head.len().min(revision.window)];
            revision.contains.iter().any(|needle| !needle.is_empty() && bounded.windows(needle.len()).any(|candidate| candidate == needle.as_bytes()))
        })
        .map_or(owner_kind_id, |revision| revision.kind_id.as_str())
}
