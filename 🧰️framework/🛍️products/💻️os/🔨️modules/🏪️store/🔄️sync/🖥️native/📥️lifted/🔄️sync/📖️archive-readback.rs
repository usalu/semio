//! 📖 Compares the current durable archive through the folder log's physical record boundary.
use super::{extend_folder_checksum, BLOB_DELETE_EVENT, BLOB_PUT_EVENT, DOCUMENT_ARCHIVE_PUT_EVENT,
    DOCUMENT_PUT_EVENT, FOLDER_EVENT_MAGIC, MAX_FOLDER_EVENT_BYTES};
use std::io::{self, ErrorKind, Read, Seek, SeekFrom};

const PAGE: usize = 4096;

pub(super) fn archive_matches(
    file: &mut (impl Read + Seek), file_len: u64, document_id: &str, expected_archive: &[u8],
    cancelled: impl Fn() -> bool,
) -> io::Result<bool> {
    let scan = scan_folder(file, file_len, document_id, None, &cancelled)?;
    let Some(DocumentRecord::Archive { start, length, .. }) = scan.latest else { return Ok(false); };
    if length != expected_archive.len() as u64 { return Ok(false); }
    file.seek(SeekFrom::Start(start))?;
    let mut page = [0; PAGE];
    let mut equal = true;
    let mut compared = 0;
    read_pages(file, length, &mut page, &cancelled, |bytes| {
        equal &= expected_archive[compared..compared + bytes.len()] == *bytes;
        compared += bytes.len();
        Ok(())
    })?;
    check_cancelled(&cancelled)?;
    Ok(equal)
}

pub(super) enum DocumentRecord {
    Pair,
    Archive { start: u64, length: u64, schema_matches: bool },
}

pub(super) struct FolderScan {
    pub valid_end: u64,
    pub latest: Option<DocumentRecord>,
}

pub(super) fn scan_folder(
    file: &mut (impl Read + Seek), file_len: u64, document_id: &str, schema: Option<&str>, cancelled: &impl Fn() -> bool,
) -> io::Result<FolderScan> {
    let mut latest = None;
    let mut offset = 0;
    let mut page = [0; PAGE];
    while offset < file_len {
        check_cancelled(cancelled)?;
        file.seek(SeekFrom::Start(offset))?;
        let mut header = [0; 24];
        let available = (file_len - offset).min(header.len() as u64) as usize;
        file.read_exact(&mut header[..available])?;
        if available >= 8 && &header[..8] != FOLDER_EVENT_MAGIC {
            return Err(invalid("invalid folder event magic"));
        }
        if available < header.len() { break; }
        let length = u64::from_le_bytes(header[8..16].try_into().unwrap());
        if length > MAX_FOLDER_EVENT_BYTES { return Err(invalid("folder event length exceeds the record boundary")); }
        let start = offset + header.len() as u64;
        if length > file_len - start { break; }
        let end = start + length;
        let expected_checksum = u64::from_le_bytes(header[16..24].try_into().unwrap());
        let mut checksum = 0xcbf29ce484222325;
        read_pages(file, length, &mut page, cancelled, |bytes| {
            checksum = extend_folder_checksum(checksum, bytes);
            Ok(())
        })?;
        if checksum != expected_checksum {
            if end == file_len { break; }
            return Err(invalid("folder event checksum mismatch"));
        }
        file.seek(SeekFrom::Start(start))?;
        let mut record = Record { file, remaining: length };
        let mut head = [0; 9];
        record.read(&mut head)?;
        let kind = head[0];
        if !matches!(kind, DOCUMENT_PUT_EVENT | BLOB_PUT_EVENT | BLOB_DELETE_EVENT | DOCUMENT_ARCHIVE_PUT_EVENT) {
            return Err(invalid("unknown folder event kind"));
        }
        let matches_key = record.text(Some(document_id.as_bytes()), &mut page, cancelled)?;
        let schema_matches = record.text(schema.map(str::as_bytes), &mut page, cancelled)?;
        let (primary_offset, primary_length) = record.skip_data(end)?;
        record.skip_data(end)?;
        if record.remaining != 0 { return Err(invalid("folder event has trailing bytes")); }
        if matches_key && matches!(kind, DOCUMENT_PUT_EVENT | DOCUMENT_ARCHIVE_PUT_EVENT) {
            latest = Some(if kind == DOCUMENT_ARCHIVE_PUT_EVENT {
                DocumentRecord::Archive { start: primary_offset, length: primary_length, schema_matches }
            } else { DocumentRecord::Pair });
        }
        offset = end;
    }
    check_cancelled(cancelled)?;
    Ok(FolderScan { valid_end: offset, latest })
}

struct Record<'a, R> {
    file: &'a mut R,
    remaining: u64,
}

impl<R: Read + Seek> Record<'_, R> {
    fn bound(&mut self, bytes: u64) -> io::Result<()> {
        self.remaining = self.remaining.checked_sub(bytes).ok_or_else(|| invalid("truncated folder event"))?;
        Ok(())
    }

    fn read(&mut self, output: &mut [u8]) -> io::Result<()> {
        self.bound(output.len() as u64)?;
        self.file.read_exact(output)
    }

    fn text(&mut self, expected: Option<&[u8]>, page: &mut [u8; PAGE], cancelled: &impl Fn() -> bool) -> io::Result<bool> {
        let mut length = [0; 4];
        self.read(&mut length)?;
        let length = u64::from(u32::from_le_bytes(length));
        self.bound(length)?;
        let mut equal = expected.is_some_and(|bytes| bytes.len() as u64 == length);
        let mut offset = 0;
        let mut tail = [0; 4];
        let mut tail_length = 0;
        read_pages(self.file, length, page, cancelled, |mut bytes| {
            if equal {
                equal = expected.and_then(|value| value.get(offset..offset + bytes.len())) == Some(bytes);
            }
            offset += bytes.len();
            while tail_length != 0 && !bytes.is_empty() {
                tail[tail_length] = bytes[0];
                tail_length += 1;
                bytes = &bytes[1..];
                match std::str::from_utf8(&tail[..tail_length]) {
                    Ok(_) => tail_length = 0,
                    Err(error) if error.error_len().is_none() && tail_length < tail.len() => {}
                    Err(_) => return Err(invalid("invalid folder event text")),
                }
            }
            if let Err(error) = std::str::from_utf8(bytes) {
                if error.error_len().is_some() { return Err(invalid("invalid folder event text")); }
                let pending = &bytes[error.valid_up_to()..];
                tail[..pending.len()].copy_from_slice(pending);
                tail_length = pending.len();
            }
            Ok(())
        })?;
        if tail_length != 0 { return Err(invalid("truncated folder event text")); }
        Ok(equal)
    }

    fn skip_data(&mut self, end: u64) -> io::Result<(u64, u64)> {
        let mut length = [0; 8];
        self.read(&mut length)?;
        let length = u64::from_le_bytes(length);
        let start = end - self.remaining;
        self.bound(length)?;
        self.file.seek(SeekFrom::Current(length as i64))?;
        Ok((start, length))
    }
}

pub(super) fn read_pages(
    file: &mut impl Read, mut length: u64, page: &mut [u8; PAGE], cancelled: &impl Fn() -> bool,
    mut visit: impl FnMut(&[u8]) -> io::Result<()>,
) -> io::Result<()> {
    while length != 0 {
        check_cancelled(cancelled)?;
        let count = length.min(PAGE as u64) as usize;
        file.read_exact(&mut page[..count])?;
        visit(&page[..count])?;
        length -= count as u64;
    }
    Ok(())
}

fn check_cancelled(cancelled: &impl Fn() -> bool) -> io::Result<()> {
    if cancelled() { Err(io::Error::new(ErrorKind::Interrupted, "folder archive verification cancelled")) } else { Ok(()) }
}

fn invalid(message: &'static str) -> io::Error { io::Error::new(ErrorKind::InvalidData, message) }
