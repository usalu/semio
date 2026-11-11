//! ⌨️ Command ingress phase of the reactor turn: the retained ingress owner, its cancellation by instance close, its resumption and the admission of one inbound command page. See [`super`].

use super::*;
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;
use semio_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework::kernel::{CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage};

/// ⌨️ The command ingress status, retained owner and exact lifetime one turn carries between its ingress steps.
pub(super) struct CommandIngressTurn {
    status: CommandIngressStatus,
    loan: CommandIngressLoan,
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
pub(super) async fn drive_command_ingress<PA: PluginApp>(runtime: &PluginRuntime<PA>, command_page: &mut Option<(CommandPageCursor, FixedCommandPage)>, close_instances: Vec<u32>, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<CommandIngressStatus, semio_framework_job::OriginalTurnFailure> {
    close_instance_command_origin(runtime,cx)?;
    let mut turn=std::pin::pin!(CommandIngressTurn{status:CommandIngressStatus::Idle,loan:CommandIngressLoan::empty(COMMAND_INGRESS_LIVE_SLOT),close_instances});
    if !turn.resume(runtime,command_page.is_some(),cx)?{let status=command_page.as_ref().map_or(CommandIngressStatus::Idle,|(cursor,_)|CommandIngressStatus::Backpressure(cursor.clone()));return Ok(command_ingress_while_owned(status,COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_LIVE_SLOT].peek().and_then(|entry|entry.state.as_ref()).map(command_ingress_owner_cursor))))}
    turn.cancel_for_close(cx)?;
    turn.readmit_reserved(runtime).await;
    turn.exchange_presence(runtime, effects, identity, cx).await?;
    turn.exchange_generic(runtime, effects, identity, cx).await?;
    if let Some((cursor,page)) = command_page.as_ref() {
        if command_page_invalid(runtime,cursor,page,&turn.close_instances){turn.status=CommandIngressStatus::Fault{cursor:cursor.clone(),fault:b"plugin.command-page-invalid".to_vec()};return turn.finish(cx)}
        let admitted={let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|reactor_close_fault("command Source lifetime is busy"))?;let slot=lifetimes.get_mut(cursor.instance).ok_or_else(||reactor_close_fault("command Source lifetime is missing"))?;slot.command_origin.admit_step(cx)?};
        if !admitted{turn.status=CommandIngressStatus::Backpressure(cursor.clone());return turn.finish(cx)}
        turn.admit_page(runtime,command_page, effects, identity, cx).await?;
    }
    turn.finish(cx)
}

fn close_instance_command_origin<PA:PluginApp>(runtime:&PluginRuntime<PA>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
 use semio_plugin_reactor_lifecycle::instance_lifetime::Phase;
 let mut lifetimes=runtime.guest_lifetimes.try_borrow_mut().map_err(|_|reactor_close_fault("command origin close lifetime is busy"))?;
 for index in 0..lifetimes.slots.values.len(){let Some(slot)=lifetimes.slots.get_mut(index)else{continue};if !matches!(slot.cell.phase,Phase::Closing|Phase::Retired)||slot.command_origin.terminal_is_empty(){continue}slot.close_command_origin(&ReactorClosePeers,cx)?;break}Ok(())
}

impl CommandIngressTurn {
    /// 🔁️ Takes the retained owner of the live slot and marks its instance for cancellation when its lifetime is no longer the exact retained one.
    fn resume<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,allow_empty:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        step_retiring_command_ingress(cx)?;
        let ready=COMMAND_INGRESS.with(|slots|slots.borrow_mut()[COMMAND_INGRESS_LIVE_SLOT].settle_step(cx))?;if !ready{return Ok(false)}
        if !allow_empty&&COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_LIVE_SLOT].peek().is_none()){return Ok(false)}
        if let Some(key)=COMMAND_INGRESS.with(|slots|slots.borrow()[COMMAND_INGRESS_LIVE_SLOT].peek().map(|entry|entry.key)){runtime.guest_lifetimes.try_borrow().map_err(|_|reactor_close_fault("command Source lifetime is busy"))?.get(key.instance()).ok_or_else(||reactor_close_fault("retained command Source lifetime is missing"))?.command_origin.validate(cx)?;}
        if !self.loan.lend(cx)?{return Ok(false)}
        if let Some(key)=self.loan.key(){if native_close_key(runtime,key.instance()).ok()!=Some(key){self.close_instances.push(key.instance());}}
        Ok(true)
    }

    /// 🚫️ Cancels the retained owner of every instance closing this turn and steps an owner that is already closing.
    fn cancel_for_close(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some(owner)=self.loan.state()else{return Ok(())};let cursor=command_ingress_owner_cursor(owner);
        if self.close_instances.contains(&cursor.instance)||matches!(owner,CommandIngressOwner::ClosingAssembly{..}){
            self.status=semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
            if let Some(slot)=self.loan.state_slot(){retire_command_ingress(slot,cx)?;if slot.is_none(){self.status=semio_framework::kernel::CommandIngressStatus::Fault{cursor,fault:b"plugin.command-cancelled-by-close".to_vec()};}}
        }Ok(())
    }

    /// 🎟️ Offers a reserved presence admission or a pending presence page to its owner once more.
    async fn readmit_reserved<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>) {
        let previous=self.loan.take_state();self.loan.put_state(match previous {
            Some(CommandIngressOwner::ReservedPresence { cursor, admission, page }) => {
                let now_ms = semio_plugin_actor_host::now_ms().await;
                match semio_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, if cursor.metadata & 0x100 != 0 { Some((cursor.metadata & 0xff) as u8) } else { None }, cursor.item_count, page, now_ms).await {
                    Ok(publication_generation) => {
                        self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                        Some(CommandIngressOwner::Presence { cursor, publication_generation,exchange:None,exchange_close:None,construction:Default::default(),terminal:false,terminal_fault:None })
                    }
                    Err((_fault, admission, page)) => {
                        self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                        Some(CommandIngressOwner::ReservedPresence { cursor, admission, page })
                    }
                }
            }
            owner => owner,
        });
        let previous=self.loan.take_state();self.loan.put_state(match previous {
            Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page }) => match semio_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                Ok(()) => {
                    self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor.clone());
                    Some(CommandIngressOwner::Presence { cursor, publication_generation,exchange:None,exchange_close:None,construction:Default::default(),terminal:false,terminal_fault:None })
                }
                Err((_fault, page)) => {
                    self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor.clone());
                    Some(CommandIngressOwner::PendingPresencePage { cursor, publication_generation, page })
                }
            },
            owner => owner,
        });
    }

    /// 👥️ Exchanges one presence page of the retained presence owner when the turn has no other status.
    async fn exchange_presence<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        if !matches!(self.status,CommandIngressStatus::Idle){return Ok(())}let Some(CommandIngressOwner::Presence{cursor,exchange,exchange_close,construction,terminal,terminal_fault,..})=self.loan.state_mut()else{return Ok(())};let cursor=cursor.clone();
        if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}
        if *terminal{let fault=terminal_fault.take();self.status=terminal_command_ingress(cursor,fault);self.loan.put_state(None);return Ok(())}
        if !semio_framework_job::admit_original_construction_settled(cx,construction,exchange,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:std::mem::size_of_val(exchange),capacity_bytes:0,release_bytes:0,depth:1},1,|_,_|semio_framework_value::RetainedWorkOutcome{result:Ok((semio_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState::empty(),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<semio_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState>>(),..Default::default()})),work:1})?{return Ok(())}
        let mut output=plugin_exchange_turn(runtime,cursor.instance,&mut None,exchange.as_mut().expect("presence retains its Source output state"),identity,cx).await?;
        if output.presence_terminal==Some(cursor.seq){*terminal=true;*terminal_fault=output.presence_terminal_fault.take();}
        route_exchange_output(cursor.instance,output,effects,cx)
    }

    /// 🔁️ Retries the retained generic command with the instance that owns it.
    async fn exchange_generic<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some(CommandIngressOwner::Generic{cursor,command,exchange,exchange_close,output,..})=self.loan.state_mut()else{return Ok(())};let cursor=cursor.clone();
        self.status=match advance_command_cursor(cursor.clone()){Ok(terminal)=>CommandIngressStatus::CommandPending(terminal),Err(cursor)=>CommandIngressStatus::Fault{cursor,fault:b"plugin.command-page-index-exhausted".to_vec()}};
        if matches!(command.as_ref(),Some((_,semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::FailedDecoded{..}))){if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}let Some((seq,failed))=command.take()else{unreachable!()};match failed.retire_failed_step_on_original(cx){semio_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Refused{owner,failure}=>{*command=Some((seq,owner));return Err(failure)},semio_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Pending(failed)=>*command=Some((seq,failed)),semio_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Failed(failure)=>return Err(failure),semio_plugin_runtime_ops::ops_ingress::OriginalCommandIngressRetirement::Closed=>{}}return Ok(())}
        if output.is_none(){let state=exchange.as_mut().expect("retained Source exchange state precedes command execution");match plugin_exchange_turn(runtime,cursor.instance,command,state,identity,cx).await{
            Ok(produced)=>{if state.applied{*output=Some(produced)}else{return route_exchange_output(cursor.instance,produced,effects,cx)}},
            Err(failure)=>{if matches!(command.as_ref(),Some((_,semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Decoded(_)))){semio_plugin_runtime_exchange::exch_plugin_exchange::retain_decoded_exchange_failure(command,Some(failure),(cx.operation(),cx.generation()),None);return Ok(())}return Err(failure)}
        }}
        if !close_exchange_owner(exchange,exchange_close,cx)?{return Ok(())}
        if let Some((_,semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Decoded(owner)))=command.as_mut(){if !cx.with_original_retirement(|cx|owner.close_original_step(cx))?{return Ok(())}}
        if command.is_some(){let bytes=std::mem::size_of_val(command);let grant=cx.retained_grant();if cx.retirement_should_yield()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(())}command.take();cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);}
        let mut produced=output.take().expect("actual semantic output survives controlled input retirement");self.status=terminal_command_ingress(cursor.clone(),produced.command_terminal_fault.take());self.loan.put_state(None);route_exchange_output(cursor.instance,produced,effects,cx)
    }

    /// 📄️ Admits one inbound command page against the retained owner: invalid, superseded, presence, first generic page or assembly continuation.
    async fn admit_page<PA: PluginApp>(&mut self, runtime: &PluginRuntime<PA>, incoming: &mut Option<(CommandPageCursor, FixedCommandPage)>, effects: &mut Vec<Effect>, identity: &mut EntityIdentityAuthority<'_>, cx: &mut semio_framework_job::StepContext<'_>) -> Result<(), semio_framework_job::OriginalTurnFailure> {
        let Some((cursor,page))=incoming.as_ref()else{return Ok(())};let cursor=cursor.clone();
        if self.loan.state().is_none(){let key=native_close_key(runtime,cursor.instance)?;if !self.loan.admit_entry(key,cx)?{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}}
        if command_page_invalid(runtime, &cursor, &page, &self.close_instances) {
            self.status = semio_framework::kernel::CommandIngressStatus::Fault { cursor, fault: b"plugin.command-page-invalid".to_vec() };
        } else if self.loan.state().is_some_and(|owner| !same_command_cursor(&command_ingress_owner_cursor(owner), &cursor)) {
            if self.loan.state().is_some_and(|owner| command_ingress_owner_cursor(owner).instance == cursor.instance) && retiring_command_ingress_is_free() {
                let key=self.loan.key().expect("a retained owner holds its exact lifetime");park_retiring_command_ingress(key,self.loan.state_slot().expect("the superseded owner retains its Source home"),cx)?;
            }
            self.status = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor);
        } else if matches!(self.loan.state(), Some(CommandIngressOwner::ReservedPresence { .. } | CommandIngressOwner::PendingPresencePage { .. })) {
            self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
        } else if cursor.kind == 28 {
            self.admit_presence_page(runtime,incoming.take().expect("validated presence input remains caller owned")).await;
        } else if cursor.page_index == 0 && self.loan.state().is_none() {
            self.admit_first_generic_page(runtime,incoming, effects, identity, cx).await?;
        } else if let Some(CommandIngressOwner::GenericAssembly { cursor: active, pages }) = {
            match self.loan.state() {
                Some(CommandIngressOwner::GenericAssembly { .. }) => self.loan.take_state(),
                _ => None,
            }
        } {
            self.continue_generic_assembly(runtime,incoming, (active, pages), effects, identity, cx).await?;
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
            if cursor.page_index == 0 && self.loan.state().is_none() {
                match semio_plugin_runtime_ops::ops_typed_operations::plugin_reserve_presence_ingress(runtime, cursor.instance, cursor.seq).await {
                    Ok(admission) => {
                        let now_ms = semio_plugin_actor_host::now_ms().await;
                        match semio_plugin_runtime_ops::ops_typed_operations::plugin_admit_reserved_presence(runtime, cursor.instance, admission, own_color, cursor.item_count, page, now_ms).await {
                            Ok(publication_generation) => {
                                self.loan.put_state(Some(CommandIngressOwner::Presence { cursor:cursor.clone(),publication_generation,exchange:None,exchange_close:None,construction:Default::default(),terminal:false,terminal_fault:None }));
                                self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                            }
                            Err((_fault, admission, page)) => {
                                self.loan.put_state(Some(CommandIngressOwner::ReservedPresence { cursor: cursor.clone(), admission, page }));
                                self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                            }
                        }
                    }
                    Err(_) => self.status = semio_framework::kernel::CommandIngressStatus::Backpressure(cursor),
                }
            } else if let Some(publication_generation) = self.loan.state().and_then(|owner| match owner {
                CommandIngressOwner::Presence { publication_generation, .. } => Some(*publication_generation),
                _ => None,
            }) {
                match semio_plugin_runtime_ops::ops_typed_operations::plugin_push_reserved_presence_page(runtime, cursor.instance, publication_generation, cursor.page_index, page).await {
                    Ok(()) => {
                        if let Some(CommandIngressOwner::Presence { cursor: active, .. }) = self.loan.state_mut() {
                            active.page_index = cursor.page_index;
                        }
                        self.status = semio_framework::kernel::CommandIngressStatus::PageAccepted(cursor);
                    }
                    Err((_fault, page)) => {
                        self.loan.put_state(Some(CommandIngressOwner::PendingPresencePage { cursor: cursor.clone(), publication_generation, page }));
                        self.status = semio_framework::kernel::CommandIngressStatus::CommandPending(cursor);
                    }
                }
            }
        }
    }

    /// 🧱️ Admits the first page of a generic command and exchanges it when it is the only page.
    async fn admit_first_generic_page<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,incoming:&mut Option<(CommandPageCursor,FixedCommandPage)>,effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let Some((cursor,_))=incoming.as_ref()else{return Ok(())};let cursor=cursor.clone();
        if cx.should_yield()||cx.fuel_remaining()==0{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        let mut pages=None;let turn=CommandPageSet::admit_source_step(&mut pages,cursor.page_count as usize,cx.retained_grant());
        self.loan.put_state(pages.map(|pages|CommandIngressOwner::GenericAssembly{cursor:cursor.clone(),pages}));
        let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));cx.consume_retained(progress)?;turn.result?;
        let Some(CommandIngressOwner::GenericAssembly{cursor:active,pages})=self.loan.take_state()else{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())};
        self.continue_generic_assembly(runtime,incoming,(active,pages),effects,identity,cx).await
    }

    async fn continue_generic_assembly<PA:PluginApp>(&mut self,runtime:&PluginRuntime<PA>,incoming:&mut Option<(CommandPageCursor,FixedCommandPage)>,assembly:(CommandPageCursor,CommandPageSet),effects:&mut Vec<Effect>,identity:&mut EntityIdentityAuthority<'_>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(),semio_framework_job::OriginalTurnFailure>{
        let(active,pages)=assembly;self.loan.put_state(Some(CommandIngressOwner::GenericAssembly{cursor:active,pages}));
        let Some((cursor,_))=incoming.as_ref()else{return Ok(())};let cursor=cursor.clone();
        let Some(CommandIngressOwner::GenericAssembly{pages,..})=self.loan.state_mut()else{unreachable!()};
        if cursor.page_index as usize!=pages.len(){self.status=CommandIngressStatus::Fault{cursor,fault:b"plugin.command-page-order".to_vec()};return Ok(())}
        if cx.should_yield()||cx.fuel_remaining()==0{self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        let turn=pages.capture_source_step(incoming,cx.retained_grant());let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};cx.consume_fuel(u64::try_from(turn.work).unwrap_or(u64::MAX));cx.consume_retained(progress)?;turn.result?;
        if incoming.is_some(){self.status=CommandIngressStatus::Backpressure(cursor);return Ok(())}
        if let Some(CommandIngressOwner::GenericAssembly{cursor:active,..})=self.loan.state_mut(){active.page_index=cursor.page_index;}
        self.status=CommandIngressStatus::PageAccepted(cursor.clone());
        if cursor.page_index.checked_add(1)!=Some(cursor.page_count){return Ok(())}
        let Some(CommandIngressOwner::GenericAssembly{pages,..})=self.loan.take_state()else{unreachable!()};
        match semio_framework::kernel::PagedCommand::try_from_pages(pages){Ok(command)=>self.loan.put_state(Some(CommandIngressOwner::Generic{cursor:cursor.clone(),command:Some((cursor.seq,semio_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(command))),exchange:Some(semio_plugin_runtime_ops::ops_ingress::OriginalCommandExchangeState::empty()),exchange_close:None,output:None,output_close:None})),Err((fault,pages))=>{self.loan.put_state(Some(CommandIngressOwner::GenericAssembly{cursor:cursor.clone(),pages}));self.status=CommandIngressStatus::Fault{cursor,fault:semio_framework_diagnostic::encode_fault_bytes(&fault)};return Ok(())}}
        self.exchange_generic(runtime,effects,identity,cx).await
    }

    /// 🏁️ Reports the status while an owner is retained and parks the owner in the live slot with its exact lifetime.
    fn finish(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<CommandIngressStatus,semio_framework_job::OriginalTurnFailure>{
        let status=std::mem::replace(&mut self.status,CommandIngressStatus::Idle);let status=command_ingress_while_owned(status,self.loan.state().map(command_ingress_owner_cursor));self.loan.clear_empty_entry(cx)?;self.loan.restore()?;Ok(status)
    }
}
