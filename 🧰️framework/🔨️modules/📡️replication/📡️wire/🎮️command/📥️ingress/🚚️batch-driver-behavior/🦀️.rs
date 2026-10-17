//! 🚚️ Command batch driver behavior: the batch driver implementation. See [`super`].

use super::*;

//#region 🔖️PagedCommandIngress
impl CommandBatchDriver {
    pub fn new(owner: u64, batch: CommandBatch) -> Self {
        Self { owner, batch, command_index: 0, page_index: 0, admitted_page_count: 0, admitted_kind: 0, faulted: false, waiting: false }
    }

    pub fn next_page(&mut self) -> Result<Option<(CommandPageCursor, FixedCommandPage)>, semio_framework_diagnostic::Fault> {
        if self.faulted || self.waiting {
            return Ok(None);
        }
        let Some(command) = self.batch.commands.front() else {
            return Ok(None);
        };
        if self.admitted_page_count == 0 {
            self.admitted_page_count = command.page_count;
            self.admitted_kind = command.kind;
        }
        let bytes = self.batch.pages.front().ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-empty"), "nonterminal command owner has no page"))?.clone();
        let cursor = CommandPageCursor {
            owner: self.owner,
            generation: self.batch.generation,
            command_index: self.command_index,
            command_count: u32::try_from(self.batch.commands.len()).unwrap_or(u32::MAX).saturating_add(self.command_index),
            instance: command.instance,
            seq: command.seq,
            kind: self.admitted_kind,
            page_index: self.page_index,
            page_count: self.admitted_page_count,
            item_count: command.item_count,
            metadata: command.metadata,
        };
        Ok(Some((cursor, bytes)))
    }

    pub fn observe(&mut self, status: &CommandIngressStatus, maximum_release_bytes: usize) -> Result<CommandBatchProgress, semio_framework_diagnostic::Fault> {
        let (cursor, shape) = match status {
            CommandIngressStatus::Idle => {
                return Ok(if self.batch.commands.is_empty() {
                    CommandBatchProgress::Complete
                } else if self.faulted {
                    CommandBatchProgress::Faulted
                } else if self.waiting {
                    CommandBatchProgress::Waiting
                } else {
                    CommandBatchProgress::PageReady
                });
            }
            CommandIngressStatus::PageAccepted(cursor) | CommandIngressStatus::Backpressure(cursor) => (cursor, CursorShape::Page),
            CommandIngressStatus::CommandPending(cursor) | CommandIngressStatus::CommandComplete(cursor) => (cursor, CursorShape::Terminal),
            CommandIngressStatus::Fault { cursor, .. } => (cursor, CursorShape::Fault),
        };
        self.validate_cursor(cursor, shape)?;
        match status {
            CommandIngressStatus::Backpressure(_) => Ok(CommandBatchProgress::PageReady),
            CommandIngressStatus::PageAccepted(_) => {
                let Some(command) = self.batch.commands.front_mut() else {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact host owner"));
                };
                let page_len =
                    self.batch.pages.front().map(FixedCommandPage::len).ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact retained batch page"))?;
                if page_len > maximum_release_bytes {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-release-budget"), "accepted page exceeds its exact release grant"));
                }
                let page = self.batch.pages.pop_front().expect("accepted retained batch page was present");
                let released = page.len();
                self.batch.bytes -= released;
                drop(page);
                command.remaining_pages = command
                    .remaining_pages
                    .checked_sub(1)
                    .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-page-underflow"), "accepted page arrived after the retained command page owner was empty"))?;
                let empty = command.remaining_pages == 0;
                self.page_index = self.page_index.saturating_add(1);
                if empty {
                    self.waiting = true;
                    Ok(CommandBatchProgress::Waiting)
                } else {
                    Ok(CommandBatchProgress::PageReady)
                }
            }
            CommandIngressStatus::CommandPending(_) => {
                self.waiting = true;
                Ok(CommandBatchProgress::Waiting)
            }
            CommandIngressStatus::CommandComplete(_) => {
                let Some(command) = self.batch.commands.front() else {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact host owner"));
                };
                if command.remaining_pages > 1 {
                    return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-terminal-pages"), "terminal acknowledgement arrived before every exact page was released"));
                }
                if command.remaining_pages == 1 {
                    let page_len = self
                        .batch
                        .pages
                        .front()
                        .map(FixedCommandPage::len)
                        .ok_or_else(|| semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact retained batch page"))?;
                    if page_len > maximum_release_bytes {
                        return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-release-budget"), "terminal page exceeds its exact release grant"));
                    }
                    let page = self.batch.pages.pop_front().expect("terminal retained batch page was present");
                    self.batch.bytes -= page.len();
                    drop(page);
                }
                let _terminal = self.batch.commands.pop_front().expect("terminal command was present");
                self.command_index = self.command_index.saturating_add(1);
                self.page_index = 0;
                self.admitted_page_count = 0;
                self.admitted_kind = 0;
                self.waiting = false;
                Ok(if self.batch.commands.is_empty() { CommandBatchProgress::Complete } else { CommandBatchProgress::PageReady })
            }
            CommandIngressStatus::Fault { .. } => {
                self.faulted = true;
                Ok(CommandBatchProgress::Faulted)
            }
            CommandIngressStatus::Idle => unreachable!(),
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.batch.commands.front_mut() else {
            return (self.batch.pages.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.batch.commands.pop_front().expect("empty retained command shell was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
            return (self.batch.terminal_is_empty(), 0);
        }
        let Some(page_len) = self.batch.pages.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.batch.pages.pop_front().expect("retained batch close page was present");
        let released = page.len();
        self.batch.bytes -= released;
        drop(page);
        command.remaining_pages -= 1;
        let empty = command.remaining_pages == 0;
        if empty {
            let _terminal = self.batch.commands.pop_front().expect("empty command was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
        }
        (self.batch.terminal_is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.batch.terminal_is_empty()
    }

    pub fn generation(&self) -> u64 {
        self.batch.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.batch.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.batch.bytes
    }

    /// 🎯️ A terminal status carries `last_page_index + 1`, and a guest that consumed a command's LAST
    /// page and ran it to completion inside ONE turn publishes only that terminal — the per-page
    /// `PageAccepted` the host would otherwise have stepped through never exists. (`⚛️reactor/🔄️turn`'s
    /// single-page fast path does exactly this for every `page_count == 1` command, i.e. every ordinary
    /// mutation.) So a terminal cursor is validated against `page_index + remaining_pages`, which is
    /// the same number in both shapes, and a fault's cursor is accepted at either end of that range —
    /// a fault must attribute itself to its owner, never be replaced by a cursor complaint that hides
    /// the guest's own message (ticket 26/09/18 slice A2).
    fn validate_cursor(&self, cursor: &CommandPageCursor, shape: CursorShape) -> Result<(), semio_framework_diagnostic::Fault> {
        let Some(command) = self.batch.commands.front() else {
            return Err(semio_framework_diagnostic::Fault::new(semio_framework_diagnostic::FaultOrigin::Framework, semio_framework_diagnostic::FaultCode::new("plugin.command-owner-missing"), "ingress status has no exact host owner"));
        };
        let terminal_page_index = self.page_index.saturating_add(command.remaining_pages);
        let page_index_matches = match shape {
            CursorShape::Page => cursor.page_index == self.page_index,
            CursorShape::Terminal => cursor.page_index == terminal_page_index,
            CursorShape::Fault => cursor.page_index == self.page_index || cursor.page_index == terminal_page_index,
        };
        let mismatch = if cursor.owner != self.owner {
            Some(("owner", u64::from(cursor.owner), self.owner))
        } else if cursor.generation != self.batch.generation {
            Some(("generation", cursor.generation, self.batch.generation))
        } else if cursor.command_index != self.command_index {
            Some(("commandIndex", u64::from(cursor.command_index), u64::from(self.command_index)))
        } else if cursor.instance != command.instance {
            Some(("instance", u64::from(cursor.instance), u64::from(command.instance)))
        } else if cursor.seq != command.seq {
            Some(("seq", cursor.seq, command.seq))
        } else if cursor.kind != self.admitted_kind {
            Some(("kind", u64::from(cursor.kind), u64::from(self.admitted_kind)))
        } else if !page_index_matches {
            Some(("pageIndex", u64::from(cursor.page_index), u64::from(if matches!(shape, CursorShape::Page) { self.page_index } else { terminal_page_index })))
        } else if cursor.page_count != self.admitted_page_count {
            Some(("pageCount", u64::from(cursor.page_count), u64::from(self.admitted_page_count)))
        } else if cursor.item_count != command.item_count {
            Some(("itemCount", u64::from(cursor.item_count), u64::from(command.item_count)))
        } else if cursor.metadata != command.metadata {
            Some(("metadata", u64::from(cursor.metadata), u64::from(command.metadata)))
        } else {
            None
        };
        if let Some((field, reported, expected)) = mismatch {
            return Err(semio_framework_diagnostic::Fault::new(
                semio_framework_diagnostic::FaultOrigin::Framework,
                semio_framework_diagnostic::FaultCode::new("plugin.command-cursor-mismatch"),
                format!("ingress status does not identify the exact retained host owner: {field} is {reported}, the retained owner's is {expected}"),
            ));
        }
        Ok(())
    }
}
//#endregion 🔖️PagedCommandIngress
