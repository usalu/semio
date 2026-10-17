//! 📄️ Document model of the Part-21 codec: the header with its attribute tables, the document, the builder and the error type. See [`super`].

use super::*;

//#region 🔖️Header
/// 📇️ The three standard `HEADER;` records (`FILE_DESCRIPTION`/`FILE_NAME`/`FILE_SCHEMA`),
/// each a parenthesized tuple of typed values — kept verbatim, not schema-interpreted.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase", deny_unknown_fields )]
pub struct Part21Header {
    pub file_description: Vec<Part21Value>,
    pub file_name: Vec<Part21Value>,
    pub file_schema: Vec<Part21Value>,
}

/// 📜️ ISO 10303-21 §8.2.2's population constraint for one attribute of a mandatory `HEADER`
/// record: either a plain `STRING` or a `LIST[1:?] OF STRING`, which the standard forbids from
/// ever being empty. This is the schema of the exchange structure itself — fixed by the standard,
/// identical for every EXPRESS schema carried in it — so it lives with the syntax rather than with
/// any one artifact's typed view of it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum HeaderAttribute {
    Text,
    NonEmptyTextList,
}

/// 📜️ `FILE_DESCRIPTION(description, implementation_level)` — ISO 10303-21 §8.2.2.
pub(super) const FILE_DESCRIPTION_ATTRIBUTES: &[HeaderAttribute] = &[HeaderAttribute::NonEmptyTextList, HeaderAttribute::Text];

/// 📜️ `FILE_NAME(name, time_stamp, author, organization, preprocessor_version,
/// originating_system, authorization)` — ISO 10303-21 §8.2.3. `author` and `organization` are
/// `LIST[1:?]`, exactly like `FILE_DESCRIPTION.description`.
pub(super) const FILE_NAME_ATTRIBUTES: &[HeaderAttribute] = &[HeaderAttribute::Text, HeaderAttribute::Text, HeaderAttribute::NonEmptyTextList, HeaderAttribute::NonEmptyTextList, HeaderAttribute::Text, HeaderAttribute::Text, HeaderAttribute::Text];

/// 📜️ `FILE_SCHEMA(schema_identifiers)` — ISO 10303-21 §8.2.4, also `LIST[1:?]`.
pub(super) const FILE_SCHEMA_ATTRIBUTES: &[HeaderAttribute] = &[HeaderAttribute::NonEmptyTextList];

impl HeaderAttribute {
    /// 🈳️ The conformant spelling of "nothing to say here" for this attribute — `''` for a
    /// `STRING`, `('')` for a `LIST[1:?] OF STRING`. `()` is NOT that spelling: the standard's
    /// lower bound of one is a population constraint, and every conformant producer writes the
    /// one-empty-string list instead (the ruststep reference reader refuses `()` outright).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn unpopulated(self) -> Part21Value {
        match self {
            HeaderAttribute::Text => Part21Value::Str(String::new()),
            HeaderAttribute::NonEmptyTextList => Part21Value::List(vec![Part21Value::Str(String::new())]),
        }
    }
}

impl Part21Header {
    /// 🌱 ISO 10303-21 §8.2's conformant minimum `HEADER` — all three mandatory records present,
    /// each with its full attribute list and every `LIST[1:?]` populated. This is what `default()`
    /// means for a header: the previous all-empty derive produced `FILE_DESCRIPTION();`, which is
    /// neither the right arity nor a readable exchange structure.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn iso_10303_21_minimum() -> Self {
        Self {
            file_description: FILE_DESCRIPTION_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
            file_name: FILE_NAME_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
            file_schema: FILE_SCHEMA_ATTRIBUTES.iter().map(|attribute| attribute.unpopulated()).collect(),
        }
    }
}

impl Default for Part21Header {
    fn default() -> Self {
        Self::iso_10303_21_minimum()
    }
}
//#endregion 🔖️Header

//#region 🔖️Document
/// 📦️ The full, lossless generic Part-21 graph: header + every `DATA;` instance.
#[derive(Clone, Debug, PartialEq, Default, value_derive::ToValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(deny_unknown_fields )]
pub struct Part21Document {
    pub header: Part21Header,
    pub instances: Vec<Part21Instance>,
}

impl Part21Document {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn instance(&self, id: u64) -> Option<&Part21Instance> {
        self.instances.iter().find(|i| i.id == id)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn resolve(&self, value: &Part21Value) -> Option<&Part21Instance> {
        value.as_ref_id().and_then(|id| self.instance(id))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn by_type<'a>(&'a self, type_name: &'a str) -> impl Iterator<Item = &'a Part21Instance> + 'a {
        self.instances.iter().filter(move |i| i.is_type(type_name))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn next_id(&self) -> u64 {
        self.instances.iter().map(|i| i.id).max().unwrap_or(0) + 1
    }
}
//#endregion 🔖️Document

//#region 🔖️Builder
/// 🏗️ Shared incremental-id instance allocator — every Part-21 writer (step's BrepMesh writer,
/// ifc's future typed writers) builds its generated graph through this, so id allocation and
/// header shape stay consistent across artifacts instead of each hand-rolling a counter.
#[derive(Default)]
pub struct Part21Builder {
    pub instances: Vec<Part21Instance>,
    next_id: u64,
}

impl Part21Builder {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new() -> Self {
        Self { instances: Vec::new(), next_id: 1 }
    }
    /// ➕️ Allocates the next `#id` and appends a simple `TYPE(args)` instance.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn alloc(&mut self, type_name: &str, args: Vec<Part21Value>) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.push(Part21Instance { id, entities: vec![(type_name.to_string(), args)] });
        id
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn build(self, header: Part21Header) -> Part21Document {
        Part21Document { header, instances: self.instances }
    }
}
//#endregion 🔖️Builder

//#region 🔖️Error
/// 🚫️ Real parse failures — malformed input always surfaces here, never silently fabricated.
#[derive(Clone, Debug, PartialEq)]
pub enum Part21Error {
    UnexpectedEof { at: usize, expected: &'static str },
    UnexpectedChar { at: usize, found: char, expected: &'static str },
    UnsupportedEscape { at: usize, detail: String },
    InvalidNumber { at: usize, text: String },
    MissingLiteral { at: usize, literal: &'static str },
}

impl fmt::Display for Part21Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part21Error::UnexpectedEof { at, expected } => write!(f, "part21: unexpected end of input at char {at}, expected {expected}"),
            Part21Error::UnexpectedChar { at, found, expected } => write!(f, "part21: unexpected char {found:?} at {at}, expected {expected}"),
            Part21Error::UnsupportedEscape { at, detail } => write!(f, "part21: unsupported string escape at {at}: {detail}"),
            Part21Error::InvalidNumber { at, text } => write!(f, "part21: invalid number {text:?} at {at}"),
            Part21Error::MissingLiteral { at, literal } => write!(f, "part21: expected literal {literal:?} at {at}"),
        }
    }
}

impl std::error::Error for Part21Error {}

/// 🧷️ Unsupported escapes are unsupported grammar; every other lexical refusal is invalid input.
impl From<Part21Error> for ValueError {
    fn from(error: Part21Error) -> Self {
        let kind = match &error { Part21Error::UnsupportedEscape { .. } => semio_framework_value::ValueRefusalKind::UnsupportedOwner, _ => semio_framework_value::ValueRefusalKind::InvalidValue };
        Self::new(kind, error.to_string())
    }
}
//#endregion 🔖️Error
