//! 🔒️ The reactor close registry and the close participant steps of the lifecycle owner.

use super::*;

struct ReactorCloseState {
    pub(super) key: instance_lifetime::NativeCloseKey,
    pub(super) instance: u32,
    pub(super) active: bool,
    pub(super) complete: bool,
    pub(super) task_cursor: usize,
    pub(super) timer_cursor: usize,
    pub(super) request_cursor: requests::RequestCloseCursor,
    pub(super) resume_remaining: usize,
    pub(super) command_ingress_complete: bool,
    pub(super) requests_complete: bool,
    pub(super) resumes_complete: bool,
    /// ✅️ `cancel_instance_tasks_step`'s own completion witness. The cursor alone cannot stand in
    /// for it: the executor answers `Complete` without advancing its cursor whenever the sweep has
    /// nothing left to visit, so a cursor-bound loop condition never terminates (26/09/09).
    pub(super) tasks_complete: bool,
    pub(super) timers_complete: bool,
    pub(super) metadata_complete: bool,
}

pub(super) struct ReactorCloseRegistry {
    pub(super) slots: ReactorFixedSlots<ReactorCloseState>,
}

impl ReactorCloseRegistry {
    pub(crate) fn new() -> Self {
        Self { slots: ReactorFixedSlots::new() }
    }

    pub(super) fn index(instance: u32) -> usize {
        instance as usize % PLUGIN_REACTOR_INSTANCE_SLOTS
    }

    pub(super) fn insert(&mut self, state: ReactorCloseState) -> Result<(), ReactorCloseState> {
        let index = Self::index(state.instance);
        self.slots.insert(index, state)
    }

    fn take_at(&mut self, index: usize) -> Option<ReactorCloseState> {
        self.slots.take(index)
    }
}

pub(super) fn preflight_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let closes = closes.try_borrow().map_err(|_| reactor_close_fault("reactor close preflight busy"))?;
        if !closes.slots.allocation_admitted {
            return Err(reactor_close_fault("reactor close backing unavailable"));
        }
        if closes.slots.get(ReactorCloseRegistry::index(key.instance())).is_some_and(|retained| retained.key != key) {
            return Err(reactor_close_fault("reactor close slot belongs to another allocation"));
        }
        Ok(())
    })
}

pub(super) fn reserve_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    let instance = key.instance();
    let request_cursor = REGISTRY.with(|registry| registry.begin_cancel_instance(instance));
    let resume_remaining = TASK_RESUMES.with(|resumes| resumes.borrow().begin_cancel_instance());
    let state = ReactorCloseState {
        key,
        instance,
        active: false,
        complete: false,
        task_cursor: 0,
        timer_cursor: 0,
        request_cursor,
        resume_remaining,
        command_ingress_complete: false,
        requests_complete: false,
        resumes_complete: false,
        tasks_complete: false,
        timers_complete: false,
        metadata_complete: false,
    };
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close authority is busy"))?;
        if let Some(retained) = closes.slots.get(ReactorCloseRegistry::index(instance)) {
            return if retained.key == key { Ok(()) } else { Err(reactor_close_fault("reactor close slot belongs to another captured allocation")) };
        }
        closes.insert(state).map_err(|_| semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-capacity"), "fixed reactor close authority is saturated or collided"))
    })
}

pub(super) fn reactor_close_fault(message: &'static str) -> semio_framework::Fault {
    semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.reactor-close-authority"), message)
}

pub(super) fn activate_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close authority is busy"))?;
        let state = closes.slots.get_mut(ReactorCloseRegistry::index(key.instance())).filter(|state| state.key == key).ok_or_else(|| reactor_close_fault("exact reactor close reservation missing"))?;
        state.active = true;
        Ok(())
    })
}

pub(super) fn reactor_close_complete(key: instance_lifetime::NativeCloseKey) -> Result<bool, semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let closes = closes.try_borrow().map_err(|_| reactor_close_fault("reactor close receipt is busy"))?;
        closes.slots.get(ReactorCloseRegistry::index(key.instance())).filter(|state| state.key == key).map(|state| state.complete).ok_or_else(|| reactor_close_fault("exact reactor close receipt missing"))
    })
}

pub(super) fn release_reactor_close(key: instance_lifetime::NativeCloseKey) -> Result<(), semio_framework::Fault> {
    REACTOR_CLOSES.with(|closes| {
        let mut closes = closes.try_borrow_mut().map_err(|_| reactor_close_fault("reactor close receipt is busy"))?;
        let index = ReactorCloseRegistry::index(key.instance());
        if !closes.slots.get(index).is_some_and(|state| state.key == key && state.complete) {
            return Err(reactor_close_fault("reactor close receipt is not terminal"));
        }
        let _ = closes.take_at(index);
        Ok(())
    })
}
