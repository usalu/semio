//#region 📄️NeutralByteStorage
pub const ACTOR_BYTE_PAGE_BYTES: usize = 4096;

#[derive(Debug, PartialEq, Eq)]
pub struct ActorBytePage {
    bytes: [u8; ACTOR_BYTE_PAGE_BYTES],
    len: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ActorBytePageBlock { words: [u64; 8] }

impl ActorBytePageBlock {
    pub fn words(&self) -> &[u64; 8] { &self.words }
}

impl ActorBytePage {
    /// 📥️ Validates one fixed storage owner without granting command, return or lifetime authority.
    pub fn try_from_array(bytes: [u8; ACTOR_BYTE_PAGE_BYTES], len: u32) -> Result<Self, &'static str> {
        if len > ACTOR_BYTE_PAGE_BYTES as u32 { return Err("actor-page.length"); }
        if bytes[len as usize..].iter().any(|byte| *byte != 0) { return Err("actor-page.padding"); }
        Ok(Self { bytes, len: len as u16 })
    }

    /// 📋️ Initializes exactly one admitted backing from the selected slice and canonical zero tail.
    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, &'static str> {
        if bytes.len() > ACTOR_BYTE_PAGE_BYTES { return Err("actor-page.length"); }
        Ok(Self { bytes: std::array::from_fn(|index| bytes.get(index).copied().unwrap_or(0)), len: bytes.len() as u16 })
    }

    pub fn as_slice(&self) -> &[u8] { &self.bytes[..usize::from(self.len)] }
    pub fn storage(&self) -> &[u8; ACTOR_BYTE_PAGE_BYTES] { &self.bytes }
    pub fn len(&self) -> usize { usize::from(self.len) }
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// 🔢️ Projects one fixed little-endian block without allocating or inspecting unrelated fields.
    pub fn block(&self, index: usize) -> Option<ActorBytePageBlock> {
        if index >= 64 { return None; }
        Some(ActorBytePageBlock { words: std::array::from_fn(|word| u64::from_le_bytes(std::array::from_fn(|byte| self.bytes[index * 64 + word * 8 + byte]))) })
    }
}
//#endregion 📄️NeutralByteStorage
