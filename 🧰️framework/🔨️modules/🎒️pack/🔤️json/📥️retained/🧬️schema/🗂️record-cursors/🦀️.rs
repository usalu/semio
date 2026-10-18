//! 🗂️ Record cursors: the field and record specs, the record cursor and the nested record cursor. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OwnedSchemaFieldSpec {
    pub id: u16,
    pub key: &'static str,
    pub required: bool,
}

#[derive(Clone, Copy)]
pub struct OwnedSchemaRecordSpec {
    pub fields: &'static [OwnedSchemaFieldSpec],
}

impl OwnedSchemaRecordSpec {
    /// 📋️ Admits a fixed schema whose field IDs and keys form an exact bijection.
    pub fn validate(self) -> Result<Self, &'static str> {
        if self.fields.is_empty() || self.fields.len() > 64 {
            return Err("schema-json.field-capacity");
        }
        for (index, field) in self.fields.iter().enumerate() {
            if field.id == 0 || field.key.is_empty() || field.key.len() > 253 || !field.key.is_ascii() {
                return Err("schema-json.invalid-field-spec");
            }
            if self.fields[..index].iter().any(|candidate| candidate.id == field.id || candidate.key == field.key) {
                return Err("schema-json.duplicate-field-spec");
            }
        }
        Ok(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaRecordStep {
    Pending,
    FieldToken { field_id: u16, token: OwnedSchemaToken, terminal: bool },
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

#[derive(Clone, Copy)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaRecordState {
    Root,
    Key,
    KeyAfterComma,
    Colon { field_index: u8 },
    Value { field_index: u8 },
    Nested { field_index: u8, depth: u16 },
    Separator,
    Trailing,
    Complete,
    Cancelled,
    Fault(OwnedSchemaDecodeDiagnostic),
}

pub struct OwnedSchemaRecordCursor {
    pub(super) spec: OwnedSchemaRecordSpec,
    pub(crate) tokens: OwnedSchemaTokenCursor,
    pub(super) state: OwnedSchemaRecordState,
    seen: u64,
    delimiters: [OwnedSchemaTokenKind; 256],
}

impl OwnedSchemaRecordCursor {
    /// 🧬️ Couples fixed schema authority to the fixed-page token cursor.
    pub fn try_new(spec: OwnedSchemaRecordSpec, tokens: OwnedSchemaTokenCursor) -> Result<Self, (OwnedSchemaTokenCursor, &'static str)> {
        let spec = match spec.validate() {
            Ok(spec) => spec,
            Err(error) => return Err((tokens, error)),
        };
        Ok(Self { spec, tokens, state: OwnedSchemaRecordState::Root, seen: 0, delimiters: [OwnedSchemaTokenKind::ObjectStart; 256] })
    }

    pub(super) fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaRecordStep {
        let path = field_index.and_then(|index| self.spec.fields.get(index as usize)).and_then(|field| OwnedSchemaPath::field(field.key)).unwrap_or(OwnedSchemaPath::ROOT);
        let diagnostic = OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: self.tokens.line, column: self.tokens.column, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        self.state = OwnedSchemaRecordState::Fault(diagnostic);
        OwnedSchemaRecordStep::Fault(diagnostic)
    }

    fn field_index(&self, token: OwnedSchemaToken) -> Option<usize> {
        self.spec.fields.iter().position(|field| self.tokens.string_token_equals(token, field.key))
    }

    fn required_complete(&self) -> bool {
        self.spec.fields.iter().enumerate().all(|(index, field)| !field.required || self.seen & (1 << index) != 0)
    }

    /// ⏭️ Advances one schema token, preserving exact field identity and duplicate state.
    pub fn step(&mut self, cx: &mut semio_framework_job::StepContext<'_>) -> OwnedSchemaRecordStep {
        if matches!(self.state, OwnedSchemaRecordState::Complete) {
            return OwnedSchemaRecordStep::Complete;
        }
        if matches!(self.state, OwnedSchemaRecordState::Cancelled) || cx.is_cancelled() {
            self.state = OwnedSchemaRecordState::Cancelled;
            return OwnedSchemaRecordStep::Cancelled;
        }
        if let OwnedSchemaRecordState::Fault(diagnostic) = self.state {
            return OwnedSchemaRecordStep::Fault(diagnostic);
        }
        cx.set_stage("schema-json-field");
        let token = match self.tokens.step(cx) {
            OwnedSchemaTokenStep::Pending => return OwnedSchemaRecordStep::Pending,
            OwnedSchemaTokenStep::Token(token) => token,
            OwnedSchemaTokenStep::Complete if matches!(self.state, OwnedSchemaRecordState::Trailing) => {
                self.state = OwnedSchemaRecordState::Complete;
                return OwnedSchemaRecordStep::Complete;
            }
            OwnedSchemaTokenStep::Complete => {
                let token = OwnedSchemaToken { kind: OwnedSchemaTokenKind::Null, start: self.tokens.offset as u64, end: self.tokens.offset as u64 };
                return self.fault("schema-json.truncated-record", token, None);
            }
            OwnedSchemaTokenStep::Cancelled => {
                self.state = OwnedSchemaRecordState::Cancelled;
                return OwnedSchemaRecordStep::Cancelled;
            }
            OwnedSchemaTokenStep::Fault(diagnostic) => {
                self.state = OwnedSchemaRecordState::Fault(diagnostic);
                return OwnedSchemaRecordStep::Fault(diagnostic);
            }
        };
        match self.state {
            OwnedSchemaRecordState::Root if token.kind == OwnedSchemaTokenKind::ObjectStart => {
                self.state = OwnedSchemaRecordState::Key;
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Root => self.fault("schema-json.expected-object", token, None),
            OwnedSchemaRecordState::Key if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if !self.required_complete() {
                    self.fault("schema-json.missing-required-field", token, None)
                } else {
                    self.state = OwnedSchemaRecordState::Trailing;
                    OwnedSchemaRecordStep::Pending
                }
            }
            OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::ObjectEnd => self.fault("schema-json.trailing-comma", token, None),
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::String => {
                let Some(index) = self.field_index(token) else {
                    return self.fault("schema-json.unknown-field", token, None);
                };
                if self.seen & (1 << index) != 0 {
                    return self.fault("schema-json.duplicate-field", token, Some(index as u8));
                }
                self.seen |= 1 << index;
                self.state = OwnedSchemaRecordState::Colon { field_index: index as u8 };
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma => self.fault("schema-json.expected-field", token, None),
            OwnedSchemaRecordState::Colon { field_index } if token.kind == OwnedSchemaTokenKind::Colon => {
                self.state = OwnedSchemaRecordState::Value { field_index };
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Colon { field_index } => self.fault("schema-json.expected-colon", token, Some(field_index)),
            OwnedSchemaRecordState::Value { field_index } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    self.delimiters[0] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth: 1 };
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd | OwnedSchemaTokenKind::Colon | OwnedSchemaTokenKind::Comma => self.fault("schema-json.expected-value", token, Some(field_index)),
                _ => {
                    self.state = OwnedSchemaRecordState::Separator;
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                }
            },
            OwnedSchemaRecordState::Nested { field_index, depth } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    let Some(depth) = depth.checked_add(1) else {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    };
                    if depth as usize > self.delimiters.len() {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    }
                    self.delimiters[depth as usize - 1] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth };
                    OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                    let opening = self.delimiters[depth as usize - 1];
                    let matching = matches!((opening, token.kind), (OwnedSchemaTokenKind::ObjectStart, OwnedSchemaTokenKind::ObjectEnd) | (OwnedSchemaTokenKind::ArrayStart, OwnedSchemaTokenKind::ArrayEnd));
                    if !matching {
                        self.fault("schema-json.mismatched-delimiter", token, Some(field_index))
                    } else if depth == 1 {
                        self.state = OwnedSchemaRecordState::Separator;
                        OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                    } else {
                        self.state = OwnedSchemaRecordState::Nested { field_index, depth: depth - 1 };
                        OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                    }
                }
                _ => OwnedSchemaRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false },
            },
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::Comma => {
                self.state = OwnedSchemaRecordState::KeyAfterComma;
                OwnedSchemaRecordStep::Pending
            }
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if !self.required_complete() {
                    self.fault("schema-json.missing-required-field", token, None)
                } else {
                    self.state = OwnedSchemaRecordState::Trailing;
                    OwnedSchemaRecordStep::Pending
                }
            }
            OwnedSchemaRecordState::Separator => self.fault("schema-json.expected-separator", token, None),
            OwnedSchemaRecordState::Trailing => self.fault("schema-json.trailing-token", token, None),
            OwnedSchemaRecordState::Complete => OwnedSchemaRecordStep::Complete,
            OwnedSchemaRecordState::Cancelled => OwnedSchemaRecordStep::Cancelled,
            OwnedSchemaRecordState::Fault(diagnostic) => OwnedSchemaRecordStep::Fault(diagnostic),
        }
    }

    pub fn copy_token_bytes(&self, token: OwnedSchemaToken, relative_offset: usize, output: &mut [u8]) -> usize {
        self.tokens.copy_token_bytes(token, relative_offset, output)
    }

    pub fn next_close_copy_byte_demand(&self) -> usize {
        self.tokens.next_close_copy_byte_demand()
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.tokens.close_step(grant)
    }

    pub fn next_close_capacity_byte_demand(&self, body:usize)->Result<usize,ValueError>{self.tokens.next_close_capacity_byte_demand(body)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{self.tokens.next_close_release_byte_demand()}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{self.tokens.next_close_depth_demand()}
    pub fn terminal_is_empty(&self) -> bool {
        self.tokens.terminal_is_empty()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[expect(clippy::large_enum_variant, reason = "The decoder carries its fixed Copy diagnostic path inline so faults remain allocation-free under exhausted grants.")]
pub enum OwnedSchemaNestedRecordStep {
    Pending,
    FieldToken { field_id: u16, token: OwnedSchemaToken, terminal: bool },
    Complete,
    Fault(OwnedSchemaDecodeDiagnostic),
}

/// 🪆 Schema-first nested-object matcher fed by an outer retained record cursor. It
/// owns no bytes and advances exactly one already-budgeted token at a time.
pub struct OwnedSchemaNestedRecordCursor {
    pub(super) spec: OwnedSchemaRecordSpec,
    pub(super) state: OwnedSchemaRecordState,
    seen: u64,
    delimiters: [OwnedSchemaTokenKind; 256],
}

impl OwnedSchemaNestedRecordCursor {
    pub fn try_new(spec: OwnedSchemaRecordSpec) -> Result<Self, &'static str> {
        Ok(Self { spec: spec.validate()?, state: OwnedSchemaRecordState::Root, seen: 0, delimiters: [OwnedSchemaTokenKind::ObjectStart; 256] })
    }

    pub(super) fn fault(&mut self, code: &'static str, token: OwnedSchemaToken, field_index: Option<u8>) -> OwnedSchemaNestedRecordStep {
        let path = field_index.and_then(|index| self.spec.fields.get(index as usize)).and_then(|field| OwnedSchemaPath::field(field.key)).unwrap_or(OwnedSchemaPath::ROOT);
        let diagnostic = OwnedSchemaDecodeDiagnostic { code, offset: token.start, line: 0, column: 0, path , refusal_kind: semio_framework_value::ValueRefusalKind::InvariantViolated, retained_progress: RetainedCloneProgress::default() };
        self.state = OwnedSchemaRecordState::Fault(diagnostic);
        OwnedSchemaNestedRecordStep::Fault(diagnostic)
    }

    fn field_index(&self, source: &OwnedSchemaRecordCursor, token: OwnedSchemaToken) -> Option<usize> {
        self.spec.fields.iter().position(|field| source.tokens.string_token_equals(token, field.key))
    }

    fn required_complete(&self) -> bool {
        self.spec.fields.iter().enumerate().all(|(index, field)| !field.required || self.seen & (1 << index) != 0)
    }

    pub fn accept(&mut self, token: OwnedSchemaToken, source: &OwnedSchemaRecordCursor) -> OwnedSchemaNestedRecordStep {
        match self.state {
            OwnedSchemaRecordState::Root if token.kind == OwnedSchemaTokenKind::ObjectStart => {
                self.state = OwnedSchemaRecordState::Key;
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Root => self.fault("schema-json.expected-object", token, None),
            OwnedSchemaRecordState::Key if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if self.required_complete() {
                    self.state = OwnedSchemaRecordState::Complete;
                    OwnedSchemaNestedRecordStep::Complete
                } else {
                    self.fault("schema-json.missing-required-field", token, None)
                }
            }
            OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::ObjectEnd => self.fault("schema-json.trailing-comma", token, None),
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma if token.kind == OwnedSchemaTokenKind::String => {
                let Some(index) = self.field_index(source, token) else { return self.fault("schema-json.unknown-field", token, None) };
                if self.seen & (1 << index) != 0 {
                    return self.fault("schema-json.duplicate-field", token, Some(index as u8));
                }
                self.seen |= 1 << index;
                self.state = OwnedSchemaRecordState::Colon { field_index: index as u8 };
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Key | OwnedSchemaRecordState::KeyAfterComma => self.fault("schema-json.expected-field", token, None),
            OwnedSchemaRecordState::Colon { field_index } if token.kind == OwnedSchemaTokenKind::Colon => {
                self.state = OwnedSchemaRecordState::Value { field_index };
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Colon { field_index } => self.fault("schema-json.expected-colon", token, Some(field_index)),
            OwnedSchemaRecordState::Value { field_index } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    self.delimiters[0] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth: 1 };
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd | OwnedSchemaTokenKind::Colon | OwnedSchemaTokenKind::Comma => self.fault("schema-json.expected-value", token, Some(field_index)),
                _ => {
                    self.state = OwnedSchemaRecordState::Separator;
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                }
            },
            OwnedSchemaRecordState::Nested { field_index, depth } => match token.kind {
                OwnedSchemaTokenKind::ObjectStart | OwnedSchemaTokenKind::ArrayStart => {
                    let Some(depth) = depth.checked_add(1) else { return self.fault("schema-json.maximum-depth", token, Some(field_index)) };
                    if depth as usize > self.delimiters.len() {
                        return self.fault("schema-json.maximum-depth", token, Some(field_index));
                    }
                    self.delimiters[depth as usize - 1] = token.kind;
                    self.state = OwnedSchemaRecordState::Nested { field_index, depth };
                    OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                }
                OwnedSchemaTokenKind::ObjectEnd | OwnedSchemaTokenKind::ArrayEnd => {
                    let opening = self.delimiters[depth as usize - 1];
                    if !matches!((opening, token.kind), (OwnedSchemaTokenKind::ObjectStart, OwnedSchemaTokenKind::ObjectEnd) | (OwnedSchemaTokenKind::ArrayStart, OwnedSchemaTokenKind::ArrayEnd)) {
                        return self.fault("schema-json.mismatched-delimiter", token, Some(field_index));
                    }
                    if depth == 1 {
                        self.state = OwnedSchemaRecordState::Separator;
                        OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: true }
                    } else {
                        self.state = OwnedSchemaRecordState::Nested { field_index, depth: depth - 1 };
                        OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false }
                    }
                }
                _ => OwnedSchemaNestedRecordStep::FieldToken { field_id: self.spec.fields[field_index as usize].id, token, terminal: false },
            },
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::Comma => {
                self.state = OwnedSchemaRecordState::KeyAfterComma;
                OwnedSchemaNestedRecordStep::Pending
            }
            OwnedSchemaRecordState::Separator if token.kind == OwnedSchemaTokenKind::ObjectEnd => {
                if self.required_complete() {
                    self.state = OwnedSchemaRecordState::Complete;
                    OwnedSchemaNestedRecordStep::Complete
                } else {
                    self.fault("schema-json.missing-required-field", token, None)
                }
            }
            OwnedSchemaRecordState::Separator => self.fault("schema-json.expected-separator", token, None),
            OwnedSchemaRecordState::Complete => self.fault("schema-json.trailing-token", token, None),
            OwnedSchemaRecordState::Fault(diagnostic) => OwnedSchemaNestedRecordStep::Fault(diagnostic),
            OwnedSchemaRecordState::Trailing | OwnedSchemaRecordState::Cancelled => self.fault("schema-json.invalid-nested-state", token, None),
        }
    }

    pub fn terminal_is_complete(&self) -> bool {
        matches!(self.state, OwnedSchemaRecordState::Complete)
    }
}
