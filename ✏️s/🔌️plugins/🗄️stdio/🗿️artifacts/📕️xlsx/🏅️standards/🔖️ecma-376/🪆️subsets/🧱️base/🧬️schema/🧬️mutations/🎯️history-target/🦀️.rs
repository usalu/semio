//! 🎯️ Borrowed conflict paths for the XLSX mutation vocabulary.

use super::XlsxMutation;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::io::Write;
use store::ArtifactOperationTargets;

struct ConflictPath<'a> {
    prefix: [&'a str; 4],
    indices: &'a [usize],
    vacancy: Option<(usize, usize)>,
}

impl<'a> ConflictPath<'a> {
    fn worksheet(part: &'a str, indices: &'a [usize], vacancy: Option<(usize, usize)>) -> Self {
        Self { prefix: ["xmlParts", part, "document", "root"], indices, vacancy }
    }

    fn package(prefix: [&'a str; 4]) -> Self {
        Self { prefix, indices: &[], vacancy: None }
    }

    fn len(&self) -> usize {
        self.prefix.len() + self.indices.len() + self.vacancy.map_or(0, |_| 2)
    }

    fn with_segment<T>(&self, index: usize, read: impl FnOnce(&[u8]) -> Result<T, ValueError>) -> Result<T, ValueError> {
        if let Some(segment) = self.prefix.get(index) {
            return read(segment.as_bytes());
        }
        let index = index - self.prefix.len();
        let value = if let Some(value) = self.indices.get(index) {
            *value
        } else {
            match (self.vacancy, index - self.indices.len()) {
                (Some((row, _)), 0) => row,
                (Some((_, column)), 1) => column,
                _ => return Err(absent_segment()),
            }
        };
        let mut scratch = [0; 3 * size_of::<usize>()];
        let mut writer = std::io::Cursor::new(&mut scratch[..]);
        write!(writer, "{value}").map_err(|_| ValueError::invariant("XLSX conflict coordinate exceeded its fixed scratch"))?;
        let length = writer.position() as usize;
        read(&scratch[..length])
    }
}

fn absent_segment() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "XLSX conflict path segment is absent")
}

impl XlsxMutation {
    fn conflict_path(&self) -> Option<ConflictPath<'_>> {
        use XlsxMutation::*;
        Some(match self {
            SetCell(operation) => ConflictPath::worksheet(&operation.address.part_path, &operation.address.node_path, None),
            RemoveCell(operation) => ConflictPath::worksheet(&operation.address.part_path, &operation.address.node_path, None),
            InsertCell(operation) => {
                let address = &operation.address;
                ConflictPath::worksheet(&address.worksheet.part_path, &address.worksheet.node_path, Some((address.row as usize, address.column as usize)))
            }
            SetRelationship(operation) => ConflictPath::package(["opc", "relationships", &operation.owner, &operation.id]),
            RemoveRelationship(operation) => ConflictPath::package(["opc", "relationships", &operation.owner, &operation.id]),
            SetContentType(operation) => ConflictPath::package(["opc", "contentTypes", content_type_list(operation.is_override), &operation.name]),
            RemoveContentType(operation) => ConflictPath::package(["opc", "contentTypes", content_type_list(operation.is_override), &operation.name]),
            InsertSheet(_) | RemoveSheet(_) | RenameSheet(_) | InsertSharedString(_) | RemoveSharedString(_) | SetSharedString(_) => return None,
        })
    }

    /// 🧭️ Lends the original mutation's conflict path to the document store.
    pub fn original_conflict_target(&self) -> Option<&dyn ArtifactOperationTargets> {
        self.conflict_path().map(|_| self as &dyn ArtifactOperationTargets)
    }
}

fn content_type_list(is_override: bool) -> &'static str {
    if is_override { "overrides" } else { "defaults" }
}

impl ArtifactOperationTargets for XlsxMutation {
    fn len(&self) -> usize {
        self.conflict_path().map_or(0, |path| path.len())
    }

    fn segment_len(&self, index: usize) -> Result<usize, ValueError> {
        self.conflict_path().ok_or_else(absent_segment)?.with_segment(index, |segment| Ok(segment.len()))
    }

    fn read_segment(&self, index: usize, offset: usize, output: &mut [u8]) -> Result<usize, ValueError> {
        self.conflict_path().ok_or_else(absent_segment)?.with_segment(index, |segment| {
            let rest = segment.get(offset..).ok_or_else(absent_segment)?;
            let copied = rest.len().min(output.len());
            output[..copied].copy_from_slice(&rest[..copied]);
            Ok(copied)
        })
    }
}
