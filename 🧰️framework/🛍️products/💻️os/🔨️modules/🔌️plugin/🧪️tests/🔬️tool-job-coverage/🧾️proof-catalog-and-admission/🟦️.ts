/** 🧾️ The interactive job evaluation fixtures, the proof catalog evaluation checks and the admission order and bounded preparation checks. */
import { toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobScanThenMonolithRows, TOOL_JOB_RESERVED_IDS, toolJobProofCatalogFailures, toolJobFrameworkReservedRoutesExact, toolJobDecodeAfterAdmission, toolJobLimitsMatch, toolJobExternalCancellationOwned, toolJobRuntimeProofQualified, toolJobQualifiedProofBeforeDecode, toolJobLiveInstanceIsolated, toolJobTypedDispatchExact, toolJobWireDispatchExact, toolJobImportPreparationBounded, toolJobFullOperationBounded, toolJobPluginInstanceSource, toolJobSourceMutation } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/⚖️laws/🟦️.ts";
import { WORKSPACE_ROOT } from "../../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";
import { evaluateToolJobSources, exactBoundedFirstStepProof } from "../🧮️source-evaluation/🟦️.ts";

/** 🔎 Runs the fixture sources that must leave at least one unmigrated tool job row and returns them for the final count. */
export function interactiveJobEvaluationFixtures() {
  const fixtures: readonly [string, Record<string, string>][] = [
    ["old-775-default", { "plugin.rs": 'app_commands! { "one" => one::Payload, "two" => two::Payload }' }],
    ["missing-alias-fallback", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "exact" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "exact" => exact::Payload }\n.action_interactive_job("exact", InteractiveJobClassification::Migrated)\ndispatch("alias");' }],
    ["nonmacro-puzzle", { "puzzle.rs": '.mutation("scramble", label, kind)' }],
    ["unbounded-handler", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "export" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "export" => export::Payload }\n.action_interactive_job("export", InteractiveJobClassification::Migrated)\nfn handle(items: &[u8]) { for item in items { work(item); } }' }],
    ["default-migrated-builder", { "manifest.rs": 'fn bounded_catalog() { definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated; }', "plugin.rs": 'app_commands! { "solve" => solve::Payload }' }],
  ];
  for (const [name, sources] of fixtures) if (evaluateToolJobSources(sources).length === 0) throw new Error(`[verify interactivity tool-jobs] self-test ${name} was falsely accepted.`);
  return { fixtures };
}

/** 📚 Checks scan then monolith rows, duplicate owners, valid, typed and shared catalogs, executable owners, qualified schemas and forged proof rows. */
export function proofCatalogEvaluationChecks() {
  const scanThenMonolithSource = `${exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "scan").replace("bounded_first_step(64, 1, 1, 64, 100)", "resumable(64, 1, 1, 64, 100, 1, 1)")}
fn scan_retained_reduce(command: &Command) { command.dispatch(); }
impl ArtifactCommandWork<EditorApp<Ownerownerrs>> for ScanWork {
  fn step(&mut self, command: &Command) { ArtifactCommandWorkStep::Progress { stage: "fixture-command-scan", preview: b"{}" }; scan_retained_reduce(command); }
}`;
  const scanThenMonolithFiles = new Map([["owner.rs", scanThenMonolithSource]]);
  const scanThenMonolithProofs = toolJobProofs(scanThenMonolithFiles);
  if (toolJobScanThenMonolithRows(scanThenMonolithFiles, scanThenMonolithProofs).length !== 1)
    throw new Error("[verify interactivity tool-jobs] self-test scan-then-monolithic reducer was falsely accepted.");
  if (toolJobScanThenMonolithRows(new Map([["owner.rs", scanThenMonolithSource.replace("scan_retained_reduce(command);", "workspace.step_one(command);")]]), scanThenMonolithProofs).length !== 0)
    throw new Error("[verify interactivity tool-jobs] self-test segmented workspace reducer was falsely rejected.");
  const restoredFinalDispatch = scanThenMonolithSource.replace("scan_retained_reduce(command);", "workspace.step_one(command);").replace("workspace.step_one(command);", "command.dispatch();");
  if (toolJobScanThenMonolithRows(new Map([["owner.rs", restoredFinalDispatch]]), scanThenMonolithProofs).length !== 1)
    throw new Error("[verify interactivity tool-jobs] self-test restored final one-shot dispatch was falsely accepted.");
  const duplicateOwner = {
    "owner-a.rs": `app_commands! { "same" => same::Payload }\n.action_interactive_job("same", InteractiveJobClassification::Migrated)\n${exactBoundedFirstStepProof("owner-a.rs", "owner-a@1/*#editor", "same")}`,
    "owner-b.rs": 'app_commands! { "same" => same::Payload }\n.action_interactive_job("same", InteractiveJobClassification::Migrated)',
  };
  const duplicateRemaining = evaluateToolJobSources(duplicateOwner);
  if (duplicateRemaining.length !== 2) throw new Error("[verify interactivity tool-jobs] self-test declaration-without-proof was falsely accepted.");
  const validCatalog = { "owner.rs": `app_commands! { "bounded" => bounded::Payload }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "bounded")}` };
  if (evaluateToolJobSources(validCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test proof-only-route was falsely accepted as executable.");
  const typedVariantCatalog = { "owner.rs": `const BOUNDED_ROUTE: &str = "bounded";\npuzzle3d_command_variants! { One = BOUNDED_ROUTE }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "bounded")}` };
  if (evaluateToolJobSources(typedVariantCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test typed-command-variant-proof-only route was falsely accepted as executable.");
  const unresolvedTypedVariant = new Map([["owner.rs", "puzzle3d_command_variants! { One = COPIED_UNRESOLVED_ROUTE }"]]);
  if (toolJobStaticRows(unresolvedTypedVariant).length !== 0) throw new Error("[verify interactivity tool-jobs] self-test unresolved typed-command variant was falsely accepted.");
  const sharedProof = exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "bounded").replace(
    /tools:\s*\{\s*"bounded"\s*=>\s*(semio_framework::ToolExecutionContract::bounded_first_step\([^)]*\)),\s*\}/,
    'contract: $1, tools: ["bounded"]',
  );
  const sharedCatalog = { "owner.rs": `app_commands! { "bounded" => bounded::Payload }\n.action_interactive_job("bounded", InteractiveJobClassification::Migrated)\n${sharedProof}` };
  if (evaluateToolJobSources(sharedCatalog).length !== 1) throw new Error("[verify interactivity tool-jobs] self-test shared-contract-proof-only route was falsely accepted as executable.");
  const executableOwner = `
${validCatalog["owner.rs"].replace("impl ArtifactEditor for Ownerownerrs { const DOCUMENT_SCHEMA: &'static str = \"fixture.document\"; }", "").replace('factory: "BoundedFirstStepCommandJobFactory",', 'factory: "AppCommandJobFactory", factory_type: AppCommandJobFactory,')}
const APP_COMMAND_TOOL_IDS: &'static [&'static str] = &["bounded"];
struct AppCommandJobFactory;
impl ToolJobFactory for AppCommandJobFactory {
  type Payload = Payload;
  type Job = Job;
  fn execution_contract(&self) -> ToolExecutionContract { contract() }
  fn create_job(&mut self, operation: Operation, payload: Self::Payload) -> Result<Self::Job, ToolJobFactoryError> { job(operation, payload) }
}
impl ArtifactOwnedToolJobFactory for AppCommandJobFactory {
  type Owner = semio_framework_plugin::EditorApp<Ownerownerrs>;
  const TOOL_IDS: &'static [&'static str] = APP_COMMAND_TOOL_IDS;
}
impl ArtifactEditor for Ownerownerrs {
  const DOCUMENT_SCHEMA: &'static str = "fixture.document";
  fn register_tool_job_factories(registry: &mut Registry) { registry.register(AppCommandJobFactory::new("owner")); }
  fn build_tool_job(request: Request) -> Result<Option<ToolOperationSpec>, Fault> { build(request) }
}`;
  if (evaluateToolJobSources({ "owner.rs": executableOwner }).length !== 0) throw new Error("[verify interactivity tool-jobs] self-test exact app-owned executable route was falsely rejected.");
  const directExecutableOwner = executableOwner
    .replaceAll("Ownerownerrs", "DirectOwner")
    .replaceAll("impl ArtifactEditor for DirectOwner", "impl ArtifactApp for DirectOwner")
    .replaceAll("semio_framework_plugin::EditorApp<DirectOwner>", "DirectOwner");
  if (evaluateToolJobSources({ "owner.rs": directExecutableOwner }).length !== 0)
    throw new Error("[verify interactivity tool-jobs] self-test direct ArtifactApp executable route was falsely rejected.");
  if (evaluateToolJobSources({ "owner.rs": directExecutableOwner.replace("type Owner = DirectOwner;", "type Owner = ForgedDirectOwner;") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test forged direct ArtifactApp owner was falsely accepted.");
  const qualifiedSchemaOwner = `const FIXTURE_DOCUMENT_SCHEMA: &str = "fixture.document";\n${executableOwner.replace(
    'const DOCUMENT_SCHEMA: &\'static str = "fixture.document";',
    "const DOCUMENT_SCHEMA: &'static str = crate::schema::FIXTURE_DOCUMENT_SCHEMA;",
  )}`;
  const qualifiedSchemaFiles = new Map([["owner.rs", qualifiedSchemaOwner]]);
  const qualifiedSchemaFailures = toolJobProofCatalogFailures(
    qualifiedSchemaFiles,
    toolJobStaticRows(qualifiedSchemaFiles),
    toolJobDispositions(qualifiedSchemaFiles),
    toolJobProofs(qualifiedSchemaFiles),
  );
  if (qualifiedSchemaFailures.some((failure) => failure.includes("forged bounded reducer document schema")))
    throw new Error("[verify interactivity tool-jobs] self-test qualified document-schema constant was falsely rejected.");
  if (evaluateToolJobSources({ "owner.rs": executableOwner.replace("registry.register(AppCommandJobFactory::new(\"owner\"));", "") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test app-owned factory without registration was falsely accepted.");
  if (evaluateToolJobSources({ "owner.rs": executableOwner.replace("fn build_tool_job(request: Request) -> Result<Option<ToolOperationSpec>, Fault> { build(request) }", "") }).length === 0)
    throw new Error("[verify interactivity tool-jobs] self-test app-owned factory without builder was falsely accepted.");
  const duplicateCatalog = { "owner.rs": validCatalog["owner.rs"]!.replace('tools: { "bounded" =>', 'tools: { "bounded" => semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100), "bounded" =>') };
  if (evaluateToolJobSources(duplicateCatalog).length === 0) throw new Error("[verify interactivity tool-jobs] self-test duplicate-proof was falsely accepted.");
  const forgedOwner = { "owner.rs": validCatalog["owner.rs"]!.replace("EditorApp<Ownerownerrs>", "EditorApp<ForgedOwner>") };
  if (evaluateToolJobSources(forgedOwner).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-owner was falsely accepted.");
  const forgedSchema = { "owner.rs": validCatalog["owner.rs"]!.replace('artifact_schema: "fixture.document"', 'artifact_schema: "forged.document"') };
  if (evaluateToolJobSources(forgedSchema).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-schema was falsely accepted.");
  const forgedFactory = { "owner.rs": validCatalog["owner.rs"]!.replace('factory: "BoundedFirstStepCommandJobFactory"', 'factory: "ForgedFactory"') };
  if (evaluateToolJobSources(forgedFactory).length === 0) throw new Error("[verify interactivity tool-jobs] self-test forged-factory was falsely accepted.");
  const extraProof = { "owner.rs": exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "extra") };
  if (toolJobProofCatalogFailures(new Map(Object.entries(extraProof)), [], new Map(), toolJobProofs(new Map(Object.entries(extraProof)))).length === 0) throw new Error("[verify interactivity tool-jobs] self-test extra-proof was falsely accepted.");
}

/** 🚦 Checks reserved route bypass, decode before lookup, contract mismatch, cancellation ownership, qualified proof order, alias fallback, reserved terminals and bounded full operation preparation. */
export function admissionOrderAndBoundedPreparationChecks() {
  const reservedBypass = 'async fn dispatch_action() { interactive-job.unknown-key; return self.dispatch_framework_reserved_action(action, args, meta).await; } async fn dispatch_framework_reserved_action() { self.store.dispatch(command); }';
  if (toolJobFrameworkReservedRoutesExact(reservedBypass)) throw new Error("[verify interactivity tool-jobs] self-test registryless-reserved-bypass was falsely accepted.");
  const decodeBeforeLookup = "async fn dispatch_action() { A::command_from_action(action, args).await; admit_command_json(action, args); } async fn dispatch_command() { A::command_from_action(action, args).await; admit_command_json(action, args); } async fn handle_intent_frame() { A::command_from_intent(intent).await; admit_command_wire(owner, tool); }";
  if (toolJobDecodeAfterAdmission(decodeBeforeLookup)) throw new Error("[verify interactivity tool-jobs] self-test decode-before-lookup was falsely accepted.");
  const mismatchProofs = toolJobProofs(new Map([["owner.rs", exactBoundedFirstStepProof("owner.rs", "owner@1/*#editor", "bounded", 64)]]));
  if (toolJobLimitsMatch(mismatchProofs, new Map([["owner@1/*#editor\0bounded", 128]]))) throw new Error("[verify interactivity tool-jobs] self-test contract-mismatch was falsely accepted.");
  const externalCancellation = "let cancellation = JobScope::root(); session.step(cancel.cancel_token());";
  if (toolJobExternalCancellationOwned(externalCancellation)) throw new Error("[verify interactivity tool-jobs] self-test external-cancellation was falsely accepted.");
  const unqualifiedRuntime = "fn bounded_first_step_contract(artifact_schema, id) {} fn bounded_first_step_public_wire_limit(command_id) {}";
  if (toolJobRuntimeProofQualified(unqualifiedRuntime)) throw new Error("[verify interactivity tool-jobs] self-test runtime-owner-proof-drop was falsely accepted.");
  const proofAfterDecode = "async fn admit_command_json() { bounded_json_items(); qualified_tool_proof(); } async fn handle_intent_frame() { serde_json::to_value(); qualified_tool_proof(); }";
  if (toolJobQualifiedProofBeforeDecode(proofAfterDecode)) throw new Error("[verify interactivity tool-jobs] self-test qualified-proof-after-decode was falsely accepted.");
  const delegatedQualifiedProof =
    'async fn admit_command_json_with_proof(proof: QualifiedToolProof) { let key = proof.key(); begin_exact_wire(); serde_json::to_writer(); proof.admits::<A>(&admission); } async fn admit_command_json() { let proof = self.qualified_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args); } async fn admit_host_configuration_json() { let proof = self.qualified_host_configuration_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args); } async fn handle_intent_frame() { qualified_tool_proof(verb); FaultCode::new("interactive-job.intent-raw-owner-required"); }';
  if (!toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof)) throw new Error("[verify interactivity tool-jobs] self-test delegated-qualified-proof route was falsely rejected.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("let proof = self.qualified_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args);", "admit_command_json_with_proof(proof, verb, args); let proof = self.qualified_tool_proof(verb)?;")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated app proof after helper was falsely accepted.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("let proof = self.qualified_host_configuration_tool_proof(verb)?; admit_command_json_with_proof(proof, verb, args);", "admit_command_json_with_proof(proof, verb, args); let proof = self.qualified_host_configuration_tool_proof(verb)?;")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated host proof after helper was falsely accepted.");
  if (toolJobQualifiedProofBeforeDecode(delegatedQualifiedProof.replace("begin_exact_wire(); serde_json::to_writer();", "serde_json::to_writer(); begin_exact_wire();")))
    throw new Error("[verify interactivity tool-jobs] self-test delegated helper decode before wire admission was falsely accepted.");
  const copiedOwnerStrings = 'struct QualifiedBoundedFirstStepProof; fn bounded_first_step_proof(controller_id: &str, factory: &str, tool_id: &str, artifact_schema: &str) { proof.owner_file != ""; } tool_job_registration(A::APP_ID, A::DOCUMENT_SCHEMA);';
  if (toolJobRuntimeProofQualified(copiedOwnerStrings)) throw new Error("[verify interactivity tool-jobs] self-test copied-owner-without-compiler-witness was falsely accepted.");
  const controllerScopedOperation = 'let app_id = app.instance_id().await.to_string(); format!("{}/{verb}/{}", self.tool_job_controller_id, base_revision.0); app_instance_id: meta.instance_id;';
  if (toolJobLiveInstanceIsolated(controllerScopedOperation)) throw new Error("[verify interactivity tool-jobs] self-test controller-scoped-runtime-instance was falsely accepted.");
  const typedAliasFallback = "pub fn dispatch(&self) { let exact = inner.aliases.get(&key); inner.factory_by_key.get(&exact); }";
  if (toolJobTypedDispatchExact(typedAliasFallback)) throw new Error("[verify interactivity tool-jobs] self-test typed-alias-fallback was falsely accepted.");
  const wireAliasFallback = "pub fn dispatch_wire(&self) { let exact = inner.aliases.get(&key); inner.factory_by_key.get(&exact); }";
  if (toolJobWireDispatchExact(wireAliasFallback)) throw new Error("[verify interactivity tool-jobs] self-test wire-alias-fallback was falsely accepted.");
  const genericReservedTerminal = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64, lanes: [${["cut", "paste", "import-media"].includes(id) ? "Artifact, Child" : "HostOnly"}]);`).join("\n")} impl FrameworkReservedCursor { fn step(&mut self) {} }`;
  if (toolJobFrameworkReservedRoutesExact(genericReservedTerminal)) throw new Error("[verify interactivity tool-jobs] self-test generic-reserved-terminal was falsely accepted.");
  const missingReservedFactory = TOOL_JOB_RESERVED_IDS.slice(1).map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64, lanes: [${["cut", "paste", "import-media"].includes(id) ? "Artifact, Child" : "HostOnly"}]);`).join("\n");
  if (toolJobFrameworkReservedRoutesExact(missingReservedFactory)) throw new Error("[verify interactivity tool-jobs] self-test missing-reserved-factory was falsely accepted.");
  const monolithicBinary = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64, lanes: [${["cut", "paste", "import-media"].includes(id) ? "Artifact, Child" : "HostOnly"}]);`).join("\n")} async fn dispatch_import_media() { A::import_media().await; } async fn dispatch_config_command_inner() { decode_op(); }`;
  if (toolJobFrameworkReservedRoutesExact(monolithicBinary)) throw new Error("[verify interactivity tool-jobs] self-test monolithic-reserved-binary was falsely accepted.");
  const earlyPermitFinish = `${TOOL_JOB_RESERVED_IDS.map((id, index) => `framework_reserved_job!(Job${index}, Factory${index}, "${id}", ${index}, 64, 1, 1, 64, lanes: [${["cut", "paste", "import-media"].includes(id) ? "Artifact, Child" : "HostOnly"}]);`).join("\n")} struct FrameworkReservedCommitPermit; async fn dispatch_action() { interactive-job.unknown-key; return self.dispatch_framework_reserved_action(action, args, meta).await; } async fn dispatch_framework_reserved_action() { permit.finish(); let result = if HISTORY_ACTION_IDS {} }`;
  if (toolJobFrameworkReservedRoutesExact(earlyPermitFinish)) throw new Error("[verify interactivity tool-jobs] self-test early-reserved-permit-finish was falsely accepted.");
  const envelopeOnlyReserved = "struct Job { raw: Vec<u8>, envelope_cursor: usize } impl InteractiveJob for Job { fn step() { Ok(self.raw.clone()); } }";
  if (toolJobFrameworkReservedRoutesExact(envelopeOnlyReserved)) throw new Error("[verify interactivity tool-jobs] self-test envelope-only-reserved-job was falsely accepted.");
  const postJobMonolith = "async fn dispatch_framework_reserved_action() { run_framework_reserved_job().await; dispatch_history_action().await; } fn ensure_reserved_emit_bounded() { serde_json::to_vec(&emit); }";
  if (toolJobFrameworkReservedRoutesExact(postJobMonolith)) throw new Error("[verify interactivity tool-jobs] self-test post-job-monolithic-operation was falsely accepted.");
  const preJobImportSerialization = "async fn dispatch_import_media() { let raw = serde_json::to_vec(&(port, media)); build_artifact_reserved_media_job(port, media, raw); }";
  if (toolJobImportPreparationBounded(preJobImportSerialization)) throw new Error("[verify interactivity tool-jobs] self-test pre-job-import-serialization was falsely accepted.");
  const instance = toolJobPluginInstanceSource(WORKSPACE_ROOT);
  const mutateIn = (owner: string, before: string, after: string): string => toolJobSourceMutation(instance, owner, before, after);
  const job = "impl<A: ArtifactApp> semio_framework_job::InteractiveJob for TypedCommandFullOperationJob<A>";
  const dispatch = "async fn dispatch_typed_command_inner(";
  if (!toolJobFullOperationBounded(instance)) throw new Error("[verify interactivity tool-jobs] self-test exact production typed-command full operation was falsely rejected.");
  const hostiles: [string, string][] = [
    ["typed-preparation-and-commit-outside-job", mutateIn(dispatch, "self.media.require_complete_tool_operation_pipeline(&admission)?;", "self.as_render().refresh_cache().await?; self.media.require_complete_tool_operation_pipeline(&admission)?;")],
    ["fake-full-operation-cancellation-cursor", mutateIn(job, "if cx.is_cancelled() {", "if cx.cancelled_flag() {")],
    ["full-operation-without-stage-watchdog", mutateIn(job, "cx.should_yield() || cx.fuel_remaining() == 0", "cx.fuel_remaining() == 0")],
    ["monolithic-huge-output-and-child-validation", mutateIn(job, 'cx.set_stage("typed-command-prepare");', 'cx.set_stage("typed-command-prepare"); let _ = serde_json::to_vec(&emit.effects);')],
    ["one-shot-generic-reducer-inside-worker", mutateIn(job, 'cx.set_stage("typed-command-reducer");', 'cx.set_stage("typed-command-reducer"); resolve_ready(A::handle(command, doc));')],
    ["generic-proof-without-reducer-fault", mutateIn(job, "generic bounded proof has no resumable app-owned reducer job", "generic bounded proof reduced")],
    ["full-operation-without-stale-result-validation", mutateIn("fn publish_mounted_typed_operation_unit", "typed_operation_document_is_fresh(&mounted.operation", "accept_stale_operation(&mounted.operation")],
  ];
  for (const [name, hostile] of hostiles) if (hostile === instance || toolJobFullOperationBounded(hostile)) throw new Error(`[verify interactivity tool-jobs] self-test ${name} was falsely accepted.`);
}
