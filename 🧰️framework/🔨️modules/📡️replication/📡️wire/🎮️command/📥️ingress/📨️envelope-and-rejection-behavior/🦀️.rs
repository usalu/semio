//! 📨️ Command envelope behavior: the envelope set, the rejected command build with its registry and the command batch implementations. See [`super`].

use super::*;

//#region 🔖️PagedCommandIngress
impl CommandEnvelopeSet {
    pub fn try_new() -> Result<Self, semio_framework_diagnostic::Fault> {
        let mut commands = std::collections::VecDeque::new();
        commands
            .try_reserve_exact(COMMAND_BATCH_MAXIMUM_ITEMS)
            .map_err(|_| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-allocation"), "fixed command batch authority could not reserve its exact 64 slots"))?;
        Ok(Self { commands, page_storage: std::collections::VecDeque::new(), pages: 0, bytes: 0 })
    }

    pub fn try_push(&mut self, command: CommandEnvelope) -> Result<(), (semio_framework_diagnostic::Fault, CommandEnvelope)> {
        if self.commands.len() == COMMAND_BATCH_MAXIMUM_ITEMS {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-cap"), "command batch exceeds its exact 64-item authority"), command));
        }
        let pages = match self.pages.checked_add(command.command.page_len()) {
            Some(pages) if pages <= COMMAND_MAXIMUM_PAGES => pages,
            _ => return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-page-cap"), "command batch exceeds its aggregate page authority"), command)),
        };
        let bytes = match self.bytes.checked_add(command.command.byte_len()) {
            Some(bytes) if bytes <= COMMAND_MAXIMUM_BYTES => bytes,
            _ => return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-byte-cap"), "command batch exceeds its aggregate assembled-byte authority"), command)),
        };
        if self.page_storage.try_reserve_exact(command.command.page_len()).is_err() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-page-allocation"), "command batch could not reserve the exact page slots this command declares"), command));
        }
        let CommandEnvelope { instance, seq, command } = command;
        let PagedCommand { pages: mut command_pages, kind, metadata, item_count, .. } = command;
        let page_count = u32::try_from(command_pages.len()).expect("admitted command page count is u32-bounded");
        self.commands.push_back(CommandBatchEntry { instance, seq, kind, metadata, item_count, page_count, remaining_pages: page_count });
        while let Some(page) = command_pages.pop_front() {
            self.page_storage.push_back(page);
        }
        self.pages = pages;
        self.bytes = bytes;
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.commands.front_mut() else {
            return (self.page_storage.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
            return (self.commands.is_empty() && self.page_storage.is_empty(), 0);
        }
        let Some(page_len) = self.page_storage.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.page_storage.pop_front().expect("command-build page was present");
        let released = page.len();
        self.pages -= 1;
        self.bytes -= released;
        command.remaining_pages -= 1;
        drop(page);
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
        }
        (self.commands.is_empty() && self.page_storage.is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.page_storage.is_empty() && self.pages == 0 && self.bytes == 0
    }
}

impl RejectedCommandBuild {
    pub fn new(admitted: CommandEnvelopeSet, rejected: CommandEnvelope) -> Self {
        Self { rejected: Some(rejected), admitted }
    }

    pub fn from_admitted(admitted: CommandEnvelopeSet) -> Self {
        Self { rejected: None, admitted }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let Some(rejected) = self.rejected.as_mut() {
            let Some((empty, released)) = rejected.command.release_front_page(maximum_bytes) else {
                return (false, 0);
            };
            if empty {
                let _terminal = self.rejected.take().expect("rejected command reached terminal empty");
            }
            return (self.terminal_is_empty(), released);
        }
        self.admitted.close_step(maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.rejected.is_none() && self.admitted.terminal_is_empty()
    }

    pub fn remaining_pages(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.page_len()) + self.admitted.pages
    }

    pub fn remaining_bytes(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.byte_len()) + self.admitted.bytes
    }
}

impl<const CAPACITY: usize> Default for RejectedCommandBuildRegistry<CAPACITY> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const CAPACITY: usize> RejectedCommandBuildRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0);
        Self { slots: std::array::from_fn(|_| None), close_index: 0, occupied: 0 }
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn try_insert(&mut self, key: u64, owner: RejectedCommandBuild) -> Result<(), (semio_framework_diagnostic::Fault, RejectedCommandBuild)> {
        let index = key as usize % CAPACITY;
        if self.slots[index].is_some() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-build-close-capacity"), "fixed rejected command-build close registry is occupied or collided"), owner));
        }
        self.slots[index] = Some(owner);
        self.occupied += 1;
        Ok(())
    }

    pub fn insert_admitted(&mut self, key: u64, owner: RejectedCommandBuild) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed rejected command-build admission changed before insert");
        self.slots[index] = Some(owner);
        self.occupied += 1;
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.occupied == 0 {
            return (true, 0, 0);
        }
        for _ in 0..CAPACITY {
            let index = self.close_index;
            self.close_index = (self.close_index + 1) % CAPACITY;
            let Some(owner) = self.slots[index].as_mut() else {
                continue;
            };
            let (terminal, released) = owner.close_step(maximum_bytes);
            if terminal {
                let terminal = self.slots[index].take().expect("terminal rejected command build was present");
                self.occupied -= 1;
                assert!(terminal.terminal_is_empty(), "rejected command build terminal witness changed before removal");
            }
            return (self.occupied == 0, 1, released);
        }
        (false, 0, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0
    }
}

impl CommandBatch {
    pub fn try_new(generation: u64, commands: CommandEnvelopeSet) -> Result<Self, (semio_framework_diagnostic::Fault, CommandEnvelopeSet)> {
        if commands.commands.is_empty() {
            return Err((semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-batch-cap"), "command batch requires at least one exact command owner"), commands));
        }
        Ok(Self { generation, commands: commands.commands, pages: commands.page_storage, bytes: commands.bytes })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.pages.is_empty() && self.bytes == 0
    }
}
//#endregion 🔖️PagedCommandIngress
