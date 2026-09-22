//#region 🧬️TestDocumentMutationRoot
//! 🧪️ Shared document state and direct mutation fixtures for the Plugin contract.

use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use store::ArtifactPack;

//#region 🧫️Snapshot
pub(crate) const MAXIMUM_CHILD_PROBE_BYTES: usize = 4 * 1024 * 1024;
pub(crate) static MAXIMUM_CHILD_CLONES: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
pub(crate) static MAXIMUM_CHILD_ENCODINGS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[derive(Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, dsl::DslArtifact)]
#[dsl(extension = "testkit-macro")]
pub(crate) struct TestSnapshot {
    pub(crate) count: i32,
    pub(crate) label: String,
    /// 🧒️ The members declared in this document's one owned-child slot, `"slot"` — the field a
    /// schema-derived snapshot spells `#[child(kind = "s.test.child")]`. `store::ArtifactChild`
    /// carries `ToValue`/`FromValue`/`DslField` but no serde impl, so the derived serde half skips
    /// it and the hand-written pack/DSL codecs below carry it as its member's canonical
    /// `ArtifactRef` uri.
    #[serde(skip)]
    pub(crate) slot: Vec<store::ArtifactChild<TestSnapshot>>,
}

impl TestSnapshot {
    /// 🧩️ The JSON carriage both hand-written codecs share. An undeclared slot writes no key at
    /// all, so a document with no child encodes exactly the bytes it always did.
    fn to_json(&self) -> Result<serde_json::Value, String> {
        let mut value = serde_json::to_value(self).map_err(|error| error.to_string())?;
        if !self.slot.is_empty() {
            let object = value.as_object_mut().ok_or_else(|| "test snapshot encodes as a json object".to_string())?;
            object.insert("slot".into(), serde_json::Value::Array(self.slot.iter().map(|child| serde_json::Value::String(child.target.to_uri())).collect()));
        }
        Ok(value)
    }

    /// 🧩️ Inverse of {@link TestSnapshot::to_json}: every declared row is one `ArtifactRef` uri,
    /// and a child's id IS its artifact id (what `ChildRestoreProjection` admits).
    fn from_json(mut value: serde_json::Value) -> Result<Self, String> {
        let declared = value.as_object_mut().and_then(|object| object.remove("slot"));
        let mut snapshot: Self = serde_json::from_value(value).map_err(|error| error.to_string())?;
        let Some(declared) = declared else { return Ok(snapshot) };
        for row in declared.as_array().ok_or_else(|| "declared child slot is an array of artifact ref uris".to_string())? {
            snapshot.slot.push(test_child_handle(row.as_str().ok_or_else(|| "declared child is an artifact ref uri".to_string())?)?);
        }
        Ok(snapshot)
    }
}

/// 🧒️ One declared owned-child handle from its canonical `ArtifactRef` uri.
pub(crate) fn test_child_handle(uri: &str) -> Result<store::ArtifactChild<TestSnapshot>, String> {
    let target = store::os_io::ArtifactRef::parse_uri(uri)?;
    Ok(store::ArtifactChild::new(target.artifact_id.clone(), target))
}

/// ♻️ The fixture child is an openable member, so its snapshot needs the same bounded owned-value
/// retirement a real member's does — `store::PackMemberSnapshotOpen` retires the decoded snapshot
/// through it when an open is cancelled or rejected mid-flight.
impl crate::store::retirement::RetireOwned for TestSnapshot {
    fn retirement(self) -> Box<dyn crate::store::retirement::RetirementCursor> {
        let Self { count, label, slot } = self;
        crate::store::retirement::sequence(vec![
            crate::store::retirement::RetireOwned::retirement(count),
            crate::store::retirement::RetireOwned::retirement(label),
            crate::store::retirement::RetireOwned::retirement(slot),
        ])
    }
}

impl semio_framework_schema::ArtifactCompositionFields for TestSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema::ChildRefVisitor<'a>>(&'a self, visitor: &mut V) -> Result<(), V::Error> {
        semio_framework_schema::ChildFieldRefs::visit_child_field(&self.slot, "slot", visitor)
    }

    fn child_slots() -> &'static [semio_framework_schema::ChildSlotSpec] {
        &[semio_framework_schema::ChildSlotSpec { name: "slot", kind: "s.test.child", many: true }]
    }
}

impl Clone for TestSnapshot {
    fn clone(&self) -> Self {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_CLONES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        Self { count: self.count, label: self.label.clone(), slot: self.slot.clone() }
    }
}

impl store::ArtifactDsl for TestSnapshot {
    const EXTENSION: &'static str = "testkit-macro";
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value = serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))?;
        Self::from_json(value).map_err(|error| store::TextError::new(error, store::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        self.to_json().and_then(|value| serde_json::to_string(&value).map_err(|error| error.to_string())).unwrap_or_default()
    }
}

impl ArtifactPack for TestSnapshot {
    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        if self.label.len() >= MAXIMUM_CHILD_PROBE_BYTES {
            MAXIMUM_CHILD_ENCODINGS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }
        let value = self.to_json().map_err(store::PackError::Schema)?;
        serde_json::to_vec(&value).map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        let value: serde_json::Value = serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Self::from_json(value).map_err(store::PackError::Schema)
    }
}
//#endregion 🧫️Snapshot

//#region 🔺️Diff
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct TestDiff {
    pub(crate) count: Option<i32>,
    pub(crate) label: Option<String>,
    /// 🧒️ The whole declared membership of the `"slot"` child slot, as canonical `ArtifactRef`
    /// uris — a child declaration is replaced wholesale, never merged row by row.
    pub(crate) slot: Option<Vec<String>>,
}

impl protocol::MutationDiff<TestSnapshot> for TestDiff {
    fn apply(&self, snapshot: &TestSnapshot) -> protocol::MutationApplyResult<TestSnapshot> {
        let slot = match &self.slot {
            None => snapshot.slot.clone(),
            Some(rows) => rows.iter().map(|uri| test_child_handle(uri)).collect::<Result<Vec<_>, String>>().map_err(|error| protocol::MutationApplyError::new("test-snapshot.declared-child-uri", error))?,
        };
        Ok(TestSnapshot { count: self.count.unwrap_or(snapshot.count), label: self.label.clone().unwrap_or_else(|| snapshot.label.clone()), slot })
    }

    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
        if other.label.is_some() {
            self.label = other.label;
        }
        if other.slot.is_some() {
            self.slot = other.slot;
        }
    }
}
//#endregion 🔺️Diff

//#region 🧬️Mutations
#[path = "../../🧫️fixtures/🖥️test-app-mutations/🧬️document/🧬️mutations/🦀️.rs"]
pub mod mutations;
pub(crate) use mutations::{SetSlotChildren, SetCount, SetLabel, TestMutation};
//#endregion 🧬️Mutations
//#endregion 🧬️TestDocumentMutationRoot
