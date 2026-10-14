//! 🪪 Original raster content and revision identity.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RasterContentIdentity {
    width: u32,
    height: u32,
    byte_len: u64,
    digest_a: u64,
    digest_b: u64,
}

impl RasterContentIdentity {
    pub fn pixels(width: u32, height: u32, byte_len: usize) -> Option<Self> {
        let byte_len = u64::try_from(byte_len).ok()?;
        let mut identity = Self { width, height, byte_len, digest_a: 0x243f_6a88_85a3_08d3, digest_b: 0x1319_8a2e_0370_7344 };
        identity.mix_word(0x7069_7865_6c73_0001);
        identity.mix_word(u64::from(width));
        identity.mix_word(u64::from(height));
        identity.mix_word(byte_len);
        Some(identity)
    }

    pub fn revision(width: u32, height: u32, revisions: [u64; 6]) -> Option<Self> {
        let byte_len = u64::from(width).checked_mul(u64::from(height))?.checked_mul(4)?;
        let mut identity = Self { width, height, byte_len, digest_a: 0x243f_6a88_85a3_08d3, digest_b: 0x1319_8a2e_0370_7344 };
        identity.mix_word(0x7265_7669_7369_6f6e);
        identity.mix_word(u64::from(width));
        identity.mix_word(u64::from(height));
        identity.mix_word(byte_len);
        for revision in revisions {
            identity.mix_word(revision);
        }
        Some(identity)
    }

    pub fn mix_word(&mut self, word: u64) {
        self.digest_a ^= word.wrapping_mul(0x9e37_79b1_85eb_ca87);
        self.digest_a = self.digest_a.rotate_left(27).wrapping_mul(0x94d0_49bb_1331_11eb);
        self.digest_b = self.digest_b.wrapping_add(word ^ 0xd6e8_feb8_6659_fd93).rotate_left(31).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    }

    pub fn mix_bytes(&mut self, offset: usize, bytes: &[u8]) {
        for (relative, byte) in bytes.iter().copied().enumerate() {
            let index = u64::try_from(offset.saturating_add(relative)).unwrap_or(u64::MAX);
            let word = index.rotate_left(17) ^ u64::from(byte) ^ 0xa076_1d64_78bd_642f;
            self.digest_a ^= word.wrapping_mul(0xe703_7ed1_a0b4_28db);
            self.digest_b = self.digest_b.wrapping_add(word.wrapping_mul(0x8ebc_6af0_9c88_c6e3));
        }
    }

    pub fn dimensions_match(self, width: u32, height: u32, byte_len: usize) -> bool {
        self.width == width && self.height == height && u64::try_from(byte_len).is_ok_and(|byte_len| self.byte_len == byte_len)
    }

    #[cfg(any(test,feature="testkit"))]
    pub fn test_pixels(width: u32, height: u32, pixels: &[u8]) -> Option<Self> {
        let mut identity = Self::pixels(width, height, pixels.len())?;
        for (page, bytes) in pixels.chunks(crate::RASTER_CONTENT_PAGE_BYTES).enumerate() {
            identity.mix_bytes(page.saturating_mul(crate::RASTER_CONTENT_PAGE_BYTES), bytes);
        }
        Some(identity)
    }
}
