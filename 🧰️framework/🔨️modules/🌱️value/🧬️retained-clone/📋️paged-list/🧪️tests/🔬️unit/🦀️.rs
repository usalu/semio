use super::*;
use serde::Deserialize;
use std::{
    mem::{ManuallyDrop, size_of},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

struct DropProbe {
    drops: Arc<AtomicUsize>,
}

impl Drop for DropProbe {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}

struct DropProbeRetirement {
    value: ManuallyDrop<Option<DropProbe>>,
    released: bool,
}

impl RetirementCursor for DropProbeRetirement {
    fn close_step(&mut self, maximum_bytes: usize) -> RetirementStep {
        if self.released {
            return RetirementStep::Complete;
        }
        if maximum_bytes < size_of::<DropProbe>() {
            return RetirementStep::BudgetExhausted;
        }
        drop(self.value.take());
        self.released = true;
        RetirementStep::Bytes(size_of::<DropProbe>())
    }

    fn terminal_is_empty(&self) -> bool {
        self.released
    }
}

impl Drop for DropProbeRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.released, "drop probe retired before terminal-empty");
    }
}

impl RetireOwned for DropProbe {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(DropProbeRetirement { value: ManuallyDrop::new(Some(self)), released: false })
    }
}

struct DropProbeCursor {
    output: Option<DropProbe>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    close: RetainedCloneClose,
}

impl RetainedCloneCursor<DropProbe> for DropProbeCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, DropProbe>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        source.bind(&mut self.source)?;
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DropProbe>(), retained_capacity_bytes: 0 };
        if !progress.fits(grant) {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.output = Some(DropProbe { drops: Arc::clone(&source.get().drops) });
        Ok(RetainedCloneStep::Complete(progress))
    }

    fn take(&mut self) -> Option<DropProbe> {
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if !self.close.is_empty() {
            return self.close.step(maximum_items, maximum_bytes);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none() && self.close.is_empty()
    }
}

impl RetainedClone for DropProbe {
    type Cursor = DropProbeCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        DropProbeCursor { output: None, source: None, closing: false, close: RetainedCloneClose::default() }
    }
}

#[derive(Clone, Copy)]
struct NonconformingChild;

impl RetireOwned for NonconformingChild {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        crate::retirement::leaf(self)
    }
}

struct NonconformingChildCursor {
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl RetainedCloneCursor<NonconformingChild> for NonconformingChildCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingChild>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        source.bind(&mut self.source)?;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: grant.maximum_copy_bytes.saturating_add(1), retained_capacity_bytes: 0 }))
    }

    fn take(&mut self) -> Option<NonconformingChild> {
        None
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.source.is_none()
    }
}

impl RetainedClone for NonconformingChild {
    type Cursor = NonconformingChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingChildCursor { source: None, closing: false }
    }
}

#[derive(Clone, Copy)]
struct NonconformingRetirementChild;

impl RetireOwned for NonconformingRetirementChild {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        crate::retirement::leaf(self)
    }
}

struct NonconformingRetirementChildCursor {
    output: Option<NonconformingRetirementChild>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
    reported: bool,
}

impl RetainedCloneCursor<NonconformingRetirementChild> for NonconformingRetirementChildCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingRetirementChild>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        source.bind(&mut self.source)?;
        self.output = Some(NonconformingRetirementChild);
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()))
    }

    fn take(&mut self) -> Option<NonconformingRetirementChild> {
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if !self.reported {
            self.reported = true;
            return Ok(SnapshotRetirementStep::Pending { released_items: maximum_items.saturating_add(1), released_bytes: maximum_bytes.saturating_add(1) });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.reported && self.source.is_none() && self.output.is_none()
    }
}

impl RetainedClone for NonconformingRetirementChild {
    type Cursor = NonconformingRetirementChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingRetirementChildCursor { output: None, source: None, closing: false, reported: false }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    maximum_entries: usize,
    entry_count: usize,
    value_prefix: String,
    grant: GrantFixture,
    cancellation_after_entries: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GrantFixture {
    maximum_items: usize,
    maximum_copy_bytes: usize,
    maximum_capacity_bytes: usize,
    maximum_depth: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/📦️copy/🔣️.json")).expect("paged list retained fixture")
}

fn source(fixture: &Fixture) -> PagedList<String, 1024> {
    assert_eq!(fixture.maximum_entries, 1024);
    let mut source = PagedList::default();
    for ordinal in 0..fixture.entry_count {
        while !source.has_reserved_slot() {
            let required = source.next_allocation_bytes().expect("fixture capacity");
            assert!(source.reserve_one(required).expect("fixture reserve").progressed);
        }
        source.push_reserved(format!("{}{}", fixture.value_prefix, ordinal)).expect("fixture placement");
    }
    source
}

fn grant(fixture: &Fixture) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth }
}

#[test]
fn retained_paged_list_copy_matches_vec_serde_and_closes_page_by_page() {
    let fixture = fixture();
    let source = source(&fixture);
    let expected: Vec<_> = source.iter().cloned().collect();
    let oracle = serde_json::to_vec(&expected).expect("serde oracle");
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    let copied = loop {
        turns += 1;
        let step = cursor.advance(retained.borrow(), grant).expect("paged retained copy");
        assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) {
            break cursor.take().expect("paged retained output");
        }
        assert!(turns < 100_000, "paged retained copy terminates");
    };
    assert!(turns > 1);
    assert_eq!(serde_json::to_vec(&copied.iter().cloned().collect::<Vec<_>>()).expect("copied serde"), oracle);
    assert!(cursor.begin_close());
    let mut close_turns = 0usize;
    while !cursor.terminal_is_empty() {
        close_turns += 1;
        let step = cursor.close_step(1, fixture.grant.maximum_capacity_bytes).expect("spent cursor close");
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        assert!(close_turns < 100_000, "spent cursor close terminates");
    }
    assert_eq!(close_turns, 1, "a spent cursor whose output was taken holds only its source binding and closes in one step");
    let mut retirement = crate::retirement::owned_retirement(copied);
    let mut retirement_turns = 0usize;
    while !retirement.terminal_is_empty() {
        retirement_turns += 1;
        match retirement.close_step(1, fixture.grant.maximum_capacity_bytes).expect("production paged retirement scheduler") {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
            }
            SnapshotRetirementStep::Blocked => panic!("exact page grant must progress"),
            SnapshotRetirementStep::Complete => assert!(retirement.terminal_is_empty()),
        }
        assert!(retirement_turns < 100_000, "paged retirement terminates");
    }
    assert!(retirement_turns > 1);
}

#[test]
fn retained_paged_list_cancellation_preserves_source_and_retires_partial_pages() {
    let fixture = fixture();
    let source = source(&fixture);
    let expected: Vec<_> = source.iter().cloned().collect();
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<String, 1024>::retained_clone_cursor();
    let grant = grant(&fixture);
    let mut turns = 0usize;
    while turns < fixture.cancellation_after_entries {
        turns += 1;
        if matches!(cursor.advance(retained.borrow(), grant).expect("paged retained prefix"), RetainedCloneStep::Complete(_)) {
            panic!("fixture must cancel a partial paged copy");
        }
    }
    assert!(cursor.begin_close());
    drop(retained);
    let mut close_turns = 0usize;
    while !cursor.terminal_is_empty() {
        close_turns += 1;
        let step = cursor.close_step(1, fixture.grant.maximum_capacity_bytes).expect("partial paged close");
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= fixture.grant.maximum_capacity_bytes);
        }
        assert!(close_turns < 100_000, "partial paged close terminates");
    }
    assert!(close_turns > 1);
    assert_eq!(expected.len(), fixture.entry_count);
}

#[test]
fn retained_paged_list_refuses_over_budget_child_before_owner_placement() {
    let mut source = PagedList::<NonconformingChild, 1>::default();
    while !source.has_reserved_slot() {
        let required = source.next_allocation_bytes().expect("nonconforming fixture capacity");
        assert!(source.reserve_one(required).expect("nonconforming fixture reserve").progressed);
    }
    assert!(source.push_reserved(NonconformingChild).is_ok());
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<NonconformingChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<NonconformingChild>().max(1), maximum_capacity_bytes: 4096, maximum_depth: 64 };
    let error = loop {
        match cursor.advance(retained.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child must not complete"),
            Err(error) => break error,
        }
    };
    assert!(error.contains("exceeded its retained clone"));
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_none());
    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(1, 4096).expect("nonconforming child close");
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= 4096);
        }
    }
}

#[test]
fn retained_paged_list_refuses_over_budget_child_retirement_before_owner_placement() {
    let mut source = PagedList::<NonconformingRetirementChild, 1>::default();
    while !source.has_reserved_slot() {
        let required = source.next_allocation_bytes().expect("nonconforming retirement fixture capacity");
        assert!(source.reserve_one(required).expect("nonconforming retirement fixture reserve").progressed);
    }
    assert!(source.push_reserved(NonconformingRetirementChild).is_ok());
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<NonconformingRetirementChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: 4096, maximum_depth: 64 };
    loop {
        match cursor.advance(retained.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child retirement must be refused before completion"),
            Err(error) => {
                assert!(error.contains("exceeded its retained retirement"));
                break;
            }
        }
    }
    assert_eq!(cursor.index, 0);
    assert_eq!(cursor.values.len(), 0);
    assert!(cursor.child_value.is_some());
    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        let step = cursor.close_step(1, 4096).expect("nonconforming retirement child closes after its refused report");
        if let SnapshotRetirementStep::Pending { released_items, released_bytes } = step {
            assert!(released_items <= 1);
            assert!(released_bytes <= 4096);
        }
    }
}

#[test]
fn retained_paged_list_abandonment_panics_without_running_payload_destructors() {
    let drops = Arc::new(AtomicUsize::new(0));
    let mut source = PagedList::<DropProbe, 64>::default();
    for _ in 0..64 {
        while !source.has_reserved_slot() {
            let required = source.next_allocation_bytes().expect("drop probe fixture capacity");
            assert!(source.reserve_one(required).expect("drop probe fixture reserve").progressed);
        }
        assert!(source.push_reserved(DropProbe { drops: Arc::clone(&drops) }).is_ok());
    }
    let retained = super::super::RetainedCloneSource::from_owner(source);
    let mut cursor = PagedList::<DropProbe, 64>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 5, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_depth: 64 };
    while cursor.values.len() < 8 {
        assert!(matches!(cursor.advance(retained.borrow(), grant).expect("drop probe prefix"), RetainedCloneStep::Progress(_)));
    }
    let abandoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor)));
    assert!(abandoned.is_err(), "active cursor abandonment fails fast");
    assert_eq!(drops.load(Ordering::SeqCst), 0, "abandonment retains every partial page and payload owner");
    drop(retained);
}
