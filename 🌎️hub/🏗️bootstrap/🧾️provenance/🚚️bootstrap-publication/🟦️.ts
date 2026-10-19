/** 🧭️ Trusted bootstrap materialization, rotation and publication of the current trusted catalog.
 * @see ../../../📦️packages/🦀️rust/📜️script.ts
 */
import { withTrustedCatalogCleanupV1 } from "../🧹️settlement/🟦️.ts";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { withRequestedIssuedResourceControlV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { configuredExactCargoLawPolicyV1 } from "@semio-tech/process/cargo-laws";
import { parseSpaceArtifactCreationStatusJsonV1 } from "@semio-tech/framework-os/modules/directory/io/text/space-artifact-creation-v1";
import { writeTrustedCatalogPublicationProvenanceV1 } from "../🟦️.ts";
import { withExactCargoLawOutputV1 } from "@semio-tech/process/testing/cargo/exact";
import { withCheckInProcessFixtureV1 } from "../📦️cargo/🧫️fixture/📥️reading/🟦️.ts";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, existsSync, lstatSync, mkdirSync, mkdtempSync, readFileSync } from "node:fs";
import { isAbsolute, join } from "node:path";
import { setTimeout as waitForHubPhaseV1 } from "node:timers/promises";
import { captureOwnedCommandResultV1 } from "@semio-tech/process/owned-execution";
import { buildAdmittedCargoArtifactsV1, runAdmittedExactCargoLawsV1 } from "@semio-tech/process/cargo-test";
import { parseDocumentOpenPlanV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { sealSpaceArtifactCreateV1 } from "@semio-tech/framework-os/modules/directory/schema/space-artifact-creation-v1";
import { exactCargoStageEnvironments } from "../../../🏗️build/🛂staging-environment/🟦️.ts";
import { issueLocalCredential } from "../../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, TRUSTED_CATALOG_READINESS_STALL_BOUND_MS, type LocalHubRun, type LocalHubHostV1, withLocalHubHostTaskV1, withLocalHubHostCandidateV1, withLocalHubRunTaskV1, withCapturedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { type TrustedBootstrapMaterializationV1, type TrustedBootstrapPublishedV1, trustedBootstrapRecordReceipt } from "../🗃️bootstrap-bundle/🟦️.ts";
import { trustedBootstrapExecutionProtocol } from "../🧮️bootstrap-claims/🟦️.ts";
import { trustedBootstrapReadRegular } from "../🧭️bootstrap-selection/🟦️.ts";
import { liveDirectoryCommandRequestId, postLiveDirectoryCommand } from "../../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { proveTrustedGisColdMapComponentV1, stageTrustedBootstrapCandidateCurrent } from "./🗺️gis-proofs-and-staging/🟦️.ts";
import { trustedBootstrapCurrent, trustedBootstrapPublicationRevision, trustedBootstrapReadCurrentPointer, trustedBootstrapVerifyPublication } from "./🔄️rotation-and-current-pointer/🟦️.ts";
import { runTrustedPublicationChild, trustedBootstrapPublicationReceipt, trustedPublicationChildReceipt, type TrustedPublicationChildResultV1 } from "./📮️receipts-and-publication-child/🟦️.ts";

/** 🔐️ Delegates the sole live compare-and-swap to the exact native Hub binary. */
async function publishTrustedBootstrapCurrent(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,receipt:TrustedBootstrapMaterializationV1,evidenceRoot:string,expected:ReturnType<typeof trustedBootstrapReadCurrentPointer>):Promise<TrustedBootstrapPublishedV1>{
  return withLocalHubHostTaskV1(host,phaseControl,context=>withRequestedIssuedResourceControlV1(context.control,0,async actual=>{
  const {invocation,dataRoot,binaryPath}=context;
  trustedBootstrapVerifyPublication(dataRoot, receipt);
  const publicationRevision = trustedBootstrapPublicationRevision((BigInt(expected?.pointer.publicationRevision ?? "0") + 1n).toString());
  const command = Object.freeze({ schema: "semio.hub.trusted-catalog-publication/v1", requestId: randomBytes(16).toString("hex"), profileId: receipt.profileId, generationId: receipt.generationId, bundleSha256: receipt.bundleSha256, expectedCurrentSha256: expected?.sha256 ?? null });
  const input = Buffer.from(JSON.stringify(command) + "\n");
  if (!isAbsolute(dataRoot) || !isAbsolute(binaryPath) || input.length > 4096) throw new Error("trusted publication transport requires exact bounded authority");
  let lastProgressMs=performance.now();
  const control={cancelled:()=>actual.signal.aborted,remainingMs:()=>Math.min(checkResourceLeaseControl(actual)??Infinity,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS-(performance.now()-lastProgressMs)),progressed:()=>{checkResourceLeaseControl(actual);lastProgressMs=performance.now();}};
  let processResult: TrustedPublicationChildResultV1 | undefined;
  try {
    processResult = await runTrustedPublicationChild(binaryPath, ["trusted-catalog", "publish"], input, dataRoot, evidenceRoot, control);
    const published = trustedPublicationChildReceipt(processResult, command, publicationRevision, evidenceRoot);
    const current = trustedBootstrapCurrent(dataRoot);
    if (!current || current.currentSha256 !== published.currentSha256) throw new Error("trusted current changed after native publication; do not activate the candidate plan");
    await writeTrustedCatalogPublicationProvenanceV1(dataRoot,receipt.producerPath,{maxBytes:134217728,maxWork:65536,chunkBytes:1048576,cancelled:()=>actual.signal.aborted,remainingMs:()=>checkResourceLeaseControl(actual)??Infinity,onProgress:()=>{checkResourceLeaseControl(actual);}});
    return current;
  } finally {
    processResult?.bytes.fill(0);
    input.fill(0);

  }
  }));
}

async function proveTrustedStdioGisCandidatePlan(run: LocalHubRun, receipt: TrustedBootstrapMaterializationV1, envelope: Record<string, any>): Promise<Record<string, any>> {
  return withLocalHubRunTaskV1(run,async control=>{
  const bundle = JSON.parse(readFileSync(receipt.bundlePath, "utf8"));
  const profile = bundle.profiles?.find((candidate: any) => candidate.id === receipt.profileId);
  const selected = bundle.packages?.find((candidate: any) => candidate.pluginId === "gis");
  const target = profile?.openTargets?.find((row: any) => row?.package?.pluginId === "gis" && row?.target?.role === "editor")?.target;
  if (!profile || !selected || !target) throw new Error("trusted stdio+GIS candidate bundle lost its exact GIS target");
  const headers = { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" };
  const created = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "Trusted GIS Bootstrap Probe", spaceKind: "studio", visibility: "private" });
  const createdBody = created.status === 202 ? (JSON.parse(created.text) as Record<string, any>) : undefined;
  const spaceId = createdBody?.events?.find((candidate: any) => candidate?.body?.kind === "space.created")?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("trusted stdio+GIS candidate could not create its private probe space");
  const creationRoute = `/spaces/${encodeURIComponent(spaceId)}/artifact-creations`;
  const creationRequest = sealSpaceArtifactCreateV1({ requestId: randomBytes(16).toString("hex"), expectedCatalogGenerationId: receipt.generationId, kindId: target.artifactKind, name: "Trusted GIS Bootstrap Probe Map" });
  const accepted = await fetch(`http://127.0.0.1:${run.port}${creationRoute}`, { method: "POST", headers, signal:AbortSignal.any([control.signal,AbortSignal.timeout(5000)]), body: JSON.stringify(creationRequest) });
  const acceptedText = await accepted.text();
  if (![200, 202].includes(accepted.status)) throw new Error(`trusted stdio+GIS candidate could not create its GIS Map probe: HTTP ${accepted.status} ${JSON.stringify(acceptedText.slice(0, 512))}`);
  let creation = parseSpaceArtifactCreationStatusJsonV1(acceptedText);
  if (creation.requestId !== creationRequest.requestId || creation.spaceId !== spaceId || creation.catalogGenerationId !== creationRequest.expectedCatalogGenerationId)
    throw new Error(`trusted stdio+GIS candidate GIS Map probe answered another creation: ${JSON.stringify(acceptedText.slice(0, 512))}`);
  const creationDeadline = Date.now() + 120_000;
  while (creation.phase !== "ready") {
    if (!["accepted", "preparing", "indeterminate"].includes(creation.phase) || Date.now() >= creationDeadline) throw new Error(`trusted stdio+GIS candidate creation reached ${creation.phase} before Ready`);
    checkResourceLeaseControl(control);await waitForHubPhaseV1(20,undefined,{signal:control.signal});
    const polled = await fetch(`http://127.0.0.1:${run.port}${creationRoute}/${encodeURIComponent(creationRequest.requestId)}`, { headers: { authorization: headers.authorization }, signal:AbortSignal.any([control.signal,AbortSignal.timeout(5000)]) });
    if (polled.status !== 200 && polled.status !== 202) throw new Error(`trusted stdio+GIS candidate creation status failed: ${polled.status}`);
    creation = parseSpaceArtifactCreationStatusJsonV1(await polled.text());
    if (creation.catalogGenerationId !== creationRequest.expectedCatalogGenerationId) throw new Error("trusted stdio+GIS candidate creation crossed its selected catalog generation");
  }
  const ready = creation.ready;
  if (!ready || ready.kindId !== target.artifactKind || ready.artifactSchema !== target.artifactSchema || !/^artifact-[0-9a-f]{32}$/u.test(ready.artifactId))
    throw new Error("trusted stdio+GIS candidate creation Ready coordinates differ from the selected GIS target");
  const documentId = ready.artifactId;
  const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/open-plan`, {
    method: "POST",
    headers,
    signal:AbortSignal.any([control.signal,AbortSignal.timeout(2000)]),
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId }, requestedSurfaceId: target.surfaceId, clientInstanceId: "trusted-bootstrap-candidate" }),
  });
  const planText = await response.text();
  if (!response.ok) throw new Error(`trusted stdio+GIS candidate was refused its GIS Map plan: ${response.status} ${planText.slice(0, 256)}`);
  const plan = parseDocumentOpenPlanV1(JSON.parse(planText), Date.now());
  if (
    !response.ok ||
    plan.scope.spaceId !== spaceId ||
    plan.scope.documentId !== documentId ||
    plan.catalog.generationId !== receipt.generationId ||
    JSON.stringify(plan.package) !==
      JSON.stringify({
        pluginId: selected.pluginId,
        packageId: selected.packageId,
        version: selected.version,
        componentSha256: selected.component.sha256,
        componentBlake3: selected.component.blake3,
        descriptorByteSha256: selected.descriptor.sha256,
        executionProtocol: trustedBootstrapExecutionProtocol(selected.executionProtocol),
        componentReceipt: trustedBootstrapRecordReceipt(selected),
      }) ||
    JSON.stringify(plan.artifact) !== JSON.stringify({ kind: target.artifactKind, schema: target.artifactSchema, packSchemaHash: target.packSchemaHash }) ||
    JSON.stringify(plan.parentDialect) !== JSON.stringify(target.parentDialect) ||
    JSON.stringify(plan.surface) !== JSON.stringify({ surfaceId: target.surfaceId, appId: target.appId, windowKindId: target.windowKindId, role: target.role, rendererTarget: target.rendererTarget }) ||
    JSON.stringify(plan.grant) !== JSON.stringify(target.grant)
  )
    throw new Error("trusted stdio+GIS candidate issued a substituted GIS Map plan");
  return plan;
  });
}

async function proveTrustedStdioGisStalePlanRejected(run: LocalHubRun, stalePlan: Record<string, any>, envelope: Record<string, any>): Promise<void> {
  return withLocalHubRunTaskV1(run,async control=>{
  try {
    const response = await fetch(`http://127.0.0.1:${run.port}/spaces/${encodeURIComponent(stalePlan.scope.spaceId)}/documents/${encodeURIComponent(stalePlan.scope.documentId)}/socket-grants`, {
      method: "POST",
      headers: { authorization: `Bearer ${envelope.capability}`, "content-type": "application/json" },
      signal:AbortSignal.any([control.signal,AbortSignal.timeout(2000)]),
      body: JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: stalePlan.receipt }),
    });
    const body = (await response.json().catch(() => undefined)) as Record<string, any> | undefined;
    if (response.status !== 401 || JSON.stringify(body) !== JSON.stringify({ schema: "semio.hub.document-open-plan-error/v1", code: "denied" })) throw new Error("prior-generation authenticated plan was not terminally denied before fresh issuance");
  } finally {
    stalePlan.receipt = "";
  }
  });
}

/** 🧪️ Qualifies the real Hub publication command with explicit synthetic guest fixture bytes. */
export async function proveTrustedPublicationCli(invocation: ScriptInvocation, control:CompilerAdmissionControl, repoRoot: string, hubRoot: string): Promise<void> {
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  await runAdmittedExactCargoLawsV1({cwd: repoRoot,...exactCargoStageEnvironments(),groups: [{
    package: "semio-hub",
    target: { kind: "bin", name: "os-hub" },
    cargoArgs: ["--features", "integration-fixtures"],
    laws: ["trusted_catalog_command::tests::trusted_publication_transport_matches_neutral_arguments_and_input_bounds"],
  },
  {
    package: "semio-hub-boot-laws",
    target: { kind: "lib" },
    cargoArgs: ["--features", "integration-fixtures"],
    laws: ["tests::check_in_process_fixture_emits_verified_gis_ledger_and_catalog"],
  }],invocation,control,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 120_000, lawMilliseconds: 180_000 },consume:outputs=>withExactCargoLawOutputV1(outputs,{version:1,receiptIndex:outputs.receipts.findIndex(row=>row.package==="semio-hub-boot-laws")},output=>buildAdmittedCargoArtifactsV1(join(hubRoot,"Cargo.toml"),["--features","integration-fixtures","--bin","os-hub"],repoRoot,{producer:"artifact",invocation,control:output.control,consume:capture=>withCapturedLocalHubHostV1(invocation,output.control,capture,(host,context)=>withCheckInProcessFixtureV1(host,context.control,outputs,{version:1,purpose:"publication"},async(data,actualPhase)=>{
  const {dataRoot,binaryPath,diagnosticsRoot:evidence}=context;
  const bundle=trustedBootstrapReadRegular(join(dataRoot,"trusted-catalog/generations",data.fixture.generationId,"trusted-catalog.json"),4*1024*1024,"publication CLI retained fixture",()=>{checkResourceLeaseControl(actualPhase);});
  let bundleSha256:string;try{bundleSha256=createHash("sha256").update(bundle).digest("hex");}finally{bundle.fill(0);}
  const command={schema:"semio.hub.trusted-catalog-publication/v1",requestId:"12".repeat(16),profileId:data.fixture.profileId,generationId:data.fixture.generationId,bundleSha256,expectedCurrentSha256:null as string|null};
  const run=async(name:string,value:Record<string,unknown>):Promise<TrustedPublicationChildResultV1>=>withLocalHubHostTaskV1(host,actualPhase,owner=>withRequestedIssuedResourceControlV1(owner.control,40000,async phase=>{
   if(!/^[a-z-]{1,32}$/u.test(name))throw Error("Closed publication case name required");const caseRoot=join(evidence,name);checkResourceLeaseControl(phase);mkdirSync(caseRoot);
   const input=JSON.stringify(value)+"\n";if(Buffer.byteLength(input)>4096)throw Error("Bounded publication command required");
   const result=await captureOwnedCommandResultV1(binaryPath,["trusted-catalog","publish"],owner.root,{control:phase,env:{...owner.environment,OS_HUB_DATA:dataRoot},input:{version:1,text:input},output:{maximumBytes:69632,maximumLines:2048}});
   const bytes=Buffer.from(result.stdout);if(bytes.length>4096||Buffer.byteLength(result.stderr)>65536){bytes.fill(0);throw Error("Bounded publication terminal output required");}return{bytes,status:result.status,signal:result.signal,failed:""};
  }));
  const first = await run("first", command);
  assert.equal(first.failed, ""); assert.equal(first.signal, null);
  const receipt = trustedBootstrapPublicationReceipt(first.bytes, command, "1");
  assert.equal(first.status, receipt.outcome === "durable" ? 0 : 1);
  const initial = trustedBootstrapReadCurrentPointer(dataRoot); assert(initial);
  assert.equal(initial.sha256, receipt.currentSha256); assert.equal(initial.pointer.publicationRevision, "1");
  command.expectedCurrentSha256 = initial.sha256; command.requestId = "34".repeat(16);
  const next = await run("next", command);
  assert.equal(next.failed, ""); assert.equal(next.signal, null);
  const nextReceipt = trustedBootstrapPublicationReceipt(next.bytes, command, "2");
  assert.equal(next.status, nextReceipt.outcome === "durable" ? 0 : 1);
  const current = trustedBootstrapReadCurrentPointer(dataRoot); assert(current);
  assert.equal(current.sha256, nextReceipt.currentSha256); assert.notEqual(current.sha256, initial.sha256);
  const retained = trustedBootstrapReadRegular(join(dataRoot, "trusted-catalog/current.json"), 65536, "CLI current", () => {});
  for (const [name, value] of [
    ["stale", { ...command, requestId: "56".repeat(16) }],
    ["caller-path", { ...command, expectedCurrentSha256: current.sha256, bundlePath: "/untrusted" }],
    ["missing-token", { ...command, expectedCurrentSha256: undefined }],
  ] as const) {
    const result = await run(name, value);
    assert.equal(result.status, 1); assert.equal(result.signal, null); assert.equal(result.failed, ""); assert.equal(result.bytes.length, 0);
    assert.deepEqual(trustedBootstrapReadRegular(join(dataRoot, "trusted-catalog/current.json"), 65536, "CLI retained current", () => {}), retained);
    result.bytes.fill(0);
  }
  first.bytes.fill(0); next.bytes.fill(0); retained.fill(0);
  console.log(`trusted publication CLI: real-Hub=1 compiled-provider=1 revisions=1,2 stale=refused invalid=2 outcome=${nextReceipt.outcome} evidence=${evidence}; synthetic guest fixture, no guest execution or activation claim`);
 }))})),progress(event){console.log(`trusted-publication-cli-native ${event.stage}: ${event.law??""} artifacts=${event.artifactDir}`);}});
}

/** 🗂️ Allocates private qualification diagnostics outside immutable generations and current. */
function trustedBootstrapValidationRoot(dataRoot: string): string {
  const trustedRoot = join(dataRoot, "trusted-catalog");
  for (const directory of [dataRoot, trustedRoot]) {
    const info = lstatSync(directory);
    if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted validation parent must be a regular directory");
  }
  const root = join(trustedRoot, "validation");
  if (!existsSync(root)) mkdirSync(root, { mode: 0o700 });
  const info = lstatSync(root);
  if (info.isSymbolicLink() || !info.isDirectory()) throw new Error("trusted validation root must be a regular directory");
  if (process.platform !== "win32") chmodSync(root, 0o700);
  return mkdtempSync(join(root, "gis-"));
}

/** 🟢️ Publishes current metadata only after exact GIS proof and isolated candidate validation. */
export async function validateAndPublishTrustedStdioGisCandidate(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,receipt:TrustedBootstrapMaterializationV1,stalePlan?:Record<string,any>):Promise<Record<string,any>>{
 return withLocalHubHostTaskV1(host,phaseControl,async context=>{
  const {invocation,workspace:repoRoot,dataRoot,control}=context,expectedCurrent=trustedBootstrapReadCurrentPointer(dataRoot),validationRoot=trustedBootstrapValidationRoot(dataRoot);
  await proveTrustedGisColdMapComponentV1(invocation,control,repoRoot,receipt);
  return withLocalHubHostCandidateV1(host,control,async(candidateHost,candidateContext)=>{
   const profile:LocalProfile={profileId:"trusted-bootstrap-probe",subject:"trusted-bootstrap-subject",displayName:"Trusted Bootstrap Probe",allowedClientClasses:["native"]};
   stageTrustedBootstrapCandidateCurrent(candidateContext.dataRoot,receipt,()=>{checkResourceLeaseControl(candidateContext.control);});
   const candidate=await startLocalHub(candidateHost,[profile],{capture:true});
   let envelope:Record<string,any>|undefined;
   return withTrustedCatalogCleanupV1(()=>withLocalHubRunTaskV1(candidate,async actual=>{
    const readiness=await waitForReadiness(candidate,false,TRUSTED_CATALOG_READINESS_STALL_BOUND_MS);
    if(readiness.artifactAuthority?.ready!==true||readiness.features?.openPlan!==true||readiness.features?.openPlanExchange!==true)throw Error("trusted stdio+GIS candidate did not expose one verified document-open authority");
    envelope=await issueLocalCredential(candidate,profile.profileId,"native");
    if(stalePlan)await proveTrustedStdioGisStalePlanRejected(candidate,stalePlan,envelope);
    const plan=await proveTrustedStdioGisCandidatePlan(candidate,receipt,envelope);
    await publishTrustedBootstrapCurrent(host,actual,receipt,validationRoot,expectedCurrent);
    return plan;
   }),[()=>{if(envelope)envelope.capability="";},()=>finishLocalHub(candidate)]);
  });
 });
}
