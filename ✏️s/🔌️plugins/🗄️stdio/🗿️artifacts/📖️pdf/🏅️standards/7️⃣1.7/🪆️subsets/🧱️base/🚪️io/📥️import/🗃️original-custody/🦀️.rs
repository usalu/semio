//! 🗃️ Local custody of the original bytes of an opened PDF file, content-addressed by the digest of its retained object graph, so a later Save can append to the file instead of rewriting it.

use std::sync::{Arc, Mutex, PoisonError};
use semio_framework_hash::Sha256;
use semio_framework_value::ByteReadSource;
use crate::standards::v1_7::subsets::base::{
    modules::lexer::object_bytes,
    schema::snapshot::{PdfIndirectObject, PdfObject},
};

/// 📄️ Bytes of one custody page; a file is never held as one contiguous allocation.
pub const PAGE_BYTES: usize = 4096;

/// 🧮️ Most files (opened origins and rebased saves) the custody keeps before the least recently used one is dropped.
const MAXIMUM_ENTRIES: usize = 16;

/// 🧮️ Most bytes the custody keeps in total (shared pages are counted once per entry).
const MAXIMUM_BYTES: usize = 128 << 20;

/// 🧾️ Bytes the graph digest hashes per step.
pub const DIGEST_STEP_BYTES: usize = 4096;

/// 📄️ File bytes in shared 4 KiB pages: cloning shares every page, appending adds pages only.
#[derive(Clone, Default)]
pub struct FilePages {
    pages: Vec<Arc<[u8]>>,
    starts: Vec<usize>,
    len: usize,
}

impl FilePages {
    pub fn len(&self) -> usize { self.len }

    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// 📎️ Appends one page (at most [`PAGE_BYTES`]) to the end of the file.
    pub fn push_page(&mut self, page: Arc<[u8]>) {
        if page.is_empty() { return; }
        self.starts.push(self.len);
        self.len += page.len();
        self.pages.push(page);
    }

    /// 📎️ Appends the pages of another file.
    pub fn append(&mut self, tail: &FilePages) {
        for page in &tail.pages { self.push_page(page.clone()); }
    }

    /// 📎️ Appends a slice, page by page.
    pub fn push_slice(&mut self, bytes: &[u8]) {
        for page in bytes.chunks(PAGE_BYTES) { self.push_page(Arc::from(page)); }
    }
}

impl ByteReadSource for FilePages {
    fn byte_len(&self) -> usize { self.len }

    fn byte_at(&self, index: usize) -> Option<u8> {
        let mut byte = [0u8];
        (self.read_bytes(index, &mut byte) == 1).then_some(byte[0])
    }

    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize {
        let mut written = 0;
        let mut at = offset;
        while written < output.len() && at < self.len {
            let index = self.starts.partition_point(|start| *start <= at) - 1;
            let page = &self.pages[index];
            let inside = at - self.starts[index];
            let count = (page.len() - inside).min(output.len() - written);
            output[written..written + count].copy_from_slice(&page[inside..inside + count]);
            written += count;
            at += count;
        }
        written
    }
}

/// 🪣️ Copies a source into custody pages, one page per step, so opening never pays for the whole file at once.
#[derive(Clone, Default)]
pub struct Stash {
    file: FilePages,
}

impl Stash {
    /// 🪜️ Copies the next page of `source`; true once the whole source is held.
    pub fn advance(&mut self, source: &dyn ByteReadSource) -> bool {
        let total = source.byte_len();
        if self.file.len >= total { return true; }
        let mut page = [0u8; PAGE_BYTES];
        let count = source.read_bytes(self.file.len, &mut page[..PAGE_BYTES.min(total - self.file.len)]);
        if count == 0 { return true; }
        self.file.push_page(Arc::from(&page[..count]));
        self.file.len >= total
    }

    pub fn finish(self) -> FilePages { self.file }
}

/// 📎️ Collects the bytes a Save appends to its base file into custody pages as they are written.
#[derive(Clone, Default)]
pub struct Appender {
    file: FilePages,
    partial: Vec<u8>,
}

impl Appender {
    pub fn push(&mut self, mut bytes: &[u8]) {
        while !bytes.is_empty() {
            let count = (PAGE_BYTES - self.partial.len()).min(bytes.len());
            self.partial.extend_from_slice(&bytes[..count]);
            bytes = &bytes[count..];
            if self.partial.len() == PAGE_BYTES { self.file.push_page(Arc::from(std::mem::take(&mut self.partial))); }
        }
    }

    pub fn finish(mut self) -> FilePages {
        if !self.partial.is_empty() { self.file.push_page(Arc::from(std::mem::take(&mut self.partial))); }
        self.file
    }
}

/// 🗃️ An original file as the next Save appends to it: the file bytes, the digest of the retained graph it was read from and the object numbers earlier appended sections replaced.
pub struct RetainedOriginal {
    pub file: FilePages,
    pub objects: [u8; 32],
    pub touched: Vec<u32>,
}

impl ByteReadSource for RetainedOriginal {
    fn byte_len(&self) -> usize { self.file.byte_len() }
    fn byte_at(&self, index: usize) -> Option<u8> { self.file.byte_at(index) }
    fn read_bytes(&self, offset: usize, output: &mut [u8]) -> usize { self.file.read_bytes(offset, output) }
}

struct Entry {
    objects: [u8; 32],
    document: String,
    saved: Arc<RetainedOriginal>,
}

static CUSTODY: Mutex<Vec<Entry>> = Mutex::new(Vec::new());

fn insert(entry: Entry) {
    let mut custody = CUSTODY.lock().unwrap_or_else(PoisonError::into_inner);
    custody.retain(|held| held.objects != entry.objects || held.document != entry.document);
    custody.push(entry);
    while custody.len() > 1 && (custody.len() > MAXIMUM_ENTRIES || custody.iter().map(|held| held.saved.file.len()).sum::<usize>() > MAXIMUM_BYTES) {
        custody.remove(0);
    }
}

/// 📥️ Keeps the original file an import just read, under the digest of the retained graph it produced.
pub fn retain(file: FilePages, objects: [u8; 32]) {
    insert(Entry { objects, document: String::new(), saved: Arc::new(RetainedOriginal { file, objects, touched: Vec::new() }) });
}

/// 📥️ Keeps a whole in-memory file (the small synchronous import) under the digest of its retained graph.
pub fn retain_file(bytes: &[u8], objects: &[PdfIndirectObject]) {
    if objects.is_empty() { return; }
    let mut file = FilePages::default();
    file.push_slice(bytes);
    retain(file, objects_digest(objects));
}

/// 🔎️ The file a document appends to: its own last saved file, else the origin file of that graph.
pub fn find(objects: &[u8; 32], document: &str) -> Option<Arc<RetainedOriginal>> {
    let mut custody = CUSTODY.lock().unwrap_or_else(PoisonError::into_inner);
    let at = custody.iter().position(|held| held.objects == *objects && held.document == document)
        .or_else(|| custody.iter().position(|held| held.objects == *objects && held.document.is_empty()))?;
    let entry = custody.remove(at);
    let saved = entry.saved.clone();
    custody.push(entry);
    Some(saved)
}

/// ♻️ Makes the saved file the next Save's base for `document`.
pub fn rebase(document: &str, saved: RetainedOriginal) {
    let objects = saved.objects;
    insert(Entry { objects, document: document.to_owned(), saved: Arc::new(saved) });
}

/// 🧾️ An order independent SHA-256 over the retained objects: every row hashes its id, directly spelled value and stream bodies in bounded chunks, and the row hashes are summed.
#[derive(Clone, Default)]
pub struct GraphDigest {
    sum: [u64; 4],
    sha: Sha256,
    index: usize,
    offset: usize,
    started: bool,
}

impl GraphDigest {
    pub fn done(&self, count: usize) -> bool { self.index >= count }

    /// 🪜️ Hashes about `budget` bytes of the rows `at` yields for `0..count` and returns how many it hashed.
    pub fn advance<'a>(&mut self, count: usize, mut at: impl FnMut(usize) -> Option<&'a PdfIndirectObject>, budget: usize) -> usize {
        let mut spent = 0;
        while spent < budget && self.index < count {
            let Some(object) = at(self.index) else { break };
            if !self.started {
                self.started = true;
                self.sha.update(&object.id.num.to_be_bytes());
                self.sha.update(&object.id.gen.to_be_bytes());
                let head = match &object.value {
                    PdfObject::Stream { dict, filters, .. } => {
                        let mut head = b"stream".to_vec();
                        head.extend_from_slice(&object_bytes(&PdfObject::Dict(dict.clone())));
                        head.extend_from_slice(format!("{filters:?}").as_bytes());
                        head
                    }
                    value => object_bytes(value),
                };
                self.sha.update(&(head.len() as u64).to_be_bytes());
                self.sha.update(&head);
                spent += head.len();
            }
            if let PdfObject::Stream { data, .. } = &object.value {
                if self.offset < data.len() {
                    let mut page = [0u8; PAGE_BYTES];
                    let count = ByteReadSource::read_bytes(data, self.offset, &mut page);
                    self.sha.update(&page[..count]);
                    self.offset += count;
                    spent += count.max(1);
                    if count == 0 { self.offset = data.len(); }
                    if self.offset < data.len() { continue; }
                }
            }
            let row = std::mem::take(&mut self.sha).finalize();
            for (slot, chunk) in self.sum.iter_mut().zip(row.chunks_exact(8)) {
                *slot = slot.wrapping_add(u64::from_be_bytes(chunk.try_into().expect("eight digest bytes")));
            }
            self.index += 1;
            self.offset = 0;
            self.started = false;
        }
        spent.max(1)
    }

    pub fn finish(self, count: usize) -> [u8; 32] {
        let mut sha = Sha256::new();
        for word in self.sum { sha.update(&word.to_be_bytes()); }
        sha.update(&(count as u64).to_be_bytes());
        sha.finalize()
    }
}

/// 🧾️ The digest of a retained graph in one go.
pub fn objects_digest(objects: &[PdfIndirectObject]) -> [u8; 32] {
    let mut digest = GraphDigest::default();
    while !digest.done(objects.len()) { digest.advance(objects.len(), |index| objects.get(index), usize::MAX); }
    digest.finish(objects.len())
}
