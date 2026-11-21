//! 📬️ Publishes only file-open capsules adopted by the exact native lifetime.

use semio_framework::kernel::Effect;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError};
use semio_plugin_decl_media::original_media::{
    OriginalMediaLoan, picker_args, issue::{decode_picker_effect, picker_args_capacity},
};
use semio_plugin_reactor_app::PluginApp;
use semio_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime;

const PUBLIC_CAPACITY: usize = 4096;

pub fn continue_original_picker_effects<PA: PluginApp>(
    runtime: &PluginRuntime<PA>, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let instances = runtime.instances.try_borrow().map_err(|_| ValueError::invariant("picker instance registry busy"))?;
    let mut runnable = false;
    let mut index = 0;
    while let Some((next, instance, entry)) = instances.next_entry_from(index) {
        if next < index { break; }
        index = next + 1;
        let Ok(mut active) = entry.instance.try_lock() else { continue; };
        if active.app.original_picker_effect().is_none() { continue; }
        let Ok(key) = runtime.live_instance_key(instance) else { continue; };
        let mut lifetimes = runtime.guest_lifetimes.try_borrow_mut().map_err(|_| ValueError::invariant("picker lifetime registry busy"))?;
        let slot = lifetimes.get_mut(instance).ok_or_else(|| ValueError::invariant("picker lifetime missing"))?;
        advance_picker(&mut active.app, slot, key, cx)?;
        runnable = true;
        if cx.should_yield() { break; }
    }
    Ok(runnable)
}

fn advance_picker<PA: PluginApp>(
    app: &mut PA, slot: &mut semio_plugin_reactor_lifecycle::instance_lifetime::NativeLifetimeSlot<PA>,
    key: semio_plugin_reactor_lifecycle::instance_lifetime::NativeCloseKey, cx: &mut StepContext<'_>,
) -> Result<bool, OriginalTurnFailure> {
    let effect = app.original_picker_effect().ok_or_else(|| ValueError::invariant("original picker outbox lost its head"))?;
    let (old_args, release) = effect_capacity(effect)?;
    let quote = RetainedCloneProgress { copied_items: 1, copied_bytes: PUBLIC_CAPACITY, retained_capacity_bytes: PUBLIC_CAPACITY,
        released_bytes: release, ..Default::default() };
    let grant = cx.retained_grant();
    if cx.should_yield() || cx.is_cancelled() || cx.fuel_remaining() == 0 || grant.maximum_depth < 2 || !quote.fits(grant) { return Ok(false); }
    let mut public = None;
    let mut discard = false;
    cx.with_retained_loan(semio_framework_value::RetainedCloneGrant {
        maximum_items: grant.maximum_items - quote.copied_items,
        maximum_copy_bytes: grant.maximum_copy_bytes - quote.copied_bytes,
        maximum_capacity_bytes: grant.maximum_capacity_bytes - quote.retained_capacity_bytes,
        maximum_release_bytes: grant.maximum_release_bytes - quote.released_bytes,
        ..grant
    }, |cx| cx.with_original_work_loan(cx.fuel_remaining() - 1, u64::MAX, |cx| {
        let effect = app.original_picker_effect().ok_or_else(|| ValueError::invariant("original picker outbox moved during adoption"))?;
        let decoded = decode_picker_effect(effect)?;
        let mut offered = None;
        if !decoded.prepare(&mut offered, cx)? { return Ok(()); }
        let issued = offered.as_ref().expect("prepared picker remains offered").loan();
        let (operation_id, generation, instance_id) = (issued.operation_id, issued.generation, issued.instance_id);
        let Some((document_id, revision)) = app.original_media_live_document() else {
            return Err(ValueError::invariant("original picker has no mounted document").into());
        };
        let live = OriginalMediaLoan { operation_id, generation, instance_id, document_id, base_revision: &revision };
        if key.instance() != instance_id || live != issued {
            discard = true;
            return Ok(());
        }
        if slot.admit_original_media(key, &mut offered, live, app.original_media_format_closed(), cx)? {
            public = picker_args(&slot.raw_media.picker);
        }
        Ok(())
    }))?;
    if discard {
        discard_effect(app, cx)?;
        return Ok(true);
    }
    let Some(args) = public else { return Ok(false); };
    let capacity = picker_args_capacity(&args);
    if capacity > PUBLIC_CAPACITY { return Err(ValueError::invariant("approved picker capsule escaped its fixed projection bound").into()); }
    let effect = app.original_picker_effect().ok_or_else(|| ValueError::invariant("approved picker lost its original effect"))?;
    let Effect::RequestFileOpen { args: host_args, read_as, .. } = effect else { return Err(ValueError::invariant("approved picker changed its effect kind").into()); };
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: capacity,
        retained_capacity_bytes: capacity, released_bytes: old_args + read_as.as_ref().map_or(0, String::capacity) , ..Default::default() })?;
    cx.consume_fuel(1);
    *host_args = Some(args);
    *read_as = None;
    Ok(true)
}

fn discard_effect<PA: PluginApp>(app: &mut PA, cx: &mut StepContext<'_>) -> Result<(), OriginalTurnFailure> {
    let effect = app.original_picker_effect().ok_or_else(|| ValueError::invariant("retired picker lost its hidden effect"))?;
    let (_, capacity) = effect_capacity(effect)?;
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<Option<Effect>>(),
        released_bytes: capacity, ..Default::default() })?;
    cx.consume_fuel(1);
    app.discard_original_picker_effect();
    Ok(())
}

fn effect_capacity(effect: &Effect) -> Result<(usize, usize), OriginalTurnFailure> {
    let Effect::RequestFileOpen { accept, read_as, import_action, args: Some(args), .. } = effect else {
        return Err(ValueError::invariant("original picker outbox has a foreign effect").into());
    };
    let args = picker_args_capacity(args);
    Ok((args, args + accept.capacity() + read_as.as_ref().map_or(0, String::capacity) + import_action.capacity()))
}
