//! 🧱️ Token bitset: the bitset over token ids with its set algebra. See [`super`].

use super::*;

// #region 🔖️Bitset
/// 🎭️ A dense bitset over `0..len` token indices, `u64`-word packed. The single shared
/// representation for hard masks (constraints, allow/forbid lists, forced-token exclusivity) —
/// every mask operation the pipeline needs is `O(vocab / 64)`.
#[derive(Clone, PartialEq, Debug)]
pub struct TokenBitset {
    pub(super) words: Vec<u64>,
    pub(super) len: usize,
}

impl TokenBitset {
    /// 🎭️ All-zero (empty) bitset over `len` tokens.
    pub async fn new_empty(len: usize) -> Self {
        Self { words: vec![0u64; len.div_ceil(64)], len }
    }

    /// 🎭️ All-one (full) bitset over `len` tokens.
    pub async fn new_full(len: usize) -> Self {
        let mut set = Self::new_empty(len).await;
        set.fill().await;
        set
    }

    pub async fn len(&self) -> usize {
        self.len
    }

    pub async fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub async fn get(&self, token: TokenId) -> bool {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        (self.words[idx / 64] >> (idx % 64)) & 1 != 0
    }

    #[inline]
    pub async fn set(&mut self, token: TokenId, value: bool) {
        let idx = token.get() as usize;
        debug_assert!(idx < self.len);
        let mask = 1u64 << (idx % 64);
        if value {
            self.words[idx / 64] |= mask;
        } else {
            self.words[idx / 64] &= !mask;
        }
    }

    /// 🎭️ Sets every bit `0..len` (trailing bits beyond `len` in the final word stay zero).
    pub async fn fill(&mut self) {
        let full_words = self.len / 64;
        self.words[..full_words].fill(u64::MAX);
        let rem = self.len % 64;
        if rem > 0 {
            self.words[full_words] = (1u64 << rem) - 1;
        }
    }

    pub async fn clear_all(&mut self) {
        self.words.fill(0);
    }

    /// 🎭️ In-place `self &= other`.
    pub async fn and_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
        }
    }

    /// 🎭️ In-place `self |= other`.
    pub async fn or_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }

    /// 🎭️ In-place `self &= !other`.
    pub async fn and_not_with(&mut self, other: &TokenBitset) {
        debug_assert_eq!(self.len, other.len);
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= !*b;
        }
    }

    pub async fn count_ones(&self) -> usize {
        self.words.iter().map(|w| w.count_ones() as usize).sum()
    }

    pub async fn is_all_zero(&self) -> bool {
        self.words.iter().all(|&w| w == 0)
    }

    /// 🎭️ Lowest set bit, word-skipping past all-zero words.
    pub async fn first_set(&self) -> Option<TokenId> {
        for (word_idx, &word) in self.words.iter().enumerate() {
            if word != 0 {
                let bit = word.trailing_zeros() as usize;
                return Some(TokenId::new((word_idx * 64 + bit) as u32));
            }
        }
        None
    }

    /// 🎭️ Iterates set bits in ascending order, skipping whole zero words at a time.
    pub async fn iter_ones(&self) -> impl Iterator<Item = TokenId> + '_ {
        self.words.iter().enumerate().flat_map(|(word_idx, &word)| {
            let mut remaining = word;
            core::iter::from_fn(move || {
                if remaining == 0 {
                    return None;
                }
                let bit = remaining.trailing_zeros();
                remaining &= remaining - 1;
                Some(TokenId::new((word_idx * 64 + bit as usize) as u32))
            })
        })
    }
}
// #endregion 🔖️Bitset
