//! 🏷️ Typed physical result sources share the canonical identity boundary.
use replication::wire::ScopedIdentityError;
use replication::wire::encode_scoped_identity;
use replication::wire::validate_scoped_identity_segments;
use replication::wire::visit_scoped_identity_segments;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ResultSource { LoadCase(String), Combination(String) }

impl ResultSource {
    pub fn id(&self) -> &str { match self { Self::LoadCase(id) | Self::Combination(id) => id } }
    pub fn kind(&self) -> &'static str { match self { Self::LoadCase(_) => "loadCase", Self::Combination(_) => "combination" } }
    pub fn from_parts(kind: &str, id: String) -> Option<Self> {
        validate_scoped_identity_segments(&[kind, &id]).ok()?;
        match kind { "loadCase" => Some(Self::LoadCase(id)), "combination" => Some(Self::Combination(id)), _ => None }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResultSourceIdentityError { Scope(ScopedIdentityError), Arity, Kind }

impl From<ScopedIdentityError> for ResultSourceIdentityError { fn from(error: ScopedIdentityError) -> Self { Self::Scope(error) } }
impl std::fmt::Display for ResultSourceIdentityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Self::Scope(error) => std::fmt::Display::fmt(error, formatter), Self::Arity => formatter.write_str("result source requires two identity segments"), Self::Kind => formatter.write_str("result source kind is not supported") }
    }
}
impl std::error::Error for ResultSourceIdentityError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResultSourceIdentity<'a> { pub kind: &'static str, pub id: &'a str }

/// 🪪️ Encodes one admitted typed source without losing its namespace.
pub fn encode_result_source_identity(source: &ResultSource) -> Result<String, ResultSourceIdentityError> {
    encode_result_source_parts(source.kind(), source.id())
}

/// 🧩️ Producers encode borrowed source parts without constructing an owned semantic key.
pub fn encode_result_source_parts(kind: &str, id: &str) -> Result<String, ResultSourceIdentityError> {
    validate_result_source_parts(kind, id)?;
    encode_scoped_identity(&[kind, id]).map_err(Into::into)
}

/// 🛡️ Validates producer namespaces and wire capacity without allocating.
pub fn validate_result_source_parts(kind: &str, id: &str) -> Result<(), ResultSourceIdentityError> {
    if kind != "loadCase" && kind != "combination" { return Err(ResultSourceIdentityError::Kind); }
    validate_scoped_identity_segments(&[kind, id]).map(|_| ()).map_err(Into::into)
}

/// 🔎️ Borrows a fully admitted source without allocating a segment vector or raw identifier.
pub fn borrow_result_source_identity(identity: &str) -> Result<ResultSourceIdentity<'_>, ResultSourceIdentityError> {
    let mut kind = None;
    let mut id = None;
    let mut cursor = 0;
    let count = visit_scoped_identity_segments(identity, |segment| { match cursor { 0 => kind = Some(segment), 1 => id = Some(segment), _ => {} } cursor += 1; })?;
    if count != 2 { return Err(ResultSourceIdentityError::Arity); }
    let kind = match kind { Some("loadCase") => "loadCase", Some("combination") => "combination", _ => return Err(ResultSourceIdentityError::Kind) };
    Ok(ResultSourceIdentity { kind, id: id.ok_or(ResultSourceIdentityError::Arity)? })
}

/// 🛡️ Controlled callers inspect only borrowed metadata from the admitted original identity.
pub fn validate_result_source_identity(identity: &str) -> Result<&'static str, ResultSourceIdentityError> {
    borrow_result_source_identity(identity).map(|source| source.kind)
}

/// 🧩️ Owns the semantic identifier only after canonical kind and arity admission.
pub fn decode_result_source_identity(identity: &str) -> Result<ResultSource, ResultSourceIdentityError> {
    let source = borrow_result_source_identity(identity)?;
    ResultSource::from_parts(source.kind, source.id.to_owned()).ok_or(ResultSourceIdentityError::Kind)
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
