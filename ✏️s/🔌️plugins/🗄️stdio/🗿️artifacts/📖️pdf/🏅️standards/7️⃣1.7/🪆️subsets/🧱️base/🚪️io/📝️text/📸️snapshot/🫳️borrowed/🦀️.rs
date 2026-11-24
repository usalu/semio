//! 🫳️ Projects the native PDF state directly into the authored snapshot Pack fields.

use super::{PdfSnapshot, FIELDS};
use semio_framework_dsl_record::{
    BorrowedFieldSpec, BorrowedRecordSpec, BorrowedShape, RecordLayout,
    native_encoding::{FieldProjectionSource, FieldProjectionView, projection_path_error},
};
use semio_framework_pack_json::{
    ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText, ArtifactCanonicalJsonTree,
};
use semio_framework_value::{Number, ValueError, ValueRefusalKind};

static FIELD_IDS: [u16; FIELDS.len()] = {
    let mut ids = [0; FIELDS.len()];
    let mut index = 0;
    while index < FIELDS.len() {
        ids[index] = FIELDS[index].0;
        index += 1;
    }
    ids
};

static BORROWED_FIELDS: [BorrowedFieldSpec; FIELDS.len()] = {
    let mut fields = [BorrowedFieldSpec::new(0, "", BorrowedShape::Value); FIELDS.len()];
    let mut index = 0;
    while index < FIELDS.len() {
        let (id, key, optional) = FIELDS[index];
        let shape = if id <= 2 { BorrowedShape::Text } else { BorrowedShape::Value };
        let mut field = BorrowedFieldSpec::new(id, key, shape);
        field.optional = optional;
        fields[index] = field;
        index += 1;
    }
    fields
};

/// 🎒️ Shares the same field IDs, declarations, and text layout with the reference snapshot codec.
pub(super) fn spec() -> BorrowedRecordSpec {
    BorrowedRecordSpec { keyword: None, layout: RecordLayout::Lines, fields: &BORROWED_FIELDS }
}

impl FieldProjectionSource for PdfSnapshot {
    fn projection_view(&self, path: &[usize]) -> Result<FieldProjectionView<'_>, ValueError> {
        use FieldProjectionView as View;

        let Some((&slot, path)) = path.split_first() else { return Ok(View::Record(&FIELD_IDS)); };
        match (slot, path) {
            (0, []) => Ok(View::Text(&self.schema)),
            (1, []) => Ok(View::Text(&self.declared_version)),
            (25, []) => Ok(if self.document_id.is_some() { View::IntrinsicArray(2) } else { View::IntrinsicNull }),
            (25, [side]) => {
                let id = self.document_id.as_ref().and_then(|id| id.get(*side)).ok_or_else(projection_path_error)?;
                Ok(View::IntrinsicBytes(id))
            }
            _ => intrinsic_value(field_at(self, slot, path)?),
        }
    }

    fn projection_key(&self, path: &[usize], index: usize) -> Result<&str, ValueError> {
        let (&slot, path) = path.split_first().ok_or_else(projection_path_error)?;
        let ArtifactCanonicalJsonText::Contiguous(key) = field_at(self, slot, path)?.canonical_tree_key(index)? else {
            return Err(unsupported());
        };
        Ok(key)
    }
}

fn field_at<'a>(snapshot: &'a PdfSnapshot, slot: usize, path: &[usize]) -> Result<&'a dyn ArtifactCanonicalJsonTree, ValueError> {
    let mut value: &dyn ArtifactCanonicalJsonTree = match slot {
        2 => &snapshot.pages,
        3 => &snapshot.fonts,
        4 => &snapshot.images,
        5 => &snapshot.forms,
        6 => &snapshot.ext_g_states,
        7 => &snapshot.shadings,
        8 => &snapshot.patterns,
        9 => &snapshot.color_spaces,
        10 => &snapshot.properties,
        11 => &snapshot.outlines,
        12 => &snapshot.named_destinations,
        13 => &snapshot.page_labels,
        14 => &snapshot.embedded_files,
        15 => &snapshot.output_intents,
        16 => &snapshot.acro_form,
        17 => &snapshot.optional_content,
        18 => &snapshot.page_layout,
        19 => &snapshot.page_mode,
        20 => &snapshot.viewer_preferences,
        21 => &snapshot.open_action,
        22 => &snapshot.language,
        23 => &snapshot.mark_info,
        24 => &snapshot.metadata,
        26 => &snapshot.encryption,
        27 => &snapshot.info,
        28 => &snapshot.catalog_extra,
        29 => &snapshot.objects,
        30 => &snapshot.trailer,
        31 => &snapshot.admitted_stream_roles,
        _ => return Err(projection_path_error()),
    };
    for &ordinal in path {
        value = value.canonical_tree_child(ordinal)?;
    }
    Ok(value)
}

fn intrinsic_value(value: &dyn ArtifactCanonicalJsonTree) -> Result<FieldProjectionView<'_>, ValueError> {
    use FieldProjectionView as View;

    if let Some(bytes) = value.canonical_tree_octets() { return Ok(View::IntrinsicBytes(bytes)); }
    if let Some(bytes) = value.canonical_tree_octet_source() { return Ok(View::IntrinsicBytesSource(bytes)); }
    Ok(match value.canonical_tree_node()? {
        Node::Null => View::IntrinsicNull,
        Node::Bool(value) => View::IntrinsicBool(value),
        Node::I64(value) => View::IntrinsicNumber(Number::Int(value)),
        Node::U64(value) => View::IntrinsicNumber(Number::UInt(value)),
        Node::F32(value) => View::IntrinsicNumber(Number::Float(f64::from(value))),
        Node::F64(value) => View::IntrinsicNumber(Number::Float(value)),
        Node::String(value) | Node::Text(ArtifactCanonicalJsonText::Contiguous(value)) => View::IntrinsicText(value),
        Node::Array(length) => View::IntrinsicArray(length),
        Node::Object(length) => View::IntrinsicObject(length),
        _ => return Err(unsupported()),
    })
}

fn unsupported() -> ValueError {
    ValueError::literal(ValueRefusalKind::UnsupportedOwner, "PDF snapshot field has no native Pack projection")
}
