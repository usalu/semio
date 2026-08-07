//! 🚰 Random-access PackSource and PackSink traits.

use crate::os_pack::codec::PackError;

//#region 🔖️Source
/// @emoji 📥️ Random-access read source a pack file is decoded from — implementable over an
/// in-memory slice, a file (see `pack_io`), or (via `pack_async`) a network range-fetcher.
pub trait PackSource {
    fn len(&self) -> u64;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError>;

    fn read_exact_at(&self, offset: u64, buf: &mut [u8]) -> Result<(), PackError> {
        let mut filled = 0usize;
        while filled < buf.len() {
            let read = self.read_at(offset + filled as u64, &mut buf[filled..])?;
            if read == 0 {
                return Err(PackError::Truncated(offset + filled as u64));
            }
            filled += read;
        }
        Ok(())
    }
}

impl PackSource for &[u8] {
    fn len(&self) -> u64 {
        (*self).len() as u64
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
        let slice: &[u8] = self;
        let total = slice.len() as u64;
        if offset > total {
            return Err(PackError::Truncated(offset));
        }
        let available = &slice[offset as usize..];
        let n = available.len().min(buf.len());
        buf[..n].copy_from_slice(&available[..n]);
        Ok(n)
    }
}

impl PackSource for Vec<u8> {
    fn len(&self) -> u64 {
        self.as_slice().len() as u64
    }

    fn read_at(&self, offset: u64, buf: &mut [u8]) -> Result<usize, PackError> {
        self.as_slice().read_at(offset, buf)
    }
}

/// @emoji 📤️ Append-only write sink a pack file is encoded into — implementable over a
/// `Vec<u8>`, a file (see `pack_io`), or any other ordered byte destination.
pub trait PackSink {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError>;

    fn position(&self) -> u64;

    fn flush(&mut self) -> Result<(), PackError> {
        Ok(())
    }
}

impl PackSink for Vec<u8> {
    fn write_all(&mut self, bytes: &[u8]) -> Result<(), PackError> {
        self.extend_from_slice(bytes);
        Ok(())
    }

    fn position(&self) -> u64 {
        self.len() as u64
    }
}

    //#region 🔖️Source
    #[test]
    fn pack_source_over_slice_reads_at_offset() {
        let data: &[u8] = b"hello world";
        let mut buf = [0u8; 5];
        let n = data.read_at(6, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"world");
        assert_eq!(PackSource::len(&data), 11);
        assert!(!data.is_empty());
    }

    #[test]
    fn pack_source_over_slice_short_read_at_end_never_panics() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 10];
        let n = data.read_at(0, &mut buf).unwrap();
        assert_eq!(n, 2);
        assert_eq!(&buf[..2], b"hi");
    }

    #[test]
    fn pack_source_read_at_offset_past_end_errors_never_panics() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 4];
        let result = data.read_at(100, &mut buf);
        assert_eq!(result, Err(PackError::Truncated(100)));
    }

    #[test]
    fn pack_source_read_exact_at_errors_on_truncated_input() {
        let data: &[u8] = b"hi";
        let mut buf = [0u8; 5];
        let result = data.read_exact_at(0, &mut buf);
        assert!(matches!(result, Err(PackError::Truncated(_))));
    }

    #[test]
    fn pack_source_over_vec_matches_slice_behavior() {
        let data: Vec<u8> = b"hello world".to_vec();
        let mut buf = [0u8; 5];
        let n = data.read_at(0, &mut buf).unwrap();
        assert_eq!(n, 5);
        assert_eq!(&buf, b"hello");
        assert_eq!(PackSource::len(&data), 11);
    }

    #[test]
    fn pack_sink_over_vec_appends_and_tracks_position() {
        let mut sink: Vec<u8> = Vec::new();
        assert_eq!(sink.position(), 0);
        sink.write_all(b"abc").unwrap();
        assert_eq!(sink.position(), 3);
        sink.write_all(b"def").unwrap();
        assert_eq!(sink.position(), 6);
        assert_eq!(sink.as_slice(), b"abcdef");
        assert!(sink.flush().is_ok());
    }
