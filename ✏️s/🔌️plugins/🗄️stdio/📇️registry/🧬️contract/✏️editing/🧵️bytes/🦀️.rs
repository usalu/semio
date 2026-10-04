//! 🧵️ Format-neutral checked byte spans for retained exact-file edits.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ByteSpan {
    pub index: usize,
    pub length: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByteSpanPatch {
    pub index: usize,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ByteSpanPlanError {
    Empty,
    InvalidBudget,
    SourceTooLarge,
    RangeOverflow,
    OutOfBounds,
    Overlap,
    TooManyPatches,
    InvalidOrdinal,
    SourceChanged,
}

impl ByteSpanPlanError {
    pub const fn code(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::InvalidBudget => "invalid-budget",
            Self::SourceTooLarge => "source-too-large",
            Self::RangeOverflow => "range-overflow",
            Self::OutOfBounds => "out-of-bounds",
            Self::Overlap => "overlap",
            Self::TooManyPatches => "too-many-patches",
            Self::InvalidOrdinal => "invalid-ordinal",
            Self::SourceChanged => "source-changed",
        }
    }
}

impl std::fmt::Display for ByteSpanPlanError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for ByteSpanPlanError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ByteSpanPatchPlan {
    source_length: usize,
    spans: Vec<ByteSpan>,
}

impl ByteSpanPatchPlan {
    pub fn new(source_length: usize, mut spans: Vec<ByteSpan>, maximum_source_bytes: usize, maximum_patch_bytes: usize, maximum_patches: usize) -> Result<Self, ByteSpanPlanError> {
        if spans.is_empty() {
            return Err(ByteSpanPlanError::Empty);
        }
        if maximum_patch_bytes == 0 || maximum_patches == 0 {
            return Err(ByteSpanPlanError::InvalidBudget);
        }
        if source_length > maximum_source_bytes {
            return Err(ByteSpanPlanError::SourceTooLarge);
        }
        if spans.len() > maximum_patches {
            return Err(ByteSpanPlanError::TooManyPatches);
        }
        spans.sort_unstable_by_key(|span| span.index);
        let mut previous_end = 0;
        for (ordinal, span) in spans.iter().enumerate() {
            if span.length == 0 || span.length > maximum_patch_bytes {
                return Err(ByteSpanPlanError::InvalidBudget);
            }
            let end = span.index.checked_add(span.length).ok_or(ByteSpanPlanError::RangeOverflow)?;
            if end > source_length {
                return Err(ByteSpanPlanError::OutOfBounds);
            }
            if ordinal != 0 && span.index < previous_end {
                return Err(ByteSpanPlanError::Overlap);
            }
            previous_end = end;
        }
        Ok(Self { source_length, spans })
    }

    pub fn patch_count(&self) -> usize {
        self.spans.len()
    }

    pub fn span(&self, ordinal: usize) -> Result<ByteSpan, ByteSpanPlanError> {
        self.spans.get(ordinal).copied().ok_or(ByteSpanPlanError::InvalidOrdinal)
    }

    pub fn patch_with(&self, source: &[u8], ordinal: usize, edit: impl FnOnce(&mut [u8])) -> Result<Option<ByteSpanPatch>, ByteSpanPlanError> {
        if source.len() != self.source_length {
            return Err(ByteSpanPlanError::SourceChanged);
        }
        let span = self.span(ordinal)?;
        let original = &source[span.index..span.index + span.length];
        let mut bytes = original.to_vec();
        edit(&mut bytes);
        Ok((bytes != original).then_some(ByteSpanPatch { index: span.index, bytes }))
    }
}
