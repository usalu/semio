//! 📬️ Paged structural edit for archive entry renames and archive comment changes.

use crate::schema::mutations::{rename_entry, set_archive_comment};
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_framework_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep};
use std::sync::Arc;

const MAXIMUM_STRUCTURAL_ITEMS: usize = 4_096;
const MAXIMUM_TEXT_BYTES: usize = u16::MAX as usize;

fn structural_items(snapshot: &ZipSnapshot) -> Option<usize> {
    snapshot.entries.iter().try_fold(snapshot.entries.len(), |total, entry| total.checked_add(entry.metadata.local.extra_fields.len())?.checked_add(entry.metadata.central.extra_fields.len()))
}

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<ZipSnapshot, ZipMutation>> {
    Some(NativeEditPreparationRoute::new(ZipTextEdit::recognizes, Arc::new(app_store::PagedOneItemPreparationFactory::<ZipSnapshot, ZipMutation, ZipTextEdit>::default())))
}

#[derive(Default)]
pub(super) struct ZipTextEdit {
    admitted: bool,
    entry_index: usize,
    matches: usize,
    collision: bool,
    located: Option<usize>,
    scanned: bool,
}

fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn unit(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }
}

impl ZipTextEdit {
    fn envelope(&mut self, post: &ZipSnapshot) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        if structural_items(post).is_none_or(|items| items > MAXIMUM_STRUCTURAL_ITEMS) {
            return Err(refusal("stdio-zip-base-snapshot-edit-structural-envelope"));
        }
        if post.comment.len() > MAXIMUM_TEXT_BYTES {
            return Err(refusal("stdio-zip-base-snapshot-edit-comment-envelope"));
        }
        self.admitted = true;
        Ok(PagedOneItemEditStep::Progress(unit(0)))
    }

    fn scan(&mut self, post: &ZipSnapshot, name: &str, new_name: &str, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        let Some(entry) = post.entries.get(self.entry_index) else {
            if self.matches != 1 || self.collision {
                return Err(refusal("stdio-zip-base-snapshot-edit-stale-rename"));
            }
            self.scanned = true;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        if entry.name.len() > MAXIMUM_TEXT_BYTES {
            return Err(refusal("stdio-zip-base-snapshot-edit-entry-name-envelope"));
        }
        let bytes = entry.name.len();
        if grant.maximum_copy_bytes < bytes {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        if entry.name == name {
            self.matches = self.matches.saturating_add(1);
            self.located = Some(self.entry_index);
        } else if entry.name == new_name {
            self.collision = true;
        }
        self.entry_index += 1;
        Ok(PagedOneItemEditStep::Progress(unit(bytes)))
    }

    fn rename(&mut self, post: &mut ZipSnapshot, name: &str, new_name: &str, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        let copied = new_name.len();
        let retained = new_name.len().saturating_mul(2).saturating_add(name.len());
        let released = name.len();
        if grant.maximum_copy_bytes < copied || grant.maximum_capacity_bytes < retained || grant.maximum_release_bytes < released {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        let index = self.located.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "stdio-zip-base-snapshot-edit-located"))?;
        let entry = post.entries.get_mut(index).ok_or_else(|| refusal("stdio-zip-base-snapshot-edit-entry-index"))?;
        entry.name = new_name.to_owned();
        let inverse = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: new_name.to_owned(), new_name: name.to_owned() });
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, retained_capacity_bytes: retained, released_bytes: released }, inverse))
    }

    fn comment(&mut self, post: &mut ZipSnapshot, comment: &str, comment_utf8: bool, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        let copied = comment.len();
        let retained = comment.len().saturating_add(post.comment.len());
        let released = post.comment.len();
        if grant.maximum_copy_bytes < copied || grant.maximum_capacity_bytes < retained || grant.maximum_release_bytes < released {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        let previous = std::mem::replace(&mut post.comment, comment.to_owned());
        let previous_utf8 = std::mem::replace(&mut post.comment_utf8, comment_utf8);
        let inverse = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: previous.clone(), comment_utf8: previous_utf8 });
        Ok(PagedOneItemEditStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: copied, retained_capacity_bytes: retained, released_bytes: released }, inverse))
    }
}

impl PagedOneItemEdit<ZipSnapshot, ZipMutation> for ZipTextEdit {
    const PREFIX: &'static str = "stdio-zip-base-snapshot-edit";

    fn recognizes(mutation: &ZipMutation) -> bool {
        matches!(mutation, ZipMutation::RenameEntry(_) | ZipMutation::SetArchiveComment(_))
    }

    fn preflight(mutation: &ZipMutation) -> Result<usize, String> {
        match mutation {
            ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) if !name.is_empty() && name != new_name && name.len() <= MAXIMUM_TEXT_BYTES && !new_name.is_empty() && new_name.len() <= MAXIMUM_TEXT_BYTES => Ok(name.len() + new_name.len()),
            ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, .. }) if comment.len() <= MAXIMUM_TEXT_BYTES => Ok(comment.len()),
            _ => Err(format!("{}-admission", Self::PREFIX)),
        }
    }

    fn advance(&mut self, post: &mut ZipSnapshot, mutation: &ZipMutation, grant: RetainedCloneGrant) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        if !self.admitted {
            return self.envelope(post);
        }
        match mutation {
            ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => {
                if !self.scanned {
                    return self.scan(post, name, new_name, grant);
                }
                let step = self.rename(post, name, new_name, grant)?;
                finish(step, post)
            }
            ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, comment_utf8 }) => {
                let step = self.comment(post, comment, *comment_utf8, grant)?;
                finish(step, post)
            }
            _ => Err(refusal("stdio-zip-base-snapshot-edit-unsupported-mutation")),
        }
    }
}

fn finish(step: PagedOneItemEditStep<ZipMutation>, post: &ZipSnapshot) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
    if matches!(step, PagedOneItemEditStep::Complete(..)) {
        crate::standards::v2_0::subsets::base::io::validate_zip_snapshot_serialization(post).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, format!("{}-post-validation-{error}", ZipTextEdit::PREFIX)))?;
    }
    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::snapshot::{ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧫️fixtures/🧵️bounded-copy/🔣️.json")).expect("bounded ZIP preparation fixture")
    }

    fn snapshot() -> ZipSnapshot {
        let metadata = ZipEntryMetadata {
            local: ZipLocalHeaderMetadata { extra_fields: vec![ZipExtraField { id: 0xCAFE, data: (0..8_193).map(|index| (index % 251) as u8).collect() }], ..Default::default() },
            central: ZipCentralHeaderMetadata { extra_fields: vec![ZipExtraField { id: 0xBEEF, data: (0..8_195).map(|index| (index % 247) as u8).collect() }], comment: "member comment".repeat(257), ..Default::default() },
            ..Default::default()
        };
        ZipSnapshot {
            schema: "stdio.zip".into(),
            entries: vec![ZipEntry { name: "before.bin".into(), data: (0..4_096).map(|index| (index % 251) as u8).collect(), metadata }, ZipEntry { name: "sibling.txt".into(), data: b"unchanged".to_vec(), ..Default::default() }],
            comment: "original comment".into(),
            ..Default::default()
        }
    }

    fn grant() -> RetainedCloneGrant {
        RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 4_096, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 8 }
    }

    fn drive(post: &mut ZipSnapshot, mutation: &ZipMutation) -> Result<(usize, RetainedCloneProgress, ZipMutation), ValueError> {
        let mut edit = ZipTextEdit::default();
        let mut turns = 0;
        loop {
            match edit.advance(post, mutation, grant())? {
                PagedOneItemEditStep::Progress(progress) => assert!(progress.fits(grant()) && progress.copied_items <= 1),
                PagedOneItemEditStep::Complete(progress, inverse) => return Ok((turns, progress, inverse)),
            }
            turns += 1;
            assert!(turns < 1_000, "bounded edit converges");
        }
    }

    #[test]
    fn rename_scans_one_entry_per_turn_and_builds_the_exact_forward_and_inverse() {
        let fixture = fixture();
        let source = snapshot();
        let replacement = "renamed".repeat(fixture["replacementRepeats"].as_u64().expect("replacementRepeats") as usize);
        let mutation = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: "before.bin".into(), new_name: replacement.clone() });
        let mut post = source.clone();
        let (turns, progress, inverse) = drive(&mut post, &mutation).expect("rename edit");
        assert!(turns >= source.entries.len());
        assert!(progress.fits(grant()));
        assert_eq!(inverse, ZipMutation::RenameEntry(rename_entry::RenameEntry { name: replacement, new_name: "before.bin".into() }));
        assert_eq!(vec![inverse], <ZipMutation as protocol::Mutation<ZipSnapshot>>::inverse(&mutation, &source).expect("independent mutation algebra inverse"));
        assert_eq!(post.entries[0].data, source.entries[0].data);
        assert_eq!(post.entries[1], source.entries[1]);
        assert_eq!(post.comment, source.comment);
        let outcome = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(post, protocol::apply_diff(outcome.diff(), &source).expect("independent mutation algebra applies the rename"));
    }

    #[test]
    fn comment_edit_changes_only_the_comment_and_builds_the_exact_inverse() {
        let source = snapshot();
        let mutation = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "replaced comment".into(), comment_utf8: false });
        let mut post = source.clone();
        let (_, progress, inverse) = drive(&mut post, &mutation).expect("comment edit");
        assert!(progress.fits(grant()));
        assert_eq!(post.entries, source.entries);
        assert_eq!(post.comment, "replaced comment");
        assert!(!post.comment_utf8);
        assert_eq!(vec![inverse], <ZipMutation as protocol::Mutation<ZipSnapshot>>::inverse(&mutation, &source).expect("independent mutation algebra inverse"));
        let outcome = <ZipMutation as protocol::Mutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(post, protocol::apply_diff(outcome.diff(), &source).expect("independent mutation algebra applies the comment"));
    }

    #[test]
    fn stale_or_colliding_rename_is_refused_before_any_entry_changes() {
        let source = snapshot();
        for (name, new_name) in [("absent.bin", "other.bin"), ("before.bin", "sibling.txt")] {
            let mutation = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: name.into(), new_name: new_name.into() });
            let mut post = source.clone();
            let error = drive(&mut post, &mutation).expect_err("stale rename is refused");
            assert!(format!("{error:?}").contains("stale-rename"));
            assert_eq!(post, source);
        }
    }

    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn() {
        let source = snapshot();
        let mutation = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "replaced".into(), comment_utf8: true });
        let mut post = source.clone();
        let mut edit = ZipTextEdit::default();
        assert!(matches!(edit.advance(&mut post, &mutation, grant()).expect("envelope turn"), PagedOneItemEditStep::Progress(_)));
        let starved = RetainedCloneGrant { maximum_capacity_bytes: 0, ..grant() };
        match edit.advance(&mut post, &mutation, starved).expect("starved comment turn") {
            PagedOneItemEditStep::Progress(progress) => assert_eq!(progress, RetainedCloneProgress::default()),
            PagedOneItemEditStep::Complete(..) => panic!("starved grant must not complete"),
        }
        assert_eq!(post, source);
    }
}
