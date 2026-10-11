use super::*;
use semio_framework_job::{root_cancel_token, Generation, InteractiveJobCloseStep, OperationId, StepBudget, StepContext};

fn wallet() -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 64, maximum_copy_bytes: 1 << 24, maximum_capacity_bytes: 1 << 24, maximum_release_bytes: 1 << 24, maximum_depth: 64 }
}

fn poll_until_delivered(publication: &mut Publication) {
    let cancel = root_cancel_token();
    for turn in 0..100_000 {
        let mut sequence = 0;
        let mut progress = RetainedCloneProgress::default();
        let mut context = StepContext::new(OperationId(9), Generation(1), StepBudget::new(1, u64::MAX, wallet()), cancel.clone(), || Some(0), &mut sequence, &mut progress);
        if publication.poll(&mut context).expect("DEFLATE publication turn").is_some() {
            return;
        }
        assert!(turn < 99_999, "publication must deliver");
    }
}

fn close(publication: &mut Publication) {
    let unbounded = RetainedCloneGrant { maximum_items: usize::MAX, maximum_copy_bytes: usize::MAX, maximum_capacity_bytes: usize::MAX, maximum_release_bytes: usize::MAX, maximum_depth: usize::MAX };
    for _ in 0..100_000 {
        if publication.terminal_is_empty() {
            return;
        }
        assert!(!matches!(publication.close_step(unbounded), InteractiveJobCloseStep::Refused { .. }), "publication close refused");
    }
    panic!("publication close must terminate");
}

#[test]
fn checkpoint_publication_delivers_once_certifies_its_progress_and_closes_to_terminal_empty() {
    let state: Vec<u8> = (0..40_000).map(|index| (index % 251) as u8).collect();
    let mut publication = Publication::new(Kind::Checkpoint(7), state, Vec::new());
    assert!(!publication.is_delivered());
    poll_until_delivered(&mut publication);
    assert!(publication.is_delivered());
    assert_eq!(publication.applied_progress(), Some(7));
    close(&mut publication);
}

#[test]
fn commit_publication_lends_the_exact_output_after_state_and_closes_to_terminal_empty() {
    let output: Vec<u8> = (0..70_000).map(|index| (index % 239) as u8).collect();
    let mut publication = Publication::new(Kind::Commit, vec![1, 2, 3], output.clone());
    poll_until_delivered(&mut publication);
    let sealed = publication.commit_output().expect("a delivered commit lends its output");
    let mut bytes = Vec::with_capacity(sealed.len());
    for index in 0..sealed.page_count() {
        bytes.extend_from_slice(sealed.page(index).expect("sealed page"));
    }
    assert_eq!(bytes, output);
    assert_eq!(publication.applied_progress(), None);
    close(&mut publication);
}
