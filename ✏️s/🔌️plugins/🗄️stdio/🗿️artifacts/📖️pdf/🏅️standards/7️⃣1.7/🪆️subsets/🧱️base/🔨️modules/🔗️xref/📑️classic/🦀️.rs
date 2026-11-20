//! 📑️ Byte-budgeted classic PDF cross-reference table reader.

#[derive(Clone, Copy)]
enum Field {
    Keyword,
    FirstObject,
    Count,
    Offset,
    Generation,
    Flag,
    Trailer,
}

pub enum ClassicXrefEvent {
    More,
    Entry { number: u32, offset: usize, generation: u16, live: bool },
    Trailer,
}

pub struct ClassicXrefReceipt {
    pub event: Result<ClassicXrefEvent, &'static str>,
    pub work: usize,
}

pub struct ClassicXrefCursor {
    position: usize,
    token_start: Option<usize>,
    comment: bool,
    field: Field,
    number: u32,
    remaining: u32,
    offset: usize,
    generation: u16,
}

impl ClassicXrefCursor {
    pub fn new(position: usize) -> Self {
        Self { position, token_start: None, comment: false, field: Field::Keyword, number: 0, remaining: 0, offset: 0, generation: 0 }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    /// 🧾️ Read at most the granted bytes of the same PDF and yield each finished row.
    pub fn advance(&mut self, source: &[u8], budget: usize) -> ClassicXrefReceipt {
        let mut work = 0;
        let event = loop {
            if matches!(self.field, Field::Trailer) {
                break Ok(ClassicXrefEvent::Trailer);
            }
            if work == budget { break Ok(ClassicXrefEvent::More); }
            let Some(&byte) = source.get(self.position) else {
                break Err("xref table ended before its trailer dictionary");
            };
            work += 1;
            let whitespace = matches!(byte, 0 | 9 | 10 | 12 | 13 | 32);
            let delimiter = matches!(byte, b'(' | b')' | b'<' | b'>' | b'[' | b']' | b'{' | b'}' | b'/' | b'%');
            if let Some(start) = self.token_start {
                if whitespace || delimiter {
                    self.token_start = None;
                    match self.finish(&source[start..self.position]) {
                        Ok(None) => {}
                        Ok(Some(event)) => break Ok(event),
                        Err(reason) => break Err(reason),
                    }
                } else {
                    if self.position - start >= 20 { break Err("xref token exceeds the numeric field width"); }
                    self.position += 1;
                    continue;
                }
            }
            if self.comment {
                self.comment = !matches!(byte, b'\r' | b'\n');
            } else if byte == b'%' {
                self.comment = true;
            } else if !whitespace {
                if delimiter { break Err("unexpected delimiter in xref table"); }
                self.token_start = Some(self.position);
            }
            self.position += 1;
        };
        ClassicXrefReceipt { event, work }
    }

    fn finish(&mut self, bytes: &[u8]) -> Result<Option<ClassicXrefEvent>, &'static str> {
        let token = std::str::from_utf8(bytes).map_err(|_| "invalid xref token")?;
        match self.field {
            Field::Keyword if token == "xref" => self.field = Field::FirstObject,
            Field::Keyword => return Err("expected xref keyword"),
            Field::FirstObject | Field::Offset if token == "trailer" => {
                self.field = Field::Trailer;
                return Ok(Some(ClassicXrefEvent::Trailer));
            }
            Field::FirstObject => {
                self.number = token.parse().map_err(|_| "invalid xref subsection start")?;
                self.field = Field::Count;
            }
            Field::Count => {
                self.remaining = token.parse().map_err(|_| "invalid xref subsection count")?;
                self.field = if self.remaining == 0 { Field::FirstObject } else { Field::Offset };
            }
            Field::Offset => {
                self.offset = token.parse().map_err(|_| "invalid xref offset")?;
                self.field = Field::Generation;
            }
            Field::Generation => {
                self.generation = token.parse().map_err(|_| "invalid xref generation")?;
                self.field = Field::Flag;
            }
            Field::Flag => {
                let live = match token { "n" => true, "f" => false, _ => return Err("invalid xref entry flag") };
                let event = ClassicXrefEvent::Entry { number: self.number, offset: self.offset, generation: self.generation, live };
                self.remaining -= 1;
                if self.remaining == 0 {
                    self.field = Field::FirstObject;
                } else {
                    self.number = self.number.checked_add(1).ok_or("xref object number overflow")?;
                    self.field = Field::Offset;
                }
                return Ok(Some(event));
            }
            Field::Trailer => return Ok(Some(ClassicXrefEvent::Trailer)),
        }
        Ok(None)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
