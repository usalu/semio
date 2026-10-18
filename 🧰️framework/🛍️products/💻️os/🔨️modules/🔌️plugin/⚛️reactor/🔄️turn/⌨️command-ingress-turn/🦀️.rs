//! ⌨️ Command ingress phase of the reactor turn: the retained ingress owner, its cancellation by instance close, its resumption and the admission of one inbound command page. See [`super`].

use super::*;
use semio_framework_plugin_reactor_app::PluginApp;
use semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework::kernel::{CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage};

/// ⌨️ The command ingress status, retained owner and exact lifetime one turn carries between its ingress steps.
pub(super) struct CommandIngressTurn {
    status: CommandIngressStatus,
    retained: Option<CommandIngressOwner>,
    key: Option<instance_lifetime::NativeCloseKey>,
    close_instances: Vec<u32>,
}

/// 🧾️ Whether a command page violates the fixed page, command or presence limits, its instance is not live or its instance is closing.
fn command_page_invalid<PA: PluginApp>(runtime: &PluginRuntime<PA>, cursor: &CommandPageCursor, page: &FixedCommandPage, close_instances: &[u32]) -> bool {
    !runtime.guest_lifetimes.borrow().get(cursor.instance).is_some_and(|slot| slot.cell.is_live())
        || close_instances.contains(&cursor.instance)
        || page.len() > semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
        || cursor.page_count == 0
        || cursor.page_count as usize > semio_framework::kernel::COMMAND_MAXIMUM_PAGES
        || cursor.command_count == 0
        || cursor.command_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
        || cursor.command_index >= cursor.command_count
        || cursor.page_index >= cursor.page_count
        || cursor.item_count as usize > semio_framework::kernel::COMMAND_BATCH_MAXIMUM_ITEMS
        || (cursor.kind == 28 && cursor.page_count != cursor.item_count.max(1))
        || (cursor.kind == 28 && ((cursor.item_count == 0) != page.is_empty()))
        || (cursor.kind != 28
            && (page.is_empty()
                || cursor.item_count != 0
                || cursor.metadata != 0
                || (cursor.page_index == 0 && cursor.kind != page.as_slice()[0])
                || (cursor.page_index.checked_add(1).is_some_and(|next| next < cursor.page_count) && page.len() != semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)))
}

/// ⌨️ Runs the whole command ingress step of one turn and returns the status the turn reports.
///
/// 📥️ A page that names a different command on THIS instance is the host saying it has
/// moved on: it drives one command per instance at a time, so the owner retained here can
/// no longer have a driver. It is superseded into the retiring lane (see
/// `COMMAND_INGRESS_RETIRING_SLOT`) and the host is asked for this page once more, which
/// the now-free live lane admits on the next turn. A page for ANOTHER instance is genuine
/// backpressure — that owner's host driver is still live — and waits as it always did.
///
/// 📥️ Taken only when it IS the assembly this page belongs to. An `if let` on a bare
/// `retained.take()` dropped every other owner shape on the floor — the page went with it,
/// the turn still answered `Idle`, and the host's drain then spun its whole continuation
/// ceiling asking a reactor that no longer owned anything.
///
/// 📥️ The terminal arm this chain never had: a page that matches no admission shape is a
/// page nobody owns, and it says so by name instead of leaving the turn `Idle` with the
/// page silently gone.
pub(super) async fn drive_command_ingress<PA: PluginApp>(runtime: &PluginRuntime<PA>, command_page: Option<(CommandPageCursor, FixedCommandPage)>, close_instances: Vec<u32>, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<CommandIngressStatus, semio_framework_job::OriginalTurnFailure> {
    let mut turn = CommandIngressTurn::resume(runtime, close_instances, cx)?;
    turn.cancel_for_close();
    turn.readmit_reserved(runtime).await;
    turn.exchange_presence(runtime, effects, identity, cx).await?;
    turn.exchange_generic(runtime, effects, identity, cx).await?;
    if let Some(incoming) = command_page {
        turn.admit_page(runtime, incoming, effects, identity, cx).await?;
    }
    Ok(turn.finish())
}

impl CommandIngressTurn {
    /// 🔁️ Takes the retained owner of the live slot and marks its instance for cancellation when its lifetime is no longer the exact retained one.
    fn resume<PA: PluginApp>(runtime: &PluginRuntime<PA>, mut close_instances: Vec<u32>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Self, semio_framework_job::OriginalTurnFailure> {
        let status = CommandIngressStatus::Idle;
        step_retiring_command_ingress(cx)?;
        let (retained, retained_key) = COMMAND_INGRESS.with(|ingress| match ingress.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT].take() {
            Some(entry) => (Some(entry.state), Some(entry.key)),
            None => (None, None),
        });
        if let Some(key) = retained_key {
            if native_close_key(runtime, key.instance()).ok() != Some(key) {
                close_instances.push(key.instance());
            }
        }
        Ok(Self { status, retained, key: retained_key, close_instances })
    }

    /// 🚫️ Cancels the retained owner of every instance closing this turn and steps an owner that is already closing.
    fn cancel_for_close(&mut self) {
        self.retained = match self.retained.take() {
            Some(CommandIngressOwner::ReservedPresence { cursor, admission, .. }) if self.close_instances.contains(&cursor.instance) => {
                admission.cancel.cancel_now();
                self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
            }
            Some(CommandIngressOwner::PendingPresencePage { cursor, .. }) if self.close_instances.contains(&cursor.instance) => {
                self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                None
            }
            Some(CommandIngressOwner::Generic { cursor, command }) if self.close_instances.contains(&cursor.instance) => {
                self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::Generic {
                    cursor,
                    command: command.cancel(semio_framework::Fault::new(semio_framework::FaultOrigin::Framework, semio_framework::FaultCode::new("plugin.command-cancelled-by-close"), "command ingress was cancelled by instance close")),
                })
            }
            Some(CommandIngressOwner::GenericAssembly { cursor, pages }) if self.close_instances.contains(&cursor.instance) => {
                self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
            }
            owner => owner,
        };
        self.retained = match self.retained.take() {
            Some(CommandIngressOwner::ClosingAssembly { cursor, mut pages }) => {
                let (complete, _) = pages.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                if complete {
                    self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                    None
                } else {
                    self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::ClosingAssembly { cursor, pages })
                }
            }
            owner => owner,
        };
    }

    /// 🎟️ Offers a reserved presence admission or a pending presence page to its owner once more.
    async fn readmit_reserved<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>) {
        self.retained = match self.retained.take() {
            Some(CommandIngressOwner::ReservedPresence { cursor, admission, page }) => {
                let now_ms = semio_framework_plugin_actor_host::now_ms().await;
                match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None }, cursor.item_count, page, now_ms).await {
                    Ok(publication_generation) => {
                        self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                        Some(CommandIngressOwner::Presence { cursor, publication_generation })
                    }
                    Err((_fault, admission, page)) => {
                        self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                        Some(CommandIngressOwner::ReservedPresence { cursor, admission, page })
                    }
                }
            }
            owner => owner,
        };
        self.retained = match self.retained.take() {
            Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                Ok(()) => {
                    self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                    Some(CommandIngressOwner::Presence { cursor, publication_generation })
                }
                Err((_fault, page)) => {
                    self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
                }
            },
            owner => owner,
        };
    }

    /// 👥️ Exchanges one presence page of the retained presence owner when the turn has no other status.
    async fn exchange_presence<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        if matches!(self.status, semio_framework::kernel::CommandIngressStatus::Idle) {
            if let Some(CommandIngressOwner::Presence { cursor, .. }) = self.retained.as_ref() {
                let cursor = cursor.clone();
                if self.close_instances.contains(&cursor.instance) {
                    self.retained = None;
                    self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-cancelled-by-close".to_vec() };
                } else {
                    match plugin_exchange_boxed(runtime, cursor.instance, None, identity,cx).await {
                        Ok(output) => {
                            let instance = cursor.instance;
                            if output.presence_terminal == Some(cursor.seq) {
                                self.retained = None;
                                self.status = match advance_command_cursor(cursor) {
                                    Ok(terminal) => match output.presence_terminal_fault.as_ref() {
                                        Some(fault) => semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() },
                                        None => semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal),
                                    },
                                    Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                };
                            } else if cursor.page_index.checked_add(1) == Some(cursor.page_count) {
                                self.status = match advance_command_cursor(cursor.clone()) {
                                    Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                    Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                };
                            }
                            route_exchange_output(instance, output, &mut *effects,cx)?;
                        }
                        Err(failure) => return Err(failure),
                    }
                }
            }
        }
        Ok(())
    }

    /// 🔁️ Retries the retained generic command with the instance that owns it.
    async fn exchange_generic<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        if let Some(owner) = self.retained.take() {
            match owner {
                CommandIngressOwner::Generic { cursor, command } => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, command)), identity,cx).await {
                    Ok(mut output) => {
                        match advance_command_cursor(cursor.clone()) {
                            Ok(terminal) => {
                                if let Some((_, command)) = output.retry_command.take() {
                                    self.retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                    self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(terminal);
                                } else if let Some(fault) = output.command_terminal_fault.as_ref() {
                                    self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor: terminal, fault: fault.clone() };
                                } else {
                                    self.status = semio_framework::kernel::CommandIngressStatus::CommandComplete(terminal);
                                }
                            }
                            Err(cursor) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                        }
                        route_exchange_output(cursor.instance, output, &mut *effects,cx)?;
                    }
                    Err(failure) => return Err(failure),
                },
                owner => self.retained = Some(owner),
            }
        }
        Ok(())
    }

    /// 📄️ Admits one inbound command page against the retained owner: invalid, superseded, presence, first generic page or assembly continuation.
    async fn admit_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let (cursor, page) = incoming;
        if self.retained.is_none() {
            self.key = native_close_key(runtime, cursor.instance).ok();
        }
        if command_page_invalid(runtime, &cursor, &page, &self.close_instances) {
            self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-invalid".to_vec() };
        } else if self.retained.as_ref().is_some_and(|owner| !same_command_cursor(&command_ingress_owner_cursor(owner), &cursor)) {
            if self.retained.as_ref().is_some_and(|owner| command_ingress_owner_cursor(owner).instance == cursor.instance) && retiring_command_ingress_is_free() {
                park_retiring_command_ingress(self.key.expect("a retained owner holds its exact lifetime"), self.retained.take().expect("the superseded owner was retained"),cx)?;
            }
            self.status = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor);
        } else if matches!(self.retained, Some(CommandIngressOwner::ReservedPresence { .. } | CommandIngressOwner::PendingPresencePage { .. })) {
            self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
        } else if cursor.kind == 28 {
            self.admit_presence_page(runtime, (cursor, page)).await;
        } else if cursor.page_index == 0 && self.retained.is_none() {
            self.admit_first_generic_page(runtime, (cursor, page), effects, identity, cx).await?;
        } else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, pages }) = {
            match self.retained {
                Some(CommandIngressOwner::GenericAssembly { .. }) => self.retained.take(),
                _ => None,
            }
        } {
            self.continue_generic_assembly(runtime, (cursor, page), (active, pages), effects, identity, cx).await?;
        } else {
            self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-unowned".to_vec() };
        }
        Ok(())
    }

    /// 👥️ Admits one presence ingress page: the first reserves the instance's presence admission, later pages push into the published generation.
    async fn admit_presence_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage)) {
        let (cursor, page) = incoming;
        let own_color = if cursor.metadata & !0x1ff != 0 {
            self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor: cursor.clone(), fault: b"plugin.command-presence-metadata".to_vec() };
            None
        } else if cursor.metadata & 0x100 != 0 {
            Some((cursor.metadata & 0xff) as u8)
        } else {
            None
        };
        if !matches!(self.status, semio_framework::kernel::CommandIngressStatus::Fault { .. }) {
            if cursor.page_index == 0 && self.retained.is_none() {
                match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq).await {
                    Ok(admission) => {
                        let now_ms = semio_framework_plugin_actor_host::now_ms().await;
                        match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, own_color, cursor.item_count, page, now_ms).await {
                            Ok(publication_generation) => {
                                self.retained = Some(CommandIngressOwner::Presence { cursor: cursor.clone(), publication_generation });
                                self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                            }
                            Err((_fault, admission, page)) => {
                                self.retained = Some(CommandIngressOwner::ReservedPresence { cursor: cursor.clone(), admission, page });
                                self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                            }
                        }
                    }
                    Err(_) => self.status = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
                }
            } else if let Some(publication_generation) = self.retained.as_ref().and_then(|owner| match owner {
                CommandIngressOwner::Presence { publication_generation, .. } => Some(*publication_generation),
                _ => None,
            }) {
                match semio_framework_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                    Ok(()) => {
                        if let Some(CommandIngressOwner::Presence { cursor: active, .. }) = self.retained.as_mut() {
                            active.page_index = cursor.page_index;
                        }
                        self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                    Err((_fault, page)) => {
                        self.retained = Some(CommandIngressOwner::PendingPresencePage { cursor: cursor.clone(), publication_generation, page });
                        self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                    }
                }
            }
        }
    }

    /// 🧱️ Admits the first page of a generic command and exchanges it when it is the only page.
    async fn admit_first_generic_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let (cursor, page) = incoming;
        match semio_framework::kernel::CommandPageSet::try_new(cursor.page_count as usize) {
            Ok(mut pages) => match pages.try_push(page) {
                Err((fault, _page)) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                Ok(()) if cursor.page_count == 1 => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                    Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command))), identity,cx).await {
                        Ok(mut output) => {
                            if let Some((_, command)) = output.retry_command.take() {
                                self.retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                self.status = match advance_command_cursor(cursor.clone()) {
                                    Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                    Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                };
                            } else {
                                self.status = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                            }
                            route_exchange_output(cursor.instance, output, &mut *effects,cx)?;
                        }
                        Err(failure) => return Err(failure),
                    },
                    Err((fault, _pages)) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                },
                Ok(()) => {
                    self.retained = Some(CommandIngressOwner::GenericAssembly { cursor: cursor.clone(), pages });
                    self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                }
            },
            Err(fault) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
        }
        Ok(())
    }

    /// 🧩️ Appends a later page to the retained assembly and exchanges the command once its last page arrives.
    async fn continue_generic_assembly<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: (CommandPageCursor, FixedCommandPage), assembly: (CommandPageCursor, CommandPageSet), effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let (cursor, page) = incoming;
        let (active, mut pages) = assembly;
        if cursor.page_index as usize != pages.len() {
            self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-order".to_vec() };
        } else {
            match pages.try_push(page) {
                Err((fault, _page)) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                Ok(()) if cursor.page_index.checked_add(1) == Some(cursor.page_count) => match semio_framework::kernel::PagedCommand::try_from_pages(pages) {
                    Ok(command) => match plugin_exchange_boxed(runtime, cursor.instance, Some((cursor.seq, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command))), identity,cx).await {
                        Ok(mut output) => {
                            if let Some((_, command)) = output.retry_command.take() {
                                self.retained = Some(CommandIngressOwner::Generic { cursor: cursor.clone(), command });
                                self.status = match advance_command_cursor(cursor.clone()) {
                                    Ok(pending) => semio_framework::kernel::CommandIngressStatus::CommandPending(pending),
                                    Err(cursor) => semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-index-exhausted".to_vec() },
                                };
                            } else {
                                self.status = terminal_command_ingress(cursor.clone(), output.command_terminal_fault.take());
                            }
                            route_exchange_output(cursor.instance, output, &mut *effects,cx)?;
                        }
                        Err(failure) => return Err(failure),
                    },
                    Err((fault, _pages)) => self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: semio_framework_diagnostic::encode_fault_bytes(&fault) },
                },
                Ok(()) => {
                    self.retained = Some(CommandIngressOwner::GenericAssembly { cursor: active, pages });
                    if let Some(CommandIngressOwner::GenericAssembly { cursor: active, .. }) = self.retained.as_mut() {
                        active.page_index = cursor.page_index;
                    }
                    self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                }
            }
        }
        Ok(())
    }

    /// 🏁️ Reports the status while an owner is retained and parks the owner in the live slot with its exact lifetime.
    fn finish(mut self) -> CommandIngressStatus {
        self.status = command_ingress_while_owned(self.status, self.retained.as_ref().map(command_ingress_owner_cursor));
        if let Some(retained) = self.retained {
            COMMAND_INGRESS.with(|ingress| {
                ingress.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT] = Some(RetainedCommandIngress { key: self.key.expect("admitted command retains its exact lifetime"), state: retained });
            });
        }
        self.status
    }
}
