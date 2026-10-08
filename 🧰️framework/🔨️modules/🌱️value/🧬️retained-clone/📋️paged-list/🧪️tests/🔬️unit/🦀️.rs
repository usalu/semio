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
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = grant.maximum_release_bytes;
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
    fn advance(&mut self, source: RetainedCloneRef<'_, DropProbe>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        source.bind(&mut self.source)?;
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<DropProbe>(), retained_capacity_bytes: 0, released_bytes: 0 };
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
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

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "drop probe close must begin")); }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        super::super::close_retained_binding(&mut self.source, grant)
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
    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingChild>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        source.bind(&mut self.source)?;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: grant.maximum_copy_bytes.saturating_add(1), retained_capacity_bytes: 0, released_bytes: 0 }))
    }

    fn take(&mut self) -> Option<NonconformingChild> {
        None
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, _maximum_items: usize, _maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
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
    fn advance(&mut self, source: RetainedCloneRef<'_, NonconformingRetirementChild>, _grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
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

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
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

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if !self.reported {
            self.reported = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: grant.maximum_items.saturating_add(1), copied_bytes: 0, retained_capacity_bytes: grant.maximum_capacity_bytes.saturating_add(1), released_bytes: grant.maximum_release_bytes.saturating_add(1) }));
        }
        super::super::close_retained_binding(&mut self.source, grant)
    }
}

impl RetainedClone for NonconformingRetirementChild {
    type Cursor = NonconformingRetirementChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        NonconformingRetirementChildCursor { output: None, source: None, closing: false, reported: false }
    }
}

#[derive(Debug, PartialEq, crate::RetainedClone, crate::RetireOwned)]
struct OversizedOwner {
    first: String,
    second: String,
    third: String,
}

#[derive(crate::RetireOwned)]
struct ReleaseProbe(u8);

struct ReleaseProbeCursor([u8; 128]);
static RELEASE_PROBE_DROPS: AtomicUsize = AtomicUsize::new(0);
static RELEASE_PROBE_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

impl Drop for ReleaseProbeCursor {
    fn drop(&mut self) { RELEASE_PROBE_DROPS.fetch_add(1, Ordering::SeqCst); }
}

impl RetainedClone for ReleaseProbe {
    type Cursor = ReleaseProbeCursor;
    fn retained_clone_cursor() -> Self::Cursor { ReleaseProbeCursor([0;128]) }
}

impl RetainedCloneCursor<ReleaseProbe> for ReleaseProbeCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, ReleaseProbe>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 || grant.maximum_copy_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        self.0[0] = 1;
        self.0[2] = source.get().0;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: 1, ..Default::default() }))
    }
    fn take(&mut self) -> Option<ReleaseProbe> {
        if self.0[0] == 0 { return None; }
        self.0[0] = 0;
        Some(ReleaseProbe(std::mem::take(&mut self.0[2])))
    }
    fn begin_close(&mut self) -> bool { let started=self.0[1]==0; self.0[1]=1; started }
    fn close_step(&mut self, _: usize, _: usize) -> Result<SnapshotRetirementStep, crate::ValueError> { Ok(SnapshotRetirementStep::Complete) }
    fn close_granted(&mut self, _: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> { Ok(RetainedCloneStep::Complete(Default::default())) }
    fn next_close_capacity_byte_demand(&self, _: usize) -> Result<usize, crate::ValueError> { Ok(0) }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { Ok(0) }
    fn terminal_is_empty(&self) -> bool { self.0[1]==1 && self.0[0]==0 }
}

#[test]
fn paged_native_typed_field_close_exposes_exact_scaffold_demand() {
    use crate::value::observe_retirement_allocations;
    let _guard=RELEASE_PROBE_LOCK.lock().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=super::super::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let (mut cursor,allocation)=observe_retirement_allocations(super::super::RetainedFieldCursor::<ReleaseProbe>::default);
    assert_eq!(allocation,(0,0));
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:64, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    let (birth,allocation)=observe_retirement_allocations(||cursor.advance(source.borrow(),grant).unwrap());
    assert_eq!(allocation,(128,0));assert_eq!(birth.progress().retained_capacity_bytes,128);
    cursor.advance(source.borrow(),grant).unwrap();assert_eq!(cursor.take().unwrap().0,17);
    let (before,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(before,(0,0));
    cursor.begin_close();
    let (metadata,allocation)=observe_retirement_allocations(||cursor.close_granted(RetainedCloneGrant {maximum_items:1,..Default::default()}).unwrap());
    assert_eq!(metadata.progress().copied_items,1);assert_eq!(allocation,(0,0));
    let (demand,allocation)=observe_retirement_allocations(||(cursor.next_close_capacity_byte_demand(0).unwrap(),cursor.next_close_release_byte_demand().unwrap()));
    assert_eq!(allocation,(0,0));assert_eq!(demand,(0,fixture["physicalCursorBytes"].as_u64().unwrap() as usize));
    let (below,allocation)=observe_retirement_allocations(||cursor.close_granted(RetainedCloneGrant {maximum_release_bytes:127,..grant}).unwrap());
    assert_eq!(below.progress(),Default::default());assert_eq!(allocation,(0,0));
    let (exact,allocation)=observe_retirement_allocations(||cursor.close_granted(RetainedCloneGrant {maximum_items:1,maximum_release_bytes:128,..Default::default()}).unwrap());
    assert_eq!(allocation,(0,128));assert_eq!(exact.progress().released_bytes,128);assert_eq!(exact.progress().copied_bytes,0);
    cursor.close_granted(RetainedCloneGrant {maximum_items:1,..Default::default()}).unwrap();assert!(cursor.terminal_is_empty());
    assert_eq!(observe_retirement_allocations(||drop(cursor)).1,(0,0));
    println!("[DEBUG] Typed child exact demand observation0heap; metadata preserves128-byte backing, below127 retains, release-only128 frees exactly once");
}

#[test]
fn retained_paged_list_release_authority_is_distinct_from_copy_and_admits_exact_physical_extent() {
    let _guard=RELEASE_PROBE_LOCK.lock().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧫️fixtures/📏️release-authority/🔣️.json")).unwrap();
    let source=super::super::RetainedCloneSource::from_owner(ReleaseProbe(fixture["inputByte"].as_u64().unwrap() as u8));
    let mut cursor=super::super::RetainedFieldCursor::<ReleaseProbe>::default();
    let grant=RetainedCloneGrant { maximum_items:2, maximum_copy_bytes:64, maximum_capacity_bytes:4096, maximum_release_bytes:4096, maximum_depth:64 };
    assert_eq!(size_of::<ReleaseProbeCursor>(),fixture["physicalCursorBytes"].as_u64().unwrap() as usize);
    RELEASE_PROBE_DROPS.store(0,Ordering::SeqCst);
    let birth=cursor.advance(source.borrow(),grant).unwrap().progress();
    assert_eq!(birth.retained_capacity_bytes,128);
    let copy=cursor.advance(source.borrow(),grant).unwrap().progress();
    assert_eq!(copy.copied_bytes,1);
    assert_eq!(cursor.take().unwrap().0,17);
    cursor.begin_close();
    assert_eq!(cursor.close_granted(grant).unwrap().progress().copied_items,1);
    let below=RetainedCloneGrant { maximum_release_bytes:fixture["oneBelowReleaseBytes"].as_u64().unwrap() as usize, ..grant };
    assert_eq!(cursor.close_granted(below).unwrap().progress(),RetainedCloneProgress::default());
    assert_eq!(RELEASE_PROBE_DROPS.load(Ordering::SeqCst),0);
    assert!(!cursor.terminal_is_empty());
    let released=cursor.close_granted(RetainedCloneGrant { maximum_release_bytes:128, ..grant }).unwrap().progress();
    assert_eq!(released.released_bytes,128);
    assert_eq!(released.copied_bytes,0);
    assert_eq!(released.retained_capacity_bytes,0);
    assert_eq!(RELEASE_PROBE_DROPS.load(Ordering::SeqCst),1);
    for _ in 0..4 { if cursor.terminal_is_empty() { break; } cursor.close_granted(grant).unwrap(); }
    assert!(cursor.terminal_is_empty());
    println!("[DEBUG] Independent retained authority copy64/release4096 preserves128-byte owner below127, exact128 releases once without copying");
}

#[derive(Clone, Copy, Debug)]
struct InsufficientScaffoldChild;

impl RetireOwned for InsufficientScaffoldChild {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        crate::retirement::leaf(self)
    }
}

struct InsufficientScaffoldChildCursor {
    output: Option<InsufficientScaffoldChild>,
    source: Option<RetainedCloneBinding>,
    closing: bool,
}

impl RetainedCloneCursor<InsufficientScaffoldChild> for InsufficientScaffoldChildCursor {
    fn advance(&mut self, source: RetainedCloneRef<'_, InsufficientScaffoldChild>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        source.bind(&mut self.source)?;
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        if grant.maximum_items == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        self.output = Some(*source.get());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    fn take(&mut self) -> Option<InsufficientScaffoldChild> {
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        let started = !self.closing;
        self.closing = true;
        started
    }

    fn close_step(&mut self, _maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        if maximum_bytes < 128 {
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.output.is_none() && self.source.is_none()
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_release_bytes < 128 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "fixture scaffold cannot progress under its declared 128-byte release grant"));
        }
        super::super::close_retained_binding(&mut self.source, grant)
    }
}

impl RetainedClone for InsufficientScaffoldChild {
    type Cursor = InsufficientScaffoldChildCursor;

    fn retained_clone_cursor() -> Self::Cursor {
        InsufficientScaffoldChildCursor { output: None, source: None, closing: false }
    }
}

#[derive(crate::RetainedClone, crate::RetireOwned)]
struct DerivedScaffoldOwner {
    child: InsufficientScaffoldChild,
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
    maximum_release_bytes: usize,
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
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth, maximum_release_bytes: fixture.grant.maximum_release_bytes }
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
fn retained_paged_list_adopts_completed_owner_larger_than_copy_budget() {
    let expected = OversizedOwner { first: "alpha".into(), second: "beta".into(), third: "gamma".into() };
    assert!(size_of::<OversizedOwner>() > 64);
    let mut values = PagedList::<OversizedOwner, 1>::default();
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().expect("oversized owner capacity");
        assert!(values.reserve_one(required).expect("oversized owner reserve").progressed);
    }
    values.push_reserved(expected).expect("oversized owner source placement");
    let source = super::super::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<OversizedOwner, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let copied = loop {
        match cursor.advance(source.borrow(), grant).expect("oversized owner retained copy") {
            RetainedCloneStep::Progress(progress) => assert_ne!(progress, RetainedCloneProgress::default()),
            RetainedCloneStep::Complete(_) => break cursor.take().expect("oversized owner retained output"),
        }
    };
    assert_eq!(copied.get(0), Some(&OversizedOwner { first: "alpha".into(), second: "beta".into(), third: "gamma".into() }));
    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        cursor.close_step(8, 4096).expect("oversized owner cursor close");
    }
    let mut retirement = crate::retirement::owned_retirement(copied);
    while !retirement.terminal_is_empty() {
        retirement.close_step(8, 4096).expect("oversized owner retirement");
    }
    drop(source);
}

#[test]
fn retained_paged_list_refuses_scaffold_grant_that_cannot_release_owner() {
    let mut values = PagedList::<InsufficientScaffoldChild, 1>::default();
    while !values.has_reserved_slot() {
        let required = values.next_allocation_bytes().expect("insufficient scaffold capacity");
        assert!(values.reserve_one(required).expect("insufficient scaffold reserve").progressed);
    }
    values.push_reserved(InsufficientScaffoldChild).expect("insufficient scaffold source placement");
    let source = super::super::RetainedCloneSource::from_owner(values);
    let mut cursor = PagedList::<InsufficientScaffoldChild, 1>::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let error = loop {
        match cursor.advance(source.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(progress)) => assert_ne!(progress, RetainedCloneProgress::default()),
            Ok(RetainedCloneStep::Complete(_)) => panic!("insufficient scaffold release must not complete"),
            Err(error) => break error,
        }
    };
    assert_eq!(error.kind, crate::ValueRefusalKind::WorkLimit);
    assert!(error.message.contains("cannot progress"));
    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        cursor.close_step(8, 4096).expect("insufficient scaffold cursor close");
    }
    drop(source);
}

#[test]
fn retained_derive_refuses_scaffold_grant_that_cannot_release_field() {
    let source = super::super::RetainedCloneSource::from_owner(DerivedScaffoldOwner { child: InsufficientScaffoldChild });
    let mut cursor = DerivedScaffoldOwner::retained_clone_cursor();
    let grant = RetainedCloneGrant { maximum_items: 8, maximum_copy_bytes: 64, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 64 };
    let error = loop {
        match cursor.advance(source.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(progress)) => assert_ne!(progress, RetainedCloneProgress::default()),
            Ok(RetainedCloneStep::Complete(_)) => panic!("derived insufficient scaffold release must not complete"),
            Err(error) => break error,
        }
    };
    assert_eq!(error.kind, crate::ValueRefusalKind::WorkLimit);
    assert!(error.message.contains("cannot progress"));
    assert!(cursor.begin_close());
    while !cursor.terminal_is_empty() {
        cursor.close_step(8, 4096).expect("derived insufficient scaffold cursor close");
    }
    drop(source);
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
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: size_of::<NonconformingChild>().max(1), maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    let error = loop {
        match cursor.advance(retained.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child must not complete"),
            Err(error) => break error,
        }
    };
    assert!(error.message.contains("exceeded its retained clone"));
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
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 1, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    loop {
        match cursor.advance(retained.borrow(), grant) {
            Ok(RetainedCloneStep::Progress(_)) => {}
            Ok(RetainedCloneStep::Complete(_)) => panic!("over-budget child retirement must be refused before completion"),
            Err(error) => {
                assert!(error.message.contains("exceeded its retained clone"));
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
    let grant = RetainedCloneGrant { maximum_items: 5, maximum_copy_bytes: 4096, maximum_capacity_bytes: 4096, maximum_depth: 64, maximum_release_bytes: 4096 };
    while cursor.values.len() < 8 {
        assert!(matches!(cursor.advance(retained.borrow(), grant).expect("drop probe prefix"), RetainedCloneStep::Progress(_)));
    }
    let abandoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(cursor)));
    assert!(abandoned.is_err(), "active cursor abandonment fails fast");
    assert_eq!(drops.load(Ordering::SeqCst), 0, "abandonment retains every partial page and payload owner");
    drop(retained);
}
