//! 🧰️ Command driver registry behavior: the default and the driver registry implementation. See [`super`].

use super::*;

//#region 🔖️PagedCommandIngress
impl<const CAPACITY: usize> Default for CommandDriverRegistry<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> CommandDriverRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0 && CAPACITY <= usize::from(u16::MAX));
        Self { slots: std::array::from_fn(|_| None), close_head: None, close_tail: None, occupied: 0 }
    }

    pub fn try_insert(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) -> Result<(), (semio_framework_diagnostic::Fault, CommandBatchDriver)> {
        if !self.can_insert(key) {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-capacity"), "fixed retained command-driver slot is occupied or collided"), driver));
        }
        self.insert_admitted(key, generation, driver);
        Ok(())
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn insert_admitted(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed retained command-driver admission changed before insert");
        self.slots[index] = Some(CommandDriverRetentionSlot { key, generation, driver, state: CommandDriverRetentionState::Active, close_previous: None, close_next: None });
        self.occupied += 1;
    }

    pub fn with_driver_mut<R>(&mut self, key: u64, generation: u64, f: impl FnOnce(&mut CommandBatchDriver) -> R) -> Result<R, semio_framework_diagnostic::Fault> {
        let slot = self.slot_mut(key, generation)?;
        if slot.state != CommandDriverRetentionState::Active {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-not-active"), "retained command driver is suspended or closing"));
        }
        Ok(f(&mut slot.driver))
    }

    pub fn prepare_suspend(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-suspend-state"), "retained command driver is not active before suspension"));
        }
        self.link_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Suspended;
        Ok(())
    }

    pub fn resume(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Suspended {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-resume-state"), "retained command driver is not suspended before resume"));
        }
        self.unlink_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Active;
        Ok(())
    }

    pub fn begin_close(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        let state = self.slots[index].as_ref().expect("retained command slot exists").state;
        if state == CommandDriverRetentionState::Active {
            self.link_close(index);
        }
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Closing;
        Ok(())
    }

    pub fn begin_close_key(&mut self, key: u64) -> Result<u64, semio_framework_diagnostic::Fault> {
        let index = key as usize % CAPACITY;
        let generation = match self.slots[index].as_ref() {
            Some(slot) if slot.key == key => slot.generation,
            _ => return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-stale"), "retained command driver key is stale")),
        };
        self.begin_close(key, generation)?;
        Ok(generation)
    }

    pub fn remove_terminal(&mut self, key: u64, generation: u64) -> Result<(), semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        if !self.slots[index].as_ref().expect("retained command slot exists").driver.terminal_is_empty() {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-nonterminal-remove"), "retained command driver cannot be removed before terminal empty"));
        }
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            self.unlink_close(index);
        }
        let terminal = self.slots[index].take().expect("retained command slot exists");
        self.occupied -= 1;
        drop(terminal);
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(index) = self.close_head.map(usize::from) else {
            return (self.occupied == 0, 0, 0);
        };
        let (terminal, released) = {
            let slot = self.slots[index].as_mut().expect("close-list command slot exists");
            slot.driver.close_step(maximum_bytes)
        };
        if terminal {
            self.unlink_close(index);
            let terminal = self.slots[index].take().expect("terminal command slot exists");
            self.occupied -= 1;
            drop(terminal);
        }
        (self.occupied == 0, 1, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0 && self.close_head.is_none() && self.close_tail.is_none()
    }

    pub fn has_close_work(&self) -> bool {
        self.close_head.is_some()
    }

    pub fn contains(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).is_ok()
    }

    pub fn is_active(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).ok().and_then(|index| self.slots[index].as_ref()).is_some_and(|slot| slot.state == CommandDriverRetentionState::Active)
    }

    fn slot_mut(&mut self, key: u64, generation: u64) -> Result<&mut CommandDriverRetentionSlot, semio_framework_diagnostic::Fault> {
        let index = self.index_of(key, generation)?;
        Ok(self.slots[index].as_mut().expect("retained command slot exists"))
    }

    fn index_of(&self, key: u64, generation: u64) -> Result<usize, semio_framework_diagnostic::Fault> {
        let index = key as usize % CAPACITY;
        match self.slots[index].as_ref() {
            Some(slot) if slot.key == key && slot.generation == generation => Ok(index),
            _ => Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-driver-stale"), "retained command driver identity or generation is stale")),
        }
    }

    fn link_close(&mut self, index: usize) {
        let previous = self.close_tail;
        let index_u16 = u16::try_from(index).expect("command registry capacity is u16-bounded");
        {
            let slot = self.slots[index].as_mut().expect("retained command slot exists");
            slot.close_previous = previous;
            slot.close_next = None;
        }
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = Some(index_u16);
        } else {
            self.close_head = Some(index_u16);
        }
        self.close_tail = Some(index_u16);
    }

    fn unlink_close(&mut self, index: usize) {
        let (previous, next) = {
            let slot = self.slots[index].as_ref().expect("retained command slot exists");
            (slot.close_previous, slot.close_next)
        };
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = next;
        } else {
            self.close_head = next;
        }
        if let Some(next) = next {
            self.slots[usize::from(next)].as_mut().expect("next close slot exists").close_previous = previous;
        } else {
            self.close_tail = previous;
        }
        let slot = self.slots[index].as_mut().expect("retained command slot exists");
        slot.close_previous = None;
        slot.close_next = None;
    }
}
//#endregion 🔖️PagedCommandIngress
