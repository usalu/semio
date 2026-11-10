//! 🔎️ Keyed lookup retains genuine list and query sources and compares one byte per funded turn.
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use semio_framework_value::{
    retained_clone::{RetainedCloneBinding, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep},
    RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind,
};
use std::mem::size_of;

/// 🎯️ Complete key uniqueness is decided before any projection write.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedKeyMatch {
    Missing,
    Unique(usize),
    Duplicate(usize, usize),
}

/// 🗝️ The row owner projects one stored String field in constant time without scanning, allocating or invoking other user code.
pub trait RetainedKeyedRow {
    /// 🪪️ Borrows only the stored key field; the lookup funds this fixed projection separately from each byte comparison.
    fn retained_key(&self) -> &String;
}

impl RetainedKeyedRow for String {
    fn retained_key(&self) -> &String {
        self
    }
}

/// 🧷️ Fixed scan state and real source aliases survive every denied or cancelled turn.
#[derive(Default)]
pub struct RetainedKeyLookup {
    rows: Option<RetainedCloneBinding>,
    key: Option<RetainedCloneBinding>,
    index: usize,
    offset: usize,
    first: Option<usize>,
    result: Option<RetainedKeyMatch>,
    closing: bool,
}

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvariantViolated, message)
}
fn scalar(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}

impl RetainedKeyLookup {
    /// 🌱️ Constructs no payload, list census or child allocation.
    pub fn new() -> Self {
        Self::default()
    }

    fn verify<R>(&self, rows: RetainedCloneRef<'_, Vec<R>>, key: RetainedCloneRef<'_, String>) -> Result<(), ValueError> {
        rows.check_binding(&self.rows)?;
        key.check_binding(&self.key)?;
        Ok(())
    }

    /// 📏️ Quotes one source alias, fixed scan event or byte comparison without traversing keys.
    pub fn demand<R: RetainedKeyedRow>(&self, rows: RetainedCloneRef<'_, Vec<R>>, key: RetainedCloneRef<'_, String>, _maximum_body_bytes: usize, maximum_work_units: u64) -> Result<RetirementDemand, ValueError> {
        if self.closing {
            return Err(invalid("key lookup already closing"));
        }
        self.verify(rows, key)?;
        if self.selected_work(rows.get().len()) > maximum_work_units || self.result.is_some() {
            return Ok(Default::default());
        }
        if self.rows.is_none() {
            return Ok(scalar(rows.binding_copy_bytes()));
        }
        if self.key.is_none() {
            return Ok(scalar(key.binding_copy_bytes()));
        }
        Ok(scalar(size_of::<&String>() + 2 + 2 * size_of::<usize>() + size_of::<Option<usize>>() + size_of::<Option<RetainedKeyMatch>>()))
    }

    fn selected_work(&self, row_count: usize) -> u64 {
        if self.result.is_some() {
            0
        } else if self.rows.is_none() || self.key.is_none() || self.index >= row_count {
            1
        } else {
            2
        }
    }

    /// ⛽️ Each row funds its fixed stored-key projection and one byte scan; alias and final turns fund their actual single event.
    pub fn advance_work_demand<R: RetainedKeyedRow>(&self, rows: RetainedCloneRef<'_, Vec<R>>, key: RetainedCloneRef<'_, String>, maximum_body_bytes: usize, maximum_work_units: u64) -> Result<u64, ValueError> {
        self.demand(rows, key, maximum_body_bytes, maximum_work_units)?;
        let selected = self.selected_work(rows.get().len());
        Ok(if selected <= maximum_work_units { selected } else { 0 })
    }

    fn matched(&mut self) -> usize {
        if let Some(first) = self.first {
            self.result = Some(RetainedKeyMatch::Duplicate(first, self.index));
            size_of::<Option<RetainedKeyMatch>>()
        } else {
            self.first = Some(self.index);
            self.index += 1;
            self.offset = 0;
            size_of::<Option<usize>>() + 2 * size_of::<usize>()
        }
    }

    /// ▶️ Advances only the selected original alias or one byte comparison after both grants fit.
    pub fn advance<R: RetainedKeyedRow>(
        &mut self,
        rows: RetainedCloneRef<'_, Vec<R>>,
        key: RetainedCloneRef<'_, String>,
        maximum_body_bytes: usize,
        maximum_work_units: u64,
        effect: &mut semio_framework_value::RetainedAllocationEffect<'_>,
    ) -> RetainedWorkOutcome<RetainedCloneStep, ValueError> {
        let grant = effect.grant();
        let mut work: u128 = 0;
        let result = (|| {
            let demand = self.demand(rows, key, maximum_body_bytes, maximum_work_units)?;
            if self.result.is_some() {
                return Ok(RetainedCloneStep::Complete(Default::default()));
            }
            let selected = self.advance_work_demand(rows, key, maximum_body_bytes, maximum_work_units)?;
            if selected == 0 || !retained_clone_grant_admits(grant, demand) {
                return Ok(RetainedCloneStep::Progress(Default::default()));
            }
            if self.rows.is_none() {
                let turn = rows.bind(&mut self.rows, effect);
                work = turn.work;
                return Ok(RetainedCloneStep::Progress(turn.result?.ok_or_else(|| invalid("key lookup row source was not captured"))?));
            }
            if self.key.is_none() {
                let turn = key.bind(&mut self.key, effect);
                work = turn.work;
                return Ok(RetainedCloneStep::Progress(turn.result?.ok_or_else(|| invalid("key lookup query source was not captured"))?));
            }
            work = u128::from(selected);
            let copied_bytes = if let Some(row) = rows.get().get(self.index) {
                let current = row.retained_key().as_bytes();
                let needle = key.get().as_bytes();
                size_of::<&String>()
                    + if current.len() != needle.len() {
                        self.index += 1;
                        self.offset = 0;
                        2 * size_of::<usize>()
                    } else if current.is_empty() {
                        self.matched()
                    } else {
                        let equal = current[self.offset] == needle[self.offset];
                        let end = self.offset + 1;
                        if !equal {
                            self.index += 1;
                            self.offset = 0;
                            2 + 2 * size_of::<usize>()
                        } else if end == current.len() {
                            2 + self.matched()
                        } else {
                            self.offset = end;
                            2 + size_of::<usize>()
                        }
                    }
            } else {
                self.result = Some(self.first.map_or(RetainedKeyMatch::Missing, RetainedKeyMatch::Unique));
                size_of::<Option<RetainedKeyMatch>>()
            };
            let progress = RetainedCloneProgress { copied_items: 1, copied_bytes, ..Default::default() };
            Ok(if self.result.is_some() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
        })();
        RetainedWorkOutcome { result, work }
    }

    /// 👁️ Returns uniqueness only after complete preflight while the original sources remain held.
    pub fn result(&self) -> Option<RetainedKeyMatch> {
        if self.closing {
            None
        } else {
            self.result
        }
    }

    /// 🛑️ Stops scans before settling the genuine immutable source aliases.
    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    fn closing_binding(&self) -> &Option<RetainedCloneBinding> {
        if self.key.is_some() {
            &self.key
        } else {
            &self.rows
        }
    }

    /// ♻️ Quotes the actual next source-alias close without taking list or query ownership.
    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.closing {
            return Err(invalid("key lookup closure not started"));
        }
        RetainedCloneBinding::close_demand(self.closing_binding(), body)
    }

    /// ⛽️ Delegates closure work to its actual retained source owner.
    pub fn close_work_demand(&self, body: usize) -> Result<u64, ValueError> {
        self.close_demand(body)?;
        RetainedCloneBinding::close_work_demand(self.closing_binding(), body)
    }

    /// 🪜️ Returns one genuine alias and preserves the defining owner's work and failure receipt.
    pub fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> semio_framework_value::OriginalRetirementPoll {
        let grant = effect.grant();
        let done = |result| semio_framework_value::RetainedWorkPoll::Complete(RetainedWorkOutcome { result, work: 0 });
        if !self.closing {
            return done(Err(invalid("key lookup closure not started").into()));
        }
        let demand = match self.close_demand(grant.maximum_copy_bytes) {
            Ok(demand) => demand,
            Err(error) => return done(Err(error.into())),
        };
        if !retained_clone_grant_admits(grant, demand) {
            return done(Ok(RetainedCloneStep::Progress(Default::default())));
        }
        let turn = RetainedCloneBinding::close_one(if self.key.is_some() { &mut self.key } else { &mut self.rows }, grant.maximum_copy_bytes, effect);
        turn.map(|step| if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    /// 🫙️ No original list or query alias remains.
    pub fn terminal_is_empty(&self) -> bool {
        self.rows.is_none() && self.key.is_none()
    }
}

impl Drop for RetainedKeyLookup {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained key lookup requires original source closure");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
