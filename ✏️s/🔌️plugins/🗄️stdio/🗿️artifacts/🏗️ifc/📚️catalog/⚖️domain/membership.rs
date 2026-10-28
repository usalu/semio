//! 🪜️ Borrowed SELECT membership and entity ancestry advance one declaration edge at a time.
use super::{IfcCatalog, LIMIT};
#[derive(Clone, Copy)]
struct Frame {
    expected: &'static str,
    ancestor: Option<&'static str>,
    member: usize,
    ancestors: usize,
    phase: u8,
}
pub(crate) struct TypeMembership {
    frames: [Option<Frame>; LIMIT + 1],
    depth: usize,
    actual: &'static str,
    done: Option<bool>,
}
impl Default for TypeMembership {
    fn default() -> Self {
        Self { frames: [None; LIMIT + 1], depth: 0, actual: "", done: None }
    }
}
impl TypeMembership {
    pub(crate) fn start(&mut self, expected: &'static str, actual: &'static str) {
        self.depth = 1;
        self.actual = actual;
        self.done = None;
        self.frames[0] = Some(Frame { expected, ancestor: Some(actual), member: 0, ancestors: 0, phase: 0 });
    }
    fn pop(&mut self) {
        self.depth -= 1;
        self.frames[self.depth] = None;
        if self.depth == 0 {
            self.done = Some(false);
        }
    }
    pub(crate) fn advance(&mut self, catalog: &'static IfcCatalog) -> Result<Option<bool>, String> {
        if self.done.is_some() {
            return Ok(self.done);
        }
        let top = self.depth.checked_sub(1).ok_or("ifc-domain-membership")?;
        let mut frame = self.frames[top].ok_or("ifc-domain-membership")?;
        match frame.phase {
            0 => {
                if frame.expected == self.actual && (catalog.entities.contains_key(self.actual) || catalog.types.contains_key(self.actual)) {
                    self.done = Some(true);
                } else if catalog.entities.contains_key(frame.expected) {
                    frame.phase = 2;
                    self.frames[top] = Some(frame);
                } else if catalog.types.get(frame.expected).is_some_and(|value| value.kind == "select") {
                    frame.phase = 1;
                    self.frames[top] = Some(frame);
                } else {
                    self.pop();
                }
            }
            1 => {
                let declaration = catalog.types.get(frame.expected).ok_or("ifc-domain-membership")?;
                if let Some(member) = declaration.members.get(frame.member) {
                    frame.member += 1;
                    self.frames[top] = Some(frame);
                    if self.depth == self.frames.len() {
                        return Ok(self.done);
                    }
                    self.frames[self.depth] = Some(Frame { expected: member, ancestor: Some(self.actual), member: 0, ancestors: 0, phase: 0 });
                    self.depth += 1;
                } else {
                    self.pop();
                }
            }
            _ => {
                if let Some(actual) = frame.ancestor {
                    if actual == frame.expected {
                        self.done = Some(true);
                    } else if frame.ancestors == catalog.entities.len() {
                        self.pop();
                    } else {
                        frame.ancestors += 1;
                        frame.ancestor = catalog.entities.get(actual).and_then(|value| value.supertype.as_deref());
                        self.frames[top] = Some(frame);
                    }
                } else {
                    self.pop();
                }
            }
        }
        Ok(self.done)
    }
}
