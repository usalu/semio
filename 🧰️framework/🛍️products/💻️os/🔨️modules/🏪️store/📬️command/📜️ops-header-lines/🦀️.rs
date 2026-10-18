//! 📜️ Ops header lines: the ops author, pin and superseded input, their conversions and the ops header line with its text and binary codecs. See [`super`].

use super::*;

/// 🖋️ One `by=[...]` list entry on a `commit` line: id then name, both positional
/// (bare-preferred, quoted only when needed — e.g. a name containing a space), then the optional
/// avatar — every field the transition's content-addressed id covers.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsAuthor {
    #[dsl(positional)]
    pub id: String,
    #[dsl(positional)]
    pub name: String,
    pub avatar: Option<String>,
}

impl From<&Author> for OpsAuthor {
    fn from(author: &Author) -> Self {
        Self { id: author.id.clone(), name: author.name.clone(), avatar: author.avatar.clone() }
    }
}

impl From<OpsAuthor> for Author {
    fn from(author: OpsAuthor) -> Self {
        Self { id: author.id, name: author.name, avatar: author.avatar }
    }
}

/// 📌️ One `pins=[...]` entry on a `repin` line: child artifact uri, then its checkpoint.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsPin {
    #[dsl(positional)]
    pub child: String,
    #[dsl(positional)]
    pub checkpoint: String,
}

/// ✏️ One `inputs=[...]` entry on a `supersede` line: the superseded operation, then its
/// replacement — `schema` plus the base64 `OpBinary` `payload`, or neither for a withdrawal.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord)]
pub struct OpsSupersededInput {
    #[dsl(positional)]
    pub target: String,
    pub schema: Option<String>,
    pub payload: Option<String>,
}

impl From<protocol::SupersededInput> for OpsSupersededInput {
    fn from(input: protocol::SupersededInput) -> Self {
        match input.replacement {
            protocol::InputReplacement::Input { schema, payload } => Self { target: input.target.0, schema: Some(schema), payload: Some(semio_framework_io_base64::base64_standard_encode(&payload)) },
            protocol::InputReplacement::Withdrawn => Self { target: input.target.0, schema: None, payload: None },
        }
    }
}

impl TryFrom<OpsSupersededInput> for protocol::SupersededInput {
    type Error = String;

    fn try_from(input: OpsSupersededInput) -> Result<Self, String> {
        let replacement = match (input.schema, input.payload) {
            (Some(schema), Some(payload)) => protocol::InputReplacement::Input { schema, payload: semio_framework_io_base64::base64_standard_decode(&payload).map_err(|error| format!("supersede input {} payload: {error}", input.target))? },
            (None, None) => protocol::InputReplacement::Withdrawn,
            _ => return Err(format!("supersede input {} needs both schema and payload, or neither", input.target)),
        };
        Ok(Self { target: protocol::MutationId(input.target), replacement })
    }
}

/// 🧾️ One `.ops` header/structural line — `doc`/`edit` and one line per history transition
/// (`revert`/`reinstate`/`commit`/`branch`/`checkout`/`repin`/`supersede`) — re-derived directly on the `dsl_schema` grammar engine (`#[derive(DslOps)]` generates
/// `OpText::parse_op`/`print_op` from this declaration; see {@link print_edit_lines}/
/// {@link print_document_text}/{@link parse_document_text}, its only callers). Sigil-free lowercase
/// keywords (bare `doc`, never `@doc` — `@` is reserved for connection points everywhere else in the
/// unified DSL syntax); `id` is always the first positional field on every line; every other field is
/// a plain `key=value` attribute that is simply OMITTED when absent (no more `-` placeholder
/// sentinel); `operations`/`after`/`by`/`pins` are real DSL lists (`by=[ u1 "Ueli Saluz" ]`), not
/// comma-joined, percent-escaped strings.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum)]
pub enum OpsHeaderLine {
    Doc {
        #[dsl(positional)]
        id: String,
        schema: String,
    },
    Edit {
        #[dsl(positional)]
        id: String,
        sequence: i32,
        started: String,
        actor: Option<semio_framework_value::SharedUtf8>,
        finished: Option<String>,
        verb: Option<String>,
        line: Vec<String>,
    },
    /// ⏪️ A `Revert` transition: `clock` is the HLC as `physical.logical.actor`, `after` its
    /// causal dependencies; every transition line carries both so its envelope id re-derives exactly.
    Revert {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        operations: Vec<String>,
    },
    /// ⏩️ A `Reinstate` transition.
    Reinstate {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        operations: Vec<String>,
    },
    /// 🚩️ A `Commit` transition with the change and checkpoint facts it introduces.
    Commit {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        parent: Option<String>,
        change: String,
        operations: Vec<String>,
        description: Option<String>,
        saved: String,
        by: Vec<OpsAuthor>,
        message: Option<String>,
        at: String,
        line: Vec<String>,
    },
    /// 🌿️ A `Branch` transition.
    Branch {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        alternative: String,
        name: String,
        checkpoint: String,
    },
    /// 🎯️ A `Checkout` transition.
    Checkout {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        alternative: Option<String>,
    },
    /// 🧩️ A `Repin` transition.
    Repin {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        checkpoint: String,
        pinned: String,
        pins: Vec<OpsPin>,
    },
    /// ✏️ A `Supersede` transition: every input it replaces, document-wide or within `scope`, and the newest foreign
    /// operation its author had `observed` (its grading frontier at the hub).
    Supersede {
        #[dsl(positional)]
        id: String,
        actor: semio_framework_value::SharedUtf8,
        clock: String,
        after: Vec<String>,
        scope: Option<String>,
        inputs: Vec<OpsSupersededInput>,
        observed: Option<String>,
    },
    /// 🔙️ One edit's complete inverse sequence, encoded with the operation's own text grammar.
    Inverse { edit: String, ops: Vec<String> },
    /// 🪪️ One authoritative metadata record for a forward operation.
    Metadata { edit: String, index: u32, data: String },
    /// 📨️ One durable diagnostic ledger entry, explicitly owned by an edit.
    Message { edit: String, data: String },
    /// ⚔️ One first-class conflict, including its content-addressed identity and lifecycle.
    Conflict { data: String },
}

/// 🌿️ A text record explicitly carries zero trunk/unknown or one authored branch identity.
pub fn ops_line_provenance(mut lines: Vec<String>) -> Result<Option<String>, TextError> {
    if lines.len() > 1 {
        return Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,("history line admits zero or one identity").to_string(),TextSpan::at(1,1)));
    }
    Ok(lines.pop())
}

/// 🎞️ Handcrafted OpText (P6).
impl OpText for OpsHeaderLine {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

/// 🎯️ Handcrafted OpBinary (P6).
impl OpBinary for OpsHeaderLine {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let ordinal = variants.iter().position(|(k, _)| *k == keyword).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 0, detail: format!("keyword {keyword:?} is not a declared variant") })?;
        let spec = (variants[ordinal].1.ordinary)();
        let body = semio_framework_os_pack::record::encode_record_body(&spec, &record, &PackEncodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let mut out = Vec::with_capacity(body.len() + 3);
        out.push(OP_BINARY_FORMAT);
        semio_framework_os_pack::write_varint_u64(&mut out, ordinal as u64);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        const OP_BINARY_FORMAT: u8 = 1;
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8()?;
        if format != OP_BINARY_FORMAT {
            return Err(protocol::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op format {format}") });
        }
        let ordinal = reader.read_varint_u64()?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or(protocol::ProtocolError::Malformed { what: "op variant", offset: 1, detail: format!("ordinal {ordinal} out of range for {} declared variants", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(protocol::ProtocolError::from)?;
        let record_offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| protocol::ProtocolError::Malformed { what: "op record", offset: record_offset, detail: error.to_string() })
    }
}
