//! 🪪️ A real documentless app answers the binary identity command without a document revision read.
use semio_framework_os_kernel as protocol;
use semio_framework_plugin_reactor_app::{PluginApp, PluginAppDelegate};
use semio_framework_plugin_instance_app::VcsArtifactApp;
use semio_framework_plugin_plugin_program::AppInstance;
use semio_framework_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::{contract_app_raw, TestApp};
use semio_framework_plugin_testing_laws::artifact_app_laws;
use std::sync::atomic::{AtomicUsize, Ordering};

struct DocumentlessApp {
    inner: VcsArtifactApp<TestApp>,
    revision_reads: AtomicUsize,
}

impl PluginAppDelegate for DocumentlessApp {
    type Target = VcsArtifactApp<TestApp>;
    fn delegate_ref(&self) -> &Self::Target { &self.inner }
    fn delegate_mut(&mut self) -> &mut Self::Target { &mut self.inner }
    fn document_identity(&self) -> Option<String> { None }
    fn document_base_revision(&self) -> Result<u64, semio_framework::Fault> {
        self.revision_reads.fetch_add(1, Ordering::Relaxed);
        Err(semio_framework::Fault::from("a documentless app has no document revision"))
    }
}

#[semio_framework_async_macros::async_test]
async fn documentless_app_identity_abi_returns_authoritative_null_without_revision_read() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🪪️document-identity-wire/🔣️.json")).unwrap();
    let row = law["cases"].as_array().unwrap().iter().find(|row| row["name"] == "unavailable").unwrap();
    let instance_id = row["identity"]["appInstanceId"].as_u64().unwrap() as u32;
    let seq = row["seq"].as_str().unwrap().parse::<u64>().unwrap();
    let policy = artifact_app_laws::fixture_mounted_policy();
    let mut identity = artifact_app_laws::fixture_identity();
    let mut caller = semio_framework_plugin_testing_caller::OriginalFixtureCaller::admit().unwrap();
    let mut context = caller.context();
    let mut inner = contract_app_raw(policy, &mut identity).await;
    inner.bind_instance_id(instance_id).await;
    let app = DocumentlessApp { inner, revision_reads: AtomicUsize::new(0) };
    let runtime = semio_framework_plugin_reactor_lifecycle::life_instance_registry::PluginRuntime::new(policy).unwrap();
    assert!(runtime.instances.borrow_mut().can_insert(instance_id));
    runtime.instances.borrow_mut().insert_admitted(instance_id, std::sync::Arc::new(
        semio_framework_plugin_reactor_lifecycle::life_app_cell::RuntimeAppCell::new(
            AppInstance { id: instance_id, app, surface_contexts: Default::default() }, policy)));
    let encoded = protocol::encode_app_command(&protocol::AppCommand::ReadDocumentIdentity { seq }).await.unwrap();
    let expected_command: Vec<u8> = serde_json::from_value(row["commandBytes"].clone()).unwrap();
    assert_eq!(encoded.front_page().unwrap().as_slice(), expected_command);
    let mut command = Some((seq, semio_framework_plugin_runtime_ops::ops_ingress::PluginCommandIngress::Encoded(encoded)));
    let mut frames = Vec::new();
    for turn in 0..128 {
        let mut output = semio_framework_plugin_runtime_exchange::exch_plugin_exchange::plugin_exchange(
            &runtime, instance_id, command.take(), &mut identity, &mut context).await.unwrap();
        frames.append(&mut output.frames);
        command = output.retry_command.take();
        if command.is_none() { break; }
        assert!(turn < 127, "identity command must finish its bounded ingress");
    }
    let expected_frame: Vec<u8> = serde_json::from_value(row["frameBytes"].clone()).unwrap();
    assert!(frames.iter().any(|frame| frame == &expected_frame), "actual ABI must return the neutral documentless identity frame");
    let frame = protocol::decode_app_frame(&expected_frame).await.unwrap();
    let protocol::AppFrame::DocumentIdentity { in_reply_to, identity } = frame else { panic!("identity response"); };
    assert_eq!(in_reply_to, seq);
    assert_eq!(serde_json::to_value(identity).unwrap(), row["identity"]);
    let cell = runtime.instances.borrow_mut().take(instance_id).unwrap();
    let mut instance = cell.host_instance().unwrap();
    assert_eq!(instance.app.revision_reads.load(Ordering::Relaxed), 0);
    artifact_app_laws::close_registered_fixture_app(&mut instance.app, policy, &mut context).unwrap();
    drop(instance);
    drop(cell);
    drop(runtime);
    let fuel = context.fuel_remaining();
    drop(context);
    caller.retire(fuel).unwrap();
}
