//! 🆔️ Identifiers of the sampling vocabulary: the token id, the sequence id and the step index. See [`super`].

// #region 🔖️Ids
/// 🧩️ Index of one vocabulary entry. `u32` keeps candidate/mask arithmetic cheap even for
/// million-token sharded vocabularies while staying far below any real model's vocab size.
/// `#[repr(transparent)]` lets [`cast_u32_slice_to_token_ids`] hand out a typed view over a raw
/// index buffer without copying.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TokenId(pub u32);

impl TokenId {
    /// 🧩️ Wraps a raw vocabulary index.
    pub const fn new(id: u32) -> Self {
        Self(id)
    }

    /// 🧩️ Raw vocabulary index.
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl core::fmt::Display for TokenId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Identifies one generation request/sequence across batch reorders — never a slot index, so
/// continuous batching can shuffle rows without breaking RNG-stream or state addressing.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SequenceId(pub u64);

impl SequenceId {
    /// 🧩️ Wraps a raw sequence identifier.
    pub const fn new(id: u64) -> Self {
        Self(id)
    }

    /// 🧩️ Raw sequence identifier.
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl core::fmt::Display for SequenceId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// 🧩️ Zero-based count of tokens generated so far for one sequence (excludes the prompt).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StepIndex(pub u32);

impl StepIndex {
    /// 🧩️ Wraps a raw step count.
    pub const fn new(step: u32) -> Self {
        Self(step)
    }

    /// 🧩️ Raw step count.
    pub const fn get(self) -> u32 {
        self.0
    }

    /// 🧩️ Next step, or `None` on overflow (caller-observable rather than a silent wraparound).
    pub const fn checked_next(self) -> Option<Self> {
        match self.0.checked_add(1) {
            Some(next) => Some(Self(next)),
            None => None,
        }
    }
}

impl core::fmt::Display for StepIndex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}
// #endregion 🔖️Ids
