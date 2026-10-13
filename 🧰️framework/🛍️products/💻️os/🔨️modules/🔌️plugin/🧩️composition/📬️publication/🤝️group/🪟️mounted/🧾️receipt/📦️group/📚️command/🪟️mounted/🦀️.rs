pub(crate) struct MountedCommandPrune {
    pub(crate) cursor: CommandPrune,
    pub(crate) requested: u64,
    pub(crate) revision: [u8; 32],
    pub(crate) generation: u64,
    pub(crate) edits: usize,
    pub(crate) transitions: usize,
    pub(crate) field: usize,
    pub(crate) index: usize,
    pub(crate) offset: usize,
    pub(crate) keep: bool,
}

impl MountedCommandPrune {
    pub(crate) fn equal_prefix(left: &str, right: &str, offset: &mut usize) -> Option<bool> {
        if left.len() != right.len() { *offset = 0; return Some(false); }
        let end = (*offset + (left.len() - *offset).min(64)).min(left.len());
        if left.as_bytes()[*offset..end] != right.as_bytes()[*offset..end] { *offset = 0; return Some(false); }
        *offset = end;
        (end == left.len()).then_some(true)
    }
}
