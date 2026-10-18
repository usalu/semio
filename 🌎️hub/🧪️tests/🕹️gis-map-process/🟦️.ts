/** 🧭️ The GIS map process gate: Shell peers and the two-author proposal process.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { checkResourceLeaseControl } from "@semio-tech/process/leases";
import { repositoryCargoNetworkV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { verifyCapturedCargoArtifactsV1 } from "@semio-tech/process/native-build";
import { readNativeCallerAuthorityV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import { type CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { withRequestedIssuedResourceControlV1, requireIssuedResourceControlInvocationV1, scriptInvocationEnvironment, type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { withExactCargoLawOutputV1 } from "@semio-tech/process/testing/cargo/exact";
import { withCheckInProcessFixtureV1 } from "../../🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/📥️reading/🟦️.ts";
import type { CheckInProcessFixtureDataV1 } from "../../🏗️bootstrap/🧾️provenance/📦️cargo/🧫️fixture/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { chmodSync, mkdtempSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, join } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import { withRepositoryPublishedCargoExecutableV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/📥️published/🟦️.ts";
import { buildRepositoryCargoArtifacts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, type ArtifactFrontier } from "@semio-tech/framework/directory/schema";
import { artifactFrontierIsGenesisForV1, artifactFrontierIsEditedForV1 } from "@semio-tech/framework/directory/schema";
import type { TestBrowserHostRootsV1 } from "@semio-tech/s-dev-catalog-ts/browser-host";
import { withTestBrowserHostV1 } from "@semio-tech/s-dev-catalog-ts/browser-host/staging";
import { runRepositoryExactCargoLaws, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/🎯️exact/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, freeLoopbackPort, type LocalHubRun, type LocalHubHostV1, withLocalHubHostTaskV1, withCapturedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { type TrustedBootstrapMaterializationV1, type TrustedBootstrapPublishedV1, trustedBootstrapWriteNew } from "../../🏗️bootstrap/🧾️provenance/🗃️bootstrap-bundle/🟦️.ts";
import { validateAndPublishTrustedStdioGisCandidate } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { trustedBootstrapReadCurrentBundle, withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import { trustedBootstrapCurrent, trustedBootstrapVerifyPublication } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🔄️rotation-and-current-pointer/🟦️.ts";
import { TRUSTED_BOOTSTRAP_LINKED_PACKAGES, trustedBootstrapSelectPackages } from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
import { checkInProcessHeadV1, commitCheckInProcessEdit, createCheckInProcessGenesis } from "../🪪️check-in-process/🟦️.ts";
import { liveDirectoryEventPageUser } from "../📣️directory-live-support/🟦️.ts";
import { proveGisMapProposalApprovalFixture } from "../🌍️gis-map-fixtures/🟦️.ts";
import { cancelGisMapProcessMcpOwner, finishGisMapProcessDocumentSocket, finishGisMapProcessMcpClient, type GisMapCheckpointProjectionV1, type GisMapDocumentOpenFixtureV1, type GisMapProcessDocumentSocketV1, gisMapProcessHttp, type GisMapProcessMcpClientV1, openGisMapProcessDocumentSocket, readGisMapProcessCheckpoint, startGisMapProcessMcpClient, waitForGisMapProcessInference } from "../🎮️gis-map-process-support/🟦️.ts";
import { type CheckInProcessFixtureV1, liveDirectoryCommandRequestId, postLiveDirectoryCommand, waitForDocumentSocketFrame, waitForUiReadiness } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { startLocalBrowserRelay } from "../../🚀️local-relay/🛰️servers/🟦️.ts";
import { hubLawGroups } from "../../🧪️testing/📖️source-readers/🟦️.ts";

type GisMapMountedShellProbeV1 = Readonly<{
  scope: Readonly<{ spaceId: string; documentId: string }>;
  clientInstanceId: string;
  activationGeneration: string;
  catalogGenerationId: string;
  componentSha256: string;
  descriptorSha256: string;
  browserActorSha256: string;
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  uiRevision: number;
  rootKind: "tiled-map";
  regionIds: readonly string[];
}>;

type GisMapShellPeerV1 = Readonly<{
  scope: Readonly<{ spaceId: string; documentId: string }>;
  readMap(): Promise<GisMapMountedShellProbeV1 | null>;
  propose(): Promise<string>;
  approve(): Promise<void>;
  undo(): Promise<void>; redo(): Promise<void>; stop(): Promise<void>;
}>;

/** 🌿️ Parses one closed scope-bound frontier from the mounted browser's public acknowledgement. */
function parseGisMapMountedFrontierV1(value: unknown, scope: Readonly<{ spaceId: string; documentId: string }>): ArtifactFrontier | null {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return null;
  const frontier = value as ArtifactFrontier;
  if (
    JSON.stringify(Object.keys(frontier).sort()) !== JSON.stringify(["chainHash", "documentId", "headEditId", "headEditOrdinal", "lastCommitSeq"]) ||
    !Array.isArray(frontier.chainHash) ||
    frontier.chainHash.length !== 32 ||
    frontier.chainHash.some((byte) => !Number.isInteger(byte) || byte < 0 || byte > 255) ||
    !Number.isSafeInteger(frontier.headEditOrdinal) ||
    !Number.isSafeInteger(frontier.lastCommitSeq) ||
    frontier.lastCommitSeq > frontier.headEditOrdinal ||
    (!artifactFrontierIsGenesisForV1(scope, frontier) && !artifactFrontierIsEditedForV1(scope, frontier))
  ) return null;
  return structuredClone(frontier);
}

/** 🔬️ Validates the closed dev probe before the process harness compares scene facts. */
function parseGisMapMountedShellProbeV1(value: unknown, scope: Readonly<{ spaceId: string; documentId: string }>): GisMapMountedShellProbeV1 | null {
  if (value === null) return null;
  if (typeof value !== "object" || Array.isArray(value)) throw new Error("mounted GIS Map probe is not an object");
  const row = value as Record<string, unknown>;
  const keys = ["activationGeneration", "activeCheckpointId", "browserActorSha256", "catalogGenerationId", "clientInstanceId", "componentSha256", "descriptorDigestV1", "descriptorSha256", "frontier", "regionIds", "rootKind", "scope", "uiRevision"].sort();
  if (JSON.stringify(Object.keys(row).sort()) !== JSON.stringify(keys)) throw new Error("mounted GIS Map probe fields are not closed");
  const probeScope = row.scope;
  if (
    probeScope === null ||
    typeof probeScope !== "object" ||
    Array.isArray(probeScope) ||
    JSON.stringify(Object.keys(probeScope).sort()) !== JSON.stringify(["documentId", "spaceId"]) ||
    (probeScope as Record<string, unknown>).spaceId !== scope.spaceId ||
    (probeScope as Record<string, unknown>).documentId !== scope.documentId
  ) throw new Error("mounted GIS Map probe scope changed");
  const boundedIdentity = (candidate: unknown): candidate is string =>
    typeof candidate === "string" &&
    candidate.length > 0 &&
    Buffer.byteLength(candidate, "utf8") <= 256 &&
    !/[\u0000-\u001f\u007f]/u.test(candidate);
  const digest = (candidate: unknown): candidate is string => typeof candidate === "string" && /^[0-9a-f]{64}$/u.test(candidate);
  const clientInstanceId = row.clientInstanceId;
  const activationGeneration = row.activationGeneration;
  const catalogGenerationId = row.catalogGenerationId;
  const componentSha256 = row.componentSha256;
  const descriptorSha256 = row.descriptorSha256;
  const browserActorSha256 = row.browserActorSha256;
  const activeCheckpointId = row.activeCheckpointId;
  const descriptorDigestV1 = row.descriptorDigestV1;
  const frontier = parseGisMapMountedFrontierV1(row.frontier, scope);
  const uiRevision = row.uiRevision;
  if (
    !boundedIdentity(clientInstanceId) ||
    !boundedIdentity(activationGeneration) ||
    !boundedIdentity(catalogGenerationId) ||
    !digest(componentSha256) ||
    !digest(descriptorSha256) ||
    !digest(browserActorSha256) ||
    !digest(activeCheckpointId) ||
    !digest(descriptorDigestV1) ||
    frontier === null ||
    !Number.isSafeInteger(uiRevision) ||
    (uiRevision as number) < 1 ||
    row.rootKind !== "tiled-map" ||
    !Array.isArray(row.regionIds) ||
    row.regionIds.length > 4_096
  ) throw new Error("mounted GIS Map probe identity is invalid");
  const regionIds = row.regionIds as unknown[];
  if (
    regionIds.some((regionId) => !boundedIdentity(regionId)) ||
    new Set(regionIds).size !== regionIds.length ||
    JSON.stringify(regionIds) !== JSON.stringify([...regionIds].sort())
  ) throw new Error("mounted GIS Map probe regions are invalid");
  return Object.freeze({
    scope: Object.freeze({ ...scope }),
    clientInstanceId,
    activationGeneration,
    catalogGenerationId,
    componentSha256,
    descriptorSha256,
    browserActorSha256,
    activeCheckpointId,
    descriptorDigestV1,
    frontier: structuredClone(frontier),
    uiRevision: uiRevision as number,
    rootKind: "tiled-map",
    regionIds: Object.freeze(regionIds as string[]),
  });
}

/** 🗺️ Starts one isolated ordinary React Shell peer and opens only Directory-derived rows. */
async function startGisMapShellPeerV1(options: {
  readonly repoRoot: string;
  readonly invocation:ScriptInvocation;
  readonly control:CompilerAdmissionControl;
  readonly run: LocalHubRun;
  readonly profileId: string;
  readonly scope: Readonly<{ spaceId: string; documentId?: string }>;
  readonly creation?: Readonly<{ kindId: string; name: string }>;
  readonly locale: "en" | "de";
  readonly artifactRoot: string;
  readonly browserHost: TestBrowserHostRootsV1;
  readonly browser: import("playwright").Browser;
  readonly credentialEnvelope: Record<string, any>;
}): Promise<GisMapShellPeerV1> {
  requireIssuedResourceControlInvocationV1(options.control,options.invocation);
  const existingDocumentId = options.scope.documentId;
  const creates = options.creation !== undefined;
  if (
    !isAbsolute(options.artifactRoot) ||
    !isAbsolute(options.browserHost.artifactRoot) ||
    options.browserHost.receipt.schema !== "semio.s.test-browser-host-staging/v1" ||
    (creates === (existingDocumentId !== undefined)) ||
    (options.creation !== undefined && (!/^[A-Za-z0-9][A-Za-z0-9._:/-]{0,255}$/u.test(options.creation.kindId) || options.creation.name.trim() !== options.creation.name || options.creation.name.length === 0 || [...options.creation.name].length > 128 || /[\u0000-\u001f\u007f]/u.test(options.creation.name))) ||
    options.credentialEnvelope.schema !== "semio.hub.local-credential-envelope/v1" ||
    options.credentialEnvelope.clientClass !== "react-relay" ||
    options.credentialEnvelope.profileId !== options.profileId
  ) throw new Error("GIS Map Shell peer authority is invalid");
  const uiPort = await freeLoopbackPort();
  const uiOrigin = "http://127.0.0.1:" + uiPort;
  const hubOrigin = "http://127.0.0.1:" + options.run.port;
  const relay = startLocalBrowserRelay(hubOrigin, uiOrigin, options.credentialEnvelope);
  const peerDataRoot = mkdtempSync(join(options.artifactRoot, "shell-peer-" + options.profileId + "-"));
  if (process.platform !== "win32") chmodSync(peerDataRoot, 0o700);
  const devRoot = join(options.repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript");
  const environment=scriptInvocationEnvironment(options.invocation,{
      ...process.env,
      SEMIO_TEST_ARTIFACT_DIR: options.browserHost.artifactRoot,
      SEMIO_TEST_BROWSER_MODULE_ROOT: options.browserHost.moduleRoot,
      SEMIO_TEST_BROWSER_ACTIVATION_ROOT: options.browserHost.activationRoot,
      SEMIO_TEST_BROWSER_HOST_RECEIPT: options.browserHost.receiptPath,
      SEMIO_PLUGIN: "s",
      SEMIO_RENDERER: "react",
      S_OS_PORT: String(uiPort),
      S_HUB_URL: hubOrigin,
      S_LOCAL_RELAY_URL: relay.url,
      S_LOCAL_RELAY_SECRET: relay.secret.toString("hex"),
      VITE_SEMIO_LOCKED_LOCALE: options.locale,
      S_DATA_DIR: peerDataRoot,
    });
  const limits=readNativeCallerAuthorityV1(options.invocation.capabilities).transport;
  let childOutput="";
  let daemon:Readonly<{child:ChildProcess;kill:()=>void}>|undefined,startedResolve=()=>{},startedReject:(reason:unknown)=>void=()=>{};
  const started=new Promise<void>((accept,reject)=>{startedResolve=accept;startedReject=reject;});
  const lifetime=withRequestedIssuedResourceControlV1(options.control,0,async control=>{
    checkResourceLeaseControl(control);
    let stopping=false,failed=false,closed=false,failure:unknown,timer:ReturnType<typeof setTimeout>|undefined,bytes=0,lines=0,progressWork:Promise<void>=Promise.resolve();const childStarted=performance.now();const lineOpen={stdout:false,stderr:false};
    const remember=(error:unknown)=>{if(!failed){failed=true;failure=error;}};
    const child=spawn(process.execPath,[join(devRoot,"📜️script.ts"),"serve","s","react","dev"],{cwd:devRoot,env:environment,shell:false,detached:process.platform!=="win32",stdio:["ignore","pipe","pipe"],windowsHide:true});
    const terminal=new Promise<{code:number|null;signal:string|null}>(accept=>{child.once("error",error=>{remember(error);try{terminateOwnedChildTree(child);}catch(fault){remember(fault);}});child.once("close",(code,signal)=>{closed=true;accept({code,signal});});});
    const fail=(error:unknown)=>{remember(error);try{terminateOwnedChildTree(child);}catch(fault){remember(fault);}},stop=()=>{stopping=true;try{terminateOwnedChildTree(child);}catch(error){remember(error);}},abort=()=>fail(control.signal.reason);
    try{
      daemon=Object.freeze({child,kill:stop});
      control.signal.addEventListener("abort",abort,{once:true});if(control.signal.aborted)abort();
      const observe=(chunk:Buffer,stream:"stdout"|"stderr")=>{try{bytes+=chunk.length;for(const byte of chunk){if(!lineOpen[stream])lines++;lineOpen[stream]=byte!==10;}if(bytes>limits.maximumBytes||lines>limits.maximumLines)throw Error("GIS Shell child output grant exceeded");childOutput=(childOutput+chunk.toString("utf8")).slice(-65_536);}catch(error){fail(error);}};
      child.stdout?.on("data",chunk=>observe(chunk,"stdout"));child.stderr?.on("data",chunk=>observe(chunk,"stderr"));
      const progress=async()=>{if(closed)return;try{checkResourceLeaseControl(control);await control.publish({resource:"gis-shell:serve",mode:"shared",elapsedMs:performance.now()-childStarted});await control.yieldContinuation();checkResourceLeaseControl(control);if(!closed)timer=setTimeout(()=>{progressWork=progress();},Math.min(1000,checkResourceLeaseControl(control)??1000));}catch(error){fail(error);}};
      timer=setTimeout(()=>{progressWork=progress();},Math.min(1000,checkResourceLeaseControl(control)??1000));startedResolve();
      const status=await terminal;
      if(failed)throw failure;checkResourceLeaseControl(control);if(!stopping||status.code!==0&&status.signal!=="SIGTERM"&&status.signal!=="SIGKILL")throw Error("GIS Shell child closed unexpectedly");
    }catch(error){remember(error);}finally{
      closed=true;clearTimeout(timer);
      try{if(child.exitCode===null&&child.signalCode===null)terminateOwnedChildTree(child);}catch(error){remember(error);}
      try{await terminal;}catch(error){remember(error);}
      try{await progressWork;}catch(error){remember(error);}
      try{control.signal.removeEventListener("abort",abort);}catch(error){remember(error);}
    }
    if(failed)throw failure;
  });
  void lifetime.catch(startedReject);try{await started;}catch(primary){await lifetime.catch(()=>{});await relay.stop().catch(()=>{});throw primary;}if(!daemon)throw Error("Actual GIS Shell child birth required");
  const ownedDaemon=daemon;
  const diagnostics: string[] = [];
  let context: import("playwright").BrowserContext | undefined;
  let page: import("playwright").Page | undefined;
  let mountedScope: Readonly<{ spaceId: string; documentId: string }> | undefined;
  let stopPromise: Promise<void> | undefined;
  const stop = (): Promise<void> => {
    stopPromise ??= (async () => {
      const errors: unknown[] = [];
      await context?.close().catch((error: unknown) => errors.push(error));
      ownedDaemon.kill();
      await lifetime.catch((error:unknown)=>errors.push(error));
      await relay.stop().catch((error: unknown) => errors.push(error));
      if (errors.length) throw new AggregateError(errors, "GIS Map Shell peer failed to stop");
    })();
    return stopPromise;
  };
  try {
    await waitForUiReadiness(uiOrigin, ownedDaemon.child);
    context = await options.browser.newContext();
    page = await context.newPage();
    page.on("console", (message) => {
      if (message.type() === "error") diagnostics.push("console:" + message.text());
    });
    page.on("pageerror", (error) => diagnostics.push("pageerror:" + error.message));
    const proof = relay.takeBrowserBootstrapProof();
    const proofHex = proof.toString("hex");
    proof.fill(0);
    await page.goto(uiOrigin + "/#semio-broker=" + proofHex, { waitUntil: "domcontentloaded", timeout: 30_000 });
    await page.locator("[data-semio-os-ready]").waitFor({ state: "attached", timeout: 120_000 });
    await page.waitForFunction(
      () => {
        const probe = (globalThis as any).__semioOsCatalogProbe;
        return probe?.ready === true && probe.plugins?.some((plugin: any) => plugin.pluginId === "s" && plugin.status === "loaded");
      },
      undefined,
      { timeout: 120_000 },
    );
    if (await page.locator("html").getAttribute("lang") !== options.locale) throw new Error("GIS Map Shell locale did not bind");
    const spaceRow = page.locator("[data-row-id=" + JSON.stringify("space:" + options.scope.spaceId) + "]");
    await spaceRow.waitFor({ state: "visible", timeout: 120_000 });
    await spaceRow.getByRole("button", { name: options.locale === "de" ? "Studio öffnen" : "Open Studio", exact: true }).click();
    await page.waitForFunction((spaceId) => decodeURIComponent(location.pathname) === "/spaces/" + spaceId, options.scope.spaceId, { timeout: 30_000 });
    if (options.creation !== undefined) {
      await page.locator("#s-space-create-artifact").click();
      const dialog = page.locator('[data-slot="dialog-box"]');
      await dialog.waitFor({ state: "visible", timeout: 30_000 });
      await dialog.locator("#name").fill(options.creation.name);
      await dialog.locator("#kindChoice").click();
      const choices = page.locator('[role="option"][data-value]');
      const choiceIndex = await choices.evaluateAll((nodes, kindId) => nodes.findIndex((node) => {
        try { return JSON.parse(node.getAttribute("data-value") ?? "null")?.kindId === kindId; } catch { return false; }
      }), options.creation.kindId);
      if (choiceIndex < 0) throw new Error("GIS Map creation kind is absent from the ordinary dialog");
      await choices.nth(choiceIndex).click();
      await dialog.locator('[id="ui.dialog.submit"]').click();
      const artifactRows = page.locator('[data-row-id^="artifact:artifact-"]');
      const deadlineAtMs = Date.now() + 120_000;
      while (mountedScope === undefined && Date.now() < deadlineAtMs) {
        for (let index = 0; index < await artifactRows.count(); index++) {
          const row = artifactRows.nth(index);
          if (!(await row.isVisible()) || !(await row.innerText()).includes(options.creation.name)) continue;
          const rowId = await row.getAttribute("data-row-id");
          const documentId = rowId?.slice("artifact:".length);
          if (documentId && /^artifact-[0-9a-f]{32}$/u.test(documentId)) mountedScope = Object.freeze({ spaceId: options.scope.spaceId, documentId });
        }
        if (mountedScope === undefined) await Bun.sleep(50);
      }
      if (mountedScope === undefined) throw new Error("GIS Map creation did not publish one Directory-derived row");
    } else {
      mountedScope = Object.freeze({ spaceId: options.scope.spaceId, documentId: existingDocumentId! });
      const artifactRow = page.locator("[data-row-id=" + JSON.stringify("artifact:" + mountedScope.documentId) + "]");
      await artifactRow.waitFor({ state: "visible", timeout: 120_000 });
      await artifactRow.getByRole("button", { name: options.locale === "de" ? "Öffnen" : "Open", exact: true }).click();
    }
    await page.waitForFunction(
      (scope) => typeof (globalThis as any).__semioMountedGisMapProbe === "function" && (globalThis as any).__semioMountedGisMapProbe(scope.spaceId, scope.documentId) !== null,
      mountedScope,
      { timeout: 120_000 },
    );
  } catch (error) {
    await stop().catch(() => undefined);
    const detail = [...diagnostics.slice(-12), childOutput].join("\n").slice(-12_000);
    throw new Error("GIS Map Shell peer failed to mount: " + (error instanceof Error ? error.message : "unknown") + "\n" + detail);
  }
  const livePage = page;
  const liveScope = mountedScope;
  if (!livePage || liveScope === undefined) throw new Error("GIS Map Shell peer lost its mounted page owner");
  return Object.freeze({
    scope: liveScope,
    async readMap(): Promise<GisMapMountedShellProbeV1 | null> {
      const value = await livePage.evaluate(
        (scope) => (globalThis as any).__semioMountedGisMapProbe?.(scope.spaceId, scope.documentId) ?? null,
        liveScope,
      );
      return parseGisMapMountedShellProbeV1(value, liveScope);
    },
    async propose(): Promise<string> {
      const canvas = livePage.locator(".semio-tiled-map-host canvas");
      await canvas.waitFor({ state: "visible", timeout: 30_000 });
      await canvas.click({ button: "right" });
      await livePage.getByRole("menuitem", { name: "Propose Bounds Region", exact: true }).click();
      const port = livePage.locator('[data-semio-inference-port="offered"]');
      await port.waitFor({ state: "visible", timeout: 120_000 });
      const regionId = await port.locator("[data-semio-inference-preview]").getAttribute("data-semio-inference-preview");
      if (regionId === null || !/^inference-[0-9a-f]{32}$/u.test(regionId)) throw new Error("GIS Map Shell proposal exposed an invalid public preview");
      return regionId;
    },
    async approve(): Promise<void> {
      const port = livePage.locator('[data-semio-inference-port="offered"]');
      await port.getByRole("button", { name: options.locale === "de" ? "Vorschlag freigeben" : "Approve proposal", exact: true }).click();
    },
    async undo(): Promise<void> {
      await livePage.getByRole("tab", { name: options.locale === "de" ? "Verlauf" : "History", exact: true }).click();
      const undo = livePage.getByRole("button", { name: "Undo", exact: true });
      await undo.waitFor({ state: "visible", timeout: 30_000 });
      if (await undo.isDisabled()) throw new Error("GIS Map Shell private durable Undo is unavailable");
      await undo.click();
    },
    async redo(): Promise<void> {
      await livePage.getByRole("tab", { name: options.locale === "de" ? "Verlauf" : "History", exact: true }).click();
      const redo = livePage.getByRole("button", { name: "Redo", exact: true });
      await redo.waitFor({ state: "visible", timeout: 30_000 });
      if (await redo.isDisabled()) throw new Error("GIS Map Shell durable collaborative Redo is unavailable after hub restart");
      await redo.click();
    },
    stop,
  });
}

type GisMapTwoAuthorPreparedV1 = Readonly<{
  artifactRoot: string;
  dataRoot: string;
  receipt: TrustedBootstrapMaterializationV1;
  current: TrustedBootstrapPublishedV1;
  binaryPath: string;
  mcpBinaryPath: string;
}>;

/** 🔏️ Revalidates the prepared selected-current bytes without publishing or creating another root. */
function assertGisMapCompositionCurrent(prepared:GisMapTwoAuthorPreparedV1,dataRoot:string):void{
  if(!isAbsolute(prepared.artifactRoot)||prepared.dataRoot!==dataRoot)throw Error("GIS Map composition requires its exact retained Host data root");
  const current = trustedBootstrapCurrent(prepared.dataRoot);
  if (!current || JSON.stringify(current) !== JSON.stringify(prepared.current)) throw new Error("GIS Map composition current pointer changed");
  for (const key of ["profileId", "generationId", "bundleSha256", "bundlePath"] as const) if (current[key] !== prepared.receipt[key]) throw new Error(`GIS Map composition receipt changed at ${key}`);
  trustedBootstrapVerifyPublication(prepared.dataRoot, prepared.receipt);
}

/** 🧑‍🤝‍🧑️ One current selection, two ordinary Shell peers, private MCP cancellation and durable restart. */
export async function proveGisMapTwoAuthorShellProcess(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,prepared:GisMapTwoAuthorPreparedV1):Promise<void>{
 return withLocalHubHostTaskV1(host,phaseControl,async context=>{
 const {invocation,workspace:repoRoot,root:hubRoot,dataRoot}=context;
  assertGisMapCompositionCurrent(prepared,dataRoot);
  const assert: (typeof import("node:assert"))["strict"] = (await import("node:assert/strict")).default;
  const { chromium } = await import("playwright");
  const bundle = trustedBootstrapReadCurrentBundle(prepared.current, () => {});
  const selected = bundle.packages.find((row: any) => row.pluginId === "gis");
  const profile = bundle.profiles.find((row: any) => row.id === prepared.current.profileId);
  const target = profile?.openTargets?.find((row: any) => row?.target?.role === "editor")?.target;
  assert.deepEqual(bundle.packages.map((row: any) => row.pluginId).sort(), ["gis", "stdio"]);
  assert.equal(selected?.executionProtocol?.appChannelVersion, DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1);
  assert.equal(target?.surfaceId, "s.gis.gismap@1/*#editor");
  assert.equal(target?.artifactKind, "s.gis.gismap");
  assert.equal(selected?.component?.path, "packages/gis/component.wasm");
  assert.equal(selected?.descriptor?.path, "packages/gis/descriptor.semio");
  const generationRoot = dirname(prepared.current.bundlePath);
  return withTestBrowserHostV1({
    invocation,
    control:context.control,
    artifactRoot: prepared.artifactRoot,
    selectedGis: {
      generationId: prepared.current.generationId,
      currentSha256: prepared.current.currentSha256,
      componentPath: join(generationRoot, selected.component.path),
      componentSha256: selected.component.sha256,
      descriptorPath: join(generationRoot, selected.descriptor.path),
      descriptorSha256: selected.descriptor.sha256,
    },
  },async (browserHost,browserPhase)=>withLocalHubHostTaskV1(host,browserPhase,async()=>{
  assert.equal(browserHost.receipt.selectedGis.generationId, prepared.current.generationId);
  assert.equal(browserHost.receipt.selectedGis.currentSha256, prepared.current.currentSha256);
  assert.equal(browserHost.receipt.selectedGis.componentSha256, selected.component.sha256);
  assert.equal(browserHost.receipt.selectedGis.descriptorSha256, selected.descriptor.sha256);
  assert.equal(browserHost.receipt.selectedGis.stagedDescriptorSha256, selected.descriptor.sha256);
  assert.match(browserHost.receipt.selectedGis.bridgeSha256, /^[0-9a-f]{64}$/u);
  assert.match(browserHost.receipt.selectedGis.materializationReceiptSha256, /^[0-9a-f]{64}$/u);
  assert.ok(browserHost.receipt.selectedGis.generatedCores.length > 0);
  const profiles: readonly LocalProfile[] = [
    { profileId: "gis-map-shell-a", subject: "gis-map-shell-author-a", displayName: "GIS Map Author A", allowedClientClasses: ["native", "mcp", "react-relay"] },
    { profileId: "gis-map-shell-b", subject: "gis-map-shell-author-b", displayName: "GIS Map Author B", allowedClientClasses: ["native", "mcp", "react-relay"] },
  ];
  const locales = ["en", "de"] as const;
  let browser: Awaited<ReturnType<typeof chromium.launch>> | undefined;
  let run: LocalHubRun | undefined;
  let nonce = 2;
  const authors: Record<string, any>[] = [];
  const clients: GisMapProcessMcpClientV1[] = [];
  const sockets: GisMapProcessDocumentSocketV1[] = [];
  const shells: Awaited<ReturnType<typeof startGisMapShellPeerV1>>[] = [];
  let completedReceipt: Record<string, unknown> | undefined;
  const closePeers = async (): Promise<void> => {
    const errors: unknown[] = [];
    for (const shell of shells.splice(0).reverse()) await shell.stop().catch((error: unknown) => errors.push(error));
    for (const socket of sockets.splice(0)) await finishGisMapProcessDocumentSocket(socket).catch((error) => errors.push(error));
    for (const client of clients.splice(0).reverse()) await finishGisMapProcessMcpClient(client).catch((error) => errors.push(error));
    for (const author of authors.splice(0)) author.capability = "";
    if (errors.length) throw new AggregateError(errors, "two-author Shell peers failed to close");
  };
  try {
    browser = await chromium.launch({ headless: true });
    run = await startLocalHub(host,profiles,{capture:true,isolatedSecuritySmoke:true,inferenceCheckpointControl:true});
    const ready = await waitForReadiness(run);
    assert.deepEqual(ready.features?.inferenceServices, [{ serviceId: "s.gis.gismap.inference", route: "inference/gis-map" }]);
    assert.equal(ready.artifactAuthority?.ready, true);
    for (const profile of profiles) authors.push(await issueLocalCredential(run, profile.profileId, "native", nonce++));
    const userB = await liveDirectoryEventPageUser(run, authors[1]!);
    const created = await postLiveDirectoryCommand(run, authors[0]!.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "Verified Two-Author Shell Map", spaceKind: "studio", visibility: "private" });
    assert.equal(created.status, 202);
    const spaceId = JSON.parse(created.text).events?.find((event: any) => event.body?.kind === "space.created")?.body?.spaceId;
    assert.equal(typeof spaceId, "string");
    assert.equal(typeof userB.email, "string");
    const admitted = await postLiveDirectoryCommand(run, authors[0]!.capability, liveDirectoryCommandRequestId(), { kind: "upsert-member", spaceId, email: userB.email, role: "author" });
    assert.equal(admitted.status, 202);
    const creationCredential = await issueLocalCredential(run, profiles[0]!.profileId, "react-relay", nonce++);
    shells.push(await startGisMapShellPeerV1({
      repoRoot,invocation,control:browserPhase,
      run,
      profileId: profiles[0]!.profileId,
      scope: { spaceId: String(spaceId) },
      creation: { kindId: target.artifactKind, name: "Verified GIS Map" },
      locale: locales[0],
      artifactRoot: prepared.artifactRoot,
      browserHost,
      browser,
      credentialEnvelope: creationCredential,
    }));
    const scope = shells[0]!.scope;
    assert.equal(scope.spaceId, spaceId);
    assert.match(scope.documentId, /^artifact-[0-9a-f]{32}$/u);
    const fixture: GisMapDocumentOpenFixtureV1 = Object.freeze({
      generationId: prepared.current.generationId,
      documentId: scope.documentId,
      package: { pluginId: selected.pluginId, packageId: selected.packageId, version: selected.version, componentSha256: selected.component.sha256 },
      artifact: { kind: target.artifactKind, schema: target.artifactSchema, packSchemaHash: target.packSchemaHash },
      surfaceId: target.surfaceId,
    });
    const openPeers = async (reopenBothShells: boolean): Promise<void> => {
      assertGisMapCompositionCurrent(prepared,dataRoot);
      for (let index = 0; index < profiles.length; index++) {
        const profile = profiles[index]!;
        const envelope = await issueLocalCredential(run!, profile.profileId, "mcp", nonce++);
        try { clients.push(await startGisMapProcessMcpClient(repoRoot, run!, envelope, scope.spaceId, scope.documentId, profile.profileId, prepared.mcpBinaryPath)); } finally { envelope.capability = ""; }
        sockets.push(await openGisMapProcessDocumentSocket(run!, authors[index]!.capability, scope.spaceId, fixture, `socket-${profile.profileId}`));
      }
      for (let index = reopenBothShells ? 0 : 1; index < profiles.length; index++) {
        const credentialEnvelope = await issueLocalCredential(run!, profiles[index]!.profileId, "react-relay", nonce++);
        shells.push(await startGisMapShellPeerV1({ repoRoot,invocation,control:browserPhase, run: run!, profileId: profiles[index]!.profileId, scope, locale: locales[index]!, artifactRoot: prepared.artifactRoot, browserHost, browser: browser!, credentialEnvelope }));
      }
    };
    const readPair = async (): Promise<GisMapCheckpointProjectionV1> => {
      const pairs = await Promise.all(clients.map((client) => readGisMapProcessCheckpoint(client, scope.spaceId, scope.documentId)));
      assert.equal(pairs.length, 2); assert.deepEqual(pairs[0], pairs[1]); return pairs[0]!;
    };
    const waitMaps = async (expectedPair: GisMapCheckpointProjectionV1, expectedRegions?: readonly string[]): Promise<readonly GisMapMountedShellProbeV1[]> => {
      const deadline = Date.now() + 120_000;
      while (Date.now() < deadline) {
        const maps = await Promise.all(shells.map((shell) => shell.readMap()));
        if (maps.length === 2 && maps.every((map) => map !== null)) {
          for (const map of maps) {
            assert.deepEqual(map!.scope, scope); assert.equal(map!.rootKind, "tiled-map");
            assert.equal(map!.catalogGenerationId, prepared.current.generationId);
            assert.equal(map!.componentSha256, selected.component.sha256); assert.equal(map!.descriptorSha256, selected.descriptor.sha256);
            assert.equal(map!.browserActorSha256, selected.browserActor.sha256); assert.ok(map!.uiRevision > 0);
            assert.equal(map!.activeCheckpointId, expectedPair.activeCheckpointId);
            assert.equal(map!.descriptorDigestV1, expectedPair.descriptorDigestV1);
            assert.deepEqual(map!.frontier, expectedPair.frontier);
          }
          assert.notEqual(maps[0]!.clientInstanceId, maps[1]!.clientInstanceId);
          const regions = JSON.stringify(maps[0]!.regionIds);
          if (regions === JSON.stringify(maps[1]!.regionIds) && (expectedRegions === undefined || regions === JSON.stringify(expectedRegions))) return maps as readonly GisMapMountedShellProbeV1[];
        }
        await Bun.sleep(50);
      }
      throw new Error("two actual acknowledged Shell maps did not reach the exact scoped regions");
    };
    const changedPair = async (before: GisMapCheckpointProjectionV1, offsets: readonly number[]): Promise<GisMapCheckpointProjectionV1> => {
      const controls = await Promise.all(sockets.map((socket, index) => waitForDocumentSocketFrame(socket.socket, socket.frames, (frame) => frame.RebootstrapRequired?.control, `Shell peer ${index} durable change`, offsets[index])));
      assert.deepEqual(controls[0], controls[1]);
      for (const socket of sockets) if (socket.error) throw socket.error;
      assert.equal(controls[0].space_id, scope.spaceId); assert.equal(controls[0].document_id, scope.documentId);
      const pair = await readPair();
      assert.notEqual(pair.activeCheckpointId, before.activeCheckpointId);
      assert.equal(pair.activeCheckpointId, Buffer.from(controls[0].checkpoint_id).toString("hex"));
      assert.equal(pair.descriptorDigestV1, Buffer.from(controls[0].descriptor_hash).toString("hex"));
      assert.equal(pair.frontier.headEditId, controls[0].baseline_frontier?.head_edit_id);
      assertGisMapCompositionCurrent(prepared,dataRoot);
      return pair;
    };
    await openPeers(false);
    const initial = await readPair();
    assert.equal(artifactFrontierIsGenesisForV1(scope, initial.frontier), true);
    const initialMaps = await waitMaps(initial);
    const initialRegions = [...initialMaps[0]!.regionIds];
    const cancelledJobId = await cancelGisMapProcessMcpOwner(run, clients[1]!, scope.documentId);
    const cancelledProgress = run.inferenceCheckpointProgress;
    assert.ok(cancelledProgress && cancelledProgress.progressCursor >= 1 && cancelledProgress.completed >= 1 && cancelledProgress.completed <= cancelledProgress.total);
    assert.deepEqual(await readPair(), initial);
    const regionId = await shells[0]!.propose();
    assert.match(regionId, /^inference-[0-9a-f]{32}$/u);
    assert.ok(!initialRegions.includes(regionId));
    const jobId = regionId.slice("inference-".length);
    const inferenceRoot = `/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}/inference/gis-map/jobs`;
    for (const [method, suffix, body] of [
      ["GET", `/${jobId}/events?after=0`, undefined], ["POST", `/${jobId}/cancel`, undefined],
      ["POST", `/${jobId}/approval`, { schema: "semio.hub.inference-approval/v1", version: 1, jobId, proposalHash: "9".repeat(64) }],
    ] as const) {
      const denied = await gisMapProcessHttp(run, authors[1]!.capability, method, `${inferenceRoot}${suffix}`, body);
      assert.equal(denied.status, 403); assert.deepEqual(denied.value, { schema: "semio.hub.inference-error/v1", code: "inference.denied" });
    }
    const approvalOffsets = sockets.map((socket) => socket.frames.length);
    await shells[0]!.approve();
    const applied = await changedPair(initial, approvalOffsets);
    const appliedMaps = await waitMaps(applied, [...initialRegions, regionId].sort());
    const undoOffsets = sockets.map((socket) => socket.frames.length);
    await shells[0]!.undo();
    const undone = await changedPair(applied, undoOffsets);
    assert.equal(undone.packSha256, initial.packSha256);
    assert.notEqual(undone.frontier.headEditId, applied.frontier.headEditId);
    const undoneMaps = await waitMaps(undone, initialRegions);
    const port = run.port, priorPid = run.child.pid;
    await closePeers();
    await finishLocalHub(run);
    assertGisMapCompositionCurrent(prepared,dataRoot);
    run = await startLocalHub(host,profiles,{port,capture:true,isolatedSecuritySmoke:true});
    await waitForReadiness(run);
    assert.notEqual(run.child.pid, priorPid);
    for (const profile of profiles) authors.push(await issueLocalCredential(run, profile.profileId, "native", nonce++));
    await openPeers(true);
    assert.deepEqual(await readPair(), undone);
    const restartedMaps = await waitMaps(undone, initialRegions);
    const redoOffsets = sockets.map((socket) => socket.frames.length);
    await shells[0]!.redo();
    const redone = await changedPair(undone, redoOffsets);
    assert.equal(redone.packSha256, applied.packSha256);
    assert.notEqual(redone.frontier.headEditId, undone.frontier.headEditId);
    const redoneMaps = await waitMaps(redone, [...initialRegions, regionId].sort());
    assertGisMapCompositionCurrent(prepared,dataRoot);
    completedReceipt = { schema: "semio.hub.gis-map-two-author-shell-receipt/v1", current: prepared.current, browserHost: browserHost.receipt, scope, locales, cancelledJobId, cancelledProgress, regionId, initial, applied, undone, redone, initialMaps, appliedMaps, undoneMaps, restartedMaps, redoneMaps, ownerPrivateJobDenials: 3, rebootstrapControls: 5, actualMapObservations: 10, restartedProcess: true, observations: ["durable-collaborative-redo"], nonclaims: ["external-model-provider", "browser-qualified-current-publication", "private-job-restart-recovery", "wgpu-rendering"] };
  } finally {
    const errors: unknown[] = [];
    await closePeers().catch((error) => errors.push(error));
    await browser?.close().catch((error) => errors.push(error));
    if (run) await finishLocalHub(run).catch((error) => errors.push(error));
    if (errors.length) throw new AggregateError(errors, "two-author Shell composition cleanup failed");
  }
  if (!completedReceipt) throw new Error("two-author Shell composition has no completed observation");
  trustedBootstrapWriteNew(join(prepared.artifactRoot, `two-author-shell-${randomBytes(8).toString("hex")}.json`), Buffer.from(`${JSON.stringify(completedReceipt, null, 2)}\n`), () => {});
  console.log(`two-author Shell Map: generation=${prepared.current.generationId} real-store-observations=8 private-job-denials=3 MCP-cancel=1 ordinary-approval=1 ordinary-undo=1 same-root-restart=1`);
  }));
 });
}

/** 🤝️ Drives the bounded two-Author Hub/MCP proposal and durable server-owned undo journey. */
async function proveGisMapProposalProcess(host:LocalHubHostV1,phaseControl:CompilerAdmissionControl,payload:CheckInProcessFixtureDataV1,mcpBinary:string):Promise<void>{
 return withLocalHubHostTaskV1(host,phaseControl,async context=>{
 const {workspace:repoRoot,root:hubRoot,dataRoot,binaryPath}=context;
  const artifactPath=context.diagnosticsRoot;
  return await withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(TRUSTED_BOOTSTRAP_LINKED_PACKAGES),async(materialized,actualPhase)=>{
  await validateAndPublishTrustedStdioGisCandidate(host,actualPhase,materialized);
  const current = trustedBootstrapCurrent(dataRoot);
  if (!current || current.generationId !== materialized.generationId) throw new Error("GIS Map proposal process did not retain its verified current generation");
  const bundle = trustedBootstrapReadCurrentBundle(current, () => {});
  const profile = bundle.profiles?.find((row: any) => row?.id === current.profileId);
  const selected = bundle.packages?.find((row: any) => row?.pluginId === "gis");
  const target = profile?.openTargets?.find((row: any) => row?.target?.role === "editor")?.target;
  if (
    !profile ||
    !selected ||
    !target ||
    selected.packageId !== "semio:gis" ||
    selected.executionProtocol?.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 ||
    target.artifactKind !== "s.gis.gismap" ||
    target.artifactSchema !== "gis.map" ||
    target.surfaceId !== "s.gis.gismap@1/*#editor"
  ) {
    throw new Error("GIS Map proposal process current does not select the exact compiled v14 Map target");
  }
  let fixture: CheckInProcessFixtureV1 = Object.freeze({
    ...payload.fixture,
    profileId: current.profileId,
    generationId: current.generationId,
    documentId: `gis-map-collaboration-${randomBytes(8).toString("hex")}`,
    package: { pluginId: selected.pluginId, packageId: selected.packageId, version: selected.version, componentSha256: selected.component.sha256 },
    artifact: { kind: target.artifactKind, schema: target.artifactSchema, packSchemaHash: target.packSchemaHash },
    surfaceId: target.surfaceId,
  });
  const profiles: readonly LocalProfile[] = [
    { profileId: "gis-map-author-a", subject: "gis-map-process-author-a", displayName: "GIS Map Author A", allowedClientClasses: ["native", "mcp"] },
    { profileId: "gis-map-author-b", subject: "gis-map-process-author-b", displayName: "GIS Map Author B", allowedClientClasses: ["native", "mcp"] },
  ];
  const run = await startLocalHub(host,profiles,{capture:true,isolatedSecuritySmoke:true,inferenceCheckpointControl:true});
  let authorA: Record<string, any> | undefined;
  let authorB: Record<string, any> | undefined;
  let mcpA: GisMapProcessMcpClientV1 | undefined;
  let mcpB: GisMapProcessMcpClientV1 | undefined;
  let socketA: GisMapProcessDocumentSocketV1 | undefined;
  let socketB: GisMapProcessDocumentSocketV1 | undefined;
  try {
    const readiness = await waitForReadiness(run);
    if (!readiness.features?.inferenceServices?.some((service: { serviceId: string }) => service.serviceId === "s.gis.gismap.inference") || readiness.artifactAuthority?.ready !== true) throw new Error("GIS Map proposal process Hub did not publish its verified inference authority");
    authorA = await issueLocalCredential(run, profiles[0]!.profileId, "native", 2);
    authorB = await issueLocalCredential(run, profiles[1]!.profileId, "native", 3);
    const userB = await liveDirectoryEventPageUser(run, authorB);
    const created = await postLiveDirectoryCommand(run, authorA.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "Genuine GIS Map Collaboration", spaceKind: "studio", visibility: "private" });
    const createdBody = created.status === 202 ? (JSON.parse(created.text) as Record<string, any>) : undefined;
    const spaceId = createdBody?.events?.find((event: any) => event?.body?.kind === "space.created")?.body?.spaceId;
    if (typeof spaceId !== "string" || spaceId.length === 0 || typeof userB.email !== "string") throw new Error("GIS Map proposal process could not create its private two-Author space");
    const promoted = await postLiveDirectoryCommand(run, authorA.capability, liveDirectoryCommandRequestId(), { kind: "upsert-member", spaceId, email: userB.email, role: "author" });
    if (promoted.status !== 202) throw new Error(`GIS Map proposal process could not admit Author B: ${promoted.status}`);
    fixture = await createCheckInProcessGenesis(run, authorA.capability, spaceId, fixture);
    const { frontier } = await commitCheckInProcessEdit(run, authorA.capability, spaceId, fixture, payload.envelopes);
    const initialCheckIn = await checkInProcessHeadV1(run, authorA.capability, spaceId, fixture.documentId, frontier);

    const mcpEnvelopeA = await issueLocalCredential(run, profiles[0]!.profileId, "mcp", 4);
    const mcpEnvelopeB = await issueLocalCredential(run, profiles[1]!.profileId, "mcp", 5);
    mcpA = await startGisMapProcessMcpClient(repoRoot, run, mcpEnvelopeA, spaceId, fixture.documentId, "gis-map-author-a",mcpBinary);
    mcpB = await startGisMapProcessMcpClient(repoRoot, run, mcpEnvelopeB, spaceId, fixture.documentId, "gis-map-author-b",mcpBinary);
    socketA = await openGisMapProcessDocumentSocket(run, authorA.capability, spaceId, fixture, "gis-map-author-a");
    socketB = await openGisMapProcessDocumentSocket(run, authorB.capability, spaceId, fixture, "gis-map-author-b");
    const initialA = await readGisMapProcessCheckpoint(mcpA, spaceId, fixture.documentId);
    const initialB = await readGisMapProcessCheckpoint(mcpB, spaceId, fixture.documentId);
    if (JSON.stringify(initialA) !== JSON.stringify(initialB) || initialA.activeCheckpointId !== initialCheckIn.ready?.checkpointId) {
      throw new Error("GIS Map process Authors did not open the exact same initial canonical pair");
    }

    const inferenceRoot = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.documentId)}/inference/gis-map/jobs`;
    const intent = (requestId: string): Record<string, unknown> => ({ schema: "semio.hub.inference-request/v1", version: 1, requestId, serviceId: "s.gis.gismap.inference", policyVersion: 1, lifetimeMs: 120_000 });
    const jobIdB = await cancelGisMapProcessMcpOwner(run, mcpB, fixture.documentId);
    const submittedA = await gisMapProcessHttp(run, authorA.capability, "POST", inferenceRoot, intent(randomBytes(16).toString("hex")));
    const jobId = submittedA.value?.jobId;
    if (submittedA.status !== 200 || submittedA.value?.state !== "running" || submittedA.value?.proposalState !== "none" || submittedA.value?.proposalHash !== null || !/^[0-9a-f]{32}$/u.test(jobId ?? "")) {
      throw new Error(`GIS Map Author A did not receive one retained Running job: ${submittedA.status}`);
    }
    for (const [method, suffix, body] of [
      ["GET", `/${jobId}/events?after=0`, undefined],
      ["POST", `/${jobId}/cancel`, undefined],
      ["POST", `/${jobId}/approval`, { schema: "semio.hub.inference-approval/v1", version: 1, jobId, proposalHash: "9".repeat(64) }],
    ] as const) {
      const denied = await gisMapProcessHttp(run, authorB.capability, method, `${inferenceRoot}${suffix}`, body);
      if (denied.status !== 403 || denied.value?.schema !== "semio.hub.inference-error/v1" || denied.value?.code !== "inference.denied" || Object.keys(denied.value).length !== 2 || denied.text.includes(jobId)) {
        throw new Error(`GIS Map Author B reached Author A's private job through ${method} ${suffix}`);
      }
    }
    const eventsA = await waitForGisMapProcessInference(run, authorA.capability, `${inferenceRoot}/${jobId}/events?after=0`, "succeeded");
    const proposalHash = eventsA.value?.proposalHash;
    if (
      eventsA.status !== 200 ||
      eventsA.value?.proposalState !== "offered" ||
      !/^[0-9a-f]{64}$/u.test(proposalHash ?? "") ||
      eventsA.value?.preview?.schema !== "semio.hub.gis-map-inference-preview/v1" ||
      eventsA.value?.preview?.jobId !== jobId ||
      eventsA.value?.preview?.proposalHash !== proposalHash ||
      eventsA.value?.preview?.regionId !== `inference-${jobId}`
    ) {
      throw new Error("GIS Map Author A did not receive the server-stamped CreateRegion preview");
    }
    const approvalBody = { schema: "semio.hub.inference-approval/v1", version: 1, jobId, proposalHash };
    const approved = await gisMapProcessHttp(run, authorA.capability, "POST", `${inferenceRoot}/${jobId}/approval`, approvalBody);
    const mutationId = approved.value?.mutationId;
    const undoHandle = approved.value?.undo;
    if (
      approved.status !== 200 ||
      approved.value?.schema !== "semio.hub.inference-approval-receipt/v1" ||
      approved.value?.applied !== true ||
      !/^[0-9a-f]{32}$/u.test(mutationId ?? "") ||
      !/^[0-9a-f]{32}$/u.test(undoHandle?.targetId ?? "") ||
      undoHandle?.expectedCurrent?.documentId !== fixture.documentId
    ) {
      throw new Error(`GIS Map Author A approval did not reach a committed public checkpoint: ${approved.status}`);
    }
    const controlA = await waitForDocumentSocketFrame(socketA.socket, socketA.frames, (frame) => frame.RebootstrapRequired?.control, "Author A RebootstrapRequired");
    const controlB = await waitForDocumentSocketFrame(socketB.socket, socketB.frames, (frame) => frame.RebootstrapRequired?.control, "Author B RebootstrapRequired");
    if (socketA.error || socketB.error) throw socketA.error ?? socketB.error;
    if (
      JSON.stringify(controlA) !== JSON.stringify(controlB) ||
      controlA.space_id !== spaceId ||
      controlA.document_id !== fixture.documentId ||
      controlA.baseline_frontier?.head_edit_id !== mutationId ||
      Buffer.from(controlA.checkpoint_id ?? []).byteLength !== 32 ||
      Buffer.from(controlA.descriptor_hash ?? []).byteLength !== 32
    ) {
      throw new Error("GIS Map Authors did not receive one identical approval-bound RebootstrapRequired control");
    }
    const refreshedA = await readGisMapProcessCheckpoint(mcpA, spaceId, fixture.documentId);
    const refreshedB = await readGisMapProcessCheckpoint(mcpB, spaceId, fixture.documentId);
    const controlCheckpoint = Buffer.from(controlA.checkpoint_id).toString("hex");
    const controlDescriptor = Buffer.from(controlA.descriptor_hash).toString("hex");
    if (
      JSON.stringify(refreshedA) !== JSON.stringify(refreshedB) ||
      refreshedA.activeCheckpointId !== controlCheckpoint ||
      refreshedA.descriptorDigestV1 !== controlDescriptor ||
      refreshedA.frontier?.headEditId !== mutationId ||
      refreshedA.activeCheckpointId === initialA.activeCheckpointId ||
      refreshedA.pairSha256 === initialA.pairSha256 ||
      JSON.stringify(trustedBootstrapCurrent(dataRoot)) !== JSON.stringify(current)
    ) {
      throw new Error("GIS Map MCP Authors did not refresh to the same approval-bound canonical pair under the retained generation");
    }
    if (JSON.stringify(undoHandle.expectedCurrent) !== JSON.stringify(refreshedA.frontier)) throw new Error("GIS Map approval undo handle did not retain the exact published frontier");
    const terminalA = await gisMapProcessHttp(run, authorA.capability, "GET", `${inferenceRoot}/${jobId}/events?after=0`);
    const kinds = terminalA.value?.events?.map((event: any) => event?.kind);
    if (terminalA.status !== 200 || terminalA.value?.proposalState !== "approved" || !Array.isArray(kinds) || !kinds.includes("approved")) throw new Error("GIS Map approval ledger did not expose its exact owner-private terminal event");

    const undoRoot = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.documentId)}/inference/gis-map/approval-undos`;
    const undoBody = {
      schema: "semio.hub.gis-map-approval-undo/v1",
      version: 1,
      targetId: undoHandle.targetId,
      idempotencyKey: randomBytes(16).toString("hex"),
      expectedCurrent: structuredClone(undoHandle.expectedCurrent),
    };
    const peerUndo = await gisMapProcessHttp(run, authorB.capability, "POST", undoRoot, undoBody);
    if (peerUndo.status !== 403 || peerUndo.value?.code !== "inference.denied" || peerUndo.text.includes(undoBody.targetId)) throw new Error("GIS Map Author B reached Author A's private durable undo target");
    const staleUndo = await gisMapProcessHttp(run, authorA.capability, "POST", undoRoot, { ...undoBody, idempotencyKey: randomBytes(16).toString("hex"), expectedCurrent: structuredClone(initialA.frontier) });
    if (staleUndo.status !== 409 || staleUndo.value?.code !== "inference.conflict") throw new Error("GIS Map stale durable undo reached Store/WAL publication");
    const undoFrameStartA = socketA.frames.length;
    const undoFrameStartB = socketB.frames.length;
    const undone = await gisMapProcessHttp(run, authorA.capability, "POST", undoRoot, undoBody);
    const undoMutationId = undone.value?.mutationId;
    if (
      undone.status !== 200 ||
      undone.value?.schema !== "semio.hub.gis-map-approval-undo-receipt/v1" ||
      undone.value?.targetId !== undoBody.targetId ||
      undone.value?.originalJobId !== jobId ||
      undone.value?.applied !== true ||
      undone.value?.replayed !== false ||
      !/^[0-9a-f]{32}$/u.test(undoMutationId ?? "")
    ) {
      throw new Error(`GIS Map owner durable undo did not reach its second committed publication: ${undone.status}`);
    }
    const undoControlA = await waitForDocumentSocketFrame(socketA.socket, socketA.frames, (frame) => frame.RebootstrapRequired?.control, "Author A undo RebootstrapRequired", undoFrameStartA);
    const undoControlB = await waitForDocumentSocketFrame(socketB.socket, socketB.frames, (frame) => frame.RebootstrapRequired?.control, "Author B undo RebootstrapRequired", undoFrameStartB);
    if (
      socketA.error ||
      socketB.error ||
      JSON.stringify(undoControlA) !== JSON.stringify(undoControlB) ||
      undoControlA.space_id !== spaceId ||
      undoControlA.document_id !== fixture.documentId ||
      undoControlA.baseline_frontier?.head_edit_id !== undoMutationId
    ) {
      throw new Error("GIS Map Authors did not receive one identical undo-bound RebootstrapRequired control");
    }
    const undoneA = await readGisMapProcessCheckpoint(mcpA, spaceId, fixture.documentId);
    const undoneB = await readGisMapProcessCheckpoint(mcpB, spaceId, fixture.documentId);
    if (
      JSON.stringify(undoneA) !== JSON.stringify(undoneB) ||
      undoneA.activeCheckpointId === refreshedA.activeCheckpointId ||
      undoneA.packSha256 !== initialA.packSha256 ||
      undoneA.frontier?.headEditId !== undoMutationId ||
      JSON.stringify(undoneA.frontier) !== JSON.stringify(undone.value?.frontier) ||
      JSON.stringify(trustedBootstrapCurrent(dataRoot)) !== JSON.stringify(current)
    ) {
      throw new Error("GIS Map MCP Authors did not refresh to one equal server-undone canonical Map pair");
    }
    const replay = await gisMapProcessHttp(run, authorA.capability, "POST", undoRoot, undoBody);
    if (replay.status !== 200 || replay.value?.applied !== true || replay.value?.replayed !== true || JSON.stringify(replay.value?.frontier) !== JSON.stringify(undoneA.frontier)) {
      throw new Error("GIS Map durable undo exact retry did not replay its sole terminal receipt");
    }
    await Bun.write(
      join(artifactPath, "gis-map-proposal-process-receipt.json"),
      `${JSON.stringify(
        {
          schema: "semio.hub.gis-map-proposal-process-receipt/v1",
          catalogGenerationId: current.generationId,
          componentSha256: selected.component.sha256,
          executionProtocol: selected.executionProtocol,
          documentId: fixture.documentId,
          initialCheckpointId: initialA.activeCheckpointId,
          appliedCheckpointId: refreshedA.activeCheckpointId,
          appliedMutationId: mutationId,
          undoneCheckpointId: undoneA.activeCheckpointId,
          undoMutationId,
          undoPeerDenial: true,
          staleUndoDenial: true,
          undoReplay: true,
          privatePeerDenials: 3,
          independentPeerJobCancelled: true,
          cancelledPeerJobId: jobIdB,
          cancellationControl: { descriptor: 4, progressPersisted: true, released: true, exactJobIdentity: true, progress: run.inferenceCheckpointProgress },
          liveRebootstrapControls: 4,
          refreshedCanonicalPairs: 4,
          hubBinarySha256: createHash("sha256").update(readFileSync(binaryPath)).digest("hex"),
          nonclaims: ["no-shell-scene", "no-wgpu-render"],
        },
        null,
        2,
      )}\n`,
    );
    console.log(`gis-map-proposal-process: generation=${current.generationId} private-denials=3 undo-denials=2 peer-cancel=1 rebootstrap=4 refreshed-pairs=4 undo-replay=1; no Shell scene or WGPU render claim; durable collaborative redo is owned by the history transition fold`);
  } catch (error) {
    const diagnostics = run.output().slice(-8_192);
    throw new Error(`${error instanceof Error ? error.message : "GIS Map proposal process failed"}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    const cleanupErrors: unknown[] = [];
    await finishGisMapProcessDocumentSocket(socketA).catch((error) => cleanupErrors.push(error));
    await finishGisMapProcessDocumentSocket(socketB).catch((error) => cleanupErrors.push(error));
    await finishGisMapProcessMcpClient(mcpA).catch((error) => cleanupErrors.push(error));
    await finishGisMapProcessMcpClient(mcpB).catch((error) => cleanupErrors.push(error));
    if (authorA) authorA.capability = "";
    if (authorB) authorB.capability = "";
    await finishLocalHub(run).catch((error) => cleanupErrors.push(error));
    if (cleanupErrors.length) throw new AggregateError(cleanupErrors, "GIS Map proposal process cleanup failed");
  }
  });
 });
}

/** 🗺️ The exact GIS Map proposal/approval gate: neutral oracle, native hub laws, honest nonclaims. */
export class GisMapProposalCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const mode = segments[0] ?? "--source";
    if (segments.length > 1 || !["--source", "--native", "--process"].includes(mode)) throw new Error("usage: gis-map-proposal-check [--source|--native|--process]");
    const hostile = await proveGisMapProposalApprovalFixture(this.repoRoot);
    if (mode === "--native" || mode === "--process") {
      const libraryLaws: [string, string][] = [["semio-hub-inference-laws", "gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation"], ["semio-hub-inference-laws", "gis_map_proposal_is_private_to_its_original_author_owner"], ["semio-hub-inference-laws", "gis_map_approval_fails_closed_without_a_composition_transaction_and_never_auto_applies"], ["semio-hub-inference-laws", "gis_map_approval_committed_event_reaches_actor_frontier_and_public_checkpoint_before_ledger_apply"], ["semio-hub-inference-laws", "gis_map_abandoned_pre_witness_request_returns_exact_stores_and_document_writer"], ["semio-hub-inference-laws", "gis_map_terminal_close_waits_for_unpolled_cleanup_and_fences_new_admission"], ["semio-hub-inference-laws", "gis_map_proposal_fixture_pins_the_exact_frozen_comparison_limits_and_error_vocabulary"]];
      const routeLaws: [string, string][] = [["semio-hub-boot-laws", "gis_map_approval_ingress_holds_sorted_hub_authority_without_outer_document_write"], ["semio-hub-boot-laws", "gis_map_applied_checkpoint_notifies_two_peers_with_one_exact_rebootstrap_pair"], ["semio-hub-boot-laws", "gis_map_proposal_routes_fail_closed_without_a_trusted_map_binding"], ["semio-hub-boot-laws", "gis_map_proposal_owner_claims_streams_and_boundedly_retires_on_cancellation"], ["semio-hub-boot-laws", "inference_reconciliation_route_is_existing_only_reader_bound_and_body_bounded"], ["semio-hub-boot-laws", "gis_map_inference_runtime_close_signals_and_joins_actual_codec_work"], ["semio-hub-boot-laws", "gis_map_inference_revocation_after_compute_refuses_late_publication"], ["semio-hub-boot-laws", "gis_map_proposal_is_private_to_every_peer_spectator_and_stale_caller"], ["semio-hub-boot-laws", "gis_map_approval_stamps_one_create_region_and_rejects_every_frozen_drift"], ["semio-hub-boot-laws", "gis_map_approval_is_idempotent_across_duplicate_requests_and_restart"]];
      if (mode === "--process") routeLaws.push(["semio-hub-boot-laws", "check_in_process_fixture_emits_verified_gis_ledger_and_catalog"]);
      const laws = [...libraryLaws, ...routeLaws];
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs(), listMilliseconds: 60_000, lawMilliseconds: 120_000 },cwd: this.root,...exactCargoStageEnvironments(),groups: [
          ...hubLawGroups(libraryLaws),
          ...hubLawGroups(routeLaws, ["--features", "integration-fixtures"]),
        ],consume:async outputs=>{
         if(mode!=="--process")return;
         await withExactCargoLawOutputV1(outputs,{version:1,receiptIndex:outputs.receipts.findIndex(row=>row.package==="semio-hub-boot-laws"&&row.laws.some(law=>law.endsWith("check_in_process_fixture_emits_verified_gis_ledger_and_catalog")))},output=>buildRepositoryCargoArtifacts(join(this.root,"Cargo.toml"),["--bin","os-hub","--features","integration-fixtures"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:output.control,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:capture=>withCapturedLocalHubHostV1(this.invocation,output.control,capture,(host,context)=>withCheckInProcessFixtureV1(host,context.control,outputs,{version:1,purpose:"runtime"},async(payload,actual)=>{
          const manifest=join(this.repoRoot,"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/Cargo.toml");
          await buildRepositoryCargoArtifacts(manifest,["--bins"],this.repoRoot,{producer:"artifact",invocation:this.invocation,control:actual,network:repositoryCargoNetworkV1(this.invocation,this.repoRoot),consume:verifyCapturedCargoArtifactsV1});
          await withRepositoryPublishedCargoExecutableV1(manifest,process.platform==="win32"?"semio-os-mcp.exe":"semio-os-mcp",this.repoRoot,readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode,this.invocation,actual,(mcp,mcpPhase)=>proveGisMapProposalProcess(host,mcpPhase,payload,mcp.path));
         }))}));
        },progress(event) {
          console.log(`gis-map-proposal ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      for (const receipt of receipts) console.log(`gis-map-proposal-receipt: ${JSON.stringify(receipt)}`);
      console.log(`gis-map-proposal-check: neutral hostile=${hostile} exact-native=${laws.length}; no external model provider, no WGPU rendering`);
    }
    if (mode === "--source") console.log(`gis-map-proposal-check: neutral source oracle passed with hostile=${hostile}; native laws and the two-user process journey remain unclaimed. No external model provider, no WGPU rendering.`);
  }
}
