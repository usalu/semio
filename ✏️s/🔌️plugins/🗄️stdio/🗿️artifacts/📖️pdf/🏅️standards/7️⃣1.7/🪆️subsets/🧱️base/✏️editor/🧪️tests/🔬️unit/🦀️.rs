use crate::editor::pdf17::modes::edit;
use crate::editor::pdf17::modes::edit::windows::main;
use crate::standards::v1_7::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfSnapshot, STDIO_PDF17_DOCUMENT_SCHEMA};
use semio_plugin::{ArtifactEditor, Editor};
use semio_framework_test_fixtures::{fixture_bytes, fixture_text};
#[allow(unused_imports)]
use crate::standards::v1_7::subsets::base::schema::PDF_ARTIFACT_SCHEMA_ID;
#[allow(unused_imports)]
use {semio_plugin::built_to_component_tree, semio_plugin::ArtifactView, semio_plugin::ComponentTree, semio_plugin::ConfigView, semio_framework_artifact_reference::Dialect, semio_plugin::DraftView, semio_plugin::Emit, semio_plugin::Fault, semio_plugin::NoConfig, semio_plugin::NoConfigMutation, semio_plugin::NoDraft, semio_plugin::NoDraftMutation, semio_plugin::NoPresence, semio_plugin::NoPresenceMutation, semio_plugin::NoTransient, semio_plugin::NoTransientMutation, semio_framework_artifact_reference::StandardId, semio_framework_artifact_reference::SubsetId};
#[allow(unused_imports)]
use semio_framework_2d_compute::EngineHandles;
use crate::editor::pdf17::*;

#[test]
fn natural_file_route_profile_checks_refuse_decrypted_security_state() {
    use crate::standards::v1_7::subsets::{a, e, vt, x};
    use crate::standards::v1_7::subsets::base::io;
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfEncryption, PdfEncryptionAlgorithm};
    use semio_framework_diagnostic::Severity;
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🔒️natural-file-profile-encryption/🔣️.json")).unwrap();
    let mut snapshot = io::import::decode_pdf(fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf")).unwrap();
    assert_eq!(fixture["algorithm"], "rc4-128");
    let password = fixture["password"].as_str().unwrap();
    snapshot.encryption = Some(PdfEncryption { algorithm: PdfEncryptionAlgorithm::Rc4_128, permissions: -1, user_password: password.into(), owner_password: None, encrypt_metadata: true });
    let encrypted = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&snapshot).unwrap();
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::encryption_present(&encrypted).unwrap(), fixture["encrypted"].as_bool().unwrap());
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(&encrypted).is_err());
    let decoded = io::import::decode_pdf_with_password(&encrypted, password).unwrap();
    assert!(decoded.encryption.is_some());
    let checkers: [(&str, fn(&PdfSnapshot) -> Vec<semio_framework_diagnostic::Diagnostic>); 4] = [
        ("a", a::io::check_pdf_a_conformance),
        ("x", x::io::check_x_conformance),
        ("e", e::io::check_e_conformance),
        ("vt", vt::io::check_vt_conformance),
    ];
    for profile in fixture["profiles"].as_array().unwrap() {
        let id = profile["id"].as_str().unwrap();
        let code = profile["code"].as_str().unwrap();
        let checker = checkers.iter().find(|(candidate, _)| *candidate == id).unwrap().1;
        assert!(checker(&decoded).iter().any(|issue| issue.code.0 == code && matches!(issue.severity, Severity::Error | Severity::Fatal)), "{id} must refuse actual encrypted input after the security dictionary is lifted into typed state");
        let mut plain = decoded.clone();
        plain.encryption = None;
        assert!(!checker(&plain).iter().any(|issue| issue.code.0 == code), "{id} must not invent encryption for a plain retained document");
    }
}

/// 🛤️ Names how the original PDF bytes reach the mounted editor: whole media, or the sealed paged natural source the browser file chooser delivers.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PdfIntake { Media, NaturalSource }

/// 📏️ Host turns a large document may need to open, re-export and settle: the 48-page vector, Type1 and sampled-image documents measure at most 1,249 under the host's per-turn fuel, so this is eight times below the 32,768-turn fixture ceiling.
const LARGE_DOCUMENT_TURN_LIMIT: usize = 4_096;
/// 📏️ Host turns the paged decoder alone may need for the 9 MiB original image: 4,610 measured, one 4,096-unit step each.
const LARGE_IMAGE_DECODE_TURN_LIMIT: usize = 8_192;
/// ⛽️ The per-turn fuel the browser guest observes after its fixed turn entry: one 1 MiB host turn minus its admission spend.
const BROWSER_TURN_FUEL: u64 = (1 << 20) - 3_685;

/// 🌐️ Opens a PDF exactly as the browser does: a sealed paged natural source, one fresh browser turn of fuel per host turn and the shipped interactive component grants (512 KiB copy, 1 MiB release), so no step may need a contiguous allocation above the grant.
async fn open_registered_pdf(input: Vec<u8>, page_count: usize, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope) -> semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>> {
    open_registered_pdf_through(input, page_count, scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await
}

async fn open_registered_pdf_through(input: Vec<u8>, page_count: usize, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, intake: PdfIntake, fuel: u64) -> semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>> {
    open_registered_pdf_with(input, page_count, scope, intake, fuel, true).await
}

/// 🌐️ [`open_registered_pdf_through`] where `warm` says whether the app's head projection is refreshed before the open starts: the browser opens into a freshly created app whose surface has not rendered yet, so its reserved media cache is born cold inside the open turns.
async fn open_registered_pdf_with(input: Vec<u8>, page_count: usize, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, intake: PdfIntake, fuel: u64, warm: bool) -> semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>> {
    try_open_registered_pdf_with(input, page_count, scope, intake, fuel, warm, LARGE_DOCUMENT_TURN_LIMIT).await.unwrap_or_else(|reason| panic!("{reason}")).0
}

/// 📊️ What one open cost the host: the turns that handed the source to the app, the turns the typed operation then needed to settle, and the wall time of both.
#[derive(Clone, Copy, Debug)]
struct OpenCost { intake_turns: usize, settle_turns: usize, wall: std::time::Duration }

impl OpenCost {
    fn turns(&self) -> usize { self.intake_turns + self.settle_turns }
}

/// 🌐️ [`open_registered_pdf_with`] that reports a refusal, a stall or more than `turn_limit` settlement turns as `Err` and the measured [`OpenCost`] as `Ok`, so a scaling law can state its bound and a probe can print where a document stops opening.
async fn try_open_registered_pdf_with(input: Vec<u8>, page_count: usize, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, intake: PdfIntake, fuel: u64, warm: bool, turn_limit: usize) -> Result<(semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>>, OpenCost), String> {
    use semio_plugin::{artifact_app_laws as laws, EditorApp, PluginApp};
    use semio_plugin::app::{MediaArtifact, MediaArtifactDescriptor, MediaWireFormat};
    let started = std::time::Instant::now();
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let meta = laws::meta("local");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().unwrap();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let natural_bytes = (intake == PdfIntake::NaturalSource).then(|| input.clone());
    let artifact = MediaArtifact {
        descriptor: MediaArtifactDescriptor {
            edge_id: None, port_id: Some(semio_plugin::NATURAL_FILE_PORT.into()),
            kind_id: Some(codec.format_kind.into()), media_type: None, blob_hash: None,
            wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() },
        },
        data: input,
    };
    let outcome: Result<OpenCost, String> = async {
        let cx = scope.cx();
        let grant = if intake == PdfIntake::NaturalSource { policy.preparation } else { cx.retained_grant() };
        laws::ensure_fixture_cancellation_issuer(&mut app, cx).expect("mounted PDF instance is born");
        if warm { app.render_ctx().refresh_cache().await.expect("mounted PDF head is projected"); }
        let mut artifact = Some(artifact);
        let mut natural = natural_bytes.map(|bytes| semio_plugin::NaturalFilePagedSource::retain(semio_framework_value::paged::PagedBytes::try_from_slice(&bytes).expect("paged natural source")));
        let mut started_operation = None;
        let mut consumed = false;
        let mut idle = 0;
        let mut intake_turns = 0;
        for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
            laws::fixture_renew_turn(cx, fuel, grant).expect("mounted PDF import turn is funded");
            intake_turns += 1;
            let before = (cx.original_work_units_spent(), cx.retained_progress());
            let accepted = match intake {
                PdfIntake::Media => PluginApp::consume_media(&mut app, semio_plugin::NATURAL_FILE_PORT, &mut artifact, cx)
                    .await.expect("mounted PDF consumes original bytes").is_some(),
                PdfIntake::NaturalSource => {
                    app.dispatch_ctx().advance_original_reserved_natural_source(&mut natural, &meta, &mut started_operation, cx).await.expect("mounted PDF routes its paged natural source");
                    started_operation.is_some()
                }
            };
            if accepted {
                consumed = true;
                break;
            }
            app.maintenance_step(cx, policy.close).expect("mounted PDF upkeep is funded");
            let after = (cx.original_work_units_spent(), cx.retained_progress());
            idle = if before == after { idle + 1 } else { 0 };
            if idle >= laws::OriginalFixtureCaller::maximum_idle_turns() { break }
        }
        if !consumed { return Err(format!("mounted PDF media intake stalled after {idle} idle turns: {:?}, {}", cx.retained_grant(), app.typed_operation_diagnostic())); }
        let settled = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation_within(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx(), turn_limit
        ).await);
        let (receipt, settle_turns) = settled.map_err(|failure| format!("the mounted PDF import did not settle within {turn_limit} host turns after {intake_turns} intake turns: {failure:?}"))?;
        if receipt.completions != 1 { return Err(format!("the mounted PDF import settled {} times", receipt.completions)); }
        if !receipt.ui_scopes.iter().any(|scope| matches!(scope, semio_plugin::kernel::UiDirtyScope::Full)) {
            return Err(format!("the mounted PDF import dirties the whole UI before any later partial refresh, got {:?} with effects {:?}", receipt.ui_scopes, receipt.effects));
        }
        if receipt.effects.iter().any(|effect| matches!(effect, semio_plugin::Effect::LoadDocument { .. })) { return Err("the mounted PDF import loads a document by reference".into()); }
        let pages = app.snapshot().expect("the mounted PDF is ready").pages.len();
        if pages != page_count { return Err(format!("the mounted PDF opened {pages} pages, not {page_count}")); }
        Ok(OpenCost { intake_turns, settle_turns, wall: started.elapsed() })
    }.await;
    match outcome {
        Ok(cost) => Ok((app, cost)),
        Err(reason) => {
            let _ = laws::close_registered_fixture_app(&mut app, policy, scope.cx());
            Err(reason)
        }
    }
}

/// 📚️ A flat generated document of `pages` vector pages, two objects per page, as the large-document laws open it.
fn scaling_document(pages: usize) -> Vec<u8> {
    large_pdf(&serde_json::json!({ "pageCount": pages, "segmentCount": 4 }))
}

/// 📏️ Settlement turns per page the open may need under the browser grants: 5.3 measured at 120 pages and 5.6 at 600, so linear growth with a margin and no fixed per-document ceiling.
const OPEN_TURNS_PER_PAGE: usize = 8;

/// 📊️ Opens `pages` pages under the browser grants and asserts the settlement turns stay proportional to the document.
async fn open_cost_scales_with(pages: usize) {
    use semio_plugin::artifact_app_laws as laws;
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let (mut app, cost) = try_open_registered_pdf_with(scaling_document(pages), pages, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL, true, 256 + OPEN_TURNS_PER_PAGE * pages).await.unwrap_or_else(|reason| panic!("{reason}"));
    assert!(cost.settle_turns <= 256 + OPEN_TURNS_PER_PAGE * pages, "{pages} pages settled in {} turns", cost.settle_turns);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

#[semio_framework_async_macros::async_test]
async fn open_turns_are_proportional_to_the_page_count_at_120_pages() { open_cost_scales_with(120).await }

#[semio_framework_async_macros::async_test]
async fn open_turns_are_proportional_to_the_page_count_at_600_pages() { open_cost_scales_with(600).await }

/// 🌐️ The browser file chooser hands the guest a sealed paged natural source; opening it under the browser's one-MiB turn fuel completes in bounded turns, so no decoder quote can exceed what a turn can fund.
#[semio_framework_async_macros::async_test]
async fn natural_source_route_opens_a_pdf_under_the_browser_turn_fuel() {
    use semio_plugin::artifact_app_laws as laws;
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    assert_eq!(app.snapshot().expect("the paged PDF is ready").pages.len(), 2);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// ⛽️ The fuel one shard turn grants in the browser (`DEFAULT_SHARD_BUDGET.fuel` of the actor shard runtime): fifty times the 1 MiB turn the first paged-natural regression ran under.
const SHARD_TURN_FUEL: u64 = 50_000_000;
/// 📏️ The most original work one host turn may spend while opening or exporting a two-page PDF: a turn that spends (or sizes its probe against) its whole fuel holds the actor lane for the wall bound instead of one bounded step.
const SHARD_TURN_WORK_BOUND: u128 = 1 << 20;
/// 🔢️ Host turns the two-page open, and the snapshot export, may need under the shard fuel (68 measured for the open under 1 MiB fuel).
const SHARD_TURN_LIMIT: usize = 512;
/// 🔢️ Turns a lifecycle or drain loop of the shard-driven law may spend before the law calls it stuck.
const SHARD_STALL_TURNS: usize = 4_096;
/// 🔢️ Host turns `AppCommand::MediaIn` of a small PDF may need until its descriptor, payload and metadata are received and the import operation is live (the mount); measured 2 under the shard grant, and one 64-item interactive grant takes tens of turns.
const SHARD_MOUNT_TURN_LIMIT: usize = 64;
/// 🔢️ Host turns the whole `AppCommand::MediaIn` of a small PDF may need until its command completes.
const SHARD_MEDIA_IN_TURN_LIMIT: usize = 512;
/// 🌐️ Opens `two-pages.pdf` into a freshly created app under the browser shard's real turn fuel, inside an armed host lifetime: the open settles in a bounded number of turns and no turn spends more than [`SHARD_TURN_WORK_BOUND`] work units (a probe must not open, nor spend, the whole 50,000,000 units of a turn). `warm` refreshes the head projection first, as the native fixtures always did; the browser never does.
async fn shard_fuel_open(intake: PdfIntake, warm: bool) {
    use semio_plugin::artifact_app_laws as laws;
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let fuel = laws::arm_host_turn_fuel(SHARD_TURN_FUEL);
    let lifetime = laws::arm_host_lifetime(scope.cx());
    let mut app = open_registered_pdf_with(input.to_vec(), 2, &mut scope, intake, SHARD_TURN_FUEL, warm).await;
    fuel.settle(scope.cx());
    drop(lifetime);
    assert!(fuel.turns() <= SHARD_TURN_LIMIT, "the shard-fuel open needed {} host turns", fuel.turns());
    assert!(fuel.peak_turn_work() <= SHARD_TURN_WORK_BOUND, "one shard-fuel open turn spent {} work units", fuel.peak_turn_work());
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

#[semio_framework_async_macros::async_test]
async fn shard_fuel_open_through_the_paged_natural_source_with_a_warm_head() { shard_fuel_open(PdfIntake::NaturalSource, true).await }

#[semio_framework_async_macros::async_test]
async fn shard_fuel_open_through_the_paged_natural_source_with_a_cold_head() { shard_fuel_open(PdfIntake::NaturalSource, false).await }

#[semio_framework_async_macros::async_test]
async fn shard_fuel_open_through_the_media_route_with_a_warm_head() { shard_fuel_open(PdfIntake::Media, true).await }

#[semio_framework_async_macros::async_test]
async fn shard_fuel_open_through_the_media_route_with_a_cold_head() { shard_fuel_open(PdfIntake::Media, false).await }

type ShardApp = semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>>;

/// 🧾️ What one reactor turn of the shard-driven open reported.
#[derive(Debug)]
struct ShardTurn { ingress: String, status: String, spent: u128, fuel_used: u64, retained: semio_framework_value::RetainedCloneProgress }

/// 🔄️ One reactor turn exactly as a shard grants it: the original caller's context renewed with the shard's fuel (armed through `arm_host_turn_fuel`) and grant, the fixture's retained input, one optional lifecycle event and one optional command page.
async fn shard_turn(runtime: &semio_plugin::plugin_runtime::PluginRuntime<ShardApp>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, event: &mut Option<semio_plugin::kernel::Event>, page: &mut Option<(semio_plugin::kernel::CommandPageCursor, semio_plugin::kernel::FixedCommandPage)>) -> Option<(semio_plugin::kernel::TurnResult, ShardTurn)> {
    use semio_plugin::{artifact_app_laws as laws, kernel::Budget};
    let grant = scope.cx().original_turn_ceiling().0;
    laws::fixture_renew_turn(scope.cx(), SHARD_TURN_FUEL, grant).ok()?;
    let (before, retained_before) = (scope.cx().original_work_units_spent(), scope.cx().retained_progress());
    let input = semio_plugin::kernel::RetainedTurnInput { operation: scope.cx().operation().0, generation: scope.cx().generation().0, epoch: 1, work: semio_plugin::kernel::OriginalActorTurnWork { clock_binding: 1, absolute_deadline_us: u64::MAX, fuel: SHARD_TURN_FUEL }, grant: scope.cx().retained_grant() };
    let budget = Budget::from_retained(input, 64, 64, 1 << 20, 8);
    let result = semio_plugin::reactor_turn_laws::poll_kernel(runtime, event, page, None, budget, &mut laws::fixture_identity(), scope.cx()).await.unwrap_or_else(|failure| panic!("shard reactor turn: {failure:?}"));
    let after = scope.cx().retained_progress();
    let retained = semio_framework_value::RetainedCloneProgress { copied_items: after.copied_items.saturating_sub(retained_before.copied_items), copied_bytes: after.copied_bytes.saturating_sub(retained_before.copied_bytes), retained_capacity_bytes: after.retained_capacity_bytes.saturating_sub(retained_before.retained_capacity_bytes), released_bytes: after.released_bytes.saturating_sub(retained_before.released_bytes) };
    let turn = ShardTurn { ingress: format!("{:?}", result.command_ingress).chars().take(24).collect(), status: format!("{:?}", result.status), spent: scope.cx().original_work_units_spent().saturating_sub(before), fuel_used: result.fuel_used, retained };
    Some((result, turn))
}

/// 🚪️ Opens one instance of the PDF editor through the real lifecycle (bundle, `InstanceOpen`, `Captured`, acknowledgement) like the shell's `createApp`.
async fn shard_open_instance(runtime: &semio_plugin::plugin_runtime::PluginRuntime<ShardApp>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, instance: u32, app_id: &str) -> semio_plugin::kernel::ActorInstanceLifetime {
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

/// 🧹️ Closes the instance through the lifecycle: Accepted, acknowledgement, Retired, acknowledgements until the owner is gone.
async fn shard_close_instance(runtime: &semio_plugin::plugin_runtime::PluginRuntime<ShardApp>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, lifetime: semio_plugin::kernel::ActorInstanceLifetime) {
    use semio_plugin::kernel::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt as Receipt, Event};
    let mut close = Some(Event::InstanceClose(ActorInstanceCloseRequest { lifetime, request_sequence: 9 }));
    let (first, _) = shard_turn(runtime, scope, &mut close, &mut None).await.expect("the close turn is funded");
    let accepted = first.lifecycle_receipt.expect("the close is accepted");
    let mut ack = Some(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: accepted }));
    shard_turn(runtime, scope, &mut ack, &mut None).await.expect("the close acknowledgement turn is funded");
    let mut retired = None;
    for _ in 0..16_384 {
        let (turn, _) = shard_turn(runtime, scope, &mut None, &mut None).await.expect("the retirement turn is funded");
        if let Some(receipt @ Receipt::Retired { .. }) = turn.lifecycle_receipt { retired = Some(receipt); break }
    }
    let retired = retired.expect("the instance retires");
    for _ in 0..8 {
        let mut ack = Some(Event::InstanceLifecycleAck(ActorInstanceLifecycleAck { receipt: retired }));
        shard_turn(runtime, scope, &mut ack, &mut None).await.expect("the retirement acknowledgement turn is funded");
        if runtime.guest_lifetimes.borrow().get(lifetime.instance_id).is_none() { break }
    }
}

/// 📂️ What one shard-driven `MediaIn` open reported.
struct ShardOpen { mounted_at: Option<usize>, completed: bool, fault: Option<String>, log: Vec<ShardTurn>, report: String, peak_work: u128, shell_pages: usize, document_pages: Option<usize> }

/// 🐚️ The acknowledgement the browser shell posts back for one typed-operation result page it received as `Effect::SendMessage`: the ack magic followed by the page's own token (receiver, operation, generation, sequence, attempt), `None` for any other message.
fn shell_ack_of(payload: &[u8]) -> Option<Vec<u8>> {
    const PAGE: &[u8] = b"semio.typed-operation-page.v1\0";
    const ACK: &[u8] = b"semio.typed-operation-ack.v1\0";
    let token = payload.strip_prefix(PAGE)?.get(..25)?;
    Some([ACK, token].concat())
}

/// 📂️ Sends `AppCommand::MediaIn` for `bytes` to a freshly created instance exactly as `openNaturalFile` does (fresh instance through the lifecycle, command pages one per turn, then continuation turns without input) and returns the turn log; `mounted_at` is the first turn the import operation is live. The open must complete or fault within `turn_limit` turns.
async fn shard_media_in_open(bytes: &[u8], turn_limit: usize, interactive_grant: bool) -> ShardOpen {
    use semio_plugin::{app::{MediaArtifactDescriptor, MediaWireFormat}, kernel::{CommandBatch, CommandBatchDriver, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus}};
    use semio_plugin::{artifact_app_laws as laws, PluginApp as _};
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let _arena = laws::FixtureUiArena::admit();
    let mut scope = laws::OriginalFixtureScope::admit();
    let fuel = laws::arm_host_turn_fuel(SHARD_TURN_FUEL);
    let _grant = interactive_grant.then(|| laws::arm_host_turn_grant(policy.preparation));
    let definition = create_pdf17_editor();
    let app_id = definition.id.clone();
    let plugin = semio_plugin::plugin_app_close_prelude::PluginBuilder::<_, ShardApp>::new("stdio").label("Stdio").version("0.0.1").package_id("semio:stdio")
        .document_app::<semio_plugin::EditorApp<Pdf17Editor>>(semio_plugin::App { definition, examples: Vec::new() }).try_build().expect("the PDF editor plugin assembles");
    let runtime = semio_plugin::plugin_runtime::PluginRuntime::<ShardApp>::new(policy).expect("plugin runtime");
    semio_plugin::plugin_runtime::install_plugin_bundle(&runtime, plugin);
    let instance = 4_022;
    let lifetime = shard_open_instance(&runtime, &mut scope, instance, &app_id).await;
    let command_lifetime = laws::arm_host_lifetime(scope.cx());
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().unwrap();
    let descriptor = MediaArtifactDescriptor { edge_id: None, port_id: Some(semio_plugin::NATURAL_FILE_PORT.into()), kind_id: Some(codec.format_kind.into()), media_type: None, blob_hash: None, wire: MediaWireFormat::Binary { format_kind: codec.format_kind.into() } };
    let seq = 512;
    let command = semio_os_kernel::AppCommand::MediaIn { seq, port: semio_plugin::NATURAL_FILE_PORT.into(), descriptor: semio_os_kernel::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor)), data: bytes.to_vec() };
    let encoded = semio_os_kernel::encode_app_command(&command).await.expect("the MediaIn command encodes");
    let mut envelopes = CommandEnvelopeSet::try_new().expect("fixed command batch authority");
    envelopes.try_push(CommandEnvelope { instance, seq, command: encoded }).unwrap_or_else(|(fault, _)| panic!("admit MediaIn: {fault:?}"));
    let batch = CommandBatch::try_new(seq, envelopes).unwrap_or_else(|(fault, _)| panic!("admit MediaIn batch: {fault:?}"));
    let mut driver = CommandBatchDriver::new(seq, batch);
    let mut carried = driver.next_page().expect("the host owner produces its first page");
    let mut open = ShardOpen { mounted_at: None, completed: false, fault: None, log: Vec::new(), report: String::new(), peak_work: 0, shell_pages: 0, document_pages: None };
    let mut acks = std::collections::VecDeque::<Vec<u8>>::new();
    let started = std::time::Instant::now();
    while open.log.len() < turn_limit && started.elapsed() < std::time::Duration::from_secs(120) {
        let mut shell_event = acks.pop_front().map(|payload| semio_plugin::kernel::Event::Message { source: semio_plugin::kernel::MessageEndpoint::Shell { instance: semio_plugin::kernel::PluginInstanceId(instance.to_string()) }, payload });
        let Some((result, turn)) = shard_turn(&runtime, &mut scope, &mut shell_event, &mut carried).await else { break };
        if let Some(semio_plugin::kernel::Event::Message { payload, .. }) = shell_event { acks.push_front(payload) }
        for effect in &result.effects {
            if let semio_plugin::kernel::Effect::SendMessage { target: semio_plugin::kernel::MessageEndpoint::Shell { .. }, payload } = effect {
                if let Some(ack) = shell_ack_of(payload) { open.shell_pages += 1; acks.push_back(ack) }
            }
        }
        open.log.push(turn);
        let live = runtime.instances.borrow().get(instance).and_then(|cell| cell.host_instance().map(|guard| guard.app.has_pending_typed_operations())).unwrap_or(false);
        if open.mounted_at.is_none() && live { open.mounted_at = Some(open.log.len()) }
        match result.command_ingress {
            CommandIngressStatus::CommandComplete(_) => { open.completed = true; break },
            CommandIngressStatus::Fault { fault, .. } => { open.fault = Some(String::from_utf8_lossy(&fault).into_owned()); break },
            ref accepted @ (CommandIngressStatus::PageAccepted(_) | CommandIngressStatus::Backpressure(_)) => {
                driver.observe(accepted, semio_plugin::kernel::COMMAND_PAGE_MAXIMUM_BYTES).expect("the host owner accepts the status its own page produced");
                carried = driver.next_page().ok().flatten();
            }
            _ => {}
        }
    }
    assert!(open.completed || open.fault.is_some(), "MediaIn neither completed nor faulted within {turn_limit} turns: {}", shard_census(&open));
    fuel.settle(scope.cx());
    open.peak_work = fuel.peak_turn_work();
    drop(command_lifetime);
    open.document_pages = runtime.instances.borrow().get(instance).and_then(|cell| cell.host_instance().map(|mut guard| guard.app.snapshot().expect("the opened PDF is ready").pages.len()));
    open.report = runtime.instances.borrow().get(instance).and_then(|cell| cell.host_instance().map(|guard| guard.app.typed_operation_diagnostic())).unwrap_or_default();
    shard_close_instance(&runtime, &mut scope, lifetime).await;
    open
}

/// 📂️ The census a stalled shard open prints: how many turns ended in which ingress and turn status.
fn shard_census(open: &ShardOpen) -> String {
    let mut census = std::collections::BTreeMap::<String, usize>::new();
    for turn in &open.log { *census.entry(format!("{} / {}", turn.ingress, turn.status)).or_default() += 1; }
    let idle = open.log.iter().filter(|turn| turn.spent == 0 && turn.fuel_used == 0 && turn.retained == semio_framework_value::RetainedCloneProgress::default()).count();
    format!("{} turns ({idle} idle), mounted at {:?}, census {census:?}, last {:?}, app: {}", open.log.len(), open.mounted_at, open.log.iter().rev().take(3).collect::<Vec<_>>(), open.report)
}

/// 📂️ LAW: `AppCommand::MediaIn` of a small PDF to a freshly created instance, driven turn by turn through the real `PluginRuntime` under the shard budget (wide grant) or the interactive per-turn grant, receives its descriptor, payload and metadata, mounts its import within [`SHARD_MOUNT_TURN_LIMIT`] turns and completes within [`SHARD_MEDIA_IN_TURN_LIMIT`]. A typed receive advances as many actions per turn as the grant and fuel admit, so the receive no longer costs one host turn per action (the shell's 25 to 250 ms backoff per quiet turn made the old custody look hung).
async fn shard_media_in_law(interactive_grant: bool) {
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let open = shard_media_in_open(input, SHARD_MEDIA_IN_TURN_LIMIT, interactive_grant).await;
    assert!(open.fault.is_none(), "MediaIn faulted: {:?}", open.fault);
    let mounted = open.mounted_at.unwrap_or_else(|| panic!("MediaIn did not reach its mount: {}", shard_census(&open)));
    assert!(mounted <= SHARD_MOUNT_TURN_LIMIT, "MediaIn needed {mounted} turns to reach its mount (limit {SHARD_MOUNT_TURN_LIMIT}): {}", shard_census(&open));
    assert!(open.completed && open.log.len() <= SHARD_MEDIA_IN_TURN_LIMIT, "MediaIn did not complete within {SHARD_MEDIA_IN_TURN_LIMIT} turns: {}", shard_census(&open));
    assert!(open.peak_work <= SHARD_TURN_WORK_BOUND, "one MediaIn turn spent {} work units: {}", open.peak_work, shard_census(&open));
    assert!(open.shell_pages > 0, "the shell received no typed-operation result page: {}", shard_census(&open));
    assert_eq!(open.document_pages, Some(2), "the opened document is not visible in the app: {}", shard_census(&open));
}

#[semio_framework_async_macros::async_test]
async fn media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_shard_budget() { shard_media_in_law(false).await }

#[semio_framework_async_macros::async_test]
async fn media_in_of_a_small_pdf_reaches_its_mount_through_the_real_runtime_under_the_interactive_grant() { shard_media_in_law(true).await }

/// 🛑️ LAW: the reactor counts the consecutive quiet turns of one retained command (no work, no retained stock, no output) and calls it stalled exactly when they exhaust the idle limit; any progressing turn, and any other command, restarts the count. Past the limit the reactor faults the command with the named `plugin.command-stalled` fault instead of spinning until the shell's 300 s wall.
#[test]
fn a_quiet_retained_command_is_faulted_as_stalled_by_the_reactor() {
    use semio_plugin::reactor_turn_laws::{command_stalled, note_command_turn, set_command_idle_turn_limit, COMMAND_IDLE_TURN_LIMIT};
    set_command_idle_turn_limit(48);
    let streaks: Vec<bool> = (0..48).map(|_| command_stalled(note_command_turn(7, 90, true))).collect();
    assert!(streaks[..47].iter().all(|stalled| !stalled) && streaks[47], "the 48th consecutive quiet turn exhausts the limit of 48");
    assert!(!command_stalled(note_command_turn(7, 90, false)), "a progressing turn restarts the count");
    assert!(!(0..47).any(|_| command_stalled(note_command_turn(7, 90, true))), "47 quiet turns after a progressing one stay below the limit");
    assert!(!command_stalled(note_command_turn(7, 91, true)), "another command of the same instance starts its own count");
    assert!(!command_stalled(note_command_turn(8, 91, true)), "another instance starts its own count");
    assert_eq!(semio_plugin::reactor_turn_laws::COMMAND_STALLED_FAULT_CODE, "plugin.command-stalled");
    set_command_idle_turn_limit(COMMAND_IDLE_TURN_LIMIT);
}

/// 🛑️ LAW: a `MediaIn` that keeps making progress is never faulted as stalled, even under an idle limit of eight turns: its descriptor, payload, metadata and import custody never leaves the reactor eight consecutive turns without work or retained stock moving.
#[semio_framework_async_macros::async_test]
async fn a_progressing_media_in_is_never_faulted_as_stalled() {
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    semio_plugin::reactor_turn_laws::set_command_idle_turn_limit(8);
    let open = shard_media_in_open(input, SHARD_MEDIA_IN_TURN_LIMIT, false).await;
    semio_plugin::reactor_turn_laws::set_command_idle_turn_limit(semio_plugin::reactor_turn_laws::COMMAND_IDLE_TURN_LIMIT);
    assert!(open.fault.is_none() && open.completed, "a progressing MediaIn was stopped: {:?} {}", open.fault, shard_census(&open));
}

/// 🪶️ A freshly born PDF editor app owns its stores, registries and queues as page tables only, so its whole live heap stays inside the bounded footprint the language-neutral law names before any document opens.
#[semio_framework_async_macros::async_test]
async fn a_freshly_born_pdf_app_and_its_stores_hold_only_a_small_bounded_heap_footprint() {
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🪶️fresh-app-footprint/🔣️.json")).unwrap();
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let before = semio_framework_trace::retained_heap_bytes_on_this_thread();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let born = semio_framework_trace::retained_heap_bytes_on_this_thread() - before;
    assert!(born > 0 && (born as u64) <= law["maximumBornBytes"].as_u64().unwrap(), "a fresh app births page tables, not slot bodies: {born} bytes");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// 💾️ Saves the edited document through the registered native Save handle and downloads its segmented output pages.
async fn save_natural_bytes(app: &mut semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, instance_id: u32, format_kind: &str, media_type: &str, upkeep: bool) -> Vec<u8> {
    save_natural_bytes_under(app, scope, instance_id, format_kind, media_type, upkeep, None).await
}

/// 💾️ Runs one registered Save to its downloaded bytes; with `browser` every submission and poll turn is issued the browser's fresh fuel and the shipped interactive grants instead of the fixture's lifetime stock.
async fn save_natural_bytes_under(app: &mut semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>>, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, instance_id: u32, format_kind: &str, media_type: &str, upkeep: bool, browser: Option<(u64, semio_framework_value::RetainedCloneGrant)>) -> Vec<u8> {
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    let revision = app.document_base_revision().expect("edited document revision");
    let document = app.document_identity().expect("edited document identity");
    app.bind_instance_id(instance_id).await;
    let mut submitted = None;
    let mut save_turns = 0usize;
    let (save_fuel, save_grant) = browser.unwrap_or_else(|| (laws::OriginalFixtureCaller::turn_fuel(), scope.cx().retained_grant()));
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        laws::fixture_renew_turn(scope.cx(), save_fuel, save_grant).expect("PDF Save submission turn is funded");
        save_turns += 1;
        if app.submit_media_export_checked(semio_plugin::NATURAL_FILE_PORT, &document, revision, None, &mut submitted, scope.cx()).await.expect("checked PDF Save submission") { break }
        if upkeep { app.maintenance_step(scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).expect("PDF upkeep is funded between Save turns"); }
    }
    let handle = submitted.unwrap_or_else(|| panic!("registered PDF Save handle after {save_turns} turns: {}", app.typed_operation_diagnostic()));
    assert_eq!(handle.parent_document_id.as_str(), document.as_str());
    let mut completed = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        if browser.is_some() { laws::fixture_renew_turn(scope.cx(), save_fuel, save_grant).expect("PDF Save poll turn is funded"); }
        match laws::poll_fixture_media_export(app, &handle, scope.cx()).await.expect("registered PDF Save poll") {
            semio_plugin::ArtifactMediaExportPoll::Running { .. } => {},
            semio_plugin::ArtifactMediaExportPoll::Complete(result) => { completed = Some(result); break },
            other => panic!("registered PDF Save did not produce its segmented output: {other:?}"),
        }
    }
    let result = completed.expect("registered PDF Save completes");
    assert_eq!(result.schema, format_kind);
    assert_eq!(result.mime_type, media_type);
    let status = laws::retain_fixture_media_export_result(app, &handle, result, scope.cx()).await.expect("PDF Save publishes download status");
    assert_eq!(status.port, semio_plugin::NATURAL_FILE_PORT);
    let mut bytes = Vec::new();
    let mut terminal = false;
    type Output = semio_plugin::ArtifactOutputChunks;
    for _ in 0..=(Output::MAXIMUM_TOTAL_BYTES / Output::CHUNK_BYTES) {
        if browser.is_some() { laws::fixture_renew_turn(scope.cx(), save_fuel, save_grant).expect("PDF Save download turn is funded"); }
        if let Some(chunk) = laws::take_fixture_media_export_chunk(app, &handle, &mut terminal, scope.cx()).await.expect("PDF Save download page") {
            assert!(!chunk.is_empty() && chunk.len() <= Output::CHUNK_BYTES);
            bytes.extend_from_slice(&chunk);
        }
        if terminal { break }
    }
    assert!(terminal, "PDF Save downloads a terminal page sequence");
    assert_eq!(bytes.len() as u64, status.total_bytes);
    assert_eq!(app.document_base_revision().unwrap(), revision);
    assert_eq!(app.document_identity().as_deref(), Some(document.as_str()));
    bytes
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_matches_independent_rotation_and_retains_other_pdf_objects() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    assert_eq!(codec.format_kind, fixture["codec"]["formatKind"].as_str().unwrap());
    assert_eq!(codec.extension, fixture["codec"]["extension"].as_str().unwrap());
    assert_eq!(codec.media_type, fixture["codec"]["mediaType"].as_str().unwrap());
    assert_eq!(codec.binary, fixture["codec"]["binary"].as_bool().unwrap());
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let original = app.snapshot().unwrap().clone();
    let action = &fixture["action"];
    let args = semio_framework_value::DslValue::object([
        ("page".into(), semio_framework_value::DslValue::float(action["page"].as_f64().unwrap())),
        ("x".into(), semio_framework_value::DslValue::float(action["x"].as_f64().unwrap())),
    ]);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, action["id"].as_str().unwrap(), Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    let snapshot = app.snapshot().unwrap();
    let index = action["page"].as_u64().unwrap() as usize - 1;
    let rotation = action["x"].as_u64().unwrap() as u16;
    assert_eq!(snapshot.pages[0], original.pages[0], "an unedited page remains unchanged");
    let bytes = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
    if let Some(path) = std::env::var_os("SEMIO_PDF_SAVE_ROTATED_OUTPUT") { std::fs::write(path, &bytes).unwrap(); }
    let revision = app.document_base_revision().unwrap();
    let document = app.document_identity().unwrap();
    assert_eq!(app.document_base_revision().unwrap(), revision);
    assert_eq!(app.document_identity().as_deref(), Some(document.as_str()));
    let reference_spec = semio_repo_test_host::parse_json(&fixture["mutation"].to_string()).unwrap();
    let reference = oracle::oracle_apply_mutation(input, &reference_spec).expect("lopdf independently edits the input");
    let observed = oracle::project_pdf_1_7(&bytes).expect("lopdf reads natural output");
    assert_eq!(observed, oracle::project_pdf_1_7(&reference).unwrap());
    let observed: serde_json::Value = serde_json::from_str(&observed.to_string()).unwrap();
    assert_eq!(observed["pageCount"], fixture["expected"]["pageCount"]);
    assert_eq!(observed["pages"][index]["rotate"], fixture["expected"]["rotation"]);
    let mut reopened_app = open_registered_pdf_through(bytes, 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let reopened = reopened_app.snapshot().unwrap();
    assert_eq!(reopened.pages[index].rotate, i32::from(rotation));
    assert_eq!(reopened.pages[0].text(), original.pages[0].text());
    assert_eq!(reopened.info, original.info);
    assert!(<Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture["invalidText"].as_str().unwrap().as_bytes()).is_err());
    laws::close_registered_fixture_app(&mut reopened_app, policy, scope.cx()).expect("reopened editor closes");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}


/// 🛡️ Upkeep between two Save turns never closes the live media export preparation; only a retained refusal or the closing app owns that close.
#[semio_framework_async_macros::async_test]
async fn maintenance_between_save_turns_keeps_the_live_media_export() {
    use semio_plugin::artifact_app_laws as laws;
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let bytes = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, true).await;
    assert!(bytes.starts_with(b"%PDF-"));
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_page_count(&bytes).unwrap(), 2);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// 📂️ Opens an existing PDF, edits its title and inserts text on page one, saves it natively and reopens it with both pages intact; lopdf reads the same pages, title and text operators.
#[semio_framework_async_macros::async_test]
async fn natural_file_route_edits_title_and_inserts_text_then_saves_both_pages() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    use semio_plugin::artifact_app_laws as laws;
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let original = app.snapshot().unwrap().clone();
    let text = |value: &str| semio_framework_value::DslValue::String(value.into());
    let number = |value: f64| semio_framework_value::DslValue::float(value);
    let edits = [
        ("set-info-field", semio_framework_value::DslValue::object([("object".into(), text("title")), ("field".into(), text("text")), ("value".into(), text("Edited Title"))])),
        ("insert-text", semio_framework_value::DslValue::object([("page".into(), number(1.0)), ("x".into(), number(72.0)), ("y".into(), number(700.0)), ("height".into(), number(12.0)), ("text".into(), text("Hello"))])),
    ];
    for (action, args) in edits {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            &mut app, action, Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap_or_else(|fault| panic!("{action} dispatch refused: {fault:?}"));
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap_or_else(|fault| panic!("{action} settlement refused: {fault:?}"));
    }
    let bytes = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
    if let Some(path) = std::env::var_os("SEMIO_PDF_SAVE_IMPORTED_OUTPUT") { std::fs::write(path, &bytes).unwrap(); }
    assert_eq!(oracle::reference_page_count(&bytes).unwrap(), 2);
    let operators = oracle::reference_page_operator_names(&bytes).unwrap();
    assert!(operators[0].iter().any(|operator| operator == "Tj"), "lopdf reads the inserted text on page one: {operators:?}");
    assert!(operators[1].is_empty(), "page two keeps its empty content: {operators:?}");
    assert!(oracle::project_pdf_1_7(&bytes).unwrap().to_string().contains("Edited Title"));
    let mut reopened_app = open_registered_pdf_through(bytes, 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let reopened = reopened_app.snapshot().unwrap();
    assert_eq!(reopened.info.title.as_deref(), Some("Edited Title"));
    assert_eq!(reopened.info.author, original.info.author);
    assert!(crate::editor::page::objects(&reopened).iter().any(|object| object.text == "Hello"));
    assert_eq!(reopened.pages[1], original.pages[1]);
    assert_eq!(reopened.fonts, original.fonts);
    laws::close_registered_fixture_app(&mut reopened_app, policy, scope.cx()).expect("reopened editor closes");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

type PdfApp = semio_plugin::VcsArtifactApp<semio_plugin::EditorApp<Pdf17Editor>>;
type PdfEdit = (&'static str, semio_framework_value::DslValue);

/// ✏️ The title edit the PDF shell dispatches for the Info panel.
fn title_edit(value: &str) -> PdfEdit {
    let text = |value: &str| semio_framework_value::DslValue::String(value.into());
    ("set-info-field", semio_framework_value::DslValue::object([("object".into(), text("title")), ("field".into(), text("text")), ("value".into(), text(value))]))
}

/// ✏️ The insert-text edit of a one-based page at the usual body position.
fn insert_text_edit(page: f64, value: &str) -> PdfEdit {
    let number = |value: f64| semio_framework_value::DslValue::float(value);
    ("insert-text", semio_framework_value::DslValue::object([("page".into(), number(page)), ("x".into(), number(72.0)), ("y".into(), number(700.0)), ("height".into(), number(12.0)), ("text".into(), semio_framework_value::DslValue::String(value.into()))]))
}

/// ✏️ Dispatches each edit through the registered app and settles it before the next one.
async fn edit_open_pdf(app: &mut PdfApp, scope: &mut semio_plugin::artifact_app_laws::OriginalFixtureScope, edits: Vec<PdfEdit>) {
    use semio_plugin::artifact_app_laws as laws;
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    for (action, args) in edits {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            app, action, Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap_or_else(|fault| panic!("{action} dispatch refused: {fault:?}"));
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap_or_else(|fault| panic!("{action} settlement refused: {fault:?}"));
    }
}

/// 📎️ The saved file starts with every byte of its base and adds a bounded section.
fn assert_appended(base: &[u8], saved: &[u8], limit: usize) {
    assert!(saved.starts_with(base), "the saved file must keep every byte of its base");
    assert!(saved.len() > base.len(), "the edit must append a section");
    assert!(saved.len() - base.len() < limit, "the appended section took {} bytes", saved.len() - base.len());
}

/// 🗃️ The file the local custody holds for an open document, whole.
fn custody_file(app: &mut PdfApp) -> Option<Vec<u8>> {
    use crate::standards::v1_7::subsets::base::io::import::original_custody as custody;
    use semio_plugin::PluginApp;
    use semio_framework_value::ByteReadSource;
    let digest = custody::objects_digest(&app.snapshot().unwrap().objects);
    let held = custody::find(&digest, &app.document_identity().unwrap())?;
    let mut bytes = vec![0u8; held.file.byte_len()];
    assert_eq!(held.file.read_bytes(0, &mut bytes), bytes.len());
    Some(bytes)
}

/// 💾️ The registered Save of an opened PDF appends to the retained original (exact prefix; lopdf reads the edit), a second Save stacks on the first, and editing the title back restores the retained Info.
#[semio_framework_async_macros::async_test]
async fn natural_file_route_save_appends_to_the_opened_file_and_stacks_the_second_save() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    use semio_plugin::artifact_app_laws as laws;
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let original = app.snapshot().unwrap().clone();
    assert_eq!(custody_file(&mut app).as_deref(), Some(&input[..]), "opening retains the original file bytes");
    let dump = |name: &str, bytes: &[u8]| if let Some(directory) = std::env::var_os("SEMIO_PDF_INCREMENTAL_DIR") { std::fs::write(std::path::Path::new(&directory).join(name), bytes).unwrap(); };

    edit_open_pdf(&mut app, &mut scope, vec![title_edit("Edited Title"), insert_text_edit(1.0, "Hello")]).await;
    let first = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
    dump("app-two-pages-first.pdf", &first);
    assert_appended(input, &first, 16 * 1024);
    assert_eq!(oracle::reference_page_count(&first).unwrap(), 2);
    let operators = oracle::reference_page_operator_names(&first).unwrap();
    assert!(operators[0].iter().any(|operator| operator == "Tj"), "lopdf reads the inserted text on page one: {operators:?}");
    assert!(operators[1].is_empty(), "page two keeps its empty content: {operators:?}");
    assert!(oracle::project_pdf_1_7(&first).unwrap().to_string().contains("Edited Title"));
    assert_eq!(custody_file(&mut app).as_deref(), Some(first.as_slice()), "the saved file is the next Save's base");

    edit_open_pdf(&mut app, &mut scope, vec![title_edit("Second Title"), insert_text_edit(2.0, "World")]).await;
    let second = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
    dump("app-two-pages-second.pdf", &second);
    assert_appended(&first, &second, 16 * 1024);
    assert!(second.starts_with(input));
    assert_eq!(oracle::reference_page_count(&second).unwrap(), 2);
    let operators = oracle::reference_page_operator_names(&second).unwrap();
    assert!(operators[0].iter().any(|operator| operator == "Tj") && operators[1].iter().any(|operator| operator == "Tj"), "both pages show their text: {operators:?}");
    assert!(oracle::project_pdf_1_7(&second).unwrap().to_string().contains("Second Title"));
    assert_eq!(custody_file(&mut app).as_deref(), Some(second.as_slice()));

    if let Some(title) = original.info.title.clone() {
        edit_open_pdf(&mut app, &mut scope, vec![title_edit(&title)]).await;
        let third = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
        assert_appended(&second, &third, 16 * 1024);
        let projection = oracle::project_pdf_1_7(&third).unwrap().to_string();
        assert!(projection.contains(&title) && !projection.contains("Second Title"), "the retained Info is restored: {projection}");
    }

    let mut reopened_app = open_registered_pdf_through(second, 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    let reopened = reopened_app.snapshot().unwrap();
    assert_eq!(reopened.info.title.as_deref(), Some("Second Title"));
    let texts: Vec<_> = crate::editor::page::objects(&reopened).into_iter().map(|object| object.text).collect();
    assert!(texts.iter().any(|text| text == "Hello") && texts.iter().any(|text| text == "World"), "{texts:?}");
    laws::close_registered_fixture_app(&mut reopened_app, policy, scope.cx()).expect("reopened editor closes");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// 🌐️ The same append-then-stack Save on the generated 120 page document, every open, edit and Save turn under the browser's fuel and interactive grants.
#[semio_framework_async_macros::async_test]
async fn natural_source_route_save_appends_to_a_120_page_document_under_the_browser_grants() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    use semio_plugin::artifact_app_laws as laws;
    let spec = serde_json::json!({ "pageCount": 120, "segmentCount": 4, "labelPrefix": "Page", "font": { "name": "helv", "face": "Helvetica", "encoding": "WinAnsiEncoding" } });
    let input = large_pdf(&spec);
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let browser = Some((BROWSER_TURN_FUEL, policy.preparation));
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.clone(), 120, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    assert_eq!(custody_file(&mut app).as_deref(), Some(input.as_slice()), "opening retains the original file bytes");
    let dump = |name: &str, bytes: &[u8]| if let Some(directory) = std::env::var_os("SEMIO_PDF_INCREMENTAL_DIR") { std::fs::write(std::path::Path::new(&directory).join(name), bytes).unwrap(); };

    edit_open_pdf(&mut app, &mut scope, vec![insert_text_edit(1.0, "Hello")]).await;
    let first = save_natural_bytes_under(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false, browser).await;
    dump("app-large-first.pdf", &first);
    assert_appended(&input, &first, 16 * 1024);
    assert_eq!(oracle::reference_page_count(&first).unwrap(), 120);
    let operators = oracle::reference_page_operator_names(&first).unwrap();
    assert!(operators[0].iter().any(|operator| operator == "Tj") && operators[119].iter().filter(|operator| *operator == "Tj").count() == 1, "only page one gained text");

    edit_open_pdf(&mut app, &mut scope, vec![insert_text_edit(120.0, "World")]).await;
    let second = save_natural_bytes_under(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false, browser).await;
    dump("app-large-second.pdf", &second);
    assert_appended(&first, &second, 16 * 1024);
    assert_eq!(oracle::reference_page_count(&second).unwrap(), 120);
    let operators = oracle::reference_page_operator_names(&second).unwrap();
    assert_eq!(operators[119].iter().filter(|operator| *operator == "Tj").count(), 2, "page 120 gained its text");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// 🛑️ Cancelling a Save mid-way publishes nothing and leaves the custody base untouched; the next Save is the complete one.
#[semio_framework_async_macros::async_test]
async fn cancelling_an_opened_pdf_save_midway_publishes_nothing_and_keeps_the_base() {
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf_through(input.to_vec(), 2, &mut scope, PdfIntake::NaturalSource, BROWSER_TURN_FUEL).await;
    edit_open_pdf(&mut app, &mut scope, vec![title_edit("Cancelled"), insert_text_edit(1.0, "Hello")]).await;
    let revision = app.document_base_revision().unwrap();
    let document = app.document_identity().unwrap();
    app.bind_instance_id(meta.instance_id).await;
    let (fuel, grant) = (laws::OriginalFixtureCaller::turn_fuel(), scope.cx().retained_grant());
    let mut submitted = None;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        laws::fixture_renew_turn(scope.cx(), fuel, grant).expect("PDF Save submission turn is funded");
        if app.submit_media_export_checked(semio_plugin::NATURAL_FILE_PORT, &document, revision, None, &mut submitted, scope.cx()).await.expect("checked PDF Save submission") { break }
    }
    let handle = submitted.expect("registered PDF Save handle");
    for _ in 0..2 {
        laws::fixture_renew_turn(scope.cx(), fuel, grant).expect("PDF Save poll turn is funded");
        assert!(matches!(laws::poll_fixture_media_export(&mut app, &handle, scope.cx()).await.expect("registered PDF Save poll"), semio_plugin::ArtifactMediaExportPoll::Running { .. }), "the Save must still be running when it is cancelled");
    }
    laws::fixture_renew_turn(scope.cx(), fuel, grant).expect("PDF Save cancel turn is funded");
    PluginApp::cancel_media_export(&mut app, &handle).await.expect("registered PDF Save cancels");
    let mut published = false;
    for _ in 0..laws::OriginalFixtureCaller::maximum_turns() {
        laws::fixture_renew_turn(scope.cx(), fuel, grant).expect("PDF Save poll turn is funded");
        match laws::poll_fixture_media_export(&mut app, &handle, scope.cx()).await {
            Ok(semio_plugin::ArtifactMediaExportPoll::Running { .. }) => {}
            Ok(semio_plugin::ArtifactMediaExportPoll::Complete(_)) => { published = true; break }
            _ => break,
        }
    }
    assert!(!published, "a cancelled Save publishes no file");
    assert_eq!(custody_file(&mut app).as_deref(), Some(&input[..]), "a cancelled Save leaves the retained original untouched");
    let saved = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, true).await;
    assert_appended(input, &saved, 16 * 1024);
    assert_eq!(custody_file(&mut app).as_deref(), Some(saved.as_slice()));
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

/// 🆕️ A document that was never opened from a file has no retained original: its Save is the compact writer's and nothing is rebased.
#[semio_framework_async_macros::async_test]
async fn a_new_pdf_document_still_saves_compactly() {
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base as oracle;
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    let codec = <Pdf17Editor as ArtifactEditor>::natural_file_codec().expect("PDF editor declares paired natural IO");
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    laws::ensure_fixture_cancellation_issuer(&mut app, scope.cx()).expect("mounted PDF instance is born");
    app.render_ctx().refresh_cache().await.expect("mounted PDF head is projected");
    edit_open_pdf(&mut app, &mut scope, vec![title_edit("Brand New")]).await;
    let saved = save_natural_bytes(&mut app, &mut scope, meta.instance_id, &*codec.format_kind, &*codec.media_type, false).await;
    assert!(saved.starts_with(b"%PDF-"));
    assert_eq!(oracle::reference_page_count(&saved).unwrap(), 1);
    assert!(oracle::project_pdf_1_7(&saved).unwrap().to_string().contains("Brand New"));
    assert_ne!(custody_file(&mut app).as_deref(), Some(saved.as_slice()), "a compact Save is not rebased");
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PDF editor closes");
}

fn large_pdf(spec: &serde_json::Value) -> Vec<u8> {
    use std::io::Write;
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let lines = spec["segmentCount"].as_u64().unwrap() as usize;
    let font = spec.get("font");
    let first_page = if font.is_some() { 5 } else { 3 };
    let page_stride = if font.is_some() { 3 } else { 2 };
    let kids = (0..pages).map(|page| format!("{} 0 R", first_page + page * page_stride)).collect::<Vec<_>>().join(" ");
    let mut rows = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [{kids}] /Count {pages} >>").into_bytes(),
    ];
    if let Some(font) = font {
        let face = font["face"].as_str().unwrap();
        let encoding = font["encoding"].as_str().unwrap();
        let name = font["name"].as_str().unwrap();
        rows.push(format!("<< /Type /Font /Subtype /Type1 /BaseFont /{face} /Encoding /{encoding} >>").into_bytes());
        rows.push(format!("<< /Font << /{name} 3 0 R >> >>").into_bytes());
    }
    for page in 0..pages {
        let mut content = b"q 0.2 0.3 0.4 RG 0.5 w\n".to_vec();
        for index in 0..lines {
            let at = 20 + index * 6;
            writeln!(content, "{at} 40 m {at} {} l", 400 + page).unwrap();
        }
        content.extend_from_slice(b"S Q\n");
        let contents = rows.len() + 2;
        let contents_and_resources = if font.is_some() {
            format!("/Resources 4 0 R /Contents [{contents} 0 R {} 0 R]", contents + 1)
        } else {
            format!("/Contents {contents} 0 R")
        };
        rows.push(format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 360 480] {contents_and_resources} >>").into_bytes());
        if let Some(font) = font {
            let name = font["name"].as_str().unwrap();
            let label = pdf_page_label(spec, page);
            rows.push(pdf_fixture_stream(format!("BT /{name} 11 Tf 48 416 Td ({label}) Tj ET\n").as_bytes()));
        }
        rows.push(pdf_fixture_stream(&content));
    }
    pdf_fixture_file(rows)
}

fn pdf_fixture_file(rows: Vec<Vec<u8>>) -> Vec<u8> {
    use std::io::Write;
    let mut output = b"%PDF-1.7\n".to_vec();
    let positions: Vec<_> = rows.into_iter().enumerate().map(|(index, row)| {
        let position = output.len();
        writeln!(output, "{} 0 obj", index + 1).unwrap();
        output.extend(row);
        output.extend_from_slice(b"\nendobj\n");
        position
    }).collect();
    let offset = output.len();
    let size = positions.len() + 1;
    write!(output, "xref\n0 {size}\n0000000000 65535 f \n").unwrap();
    for position in positions { writeln!(output, "{position:010} 00000 n ").unwrap(); }
    write!(output, "trailer\n<< /Size {size} /Root 1 0 R >>\nstartxref\n{offset}\n%%EOF\n").unwrap();
    output
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_large_retained_vector_document_and_reexports() {
    use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::PdfOp};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;
    let fixtures: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let spec = &fixtures["largeVector"];
    let input = large_pdf(spec);
    assert_eq!(input.len(), spec["byteLength"].as_u64().unwrap() as usize);
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let lines = spec["segmentCount"].as_u64().unwrap() as usize;
    let reference = decode_pdf(&input).expect("the cold PDF reader projects the fixture");
    let oracle: serde_json::Value = serde_json::from_str(&project_pdf_1_7(&input).unwrap().to_string()).unwrap();
    assert_eq!(oracle["pageCount"].as_u64().unwrap() as usize, pages);
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input, pages, &mut scope).await;
    let snapshot = app.snapshot().unwrap();
    assert_eq!(snapshot.pages, reference.pages);
    assert_eq!(snapshot.objects, reference.objects);
    assert_eq!(snapshot.trailer, reference.trailer);
    assert_eq!(snapshot.admitted_stream_roles, reference.admitted_stream_roles);
    assert_eq!(snapshot.pages.last().unwrap().content.iter().filter(|operator| matches!(operator, PdfOp::LineTo { .. })).count(), lines);
    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("controlled PDF reexports").expect("natural PDF output is present");
    let saved: serde_json::Value = serde_json::from_str(&project_pdf_1_7(&artifact.data).unwrap().to_string()).unwrap();
    assert_eq!(saved["pageCount"].as_u64().unwrap() as usize, pages);
    assert_eq!(decode_pdf(&artifact.data).unwrap().pages, reference.pages);
    laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("large editor closes");
}

fn pdf_fixture_stream(content: &[u8]) -> Vec<u8> {
    let mut stream = format!("<< /Length {} >>\nstream\n", content.len()).into_bytes();
    stream.extend_from_slice(content);
    stream.extend_from_slice(b"endstream");
    stream
}

fn pdf_page_label(spec: &serde_json::Value, index: usize) -> String {
    let label = spec["labelPrefix"].as_str().unwrap();
    let pages = spec["pageCount"].as_u64().unwrap();
    format!("{label} {:02} of {pages}", index + 1)
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_large_retained_type1_document_and_reexports() {
    use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::PdfFontKind};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7;

    let corpus: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let spec = &corpus["largeType1"];
    let pages = spec["pageCount"].as_u64().unwrap() as usize;
    let input = large_pdf(spec);
    assert!(input.len() > spec["minimumByteLength"].as_u64().unwrap() as usize);
    let reference = decode_pdf(&input).unwrap();
    let check_independent_text = |bytes: &[u8]| {
        let witness: serde_json::Value = serde_json::from_str(&project_pdf_1_7(bytes).unwrap().to_string()).unwrap();
        let actual = witness["pages"].as_array().unwrap();
        assert_eq!(actual.len(), pages);
        for (index, page) in actual.iter().enumerate() {
            assert_eq!(page["text"].as_array().unwrap().as_slice(), &[serde_json::Value::String(pdf_page_label(spec, index))]);
        }
    };
    check_independent_text(&input);
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = open_registered_pdf(input, pages, &mut scope).await;
    let snapshot = app.snapshot().unwrap();
    assert_eq!(snapshot.pages, reference.pages);
    assert_eq!(snapshot.fonts, reference.fonts);
    assert_eq!(snapshot.objects, reference.objects);
    assert_eq!(snapshot.trailer, reference.trailer);
    assert_eq!(snapshot.admitted_stream_roles, reference.admitted_stream_roles);
    let [font] = snapshot.fonts.as_slice() else { panic!("expected the shared Type1 font"); };
    assert_eq!(font.id, spec["font"]["name"].as_str().unwrap());
    assert!(matches!(&font.kind, PdfFontKind::Type1 { base_font, .. } if base_font == spec["font"]["face"].as_str().unwrap()));
    for (index, page) in snapshot.pages.iter().enumerate() {
        assert_eq!(page.text().trim(), pdf_page_label(spec, index));
    }
    let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.expect("controlled text PDF reexports").expect("natural PDF output is present");
    check_independent_text(&artifact.data);
    assert_eq!(decode_pdf(&artifact.data).unwrap().pages, reference.pages);
    laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("large text editor closes");
}

#[semio_framework_async_macros::async_test]
async fn create_pdf17_editor_builds_a_definition_for_the_editor_role() {
    let def = create_pdf17_editor();
    assert_eq!(def.role, semio_plugin::AppRole::Editor);
    assert_eq!(def.dialect, PDF17_DIALECT.into());
}

#[semio_framework_async_macros::async_test]
async fn editor_dialect_matches_the_artifact_coordinate() {
    assert_eq!(<Pdf17Editor as ArtifactEditor>::DIALECT, PDF17_DIALECT);
}

#[semio_framework_async_macros::async_test]
async fn editor_declares_the_main_window() {
    let def = create_pdf17_editor();
    assert!(def.window_kinds.iter().any(|w| w.id == main::WINDOW_KIND_ID));
}

#[semio_framework_async_macros::async_test]
async fn missing_set_page_payload_is_rejected() {
    assert!(<Pdf17Editor as ArtifactEditor>::command_from_action("set-page", &mut None, &mut None).is_err());
}

#[semio_framework_async_macros::async_test]
async fn explicit_nonzero_page_payload_is_preserved() {
    let args = semio_framework_value::DslValue::Object(vec![
        ("page".into(), semio_framework_value::DslValue::float(3.0)),
        ("item".into(), semio_framework_value::DslValue::float(0.0)),
        ("revision".into(), semio_framework_value::DslValue::String("0123456789abcdef".into())),
        ("text".into(), semio_framework_value::DslValue::String("replacement".into())),
    ]);
    let command = <Pdf17Editor as ArtifactEditor>::command_from_action("set-page", &mut Some(args), &mut None).expect("typed payload");
    assert!(matches!(command, semio_s_artifact_stdio_contract::editing::SnapshotEditingCommand::Native(Pdf17EditorCommand::SetPage { page: 3, item: 0, revision, text }) if revision == "0123456789abcdef" && text == "replacement"));
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_draft_publishes_once_refuses_stale_and_undoes_redoes() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfPage, PdfTextString};
    use semio_plugin::app::DocumentWindowKit;
    use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = PdfSnapshot::default();
    original.pages.push(PdfPage { content: vec![PdfOp::BeginText, PdfOp::ShowText { text: PdfTextString::text("before") }, PdfOp::EndText], ..Default::default() });
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).unwrap();
    let loaded = app.snapshot().unwrap().clone();
    assert_eq!(loaded.pages[0].text(), "before");
    let meta = artifact_app_laws::meta("local");
    let arguments = |revision: String, text: &str| {
        semio_framework_value::DslValue::object([("page".into(), semio_framework_value::DslValue::float(0.0)), ("item".into(), semio_framework_value::DslValue::float(0.0)), ("revision".into(), semio_framework_value::DslValue::String(revision)), ("text".into(), semio_framework_value::DslValue::String(text.into()))])
    };

    let edit = arguments(DocumentWindowKit::text_revision("before"), "after");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(edit.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    let expected = app.snapshot().unwrap().clone();
    assert_eq!(expected.pages[0].text(), "after");

    let no_op = arguments(DocumentWindowKit::text_revision("after"), "after");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(no_op.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), loaded, "an identical Apply must not add a history entry");
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), expected);

    let stale = arguments(DocumentWindowKit::text_revision("before"), "refused draft");
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "set-page", Some(stale.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    assert!(semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).is_err());
    assert_eq!(app.snapshot().unwrap(), expected, "a refused draft must preserve every PDF page operator");
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_page_edit_publishes_and_undoes_redoes() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::PdfOp;
    use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = crate::standards::v1_7::subsets::base::io::export::demo_pdf17_snapshot();
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).unwrap();
    let loaded = app.snapshot().unwrap().clone();
    let meta = artifact_app_laws::meta("local");
    let insert = semio_framework_value::DslValue::object([
        ("page".into(), semio_framework_value::DslValue::float(1.0)),
        ("x".into(), semio_framework_value::DslValue::float(12.0)),
        ("y".into(), semio_framework_value::DslValue::float(24.0)),
        ("width".into(), semio_framework_value::DslValue::float(30.0)),
        ("height".into(), semio_framework_value::DslValue::float(16.0)),
        ("red".into(), semio_framework_value::DslValue::float(0.2)),
        ("green".into(), semio_framework_value::DslValue::float(0.4)),
        ("blue".into(), semio_framework_value::DslValue::float(0.6)),
    ]);
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, "insert-rectangle", Some(insert.clone()), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
    let edited = app.snapshot().unwrap().clone();
    assert!(edited.pages[0].content.iter().any(|op| matches!(op, PdfOp::Rectangle { x, y, width, height } if (*x - 12.0).abs() < 0.01 && (*y - 24.0).abs() < 0.01 && (*width - 30.0).abs() < 0.01 && (*height - 16.0).abs() < 0.01)));
    assert!(edited.pages[0].text().contains("Semio"));
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), loaded);
    semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
    assert_eq!(app.snapshot().unwrap(), edited);
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

semio_plugin::history_edit_acceptance_law!("stdio", Pdf17Editor, || semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() }, "../../../../../..");

fn qpdf_accepts(bytes: &[u8], name: &str) {
    let path = std::env::temp_dir().join(format!("semio-{name}-{}.pdf", std::process::id()));
    std::fs::write(&path, bytes).unwrap();
    let output = std::process::Command::new("qpdf").arg("--check").arg(&path).output();
    let _ = std::fs::remove_file(&path);
    match output {
        Ok(output) => assert!(output.status.success(), "qpdf --check refused the saved PDF: {}", String::from_utf8_lossy(&output.stderr)),
        Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound, "qpdf could not run: {error}"),
    }
}

#[test]
fn every_pdf_17_subset_editor_opens_and_saves_the_same_natural_file() {
    use crate::editor::{pdf17a::Pdf17AEditor, pdf17e::Pdf17EEditor, pdf17h::Pdf17HEditor, pdf17ua::Pdf17UaEditor, pdf17vt::Pdf17VtEditor, pdf17x::Pdf17XEditor};
    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let base = <Pdf17Editor as ArtifactEditor>::decode_natural_file(input).expect("natural PDF opens");
    let expected = <Pdf17Editor as ArtifactEditor>::encode_natural_file(&base).unwrap();
    macro_rules! assert_subset {
        ($editor:ty, $kind:expr) => {{
            let codec = <$editor as ArtifactEditor>::natural_file_codec().expect("every subset editor declares the natural PDF file");
            assert_eq!((codec.format_kind, codec.extension, codec.media_type, codec.binary), ($kind, ".pdf", "application/pdf", true));
            let opened = <$editor as ArtifactEditor>::decode_natural_file(input).expect("natural PDF opens");
            let saved = <$editor as ArtifactEditor>::encode_natural_file(&opened).unwrap();
            assert_eq!(saved, expected);
            qpdf_accepts(&saved, stringify!($editor));
            assert!(<$editor as ArtifactEditor>::decode_natural_file(b"%PDF-1.7 truncated").is_err());
        }};
    }
    assert_subset!(Pdf17AEditor, "s.stdio.pdf@1.7/a");
    assert_subset!(Pdf17EEditor, "s.stdio.pdf@1.7/e");
    assert_subset!(Pdf17HEditor, "s.stdio.pdf@1.7/h");
    assert_subset!(Pdf17UaEditor, "s.stdio.pdf@1.7/ua");
    assert_subset!(Pdf17VtEditor, "s.stdio.pdf@1.7/vt");
    assert_subset!(Pdf17XEditor, "s.stdio.pdf@1.7/x");
}

#[test]
fn text_layer_lists_a_real_document_in_reading_order_with_localized_labels() {
    use semio_ui_locale::Locale;
    let snapshot = <Pdf17Editor as ArtifactEditor>::decode_natural_file(fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/🔀️move-page/⬅️before.pdf")).expect("natural PDF opens");
    let english = crate::editor::page::text_layer_lines(&snapshot, Locale::En);
    let german = crate::editor::page::text_layer_lines(&snapshot, Locale::De);
    assert_eq!(english.len(), german.len());
    let texts = |lines: &[(String, String)]| lines.iter().map(|(_, line)| line.clone()).collect::<Vec<_>>();
    let english = texts(&english);
    let german = texts(&german);
    assert!(english.iter().any(|line| line.starts_with("Language: ")) && german.iter().any(|line| line.starts_with("Sprache: ")));
    assert!(english.iter().any(|line| line.starts_with("Tagged PDF: ")) && german.iter().any(|line| line.starts_with("Getaggtes PDF: ")));
    let first = english.iter().position(|line| line == "Page 1 / 2").expect("page one heading");
    let second = english.iter().position(|line| line == "Page 2 / 2").expect("page two heading");
    assert_eq!(german.iter().position(|line| line == "Seite 2 / 2"), Some(second));
    let page_one_text = snapshot.pages[0].text();
    let found = english.iter().position(|line| line.starts_with("Text: ") && page_one_text.contains(line.trim_start_matches("Text: "))).expect("page one text is listed");
    assert!(first < found && found < second, "page one text sits between the page headings");
    let _arena = semio_plugin::artifact_app_laws::FixtureUiArena::admit();
    let panel = crate::editor::page::render_text_layer(&snapshot, "s.stdio.pdf@1.7/*#editor", Locale::De).unwrap();
    semio_plugin::artifact_app_laws::observe_and_retire_fixture_tree(semio_plugin::built_to_component_tree(panel), |panel| {
        let page_two = &panel.children[second];
        let binding = &page_two.bindings[0];
        assert_eq!(binding.action.scope.as_str(), "s.stdio.pdf@1.7/*#editor");
        assert_eq!(binding.action.name.as_str(), semio_plugin::INTERACTION_SELECT_ACTION_ID);
        let Some(semio_ui_contract::UiValue::Map(args)) = &binding.args else { panic!("accessible page selection") };
        assert!(args.iter().any(|(key, value)| key.as_str() == "targets" && matches!(value, semio_ui_contract::UiValue::Text(text) if text.as_str().contains("p1:paper"))));
    });
}

//#region 🧪️Controlled image natural file

fn sampled_fixture_bytes(spec: &serde_json::Value, mixed: bool) -> (Vec<u8>, Vec<(u32, Vec<u8>)>) {
    use serde_json::Value;
    let dimensions = |key| serde_json::from_value::<[usize; 2]>(spec[key].clone()).unwrap();
    let [width, height] = dimensions("imageSize");
    let [page_width, page_height] = dimensions("pageSize");
    let [x, y] = dimensions("imageOrigin");
    let images = spec["images"].as_array().unwrap();
    let samples = |image: &Value| {
        let quadrants: [Vec<u8>; 4] = serde_json::from_value(image["samples"].clone()).unwrap();
        let row = |index: usize| [quadrants[index].repeat(width / 2), quadrants[index + 1].repeat(width / 2)].concat();
        [row(0).repeat(height / 2), row(2).repeat(height / 2)].concat()
    };
    let source_samples: Vec<_> = images.iter().map(|image| (8, samples(image))).collect();
    let resources = |image: &Value| {
        let name = image["name"].as_str().unwrap();
        let object = image["object"].as_u64().unwrap();
        let font = if mixed { "/Font << /F1 15 0 R >> " } else { "" };
        format!("<< {font}/XObject << /{name} {object} 0 R >> >>")
    };
    let page = |contents, entry: &str| {
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {page_width} {page_height}] /Contents {contents} 0 R{entry} >>").into_bytes()
    };
    let stream = |fields: &str, body: &[u8]| {
        let mut stream = format!("<< /Length {}{fields} >>\nstream\n", body.len()).into_bytes();
        stream.extend_from_slice(body);
        stream.extend_from_slice(b"\nendstream");
        stream
    };
    let content = |image: &Value, index: usize| {
        let name = image["name"].as_str().unwrap();
        let mut body = format!("q {width} 0 0 {height} {x} {y} cm /{name} Do Q\n");
        if mixed {
            let text = spec["mixedText"][index].as_str().unwrap();
            body.push_str(&format!("BT /F1 12 Tf 20 102 Td ({text}) Tj ET\n"));
        }
        stream("", body.as_bytes())
    };
    let picture = |index: usize| {
        let image = &images[index];
        let space = image["space"].as_str().unwrap();
        let mut fields = format!(" /Type /XObject /Subtype /Image /Width {width} /Height {height} /ColorSpace /{space} /BitsPerComponent 8");
        let samples = &source_samples[index].1;
        let body = if let Some(filter) = image["filter"].as_str() {
            fields.push_str(&format!(" /Filter /{filter}"));
            fixture_stored_zlib(samples)
        } else { samples.clone() };
        stream(&fields, &body)
    };
    let mut rows = vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        format!("<< /Type /Pages /Kids [3 0 R 6 0 R 9 0 R 13 0 R] /Count 4 /Resources {} >>", resources(&images[0])).into_bytes(),
        page(4, ""), content(&images[0], 0), picture(0),
        page(7, &format!(" /Resources {}", resources(&images[1]))), content(&images[1], 1), picture(1),
        page(12, " /Resources 10 0 R"), resources(&images[2]).into_bytes(), picture(2), content(&images[2], 2),
        page(14, ""), content(&images[0], 3),
    ];
    if mixed {
        rows.push(b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding /WinAnsiEncoding >>".to_vec());
    }
    (pdf_fixture_file(rows), source_samples)
}

fn fixture_stored_zlib(samples: &[u8]) -> Vec<u8> {
    let length = u16::try_from(samples.len()).unwrap();
    let mut out = vec![0x78, 0x01, 0x01];
    out.extend_from_slice(&length.to_le_bytes());
    out.extend_from_slice(&(!length).to_le_bytes());
    out.extend_from_slice(samples);
    let (a, b) = samples.iter().fold((1u32, 0u32), |(a, b), byte| {
        let a = (a + u32::from(*byte)) % 65_521;
        (a, (b + a) % 65_521)
    });
    out.extend_from_slice(&((b << 16) | a).to_be_bytes());
    out
}

#[semio_framework_async_macros::async_test]
async fn natural_file_route_opens_reexports_and_reopens_large_sampled_images() {
    use crate::standards::v1_7::subsets::base::{
        io::import::decode_pdf, schema::{snapshot::{ObjRef, PdfOp}, stream_roles::PdfStreamRoleValue},
    };
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{project_pdf_1_7, reference_image_sample_streams};

    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🎞️controlled-images/🔣️.json")).unwrap();
    for mixed in [false, true] {
        let (input, samples) = sampled_fixture_bytes(&spec, mixed);
        assert!(input.len() > spec["minimumByteLength"].as_u64().unwrap() as usize);
        assert_eq!(reference_image_sample_streams(&input).unwrap(), samples);
        let verify_text = |data: &[u8]| {
            if mixed {
                let projected: serde_json::Value = serde_json::from_str(&project_pdf_1_7(data).unwrap().to_string()).unwrap();
                for (page, text) in projected["pages"].as_array().unwrap().iter().zip(spec["mixedText"].as_array().unwrap()) {
                    assert_eq!(page["text"].as_array().unwrap().as_slice(), std::slice::from_ref(text));
                }
            }
        };
        verify_text(&input);
        let cold = decode_pdf(&input).unwrap();
        let pages = spec["pages"].as_array().unwrap();
        let mut scope = laws::OriginalFixtureScope::admit();
        let mut app = open_registered_pdf(input, pages.len(), &mut scope).await;
        let mounted = app.snapshot().unwrap();
        assert_eq!(mounted.pages, cold.pages);
        assert_eq!(mounted.images, cold.images);
        assert_eq!(mounted.fonts, cold.fonts);
        assert_eq!(mounted.objects, cold.objects);
        assert_eq!(mounted.trailer, cold.trailer);
        assert_eq!(mounted.admitted_stream_roles, cold.admitted_stream_roles);
        for (image, role) in mounted.images.iter().zip(&mounted.admitted_stream_roles) {
            let spec = spec["images"].as_array().unwrap().iter().find(|spec| spec["name"].as_str() == Some(image.id.as_str())).unwrap();
            assert_eq!(role.identity.owner, ObjRef { num: spec["object"].as_u64().unwrap() as u32, gen: 0 });
            assert!(role.identity.path.is_empty());
            assert_eq!(role.dependencies.as_slice(), std::slice::from_ref(&role.identity));
            assert!(matches!(&role.value, PdfStreamRoleValue::Image { image: owned } if owned == image));
        }
        for (page, expected) in mounted.pages.iter().zip(pages) {
            let image = expected["image"].as_u64().unwrap() as usize;
            let name = spec["images"][image]["name"].as_str().unwrap();
            assert!(matches!(&page.content[1..3], [PdfOp::Transform { matrix }, PdfOp::PaintXObject { name: actual }]
                if actual == name && *matrix == [72., 0., 0., 72., 20., 20.]));
        }

        let artifact = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
            PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
        }).await.expect("mounted PDF reexports").expect("natural PDF output");
        assert_eq!(reference_image_sample_streams(&artifact.data).unwrap(), samples);
        verify_text(&artifact.data);
        let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
        laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("original image editor closes");
        let mut reopened = open_registered_pdf(artifact.data, pages.len(), &mut scope).await;
        assert_eq!(reopened.snapshot().unwrap().images, cold.images);
        assert_eq!(reopened.snapshot().unwrap().pages, cold.pages);
        laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).expect("reopened image editor closes");
    }
}

//#endregion 🧪️Controlled image natural file

#[semio_framework_async_macros::async_test]
async fn registered_pdf_font_change_preserves_encoded_text_and_the_next_line() {
    use crate::editor::page::{objects, ObjectKind};
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfOp, PdfTextArrayItem};
    use semio_plugin::{artifact_app_laws as laws, PluginApp};
    use semio_framework_value::DslValue;
    use serde_json::json;

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/📄️natural-file-edit/🔣️.json")).unwrap();
    let case = &fixture["fontReassignment"];
    let size = &fixture["fontSize"];
    let byte = case["sourceCode"].as_u64().unwrap();
    let adjust = case["kerning"].as_f64().unwrap();
    let alternate = case["sourceAlternateCode"].as_u64().unwrap();
    let program = format!("BT /Fsrc {} Tf 10 50 Td 24 TL [<{byte:02X}> {adjust} <{byte:02X}>] TJ [<{}> -20] TJ T* ({}) Tj ET", size["before"], size["sameLineCode"].as_str().unwrap(), size["followingLine"].as_str().unwrap());
    let input = pdf_fixture_file(vec![
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
        b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 180 90] /Resources << /Font << /Fsrc 5 0 R /Fdst 6 0 R >> >> /Contents 4 0 R >>".to_vec(),
        pdf_fixture_stream(program.as_bytes()),
        format!("<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica /Encoding << /BaseEncoding /WinAnsiEncoding /Differences [{alternate} /eacute] >> >>").into_bytes(),
        b"<< /Type /Font /Subtype /Type1 /BaseFont /Times-Roman /Encoding /MacRomanEncoding >>".to_vec(),
    ]);
    let mut scope = laws::OriginalFixtureScope::admit();
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut app = open_registered_pdf(input, 1, &mut scope).await;
    let text_objects = |snapshot: &PdfSnapshot| objects(snapshot).into_iter().filter(|object| object.kind == ObjectKind::Text).collect::<Vec<_>>();
    let before = text_objects(&app.snapshot().unwrap());
    assert_eq!(before[0].text.as_str(), case["firstLine"].as_str().unwrap());
    assert_eq!(before[1].text.as_str(), size["sameLine"].as_str().unwrap());
    assert_eq!(before[2].text.as_str(), size["followingLine"].as_str().unwrap());
    let first = before[0].id.as_str();
    let original_same_line = app.snapshot().unwrap().pages[0].content[before[1].end].clone();
    for (action, args) in [
        ("set-font", json!({"page": 1, "object": first, "text": case["targetFont"]})),
        ("set-font-size", json!({"page": 1, "object": first, "field": "size", "value": size["after"]})),
    ] {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            &mut app, action, Some(DslValue::from(args.clone())), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap();
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap();
    }

    let verify = |snapshot: &PdfSnapshot, from_original: bool| {
        let after = text_objects(snapshot);
        assert_eq!(after[0].text, before[0].text);
        assert_eq!((after[0].x, after[0].y), (before[0].x, before[0].y));
        assert_eq!(after[0].font_size, size["after"].as_f64().unwrap());
        assert_eq!(after.len(), before.len());
        assert_eq!(snapshot.pages[0].content[after[1].end], original_same_line);
        for (after, before) in after[1..].iter().zip(&before[1..]) {
            assert_eq!((&after.text, after.font_size), (&before.text, before.font_size));
            assert!((after.x - before.x).abs() < 1e-5 && (after.y - before.y).abs() < 1e-5);
        }
        for (object, expected) in after.iter().zip(["targetFont", "sourceFont", "sourceFont"]) {
            assert_eq!(snapshot.font(&object.font).unwrap().base_font(), case[expected].as_str().unwrap());
        }
        let PdfOp::ShowTextArray { items } = &snapshot.pages[0].content[after[0].end] else { panic!("the edited text array retains its spacing"); };
        assert!(items.iter().any(|item| matches!(item, PdfTextArrayItem::Adjust { amount } if *amount == adjust)));
        if from_original {
            assert!(matches!(&items[0], PdfTextArrayItem::Text { text } if text == "é"));
        }
    };
    verify(&app.snapshot().unwrap(), true);
    let saved = semio_s_artifact_stdio_contract::testing::with_encoding_async(async |owner| {
        PluginApp::produce_media(&mut app, semio_plugin::NATURAL_FILE_PORT, owner, scope.cx()).await
    }).await.unwrap().unwrap();
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).unwrap();
    let mut reopened = open_registered_pdf(saved.data, 1, &mut scope).await;
    verify(&reopened.snapshot().unwrap(), false);
    laws::close_registered_fixture_app(&mut reopened, policy, scope.cx()).unwrap();
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_with_several_fonts_images_and_inherited_resources_opens_like_both_reference_readers() {
    use crate::standards::v1_7::subsets::base::{io::import::decode_pdf, schema::snapshot::PdfOp};
    use semio_plugin::artifact_app_laws as laws;
    use semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::{project_pdf_1_7, reference_page_resources};

    let spec: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🧩️multi-resource/spec.json")).unwrap();
    let inputs = [("generated", fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/🧩️multi-resource/generated.pdf")), ("report", fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/🧩️multi-resource/report.pdf"))];
    for (name, input) in inputs {
        let case = &spec[name];
        let pages = case["pages"].as_array().unwrap();
        let strings = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_owned()).collect::<Vec<_>>();
        let sorted = |mut values: Vec<String>| { values.sort(); values.dedup(); values };
        let cold = decode_pdf(input).unwrap_or_else(|error| panic!("{name}: the cold reader refuses the fixture: {error:?}"));
        let independent = reference_page_resources(input).unwrap();
        let projected: serde_json::Value = serde_json::from_str(&project_pdf_1_7(input).unwrap().to_string()).unwrap();
        assert_eq!(independent.len(), pages.len(), "{name}");
        assert_eq!(projected["pageCount"].as_u64().unwrap() as usize, pages.len(), "{name}");

        let mut scope = laws::OriginalFixtureScope::admit();
        let mut app = open_registered_pdf(input.to_vec(), pages.len(), &mut scope).await;
        let mounted = app.snapshot().unwrap().clone();
        assert_eq!(mounted.pages, cold.pages, "{name}: pages");
        assert_eq!(mounted.fonts, cold.fonts, "{name}: fonts");
        assert_eq!(mounted.images, cold.images, "{name}: images");
        assert_eq!(mounted.admitted_stream_roles, cold.admitted_stream_roles, "{name}: roles");
        assert_eq!(mounted.objects, cold.objects, "{name}: objects");
        assert_eq!(mounted.trailer, cold.trailer, "{name}: trailer");
        assert_eq!(mounted.fonts.len() as u64, case["fontCount"].as_u64().unwrap(), "{name}");
        assert_eq!(mounted.images.len() as u64, case["imageCount"].as_u64().unwrap(), "{name}");
        assert_eq!(mounted.ext_g_states, cold.ext_g_states, "{name}: graphics states");
        assert_eq!(mounted.properties, cold.properties, "{name}: property lists");
        assert_eq!(mounted.ext_g_states.len() as u64, case["extGStateCount"].as_u64().unwrap(), "{name}");
        assert_eq!(mounted.properties.len() as u64, case["propertiesCount"].as_u64().unwrap(), "{name}");

        for (index, expected) in pages.iter().enumerate() {
            let (declared_fonts, declared_images) = &independent[index];
            assert_eq!(declared_fonts, &sorted(strings(&expected["fonts"])), "{name} page {}: lopdf font resources", index + 1);
            assert_eq!(declared_images, &sorted(strings(&expected["images"])), "{name} page {}: lopdf image resources", index + 1);
            let content = &mounted.pages[index].content;
            let used = sorted(content.iter().filter_map(|operator| if let PdfOp::SetFont { name, .. } = operator { Some(name.clone()) } else { None }).collect());
            assert_eq!(used, sorted(strings(&expected["usedFonts"])), "{name} page {}: fonts selected by the content", index + 1);
            for font in &used {
                assert!(declared_fonts.contains(font), "{name} page {}: {font} is declared by the page resources", index + 1);
                assert!(mounted.font(font).is_some(), "{name} page {}: {font} is a typed font", index + 1);
            }
            let painted = sorted(content.iter().filter_map(|operator| if let PdfOp::PaintXObject { name } = operator { Some(name.clone()) } else { None }).collect());
            assert_eq!(painted, sorted(strings(&expected["paintedImages"])), "{name} page {}: images painted by the content", index + 1);
            for image in &painted {
                assert!(declared_images.contains(image), "{name} page {}: {image} is declared by the page resources", index + 1);
                assert!(mounted.image(image).is_some(), "{name} page {}: {image} is a typed image", index + 1);
            }
            let shown: Vec<String> = content.iter().filter_map(|operator| match operator {
                PdfOp::ShowText { text: crate::standards::v1_7::subsets::base::schema::snapshot::PdfTextString::Text { text } } => Some(text.clone()),
                _ => None,
            }).collect();
            assert_eq!(shown, strings(&expected["text"]), "{name} page {}: decoded text", index + 1);
            if expected["independentText"].as_bool().unwrap() {
                let reference: Vec<String> = projected["pages"][index]["text"].as_array().unwrap().iter().map(|item| item.as_str().unwrap().to_owned()).collect();
                assert_eq!(shown, reference, "{name} page {}: lopdf text", index + 1);
                let operators = projected["pages"][index]["contentOperators"].as_array().unwrap();
                assert_eq!(operators.iter().filter(|operator| operator.as_str() == Some("Do")).count(), painted.len(), "{name} page {}: lopdf image paints", index + 1);
            }
        }
        laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("multi-resource editor closes");
    }
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_png_picker_publishes_an_atomic_revision_guarded_image_edit() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfFormXObject, PdfImage, PdfOp, PdfPage};
    use semio_framework_value::DslValue;
    use semio_plugin::{artifact_app_laws as laws, EditorApp, Effect, PluginApp};

    let fixture: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🖼️local-png/🔣️.json")).unwrap();
    let samples = |key: &str| fixture[key].as_array().unwrap().iter().map(|sample| u8::try_from(sample.as_u64().unwrap()).unwrap()).collect::<Vec<_>>();
    let matrix: [f64; 6] = std::array::from_fn(|index| fixture["existing"]["insertPlacement"][index].as_f64().unwrap());
    let source_id = fixture["existing"]["images"][0].as_str().unwrap();
    let form_id = fixture["existing"]["forms"][0].as_str().unwrap();
    let image_id = fixture["expected"]["image"].as_str().unwrap();
    let mask_id = fixture["expected"]["mask"].as_str().unwrap();

    let mut original = PdfSnapshot::default();
    let mut first = PdfPage::new(160.0, 120.0);
    first.content.push(PdfOp::PaintXObject { name: source_id.into() });
    original.pages = vec![first.clone(), first];
    original.images.push(PdfImage::gray8(source_id, 1, 1, vec![127]));
    original.forms.push(PdfFormXObject::new(form_id, [0.0, 0.0, 1.0, 1.0], Vec::new()));

    let mut scope = laws::OriginalFixtureScope::admit();
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let meta = laws::meta("local");
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else {
        panic!("the seed PDF can produce a document load");
    };
    semio_plugin::with_authoring_identity!(|identity| laws::load_document(
        &mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()
    ).await).unwrap();
    let loaded = app.snapshot().unwrap().clone();
    let request = DslValue::object([
        ("page".into(), DslValue::float(1.0)),
        ("x".into(), DslValue::float(matrix[4])),
        ("y".into(), DslValue::float(matrix[5])),
        ("width".into(), DslValue::float(matrix[0])),
        ("height".into(), DslValue::float(matrix[3])),
    ]);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "insert-image", Some(request), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    let picker = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    let [Effect::RequestFileOpen { accept, read_as, import_action, multiple, args: Some(args), max_file_bytes, .. }] = picker.effects.as_slice() else {
        panic!("the registered action emits exactly one local file picker");
    };
    assert_eq!(accept, "image/png,.png");
    assert_eq!(read_as.as_deref(), Some("dataUrl"));
    assert_eq!(import_action, "import-local-image");
    assert!(!*multiple);
    assert_eq!(max_file_bytes.map(|bytes| bytes as u64), fixture["bounds"]["maxFileBytes"].as_u64());
    assert_eq!(args.get("page").and_then(DslValue::as_u64), Some(0));
    assert_eq!(args.get("operator").and_then(DslValue::as_u64), Some(u64::from(u32::MAX)));
    let revision = args.get("revision").and_then(DslValue::as_str).expect("exact picker revision");
    assert_eq!(revision.len(), 64);
    assert!(revision.bytes().all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(app.snapshot().unwrap(), loaded);

    let mut callback = args.as_object().unwrap().to_vec();
    callback.extend([
        ("name".into(), DslValue::String("sample.png".into())),
        ("chunk".into(), DslValue::uint(0)),
        ("chunkCount".into(), DslValue::uint(1)),
        ("payload".into(), DslValue::String(format!("data:application/octet-stream;base64,{}", fixture["png"].as_str().unwrap()))),
    ]);
    let callback = DslValue::object(callback);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, import_action, Some(callback.clone()), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    assert_eq!(app.snapshot().unwrap(), loaded, "dispatch leaves the retained import private");
    let receipt = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap();
    assert_eq!(receipt.completions, 1);

    let published = app.snapshot().unwrap().clone();
    let width = u32::try_from(fixture["width"].as_u64().unwrap()).unwrap();
    let height = u32::try_from(fixture["height"].as_u64().unwrap()).unwrap();
    let mut image = PdfImage::rgb8(image_id, width, height, samples("rgb"));
    image.soft_mask = Some(mask_id.into());
    let mask = PdfImage::gray8(mask_id, width, height, samples("softMask"));
    assert_eq!(published.images.len(), loaded.images.len() + 2);
    assert_eq!(published.image(image_id), Some(&image));
    assert_eq!(published.image(mask_id), Some(&mask));
    assert_eq!(published.image(source_id), loaded.image(source_id));
    assert_eq!(published.forms, loaded.forms);
    assert_eq!(published.pages[1], loaded.pages[1]);
    assert!(published.pages[0].content.starts_with(&loaded.pages[0].content));
    assert!(matches!(
        &published.pages[0].content[loaded.pages[0].content.len()..],
        [PdfOp::Save, PdfOp::Transform { matrix: applied }, PdfOp::PaintXObject { name }, PdfOp::Restore]
            if applied == &matrix && name == image_id
    ));

    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, import_action, Some(callback), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap();
    let stale = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert!(format!("{:?}", stale.err().expect("the completed picker revision is stale")).contains("PDF changed while the image picker was open"));
    assert_eq!(app.snapshot().unwrap(), published);

    semio_plugin::with_authoring_identity!(|identity| laws::settle_history_verb(
        &mut app, "undo", meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert_eq!(app.snapshot().unwrap(), loaded);
    semio_plugin::with_authoring_identity!(|identity| laws::settle_history_verb(
        &mut app, "redo", meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    assert_eq!(app.snapshot().unwrap(), published);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("registered PNG fixture closes");
}

#[test]
fn controlled_pdf_terminal_publication_requires_both_owned_headers() {
    use crate::standards::v1_7::subsets::base::io::import::retained_graph::PreparedPdfFile;
    use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, RetirementPollExit};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_plugin::{NaturalFileDecodeOwner, NaturalFileDecodeStep, ToolExecutionContract};
    use std::mem::{size_of, size_of_val};

    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let contract = ToolExecutionContract::resumable(input.len(), 16_384, 1, 16 << 20, 7_500, 1, 1);
    let quote = size_of::<PreparedPdfFile>() + size_of::<PdfSnapshot>();
    #[cfg(all(target_arch = "wasm32", target_os = "wasi"))]
    assert_eq!(quote, 964);
    let stock = OriginalWorkFixture::stock();
    let run = |constrained_at: Option<usize>, paged: bool| {
        let mut scope = OriginalWorkFixture::new();
        let mut claimed = None;
        if paged {
            use crate::standards::v1_7::subsets::base::io::import::admit_pdf_paged_natural_decode;
            use semio_framework_value::paged::PagedBytes;
            use semio_plugin::NaturalFilePagedSource;
            let mut source = Some(NaturalFilePagedSource::retain(PagedBytes::try_from_slice(input).unwrap()));
            let smaller_wire_contract = ToolExecutionContract { max_raw_wire_bytes: 1, ..contract };
            let denied = RetainedCloneGrant { maximum_items: 0, ..stock };
            assert!(!scope.turn(denied, |cx| admit_pdf_paged_natural_decode(&mut source, smaller_wire_contract, &mut claimed, cx)).unwrap());
            assert!(source.as_ref().is_some_and(|owner| owner.original_is_untouched()));
            assert!(claimed.is_none());
            assert!(scope.turn(stock, |cx| admit_pdf_paged_natural_decode(&mut source, smaller_wire_contract, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the original paged PDF owner transferred");
        } else {
            let mut source = Some(input.to_vec());
            assert!(scope.turn(stock, |cx| Pdf17Editor::admit_natural_file_decode_owner(&mut source, contract, &mut claimed, cx)).unwrap());
            assert!(source.is_none(), "the exact original PDF bytes transferred");
        }
        let Some(NaturalFileDecodeOwner::Controlled(decoder)) = claimed else {
            panic!("one-unit file contract must use the real controlled PDF decoder");
        };
        let mut decoder = Some(decoder);
        let mut complete = None;
        for turn in 0..200_000 {
            let owner = decoder.as_mut().unwrap();
            if constrained_at == Some(turn) {
                let short = RetainedCloneGrant { maximum_copy_bytes: quote - 1, ..stock };
                for _ in 0..2 {
                    scope.turn(short, |cx| {
                        let before = (cx.retained_progress(), cx.original_work_units_spent());
                        assert!(matches!(owner.advance(cx).unwrap(), NaturalFileDecodeStep::Yield));
                        assert_eq!((cx.retained_progress(), cx.original_work_units_spent()), before);
                        Ok(())
                    }).unwrap();
                }
            }
            let grant = RetainedCloneGrant { maximum_copy_bytes: if constrained_at == Some(turn) { quote } else { stock.maximum_copy_bytes }, ..stock };
            let (step, copied, items) = scope.turn(grant, |cx| {
                let before = cx.retained_progress();
                let step = owner.advance(cx).unwrap();
                let after = cx.retained_progress();
                Ok((step, after.copied_bytes - before.copied_bytes, after.copied_items - before.copied_items))
            }).unwrap();
            if let NaturalFileDecodeStep::Complete { snapshot, progress } = step {
                assert_eq!((copied, items), (quote, 1));
                if let Some(expected) = constrained_at { assert_eq!(turn, expected); }
                complete = Some((snapshot, progress, turn));
                break;
            }
            assert_ne!(Some(turn), constrained_at, "the exact quote must publish after the two refusals");
        }
        let (snapshot, progress, terminal_turn) = complete.expect("the original controlled PDF reached publication");
        let owner = decoder.as_mut().unwrap();
        owner.begin_close();
        for _ in 0..200_000 {
            if owner.terminal_is_empty() { break; }
            scope.turn(stock, |cx| {
                let demand = owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
                semio_plugin::__job::close_original_effect_step(cx, demand, |effect| {
                    let result = owner.close_chained(effect).map(|step| RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into);
                    RetirementPollExit::settle(effect, result)
                })?;
                Ok(())
            }).unwrap();
        }
        assert!(owner.terminal_is_empty());
        let demand = RetirementDemand {
            release_bytes: size_of_val(&**owner),
            copy_bytes: size_of_val(&decoder),
            depth: 1,
            ..Default::default()
        };
        scope.turn(stock, |cx| {
            semio_plugin::__job::close_original_retained_step(cx, demand, |_| {
                drop(decoder.take());
                RetainedWorkOutcome {
                    work: 1,
                    result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress {
                        copied_items: 1,
                        copied_bytes: demand.copy_bytes,
                        released_bytes: demand.release_bytes,
                        ..Default::default()
                    })),
                }
            })?;
            Ok(())
        }).unwrap();
        assert!(decoder.is_none());
        (snapshot, progress, terminal_turn, scope)
    };

    let (reference, reference_progress, terminal_turn, mut reference_scope) = run(None, false);
    let (published, progress, _, mut scope) = run(Some(terminal_turn), false);
    let (paged, paged_progress, _, mut paged_scope) = run(None, true);
    assert_eq!(published, reference);
    assert_eq!(paged, reference);
    assert_eq!(progress, reference_progress);
    assert_eq!(paged_progress.consumed_input_bytes, input.len());
    paged_scope.retire_owned(paged, stock);
    paged_scope.finish();
    assert_eq!(published.pages.len(), 2);
    assert!(!published.objects.is_empty());
    assert!(!published.trailer.is_empty());
    scope.retire_owned(published, stock);
    scope.finish();
    reference_scope.retire_owned(reference, stock);
    reference_scope.finish();
}

/// 🏞️ Builds the neutral large one-image PDF and its exact decoded image samples from the schema-first law.
fn large_original_image_pdf(law: &serde_json::Value) -> (Vec<u8>, Vec<u8>) {
    let side = law["imageSide"].as_u64().unwrap() as usize;
    let media = law["mediaSide"].as_u64().unwrap();
    let gray = |index: usize| law["grayscale"][index].as_u64().unwrap() as u8;
    let half = side / 2;
    let rows = |left: u8, right: u8| [vec![left; half], vec![right; half]].concat();
    let (top, bottom) = (rows(gray(0), gray(1)), rows(gray(2), gray(3)));
    let image: Vec<u8> = (0..half).flat_map(|_| top.iter()).chain((0..half).flat_map(|_| bottom.iter())).copied().collect();
    let framed = |dictionary: &str, data: &[u8]| [format!("<< {dictionary} /Length {} >>\nstream\n", data.len()).as_bytes(), data, b"\nendstream"].concat();
    let objects = [
        b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
        b"<< /Type /Pages /Count 1 /Kids [3 0 R] >>".to_vec(),
        format!("<< /Type /Page /Parent 2 0 R /MediaBox [0 0 {media} {media}] /Contents 4 0 R /Resources << /XObject << /FixtureImage 5 0 R >> >> >>").into_bytes(),
        framed("", format!("q {media} 0 0 {media} 0 0 cm /FixtureImage Do Q\n").as_bytes()),
        framed(&format!("/Type /XObject /Subtype /Image /Width {side} /Height {side} /ColorSpace /DeviceGray /BitsPerComponent 8"), &image),
    ];
    let mut out = b"%PDF-1.7\n%\xe2\xe3\xcf\xd3\n".to_vec();
    let mut offsets = Vec::new();
    for (index, object) in objects.iter().enumerate() {
        offsets.push(out.len());
        out.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
        out.extend_from_slice(object);
        out.extend_from_slice(b"\nendobj\n");
    }
    let xref = out.len();
    out.extend_from_slice(format!("xref\n0 {}\n0000000000 65535 f \n", offsets.len() + 1).as_bytes());
    for offset in &offsets { out.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes()); }
    out.extend_from_slice(format!("trailer\n<< /Size {} /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n", offsets.len() + 1).as_bytes());
    (out, image)
}

/// 🏞️ Opens one PDF through the production paged natural-file decoder under a 16 MiB capacity grant and retires it on the bounded close path.
fn paged_pdf_snapshot(input: &[u8]) -> (PdfSnapshot, semio_plugin::artifact_app_laws::OriginalWorkFixture, usize) {
    paged_pdf_snapshot_under(input, semio_plugin::app::NATURAL_SOURCE_TOOL_CONTRACT, LARGE_IMAGE_DECODE_TURN_LIMIT)
}

/// 📥️ Admits the production paged decoder of `input` on a hosted fixture: every turn is issued the host's fresh fuel, so a large document is bounded by per-turn fuel and not by a one-time stock.
fn admit_paged_decoder(input: &[u8], contract: semio_plugin::ToolExecutionContract) -> (semio_plugin::artifact_app_laws::OriginalWorkFixture, Box<dyn semio_plugin::NaturalFileDecodeCursor<PdfSnapshot>>, semio_framework_value::RetainedCloneGrant) {
    use crate::standards::v1_7::subsets::base::io::import::admit_pdf_paged_natural_decode;
    use semio_framework_value::{paged::PagedBytes, RetainedCloneGrant};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_plugin::{NaturalFileDecodeOwner, NaturalFilePagedSource};

    let granted = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.preparation;
    let mut scope = OriginalWorkFixture::hosted(semio_plugin::artifact_app_laws::HostLifetimeStock::FUEL);
    let mut claimed = None;
    let mut source = Some(NaturalFilePagedSource::retain(PagedBytes::try_from_slice(input).unwrap()));
    assert!(scope.turn(granted, |cx| admit_pdf_paged_natural_decode(&mut source, contract, &mut claimed, cx)).unwrap());
    let Some(NaturalFileDecodeOwner::Controlled(decoder)) = claimed else { panic!("the large PDF must use the controlled decoder") };
    (scope, decoder, granted)
}

/// ♻️ Retires a paged decoder, finished or cancelled, through the bounded close path of its caller and returns how many host turns the close took.
fn close_paged_decoder(scope: &mut semio_plugin::artifact_app_laws::OriginalWorkFixture, decoder: Box<dyn semio_plugin::NaturalFileDecodeCursor<PdfSnapshot>>, granted: semio_framework_value::RetainedCloneGrant) -> usize {
    use semio_framework_value::{RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, RetirementPollExit};
    use std::mem::size_of_val;

    let mut decoder = Some(decoder);
    let owner = decoder.as_mut().unwrap();
    owner.begin_close();
    let mut turns = 0;
    for _ in 0..4_000_000 {
        if owner.terminal_is_empty() { break; }
        turns += 1;
        scope.turn(granted, |cx| {
            let demand = owner.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;
            semio_plugin::__job::close_original_effect_step(cx, demand, |effect| {
                let result = owner.close_step(effect).map(|step| RetainedCloneStep::Progress(step.progress().unwrap_or_default())).map_err(Into::into);
                RetirementPollExit::settle(effect, result)
            })?;
            Ok(())
        }).unwrap();
    }
    assert!(owner.terminal_is_empty());
    let demand = RetirementDemand { release_bytes: size_of_val(&**owner), copy_bytes: size_of_val(&decoder), depth: 1, ..Default::default() };
    scope.turn(granted, |cx| {
        semio_plugin::__job::close_original_retained_step(cx, demand, |_| {
            drop(decoder.take());
            RetainedWorkOutcome { work: 1, result: Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, released_bytes: demand.release_bytes, ..Default::default() })) }
        })?;
        Ok(())
    }).unwrap();
    turns
}

/// 🔁️ Drives the production paged decoder under one execution contract, failing when it needs more than `turn_limit` continuations, and returns how many it used.
fn paged_pdf_snapshot_under(input: &[u8], contract: semio_plugin::ToolExecutionContract, turn_limit: usize) -> (PdfSnapshot, semio_plugin::artifact_app_laws::OriginalWorkFixture, usize) {
    use semio_plugin::NaturalFileDecodeStep;

    let (mut scope, mut decoder, granted) = admit_paged_decoder(input, contract);
    let mut complete = None;
    for turn in 1..=turn_limit {
        if let NaturalFileDecodeStep::Complete { snapshot, progress } = scope.turn(granted, |cx| Ok(decoder.advance(cx).unwrap())).unwrap() {
            complete = Some((snapshot, progress, turn));
            break;
        }
    }
    let (snapshot, progress, turns) = complete.unwrap_or_else(|| panic!("the paged PDF decoder did not publish within {turn_limit} continuations"));
    assert_eq!(progress.consumed_input_bytes, input.len());
    close_paged_decoder(&mut scope, decoder, granted);
    (snapshot, scope, turns)
}

/// 📚️ A 963-byte file fits in one 4096-unit browser step, so a decoder that stops unpaid `More` replies finishes in at most 128 host turns.
#[test]
fn paged_natural_decode_of_the_browser_contract_terminates_with_every_page() {
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;
    use semio_plugin::app::NATURAL_SOURCE_TOOL_CONTRACT;

    let input = fixture_bytes!("../../../../../../🧪️tests/🧫️fixtures/✏️edit-existing-pdf/2️⃣two-pages.pdf");
    let (opened, mut scope, turns) = paged_pdf_snapshot_under(input, NATURAL_SOURCE_TOOL_CONTRACT, 1_000);
    assert!(turns <= 128, "browser contract used {turns} continuations");
    let projected: serde_json::Value = serde_json::from_str(&semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::project_pdf_1_7(input).expect("lopdf reads the fixture").to_string()).unwrap();
    assert_eq!(opened.pages.len(), 2);
    assert_eq!(opened.pages.len() as u64, projected["pageCount"].as_u64().unwrap());
    assert_eq!(opened, <Pdf17Editor as ArtifactEditor>::decode_natural_file(input).unwrap());
    scope.retire_owned(opened, OriginalWorkFixture::stock());
    scope.finish();
}

#[test]
fn paged_natural_decode_round_trips_a_large_original_image_exactly() {
    use crate::standards::v1_7::subsets::base::{io::{export::encode_pdf, import::decode_pdf}, schema::snapshot::PdfObject};
    use semio_plugin::artifact_app_laws::OriginalWorkFixture;

    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🏞️large-original-image/🔣️.json")).unwrap();
    let (input, image) = large_original_image_pdf(&law);
    let fnv = format!("{:016x}", input.iter().fold(0xCBF2_9CE4_8422_2325u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0000_0100_0000_01B3)));
    assert_eq!((input.len() as u64, fnv.as_str()), (law["sourceBytes"].as_u64().unwrap(), law["sourceFnv1a64"].as_str().unwrap()));
    assert_eq!(image.len() as u64, law["imageBytes"].as_u64().unwrap());
    let largest = |snapshot: &PdfSnapshot| snapshot.objects.iter().filter_map(|object| match &object.value {
        PdfObject::Stream { data, .. } => Some(data.to_vec_owner()),
        _ => None,
    }).max_by_key(Vec::len).unwrap();

    let (opened, scope, open_turns) = paged_pdf_snapshot(&input);
    assert!(open_turns <= 4_700, "the 9 MiB image opened in {open_turns} host turns under the browser grants");
    assert_eq!(opened, decode_pdf(&input).unwrap());
    assert_eq!(opened.pages.len() as u64, law["pageCount"].as_u64().unwrap());
    assert!(largest(&opened) == image, "the paged open retains every decoded image sample");
    let saved = encode_pdf(&opened).unwrap();
    if let Some(path) = std::env::var_os("SEMIO_LARGE_ORIGINAL_IMAGE_SAVED") { std::fs::write(path, &saved).unwrap(); }
    let (reopened, reopened_scope, reopen_turns) = paged_pdf_snapshot(&saved);
    assert!(reopen_turns <= 4_700, "the saved 9 MiB image reopened in {reopen_turns} host turns");
    assert!(largest(&reopened) == image, "the saved PDF reopens with exactly the original image samples");
    assert_eq!(reopened.pages, opened.pages);
    let stock = semio_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.close;
    let (mut scope, mut reopened_scope) = (scope, reopened_scope);
    scope.retire_owned(opened, stock);
    scope.finish();
    reopened_scope.retire_owned(reopened, stock);
    reopened_scope.finish();
}

/// 🛑️ A large paged open that is cancelled mid-operation stops consuming work at its next step, reports no further progress, and retires every byte it owns through the bounded close path.
#[test]
fn paged_natural_decode_of_a_large_original_image_honours_cancellation_mid_operation() {
    use semio_plugin::NaturalFileDecodeStep;

    let law: serde_json::Value = serde_json::from_str(fixture_text!("../../../../../../🧪️tests/🧫️fixtures/🏞️large-original-image/🔣️.json")).unwrap();
    let (input, _) = large_original_image_pdf(&law);
    let (mut scope, mut decoder, granted) = admit_paged_decoder(&input, semio_plugin::app::NATURAL_SOURCE_TOOL_CONTRACT);
    let mut reported = 0;
    for _ in 0..16 {
        match scope.turn(granted, |cx| Ok(decoder.advance(cx).unwrap())).unwrap() {
            NaturalFileDecodeStep::Progress(progress) => reported = progress.completed_work_units,
            NaturalFileDecodeStep::Yield => {}
            NaturalFileDecodeStep::Complete { .. } => panic!("a 9 MiB image cannot open within 16 host turns"),
        }
    }
    assert!(reported > 0 && reported < input.len() as u64, "the open is mid-operation, {reported} of {} units", input.len());
    semio_framework_async::poll::resolve_ready(scope.cancel().cancel());
    let after = scope.turn(granted, |cx| Ok(decoder.advance(cx).unwrap())).unwrap();
    assert!(matches!(after, NaturalFileDecodeStep::Yield), "a cancelled decoder consumes no further work");
    let turns = close_paged_decoder(&mut scope, decoder, granted);
    assert!(turns < LARGE_DOCUMENT_TURN_LIMIT * 8, "the cancelled open retired in {turns} host turns");
    scope.finish();
}

#[semio_framework_async_macros::async_test]
async fn registered_pdf_page_and_authoring_actions_undo_and_redo_step_by_step() {
    use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfAcroForm, PdfAnnotation, PdfAnnotationKind, PdfFont, PdfFormField, PdfFormFieldKind, PdfPage};
    use semio_framework_value::DslValue;
    use semio_plugin::{artifact_app_laws, EditorApp, PluginApp};

    let mut original = PdfSnapshot::default();
    let mut first = PdfPage::new(200.0, 300.0);
    first.annotations.push(PdfAnnotation::new([10.0, 250.0, 110.0, 270.0], PdfAnnotationKind::Widget { field: Some("name".into()), highlight: None, characteristics: Vec::new(), action: None, additional_actions: Vec::new() }));
    original.pages = vec![first, PdfPage::new(100.0, 100.0)];
    original.fonts.push(PdfFont::standard("Helv", "Helvetica"));
    original.acro_form = Some(PdfAcroForm {
        fields: vec![PdfFormField { name: "name".into(), kind: PdfFormFieldKind::Text { value: None, default_value: None, max_length: None, rich_value: None }, flags: 0, alternate_name: None, mapping_name: None, default_appearance: Some("/Helv 12 Tf 0 g".into()), quadding: None, widgets: vec![[0, 0]], children: Vec::new(), additional_actions: Vec::new(), extra: Vec::new() }],
        ..Default::default()
    });
    let mut scope = semio_plugin::artifact_app_laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } }, semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity).await);
    let semio_plugin::Effect::LoadDocument { pack, spr } = semio_s_artifact_stdio_contract::load_example_effect(&original, STDIO_PDF17_DOCUMENT_SCHEMA) else { panic!("PDF fixture produces a document load") };
    semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::load_document(&mut app, &store::os_store::serialization::ArtifactPackFiles { pack, spr, ops: String::new() }, &mut identity, scope.cx()).await).unwrap();
    let meta = artifact_app_laws::meta("local");
    let number = DslValue::float;
    let text = |value: &str| DslValue::String(value.into());
    macro_rules! step {
        ($action:expr, $entries:expr) => {{
            let arguments = DslValue::object($entries.into_iter().map(|(key, value): (&str, DslValue)| (key.to_string(), value)));
            semio_plugin::with_authoring_identity!(|identity| semio_plugin::artifact_app_laws::dispatch_fixture_action(&mut app, $action, Some(arguments), &meta, &mut identity, scope.cx(), semio_s_artifact_stdio_contract::testing::mounted_policy().close).await).unwrap();
            semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_registered_typed_operation(&mut app, meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await).unwrap();
            app.snapshot().unwrap().clone()
        }};
    }
    let loaded = app.snapshot().unwrap().clone();
    let mut states = vec![loaded];
    states.push(step!("insert-page", [("page", number(2.0)), ("width", number(50.0)), ("height", number(60.0))]));
    assert_eq!(states[1].pages.len(), 3);
    states.push(step!("move-page", [("page", number(1.0)), ("x", number(3.0))]));
    assert_eq!(states[2].acro_form.as_ref().unwrap().fields[0].widgets, vec![[2, 0]]);
    states.push(step!("create-annotation", [("page", number(3.0)), ("object", text("highlight")), ("x", number(10.0)), ("y", number(10.0)), ("width", number(80.0)), ("height", number(12.0))]));
    assert_eq!(states[3].pages[2].annotations.len(), 2);
    assert!(states[3].pages[2].annotations[1].appearance.is_some() && !states[3].forms.is_empty());
    states.push(step!("fill-form-field", [("object", text("name")), ("text", text("Ada"))]));
    assert!(states[4].pages[2].annotations[0].appearance.is_some());
    states.push(step!("replace-page", [("page", number(1.0)), ("width", number(33.0)), ("height", number(44.0))]));
    assert_eq!(states[5].pages[0].width(), 33.0);
    states.push(step!("remove-page", [("page", number(2.0))]));
    assert_eq!(states[6].pages.len(), 2);
    for index in (0..states.len() - 1).rev() {
        semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "undo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
        assert_eq!(app.snapshot().unwrap(), states[index], "undo to state {index}");
    }
    for index in 1..states.len() {
        semio_plugin::with_authoring_identity!(|identity| artifact_app_laws::settle_history_verb(&mut app, "redo", meta.instance_id, semio_s_artifact_stdio_contract::testing::mounted_policy(), &mut identity, scope.cx()).await);
        assert_eq!(app.snapshot().unwrap(), states[index], "redo to state {index}");
    }
    artifact_app_laws::close_registered_fixture_app(&mut app, semio_s_artifact_stdio_contract::testing::mounted_policy(), scope.cx()).expect("original caller paid app close");
}

/// 🪟️ The shell notes every window activation as a reserved `noteShellCommand` job; a typed edit after it still settles (browser: the text never appeared).
#[semio_framework_async_macros::async_test]
async fn typed_edit_after_a_reserved_shell_note_settles_through_the_registered_typed_operation() {
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let text = |value: &str| semio_framework_value::DslValue::String(value.into());
    let number = |value: f64| semio_framework_value::DslValue::float(value);
    semio_plugin::with_authoring_identity!(|identity| laws::settle_reserved_action_as(
        &mut app, "noteShellCommand", Some(semio_framework_value::DslValue::object([("commandId".into(), text("command.activateWindow"))])), &meta, meta.instance_id, policy, &mut identity, scope.cx()
    ).await);
    let args = semio_framework_value::DslValue::object([("page".into(), number(1.0)), ("x".into(), number(72.0)), ("y".into(), number(700.0)), ("height".into(), number(12.0)), ("text".into(), text("Hello"))]);
    semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "insert-text", Some(args), &meta, &mut identity, scope.cx(), policy.close
    ).await).unwrap_or_else(|fault| panic!("insert-text dispatch refused: {fault:?}"));
    semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
        &mut app, meta.instance_id, policy, &mut identity, scope.cx()
    ).await).unwrap_or_else(|fault| panic!("insert-text settlement refused: {fault:?}"));
    let snapshot = app.snapshot().expect("the blank PDF is ready");
    assert!(crate::editor::page::objects(&snapshot).iter().any(|object| object.text == "Hello"));
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("blank editor closes");
}

/// 🧯️ The browser's first edits of a blank PDF 1.7 document settle through the registered typed operation path.
#[semio_framework_async_macros::async_test]
async fn blank_document_first_edits_settle_through_the_registered_typed_operation() {
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let text = |value: &str| semio_framework_value::DslValue::String(value.into());
    let number = |value: f64| semio_framework_value::DslValue::float(value);
    let edits = [
        ("set-info-field", semio_framework_value::DslValue::object([("object".into(), text("title")), ("field".into(), text("text")), ("value".into(), text("Quarterly report"))])),
        ("insert-text", semio_framework_value::DslValue::object([("page".into(), number(1.0)), ("x".into(), number(72.0)), ("y".into(), number(700.0)), ("height".into(), number(12.0)), ("text".into(), text("Hello"))])),
    ];
    for (action, args) in edits {
        semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
            &mut app, action, Some(args.clone()), &meta, &mut identity, scope.cx(), policy.close
        ).await).unwrap_or_else(|fault| panic!("{action} dispatch refused: {fault:?}"));
        semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
            &mut app, meta.instance_id, policy, &mut identity, scope.cx()
        ).await).unwrap_or_else(|fault| panic!("{action} settlement refused: {fault:?}"));
    }
    let snapshot = app.snapshot().expect("the blank PDF is ready");
    assert_eq!(snapshot.info.title.as_deref(), Some("Quarterly report"));
    assert!(crate::editor::page::objects(&snapshot).iter().any(|object| object.text == "Hello"));
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("blank editor closes");
}

/// 🚫️ A text edit naming a page the document lacks is refused with the localized range fault through the registered typed operation: the refusal is a fault the shell can tell, and the document stays as it was.
#[semio_framework_async_macros::async_test]
async fn an_out_of_range_page_edit_is_refused_with_the_range_fault_through_the_registered_typed_operation() {
    use semio_plugin::{artifact_app_laws as laws, EditorApp};
    let meta = laws::meta("local");
    let policy = semio_s_artifact_stdio_contract::testing::mounted_policy();
    let mut scope = laws::OriginalFixtureScope::admit();
    let mut app = semio_plugin::with_authoring_identity!(|identity| laws::new_registered_app::<EditorApp<Pdf17Editor>, _>(
        async { semio_plugin::App { definition: create_pdf17_editor(), examples: Vec::new() } },
        semio_os_kernel::ActorId(semio_os_kernel::LOCAL_ACTOR_ID.into()), policy, &mut identity
    ).await);
    let before = app.snapshot().expect("the blank PDF is ready").clone();
    let number = |value: f64| semio_framework_value::DslValue::float(value);
    let args = semio_framework_value::DslValue::object([("page".into(), number(99.0)), ("x".into(), number(72.0)), ("y".into(), number(700.0)), ("height".into(), number(12.0)), ("text".into(), semio_framework_value::DslValue::String("Nowhere".into()))]);
    let dispatched = semio_plugin::with_authoring_identity!(|identity| laws::dispatch_fixture_action(
        &mut app, "insert-text", Some(args), &meta, &mut identity, scope.cx(), policy.close
    ).await);
    let refusal = match dispatched {
        Err(fault) => format!("dispatch refused: {fault:?}"),
        Ok(_) => {
            let settled = semio_plugin::with_authoring_identity!(|identity| laws::settle_registered_typed_operation(
                &mut app, meta.instance_id, policy, &mut identity, scope.cx()
            ).await);
            settled.map(|_| "the refused edit settled without a fault".to_string()).unwrap_or_else(|fault| format!("settlement refused: {fault:?}"))
        }
    };
    assert!(refusal.contains("stdio.pdf.page-number-out-of-range"), "the range fault is told, got {refusal}");
    assert_eq!(app.snapshot().expect("the blank PDF stays ready"), before);
    laws::close_registered_fixture_app(&mut app, policy, scope.cx()).expect("blank editor closes");
}
