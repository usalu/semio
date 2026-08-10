//! ⚙️ BCF engine — ZIP container of parts, reusing the real `stdio.zip` codec for every byte
//! concern, plus a typed `BcfTopic`/`BcfComment` view over each topic folder's `markup.bcf`,
//! parsed/re-emitted via the real `stdio.xml` codec (never a hand-rolled parser here).

use crate::artifacts::bcf::{
    schema::snapshot::{BcfComment, BcfEntry, BcfTopic},
    BcfArtifact, BcfDiff, BcfMutation, BcfSnapshot, STDIO_BCF_DOCUMENT_SCHEMA,
};
use crate::artifacts::xml::schema::snapshot::{xml_document_from_text, xml_document_to_text, XmlAttr, XmlDocument, XmlNode};

fn to_zip(snap: &BcfSnapshot) -> crate::artifacts::zip::ZipSnapshot {
    crate::artifacts::zip::ZipSnapshot {
        schema: crate::artifacts::zip::STDIO_ZIP_DOCUMENT_SCHEMA.into(),
        entries: snap.entries.iter().map(|e| crate::artifacts::zip::schema::snapshot::ZipEntry {
            name: e.name.clone(),
            data: e.data.clone(),
            ..Default::default()
        }).collect(),
        comment: String::new(),
    }
}

fn from_zip(z: crate::artifacts::zip::ZipSnapshot) -> BcfSnapshot {
    BcfSnapshot {
        schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
        entries: z.entries.into_iter().map(|e| BcfEntry { name: e.name, data: e.data }).collect(),
        topics: Vec::new(),
    }
}

//#region 🔖️MarkupXml
/// 🌳️ Narrows an `XmlNode` to its `Element` shape, if it is one.
fn as_element(node: &XmlNode) -> Option<(&str, &[XmlAttr], &[XmlNode])> {
    match node {
        XmlNode::Element { name, attrs, children } => Some((name.as_str(), attrs.as_slice(), children.as_slice())),
        _ => None,
    }
}

/// 🔎️ First direct child element named `name`.
fn find_child<'a>(children: &'a [XmlNode], name: &str) -> Option<&'a XmlNode> {
    children.iter().find(|c| as_element(c).map(|(n, _, _)| n == name).unwrap_or(false))
}

/// 🔎️ All direct child elements named `name`, in document order.
fn find_children<'a>(children: &'a [XmlNode], name: &str) -> Vec<&'a XmlNode> {
    children.iter().filter(|c| as_element(c).map(|(n, _, _)| n == name).unwrap_or(false)).collect()
}

/// 🏷️ Attribute value by name.
fn attr<'a>(attrs: &'a [XmlAttr], name: &str) -> Option<&'a str> {
    attrs.iter().find(|a| a.name == name).map(|a| a.value.as_str())
}

/// 🔤️ Concatenated text/CDATA content of an element's direct children (BCF's leaf elements —
/// `Title`, `Date`, `Author`, `Comment` — are always simple text content, never mixed markup).
fn text_content(node: &XmlNode) -> String {
    let Some((_, _, children)) = as_element(node) else { return String::new() };
    let mut out = String::new();
    for child in children {
        match child {
            XmlNode::Text { text } | XmlNode::CData { text } => out.push_str(text),
            _ => {}
        }
    }
    out
}

/// 🧩️ Parses one topic folder's `markup.bcf` XML bytes into a `BcfTopic` (BCF-XML 2.1
/// `markup.xsd`: root `<Markup>` with a required `<Topic Guid="..." TopicStatus="...">` child
/// carrying a required `<Title>`, zero-or-more sibling `<Comment Guid="...">` elements each with
/// `<Date>`/`<Author>`/`<Comment>`, and zero-or-more `<Viewpoints Viewpoint="...bcfv">`).
fn parse_markup_bcf(data: &[u8]) -> Option<BcfTopic> {
    let text = std::str::from_utf8(data).ok()?;
    let doc = xml_document_from_text(text).ok()?;
    let root = doc.root.as_ref()?;
    let (root_name, _, root_children) = as_element(root)?;
    if root_name != "Markup" {
        return None;
    }
    let topic = find_child(root_children, "Topic")?;
    let (_, topic_attrs, topic_children) = as_element(topic)?;
    let guid = attr(topic_attrs, "Guid").unwrap_or_default().to_string();
    let status = attr(topic_attrs, "TopicStatus").unwrap_or_default().to_string();
    let title = find_child(topic_children, "Title").map(text_content).unwrap_or_default();

    let comments = find_children(root_children, "Comment")
        .into_iter()
        .map(|c| {
            let (_, c_attrs, c_children) = as_element(c).unwrap_or(("Comment", &[], &[]));
            BcfComment {
                guid: attr(c_attrs, "Guid").unwrap_or_default().to_string(),
                date: find_child(c_children, "Date").map(text_content).unwrap_or_default(),
                author: find_child(c_children, "Author").map(text_content).unwrap_or_default(),
                comment: find_child(c_children, "Comment").map(text_content).unwrap_or_default(),
            }
        })
        .collect();

    let viewpoint_ref = find_child(root_children, "Viewpoints")
        .and_then(|v| as_element(v))
        .and_then(|(_, v_attrs, _)| attr(v_attrs, "Viewpoint"))
        .map(|s| s.to_string());

    Some(BcfTopic { guid, title, status, comments, viewpoint_ref })
}

/// 🔤️ Wraps a leaf text element `<name>text</name>` (only emitted when `text` is non-empty,
/// mirroring how real BCF writers omit optional leaf elements rather than emit them empty).
fn text_element(name: &str, text: &str) -> Option<XmlNode> {
    if text.is_empty() {
        return None;
    }
    Some(XmlNode::Element {
        name: name.into(),
        attrs: Vec::new(),
        children: vec![XmlNode::Text { text: text.into() }],
    })
}

/// 🧩️ Re-emits a `BcfTopic` as a full `markup.bcf` XML document (the inverse of
/// `parse_markup_bcf`), via the real `stdio.xml` text codec.
fn markup_bcf_bytes(topic: &BcfTopic) -> Vec<u8> {
    let mut topic_children = Vec::new();
    if let Some(title) = text_element("Title", &topic.title) {
        topic_children.push(title);
    }

    let mut markup_children = vec![XmlNode::Element {
        name: "Topic".into(),
        attrs: vec![
            XmlAttr { name: "Guid".into(), value: topic.guid.clone() },
            XmlAttr { name: "TopicStatus".into(), value: topic.status.clone() },
        ],
        children: topic_children.drain(..).collect(),
    }];

    for comment in &topic.comments {
        let mut children = Vec::new();
        if let Some(n) = text_element("Date", &comment.date) { children.push(n); }
        if let Some(n) = text_element("Author", &comment.author) { children.push(n); }
        if let Some(n) = text_element("Comment", &comment.comment) { children.push(n); }
        markup_children.push(XmlNode::Element {
            name: "Comment".into(),
            attrs: vec![XmlAttr { name: "Guid".into(), value: comment.guid.clone() }],
            children,
        });
    }

    if let Some(viewpoint) = &topic.viewpoint_ref {
        markup_children.push(XmlNode::Element {
            name: "Viewpoints".into(),
            attrs: vec![XmlAttr { name: "Viewpoint".into(), value: viewpoint.clone() }],
            children: Vec::new(),
        });
    }

    let doc = XmlDocument {
        root: Some(XmlNode::Element { name: "Markup".into(), attrs: Vec::new(), children: markup_children }),
        doctype: None,
    };
    let mut out = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    out.push_str(&xml_document_to_text(&doc));
    out.into_bytes()
}

/// 🧬️ Derives the typed `topics` view from the raw `entries` substrate: every entry whose name
/// is `<folder>/markup.bcf` is a topic folder's markup, parsed via the real xml codec. Entries
/// that fail to parse (not valid xml/utf-8, or missing the required `Topic`) are skipped rather
/// than surfaced as an error -- `decode_bcf` never fails outright over an unmodeled/malformed
/// topic folder, it just leaves that folder out of `topics` (still present verbatim in `entries`).
pub fn derive_topics(entries: &[BcfEntry]) -> Vec<BcfTopic> {
    entries
        .iter()
        .filter(|e| e.name.rsplit_once('/').map(|(_, file)| file.eq_ignore_ascii_case("markup.bcf")).unwrap_or(false))
        .filter_map(|e| parse_markup_bcf(&e.data))
        .collect()
}

/// 🔄️ Reconciles `topics` back onto `entries`: for every topic with a non-empty guid, the
/// corresponding `<guid>/markup.bcf` entry is regenerated from the typed fields (overwriting it
/// if present, inserting it if this is a topic that only exists in `topics` so far). Topic
/// folders present only in `entries` (no matching `topics` element) are left untouched.
pub fn apply_topics_to_entries(mut entries: Vec<BcfEntry>, topics: &[BcfTopic]) -> Vec<BcfEntry> {
    for topic in topics {
        if topic.guid.is_empty() {
            continue;
        }
        let entry_name = format!("{}/markup.bcf", topic.guid);
        let data = markup_bcf_bytes(topic);
        match entries.iter_mut().find(|e| e.name == entry_name) {
            Some(existing) => existing.data = data,
            None => entries.push(BcfEntry { name: entry_name, data }),
        }
    }
    entries
}
//#endregion 🔖️MarkupXml

pub fn encode_bcf(snap: &BcfSnapshot) -> Result<Vec<u8>, String> {
    let reconciled = BcfSnapshot {
        schema: snap.schema.clone(),
        entries: apply_topics_to_entries(snap.entries.clone(), &snap.topics),
        topics: snap.topics.clone(),
    };
    crate::artifacts::zip::engine::encode_zip(&to_zip(&reconciled)).map_err(|e| e.to_string())
}

pub fn decode_bcf(data: &[u8]) -> Result<BcfSnapshot, String> {
    let mut snap = from_zip(crate::artifacts::zip::engine::decode_zip(data).map_err(|e| e.to_string())?);
    snap.topics = derive_topics(&snap.entries);
    Ok(snap)
}

pub fn empty_bcf_snapshot() -> BcfSnapshot { BcfSnapshot::default() }

pub fn register() {
    crate::artifacts::bcf::composer::register();
    ::schema::register_artifact_schema_descriptor(crate::artifacts::bcf::schema::bcf_artifact_schema_descriptor());
    store::register_document_codec(store::ArtifactCodec::of::<BcfSnapshot, BcfMutation>(STDIO_BCF_DOCUMENT_SCHEMA));
}

pub struct BcfEngine { artifact_state: BcfArtifact, snapshot_state: BcfSnapshot }
impl BcfEngine {
    pub fn new(snapshot: BcfSnapshot) -> Self {
        Self { artifact_state: BcfArtifact::from_snapshot(snapshot.clone()), snapshot_state: snapshot }
    }
}
impl protocol::ArtifactEngine for BcfEngine {
    type Artifact = BcfArtifact; type Snapshot = BcfSnapshot; type Mutation = BcfMutation; type Diff = BcfDiff;
    fn artifact(&self) -> &Self::Artifact { &self.artifact_state }
    fn snapshot(&self) -> &Self::Snapshot { &self.snapshot_state }
    fn apply(&mut self, mutation: &Self::Mutation) -> Result<Self::Diff, protocol::EngineFault> {
        let diff = <Self::Mutation as protocol::Mutation<Self::Snapshot>>::diff(mutation, &self.snapshot_state);
        self.snapshot_state = <Self::Diff as protocol::MutationDiff<Self::Snapshot>>::apply(&diff, &self.snapshot_state);
        self.artifact_state.set_snapshot(self.snapshot_state.clone());
        Ok(diff)
    }
    fn inverse(&self, mutation: &Self::Mutation) -> Vec<Self::Mutation> {
        <Self::Mutation as protocol::Mutation<Self::Snapshot>>::inverse(mutation, &self.snapshot_state)
    }
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    //#region Fixtures
    /// 🏗️ A real `markup.bcf` document (BCF-XML 2.1 shape): `Topic` with `Guid`/`TopicStatus`
    /// attributes and a `Title` child, one sibling `Comment`, and a `Viewpoints` reference.
    fn sample_markup_xml(guid: &str) -> Vec<u8> {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <Markup><Topic Guid=\"{guid}\" TopicStatus=\"Open\"><Title>Clash on Level 2</Title></Topic>\
             <Comment Guid=\"c1\"><Date>2024-01-01T00:00:00+00:00</Date><Author>ueli@example.com</Author>\
             <Comment>Please review this clash.</Comment></Comment>\
             <Viewpoints Viewpoint=\"viewpoint.bcfv\"/></Markup>"
        ).into_bytes()
    }

    /// 🏗️ A `.bcfv` viewpoint file — deliberately opaque here (only its filename is modeled via
    /// `viewpoint_ref`, never its camera/component content), so any well-formed bytes suffice.
    fn sample_bcfv_xml() -> Vec<u8> {
        b"<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<VisualizationInfo Guid=\"vp-1\"><Components/></VisualizationInfo>".to_vec()
    }

    fn version_entry() -> BcfEntry {
        BcfEntry { name: "bcf.version".into(), data: b"<?xml version=\"1.0\"?><Version VersionId=\"2.1\"/>".to_vec() }
    }
    //#endregion Fixtures

    /// 🧪️ Builds a small real BCF zip in-code (`bcf.version` root entry, one topic folder with
    /// `markup.bcf` + a referenced `.bcfv`), round-trips it through `decode_bcf`, and asserts the
    /// derived `BcfTopic`/`BcfComment` fields match what was encoded — not just that the raw zip
    /// entries survive byte-for-byte (though `decode_rich_synthetic_archive`-style entry survival
    /// is asserted too).
    #[test]
    fn decode_derives_topics_from_markup_xml() {
        let guid = "8e6c1f2e-1111-4a2b-9c3d-000000000001";
        let entries = vec![
            version_entry(),
            BcfEntry { name: format!("{guid}/markup.bcf"), data: sample_markup_xml(guid) },
            BcfEntry { name: format!("{guid}/viewpoint.bcfv"), data: sample_bcfv_xml() },
        ];
        let snap = BcfSnapshot { schema: STDIO_BCF_DOCUMENT_SCHEMA.into(), entries, topics: Vec::new() };
        let bytes = encode_bcf(&snap).expect("encode bcf");

        let decoded = decode_bcf(&bytes).expect("decode bcf");

        // Raw zip substrate survives verbatim.
        assert_eq!(decoded.entries.len(), 3);
        assert!(decoded.entries.iter().any(|e| e.name == "bcf.version"));
        assert!(decoded.entries.iter().any(|e| e.name == format!("{guid}/viewpoint.bcfv")));

        // Typed, derived view matches the xml content exactly.
        assert_eq!(decoded.topics.len(), 1);
        let topic = &decoded.topics[0];
        assert_eq!(topic.guid, guid);
        assert_eq!(topic.title, "Clash on Level 2");
        assert_eq!(topic.status, "Open");
        assert_eq!(topic.viewpoint_ref.as_deref(), Some("viewpoint.bcfv"));
        assert_eq!(topic.comments.len(), 1);
        let comment = &topic.comments[0];
        assert_eq!(comment.guid, "c1");
        assert_eq!(comment.date, "2024-01-01T00:00:00+00:00");
        assert_eq!(comment.author, "ueli@example.com");
        assert_eq!(comment.comment, "Please review this clash.");
    }

    /// 🧪️ A caller mutates `topics` directly without touching `entries` — `encode_bcf` must
    /// regenerate the corresponding `markup.bcf` bytes so the two views never silently diverge
    /// (the mandatory "topics -> entries" direction from the D2 depth requirement).
    #[test]
    fn encode_regenerates_markup_from_mutated_typed_topics() {
        let guid = "8e6c1f2e-2222-4a2b-9c3d-000000000002";
        let base = BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            entries: vec![version_entry(), BcfEntry { name: format!("{guid}/markup.bcf"), data: sample_markup_xml(guid) }],
            topics: Vec::new(),
        };
        let mut snap = decode_bcf(&encode_bcf(&base).unwrap()).unwrap();
        assert_eq!(snap.topics.len(), 1);

        snap.topics[0].title = "Renamed via typed topics".into();
        snap.topics[0].status = "Closed".into();
        snap.topics[0].comments.push(BcfComment {
            guid: "c2".into(),
            date: "2024-02-02T00:00:00+00:00".into(),
            author: "second@example.com".into(),
            comment: "Second comment".into(),
        });

        let re_decoded = decode_bcf(&encode_bcf(&snap).expect("re-encode with mutated typed topics")).expect("decode re-encoded bcf");

        assert_eq!(re_decoded.topics.len(), 1);
        let topic = &re_decoded.topics[0];
        assert_eq!(topic.title, "Renamed via typed topics");
        assert_eq!(topic.status, "Closed");
        assert_eq!(topic.comments.len(), 2);
        assert_eq!(topic.comments[0].comment, "Please review this clash.");
        assert_eq!(topic.comments[1].comment, "Second comment");
    }

    /// 🧪️ A topic that exists only in `topics` (no pre-existing `entries` for it) still gains a
    /// real `<guid>/markup.bcf` entry on encode — the "insert", not just "overwrite", branch of
    /// `apply_topics_to_entries`.
    #[test]
    fn topic_created_purely_via_typed_field_gains_a_markup_entry() {
        let guid = "8e6c1f2e-3333-4a2b-9c3d-000000000003";
        let snap = BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            entries: vec![version_entry()],
            topics: vec![BcfTopic {
                guid: guid.into(),
                title: "Freshly typed topic".into(),
                status: "Open".into(),
                comments: Vec::new(),
                viewpoint_ref: None,
            }],
        };
        let decoded = decode_bcf(&encode_bcf(&snap).expect("encode topic-only-in-typed-field")).expect("decode");
        assert!(decoded.entries.iter().any(|e| e.name == format!("{guid}/markup.bcf")));
        assert_eq!(decoded.topics.len(), 1);
        assert_eq!(decoded.topics[0].title, "Freshly typed topic");
        assert_eq!(decoded.topics[0].status, "Open");
    }

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_bcf_snapshot();
        assert_eq!(snapshot.schema, STDIO_BCF_DOCUMENT_SCHEMA);
        assert!(snapshot.entries.is_empty());
        assert!(snapshot.topics.is_empty());
    }

    #[test]
    fn codec_round_trip() {
        let guid = "8e6c1f2e-4444-4a2b-9c3d-000000000004";
        let snap = BcfSnapshot {
            schema: STDIO_BCF_DOCUMENT_SCHEMA.into(),
            entries: vec![version_entry(), BcfEntry { name: format!("{guid}/markup.bcf"), data: sample_markup_xml(guid) }],
            topics: Vec::new(),
        };
        let snap = decode_bcf(&encode_bcf(&snap).unwrap()).unwrap();

        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <BcfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.entries.len(), snap.entries.len());
        assert_eq!(parsed.topics, snap.topics);

        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <BcfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded.entries.len(), snap.entries.len());
        assert_eq!(decoded.topics, snap.topics);
    }
}
//#endregion 🧪️Tests
