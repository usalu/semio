//! 🧵️ The portable prepared actor wire contains only initialized resources of a content-bound Core module.

use super::*;
use crate::checkpoint_codec::{encode_checkpoint_value, CheckpointReader};

const HEADER: &[u8; 9] = b"SEMIOPA\0\x01";
const CHUNK_BYTES: usize = 65_536;
const NULL_FUNCTION: u64 = u64::MAX;

impl PreparedCoreInstance {
    /// 📤️ Streams a sealed image in bounded chunks. The caller binds the actual retained component SHA-256 and the sink owns cancellation.
    pub fn write_image(&self, component_sha256: [u8; 32], maximum_wire_bytes: usize, write: impl FnMut(&[u8]) -> Result<(), CoreError>) -> Result<(), CoreError> {
        let mut wire = ImageWriter { write, remaining: maximum_wire_bytes };
        wire.bytes(HEADER)?;
        wire.bytes(&component_sha256)?;
        wire.u64(self.module.bytes_fingerprint)?;

        wire.count(self.memories.len())?;
        for memory in &self.memories {
            wire.count(memory.bytes.len())?;
            for chunk in memory.bytes.chunks(CHUNK_BYTES) {
                wire.bytes(chunk)?;
            }
        }

        wire.count(self.tables.len())?;
        let mut buffer = [0; CHUNK_BYTES];
        for table in &self.tables {
            wire.count(table.values.len())?;
            for chunk in table.values.chunks(CHUNK_BYTES / 8) {
                for (value, encoded) in chunk.iter().zip(buffer.chunks_exact_mut(8)) {
                    encoded.copy_from_slice(&value.unwrap_or(NULL_FUNCTION).to_le_bytes());
                }
                wire.bytes(&buffer[..chunk.len() * 8])?;
            }
        }

        wire.count(self.globals.len())?;
        for global in &self.globals {
            wire.bytes(encode_checkpoint_value(global.value, &mut [0; 10]))?;
        }
        wire.segments(&self.data)?;
        wire.segments(&self.elements)
    }

    pub(crate) fn read_image(module: Arc<CoreModule>, digest: [u8; 32], bytes: &[u8], maximum_wire_bytes: usize, maximum_storage_bytes: usize, mut checkpoint: impl FnMut() -> bool) -> Result<Self, CoreError> {
        check_prepared_checkpoint(&mut checkpoint)?;
        if bytes.len() > maximum_wire_bytes {
            return Err(state("prepared image exceeds the wire byte limit"));
        }
        let mut reader = CheckpointReader::new(bytes);
        if reader.bytes(HEADER.len())? != HEADER || reader.bytes(digest.len())? != digest || reader.u64()? != module.bytes_fingerprint {
            return Err(state("prepared image header or exact component identity differs"));
        }
        let mut storage = ImageStorage { remaining: maximum_storage_bytes, checkpoint: &mut checkpoint };

        exact_count(&mut reader, module.memories.len(), 8)?;
        let mut memories = storage.vector(module.memories.len())?;
        for limits in &module.memories {
            let length = reader.usize()?;
            let pages = (length / WASM_PAGE_BYTES) as u64;
            if length % WASM_PAGE_BYTES != 0 || pages < limits.minimum_pages || limits.maximum_pages.is_some_and(|maximum| pages > maximum) {
                return Err(state("prepared image memory exceeds its module extent"));
            }
            let contents = storage.copy(reader.bytes(length)?)?;
            memories.push(MemoryState { bytes: contents, maximum_pages: limits.maximum_pages, memory64: limits.memory64 });
        }

        exact_count(&mut reader, module.tables.len(), 8)?;
        let mut tables = storage.vector(module.tables.len())?;
        for limits in &module.tables {
            let count = reader.usize()?;
            if (count as u64) < limits.minimum || limits.maximum.is_some_and(|maximum| count as u64 > maximum) {
                return Err(state("prepared image table exceeds its module extent"));
            }
            let records = reader.bytes(byte_count::<u64>(count)?)?;
            let mut values = storage.vector(count)?;
            for chunk in records.chunks(CHUNK_BYTES) {
                check_prepared_checkpoint(storage.checkpoint)?;
                for encoded in chunk.chunks_exact(8) {
                    let value = u64::from_le_bytes(encoded.try_into().unwrap());
                    if value != NULL_FUNCTION && (limits.element != ValueType::FuncRef || value >= module.functions.len() as u64) {
                        return Err(state("prepared image table contains a foreign reference"));
                    }
                    values.push((value != NULL_FUNCTION).then_some(value));
                }
            }
            tables.push(TableState { values, element: limits.element, maximum: limits.maximum });
        }

        exact_count(&mut reader, module.globals.len(), 2)?;
        let mut globals = storage.vector(module.globals.len())?;
        for declared in &module.globals {
            check_prepared_checkpoint(storage.checkpoint)?;
            let value = reader.value()?;
            globals.push(GlobalState { value, mutable: declared.mutable });
        }
        let data = storage.segments(&mut reader, &module.data, |segment| (segment.mode, &segment.bytes))?;
        let elements = storage.segments(&mut reader, &module.elements, |segment| (segment.mode, &segment.values))?;
        if !reader.done() {
            return Err(state("prepared image has trailing bytes"));
        }
        Self::seal_checked(CoreInstance { module, memories, tables, globals, data, elements, machine: None, next_host_call: 1, preparation_storage: None }, &mut checkpoint)
    }
}

struct ImageWriter<W> {
    write: W,
    remaining: usize,
}

impl<W: FnMut(&[u8]) -> Result<(), CoreError>> ImageWriter<W> {
    fn bytes(&mut self, bytes: &[u8]) -> Result<(), CoreError> {
        self.remaining = self.remaining.checked_sub(bytes.len()).ok_or_else(|| state("prepared image exceeds the wire byte limit"))?;
        (self.write)(bytes)
    }

    fn u64(&mut self, value: u64) -> Result<(), CoreError> {
        self.bytes(&value.to_le_bytes())
    }

    fn count(&mut self, value: usize) -> Result<(), CoreError> {
        self.u64(value as u64)
    }

    fn segments<T>(&mut self, segments: &[Option<Vec<T>>]) -> Result<(), CoreError> {
        self.count(segments.len())?;
        for segment in segments {
            self.bytes(&[u8::from(segment.is_some())])?;
        }
        Ok(())
    }
}

struct ImageStorage<'a, C> {
    remaining: usize,
    checkpoint: &'a mut C,
}

impl<C: FnMut() -> bool> ImageStorage<'_, C> {
    fn vector<T>(&mut self, count: usize) -> Result<Vec<T>, CoreError> {
        check_prepared_checkpoint(self.checkpoint)?;
        if byte_count::<T>(count)? > self.remaining {
            return Err(state("prepared image exceeds the storage byte limit"));
        }
        let mut values = Vec::new();
        values.try_reserve_exact(count).map_err(|_| state("prepared image storage allocation refused"))?;
        self.remaining = self.remaining.checked_sub(byte_count::<T>(values.capacity())?).ok_or_else(|| state("prepared image exceeds the storage byte limit"))?;
        Ok(values)
    }

    fn copy<T: Copy>(&mut self, source: &[T]) -> Result<Vec<T>, CoreError> {
        let mut destination = self.vector(source.len())?;
        for chunk in source.chunks((CHUNK_BYTES / size_of::<T>().max(1)).max(1)) {
            check_prepared_checkpoint(self.checkpoint)?;
            destination.extend_from_slice(chunk);
        }
        Ok(destination)
    }

    fn segments<S, T: Copy>(&mut self, reader: &mut CheckpointReader<'_>, declarations: &[S], project: impl Fn(&S) -> (SegmentMode, &[T])) -> Result<Vec<Option<Vec<T>>>, CoreError> {
        exact_count(reader, declarations.len(), 1)?;
        let mut segments = self.vector(declarations.len())?;
        for (present, declaration) in reader.bytes(declarations.len())?.iter().zip(declarations) {
            check_prepared_checkpoint(self.checkpoint)?;
            let (mode, source) = project(declaration);
            let segment = match *present {
                0 => None,
                1 if mode == SegmentMode::Passive => Some(self.copy(source)?),
                _ => return Err(state("prepared image claims an invalid passive segment")),
            };
            segments.push(segment);
        }
        Ok(segments)
    }
}

fn exact_count(reader: &mut CheckpointReader<'_>, expected: usize, minimum_bytes: usize) -> Result<(), CoreError> {
    if reader.usize()? != expected || expected > reader.remaining() / minimum_bytes {
        return Err(state("prepared image vector differs from its module shape or wire extent"));
    }
    Ok(())
}

fn byte_count<T>(count: usize) -> Result<usize, CoreError> {
    count.checked_mul(size_of::<T>()).filter(|size| *size <= isize::MAX as usize).ok_or_else(|| state("prepared image extent overflows its host address space"))
}

fn state(message: &'static str) -> CoreError {
    CoreError::State(message.into())
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
