//! 📬️ Paged structural edit for archive entry renames and archive comment changes.

use crate::schema::mutations::{rename_entry, set_archive_comment};
use crate::{ZipMutation, ZipSnapshot};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
use semio_s_artifact_stdio_contract::editing::NativeEditPreparationRoute;
use semio_plugin::plugin_app_close_prelude::store::{self as app_store, PagedOneItemEdit, PagedOneItemEditStep, PagedOneItemPost, PagedOneItemPostMode, PagedOneItemPostRead};
use std::sync::Arc;

const MAXIMUM_STRUCTURAL_ITEMS: usize = 4_096;
const MAXIMUM_TEXT_BYTES: usize = u16::MAX as usize;

fn structural_items(snapshot: &ZipSnapshot) -> Option<usize> {
    snapshot.entries.iter().try_fold(snapshot.entries.len(), |total, entry| total.checked_add(entry.metadata.local.extra_fields.len())?.checked_add(entry.metadata.central.extra_fields.len()))
}

pub(super) fn route(_prefix: &'static str) -> Option<NativeEditPreparationRoute<ZipSnapshot, ZipMutation>> {
    None
}

#[derive(Default)]
pub(super) struct ZipTextEdit {
    last_work: u64,
    admitted: bool,
    entry_index: usize,
    matches: usize,
    collision: bool,
    located: Option<usize>,
    scanned: bool,
    closing: bool,
}

fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

fn unit(bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }
}

impl ZipTextEdit {
    fn envelope(&mut self, post: &ZipSnapshot) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        self.last_work+=1;
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
            self.last_work+=1;
            if self.matches != 1 || self.collision {
                return Err(refusal("stdio-zip-base-snapshot-edit-stale-rename"));
            }
            self.scanned = true;
            return Ok(PagedOneItemEditStep::Progress(unit(0)));
        };
        if entry.name.len() > MAXIMUM_TEXT_BYTES {
            self.last_work+=1;
            return Err(refusal("stdio-zip-base-snapshot-edit-entry-name-envelope"));
        }
        let bytes = entry.name.len();
        if grant.maximum_copy_bytes < bytes {
            return Ok(PagedOneItemEditStep::Progress(RetainedCloneProgress::default()));
        }
        self.last_work+=1;
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
        self.last_work+=1;
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
        self.last_work+=1;
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

    fn post_mode(&self) -> PagedOneItemPostMode {
        PagedOneItemPostMode::Mutable
    }

    fn advance_demands(&self, _before: RetainedCloneRef<'_, ZipSnapshot>, _mutation: RetainedCloneRef<'_, ZipMutation>, _post: PagedOneItemPostRead<'_, ZipSnapshot>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn advance(&mut self, _before: RetainedCloneRef<'_, ZipSnapshot>, mutation: RetainedCloneRef<'_, ZipMutation>, post: PagedOneItemPost<'_, ZipSnapshot>, _body: usize, grant: RetainedCloneGrant, receiving: &mut NativeDecodeControl<'_>) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
        self.last_work=0;
        if grant.maximum_items==0||grant.maximum_depth==0{return Ok(PagedOneItemEditStep::Progress(Default::default()))}
        if !receiving.prepare_work(if !self.admitted||matches!(mutation.get(),ZipMutation::RenameEntry(_))&&!self.scanned{1}else{2})?{return Ok(PagedOneItemEditStep::Progress(Default::default()))}
        let result=(||{
        let mutation = mutation.get();
        let post = post.mutable()?;
        if !self.admitted {
            return self.envelope(post);
        }
        match mutation {
            ZipMutation::RenameEntry(rename_entry::RenameEntry { name, new_name }) => {
                if !self.scanned {
                    return self.scan(post, name, new_name, grant);
                }
                let step = self.rename(post, name, new_name, grant)?;
                finish(step, post, &mut self.last_work)
            }
            ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment, comment_utf8 }) => {
                let step = self.comment(post, comment, *comment_utf8, grant)?;
                finish(step, post, &mut self.last_work)
            }
            _ => Err(refusal("stdio-zip-base-snapshot-edit-unsupported-mutation")),
        }
        })();
        receiving.commit_work(self.last_work)?;
        result
    }

    fn last_turn_work(&self)->u64{self.last_work}

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_demands(&self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<ZipMutation>, _body: usize) -> Result<RetirementDemand, ValueError> {
        Ok(RetirementDemand::default())
    }

    fn close_step(&mut self, _mutations: &dyn ArtifactOwnedValueRetirementFactory<ZipMutation>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.closing = true;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
    }
}

fn finish(step: PagedOneItemEditStep<ZipMutation>, post: &ZipSnapshot, last_work:&mut u64) -> Result<PagedOneItemEditStep<ZipMutation>, ValueError> {
    if matches!(step, PagedOneItemEditStep::Complete(..)) {
        *last_work+=1;
        crate::standards::v2_0::subsets::base::io::validate_zip_snapshot_serialization(post).map_err(|error| ValueError::new(ValueRefusalKind::InvalidValue, format!("{}-post-validation-{error}", ZipTextEdit::PREFIX)))?;
    }
    Ok(step)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::snapshot::{ZipCentralHeaderMetadata, ZipEntry, ZipEntryMetadata, ZipExtraField, ZipLocalHeaderMetadata};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("🧪️tests/🧫️fixtures/🧵️bounded-copy/🔣️.json")).expect("bounded ZIP preparation fixture")
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


    type Fixture=semio_os_store_one_item::testkit::PagedEditFixture<ZipSnapshot,ZipMutation,ZipTextEdit>;
    fn close_grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096}}
    fn setup(post:ZipSnapshot,mutation:ZipMutation)->Fixture{
        use semio_os_store_one_item::testkit::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
        let before=semio_framework_value::testing::admit_owned_source(post.clone(),()).0;
        let mutation=semio_framework_value::testing::admit_owned_source(mutation,()).0;
        let caller=OriginalJobFixtureCaller::admit(OperationId(44793),Generation(8),RetainedCloneGrant{maximum_items:1<<20,maximum_copy_bytes:16_777_216,maximum_capacity_bytes:16_777_216,maximum_release_bytes:16_777_216,maximum_depth:4096},||Some(1),u64::MAX,1<<20);
        Fixture::new(caller,OriginalJobDecodeOwner::empty(),before,mutation,post,ZipTextEdit::default())
    }
    fn retire<T:semio_framework_value::RetireOwned+'static>(fixture:&mut Fixture,value:T){let mut value=Some(value);let mut active=None;assert!(fixture.retire_owned(&mut value,&mut active,close_grant(),1_000_000).expect("funded ZIP fixture output retirement"));assert!(value.is_none()&&active.is_none());}
    fn drive(fixture:&mut Fixture)->Result<(usize,RetainedCloneProgress,ZipMutation),ValueError>{
        let mut turns=0;
        loop{
            let outcome=fixture.advance(grant()).expect("original ZIP caller");assert_eq!(outcome.work,fixture.last_work());
            match outcome.edit?{
                PagedOneItemEditStep::Progress(p)=>{assert!(p.fits(grant())&&p.copied_items<=1);if p!=Default::default(){assert!(outcome.work>0);}},
                PagedOneItemEditStep::Complete(p,inverse)=>return Ok((turns,p,inverse)),
                _=>panic!("ZIP edit completes with one inverse"),
            }
            turns+=1;assert!(turns<1000,"bounded ZIP edit converges");
        }
    }
    #[test]
    fn rename_scans_one_entry_per_turn_and_builds_the_exact_forward_and_inverse() {
        let fixture = fixture();
        let source = snapshot();
        let replacement = "renamed".repeat(fixture["replacementRepeats"].as_u64().expect("replacementRepeats") as usize);
        let mutation = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: "before.bin".into(), new_name: replacement.clone() });
        let mut fixture = setup(source.clone(),mutation.clone());
        let (turns, progress, inverse) = drive(&mut fixture).expect("rename edit");
        let post=fixture.post().unwrap();
        assert!(turns >= source.entries.len());
        assert!(progress.fits(grant()));
        assert_eq!(inverse, ZipMutation::RenameEntry(rename_entry::RenameEntry { name: replacement, new_name: "before.bin".into() }));
        assert_eq!(vec![inverse.clone()], <ZipMutation as protocol::ColdMutation<ZipSnapshot>>::inverse(&mutation, &source).expect("independent mutation algebra inverse"));
        assert_eq!(post.entries[0].data, source.entries[0].data);
        assert_eq!(post.entries[1], source.entries[1]);
        assert_eq!(post.comment, source.comment);
        let outcome = <ZipMutation as protocol::ColdMutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(post, &protocol::apply_diff_cold(outcome.diff(), &source).expect("independent mutation algebra applies the rename"));
        retire(&mut fixture,inverse);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }

    #[test]
    fn comment_edit_changes_only_the_comment_and_builds_the_exact_inverse() {
        let source = snapshot();
        let mutation = ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment { comment: "replaced comment".into(), comment_utf8: false });
        let mut fixture = setup(source.clone(),mutation.clone());
        let (_, progress, inverse) = drive(&mut fixture).expect("comment edit");
        let post=fixture.post().unwrap();
        assert!(progress.fits(grant()));
        assert_eq!(post.entries, source.entries);
        assert_eq!(post.comment, "replaced comment");
        assert!(!post.comment_utf8);
        assert_eq!(vec![inverse.clone()], <ZipMutation as protocol::ColdMutation<ZipSnapshot>>::inverse(&mutation, &source).expect("independent mutation algebra inverse"));
        let outcome = <ZipMutation as protocol::ColdMutation<ZipSnapshot>>::diff(&mutation, &source);
        assert_eq!(post, &protocol::apply_diff_cold(outcome.diff(), &source).expect("independent mutation algebra applies the comment"));
        retire(&mut fixture,inverse);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }

    #[test]
    fn stale_or_colliding_rename_is_refused_before_any_entry_changes() {
        let source = snapshot();
        for (name, new_name) in [("absent.bin", "other.bin"), ("before.bin", "sibling.txt")] {
            let mutation = ZipMutation::RenameEntry(rename_entry::RenameEntry { name: name.into(), new_name: new_name.into() });
            let mut fixture = setup(source.clone(),mutation.clone());
            let error = drive(&mut fixture).expect_err("stale rename is refused");
            let post=fixture.post().unwrap();
            assert!(format!("{error:?}").contains("stale-rename"));
            assert_eq!(post, &source);
            let reset=fixture.advance(RetainedCloneGrant{maximum_items:0,..grant()}).unwrap();assert_eq!(reset.work,0);assert_eq!(reset.progress,Default::default());retire(&mut fixture,error);assert!(fixture.close(close_grant(),1_000_000).unwrap());
        }
    }

    #[test]
    fn edit_waits_without_progress_when_the_grant_cannot_cover_one_turn(){
        let source=snapshot();let mutation=ZipMutation::SetArchiveComment(set_archive_comment::SetArchiveComment{comment:"replaced".into(),comment_utf8:true});let mut fixture=setup(source.clone(),mutation);
        let first=fixture.advance(grant()).unwrap();assert!(matches!(first.edit,Ok(PagedOneItemEditStep::Progress(_))));assert!(first.work>0);
        let denied=fixture.advance_with_work(grant(),0).unwrap();assert_eq!(denied.work,0);assert_eq!(denied.progress,Default::default());
        let starved=fixture.advance(RetainedCloneGrant{maximum_capacity_bytes:0,..grant()}).unwrap();assert_eq!(starved.work,0);assert_eq!(starved.progress,Default::default());assert_eq!(fixture.post().unwrap(),&source);assert!(fixture.close(close_grant(),1_000_000).unwrap());
    }
}
