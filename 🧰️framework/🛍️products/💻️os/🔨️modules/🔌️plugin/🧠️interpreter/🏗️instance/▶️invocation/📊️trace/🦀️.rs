//! 📊️ Observes committed retained receipts and execution fuel on explicitly enabled ticket runs.
use super::*;
use std::cell::Cell;
use std::io::Write;
use std::sync::OnceLock;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Category {
    Scalar,
    FrameBirth,
    FrameReturn,
    Control,
    Memory,
    HostCall,
}

impl Category {
    const ALL: [Self; 6] = [
        Self::Scalar, Self::FrameBirth, Self::FrameReturn, Self::Control, Self::Memory, Self::HostCall,
    ];
    const NAMES: [&str; Self::ALL.len()] = [
        "scalar", "frame_birth", "frame_return", "control", "memory", "host_call",
    ];
}

#[derive(Clone, Copy, Default, PartialEq, Debug)]
pub(super) struct Work {
    pub(super) fuel: u64,
    pub(super) items: u64,
    pub(super) copy: u64,
    pub(super) capacity: u64,
    pub(super) release: u64,
}

impl Work {
    const ZERO: Self = Self { fuel: 0, items: 0, copy: 0, capacity: 0, release: 0 };

    fn observed(cx: &StepContext<'_>, fuel: u64) -> Self {
        let progress = cx.retained_progress();
        Self {
            fuel, items: progress.copied_items as u64, copy: progress.copied_bytes as u64,
            capacity: progress.retained_capacity_bytes as u64, release: progress.released_bytes as u64,
        }
    }

    fn since(self, before: Self) -> Self {
        Self {
            fuel: self.fuel - before.fuel, items: self.items - before.items, copy: self.copy - before.copy,
            capacity: self.capacity - before.capacity, release: self.release - before.release,
        }
    }

    pub(super) fn add(&mut self, other: Self) {
        self.fuel = self.fuel.saturating_add(other.fuel);
        self.items = self.items.saturating_add(other.items);
        self.copy = self.copy.saturating_add(other.copy);
        self.capacity = self.capacity.saturating_add(other.capacity);
        self.release = self.release.saturating_add(other.release);
    }
}

type Breakdown = [Work; Category::ALL.len()];

pub(super) struct Slice {
    category: Category,
    start: Work,
    before: Work,
    work: Breakdown,
}

impl Slice {
    pub(super) fn begin(cx: &StepContext<'_>, category: Category) -> Option<Self> {
        #[cfg(test)]
        let capturing = CAPTURE.with(|capture| capture.get().is_some());
        #[cfg(not(test))]
        let capturing = false;
        if !(capturing || enabled()) {
            return None;
        }
        let start = Work::observed(cx, 0);
        Some(Self { category, start, before: start, work: [Work::ZERO; Category::ALL.len()] })
    }

    pub(super) fn mark(&mut self, category: Category, cx: &StepContext<'_>, fuel: u64) {
        if self.category == category {
            return;
        }
        self.record(cx, fuel);
        self.category = category;
    }

    fn record(&mut self, cx: &StepContext<'_>, fuel: u64) {
        let current = Work::observed(cx, fuel);
        self.work[self.category as usize].add(current.since(self.before));
        self.before = current;
    }

    pub(super) fn zero_progress(&self, cx: &StepContext<'_>, fuel: u64) -> bool {
        Work::observed(cx, fuel) == self.start
    }

    pub(super) fn finish(mut self, module: &CoreModule, function: u32, cx: &StepContext<'_>, fuel: u64, pending: bool) {
        self.record(cx, fuel);
        let stalled = pending && self.before == self.start;
        #[cfg(test)]
        if CAPTURE.with(|capture| {
            if capture.get().is_none() {
                return false;
            }
            capture.set(Some((self.work, stalled)));
            true
        }) {
            return;
        }
        let cleanup = cx.original_retirement_active();
        let (index, name) = stage(module, function, cleanup);
        STAGES.with(|all| {
            let mut all = all.borrow_mut();
            let source = (cx.operation().0, cx.generation().0);
            if all.source != source {
                *all = Stages { source, ..Stages::ZERO };
            }
            let stage = &mut all.stages[index];
            stage.samples = stage.samples.saturating_add(1);
            stage.pending = stage.pending.saturating_add(u64::from(pending));
            stage.stalled = stage.stalled.saturating_add(u64::from(stalled));
            for category in Category::ALL {
                stage.work[category as usize].add(self.work[category as usize]);
            }
            if stage.samples.is_power_of_two() {
                stage.log(name, function, cx, cleanup, "sample");
            }
        });
    }
}

const ROOTS: usize = OwnedSemioExport::ALL.len() + 1;
static ENABLED: OnceLock<bool> = OnceLock::new();

fn enabled() -> bool {
    *ENABLED.get_or_init(|| std::env::var_os("SEMIO_TRACE_CORE_PHYSICAL").is_some_and(|value| value == "1"))
}

fn stage(module: &CoreModule, function: u32, cleanup: bool) -> (usize, &'static str) {
    let base = usize::from(cleanup) * ROOTS;
    for (index, export) in OwnedSemioExport::ALL.iter().enumerate() {
        let name = export.core_name();
        if matches!(module.export(name), Some(ExportKind::Function(index)) if *index == function) {
            return (base + index, name);
        }
    }
    (base + ROOTS - 1, "other")
}

#[derive(Clone, Copy)]
struct Stage {
    samples: u64,
    pending: u64,
    stalled: u64,
    flushed: bool,
    work: Breakdown,
}

impl Stage {
    const ZERO: Self = Self { samples: 0, pending: 0, stalled: 0, flushed: false, work: [Work::ZERO; Category::ALL.len()] };

    fn log(&self, name: &str, function: u32, cx: &StepContext<'_>, cleanup: bool, event: &str) {
        let categories: [_; Category::ALL.len()] = std::array::from_fn(|index| (Category::NAMES[index], self.work[index]));
        let (operation, generation) = (cx.operation().0, cx.generation().0);
        let epoch = cx.original_source_epoch();
        let _ = writeln!(std::io::stderr().lock(),
            "[DEBUG] Core physical event={event} operation={operation} generation={generation} epoch={epoch:?} stage={name} function={function} cleanup={cleanup} samples={} pending={} zero_progress_pending={} categories={categories:?}",
            self.samples, self.pending, self.stalled);
    }
}

struct Stages {
    source: (u64, u64),
    stages: [Stage; ROOTS * 2],
}

impl Stages {
    const ZERO: Self = Self { source: (0, 0), stages: [Stage::ZERO; ROOTS * 2] };
}

thread_local! {
    static STAGES: std::cell::RefCell<Stages> = const { std::cell::RefCell::new(Stages::ZERO) };
    static FRAME_PENDING: Cell<((u64, u64), u64)> = const { Cell::new(((0, 0), 0)) };
    #[cfg(test)]
    static CAPTURE: Cell<Option<(Breakdown, bool)>> = const { Cell::new(None) };
}

pub(super) fn pending_frame_return(machine: &Machine, cx: &StepContext<'_>, vm_work: u64) {
    if !enabled() {
        return;
    }
    let source = (cx.operation().0, cx.generation().0);
    let step = FRAME_PENDING.with(|pending| {
        let (previous, count) = pending.get();
        let count = if previous == source { count.saturating_add(1) } else { 1 };
        pending.set((source, count));
        count
    });
    if !step.is_power_of_two() {
        return;
    }
    let Some(frame) = machine.frames.last() else { return };
    let (tail, copy, release) = if let Some(control) = frame.controls.last() {
        if control.branch_types.capacity() != 0 {
            ("branch_types", size_of::<Vec<ValueType>>(), control.branch_types.capacity() * size_of::<ValueType>())
        } else if control.result_types.capacity() != 0 {
            ("result_types", size_of::<Vec<ValueType>>(), control.result_types.capacity() * size_of::<ValueType>())
        } else {
            ("control_frame", size_of::<ControlFrame>(), 0)
        }
    } else if frame.controls.capacity() != 0 {
        ("controls", size_of::<Vec<ControlFrame>>(), frame.controls.capacity() * size_of::<ControlFrame>())
    } else if frame.locals.capacity() != 0 {
        ("locals", size_of::<Vec<Value>>(), frame.locals.capacity() * size_of::<Value>())
    } else {
        ("frame", size_of::<Frame>() + size_of::<bool>(), 0)
    };
    let available = cx.retained_grant();
    let source_total = cx.original_source_total_grant();
    let work = cx.fuel_remaining();
    let source_work = cx.original_source_fuel_remaining();
    let root = machine.frames[0].function;
    let generation = source.1;
    let operation = source.0;
    let epoch = cx.original_source_epoch();
    let leaf = frame.function;
    let pc = frame.pc;
    let depth = machine.frames.len();
    let controls = (frame.controls.len(), frame.controls.capacity());
    let locals = (frame.locals.len(), frame.locals.capacity());
    let operands = (machine.values.len(), machine.values.capacity());
    let _ = writeln!(std::io::stderr().lock(),
        "[DEBUG] Core pending frame return sample={step} operation={operation} generation={generation} epoch={epoch:?} root={root} leaf={leaf} pc={pc} depth={depth} tail={tail} quote=(work=1, items=1, copy={copy}, capacity=0, release={release}, depth=1) vm_work={vm_work} work={work} available={available:?} source_work={source_work} source_total={source_total:?} controls={controls:?} locals={locals:?} operands={operands:?}");
}

pub(super) fn ordinary_flush_enabled(cx: &StepContext<'_>) -> bool {
    cx.original_retirement_active() && enabled()
}

pub(super) fn flush_ordinary(module: &CoreModule, function: u32, cx: &StepContext<'_>) {
    let (index, name) = stage(module, function, false);
    STAGES.with(|all| {
        let mut all = all.borrow_mut();
        if all.source == (cx.operation().0, cx.generation().0) {
            let stage = &mut all.stages[index];
            if !stage.flushed && stage.samples > 0 {
                stage.flushed = true;
                stage.log(name, function, cx, false, "ordinary_retirement");
            }
        }
    });
}

#[cfg(test)]
pub(super) fn capture<T>(run: impl FnOnce() -> T) -> (T, Breakdown, bool) {
    CAPTURE.with(|capture| {
        assert!(capture.replace(Some(([Work::ZERO; Category::ALL.len()], false))).is_none());
        let result = run();
        let (work, stalled) = capture.replace(None).unwrap();
        (result, work, stalled)
    })
}
