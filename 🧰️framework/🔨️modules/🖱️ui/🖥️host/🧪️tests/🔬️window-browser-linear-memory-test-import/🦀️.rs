pub(super) mod linear_memory_test_import {
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum AfterProbe {
        None,
        Cancel,
        Close,
    }

    #[derive(Default)]
    struct State {
        incoming: VecDeque<Vec<u8>>,
        sent: Vec<Vec<u8>>,
        capacities: Vec<usize>,
        copies: usize,
        closed: bool,
        after_probe: Option<AfterProbe>,
    }

    thread_local! {
        static STATE: RefCell<State> = RefCell::new(State::default());
    }

    pub(crate) fn reset() {
        STATE.with(|state| *state.borrow_mut() = State::default());
    }

    pub(crate) fn enqueue(bytes: Vec<u8>) {
        STATE.with(|state| state.borrow_mut().incoming.push_back(bytes));
    }

    pub(crate) fn after_probe(action: AfterProbe) {
        STATE.with(|state| state.borrow_mut().after_probe = Some(action));
    }

    pub(crate) fn census() -> (Vec<usize>, usize, usize, bool) {
        STATE.with(|state| {
            let state = state.borrow();
            (state.capacities.clone(), state.copies, state.incoming.len(), state.closed)
        })
    }

    pub(crate) fn sent_lengths() -> Vec<usize> {
        STATE.with(|state| state.borrow().sent.iter().map(Vec::len).collect())
    }

    pub(super) unsafe fn send(pointer: *const u8, length: usize) -> i32 {
        let bytes = unsafe { std::slice::from_raw_parts(pointer, length) }.to_vec();
        STATE.with(|state| state.borrow_mut().sent.push(bytes));
        1
    }

    pub(super) unsafe fn poll(pointer: *mut u8, capacity: usize) -> i32 {
        STATE.with(|state| {
            let mut state = state.borrow_mut();
            state.capacities.push(capacity);
            let Some(bytes) = state.incoming.front() else { return if state.closed { -1 } else { 0 } };
            let length = bytes.len();
            if length > capacity {
                match state.after_probe.take().unwrap_or(AfterProbe::None) {
                    AfterProbe::None => {}
                    AfterProbe::Cancel => {
                        state.incoming.pop_front();
                    }
                    AfterProbe::Close => {
                        state.incoming.clear();
                        state.closed = true;
                    }
                }
                return length as i32;
            }
            unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), pointer, length) };
            state.incoming.pop_front();
            state.copies += 1;
            length as i32
        })
    }
}
