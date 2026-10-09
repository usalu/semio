//! 🧾️ Retained physical JSON publication over borrowed semantic selection facts.
use crate::infinite::board::schema::dag_input::{DagSelectionDomain,DagSelectionSource};

pub const DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES: usize = 65_536;

/// ⛽️ One cancellable physical-unit grant for a retained DAG cursor.
#[derive(Clone, Copy, Debug)]
pub struct DagSelectionTextGrant {
    pub fuel: u8,
    pub now_milliseconds: u64,
    pub deadline_milliseconds: u64,
    pub cancelled: bool,
    pub interrupted: bool,
}

/// 🚧️ Fail-closed retained DAG cursor faults.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagSelectionTextFault {
    Cancelled,
    Interrupted,
    Deadline,
    NoFuel,
    Limit,
    Sealed,
}

/// 📬️ A census, byte, progress, or terminal result from one DAG cursor grant.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagSelectionTextStep {
    Progress { completed: usize, total: usize },
    Census { bytes: usize },
    Byte(u8),
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum DagSelectionJsonPhase {
    CensusNode,
    CensusText,
    Open,
    Seek,
    Separator,
    QuoteOpen,
    Text,
    Escape,
    QuoteClose,
    Close,
    Complete,
}

/// 🎯️ Exact-preflight JSON array encoder that inspects or emits at most one unit per grant.
pub struct DagSelectionJsonCursor {
    kind: DagSelectionDomain,
    phase: DagSelectionJsonPhase,
    node_cursor: usize,
    text_cursor: usize,
    selected_count: usize,
    emitted_count: usize,
    census_bytes: usize,
    output_cursor: usize,
    escape: [u8; 6],
    escape_length: u8,
    escape_cursor: u8,
}

impl Default for DagSelectionJsonCursor {
    fn default() -> Self {
        Self {
            kind: DagSelectionDomain::Nodes,
            phase: DagSelectionJsonPhase::CensusNode,
            node_cursor: 0,
            text_cursor: 0,
            selected_count: 0,
            emitted_count: 0,
            census_bytes: 2,
            output_cursor: 0,
            escape: [0; 6],
            escape_length: 0,
            escape_cursor: 0,
        }
    }
}

impl DagSelectionJsonCursor {
    pub fn edges() -> Self {
        Self { kind: DagSelectionDomain::Edges, ..Self::default() }
    }

    fn guard(grant: DagSelectionTextGrant) -> Result<(), DagSelectionTextFault> {
        if grant.cancelled {
            Err(DagSelectionTextFault::Cancelled)
        } else if grant.interrupted {
            Err(DagSelectionTextFault::Interrupted)
        } else if grant.now_milliseconds >= grant.deadline_milliseconds {
            Err(DagSelectionTextFault::Deadline)
        } else if grant.fuel == 0 {
            Err(DagSelectionTextFault::NoFuel)
        } else {
            Ok(())
        }
    }

    fn count(&self, source: &impl DagSelectionSource) -> usize { source.selection_candidate_count(self.kind) }

    fn item<'a>(&self, source: &'a impl DagSelectionSource, index: usize) -> Option<&'a str> { source.selection_candidate_id(self.kind,index) }

    fn escape(byte: u8) -> ([u8; 6], u8) {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        match byte {
            b'"' => ([b'\\', b'"', 0, 0, 0, 0], 2),
            b'\\' => ([b'\\', b'\\', 0, 0, 0, 0], 2),
            b'\x08' => ([b'\\', b'b', 0, 0, 0, 0], 2),
            b'\x0c' => ([b'\\', b'f', 0, 0, 0, 0], 2),
            b'\n' => ([b'\\', b'n', 0, 0, 0, 0], 2),
            b'\r' => ([b'\\', b'r', 0, 0, 0, 0], 2),
            b'\t' => ([b'\\', b't', 0, 0, 0, 0], 2),
            0x00..=0x1f => ([b'\\', b'u', b'0', b'0', HEX[usize::from(byte >> 4)], HEX[usize::from(byte & 0x0f)]], 6),
            _ => ([byte, 0, 0, 0, 0, 0], 1),
        }
    }

    pub fn step(&mut self, source: &impl DagSelectionSource, grant: DagSelectionTextGrant) -> Result<DagSelectionTextStep, DagSelectionTextFault> {
        Self::guard(grant)?;
        let count = self.count(source);
        match self.phase {
            DagSelectionJsonPhase::CensusNode => {
                if self.node_cursor == count {
                    if self.census_bytes > DAG_SELECTION_TEXT_MAX_OUTPUT_BYTES {
                        return Err(DagSelectionTextFault::Limit);
                    }
                    self.node_cursor = 0;
                    self.text_cursor = 0;
                    self.phase = DagSelectionJsonPhase::Open;
                    return Ok(DagSelectionTextStep::Census { bytes: self.census_bytes });
                }
                if self.item(source, self.node_cursor).is_some() {
                    self.census_bytes = self.census_bytes.checked_add(2 + usize::from(self.selected_count != 0)).ok_or(DagSelectionTextFault::Limit)?;
                    self.phase = DagSelectionJsonPhase::CensusText;
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::CensusText => {
                let text = self.item(source, self.node_cursor).ok_or(DagSelectionTextFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.selected_count += 1;
                    self.node_cursor += 1;
                    self.text_cursor = 0;
                    self.phase = DagSelectionJsonPhase::CensusNode;
                } else {
                    self.census_bytes = self.census_bytes.checked_add(usize::from(Self::escape(text[self.text_cursor]).1)).ok_or(DagSelectionTextFault::Limit)?;
                    self.text_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::Open => {
                self.phase = DagSelectionJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'['))
            }
            DagSelectionJsonPhase::Seek => {
                if self.node_cursor == count {
                    self.phase = DagSelectionJsonPhase::Close;
                } else if self.item(source, self.node_cursor).is_some() {
                    self.phase = if self.emitted_count == 0 { DagSelectionJsonPhase::QuoteOpen } else { DagSelectionJsonPhase::Separator };
                } else {
                    self.node_cursor += 1;
                }
                Ok(DagSelectionTextStep::Progress { completed: self.node_cursor, total: count })
            }
            DagSelectionJsonPhase::Separator => {
                self.phase = DagSelectionJsonPhase::QuoteOpen;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b','))
            }
            DagSelectionJsonPhase::QuoteOpen => {
                self.phase = DagSelectionJsonPhase::Text;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'"'))
            }
            DagSelectionJsonPhase::Text => {
                let text = self.item(source, self.node_cursor).ok_or(DagSelectionTextFault::Limit)?.as_bytes();
                if self.text_cursor == text.len() {
                    self.phase = DagSelectionJsonPhase::QuoteClose;
                    return Ok(DagSelectionTextStep::Progress { completed: self.output_cursor, total: self.census_bytes });
                }
                let (escape, length) = Self::escape(text[self.text_cursor]);
                if length == 1 {
                    self.text_cursor += 1;
                    self.output_cursor += 1;
                    Ok(DagSelectionTextStep::Byte(escape[0]))
                } else {
                    self.escape = escape;
                    self.escape_length = length;
                    self.escape_cursor = 0;
                    self.phase = DagSelectionJsonPhase::Escape;
                    Ok(DagSelectionTextStep::Progress { completed: self.output_cursor, total: self.census_bytes })
                }
            }
            DagSelectionJsonPhase::Escape => {
                let byte = self.escape[usize::from(self.escape_cursor)];
                self.escape_cursor += 1;
                self.output_cursor += 1;
                if self.escape_cursor == self.escape_length {
                    self.text_cursor += 1;
                    self.phase = DagSelectionJsonPhase::Text;
                }
                Ok(DagSelectionTextStep::Byte(byte))
            }
            DagSelectionJsonPhase::QuoteClose => {
                self.emitted_count += 1;
                self.node_cursor += 1;
                self.text_cursor = 0;
                self.phase = DagSelectionJsonPhase::Seek;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b'"'))
            }
            DagSelectionJsonPhase::Close => {
                self.phase = DagSelectionJsonPhase::Complete;
                self.output_cursor += 1;
                Ok(DagSelectionTextStep::Byte(b']'))
            }
            DagSelectionJsonPhase::Complete if self.output_cursor == self.census_bytes => Ok(DagSelectionTextStep::Complete),
            DagSelectionJsonPhase::Complete => Err(DagSelectionTextFault::Limit),
        }
    }
}

