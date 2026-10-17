//! 📖️ Vocabulary and text adapters: the token vocabulary, the token text adapter trait, the slice adapter and the closed adapter enum. See [`super`].

use super::*;

// #region 🔖️Vocabulary
/// 📖️ Static facts about the token space a [`SamplingConfig`] samples over.
#[derive(Clone, PartialEq, Debug)]
pub struct Vocabulary {
    pub size: usize,
    pub eos: Vec<TokenId>,
    pub bos: Option<TokenId>,
    pub pad: Option<TokenId>,
    pub unk: Option<TokenId>,
    pub special: TokenBitset,
}

impl Vocabulary {
    /// 📖️ A vocabulary of `size` tokens with no special tokens configured.
    pub async fn new(size: usize) -> Self {
        Self { size, eos: Vec::new(), bos: None, pad: None, unk: None, special: TokenBitset::new_empty(size).await }
    }

    pub async fn with_eos(mut self, eos: Vec<TokenId>) -> Self {
        self.eos = eos;
        self
    }

    pub async fn with_bos(mut self, bos: TokenId) -> Self {
        self.bos = Some(bos);
        self
    }

    pub async fn with_pad(mut self, pad: TokenId) -> Self {
        self.pad = Some(pad);
        self
    }

    pub async fn with_unk(mut self, unk: TokenId) -> Self {
        self.unk = Some(unk);
        self
    }

    /// 📖️ Marks `tokens` as special (suppressible via `ProcessorSpec::SuppressSpecial`).
    pub async fn with_special(mut self, tokens: &[TokenId]) -> Self {
        for &token in tokens {
            self.special.set(token, true).await;
        }
        self
    }

    pub async fn is_eos(&self, token: TokenId) -> bool {
        self.eos.contains(&token)
    }

    /// 📖️ Errors unless `len` matches this vocabulary's declared size.
    pub async fn validate_logits_len(&self, len: usize) -> Result<(), SamplingError> {
        if len != self.size {
            Err(SamplingError::VocabMismatch { expected: self.size, actual: len })
        } else {
            Ok(())
        }
    }
}

/// 📖️ Maps [`TokenId`]s to their surface-form bytes, for constraints and stop matching that
/// operate on generated text rather than raw ids.
#[dyn_enum]
pub trait TokenTextAdapter {
    async fn vocab_size(&self) -> usize;
    /// 📖️ Raw (possibly partial-UTF-8) bytes of one token; `None` for byte-less special tokens.
    async fn token_bytes(&self, token: TokenId) -> Option<&[u8]>;
    /// 📖️ Stable hash of the whole token table, used to key automaton-state×token caches so a
    /// swapped tokenizer can never silently reuse another tokenizer's cached transitions.
    async fn fingerprint(&self) -> u64;
}

/// 📖️ Owning [`TokenTextAdapter`] over a copied `Vec<Vec<u8>>` token table — owned rather than
/// borrowed (unlike the `&[&[u8]]` this was built from) so it can sit in [`TokenTextAdapters`]
/// without a struct-level lifetime: `dyn_enum_close!`'s enum-name grammar has no generic/lifetime
/// parameter slot (verified against `🔀️dispatch/🦀️.rs`'s `DynEnumInput::parse`, which
/// reads a bare `Ident` for the enum name). This type is test-only in this crate today (never
/// constructed outside `#[cfg(test)]`), so the one-time copy in [`SliceTextAdapter::new`] costs
/// nothing on any real path.
pub struct SliceTextAdapter {
    pub(super) tokens: Vec<Vec<u8>>,
}

impl SliceTextAdapter {
    pub async fn new(tokens: &[&[u8]]) -> Self {
        Self { tokens: tokens.iter().map(|t| t.to_vec()).collect() }
    }
}

impl TokenTextAdapter for SliceTextAdapter {
    async fn vocab_size(&self) -> usize {
        self.tokens.len()
    }

    async fn token_bytes(&self, token: TokenId) -> Option<&[u8]> {
        self.tokens.get(token.get() as usize).map(Vec::as_slice)
    }

    async fn fingerprint(&self) -> u64 {
        // 📖️ FNV-1a over every token's bytes with a separator byte between entries so `["ab","c"]`
        // and `["a","bc"]` never collide.
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        for token in &self.tokens {
            for &b in token {
                hash = (hash ^ b as u64).wrapping_mul(0x0000_0100_0000_01b3);
            }
            hash = (hash ^ 0xFF).wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }
}

dyn_enum_close! {
    pub enum TokenTextAdapters: TokenTextAdapter {
        Slice(SliceTextAdapter),
    }
}
// #endregion 🔖️Vocabulary
