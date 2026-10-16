/** 🧭️ Browser broker, execution target and inference relay gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { configuredExactCargoLawPolicyV1 } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️laws/🟦️.ts";
import { GIS_MAP_INFERENCE_REQUEST_MAX_BYTES, GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES, GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR } from "../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🧬️schema/🟦️.ts";
import { buildBudgetMs } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/⏱️budget/🟦️.ts";
import { randomBytes } from "node:crypto";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES, parseDocumentOpenIntentV1 } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🧬️schema/🔏️document-open/🟦️.ts";
import { DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema";
import { DOCUMENT_BROWSER_ACTOR_MAX_BYTES } from "@semio-tech/framework/directory/schema/browser-actor";
import { runRepositoryExactCargoLaws, runVitest, configuredRepositoryCargoPreparationPolicyV1 } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { exactCargoStageEnvironments } from "../../🏗️build/🛂staging-environment/🟦️.ts";
import { localRelayUpstreamPath } from "../../🚀️local-relay/🧭️routing/🟦️.ts";
import { INFERENCE_PROGRESS_MAX_CURSOR } from "../../💡️inference/🧬️schema/🟦️.ts";
import { BROWSER_BROKER_BOOTSTRAP_PROOF_TTL_MS, BROWSER_BROKER_PROOF_TTL_MS, browserBrokerProofDigest, EXECUTION_TARGET_RELAY_DEADLINE_MS, EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES, EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT, EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES, type LocalBrowserRelay, readLocalRelayBody, startLocalBrowserRelay } from "../../🚀️local-relay/🛰️servers/🟦️.ts";
import { readHubBinSource, readHubBinTestSource, readTrustedCatalogSource } from "../../🧪️testing/📖️source-readers/🟦️.ts";

type BrowserBrokerOracleUpstream = { effects: number; status: number; hold: boolean };

/** 🔢️ Reads a relay-oracle counter that the upstream `Bun.serve` handler advances out of band, where control-flow narrowing would otherwise freeze the field at the literal the previous assertion proved. */
export function observedCount(counter: number): number {
  return counter;
}

function browserBrokerOracleEnvelope(): Record<string, unknown> {
  return { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability: `session.v1.${"1".repeat(32)}.${"2".repeat(64)}` };
}

async function browserBrokerOracleRequest(relay: LocalBrowserRelay, uiOrigin: string, proof?: string, nextProof?: Buffer, path = "/_semio/hub/auth/sessions/me", signal?: AbortSignal): Promise<Response> {
  const headers: Record<string, string> = {
    host: new URL(uiOrigin).host,
    origin: uiOrigin,
    referer: `${uiOrigin}/`,
    "sec-fetch-site": "same-origin",
    "x-semio-local-relay": relay.secret.toString("hex"),
  };
  if (proof) headers["x-semio-browser-broker"] = proof;
  if (nextProof) headers["x-semio-browser-broker-next"] = browserBrokerProofDigest(nextProof).toString("hex");
  return fetch(`${relay.url}${path}`, { headers, redirect: "error", signal });
}

async function waitForBrowserBrokerEffect(upstream: BrowserBrokerOracleUpstream, expected: number): Promise<void> {
  const deadline = Date.now() + 2_000;
  while (observedCount(upstream.effects) < expected && Date.now() < deadline) await Bun.sleep(5);
  if (observedCount(upstream.effects) !== expected) throw new Error("browser broker upstream effect deadline exceeded");
}

async function runHostilePluginShard(relay: LocalBrowserRelay, uiOrigin: string): Promise<{ readonly status: number; readonly hasProof: boolean; readonly hasPort: boolean; readonly hash: string }> {
  const source = String.raw`
self.onmessage = async event => {
  const { url, uiOrigin, relaySecret } = event.data;
  let status = 0;
  try {
    const response = await fetch(url, { headers: { host: new URL(uiOrigin).host, origin: uiOrigin, referer: uiOrigin + "/", "sec-fetch-site": "same-origin", "x-semio-local-relay": relaySecret } });
    status = response.status;
  } catch {}
  self.postMessage({ status, hasProof: "localBrowserBrokerProof" in self, hasPort: "localBrowserBrokerPort" in self, hash: self.location?.hash ?? "" });
};`;
  const objectUrl = URL.createObjectURL(new Blob([source], { type: "text/javascript" }));
  const worker = new Worker(objectUrl);
  try {
    return await Promise.race([
      new Promise<{ status: number; hasProof: boolean; hasPort: boolean; hash: string }>((resolveResult, rejectResult) => {
        worker.onmessage = (event: MessageEvent) => resolveResult(event.data);
        worker.onerror = () => rejectResult(new Error("hostile plugin shard worker failed"));
        worker.postMessage({ url: `${relay.url}/_semio/hub/auth/sessions/me`, uiOrigin, relaySecret: relay.secret.toString("hex") });
      }),
      Bun.sleep(2_000).then(() => {
        throw new Error("hostile plugin shard worker deadline exceeded");
      }),
    ]);
  } finally {
    worker.terminate();
    URL.revokeObjectURL(objectUrl);
  }
}

async function proveBrowserBrokerRelay(repoRoot: string): Promise<void> {
  const lifecycleFixture = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🔐️browser-broker-proof-lifecycle-v1/🔣️.json"), "utf8")) as {
    readonly limits: { readonly bootstrapMaximumMs: number; readonly activeMaximumMs: number };
    readonly delayedBootstrap: { readonly activeProofTtlMs: number; readonly bootstrapProofTtlMs: number; readonly delayMs: number };
    readonly bootstrapExpiry: { readonly activeProofTtlMs: number; readonly bootstrapProofTtlMs: number; readonly delayMs: number };
    readonly expected: { readonly unarmed: number; readonly admitted: number; readonly expired: number; readonly replayed: number };
  };
  if (
    lifecycleFixture.limits.bootstrapMaximumMs !== BROWSER_BROKER_BOOTSTRAP_PROOF_TTL_MS ||
    lifecycleFixture.limits.activeMaximumMs !== BROWSER_BROKER_PROOF_TTL_MS ||
    lifecycleFixture.delayedBootstrap.delayMs <= lifecycleFixture.delayedBootstrap.activeProofTtlMs ||
    lifecycleFixture.delayedBootstrap.delayMs >= lifecycleFixture.delayedBootstrap.bootstrapProofTtlMs ||
    lifecycleFixture.bootstrapExpiry.delayMs <= lifecycleFixture.bootstrapExpiry.bootstrapProofTtlMs
  ) throw new Error(`browser broker lifecycle example timing does not match runtime limits`);
  const uiOrigin = "http://127.0.0.1:6066";
  const upstreamState: BrowserBrokerOracleUpstream = { effects: 0, status: 200, hold: false };
  const upstreamServer = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      upstreamState.effects += 1;
      if (request.headers.get("authorization") !== `Bearer ${browserBrokerOracleEnvelope().capability}`) return new Response("unauthorized", { status: 401 });
      if (upstreamState.hold) {
        await Promise.race([new Promise<void>((resolveAbort) => request.signal.addEventListener("abort", () => resolveAbort(), { once: true })), Bun.sleep(2_000)]);
      }
      return new Response("{}", { status: upstreamState.status, headers: { "content-type": "application/json" } });
    },
  });
  const hubOrigin = `http://127.0.0.1:${upstreamServer.port}`;
  let relay: LocalBrowserRelay | undefined;
  try {
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope());
    const rawLocal = await browserBrokerOracleRequest(relay, uiOrigin);
    if (rawLocal.status !== lifecycleFixture.expected.unarmed || observedCount(upstreamState.effects) !== 0) throw new Error("unarmed browser broker reached upstream");
    const shardAttempt = await runHostilePluginShard(relay, uiOrigin);
    if (shardAttempt.status !== 401 || shardAttempt.hasProof || shardAttempt.hasPort || shardAttempt.hash !== "" || observedCount(upstreamState.effects) !== 0) throw new Error("running same-origin plugin shard crossed private broker boundary");
    const proof0 = relay.takeBrowserBootstrapProof();
    const proof0Hex = proof0.toString("hex");
    proof0.fill(0);
    let duplicateTakeRejected = false;
    try {
      relay.takeBrowserBootstrapProof();
    } catch {
      duplicateTakeRejected = true;
    }
    if (!duplicateTakeRejected) throw new Error("browser broker issued duplicate bootstrap proof");
    const proof1 = randomBytes(32);
    const admitted = await browserBrokerOracleRequest(relay, uiOrigin, proof0Hex, proof1);
    if (admitted.status !== lifecycleFixture.expected.admitted || admitted.headers.get("x-semio-browser-broker-advanced") !== "1" || observedCount(upstreamState.effects) !== 1) throw new Error("browser broker did not acknowledge an admitted ratchet");
    const replay = await browserBrokerOracleRequest(relay, uiOrigin, proof0Hex, randomBytes(32));
    if (replay.status !== lifecycleFixture.expected.replayed || observedCount(upstreamState.effects) !== 1 || (await replay.text()).includes(proof0Hex)) throw new Error("browser broker accepted or reflected a replayed proof");
    await relay.stop();

    upstreamState.effects = 0;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), lifecycleFixture.delayedBootstrap);
    await Bun.sleep(lifecycleFixture.delayedBootstrap.delayMs);
    const ttlProof = relay.takeBrowserBootstrapProof();
    const ttlProofHex = ttlProof.toString("hex");
    ttlProof.fill(0);
    const ttlNext = randomBytes(32);
    const ttlAdvanced = await browserBrokerOracleRequest(relay, uiOrigin, ttlProofHex, ttlNext);
    if (ttlAdvanced.status !== lifecycleFixture.expected.admitted || ttlAdvanced.headers.get("x-semio-browser-broker-advanced") !== "1" || observedCount(upstreamState.effects) !== 1) throw new Error("delayed bootstrap did not advance the initial broker proof");
    await Bun.sleep(lifecycleFixture.delayedBootstrap.delayMs);
    const expired = await browserBrokerOracleRequest(relay, uiOrigin, ttlNext.toString("hex"), randomBytes(32));
    if (expired.status !== lifecycleFixture.expected.expired || observedCount(upstreamState.effects) !== 1) throw new Error("expired rotated broker proof reached upstream");
    await relay.stop();

    upstreamState.effects = 0;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope(), lifecycleFixture.bootstrapExpiry);
    const expiredBootstrapProof = relay.takeBrowserBootstrapProof();
    const expiredBootstrapProofHex = expiredBootstrapProof.toString("hex");
    expiredBootstrapProof.fill(0);
    await Bun.sleep(lifecycleFixture.bootstrapExpiry.delayMs);
    const expiredBootstrap = await browserBrokerOracleRequest(relay, uiOrigin, expiredBootstrapProofHex, randomBytes(32));
    if (expiredBootstrap.status !== lifecycleFixture.expected.expired || observedCount(upstreamState.effects) !== 0) throw new Error("expired bootstrap proof reached upstream");
    await relay.stop();

    upstreamState.effects = 0;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope());
    const rejectedProof = relay.takeBrowserBootstrapProof();
    const rejectedProofHex = rejectedProof.toString("hex");
    rejectedProof.fill(0);
    upstreamState.status = 401;
    const rejectedNext = randomBytes(32);
    const rejected = await browserBrokerOracleRequest(relay, uiOrigin, rejectedProofHex, rejectedNext);
    if (rejected.status !== 401 || rejected.headers.get("x-semio-browser-broker-advanced") !== "1" || observedCount(upstreamState.effects) !== 1) throw new Error("upstream rejection did not close the broker epoch");
    const afterRejection = await browserBrokerOracleRequest(relay, uiOrigin, rejectedNext.toString("hex"), randomBytes(32));
    if (afterRejection.status !== 401 || observedCount(upstreamState.effects) !== 1) throw new Error("broker retained authority after upstream rejection");
    await relay.stop();

    upstreamState.effects = 0;
    upstreamState.status = 200;
    upstreamState.hold = true;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, browserBrokerOracleEnvelope());
    const cancelProof = relay.takeBrowserBootstrapProof();
    const cancelProofHex = cancelProof.toString("hex");
    cancelProof.fill(0);
    const abort = new AbortController();
    const cancelled = browserBrokerOracleRequest(relay, uiOrigin, cancelProofHex, randomBytes(32), "/_semio/hub/auth/sessions/me", abort.signal).catch(() => undefined);
    await waitForBrowserBrokerEffect(upstreamState, 1);
    abort.abort();
    await cancelled;
    const replayAfterCancel = await browserBrokerOracleRequest(relay, uiOrigin, cancelProofHex, randomBytes(32));
    if (replayAfterCancel.status !== 401 || observedCount(upstreamState.effects) !== 1) throw new Error("cancel-after-send allowed proof reuse");
  } finally {
    await relay?.stop();
    await upstreamServer.stop(true);
  }
}

type ExecutionTargetRelayFixture = {
  readonly schema: "semio.hub.execution-target-relay/v1";
  readonly intent: Record<string, unknown>;
  readonly limits: {
    readonly requestBytes: number;
    readonly manifestBytes: number;
    readonly componentBytes: number;
    readonly descriptorBytes: number;
    readonly browserActorBytes: number;
    readonly inFlight: number;
    readonly hubDeadlineMs: number;
    readonly relayDeadlineMs: number;
  };
  readonly routes: readonly { readonly id: string; readonly method: string; readonly path: string; readonly admitted: boolean }[];
  readonly responses: readonly { readonly id: string; readonly asset: string; readonly bytes: number; readonly delayMs: number; readonly status: number }[];
  readonly fences: readonly { readonly mutation: string; readonly expected: string }[];
};

type NativeArtifactProviderFrontierFixture = {
  readonly schema: "semio.hub.native-artifact-provider-frontier/v1";
  readonly production: {
    readonly providerId: "stdio+gis+vcs/native-codecs/v1";
    readonly receiptAuthority: "compiled-kind-schema-roster";
    readonly pluginDependencies: readonly ["stdio-catalog", "gis", "vcs"];
  };
  readonly configuredWithoutProvider: "reject";
};

async function proveNativeArtifactProviderFrontier(repoRoot: string): Promise<number> {
  const fixtureRoot = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🧭️native-artifact-provider-frontier-v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as NativeArtifactProviderFrontierFixture;
  const providerIdentity = hubSchemaExport(repoRoot, "schema://hub.artifact-authority.native-openable-provider/NativeCodecProviderSetIdentityV1");
  if (!providerIdentity(fixture.production)) throw new Error("native artifact provider frontier fixture violates its owning scope contract");
  if (String(fixture.schema) !== "semio.hub.native-artifact-provider-frontier/v1" || String(fixture.configuredWithoutProvider) !== "reject") throw new Error("native artifact provider frontier envelope differs");
  if (String(fixture.production.providerId) !== "stdio+gis+vcs/native-codecs/v1" || fixture.production.receiptAuthority !== "compiled-kind-schema-roster"
    || JSON.stringify(fixture.production.pluginDependencies) !== JSON.stringify(["stdio-catalog", "gis", "vcs"])) throw new Error("production native artifact provider frontier differs");
  const manifest = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/📦️packages/🦀️rust/Cargo.toml"), "utf8");
  const pluginPackages = fixture.production.pluginDependencies.map((plugin) => "semio-hub-" + plugin);
  const document = Bun.TOML.parse(manifest) as { dependencies: Record<string, { workspace?: boolean; optional?: boolean }> };
  for (const dependency of pluginPackages) {
    const local = document.dependencies[dependency];
    if (local?.workspace !== true || local.optional === true) throw new Error(dependency + " is not an unconditional workspace dependency of the native provider crate");
  }
  const hubManifest = Bun.TOML.parse(readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/Cargo.toml"), "utf8")) as { dependencies: Record<string, { workspace?: boolean; optional?: boolean }> };
  const providerCrate = hubManifest.dependencies["semio-hub-native-openable"];
  if (providerCrate?.workspace !== true || providerCrate.optional === true) throw new Error("native provider crate is no longer an unconditional production dependency of the Hub");
  const trusted = readTrustedCatalogSource(repoRoot);
  if (!trusted.includes("pub trait NativeCodecProviderSourceV1: Sync") || trusted.includes("use super::native_openable_provider::NativeCodecProviderSetV1;")) throw new Error("trusted catalog core still owns a concrete native plugin provider");
  const provider = readFileSync(join(repoRoot, "🌎️hub/🗿️artifact-authority/📇️native-openable-provider/🦀️.rs"), "utf8");
  if (!provider.includes('pub const NATIVE_OPENABLE_PROVIDER_SET_V1_ID: &str = "' + fixture.production.providerId + '";') || !provider.includes("pub fn native_openable_provider_receipt_count()"))
    throw new Error("production provider identity or receipt closure drifted");
  const startup = readHubBinSource(repoRoot);
  if (
    !startup.includes("providers: Option<&dyn NativeCodecProviderSourceV1>") ||
    !startup.includes("configured trusted catalog requires the native-artifact-execution provider") ||
    !readHubBinTestSource(repoRoot).includes("configured_catalog_without_a_native_provider_fails_closed")
  )
    throw new Error("headless configured-catalog startup no longer fails closed");
  console.log(
    "hub-native-artifact-provider-frontier-oracle: scope-exports=1 plugin-deps=" +
      pluginPackages.length +
      " production-receipts=" +
      fixture.production.receiptAuthority +
      " configured-no-provider=" +
      fixture.configuredWithoutProvider,
  );
  return 12;
}

async function proveExecutionTargetRelay(repoRoot: string): Promise<number> {
  const root = join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/🪪️execution-target-relay-v1");
  const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8")) as ExecutionTargetRelayFixture;
  if (fixture.schema !== "semio.hub.execution-target-relay/v1") throw new Error("execution-target relay fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "fences,intent,limits,responses,routes,schema") throw new Error("execution-target relay fixture envelope drift");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DocumentOpenIntentV1")(fixture.intent)) throw new Error("execution-target relay intent is not the owned document-open-intent contract");
  if (fixture.routes.length < 12 || new Set(fixture.routes.map((row) => row.id)).size !== fixture.routes.length) throw new Error("execution-target relay route inventory drift");
  if (fixture.responses.length < 5 || new Set(fixture.responses.map((row) => row.id)).size !== fixture.responses.length) throw new Error("execution-target relay response inventory drift");
  const relayFenceOracle: Readonly<Record<string, string>> = { unchanged: "selected", "directory-advanced": "stale", "session-revoked": "denied", cancelled: "cancelled" };
  if (fixture.fences.length < 4 || fixture.fences.some((row) => relayFenceOracle[row.mutation] !== row.expected)) throw new Error("execution-target relay fence oracle drift");
  if (new Set(fixture.fences.map((row) => row.mutation)).size !== Object.keys(relayFenceOracle).length) throw new Error("execution-target relay fence inventory is not exhaustive");
  for (const row of fixture.responses) {
    if (!["manifest", "component", "descriptor", "browser-actor"].includes(row.asset) || ![200, 503].includes(row.status)) throw new Error(`execution-target relay response taxonomy drift: ${row.id}`);
    if (!Number.isSafeInteger(row.bytes) || row.bytes < 1 || row.bytes > 67108865 || !Number.isSafeInteger(row.delayMs) || row.delayMs < 0 || row.delayMs > 2500) throw new Error(`execution-target relay response bound drift: ${row.id}`);
  }
  const limits = fixture.limits;
  if (
    limits.requestBytes !== EXECUTION_TARGET_RELAY_REQUEST_MAX_BYTES ||
    limits.manifestBytes !== EXECUTION_TARGET_RELAY_MANIFEST_MAX_BYTES ||
    limits.componentBytes !== DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES ||
    limits.descriptorBytes !== DOCUMENT_EXECUTION_TARGET_DESCRIPTOR_MAX_BYTES ||
    limits.browserActorBytes !== DOCUMENT_BROWSER_ACTOR_MAX_BYTES ||
    limits.inFlight !== EXECUTION_TARGET_RELAY_MAX_IN_FLIGHT ||
    limits.relayDeadlineMs !== EXECUTION_TARGET_RELAY_DEADLINE_MS
  )
    throw new Error("execution-target relay bound drift");
  const hub = readHubBinSource(repoRoot);
  const selection = hub.slice(hub.indexOf("async fn document_execution_target_selection("), hub.indexOf("async fn issue_document_execution_target("));
  const finalFence = selection.slice(selection.indexOf("fields.validate()"));
  if (
    !hub.includes(`const DOCUMENT_EXECUTION_TARGET_DEADLINE_MS: u64 = ${limits.hubDeadlineMs.toLocaleString("en-US").replaceAll(",", "_")};`) ||
    !finalFence.includes("subject.revalidate(") ||
    !finalFence.includes("state.directory.head_seq().await") ||
    !finalFence.includes("DocumentOpenPlanErrorCodeV1::Stale") ||
    !readHubBinTestSource(repoRoot).includes('fixture["fences"]')
  )
    throw new Error("execution-target final authorization/revision fence or native corpus missing");
  const admittedRoute = hubSchemaExport(repoRoot, "schema://hub.directory/ExecutionTargetAssetRouteV1");
  for (const row of fixture.routes) {
    if (admittedRoute(row) !== row.admitted) throw new Error(`execution-target independent schema route mismatch: ${row.id}`);
    if ((localRelayUpstreamPath(row.method, new URL(row.path, "http://127.0.0.1")) !== undefined) !== row.admitted) throw new Error(`execution-target production route mismatch: ${row.id}`);
  }
  const slowAbort = new AbortController();
  const slowBody = new Request("http://127.0.0.1", { method: "POST", body: new ReadableStream<Uint8Array>() });
  const slowRead = readLocalRelayBody(slowBody, limits.requestBytes, slowAbort.signal).then(
    () => false,
    () => true,
  );
  slowAbort.abort();
  if (!(await Promise.race([slowRead, Bun.sleep(100).then(() => false)]))) throw new Error("execution-target slow upload did not release on cancellation");
  const uiOrigin = "http://127.0.0.1:6066";
  const intentBody = JSON.stringify(fixture.intent);
  parseDocumentOpenIntentV1(fixture.intent);
  let effects = 0;
  let responseBytes = 1;
  let delayMs = 0;
  let release: Promise<void> | undefined;
  const upstream = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      effects += 1;
      if (request.headers.get("authorization") !== `Bearer ${browserBrokerOracleEnvelope().capability}`) throw new Error("execution-target upstream credential mismatch");
      const body = await request.text();
      if (body !== intentBody) throw new Error("execution-target intent body changed in transit");
      if (release) await release;
      if (delayMs) await Bun.sleep(delayMs);
      return new Response(new Uint8Array(responseBytes).fill(73), { headers: { "content-type": "application/octet-stream", "cache-control": "no-store" } });
    },
  });
  const relay = startLocalBrowserRelay(`http://127.0.0.1:${upstream.port}`, uiOrigin, browserBrokerOracleEnvelope());
  let proof = relay.takeBrowserBootstrapProof();
  const request = async (path: string, method = "POST", body = intentBody, signal?: AbortSignal): Promise<Response> => {
    const current = proof;
    proof = randomBytes(32);
    const response = await fetch(`${relay.url}${path}`, {
      method,
      body: method === "GET" ? undefined : body,
      signal,
      redirect: "error",
      headers: {
        host: new URL(uiOrigin).host,
        origin: uiOrigin,
        referer: `${uiOrigin}/`,
        "sec-fetch-site": "same-origin",
        "x-semio-local-relay": relay.secret.toString("hex"),
        "x-semio-browser-broker": current.toString("hex"),
        "x-semio-browser-broker-next": browserBrokerProofDigest(proof).toString("hex"),
        "content-type": "application/json",
      },
    });
    if (response.headers.get("x-semio-browser-broker-advanced") !== "1") proof = current;
    return response;
  };
  const path = (asset: string): string => `/_semio/hub/spaces/studio/documents/map/execution-target/${asset}`;
  try {
    for (const row of fixture.routes) {
      const before = effects;
      const response = await request(row.path, row.method);
      if (response.status !== (row.admitted ? 200 : 404) || effects !== before + Number(row.admitted)) throw new Error(`execution-target live route mismatch: ${row.id}`);
      await response.arrayBuffer();
    }
    for (const row of fixture.responses) {
      responseBytes = row.bytes;
      delayMs = row.delayMs;
      const response = await request(path(row.asset));
      if (response.status !== row.status) throw new Error(`execution-target live response mismatch: ${row.id}: ${response.status}`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (row.status === 200 && (response.headers.get("content-length") !== String(row.bytes) || response.headers.get("cache-control") !== "no-store" || bytes.length !== row.bytes || bytes.some((value) => value !== 73)))
        throw new Error(`execution-target live bytes mismatch: ${row.id}`);
    }
    const before = effects;
    const oversized = await request(path("manifest"), "POST", "x".repeat(fixture.limits.requestBytes + 1));
    if (oversized.status !== 413 || effects !== before || oversized.headers.has("x-semio-browser-broker-advanced")) throw new Error("execution-target request overflow crossed ratchet or upstream");
    delayMs = 0;
    responseBytes = 1024 * 1024 + 17;
    let resolveRelease!: () => void;
    release = new Promise<void>((resolve) => {
      resolveRelease = resolve;
    });
    const pending: Promise<Response>[] = [];
    try {
      for (let index = 0; index < fixture.limits.inFlight; index += 1) {
        pending.push(request(path("component")));
        const deadline = Date.now() + 2_000;
        while (effects < before + index + 1 && Date.now() < deadline) await Bun.sleep(5);
        if (effects !== before + index + 1) throw new Error("execution-target concurrent admission did not reach upstream");
      }
      const saturated = await request(path("component"));
      if (saturated.status !== 503 || saturated.headers.has("x-semio-browser-broker-advanced") || effects !== before + fixture.limits.inFlight) throw new Error("execution-target saturation consumed authority or exceeded capacity");
    } finally {
      resolveRelease();
      await Promise.all(
        pending.map(async (response) => {
          if (!(await response).ok) throw new Error("execution-target admitted concurrent request failed");
        }),
      );
      release = undefined;
    }
    const recovered = await request(path("descriptor"));
    if (!recovered.ok) throw new Error("execution-target capacity did not recover");
    await recovered.arrayBuffer();
    const cancelBefore = effects;
    let resolveCancelled!: () => void;
    release = new Promise<void>((resolve) => {
      resolveCancelled = resolve;
    });
    const abort = new AbortController();
    const cancelled = request(path("component"), "POST", intentBody, abort.signal).then(
      () => {
        throw new Error("execution-target cancelled request returned bytes");
      },
      () => undefined,
    );
    try {
      const deadline = Date.now() + 2_000;
      while (effects < cancelBefore + 1 && Date.now() < deadline) await Bun.sleep(5);
      if (effects !== cancelBefore + 1) throw new Error("execution-target cancellation did not reach upstream");
      abort.abort();
      await cancelled;
    } finally {
      resolveCancelled();
      release = undefined;
    }
    responseBytes = 19;
    const afterCancel = await request(path("component"));
    if (!afterCancel.ok || (await afterCancel.arrayBuffer()).byteLength !== 19) throw new Error("execution-target cancellation did not release capacity and ratchet");
    console.log(`execution-target relay runtime routes=${fixture.routes.length} responses=${fixture.responses.length} bounded-capacity=${fixture.limits.inFlight} exact-bytes=true`);
    return fixture.routes.length + fixture.responses.length + fixture.fences.length + 5;
  } finally {
    proof.fill(0);
    await relay.stop();
    await upstream.stop(true);
  }
}

type InferenceRelayFixture = {
  readonly schema: string;
  readonly limits: { readonly requestBytes: number; readonly responseBytes: number; readonly progressCursor: number };
  readonly routes: readonly { readonly id: string; readonly method: string; readonly path: string; readonly admitted: boolean }[];
  readonly responses: readonly { readonly id: string; readonly bytes: number; readonly streamed: boolean; readonly status: number }[];
};

async function proveInferenceRelay(repoRoot: string): Promise<number> {
  const fixture = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/🧪️tests/🧫️fixtures/💡️inference-relay-v1/🔣️.json"), "utf8")) as InferenceRelayFixture;
  const schema = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/💡️inference/🌐️relay/🔣️.json"), "utf8"));
  const oracle = new Ajv({ strict: true }).compile(schema);
  if (fixture.schema !== "semio.hub.inference-relay-fixture/v1" || fixture.routes.length < 29 || new Set(fixture.routes.map((row) => row.id)).size !== fixture.routes.length) throw new Error("inference relay fixture inventory drift");
  if (fixture.limits.requestBytes !== GIS_MAP_INFERENCE_REQUEST_MAX_BYTES || fixture.limits.responseBytes !== GIS_MAP_INFERENCE_RESPONSE_MAX_BYTES || fixture.limits.progressCursor !== GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR || INFERENCE_PROGRESS_MAX_CURSOR !== GIS_MAP_INFERENCE_PROGRESS_MAX_CURSOR) throw new Error("inference relay bound drift");
  for (const row of fixture.routes) if (oracle({ method: row.method, path: row.path }) !== row.admitted) throw new Error(`inference relay independent schema mismatch: ${row.id}`);
  const uiOrigin = "http://127.0.0.1:6066";
  let calls = 0;
  let responseBytes = 2;
  let responseStreamed = false;
  let expectedPath = "";
  let expectedMethod = "";
  let expectedBody = "";
  const upstream = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      calls += 1;
      const url = new URL(request.url);
      if (request.headers.get("authorization") !== `Bearer ${browserBrokerOracleEnvelope().capability}` || request.headers.has("x-semio-browser-broker")) throw new Error("inference relay upstream credential boundary failed");
      if (url.pathname + url.search !== expectedPath || request.method !== expectedMethod || await request.text() !== expectedBody) throw new Error("inference relay changed upstream intent");
      const bytes = new Uint8Array(responseBytes).fill(73);
      const body = responseStreamed ? new ReadableStream<Uint8Array>({ start(controller) { controller.enqueue(bytes.subarray(0, 8192)); controller.enqueue(bytes.subarray(8192)); controller.close(); } }) : bytes;
      return new Response(body, { headers: { "content-type": "application/json" } });
    },
  });
  const relay = startLocalBrowserRelay(`http://127.0.0.1:${upstream.port}`, uiOrigin, browserBrokerOracleEnvelope());
  let proof = relay.takeBrowserBootstrapProof();
  const request = async (path: string, method: string, body = "{}", streamed = false): Promise<Response> => {
    const current = proof;
    const next = randomBytes(32);
    expectedPath = path.slice("/_semio/hub".length);
    expectedMethod = method;
    expectedBody = method === "GET" ? "" : body;
    const requestBody = streamed ? new ReadableStream<Uint8Array>({ start(controller) { const bytes = new TextEncoder().encode(body); controller.enqueue(bytes.subarray(0, 512)); controller.enqueue(bytes.subarray(512)); controller.close(); } }) : body;
    const response = await fetch(`${relay.url}${path}`, {
      method,
      body: method === "GET" ? undefined : requestBody,
      redirect: "error",
      headers: {
        host: new URL(uiOrigin).host,
        origin: uiOrigin,
        referer: `${uiOrigin}/`,
        "sec-fetch-site": "same-origin",
        "x-semio-local-relay": relay.secret.toString("hex"),
        "x-semio-browser-broker": current.toString("hex"),
        "x-semio-browser-broker-next": browserBrokerProofDigest(next).toString("hex"),
        "content-type": "application/json",
      },
    });
    if (response.headers.get("x-semio-browser-broker-advanced") === "1") { current.fill(0); proof = next; } else next.fill(0);
    return response;
  };
  try {
    for (const row of fixture.routes) {
      const before = calls;
      const response = await request(row.path, row.method);
      if (response.status !== (row.admitted ? 200 : 404) || calls !== before + Number(row.admitted) || response.headers.has("x-semio-browser-broker-advanced") !== row.admitted) throw new Error(`inference live relay route mismatch: ${row.id}: status=${response.status} calls=${calls - before}`);
      await response.arrayBuffer();
    }
    const path = fixture.routes[0]!.path;
    const before = calls;
    const oversized = await request(path, "POST", "x".repeat(fixture.limits.requestBytes + 1));
    if (oversized.status !== 413 || calls !== before || oversized.headers.has("x-semio-browser-broker-advanced")) throw new Error("inference request overflow consumed proof or reached upstream");
    const streamed = await request(path, "POST", "x".repeat(fixture.limits.requestBytes + 1), true);
    if (streamed.status !== 413 || calls !== before || streamed.headers.get("x-semio-browser-broker-advanced") !== "1") throw new Error("inference streamed overflow reached upstream or failed its single proof advance");
    const exact = await request(path, "POST", "x".repeat(fixture.limits.requestBytes));
    if (exact.status !== 200 || calls !== before + 1) throw new Error("inference request exact bound rejected");
    await exact.arrayBuffer();
    for (const row of fixture.responses) {
      responseBytes = row.bytes;
      responseStreamed = row.streamed;
      const response = await request(path, "POST");
      if (response.status !== row.status || response.headers.get("x-semio-browser-broker-advanced") !== "1") throw new Error(`inference response bound mismatch: ${row.id}: ${response.status}`);
      const bytes = new Uint8Array(await response.arrayBuffer());
      if (row.status === 200 && (bytes.length !== row.bytes || bytes.some((byte) => byte !== 73) || response.headers.get("cache-control") !== "no-store")) throw new Error("inference response bytes or privacy changed");
    }
    console.log(`inference live relay routes=${fixture.routes.length} response-bounds=${fixture.responses.length} proof-ratchet=true upstream-auth=true`);
    return fixture.routes.length + fixture.responses.length + 3;
  } finally {
    proof.fill(0);
    await relay.stop();
    await upstream.stop(true);
  }
}

export class InferenceRelayCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("inference-relay-check accepts no arguments");
    console.log(`inference-relay-check: checks=${await proveInferenceRelay(this.repoRoot)}`);
  }
}

export class ExecutionTargetRelayCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length > 1 || (segments.length === 1 && segments[0] !== "--native")) throw new Error("execution-target-relay-check accepts only --native");
    console.log("execution-target-provider-frontier: checks=" + (await proveNativeArtifactProviderFrontier(this.repoRoot)));
    console.log(`execution-target-relay-check: checks=${await proveExecutionTargetRelay(this.repoRoot)}`);
    await proveBrowserBrokerRelay(this.repoRoot);
    console.log("execution-target-relay-check: existing browser proof-ratchet runtime regression clean");
    if (segments[0] === "--native") {
      const receipts = await runRepositoryExactCargoLaws({control:exactCargoCallerControlV1(this.invocation,"exact-cargo:caller",0),preparation: configuredRepositoryCargoPreparationPolicyV1(),invocation: this.invocation,policy: { ...configuredExactCargoLawPolicyV1(), buildMilliseconds: buildBudgetMs() },cwd: this.repoRoot,...exactCargoStageEnvironments(),groups: [
          {
            package: "semio-hub-boot-laws",
            target: { kind: "lib" },
            laws: ["configured_catalog_without_a_native_provider_fails_closed"],
          },
          {
            package: "semio-hub-open-plan-laws",
            target: { kind: "lib" },
            laws: ["execution_target_asset_routes_revalidate_scope_role_descriptor_and_catalog_before_each_body", "execution_target_selection_final_fence_matches_neutral_races"],
          },
        ],progress(event) {
          console.log(`execution-target-native ${event.stage}: ${event.law ?? ""} artifacts=${event.artifactDir}`);
        }});
      console.log(`execution-target-native-receipts: ${JSON.stringify(receipts)}`);
    }
  }
}

export class BrowserBrokerCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveBrowserBrokerRelay(this.repoRoot);
    console.log("[DEBUG] browser broker HTTP: actual expiry/replay/cancellation/redaction checks passed");
    await runVitest(this.root, ["--run", "-t", "binds acknowledged Shell session reads through the actual worker broker|requires an acknowledged private broker port before Shell session requests|closes the private broker proof before or during a session-authority read"], "../../🧪️testing/🎚️config/🟦️.ts");
    console.log("[DEBUG] browser-broker-check: actual HTTP expiry/replay/cancellation/redaction and current private worker session authority laws passed");
  }
}
