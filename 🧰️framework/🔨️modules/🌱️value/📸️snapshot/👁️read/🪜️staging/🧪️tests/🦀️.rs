//! 🧪️ The captured read keeps its original pointer through projection, split publication, and cancellation.
use super::*;
use crate::{ErasedSnapshotRead,SnapshotReadRegistryHandle};
use semio_framework_value::RetainedCloneGrant;
use semio_framework_value_clone_source_testing::drive::{drive,drive_reply};
use std::sync::Arc;

fn grant() -> RetainedCloneGrant {
    RetainedCloneGrant {
        maximum_items: 1,
        maximum_copy_bytes: 16384,
        maximum_capacity_bytes: 16384,
        maximum_release_bytes: 16384,
        maximum_depth: 128,
    }
}

fn turn(mut step: impl FnMut(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome) {
    drive_reply(grant(), 64, |result| result.as_ref().map_or_else(
        ValueError::retained_progress, |step: &RetainedCloneStep| step.progress(),
    ), |effect, reply, recover| {
        assert!(!recover);
        Ok(reply.complete(step(effect).result))
    }).result.unwrap();
}

#[test]
fn snapshot_read_staging_projects_and_releases_the_exact_store_read() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let source=fixture["source"].as_str().unwrap();
    let cuts=fixture["cancelAfterTurns"].as_array().unwrap().iter().map(|cut|cut.as_u64().unwrap());
    for cut in cuts.chain([256]) {
        let registry=SnapshotReadRegistryHandle::new();
        let original=Arc::new(source.to_owned());
        let lease=registry.try_issue(original.clone()).unwrap();
        let read=ErasedSnapshotRead::new(original.clone(),lease).try_into_typed::<String>().ok().unwrap();
        let (_, index, generation)=read.lease_identity().unwrap();
        let mut stage=SnapshotReadSourceStaging::default();
        stage.begin(read).ok().unwrap();
        let mut published=None;
        let mut saw_reserved=false;
        for _ in 0..cut {
            if stage.terminal_is_empty(){break}
            assert!(stage.next_advance_work_demand().unwrap()>0);
            turn(|effect|stage.step(&mut published,effect));
            if let Some(reserved)=published.as_ref().filter(|owner|owner.publication_pending()){
                assert!(reserved.try_borrow().is_err());
                saw_reserved=true;
            }
        }
        if cut==256 {
            assert!(saw_reserved&&stage.terminal_is_empty());
            let owner=published.as_ref().unwrap();
            assert!(std::ptr::eq(owner.borrow().get(),original.as_ref()));
            assert_eq!(owner.borrow().get(),fixture["published"].as_str().unwrap());
        }
        let mut returned=None;
        for _ in 0..256 {
            if stage.terminal_is_empty(){break}
            turn(|effect|stage.close_step(&mut returned,&mut published,effect));
        }
        assert!(stage.terminal_is_empty());
        if let Some(read)=returned{assert!(read.return_to_registry());}
        if let Some(mut owner)=published{
            for _ in 0..256 {
                if owner.terminal_is_empty(){break}
                drive(grant(),64,|effect|owner.close_step(4096,effect)).result.unwrap();
            }
            assert!(owner.terminal_is_empty());
        }
        if registry.has_returned(){
            let returned=registry.try_take(index,generation).unwrap().downcast::<String>().ok().unwrap();
            assert!(Arc::ptr_eq(&original,&returned));
        }
        assert!(registry.terminal_is_empty());
    }
}
