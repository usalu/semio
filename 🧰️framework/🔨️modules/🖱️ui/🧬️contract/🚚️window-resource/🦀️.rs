//! 🚚️ Exact text resources delivered through window-scoped page requests.

pub const WINDOW_RESOURCE_PAGE_BYTES: usize = 128 * 1024;
pub const WINDOW_RESOURCE_MAX_BYTES: usize = 4 * 1024 * 1024;
const PREFIX: &str = "framework.resource.v1.";
const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;

fn hash_from(mut hash: u64, source: &[u8]) -> u64 {
    for byte in source {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// 🪪️ Identifies the owner and full content for each page.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowResource {
    pub owner: u64,
    pub digest: u64,
    pub bytes: u32,
}

impl WindowResource {
    pub fn matches_owner(self, window: &str, key: &str) -> bool {
        self.owner == Self::owner_digest(window, key)
    }

    /// 🪪️ Derives the stable identity of a semantic resource path within one window.
    pub fn owner_digest(window: &str, key: &str) -> u64 {
        hash_from(hash_from(hash_from(OFFSET, window.as_bytes()), &[0]), key.as_bytes())
    }

    /// 🏷️ Binds a source to one window and one semantic resource path within it.
    pub fn new(window: &str, key: &str, source: &str) -> Option<Self> {
        if source.is_empty() || source.len() > WINDOW_RESOURCE_MAX_BYTES {
            return None;
        }
        let owner = Self::owner_digest(window, key);
        Some(Self { owner, digest: hash_from(OFFSET, source.as_bytes()), bytes: source.len() as u32 })
    }

    pub fn pages(self) -> u32 {
        (self.bytes as usize).div_ceil(WINDOW_RESOURCE_PAGE_BYTES) as u32
    }

    pub fn request_key(self) -> String {
        format!("{PREFIX}{:016x}.{:016x}.{}", self.owner, self.digest, self.bytes)
    }
}

/// 📄️ Identifies one one-based page of an owned resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowResourcePage {
    pub resource: WindowResource,
    pub index: u32,
}

impl WindowResourcePage {
    pub fn key(self) -> String {
        format!("{}.{}", self.resource.request_key(), self.index)
    }

    pub fn parse(key: &str) -> Option<Self> {
        let mut fields = key.strip_prefix(PREFIX)?.split('.');
        let hex = |text: &str| (text.len() == 16 && text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))).then(|| u64::from_str_radix(text, 16).ok()).flatten();
        let decimal = |text: &str| (!text.starts_with('0') && text.bytes().all(|b| b.is_ascii_digit())).then(|| text.parse::<u32>().ok()).flatten();
        let resource = WindowResource { owner: hex(fields.next()?)?, digest: hex(fields.next()?)?, bytes: decimal(fields.next()?)? };
        let index = decimal(fields.next()?)?;
        (fields.next().is_none() && resource.bytes as usize <= WINDOW_RESOURCE_MAX_BYTES && index <= resource.pages()).then_some(Self { resource, index })
    }

    /// ✂️ Borrows one page without cutting a UTF-8 scalar.
    pub fn text(self, source: &str) -> Option<&str> {
        if self.index == 0 || self.index > self.resource.pages() || source.len() != self.resource.bytes as usize {
            return None;
        }
        let boundary = |index: usize| {
            let mut end = index.min(source.len());
            while !source.is_char_boundary(end) {
                end -= 1;
            }
            end
        };
        let start = boundary((self.index - 1) as usize * WINDOW_RESOURCE_PAGE_BYTES);
        let end = boundary(self.index as usize * WINDOW_RESOURCE_PAGE_BYTES);
        source.get(start..end)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WindowResourceRefusal {
    Authority,
    Sequence,
    Extent,
    Digest,
    Capacity,
}

/// 📥️ Retains only the contiguous pages of one exact resource until completion.
pub struct WindowResourceReader {
    resource: WindowResource,
    next: u32,
    hash: u64,
    source: String,
}

impl WindowResourceReader {
    pub fn new(resource: WindowResource) -> Self {
        Self { resource, next: 1, hash: OFFSET, source: String::new() }
    }

    pub fn resource(&self) -> WindowResource {
        self.resource
    }

    pub fn next(&self) -> Option<u32> {
        (self.next <= self.resource.pages()).then_some(self.next)
    }

    pub fn completed_bytes(&self) -> usize {
        self.source.len()
    }

    pub fn accept(&mut self, page: WindowResourcePage, text: &str) -> Result<(), WindowResourceRefusal> {
        use WindowResourceRefusal::*;

        if page.resource != self.resource {
            return Err(Authority);
        }
        if self.next() != Some(page.index) {
            return Err(Sequence);
        }
        if self.resource.bytes as usize > WINDOW_RESOURCE_MAX_BYTES || text.is_empty() {
            return Err(Extent);
        }
        let end = self.source.len().checked_add(text.len()).ok_or(Extent)?;
        let expected = (page.index as usize * WINDOW_RESOURCE_PAGE_BYTES).min(self.resource.bytes as usize);
        let terminal = page.index == self.resource.pages();
        if end > expected || end < expected.saturating_sub(if terminal { 0 } else { 3 }) {
            return Err(Extent);
        }
        let hash = hash_from(self.hash, text.as_bytes());
        if terminal && hash != self.resource.digest {
            return Err(Digest);
        }
        self.source.try_reserve(text.len()).map_err(|_| Capacity)?;
        self.source.push_str(text);
        self.hash = hash;
        self.next += 1;
        Ok(())
    }

    pub fn complete(&self) -> Option<&str> {
        self.next().is_none().then_some(self.source.as_str())
    }
}
