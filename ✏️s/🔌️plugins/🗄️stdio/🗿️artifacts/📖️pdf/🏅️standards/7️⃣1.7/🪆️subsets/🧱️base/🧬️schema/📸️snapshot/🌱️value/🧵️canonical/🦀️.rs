//! 🧵️ Hand-projected canonical trees for the COS shapes whose wire is not a plain field-role record.
use super::*;
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node, ArtifactCanonicalJsonText as Text, ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError, ValueRefusalKind};

type Field<'a> = Option<(&'static str, &'a dyn Tree)>;

fn field<'a, T: Tree>(name: &'static str, value: &'a T) -> Field<'a> {
    Some((name, value as &dyn Tree))
}

fn absent() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical COS ordinal is absent")
}

fn present<'a, 'b, const N: usize>(fields: &'b [Field<'a>; N]) -> impl Iterator<Item = (&'static str, &'a dyn Tree)> + 'b
where
    'a: 'b,
{
    fields.iter().flatten().copied()
}

fn node<const N: usize>(fields: &[Field<'_>; N], tagged: bool) -> Result<Node<'static>, ValueError> {
    Ok(Node::Object(present(fields).count() + usize::from(tagged)))
}

fn child<'a, const N: usize>(fields: &[Field<'a>; N], kind: Option<&'a &'static str>, ordinal: usize) -> Result<&'a dyn Tree, ValueError> {
    match (kind, ordinal) {
        (Some(kind), 0) => Ok(kind),
        (Some(_), ordinal) => present(fields).nth(ordinal - 1).map(|(_, value)| value).ok_or_else(absent),
        (None, ordinal) => present(fields).nth(ordinal).map(|(_, value)| value).ok_or_else(absent),
    }
}

fn key<const N: usize>(fields: &[Field<'_>; N], tagged: bool, ordinal: usize) -> Result<Text<'static>, ValueError> {
    match (tagged, ordinal) {
        (true, 0) => Ok("kind".into()),
        (true, ordinal) => present(fields).nth(ordinal - 1).map(|(name, _)| name.into()).ok_or_else(absent),
        (false, ordinal) => present(fields).nth(ordinal).map(|(name, _)| name.into()).ok_or_else(absent),
    }
}

static COS_KINDS: [&str; 12] = ["null", "bool", "int", "real", "str", "text", "date", "name", "array", "dict", "ref", "stream"];

impl PdfObject {
    fn canonical_parts(&self) -> (&'static &'static str, [Field<'_>; 4]) {
        match self {
            Self::Null => (&COS_KINDS[0], [None, None, None, None]),
            Self::Bool(value) => (&COS_KINDS[1], [field("value", value), None, None, None]),
            Self::Int(value) => (&COS_KINDS[2], [field("value", value), None, None, None]),
            Self::Real(value) => (&COS_KINDS[3], [field("negative", &value.negative), field("coefficient", &value.coefficient), field("scale", &value.scale), None]),
            Self::Str(value) => (&COS_KINDS[4], [field("value", value), None, None, None]),
            Self::Text(value) => (&COS_KINDS[5], [field("value", value), None, None, None]),
            Self::Date(value) => (&COS_KINDS[6], [field("value", value), None, None, None]),
            Self::Name(value) => (&COS_KINDS[7], [field("value", value), None, None, None]),
            Self::Array(value) => (&COS_KINDS[8], [field("value", value), None, None, None]),
            Self::Dict(value) => (&COS_KINDS[9], [field("value", value), None, None, None]),
            Self::Ref(value) => (&COS_KINDS[10], [field("num", &value.num), field("gen", &value.gen), None, None]),
            Self::Stream { dict, data, filters } => (&COS_KINDS[11], [field("dict", dict), field("data", data), field("filters", filters), None]),
        }
    }
}

impl Tree for PdfObject {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        node(&self.canonical_parts().1, true)
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        let (kind, fields) = self.canonical_parts();
        child(&fields, Some(kind), ordinal)
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        key(&self.canonical_parts().1, true, ordinal)
    }
}

static FILTER_KINDS: [&str; 10] = ["flate", "lzw", "asciiHex", "ascii85", "runLength", "dct", "jpx", "ccitt", "jbig2", "crypt"];

impl PdfStreamFilter {
    fn canonical_parts(&self) -> (&'static &'static str, [Field<'_>; 2]) {
        match self {
            Self::Flate { predictor } => (&FILTER_KINDS[0], [field("predictor", predictor), None]),
            Self::Lzw { predictor, early_change } => (&FILTER_KINDS[1], [field("predictor", predictor), field("earlyChange", early_change)]),
            Self::AsciiHex => (&FILTER_KINDS[2], [None, None]),
            Self::Ascii85 => (&FILTER_KINDS[3], [None, None]),
            Self::RunLength => (&FILTER_KINDS[4], [None, None]),
            Self::Dct { color_transform } => (&FILTER_KINDS[5], [field("colorTransform", color_transform), None]),
            Self::Jpx => (&FILTER_KINDS[6], [None, None]),
            Self::Ccitt { parameters } => (&FILTER_KINDS[7], [field("parameters", parameters), None]),
            Self::Jbig2 { globals } => (&FILTER_KINDS[8], [field("globals", globals), None]),
            Self::Crypt { name } => (&FILTER_KINDS[9], [field("name", name), None]),
        }
    }
}

impl Tree for PdfStreamFilter {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        node(&self.canonical_parts().1, true)
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        let (kind, fields) = self.canonical_parts();
        child(&fields, Some(kind), ordinal)
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        key(&self.canonical_parts().1, true, ordinal)
    }
}

impl PdfEmbeddedFile {
    fn canonical_parts(&self) -> [Field<'_>; 9] {
        [
            field("id", &self.id),
            field("fileName", &self.file_name),
            self.description.as_ref().map(|value| ("description", value as &dyn Tree)),
            self.mime_type.as_ref().map(|value| ("mimeType", value as &dyn Tree)),
            field("data", &self.data),
            self.creation_date.as_ref().map(|value| ("creationDate", value as &dyn Tree)),
            self.modification_date.as_ref().map(|value| ("modificationDate", value as &dyn Tree)),
            self.relationship.as_ref().map(|value| ("relationship", value as &dyn Tree)),
            field("listed", &self.listed),
        ]
    }
}

impl Tree for PdfEmbeddedFile {
    fn canonical_tree_node(&self) -> Result<Node<'_>, ValueError> {
        node(&self.canonical_parts(), false)
    }
    fn canonical_tree_child(&self, ordinal: usize) -> Result<&dyn Tree, ValueError> {
        child(&self.canonical_parts(), None, ordinal)
    }
    fn canonical_tree_key(&self, ordinal: usize) -> Result<Text<'_>, ValueError> {
        key(&self.canonical_parts(), false, ordinal)
    }
}
