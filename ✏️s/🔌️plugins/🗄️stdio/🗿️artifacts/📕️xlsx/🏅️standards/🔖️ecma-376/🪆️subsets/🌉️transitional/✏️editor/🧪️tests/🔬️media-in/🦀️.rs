use super::{create_xlsx_transitional_editor, XlsxTransitionalEditor};
use semio_plugin::ArtifactEditor;

/// ⛽️ The fuel one shard turn grants in the browser.
const SHARD_TURN_FUEL: u64 = 50_000_000;
/// 🔢️ Turns a lifecycle loop may spend before the law calls it stuck.
const SHARD_STALL_TURNS: usize = 4_096;
/// 🔢️ Host turns the whole `AppCommand::MediaIn` of a small workbook may need until its command completes.
const SHARD_MEDIA_IN_TURN_LIMIT: usize = 512;

thread_local! { static TURN_WALL_US: std::cell::Cell<Option<u64>> = const { std::cell::Cell::new(None) }; }

type ShardApp = semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<XlsxTransitionalEditor>>;

/// 🧾️ What one reactor turn of the shard-driven open reported.
#[derive(Debug)]
struct ShardTurn { ingress: String, status: String, sources: String, spent: u128, fuel_used: u64, retained: semio_framework_value::RetainedCloneProgress }

/// 🔄️ One reactor turn exactly as a shard grants it: the original caller's context renewed with the shard's fuel (armed through `arm_host_turn_fuel`) and grant, the fixture's retained input, one optional lifecycle event and one optional command page.
async fn shard_turn<PA: semio_plugin::PluginApp>(runtime: &semio_plugin::plugin_runtime::PluginRuntime<PA>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, event: &mut Option<semio_plugin::kernel::Event>, page: &mut Option<(semio_plugin::kernel::CommandPageCursor, semio_plugin::kernel::FixedCommandPage)>) -> Option<(semio_plugin::kernel::TurnResult, ShardTurn)> {
    use semio_plugin::{artifact_app_laws as laws, kernel::Budget};
    let grant = scope.cx().original_turn_ceiling().0;
    laws::fixture_renew_turn(scope.cx(), SHARD_TURN_FUEL, grant).ok()?;
    let (before, retained_before) = (scope.cx().original_work_units_spent(), scope.cx().retained_progress());
    let input = semio_plugin::kernel::RetainedTurnInput { operation: scope.cx().operation().0, generation: scope.cx().generation().0, epoch: 1, work: semio_plugin::kernel::OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: TURN_WALL_US.with(|wall| wall.get()).map_or(u64::MAX, |wall| semio_framework_trace::try_now_us().unwrap_or(0) + wall), fuel: SHARD_TURN_FUEL }, grant: scope.cx().retained_grant() };
    let budget = Budget::from_retained(input, 64, 64, 1 << 20, 8);
    let result = semio_plugin::reactor_turn_laws::poll_kernel(runtime, event, page, None, budget, &mut laws::fixture_identity(), scope.cx()).await.unwrap_or_else(|failure| panic!("shard reactor turn: {failure:?}"));
    let after = scope.cx().retained_progress();
    let retained = semio_framework_value::RetainedCloneProgress { copied_items: after.copied_items.saturating_sub(retained_before.copied_items), copied_bytes: after.copied_bytes.saturating_sub(retained_before.copied_bytes), retained_capacity_bytes: after.retained_capacity_bytes.saturating_sub(retained_before.retained_capacity_bytes), released_bytes: after.released_bytes.saturating_sub(retained_before.released_bytes) };
    let turn = ShardTurn { ingress: format!("{:?}", result.command_ingress).chars().take(240).collect(), status: format!("{:?}", result.status), sources: { let sources = semio_plugin::reactor_turn_laws::last_turn_more_work_sources(); format!("{:?}{}", sources.names(), if sources.typed_operation_contended { " contended" } else { "" }) }, spent: scope.cx().original_work_units_spent().saturating_sub(before), fuel_used: result.fuel_used, retained };
    Some((result, turn))
}

/// 🚪️ Opens one instance of the PDF editor through the real lifecycle (bundle, `InstanceOpen`, `Captured`, acknowledgement) like the shell's `createApp`.
async fn shard_open_instance<PA: semio_plugin::PluginApp>(runtime: &semio_plugin::plugin_runtime::PluginRuntime<PA>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, instance: u32, app_id: &str) -> semio_plugin::kernel::ActorInstanceLifetime {
    use semio_plugin::kernel::{ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, ActorInstanceOpenPurpose, ActorInstanceOpenRequest, AppInstanceId, Event};
    let mut event = Some(Event::InstanceOpen { purpose: ActorInstanceOpenPurpose::Ordinary, request: ActorInstanceOpenRequest { activation_generation: 41, instance_id: instance, request_sequence: 8 }, app_id: AppInstanceId(app_id.into()), actor: semio_os_kernel::LOCAL_ACTOR_ID.into(), config: Vec::new(), assets: Vec::new(), capabilities: Vec::new(), quotas: Default::default() });
    let mut captured = None;
    for _ in 0..SHARD_STALL_TURNS {
        let (result, _) = shard_turn(runtime, scope, &mut event, &mut None).await.expect("the open turn is funded");
        if let Some(receipt) = result.lifecycle_receipt { captured = Some(receipt); break }
    }
    let receipt = captured.expect("the PDF instance publishes its Captured receipt");
    let Receipt::Captured { lifetime, .. } = receipt else { panic!("open must publish Captured, got {receipt:?}") };
    let mut ack = Some(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt }));
    shard_turn(runtime, scope, &mut ack, &mut None).await.expect("the acknowledgement turn is funded");
    lifetime
}

/// 🐚️ The acknowledgement the browser shell posts back for one typed-operation result page it received as `Effect::SendMessage`: the ack magic followed by the page's own token (receiver, operation, generation, sequence, attempt), `None` for any other message.
fn shell_ack_of(payload: &[u8]) -> Option<Vec<u8>> {
    const PAGE: &[u8] = b"semio.typed-operation-page.v1\0";
    const ACK: &[u8] = b"semio.typed-operation-ack.v1\0";
    let token = payload.strip_prefix(PAGE)?.get(..25)?;
    Some([ACK, token].concat())
}


/// 🧹️ Closes the instance through the lifecycle: Accepted, acknowledgement, Retired, acknowledgements until the owner is gone.
async fn shard_close_instance<PA: semio_plugin::PluginApp>(runtime: &semio_plugin::plugin_runtime::PluginRuntime<PA>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, lifetime: semio_plugin::kernel::ActorInstanceLifetime) {
    use semio_plugin::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let mut pending = std::collections::VecDeque::from([Event::InstanceClose(ActorInstanceCloseRequest { lifetime, request_sequence: 9 })]);
    let (mut retired, mut census) = (None, std::collections::BTreeMap::<String, usize>::new());
    for _ in 0..16_384 {
        let turn = shard_acking_turn(runtime, scope, &mut pending).await;
        *census.entry(format!("{:?} patches={} effects={} receipt={}", turn.status, turn.ui_patches.len(), turn.effects.len(), turn.lifecycle_receipt.is_some())).or_default() += 1;
        match turn.lifecycle_receipt {
            Some(receipt @ Receipt::Retired { .. }) => { retired = Some(receipt); break }
            Some(receipt) => pending.push_back(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt })),
            None => {}
        }
    }
    let retired = retired.unwrap_or_else(|| panic!("the instance retires: {census:?}, pending {}", pending.len()));
    for _ in 0..8 {
        pending.push_front(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: retired }));
        shard_acking_turn(runtime, scope, &mut pending).await;
        if runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none() { break }
    }
}

/// 🐚️ One turn fed with the next pending event; every patch it publishes is acknowledged on a later turn, as the shell acknowledges each installed patch.
async fn shard_acking_turn<PA: semio_plugin::PluginApp>(runtime: &semio_plugin::plugin_runtime::PluginRuntime<PA>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, pending: &mut std::collections::VecDeque<semio_plugin::kernel::Event>) -> semio_plugin::kernel::TurnResult {
    let mut event = pending.pop_front();
    let (result, _) = shard_turn(runtime, scope, &mut event, &mut None).await.expect("the shell turn is funded");
    if let Some(unconsumed) = event { pending.push_front(unconsumed) }
    if let Some(receipt) = result.ui_patch_receipt {
        for patch in result.ui_patches.iter() { pending.push_back(semio_plugin::kernel::Event::PatchAck { receipt, surface: patch.surface.as_ref().to_string(), revision: patch.revision.0 }) }
    }
    result
}

/// 📂️ LAW: `AppCommand::MediaIn` of a small workbook to a freshly created transitional xlsx instance, driven turn by turn through the real `PluginRuntime` under the browser shard budget with the shell acknowledging typed-operation pages, completes or faults within [`SHARD_MEDIA_IN_TURN_LIMIT`] turns instead of answering `more-work` and `command-pending` forever.
async fn shard_media_in(input: &[u8], interactive_grant: bool, resident_editor: bool, wall_us: Option<u64>) {
    TURN_WALL_US.with(|wall| wall.set(wall_us));
    use semio_plugin::{app::{MediaArtifactDescriptor, MediaWireFormat}, kernel::{CommandBatch, CommandBatchDriver, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus}};
    use semio_plugin::artifact_app_laws as laws;
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let _arena = laws::FixtureUiArena::admit();
    let mut scope = laws::OriginalFixtureScope::admit();
    let fuel = laws::arm_host_turn_fuel(SHARD_TURN_FUEL);
    let _grant = interactive_grant.then(|| laws::arm_host_turn_grant(policy.preparation));
    let definition = create_xlsx_transitional_editor();
    let app_id = definition.id.clone();
    let plugin = semio_plugin::plugin_app_close_prelude::PluginBuilder::<_, ShardApp>::new("stdio").label(semio_plugin::LocalizedLabel::native("Stdio", "Stdio")).version("0.0.1").package_id("semio:stdio")
        .document_app::<semio_plugin::EditorApp<XlsxTransitionalEditor>>(semio_plugin::App { definition, examples: Vec::new() }).try_build().expect("the xlsx editor plugin assembles");
    let runtime = semio_plugin::plugin_runtime::PluginRuntime::<ShardApp>::new(policy).expect("plugin runtime");
    semio_plugin::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let format_kind = <XlsxTransitionalEditor as ArtifactEditor>::natural_file_codec().expect("the xlsx editor declares its natural file codec").format_kind.to_string();
    let resident = if resident_editor { Some(shard_open_instance(&runtime, &mut scope, 4_021, &app_id).await) } else { None };
    let instance = 4_022;
    let lifetime = shard_open_instance(&runtime, &mut scope, instance, &app_id).await;
    let command_lifetime = laws::arm_host_lifetime(scope.cx());
    let descriptor = MediaArtifactDescriptor { edge_id: None, port_id: Some(semio_plugin::NATURAL_FILE_PORT.into()), kind_id: Some(format_kind.clone()), media_type: None, blob_hash: None, wire: MediaWireFormat::Binary { format_kind } };
    let seq = 512;
    let command = semio_os_kernel::AppCommand::MediaIn { seq, port: semio_plugin::NATURAL_FILE_PORT.into(), descriptor: semio_os_kernel::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor)), data: input.to_vec() };
    let encoded = semio_os_kernel::encode_app_command(&command).await.expect("the MediaIn command encodes");
    let mut envelopes = CommandEnvelopeSet::try_new().expect("fixed command batch authority");
    envelopes.try_push(CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit MediaIn: {fault:?}"));
    let batch = CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit MediaIn batch: {fault:?}"));
    let mut driver = CommandBatchDriver::new(seq, batch);
    let mut carried = driver.next_page().expect("the host owner produces its first page");
    let mut acks = std::collections::VecDeque::<Vec<u8>>::new();
    let mut census = std::collections::BTreeMap::<String, usize>::new();
    let (mut turns, mut completed, mut fault, mut load_documents) = (0usize, false, None, 0usize);
    let started = std::time::Instant::now();
    while turns < SHARD_MEDIA_IN_TURN_LIMIT && started.elapsed() < std::time::Duration::from_secs(120) {
        let mut shell_event = acks.pop_front().map(|payload| semio_plugin::kernel::Event::Message { source: semio_plugin::kernel::MessageEndpoint::Shell { instance: semio_plugin::kernel::PluginInstanceId(instance.to_string()) }, payload });
        let Some((result, turn)) = shard_turn(&runtime, &mut scope, &mut shell_event, &mut carried).await else { break };
        if let Some(semio_plugin::kernel::Event::Message { payload, .. }) = shell_event { acks.push_front(payload) }
        for effect in &result.effects {
            match effect {
                semio_plugin::kernel::Effect::SendMessage { target: semio_plugin::kernel::MessageEndpoint::Shell { .. }, payload } => if let Some(ack) = shell_ack_of(payload) { acks.push_back(ack) },
                semio_plugin::kernel::Effect::LoadDocument { .. } => load_documents += 1,
                _ => {}
            }
        }
        turns += 1;
        *census.entry(format!("{} / {} / {}", turn.ingress, turn.status, turn.sources)).or_default() += 1;
        match result.command_ingress {
            CommandIngressStatus::CommandComplete(_) => { completed = true; break },
            CommandIngressStatus::Fault { fault: bytes, .. } => { fault = Some(String::from_utf8_lossy(&bytes).into_owned()); break },
            ref accepted @ (CommandIngressStatus::PageAccepted(_) | CommandIngressStatus::Backpressure(_)) => {
                driver.observe(accepted, semio_plugin::kernel::COMMAND_PAGE_MAXIMUM_BYTES).expect("the host owner accepts the status its own page produced");
                carried = driver.next_page().ok().flatten();
            }
            _ => {}
        }
    }
    fuel.settle(scope.cx());
    drop(command_lifetime);
    shard_close_instance(&runtime, &mut scope, lifetime).await;
    if let Some(resident) = resident { shard_close_instance(&runtime, &mut scope, resident).await }
    assert!(fault.is_none(), "MediaIn faulted: {fault:?}");
    assert!(completed, "MediaIn did not complete within {turns} turns: census {census:?}");
    assert!(load_documents <= 1, "MediaIn parked {load_documents} whole-document loads");
}

const REUSE_MARKETPLACES: &[u8] = include_bytes!("../../../../🧱️base/🧪️tests/🧫️fixtures/📕️reuse-marketplaces.xlsx");
const BUDGET: &[u8] = include_bytes!("../../../../🧱️base/🧪️tests/🧫️fixtures/💰️budget.xlsx");

#[semio_framework_async_macros::async_test]
async fn media_in_of_the_marketplaces_workbook_completes_under_the_shard_budget() { shard_media_in(REUSE_MARKETPLACES, false, false, None).await }

#[semio_framework_async_macros::async_test]
async fn media_in_of_the_small_budget_workbook_completes_under_the_shard_budget() { shard_media_in(BUDGET, false, false, None).await }

#[semio_framework_async_macros::async_test]
async fn media_in_of_the_small_budget_workbook_completes_under_the_interactive_grant() { shard_media_in(BUDGET, true, false, None).await }

#[semio_framework_async_macros::async_test]
async fn media_in_of_the_small_budget_workbook_completes_beside_a_resident_editor_instance() { shard_media_in(BUDGET, false, true, None).await }


/// 🕐️ LAW: the same open under a two millisecond wall per turn (the browser grants one hundred, and a native debug step is slower than a wasm one) still completes: a decode step that cannot finish inside its turn resumes on the next instead of restarting.
#[semio_framework_async_macros::async_test]
async fn media_in_of_the_small_budget_workbook_completes_under_a_two_millisecond_turn_wall() { shard_media_in(BUDGET, false, false, Some(2_000)).await }
