import fundedBatchFixture from "./🧫️fixtures/🧾️funded-batch.json";

import { spawnSync } from "node:child_process";

import { readFileSync } from "node:fs";

import { join } from "node:path";

import { createRequire } from "node:module";

import persistentFoundationFixture from "./🧫️fixtures/🧾️persistent-foundation.json";

import contractExpressionFixture from "./🧫️fixtures/🧾️contract-expression.json";

import { toolJobContractSource, toolJobStaticRows, toolJobDispositions, toolJobProofs, toolJobAppOwnedRows, toolJobProductionProtocolSource, toolJobGlobalPayloadStoreInventory, toolJobGlobalPayloadStoreIsSharedFixedAuthority, toolJobScanThenMonolithRows, TOOL_JOB_RESERVED_IDS, toolJobPublicationContracts, toolJobPublicationAuthorityReady, toolJobProofCatalogFailures } from "@semio-tech/repo-lib/laws";

import { WORKSPACE_ROOT } from "@semio-tech/repo-lib/inventory";
import { toolJobRasterEnvelopeSourceEvidence, toolJobRasterEnvelopeCallerFailures, toolJobRasterEnvelopeCallerRetainedExact, toolJobStoreBatchPublicationBounded, toolJobTypedPersistentFoundation, toolJobPluginInstanceSource, toolJobStoreSource } from "@semio-tech/repo-lib/laws";
import { toolJobMemberStoreProducerExact } from "@semio-tech/framework-os/modules/store/initialization/owners/member/source";
import { memberStoreProducerSourceSelfTests } from "@semio-tech/framework-os/modules/store/initialization/owners/member/source/testing";
import { toolJobCheckpointSelfTests } from "../../🧵️retained-command/🧪️tests/🔬️tool-job-checkpoint/🟦️.ts";
import { toolJobScalarConfigCohortSelfTests } from "../../🧵️retained-command/🧪️tests/🔬️tool-job-scalar-config-cohort/🟦️.ts";
import { storeCanonicalEditSealerSelfTests } from "@semio-tech/framework-os/modules/store/canonical-edit/testing/store-canonical-edit-sealer";
import { toolJobOwnerFactoryResolutionSelfTests } from "../../🧵️retained-command/🧪️tests/🔬️tool-job-owner-factory-resolution/🟦️.ts";
import { toolJobFactoryProofJoinSelfTests } from "../🔬️tool-job-factory-proof-join/🟦️.ts";
import { toolJobLatestWinsSelfTests } from "../🔬️tool-job-latest-wins/🟦️.ts";
import { toolJobMicrosecondBudgetSelfTests } from "@semio-tech/framework/job/budget/testing/tool-job-microsecond-budget";
import { toolJobMicrosecondWorkerBindingSelfTests } from "@semio-tech/repo-lib/laws";
import { toolJobTelemetryContentionSelfTests } from "@semio-tech/framework/trace/clock/testing/tool-job-telemetry-contention";
import { toolJobCooperativeMaintenanceSelfTests } from "@semio-tech/framework-async/cooperative/testing/tool-job-cooperative-maintenance";
import { proceduralGenerationRootSelfTests } from "@semio-tech/framework-os/modules/playbook/artifacts/playbook/generation/testing/procedural-generation-root";
import { flowTypedRetirementSelfTests } from "@semio-tech/framework-os/modules/flow/artifacts/flow/retained/testing/flow-typed-retirement";
import { flowSelectedCopySelfTests } from "@semio-tech/framework-os/modules/flow/artifacts/flow/retained/copy/testing/flow-selected-copy";
import { admissionOrderAndBoundedPreparationChecks, interactiveJobEvaluationFixtures, proofCatalogEvaluationChecks } from "./🧾️proof-catalog-and-admission/🟦️.ts";
import { actorProgressOverlayChecks, fem2dMountedSessionChecks, globalPayloadStoreInventoryChecks } from "./🧯️payload-store-progress-and-mounted-session/🟦️.ts";
import { ephemeralPublicationChecks, freshTurnAndHostOnlyPublicationChecks, retainedPublicationChecks } from "./📤️publication-contracts/🟦️.ts";
import { segmentedOutputAndChildRootChecks, snapshotAndReactorCloseChecks, typedRouteCancellationAndCloseChecks } from "./🛑️cancellation-close-and-output-ownership/🟦️.ts";
import { artifactStoreStructuralAndEnvelopeCodecChecks } from "./🗄️artifact-store-and-envelope-codec/🟦️.ts";
import { gisMapEnvelopeCallerRetainedChecks, jackEnvelopeCallerRetainedChecks, trinityRewriteEnvelopeCallerRetainedChecks, writerEnvelopeCallerRetainedChecks } from "./✍️caller-retained-document-envelopes/🟦️.ts";
import { drawingEnvelopeMutationChecks, drawingEnvelopeRouteAndSemanticChecks, drawingEnvelopeSourceFixtures } from "./🎨️drawing-envelope/🟦️.ts";
import { commandIngressAndKernelRequestChecks, peerPresenceRosterChecks } from "./🤝️peer-presence-and-command-ingress/🟦️.ts";
import { layoutAndColdRelayRetainedChecks, universalRetainedOwnershipChecks } from "./🔭️universal-and-layout-retained-ownership/🟦️.ts";

/** 📬️ Checks paid original batch admission, folding, publication and retirement against independent expected denials. */
export function toolJobFundedBatchSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules");
  const source = {store:toolJobStoreSource(WORKSPACE_ROOT),plugin:toolJobPluginInstanceSource(WORKSPACE_ROOT)};
  const Ajv = createRequire(import.meta.url)("ajv"), oracle = new Ajv({strict:true}).compile({const:"none"});
  for (const law of fundedBatchFixture.cases) {
    const kind = law.owner as "store" | "plugin";
    if (law.id !== "none" && !source[kind].includes(law.before)) throw new Error(`batch hostile target is missing: ${law.id}`);
    const candidate = {...source, [kind]: law.id === "none" ? source[kind] : source[kind].replaceAll(law.before,law.after)};
    if (oracle(law.id) !== law.accepted || toolJobStoreBatchPublicationBounded(candidate.store,candidate.plugin) !== law.accepted) throw new Error(`funded batch authority law: ${law.id}`);
  }
  return fundedBatchFixture.cases.length;
}

/** 🪙️ Preserves exact producer, receipt, fairness and retirement authority across mounted current source owners. */
export function toolJobPersistentFoundationSelfTests(): number {
  const base = join(WORKSPACE_ROOT, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin");
  const source=toolJobPluginInstanceSource(WORKSPACE_ROOT);
  const Ajv = createRequire(import.meta.url)("ajv"), oracle = new Ajv({strict:true}).compile({const:"none"});
  for (const law of persistentFoundationFixture.cases) {
    if (law.id !== "none" && !source.includes(law.before)) throw new Error(`persistent hostile target is missing: ${law.id}`);
    const changed = law.id === "none" ? source : source.replaceAll(law.before, law.after);
    if (oracle(law.id) !== law.accepted || toolJobTypedPersistentFoundation(changed) !== law.accepted) throw new Error(`persistent authority law refused: ${law.id}`);
  }
  return persistentFoundationFixture.cases.length;
}

/** 🧪️ Executes tool job coverage policy assertions. */
export function toolJobContractSourceSelfTests(): number {
  for (const row of contractExpressionFixture.cases) {
    const actual = toolJobContractSource(row.source, row.expression) ?? null;
    if (JSON.stringify(actual) !== JSON.stringify(row.expected)) throw new Error(`contract expression law ${row.id}`);
  }
  for (const row of contractExpressionFixture.publicationCases) {
    const actual = toolJobPublicationContracts("", row.factory, new Map());
    if (JSON.stringify(actual ? [...actual] : null) !== JSON.stringify(row.expected)) throw new Error(`publication contract law ${row.id}`);
  }
  return contractExpressionFixture.cases.length + contractExpressionFixture.publicationCases.length;
}


/** 🧪️ Exercises current mounted Raster owners against neutral hostile vectors and an independent acceptance oracle. */
export function toolJobRasterSourceSelfTests(): number {
  const evidence = toolJobRasterEnvelopeSourceEvidence(WORKSPACE_ROOT), fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🖨️raster-source.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv"), schema = JSON.parse(readFileSync(join(import.meta.dir, "🧬️schema/🖨️raster-source.json"), "utf8")), validate = new Ajv({strict:true}).compile(schema);
  if (!validate(fixture)) throw new Error("Raster neutral source schema: " + JSON.stringify(validate.errors));
  const oracle = new Ajv({strict:true}).compile({const:fixture.acceptedIdentity});
  const identities = new Set<string>(), observedObligations = new Set<string>();
  for (const row of fixture.cases) {
    if (identities.has(row.id)) throw new Error("duplicate Raster hostile identity: " + row.id);
    identities.add(row.id);
    const owner = row.owner as "store" | "raster" | "editor" | "plugin" | "laws", current = evidence[owner];
    if (row.id !== fixture.acceptedIdentity && row.before && !current.includes(row.before)) throw new Error("Raster hostile has no current owner target: " + row.id);
    const candidate = {...evidence, [owner]: row.id === fixture.acceptedIdentity ? current : row.before ? current.replaceAll(row.before,row.after) : current + "\n" + row.after};
    if (row.id !== fixture.acceptedIdentity && candidate[owner] === current) throw new Error("Raster hostile did not change its owner: " + row.id);
    const failures = toolJobRasterEnvelopeCallerFailures(candidate.store,candidate.raster,candidate.editor,candidate.plugin,candidate.laws);
    for (const failure of failures) observedObligations.add(failure);
    if (oracle(row.id) !== row.accepted || (failures.length === 0) !== row.accepted || toolJobRasterEnvelopeCallerRetainedExact(candidate.store,candidate.raster,candidate.editor,candidate.plugin,candidate.laws) !== row.accepted) throw new Error("Raster authority " + row.id + ": " + failures.join(", "));
  }
  for (const obligation of fixture.obligations) if (!observedObligations.has(obligation)) throw new Error("Raster source obligation lacks a rejecting hostile: " + obligation);
  console.log("[DEBUG] Raster actual production owners=" + evidence.productionPaths.length + " mounted test owners=" + evidence.testPaths.length + " neutral authority cases=" + fixture.cases.length);
  return fixture.cases.length;
}

/** 🧱️ Keeps production fields and cfg alternatives while independently validating the exact remaining Rust syntax. */
export function toolJobRustProductionEvidenceSelfTests(): number {
  const fixture = JSON.parse(readFileSync(join(import.meta.dir, "🧫️fixtures/🖨️raster-source.json"), "utf8"));
  const Ajv = createRequire(import.meta.url)("ajv");
  for (const row of fixture.scrubberCases) {
    const production = toolJobProductionProtocolSource(row.source), expected = {kept:row.kept.map(() => true),removed:row.removed.map(() => false)}, observed = {kept:row.kept.map((text:string) => production.includes(text)),removed:row.removed.map((text:string) => production.includes(text))};
    const oracle = new Ajv({strict:true}).compile({const:expected});
    if (!oracle(observed) || production.length !== row.source.length) throw new Error("Rust production exclusion " + row.id + ": " + JSON.stringify(observed));
    const executable = process.platform === "win32" ? join(process.env.USERPROFILE!,".rustup/toolchains/stable-x86_64-pc-windows-msvc/bin/rustfmt.exe") : "rustfmt";
    const parsed = spawnSync(executable,["--edition","2021","--config","skip_children=true","--emit","stdout"],{input:production,encoding:"utf8",timeout:10000,stdio:["pipe","ignore","pipe"]});
    if (parsed.error || parsed.status !== 0) throw new Error("independent Rust production parser " + row.id + ": " + (parsed.error?.message ?? parsed.stderr));
  }
  console.log("[DEBUG] Rust production exclusion neutral cases=" + fixture.scrubberCases.length + " independently parsed");
  return fixture.scrubberCases.length;
}

export function toolJobCoverageSelfTests(): number {
  const { fixtures } = interactiveJobEvaluationFixtures();
  const contractChecks = toolJobContractSourceSelfTests() + toolJobPersistentFoundationSelfTests();
  const evaluate = (sources: Record<string, string>) => {
    const files = new Map(Object.entries(sources));
    const rows = toolJobStaticRows(files);
    const dispositions = toolJobDispositions(files);
    const proofs = toolJobProofs(files);
    const appOwned = new Set(toolJobAppOwnedRows(files).map((row) => `${row.ownerFile}\0${row.toolId}`));
    const catalogClean = toolJobProofCatalogFailures(files, rows, dispositions, proofs).length === 0;
    const registered = new Set(rows.map((row) => row.id));
    const remaining = rows.filter((row) => {
      const disposition = dispositions.get(`${row.file}\0${row.id}`);
      const unbounded = /for\s+item\s+in\s+items\b|while\s+true\b/.test(sources[row.file] ?? "");
      const proof = catalogClean ? proofs.find((candidate) => candidate.ownerFile === row.file && candidate.ownerTypeName.length > 0 && candidate.controllerId.length > 0 && candidate.factory.length > 0 && candidate.toolId === row.id && candidate.documentSchema.length > 0) : undefined;
      return !["BatchOnlyPendingRewrite", "ForbiddenFromUi", "Deleted"].includes(disposition ?? "") && !(disposition === "Migrated" && proof && !unbounded && appOwned.has(`${row.file}\0${row.id}`));
    });
    for (const [file, source] of files) {
      for (const match of source.matchAll(/dispatch\(\s*"([^"]+)"/g)) if (!registered.has(match[1]!)) remaining.push({ file, id: match[1]!, source: "literal" });
    }
    return remaining;
  };
  const admissionFixtures: readonly [string, Record<string, string>][] = [
    ["old-775-default", { "plugin.rs": 'app_commands! { "one" => one::Payload, "two" => two::Payload }' }],
    ["missing-alias-fallback", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "exact" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "exact" => exact::Payload }\n.action_interactive_job("exact", InteractiveJobClassification::Migrated)\ndispatch("alias");' }],
    ["nonmacro-puzzle", { "puzzle.rs": '.mutation("scramble", label, kind)' }],
    ["unbounded-handler", { "framework.rs": 'fn bounded_first_step_contract(id: &str) { match id { "export" => Some(semio_framework::ToolExecutionContract::bounded_first_step(64, 1, 1, 64, 100)), _ => None } }', "plugin.rs": 'app_commands! { "export" => export::Payload }\n.action_interactive_job("export", InteractiveJobClassification::Migrated)\nfn handle(items: &[u8]) { for item in items { work(item); } }' }],
    ["default-migrated-builder", { "manifest.rs": 'fn bounded_catalog() { definition.semantics.execution.interactive_job = InteractiveJobClassification::Migrated; }', "plugin.rs": 'app_commands! { "solve" => solve::Payload }' }],
  ];
  for (const [name, sources] of admissionFixtures) if (evaluate(sources).length === 0) throw new Error(`[verify interactivity tool-jobs] self-test ${name} was falsely accepted.`);
  const fundedBatchChecks=toolJobFundedBatchSelfTests();
  const rasterChecks=toolJobRasterSourceSelfTests()+toolJobRustProductionEvidenceSelfTests();
  const checkpointChecks = toolJobCheckpointSelfTests();
  globalPayloadStoreInventoryChecks();
  actorProgressOverlayChecks();
  fem2dMountedSessionChecks();
  proofCatalogEvaluationChecks();
  admissionOrderAndBoundedPreparationChecks();
  retainedPublicationChecks();
  ephemeralPublicationChecks();
  freshTurnAndHostOnlyPublicationChecks();
  typedRouteCancellationAndCloseChecks();
  snapshotAndReactorCloseChecks();
  segmentedOutputAndChildRootChecks();
  const memberProducerChecks = memberStoreProducerSourceSelfTests(toolJobMemberStoreProducerExact);
  artifactStoreStructuralAndEnvelopeCodecChecks();
  const { retainedWriterWasm, retainedWriterPlugin } = writerEnvelopeCallerRetainedChecks();
  const { retainedJackStore, retainedJackCodec, retainedJackEditor } = jackEnvelopeCallerRetainedChecks({ retainedWriterWasm, retainedWriterPlugin });
  trinityRewriteEnvelopeCallerRetainedChecks({ retainedJackStore, retainedJackCodec, retainedJackEditor, retainedWriterPlugin });
  gisMapEnvelopeCallerRetainedChecks({ retainedWriterWasm, retainedJackStore, retainedWriterPlugin });
  const { retainedDrawingCodec, retainedDrawingEditor } = drawingEnvelopeSourceFixtures();
  drawingEnvelopeRouteAndSemanticChecks({ retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin });
  drawingEnvelopeMutationChecks({ retainedJackStore, retainedDrawingCodec, retainedDrawingEditor, retainedWriterPlugin });
  peerPresenceRosterChecks();
  commandIngressAndKernelRequestChecks();
  universalRetainedOwnershipChecks();
  layoutAndColdRelayRetainedChecks();
  const scalarConfig = toolJobScalarConfigCohortSelfTests();
  const sealer = storeCanonicalEditSealerSelfTests();
  return contractChecks + fundedBatchChecks + fixtures.length + (398 - 102) + rasterChecks + checkpointChecks + toolJobOwnerFactoryResolutionSelfTests() + toolJobFactoryProofJoinSelfTests() + toolJobLatestWinsSelfTests() + toolJobMicrosecondBudgetSelfTests() + toolJobMicrosecondWorkerBindingSelfTests() + toolJobTelemetryContentionSelfTests() + toolJobCooperativeMaintenanceSelfTests() + proceduralGenerationRootSelfTests() + flowTypedRetirementSelfTests() + flowSelectedCopySelfTests() + scalarConfig.routes + scalarConfig.mutationOracles + scalarConfig.hostileCases + sealer.grants + sealer.editHostiles + sealer.sourceHostiles + sealer.digestOracles + sealer.mapGrants + sealer.mapEditHostiles + sealer.mapSourceHostiles + sealer.mapDigestOracles + sealer.readerChecks;
}
