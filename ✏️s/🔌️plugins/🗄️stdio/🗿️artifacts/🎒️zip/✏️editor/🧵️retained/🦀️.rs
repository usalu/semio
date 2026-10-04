//! 🧵️ Retained archive target resolution over one bounded member name per turn.

use super::{edit_node, entry_node_id, fault, require_revision, validate_text, COMMENT_NODE_ID, ENTRY_NODE_PREFIX};
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_plugin::retained_command::{ArtifactCommandInputs, ArtifactCommandWork, ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactEditor, EditorApp, Emit, Fault, NoConfigMutation, NoDraftMutation};
use std::marker::PhantomData;

pub const CHECKPOINT_BYTES: usize = 96;
const WORKSPACE_IDENTITY: u64 = u64::from_le_bytes(*b"ZIPTEXT1");

pub type Arguments<C> = for<'a> fn(&'a C) -> Result<(&'a str, &'a str, &'a str), Fault>;

#[derive(Default)]
pub struct ArchiveTextCursor {
    cursor: usize,
    matched: Option<usize>,
    collision: bool,
    complete: bool,
    binding: Option<([u8; 64], usize)>,
}

impl ArchiveTextCursor {
    pub fn scanned_entries(&self) -> usize {
        self.cursor
    }

    pub fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        let bytes = target.get_mut(..CHECKPOINT_BYTES).ok_or_else(|| fault("stdio.zip.checkpoint-capacity", "archive cursor checkpoint requires 96 bytes"))?;
        bytes.fill(0);
        bytes[..4].copy_from_slice(b"ZAT1");
        bytes[4] = 1;
        bytes[5] = u8::from(self.matched.is_some()) | u8::from(self.collision) << 1 | u8::from(self.complete) << 2 | u8::from(self.binding.is_some()) << 3;
        bytes[8..16].copy_from_slice(&(self.cursor as u64).to_le_bytes());
        bytes[16..24].copy_from_slice(&(self.matched.unwrap_or(0) as u64).to_le_bytes());
        if let Some((digest, entries)) = self.binding {
            bytes[24..32].copy_from_slice(&(entries as u64).to_le_bytes());
            bytes[32..].copy_from_slice(&digest);
        }
        Ok(CHECKPOINT_BYTES)
    }

    pub fn restore(&mut self, bytes: &[u8]) -> Result<(), Fault> {
        let invalid = || fault("stdio.zip.checkpoint-invalid", "archive cursor checkpoint is malformed");
        if bytes.len() != CHECKPOINT_BYTES || &bytes[..4] != b"ZAT1" || bytes[4] != 1 || bytes[5] > 15 || bytes[6..8] != [0, 0] {
            return Err(invalid());
        }
        let index = |offset| -> Result<usize, Fault> {
            let number = u64::from_le_bytes(bytes[offset..offset + 8].try_into().map_err(|_| invalid())?);
            if number > 9_007_199_254_740_991 {
                return Err(invalid());
            }
            usize::try_from(number).map_err(|_| invalid())
        };
        let cursor = index(8)?;
        let matched = index(16)?;
        let entries = index(24)?;
        let flags = bytes[5];
        let bound = flags & 8 != 0;
        if cursor > entries
            || (flags & 1 != 0 && matched >= cursor)
            || (flags & 1 == 0 && matched != 0)
            || (!bound && (flags != 0 || cursor != 0 || entries != 0 || bytes[32..].iter().any(|byte| *byte != 0)))
            || (bound && !bytes[32..].iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte)))
        {
            return Err(invalid());
        }
        *self = Self { cursor, matched: (flags & 1 != 0).then_some(matched), collision: flags & 2 != 0, complete: flags & 4 != 0, binding: bound.then(|| (bytes[32..].try_into().expect("checked 64-byte digest"), entries)) };
        Ok(())
    }

    pub fn advance(&mut self, snapshot: &ZipSnapshot, node_id: &str, value: &str, revision: &str) -> Result<Option<Emit<ZipMutation>>, Fault> {
        self.bind(snapshot, node_id, value, revision)?;
        self.advance_bound(snapshot, node_id, value, revision)
    }

    pub(super) fn bind(&mut self, snapshot: &ZipSnapshot, node_id: &str, value: &str, revision: &str) -> Result<(), Fault> {
        if self.complete {
            return Err(fault("stdio.zip.work-complete", "archive target resolution has already completed"));
        }
        validate_text(value)?;
        if node_id.len() > 70 || revision.len() > 64 {
            return Err(fault("stdio.zip.argument-invalid", "archive draft address or revision is malformed"));
        }
        let digest: [u8; 64] = super::text_revision(&format!("{node_id}\0{revision}\0{value}")).as_bytes().try_into().expect("BLAKE3 hexadecimal digest is 64 bytes");
        let binding = (digest, snapshot.entries.len());
        if self.binding.is_some_and(|current| current != binding) {
            return Err(fault("stdio.zip.checkpoint-context", "archive cursor belongs to another command or snapshot"));
        }
        self.binding = Some(binding);
        Ok(())
    }

    pub(super) fn advance_bound(&mut self, snapshot: &ZipSnapshot, node_id: &str, value: &str, revision: &str) -> Result<Option<Emit<ZipMutation>>, Fault> {
        if self.complete {
            return Err(fault("stdio.zip.work-complete", "archive target resolution has already completed"));
        }
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
        Ok(Some(Emit { artifact_mutations: vec![ZipMutation::RenameEntry(crate::schema::mutations::rename_entry::RenameEntry { name: entry.name.clone(), new_name: value.into() })], ..Default::default() }))
    }
}

pub struct ArchiveTextWork<E: ArtifactEditor> {
    arguments: Arguments<E::Command>,
    cursor: ArchiveTextCursor,
    restored: bool,
    bound: bool,
    marker: PhantomData<fn() -> E>,
}

impl<E: ArtifactEditor> ArchiveTextWork<E> {
    pub fn new(arguments: Arguments<E::Command>) -> Self {
        Self { arguments, cursor: ArchiveTextCursor::default(), restored: false, bound: false, marker: PhantomData }
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
        WORKSPACE_IDENTITY
    }

    fn extent(&self, command: &E::Command, _snapshot: &ZipSnapshot, _interaction: &protocol::InteractionState, _context: Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<EditorApp<E>>>) -> Option<usize> {
        let (_, value, _) = (self.arguments)(command).ok()?;
        validate_text(value).ok()?;
        Some(2)
    }

    fn step(&mut self, input: &ArtifactCommandInputs<'_, EditorApp<E>>, _cx: &mut semio_framework_job::StepContext<'_>) -> Result<ArtifactCommandWorkStep<EditorApp<E>>, Fault> {
        if self.restored && input.context.is_none() {
            return Err(fault("stdio.zip.checkpoint-context", "resuming archive work requires the captured canonical context"));
        }
        let (node_id, value, revision) = (self.arguments)(input.command)?;
        if !self.bound {
            self.cursor.bind(input.snapshot, node_id, value, revision)?;
            self.bound = true;
        }
        match self.cursor.advance_bound(input.snapshot, node_id, value, revision)? {
            Some(emit) => Ok(ArtifactCommandWorkStep::Complete(emit)),
            None => Ok(ArtifactCommandWorkStep::Replay { stage: "zip-resolve-entry", preview: r#"{"en":"Checking archive entries","de":"Archiveinträge werden geprüft"}"#.as_bytes() }),
        }
    }

    fn checkpoint(&self, target: &mut [u8]) -> Result<usize, Fault> {
        self.cursor.checkpoint(target)
    }

    fn restore(&mut self, checkpoint: &[u8]) -> Result<(), Fault> {
        self.cursor.restore(checkpoint)?;
        self.restored = true;
        self.bound = false;
        Ok(())
    }

    fn begin_close(&mut self) {
        self.cursor.complete = true;
    }
}
