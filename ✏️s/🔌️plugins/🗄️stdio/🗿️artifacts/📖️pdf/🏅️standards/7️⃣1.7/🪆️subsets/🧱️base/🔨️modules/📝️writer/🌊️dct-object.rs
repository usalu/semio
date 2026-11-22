//! 🌊️ Emits an admitted JPEG XObject through the classic PDF writer with bounded byte custody.
use super::{encode_stream, write_dict, ObjRef, PdfDictEntry, PdfObject, PdfWriter};
use crate::standards::v1_7::subsets::base::{
    io::foreign_artifacts::dct_view::{PdfDctImageColor, PdfDctImageView},
    schema::snapshot::PdfStreamFilter,
};
use semio_framework_value::paged::PAGED_BYTES_CHUNK_BYTES;
use std::io::{self, Write};

pub const MAX_DCT_OBJECT_PREFIX: usize = 256;
const SUFFIX: &[u8] = b"\nendstream\nendobj\n";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DctObjectStartError {
    UnsupportedWriter,
    InvalidObject,
    DocumentTooLarge,
}

#[derive(Debug)]
pub enum DctObjectWriteError {
    Faulted,
    ResourceMismatch,
    SourceChanged,
    InvalidSinkCount,
    Sink(io::Error),
}

#[derive(Default)]
pub struct DctObjectCounts {
    pub read: usize,
    pub written: usize,
}

pub struct DctObjectTurn {
    pub counts: DctObjectCounts,
    pub result: Result<bool, DctObjectWriteError>,
}

pub struct DctObjectWriter {
    writer: PdfWriter,
    object: ObjRef,
    digest: [u8; 32],
    prefix: Vec<u8>,
    length: usize,
    end: usize,
    written: usize,
    faulted: bool,
}

impl PdfWriter {
    /// 🔒️ Transfers exclusive writer custody until the complete classic PDF image object has been emitted.
    pub fn begin_dct_object(
        self,
        object: ObjRef,
        image: PdfDctImageView<'_>,
    ) -> Result<DctObjectWriter, (DctObjectStartError, Self)> {
        if self.cipher.is_some() || self.xref_stream {
            return Err((DctObjectStartError::UnsupportedWriter, self));
        }
        if object.num == 0 || object.num == u32::MAX || self.offsets.contains_key(&object.num) {
            return Err((DctObjectStartError::InvalidObject, self));
        }

        let length = image.encoded().byte_len();
        let Ok(pdf_length) = u32::try_from(length) else { return Err((DctObjectStartError::DocumentTooLarge, self)); };
        let (width, height) = image.dimensions();
        let space = match image.color() {
            PdfDctImageColor::Gray { .. } => "DeviceGray",
            PdfDctImageColor::Rgb { .. } => "DeviceRGB",
        };
        let mut entries = vec![
            PdfDictEntry::new("Type", PdfObject::name("XObject")),
            PdfDictEntry::new("Subtype", PdfObject::name("Image")),
            PdfDictEntry::new("Width", PdfObject::Int(i64::from(width))),
            PdfDictEntry::new("Height", PdfObject::Int(i64::from(height))),
            PdfDictEntry::new("BitsPerComponent", PdfObject::Int(8)),
            PdfDictEntry::new("ColorSpace", PdfObject::name(space)),
        ];
        let filter = PdfStreamFilter::Dct { color_transform: image.color().color_transform() };
        entries.extend(encode_stream(&[], &[filter]).1);
        entries.push(PdfDictEntry::new("Length", PdfObject::Int(i64::from(pdf_length))));

        let mut prefix = Vec::with_capacity(MAX_DCT_OBJECT_PREFIX);
        writeln!(prefix, "{} {} obj", object.num, object.gen).expect("writing into a byte vector");
        write_dict(&mut prefix, &entries);
        prefix.extend_from_slice(b"\nstream\n");
        debug_assert!(prefix.len() <= MAX_DCT_OBJECT_PREFIX);
        let end = self.offset.checked_add(prefix.len())
            .and_then(|offset| offset.checked_add(length))
            .and_then(|offset| offset.checked_add(SUFFIX.len()));
        let Some(end) = end.filter(|offset| u32::try_from(*offset).is_ok()) else {
            return Err((DctObjectStartError::DocumentTooLarge, self));
        };
        Ok(DctObjectWriter { writer: self, object, digest: image.digest(), prefix, length, end, written: 0, faulted: false })
    }
}

impl DctObjectWriter {
    fn complete(&self) -> bool {
        !self.faulted && self.written == self.end - self.writer.offset
    }

    /// ⏩️ Transfers one grant-bounded span from the matching graph loan into the operation's sequential sink.
    pub fn step(&mut self, image: PdfDctImageView<'_>, sink: &mut dyn Write, grant: usize) -> DctObjectTurn {
        let mut counts = DctObjectCounts::default();
        if self.faulted {
            return DctObjectTurn { counts, result: Err(DctObjectWriteError::Faulted) };
        }
        let source = image.encoded();
        if image.digest() != self.digest || source.byte_len() != self.length {
            return DctObjectTurn { counts, result: Err(DctObjectWriteError::ResourceMismatch) };
        }
        let body_start = self.prefix.len();
        let body_end = body_start + self.length;
        let body_offset = self.written.saturating_sub(body_start);
        let mut page = [0u8; PAGED_BYTES_CHUNK_BYTES];
        let pending = if self.written < body_start {
            &self.prefix[self.written..body_start.min(self.written.saturating_add(grant))]
        } else if body_offset < self.length {
            let count = (self.length - body_offset).min(page.len()).min(grant / 2);
            counts.read = source.read_bytes(body_offset, &mut page[..count]);
            if counts.read != count {
                self.faulted = true;
                return DctObjectTurn { counts, result: Err(DctObjectWriteError::SourceChanged) };
            }
            &page[..count]
        } else {
            let suffix_offset = self.written - body_end;
            &SUFFIX[suffix_offset..SUFFIX.len().min(suffix_offset.saturating_add(grant))]
        };
        if pending.is_empty() {
            return DctObjectTurn { counts, result: Ok(self.complete()) };
        }
        let result = match sink.write(pending) {
            Ok(written) if written <= pending.len() => {
                self.written += written;
                counts.written = written;
                Ok(self.complete())
            }
            Ok(_) => {
                self.faulted = true;
                Err(DctObjectWriteError::InvalidSinkCount)
            }
            Err(error) => Err(DctObjectWriteError::Sink(error)),
        };
        DctObjectTurn { counts, result }
    }

    /// ✅️ Registers the complete object's byte offset and resumes the same document writer.
    pub fn finish(mut self) -> Result<PdfWriter, Self> {
        if !self.complete() { return Err(self); }
        self.writer.offsets.insert(self.object.num, (self.writer.offset, self.object.gen));
        self.writer.max_number = self.writer.max_number.max(self.object.num);
        self.writer.offset = self.end;
        Ok(self.writer)
    }
}
