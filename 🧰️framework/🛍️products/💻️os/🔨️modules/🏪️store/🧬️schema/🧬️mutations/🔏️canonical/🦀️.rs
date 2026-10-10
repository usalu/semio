//! 🔏️ Borrowed exact typed space-history mutation traversal: one indexed navigator serves operation identity, canonical sealing and the borrowed root.
use super::{CommitSpaceCheckpoint, CreateSpaceAlternative, RemoveSpaceAlternative, RemoveSpaceCheckpoint, SetActiveSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};
use super::super::{SpaceAlternative, SpaceCheckpoint, SpaceMemberPin};
use crate::os_spr::HybridLogicalTimestamp;
use crate::os_store::{ArtifactCanonicalJson, ArtifactCanonicalJsonArray as A, ArtifactCanonicalJsonNode as N, ArtifactCanonicalJsonObject as O, ArtifactCanonicalJsonText as T, ArtifactCanonicalJsonValue as V};
use crate::os_vcs::Author;

//#region 🧭️Navigator
type Entries<'a> = [Option<(&'static str, Child<'a>)>; 6];

fn invalid() -> String {
    "canonical-edit.invalid-typed-path".into()
}

/// 🧭️ One original position inside a space-history mutation; every child is a borrow of the owner, never a copy.
#[derive(Clone, Copy)]
enum Child<'a> {
    Null,
    UInt(u64),
    Text(&'a str),
    Record(&'a dyn Record),
    List(&'a dyn Sequence),
}

/// 🗂️ A fixed-field object in its exact serde order; absent optional fields are `None` and take no position.
trait Record: Sync {
    fn entries(&self) -> Entries<'_>;

    fn length(&self) -> usize {
        self.entries().into_iter().flatten().count()
    }

    fn entry(&self, index: usize) -> Result<(&'static str, Child<'_>), String> {
        self.entries().into_iter().flatten().nth(index).ok_or_else(invalid)
    }
}

/// 📚️ An ordered sequence with bounded indexed access.
trait Sequence: Sync {
    fn length(&self) -> usize;
    fn item(&self, index: usize) -> Option<Child<'_>>;
}

impl<'a> Child<'a> {
    fn node(self, path: &[usize]) -> Result<N<'a>, String> {
        match (self, path) {
            (Self::Null, []) => Ok(N::Null),
            (Self::UInt(value), []) => Ok(N::U64(value)),
            (Self::Text(value), []) => Ok(N::String(value)),
            (Self::Record(record), []) => Ok(N::Object(record.length())),
            (Self::Record(record), [index, rest @ ..]) => record.entry(*index)?.1.node(rest),
            (Self::List(list), []) => Ok(N::Array(list.length())),
            (Self::List(list), [index, rest @ ..]) => list.item(*index).ok_or_else(invalid)?.node(rest),
            _ => Err(invalid()),
        }
    }

    fn key(self, path: &[usize], index: usize) -> Result<T<'a>, String> {
        match (self, path) {
            (Self::Record(record), []) => Ok(T::from(record.entry(index)?.0)),
            (Self::Record(record), [head, rest @ ..]) => record.entry(*head)?.1.key(rest, index),
            (Self::List(list), [head, rest @ ..]) => list.item(*head).ok_or_else(invalid)?.key(rest, index),
            _ => Err(invalid()),
        }
    }

    fn borrowed(self) -> V<'a> {
        match self {
            Self::Record(record) => V::Object(O::new((0..record.length()).map(move |index| {
                let (key, child) = record.entry(index).expect("space history record entry");
                (key, child.borrowed())
            }))),
            Self::List(list) => V::Array(A::new((0..list.length()).map(move |index| list.item(index).expect("space history list item").borrowed()))),
            scalar => V::Scalar(scalar.node(&[]).expect("space history scalar")),
        }
    }
}
//#endregion 🧭️Navigator

//#region 🧬️Records
impl Sequence for Vec<String> {
    fn length(&self) -> usize {
        self.len()
    }

    fn item(&self, index: usize) -> Option<Child<'_>> {
        self.get(index).map(|value| Child::Text(value))
    }
}

impl Sequence for Vec<Author> {
    fn length(&self) -> usize {
        self.len()
    }

    fn item(&self, index: usize) -> Option<Child<'_>> {
        self.get(index).map(|value| Child::Record(value))
    }
}

impl Sequence for Vec<SpaceMemberPin> {
    fn length(&self) -> usize {
        self.len()
    }

    fn item(&self, index: usize) -> Option<Child<'_>> {
        self.get(index).map(|value| Child::Record(value))
    }
}

impl Record for Author {
    fn entries(&self) -> Entries<'_> {
        [Some(("id", Child::Text(&self.id))), Some(("name", Child::Text(&self.name))), self.avatar.as_deref().map(|avatar| ("avatar", Child::Text(avatar))), None, None, None]
    }
}

impl Record for HybridLogicalTimestamp {
    fn entries(&self) -> Entries<'_> {
        [Some(("actor", Child::UInt(self.actor))), Some(("physical_ms", Child::UInt(self.physical_ms))), Some(("logical", Child::UInt(self.logical))), None, None, None]
    }
}

impl Record for SpaceMemberPin {
    fn entries(&self) -> Entries<'_> {
        [Some(("documentId", Child::Text(&self.document_id))), Some(("checkpointId", Child::Text(&self.checkpoint_id))), Some(("alternativeId", Child::Text(&self.alternative_id))), None, None, None]
    }
}

impl Record for SpaceCheckpoint {
    fn entries(&self) -> Entries<'_> {
        [
            Some(("id", Child::Text(&self.id))),
            self.parent_id.as_deref().map(|parent| ("parentId", Child::Text(parent))),
            Some(("message", Child::Text(&self.message))),
            Some(("authors", Child::List(&self.authors))),
            Some(("timestamp", Child::Record(&self.timestamp))),
            Some(("members", Child::List(&self.members))),
        ]
    }
}

impl Record for SpaceAlternative {
    fn entries(&self) -> Entries<'_> {
        [Some(("id", Child::Text(&self.id))), Some(("name", Child::Text(&self.name))), Some(("checkpointIds", Child::List(&self.checkpoint_ids))), None, None, None]
    }
}

impl Record for CommitSpaceCheckpoint {
    fn entries(&self) -> Entries<'_> {
        [Some(("checkpoint", Child::Record(&self.checkpoint))), None, None, None, None, None]
    }
}

impl Record for CreateSpaceAlternative {
    fn entries(&self) -> Entries<'_> {
        [Some(("alternative", Child::Record(&self.alternative))), None, None, None, None, None]
    }
}

impl Record for SwitchSpaceAlternative {
    fn entries(&self) -> Entries<'_> {
        [Some(("alternativeId", Child::Text(&self.alternative_id))), None, None, None, None, None]
    }
}

impl Record for RemoveSpaceCheckpoint {
    fn entries(&self) -> Entries<'_> {
        [Some(("checkpointId", Child::Text(&self.checkpoint_id))), None, None, None, None, None]
    }
}

impl Record for RemoveSpaceAlternative {
    fn entries(&self) -> Entries<'_> {
        [Some(("alternativeId", Child::Text(&self.alternative_id))), None, None, None, None, None]
    }
}

impl Record for SetActiveSpaceAlternative {
    fn entries(&self) -> Entries<'_> {
        [Some(("alternativeId", self.alternative_id.as_deref().map_or(Child::Null, Child::Text))), None, None, None, None, None]
    }
}

impl Record for SpaceHistoryMutation {
    fn entries(&self) -> Entries<'_> {
        let (operation, payload): (&'static str, Child<'_>) = match self {
            Self::CommitSpaceCheckpoint(payload) => ("commitSpaceCheckpoint", Child::Record(payload)),
            Self::CreateSpaceAlternative(payload) => ("createSpaceAlternative", Child::Record(payload)),
            Self::SwitchSpaceAlternative(payload) => ("switchSpaceAlternative", Child::Record(payload)),
            Self::RemoveSpaceCheckpoint(payload) => ("removeSpaceCheckpoint", Child::Record(payload)),
            Self::RemoveSpaceAlternative(payload) => ("removeSpaceAlternative", Child::Record(payload)),
            Self::SetActiveSpaceAlternative(payload) => ("setActiveSpaceAlternative", Child::Record(payload)),
        };
        [Some(("operation", Child::Text(operation))), Some(("payload", payload)), None, None, None, None]
    }
}
//#endregion 🧬️Records

impl ArtifactCanonicalJson for SpaceHistoryMutation {
    fn canonical_json_node(&self, path: &[usize]) -> Result<N<'_>, String> {
        Child::Record(self).node(path)
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<T<'_>, String> {
        Child::Record(self).key(object_path, index)
    }

    fn canonical_json_borrowed_root(&self) -> Result<Option<V<'_>>, String> {
        Ok(Some(Child::Record(self).borrowed()))
    }
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
