//! 🧵️ Retained archive target resolution over one bounded member name per turn.

use super::{edit_node, entry_node_id, fault, require_revision, validate_text, COMMENT_NODE_ID, ENTRY_NODE_PREFIX};
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactEditor, EditorApp, Emit, Fault, NoConfigMutation, NoDraftMutation};
use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};

pub type Arguments<C> = for<'a> fn(&'a C) -> Result<(&'a str, &'a str, &'a str), Fault>;

#[derive(Default)]
pub struct ArchiveTextCursor {
    cursor: usize,
    matched: Option<usize>,
    collision: bool,
    complete: bool,
}

impl ArchiveTextCursor {
    pub fn scanned_entries(&self) -> usize {
        self.cursor
    }

    pub fn advance(&mut self, snapshot: &ZipSnapshot, node_id: &str, value: &str, revision: &str) -> Result<Option<Emit<ZipMutation>>, Fault> {
        if self.complete {
            return Err(fault("stdio.zip.work-complete", "archive target resolution has already completed"));
        }
        validate_text(value)?;
        if node_id == COMMENT_NODE_ID {
            validate_text(&snapshot.comment)?;
            self.complete = true;
            return edit_node(snapshot, node_id, value, revision).map(Some);
        }
        if !node_id.starts_with(ENTRY_NODE_PREFIX) {
            return Err(fault("stdio.zip.target-missing", "the archive edit target no longer exists"));
        }
        if let Some(entry) = snapshot.entries.get(self.cursor) {
            validate_text(&entry.name)?;
            if entry_node_id(&entry.name) == node_id {
                if self.matched.is_some() {
                    return Err(fault("stdio.zip.target-ambiguous", "entries with identical names must be disambiguated before renaming"));
                }
                self.matched = Some(self.cursor);
            }
            self.collision |= entry.name == value;
            self.cursor += 1;
            return Ok(None);
        }
        let index = self.matched.ok_or_else(|| fault("stdio.zip.target-missing", "the archive entry changed or no longer exists"))?;
        let entry = &snapshot.entries[index];
        require_revision(&entry.name, revision)?;
        if value.is_empty() {
            return Err(fault("stdio.zip.name-required", "an archive entry requires a non-empty name"));
        }
        self.complete = true;
        if entry.name == value {
            return Ok(Some(Emit::default()));
        }
        if self.collision {
            return Err(fault("stdio.zip.name-exists", "another archive entry already has the requested name"));
        }
        Ok(Some(Emit {
            artifact_mutations: vec![ZipMutation::RenameEntry(crate::schema::mutations::rename_entry::RenameEntry { name: entry.name.clone(), new_name: value.into() })],
            description: Some("Rename archive entry".into()),
            ..Default::default()
        }))
    }
}

pub struct ArchiveTextWork<E: ArtifactEditor> {
    arguments: Arguments<E::Command>,
    cursor: ArchiveTextCursor,
    identity: u64,
    marker: PhantomData<fn() -> E>,
}

impl<E: ArtifactEditor> ArchiveTextWork<E> {
    pub fn new(arguments: Arguments<E::Command>) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self { arguments, cursor: ArchiveTextCursor::default(), identity: NEXT_ID.fetch_add(1, Ordering::Relaxed), marker: PhantomData }
    }
}

impl<E> ArtifactCommandWork<EditorApp<E>> for ArchiveTextWork<E>
where
    E: ArtifactEditor<Snapshot = ZipSnapshot, Mutation = ZipMutation, ConfigMutation = NoConfigMutation, DraftMutation = NoDraftMutation>,
{
    fn tool_id(&self) -> &'static str {
        "set-node"
    }

    fn workspace_identity(&self) -> u64 {
        self.identity
    }

    fn extent(&self, command: &E::Command, _snapshot: &ZipSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>) -> Option<usize> {
        let (_, value, _) = (self.arguments)(command).ok()?;
        validate_text(value).ok()?;
        Some(2)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<E>>) -> Result<ArtifactCommandWorkStep<EditorApp<E>>, Fault> {
        let (node_id, value, revision) = (self.arguments)(input.command)?;
        match self.cursor.advance(input.snapshot, node_id, value, revision)? {
            Some(emit) => Ok(ArtifactCommandWorkStep::Complete(emit)),
            None => Ok(ArtifactCommandWorkStep::Replay { stage: "zip-resolve-entry", preview: r#"{"en":"Checking archive entries","de":"Archiveinträge werden geprüft"}"#.as_bytes() }),
        }
    }

    fn begin_close(&mut self) {
        self.cursor.complete = true;
    }
}
