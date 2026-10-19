/** 🧭️ Admin relay boundary, backend and live-journey gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { type ScriptInvocation } from "@semio-tech/process/routing";
import { randomBytes } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { runCargo, runCmd, orchestratorBudgetOpts, runProbe } from "@semio-tech/process/execution";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { assertObservationExpectation, type ObservationExpectation } from "../../🧪️testing/🛂️expectation/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { finishLocalHub, type LocalHubRun, withPublishedLocalHubHostV1, startLocalHub, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { buildAdminSpa } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";
import { observedCount } from "../🌐️relay-checks/🟦️.ts";
import { ADMIN_RELAY_COOKIE, type AdminLiveJourneyFixture, LOCAL_RELAY_MAX_IN_FLIGHT, LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES, type LocalAdminRelay, startLocalAdminRelay } from "../../🚀️local-relay/🛰️servers/🟦️.ts";

function adminRelayOracleEnvelope(capability: string): Record<string, unknown> {
  return { schema: "semio.hub.local-credential-envelope/v1", clientClass: "admin-relay", capability, expiresAt: Date.now() + 60_000 };
}

async function adminRelayBootstrap(relay: LocalAdminRelay, proof: string): Promise<Response> {
  return fetch(`${relay.url}/__semio/admin/bootstrap`, {
    method: "POST",
    headers: { origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "x-semio-admin-bootstrap": proof },
    redirect: "error",
  });
}

async function proveAdminRelayBoundary(repoRoot: string): Promise<void> {
  const capability = `session.v1.${"3".repeat(32)}.${"4".repeat(64)}`;
  const upstream = {
    effects: 0,
    unsafeEffects: 0,
    staticAuthorization: "",
    status: 200,
    responseBytes: 0,
    blocked: false,
    started: 0,
    aborted: 0,
    releases: new Set<() => void>(),
    staticResponseBytes: 0,
    staticBlocked: false,
    staticStarted: 0,
    staticAborted: 0,
    staticReleases: new Set<() => void>(),
  };
  const upstreamServer = Bun.serve({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request): Promise<Response> {
      const url = new URL(request.url);
      const staticRequest = url.pathname.startsWith("/admin/") && !url.pathname.startsWith("/admin/api/");
      if (staticRequest) {
        upstream.staticAuthorization = request.headers.get("authorization") ?? "";
      } else {
        upstream.effects += 1;
        if (request.method === "POST") upstream.unsafeEffects += 1;
        if (request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("unauthorized", { status: 401 });
      }
      if (staticRequest ? upstream.staticBlocked : upstream.blocked) {
        if (staticRequest) upstream.staticStarted += 1;
        else upstream.started += 1;
        await new Promise<void>((resolveBlocked) => {
          const finish = (): void => {
            request.signal.removeEventListener("abort", abort);
            (staticRequest ? upstream.staticReleases : upstream.releases).delete(finish);
            resolveBlocked();
          };
          const abort = (): void => {
            if (staticRequest) upstream.staticAborted += 1;
            else upstream.aborted += 1;
            finish();
          };
          (staticRequest ? upstream.staticReleases : upstream.releases).add(finish);
          request.signal.addEventListener("abort", abort, { once: true });
        });
      }
      const responseBytes = staticRequest ? upstream.staticResponseBytes : upstream.responseBytes;
      if (responseBytes > 0) return new Response(new Uint8Array(responseBytes), { status: upstream.status, headers: { "content-type": "application/octet-stream" } });
      if (staticRequest) return new Response("<main>admin</main>", { headers: { "content-type": "text/html" } });
      return new Response(JSON.stringify({ ok: true }), { status: upstream.status, headers: { "content-type": "application/json" } });
    },
  });
  const hubOrigin = `http://127.0.0.1:${upstreamServer.port}`;
  let relay: LocalAdminRelay | undefined;
  try {
    const proof = randomBytes(32);
    const proofHex = proof.toString("hex");
    relay = startLocalAdminRelay(hubOrigin, adminRelayOracleEnvelope(capability), Buffer.from(proof));
    const staticResponse = await fetch(`${relay.url}/admin/`);
    if (!staticResponse.ok || upstream.staticAuthorization !== "" || (await staticResponse.text()) !== "<main>admin</main>") throw new Error("admin relay static shell carried administrator authority");
    upstream.staticResponseBytes = LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES;
    const exactStatic = await fetch(`${relay.url}/admin/static-exact.js`);
    if (!exactStatic.ok || (await exactStatic.arrayBuffer()).byteLength !== LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES) throw new Error("admin relay rejected its exact static response ceiling");
    upstream.staticResponseBytes = LOCAL_RELAY_MAX_STATIC_RESPONSE_BYTES + 1;
    const oversizedStatic = await fetch(`${relay.url}/admin/static-oversized.js`);
    upstream.staticResponseBytes = 0;
    if (oversizedStatic.status !== 503) throw new Error("admin relay static response max+1 law failed");
    const rawLocal = await fetch(`${relay.url}/admin/api/overview`);
    if (rawLocal.status !== 401 || observedCount(upstream.effects) !== 0 || (await rawLocal.text()).includes(capability)) throw new Error("raw local caller crossed the admin relay cookie boundary");
    const bootstrapped = await adminRelayBootstrap(relay, proofHex);
    const setCookie = bootstrapped.headers.get("set-cookie") ?? "";
    const cookie = setCookie.match(new RegExp(`^${ADMIN_RELAY_COOKIE}=([0-9a-f]{64});`, "u"))?.[1];
    if (
      bootstrapped.status !== 204 ||
      !cookie ||
      !setCookie.includes("HttpOnly") ||
      !setCookie.includes("SameSite=Strict") ||
      !setCookie.includes("Path=/") ||
      /(?:^|;)\s*Domain=/iu.test(setCookie) ||
      setCookie.includes(capability) ||
      (await bootstrapped.text()).includes(proofHex)
    )
      throw new Error("admin relay did not issue an opaque host-only HttpOnly strict cookie");
    const replay = await adminRelayBootstrap(relay, proofHex);
    if (replay.status !== 401 || observedCount(upstream.effects) !== 0) throw new Error("admin relay replayed its one-use fragment bootstrap");
    const admitted = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const admittedText = await admitted.text();
    if (!admitted.ok || admittedText !== '{"ok":true}' || admittedText.includes(capability) || observedCount(upstream.effects) !== 1) throw new Error("admin relay did not proxy an opaque-cookie request with memory-only authority");
    const intentBody = JSON.stringify({ kind: "rebuild-directory-projections", requestId: "relay-oracle:rebuild", expectedHeadSeq: 0 });
    const forgedUnsafe = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, "content-type": "application/json" }, body: intentBody });
    if (forgedUnsafe.status !== 401 || observedCount(upstream.unsafeEffects) !== 0) throw new Error("admin relay admitted a cross-origin unsafe request");
    const admittedUnsafe = await fetch(`${relay.url}/admin/api/intents`, {
      method: "POST",
      headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "content-type": "application/json" },
      body: intentBody,
    });
    if (!admittedUnsafe.ok || observedCount(upstream.unsafeEffects) !== 1) throw new Error("admin relay rejected its same-origin unsafe request");
    const paged = await fetch(`${relay.url}/admin/api/connections?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!paged.ok || observedCount(upstream.effects) !== 3) throw new Error("admin relay rejected an exact bounded page query");
    const documents = await fetch(`${relay.url}/admin/api/documents?limit=100&space=studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!documents.ok || observedCount(upstream.effects) !== 4) throw new Error("admin relay rejected an exact bounded document projection query");
    const spaces = await fetch(`${relay.url}/admin/api/spaces?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!spaces.ok || observedCount(upstream.effects) !== 5) throw new Error("admin relay rejected an exact bounded space projection query");
    const space = await fetch(`${relay.url}/admin/api/spaces/studio?limit=100`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!space.ok || observedCount(upstream.effects) !== 6) throw new Error("admin relay rejected an exact bounded space-member projection query");
    const arbitraryQuery = await fetch(`${relay.url}/admin/api/connections?limit=100&scope=all`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedDocuments = await fetch(`${relay.url}/admin/api/documents?space=studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedSpaces = await fetch(`${relay.url}/admin/api/spaces`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const unboundedSpace = await fetch(`${relay.url}/admin/api/spaces/studio`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const legacyMutation = await fetch(`${relay.url}/admin/api/directory/rebuild`, {
      method: "POST",
      headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin" },
    });
    if (arbitraryQuery.status !== 404 || unboundedDocuments.status !== 404 || unboundedSpaces.status !== 404 || unboundedSpace.status !== 404 || legacyMutation.status !== 404 || observedCount(upstream.effects) !== 6)
      throw new Error("admin relay admitted an arbitrary, unbounded, or legacy request");

    const sameOriginHeaders = { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}`, origin: relay.url, referer: `${relay.url}/admin/`, "sec-fetch-site": "same-origin", "content-type": "application/json" };
    const beforeLimits = upstream.effects;
    const exactBody = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: sameOriginHeaders, body: new Uint8Array(8 * 1024) });
    const oversizedBody = await fetch(`${relay.url}/admin/api/intents`, { method: "POST", headers: sameOriginHeaders, body: new Uint8Array(8 * 1024 + 1) });
    if (!exactBody.ok || oversizedBody.status !== 413 || observedCount(upstream.effects) !== beforeLimits + 1) throw new Error("admin relay request-body max+1 law failed");
    upstream.responseBytes = 64 * 1024;
    const exactResponse = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    if (!exactResponse.ok || (await exactResponse.arrayBuffer()).byteLength !== 64 * 1024) throw new Error("admin relay rejected its exact response ceiling");
    upstream.responseBytes = 64 * 1024 + 1;
    const oversizedResponse = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    upstream.responseBytes = 0;
    if (oversizedResponse.status !== 503 || observedCount(upstream.effects) !== beforeLimits + 3) throw new Error("admin relay response max+1 law failed");

    upstream.blocked = true;
    upstream.started = 0;
    const abortController = new AbortController();
    const abortedRequest = fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` }, signal: abortController.signal }).catch(() => undefined);
    for (let attempt = 0; observedCount(upstream.started) !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    if (observedCount(upstream.started) !== 1) throw new Error("admin relay downstream-abort law did not reach upstream");
    const abortedBefore = upstream.aborted;
    abortController.abort();
    await abortedRequest;
    for (let attempt = 0; upstream.aborted === abortedBefore && attempt < 200; attempt += 1) await Bun.sleep(5);
    if (observedCount(upstream.aborted) !== abortedBefore + 1) throw new Error("admin relay did not propagate downstream cancellation upstream");

    upstream.staticBlocked = true;
    upstream.staticStarted = 0;
    const staticAbortController = new AbortController();
    const abortedStatic = fetch(`${relay.url}/admin/static-abort.js`, { signal: staticAbortController.signal }).catch(() => undefined);
    for (let attempt = 0; observedCount(upstream.staticStarted) !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    const staticAbortedBefore = upstream.staticAborted;
    staticAbortController.abort();
    await abortedStatic;
    for (let attempt = 0; upstream.staticAborted === staticAbortedBefore && attempt < 200; attempt += 1) await Bun.sleep(5);
    upstream.staticBlocked = false;
    if (observedCount(upstream.staticStarted) !== 1 || observedCount(upstream.staticAborted) !== staticAbortedBefore + 1) throw new Error("admin relay did not propagate static downstream cancellation upstream");

    upstream.started = 0;
    const saturationUrl = `${relay.url}/admin/api/overview`;
    const saturated = Array.from({ length: LOCAL_RELAY_MAX_IN_FLIGHT }, () => fetch(saturationUrl, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } }));
    for (let attempt = 0; observedCount(upstream.started) !== LOCAL_RELAY_MAX_IN_FLIGHT && attempt < 400; attempt += 1) await Bun.sleep(5);
    if (observedCount(upstream.started) !== LOCAL_RELAY_MAX_IN_FLIGHT) throw new Error("admin relay did not fill its exact in-flight boundary");
    const overflow = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${cookie}` } });
    const staticOverflow = await fetch(`${relay.url}/admin/static-overflow.js`);
    if (overflow.status !== 503 || staticOverflow.status !== 503) throw new Error("admin relay admitted max+1 API or static request outside its aggregate boundary");
    upstream.blocked = false;
    for (const release of [...upstream.releases]) release();
    if ((await Promise.all(saturated)).some((response) => !response.ok)) throw new Error("admin relay rejected an admitted in-flight request");

    upstream.staticBlocked = true;
    upstream.staticStarted = 0;
    const stoppedRequest = fetch(`${relay.url}/admin/static-stop.js`).catch(() => undefined);
    for (let attempt = 0; observedCount(upstream.staticStarted) !== 1 && attempt < 200; attempt += 1) await Bun.sleep(5);
    const stopAbortedBefore = upstream.staticAborted;
    await relay.stop();
    await stoppedRequest;
    if (observedCount(upstream.staticStarted) !== 1 || observedCount(upstream.staticAborted) !== stopAbortedBefore + 1)
      throw new Error(`admin relay stop did not cancel active static upstream ownership: started=${upstream.staticStarted} aborted=${upstream.staticAborted} before=${stopAbortedBefore}`);
    upstream.staticBlocked = false;

    upstream.effects = 0;
    const expiringProof = randomBytes(32);
    const expiringProofHex = expiringProof.toString("hex");
    relay = startLocalAdminRelay(hubOrigin, adminRelayOracleEnvelope(capability), Buffer.from(expiringProof), 250, 250);
    const expiringBootstrap = await adminRelayBootstrap(relay, expiringProofHex);
    const expiringCookie = (expiringBootstrap.headers.get("set-cookie") ?? "").match(new RegExp(`^${ADMIN_RELAY_COOKIE}=([0-9a-f]{64});`, "u"))?.[1];
    if (!expiringCookie) throw new Error("admin relay expiry oracle lacked its opaque cookie");
    await Bun.sleep(300);
    const expired = await fetch(`${relay.url}/admin/api/overview`, { headers: { cookie: `${ADMIN_RELAY_COOKIE}=${expiringCookie}` } });
    if (expired.status !== 401 || observedCount(upstream.effects) !== 0) throw new Error("expired admin relay cookie reached upstream");

    const sessionSource = readFileSync(join(repoRoot, "🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx"), "utf8");
    const viteSource = readFileSync(join(repoRoot, "🌎️hub/🔨️modules/🛡️admin/🏗️builder/🌐️vite/🟦️.ts"), "utf8");
    const adminDevCommand = JSON.parse(readFileSync(join(repoRoot, "🌎️hub/📦️packages/🦀️rust/📋️project.json"), "utf8")).targets?.["dev-secure-admin"]?.options?.command;
    if (sessionSource.includes("sessionStorage") || sessionSource.includes("headers.authorization") || sessionSource.includes("Bearer ${") || !sessionSource.includes("#semio-admin=") || !sessionSource.includes('credentials: "same-origin"'))
      throw new Error("admin SPA regained a browser-owned bearer carrier");
    if (viteSource.includes('"/admin/api": hubProxy') || adminDevCommand !== "bun ./📜️script.ts dev secure-admin") throw new Error("admin relay dev target ownership drift");
  } finally {
    await relay?.stop();
    await upstreamServer.stop(true);
  }
}

/** 🧬 Validates the literal cross-language journey before it may drive a real process/browser. */
async function adminLiveJourneyFixture(repoRoot: string): Promise<AdminLiveJourneyFixture> {
  const fixtureRoot = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🚶️admin-live-journey-v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as AdminLiveJourneyFixture;
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryProfileV1")(fixture.profile)) throw new Error("admin live journey profile is not the owned directory profile contract");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/AdminCreateSpaceIntentV1")(fixture.mutation)) throw new Error("admin live journey mutation is not the owned create-space intent contract");
  if (!hubSchemaExport(repoRoot, "schema://hub.directory/AdminOperationIntentV1")(fixture.operation)) throw new Error("admin live journey operation is not the owned admin operation intent contract");
  if (fixture.schema !== "semio.hub.admin-live-journey/v1" || fixture.limits.responseBytes !== 65536) throw new Error("admin live journey envelope drift");
  if (fixture.limits.journeyMs < 1000 || fixture.limits.journeyMs > 60000 || fixture.limits.pollMs < 10 || fixture.limits.pollMs > 5000) throw new Error("admin live journey bounds drift");
  if (fixture.languages.length !== 2) throw new Error("admin live journey requires exactly two languages with no default");
  const admissionRoot = join(repoRoot, "🌎️hub/🚀️local-bootstrap/🧪️tests/🧫️fixtures/⏳️idle-admission-v1");
  const admissionDocument = JSON.parse(readFileSync(join(admissionRoot, "🔣️.json"), "utf8")) as Record<string, unknown> & { hostile: readonly (ObservationExpectation & { id: string; mutation: Record<string, unknown> })[] };
  const { hostile: admissionHostile, ...admissionContract } = admissionDocument;
  const admission = admissionContract as unknown as { exchangeDeadlineMs: number; idleBeforeAdmissionMs: number; frameHex: string; payloadHex: string };
  const validateAdmission = hubSchemaExport(repoRoot, "schema://hub.local-bootstrap/LocalBootstrapIdleAdmissionV1");
  if (!validateAdmission(admission)) throw new Error("local bootstrap idle-admission fixture violates schema://hub.local-bootstrap/LocalBootstrapIdleAdmissionV1");
  if (admissionHostile.length !== 3 || new Set(admissionHostile.map((entry) => entry.id)).size !== admissionHostile.length) throw new Error("local bootstrap idle-admission hostile table drift");
  for (const entry of admissionHostile) assertObservationExpectation(`local-bootstrap idle-admission/${entry.id}`, entry, validateAdmission({ ...admission, ...entry.mutation }));
  const frame = Buffer.from(admission.frameHex, "hex");
  const payload = Buffer.from(admission.payloadHex, "hex");
  if (admission.idleBeforeAdmissionMs <= admission.exchangeDeadlineMs || frame.readUInt32BE(0) !== payload.byteLength || !frame.subarray(4).equals(payload)) throw new Error("local bootstrap idle/admitted-frame oracle mismatch");
  const locales = fixture.languages
    .map(({ locale }) => locale)
    .sort()
    .join(",");
  if (locales !== "de,en" || fixture.languages[0]?.overview === fixture.languages[1]?.overview) throw new Error("admin live journey bilingual inventory drift");
  if (Buffer.byteLength(JSON.stringify(fixture.mutation), "utf8") > 8 * 1024 || Buffer.byteLength(JSON.stringify(fixture.operation), "utf8") > 8 * 1024) throw new Error("admin live journey intent exceeded relay bound");
  console.log("admin-live-journey fixture: hub.directory exports 3/3; hub.local-bootstrap idle-admission 1/1 with hostile 3/3; idle/admitted frame 2/2; bilingual inventory 2/2; bounded intents 2/2");
  return fixture;
}

/** 🎭 Exercises the protected relay, real SQLite hub and shipped SPA in one bounded Chromium journey. */
async function proveAdminLiveJourney(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,fixture:AdminLiveJourneyFixture): Promise<void> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const profile: LocalProfile = { ...fixture.profile, allowedClientClasses: ["admin-relay"] };
  const deadline = Date.now() + fixture.limits.journeyMs;
  const remaining = (): number => {
    const value = deadline - Date.now();
    if (value <= 0) throw new Error("admin live journey deadline exceeded");
    return value;
  };
  let run: LocalHubRun | undefined;
  let relay: LocalAdminRelay | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const retainBrowserDiagnostic = (value: string): void => {
    browserDiagnostics.push(value.slice(0, 512));
    if (browserDiagnostics.length > 16) browserDiagnostics.shift();
  };
  try {
    run = await startLocalHub(host, [profile], {
      capture: true,
      isolatedSecuritySmoke: true,
      adminSubjects: [`semio.local.bootstrap/v1:${fixture.profile.subject}`],
    });
    const readiness = await waitForReadiness(run, true);
    if (readiness.directory?.ready !== true) throw new Error("admin live journey directory was not ready");
    const envelope = await issueLocalCredential(run, fixture.profile.profileId, "admin-relay");
    const proof = randomBytes(32);
    const proofHex = proof.toString("hex");
    relay = startLocalAdminRelay(`http://127.0.0.1:${run.port}`, envelope, proof);
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ locale: "fr-FR" });
    const page = await context.newPage();
    page.on("console", (message) => retainBrowserDiagnostic(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => retainBrowserDiagnostic(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => retainBrowserDiagnostic(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    page.on("response", (response) => {
      if (response.status() >= 400) retainBrowserDiagnostic(`response:${response.status()}:${response.url()}`);
    });
    page.setDefaultTimeout(Math.min(10_000, remaining()));
    await page.goto(`${relay.url}/admin/#semio-admin=${proofHex}`, { waitUntil: "domcontentloaded", timeout: remaining() });
    await page.getByRole("dialog", { name: "Language · Sprache" }).waitFor();
    await page.getByRole("button", { name: "English", exact: true }).click();
    await page.locator("#admin-tab-overview").waitFor();
    if (new URL(page.url()).hash !== "") throw new Error("admin live journey retained the one-use bootstrap fragment");

    const request = async (method: "GET" | "POST", path: string, body?: unknown): Promise<{ status: number; bytes: number; value: any }> => {
      const result = await page.evaluate(
        async ({ method, path, body }) => {
          const response = await fetch(path, {
            method,
            credentials: "same-origin",
            headers: body === undefined ? undefined : { "content-type": "application/json" },
            body: body === undefined ? undefined : JSON.stringify(body),
            signal: AbortSignal.timeout(2_000),
          });
          const text = await response.text();
          return { status: response.status, bytes: new TextEncoder().encode(text).byteLength, text };
        },
        { method, path, body },
      );
      if (result.bytes > fixture.limits.responseBytes) throw new Error(`admin live journey response exceeded ${fixture.limits.responseBytes} bytes`);
      let value: any;
      try {
        value = result.text.length === 0 ? undefined : JSON.parse(result.text);
      } catch {
        throw new Error(`admin live journey ${method} ${path} returned non-JSON status ${result.status}`);
      }
      return { status: result.status, bytes: result.bytes, value };
    };

    const english = fixture.languages.find(({ locale }) => locale === "en")!;
    if ((await page.locator("#admin-tab-overview").textContent())?.trim() !== english.overview || (await page.locator("#admin-tab-spaces").textContent())?.trim() !== english.spaces) throw new Error("admin live journey English navigation mismatch");
    const overview = await request("GET", "/admin/api/overview");
    if (overview.status !== 200 || overview.value?.backends?.sqlite !== true || !Number.isSafeInteger(overview.value?.headSeq)) throw new Error("admin live journey bounded SQLite overview mismatch");

    const created = await request("POST", "/admin/api/intents", fixture.mutation);
    if (created.status !== 200 || created.value?.state !== "succeeded" || created.value?.outcome?.code !== "directory-events-appended" || created.value?.outcome?.durable !== true) throw new Error("admin live journey create-space receipt mismatch");
    if (!existsSync(join(run.runRoot, "data", "directory.db"))) throw new Error("admin live journey did not durably materialize the SQLite directory");
    await page.locator("#admin-tab-spaces").click();
    await page.getByText(fixture.mutation.name, { exact: true }).waitFor();
    if ((await page.locator("#admin-space-create-open").textContent())?.trim() !== english.newSpace) throw new Error("admin live journey English mutation surface mismatch");

    const current = await request("GET", "/admin/api/overview");
    const rebuild = await request("POST", "/admin/api/intents", { ...fixture.operation, expectedHeadSeq: current.value?.headSeq });
    if (rebuild.status !== 202 || rebuild.value?.state !== "accepted" || typeof rebuild.value?.operationId !== "string") throw new Error("admin live journey rebuild acceptance mismatch");
    const operationPath = `/admin/api/operations/${encodeURIComponent(rebuild.value.operationId)}`;
    const cancellation = await request("POST", `${operationPath}/cancel`);
    if (cancellation.status !== 200 && cancellation.status !== 409) throw new Error("admin live journey cancellation admission mismatch");
    if (cancellation.status === 200 && cancellation.value?.progress?.cancelRequested !== true) throw new Error("admin live journey cancellation was not observable");
    let completed = -1;
    let terminal: any;
    while (Date.now() < deadline) {
      const status = await request("GET", operationPath);
      if (status.status !== 200 || typeof status.value?.receipt?.state !== "string") throw new Error("admin live journey operation polling mismatch");
      const progress = status.value.progress;
      if (progress) {
        if (!Number.isSafeInteger(progress.completedEvents) || progress.completedEvents < completed || progress.completedEvents > progress.totalEvents) throw new Error("admin live journey operation progress was not bounded monotonic state");
        completed = progress.completedEvents;
      }
      if (status.value.receipt.state !== "accepted") {
        terminal = status.value.receipt;
        break;
      }
      await Bun.sleep(Math.min(fixture.limits.pollMs, remaining()));
    }
    if (!terminal || (terminal.state !== "cancelled" && terminal.state !== "succeeded") || (cancellation.status === 200 && terminal.state !== "cancelled"))
      throw new Error(`admin live journey operation lacked a valid terminal: cancel=${cancellation.status}:${JSON.stringify(cancellation.value)} terminal=${JSON.stringify(terminal)}`);

    await page.locator("#admin-locale-switch").click();
    await page.getByRole("option", { name: "DE", exact: true }).click();
    const german = fixture.languages.find(({ locale }) => locale === "de")!;
    if (
      (await page.locator("#admin-tab-overview").textContent())?.trim() !== german.overview ||
      (await page.locator("#admin-tab-spaces").textContent())?.trim() !== german.spaces ||
      (await page.locator("#admin-space-create-open").textContent())?.trim() !== german.newSpace
    )
      throw new Error("admin live journey German navigation mismatch");
    const spaces = await request("GET", "/admin/api/spaces?limit=100");
    if (spaces.status !== 200 || !Array.isArray(spaces.value?.rows) || !spaces.value.rows.some((space: any) => space.name === fixture.mutation.name)) throw new Error("admin live journey bounded created-space read mismatch");
    console.log(`admin-live-journey: SQLite overview/create/read, EN/DE UI, operation poll, and ${terminal.state === "cancelled" ? "cancel" : "already-terminal cancel race"} passed`);
  } catch (error) {
    const diagnostics = run?.output().slice(-4_096) ?? "";
    const browserOutput = browserDiagnostics.join("\n");
    throw new Error(`${error instanceof Error ? error.message : "admin live journey failed"}${browserOutput ? `\nbrowser diagnostics:\n${browserOutput}` : ""}${diagnostics ? `\nhub diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    await browser?.close().catch(() => undefined);
    await relay?.stop().catch(() => undefined);
    if (run) await finishLocalHub(run);
  }
  });
}

export class AdminRelayCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveAdminRelayBoundary(this.repoRoot);
    runCmd("bun", ["repo", "run", "os-hub-admin:test", "--skip-cache", "--", "long", "--run", "🛡️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    console.log("admin-relay-check: one-use fragment bootstrap, host-only HttpOnly strict cookie, expiry/replay/CSRF/raw-local denial, bearer redaction, and EN/DE UI laws passed");
  }
}

export class AdminBackendCheckScript extends BundleScript {
  async run(): Promise<void> {
    runCmd("bun", [join(this.repoRoot, "🌎️hub/📇️directory/🧪️tests/🎯️admin-intent-v1/🟦️.ts")], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    const exactLaw = (target: string[], suffix: string): string => {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "--all-features", ...target, suffix, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout
        .split("\n")
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -": test".length))
        .filter((name) => name.endsWith(suffix));
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`admin-backend-check expected exactly one ${suffix} law, selected ${matches.length}`);
      return matches[0]!;
    };
    const laws = [
      { target: ["-p", "semio-hub-directory-laws", "--lib"], suffix: "admin_operation_audit_concurrent_first_writer_is_idempotent_and_first_terminal_wins" },
      { target: ["-p", "semio-hub-directory-laws", "--lib"], suffix: "admin_bounded_overview_space_and_document_projections_enforce_exact_page_boundary" },
      { target: ["-p", "semio-hub-directory-route-laws", "--lib"], suffix: "admin_intent_wire_taxonomy_rejects_generic_and_unknown_commands" },
      { target: ["-p", "semio-hub-directory-route-laws", "--lib"], suffix: "admin_document_cursor_is_principal_route_and_exact_page_bound" },
      { target: ["-p", "semio-hub-hostile-input-laws", "--lib"], suffix: "admin_response_pages_stop_before_exact_byte_max_and_reject_one_oversized_row" },
      { target: ["-p", "semio-hub-directory-route-laws", "--lib"], suffix: "admin_rebuild_slots_are_atomic_and_abort_closes_once" },
    ].map((law) => ({ ...law, name: exactLaw(law.target, law.suffix) }));
    const postgresLaw = { target: ["-p", "semio-hub-directory-backend-laws", "--lib"], suffix: "admin_operation_audit_concurrent_absent_request_rereads_established_receipt" };
    const postgres = { ...postgresLaw, name: exactLaw(postgresLaw.target, postgresLaw.suffix) };
    console.log(`admin-backend-laws: ${[...laws, postgres].map((law) => law.name).join(",")}`);
    for (const law of laws) runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...law.target, law.name, "--", "--exact", "--test-threads=1"], this.root);
    await proveAdminRelayBoundary(this.repoRoot);
    runCmd("bun", ["repo", "run", "os-hub-admin:test", "--skip-cache", "--", "long", "--run", "🛡️admin.test.tsx"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", ...postgres.target, postgres.name, "--", "--exact", "--test-threads=1"], this.root);
    console.log("admin-backend-check: closed intents, verified principal, durable audit, bounded snapshots, rebuild lifecycle, relay, and SPA laws passed");
  }
}

export class AdminLiveJourneyCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = await adminLiveJourneyFixture(this.repoRoot);
    const laws: [string, string][] = [["semio-hub-local-bootstrap", "tests::local_bootstrap_idle_listener_survives_until_admission_and_admitted_frame_is_deadline_bounded"], ["semio-hub-directory-backend-laws", "sqlite_tests::projection_rebuild_preserves_live_credential_invite_and_session_bindings"]];
    for (const [owner, law] of laws) {
      const listed = runProbe("cargo", ["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", law, "--", "--list"], { cwd: this.root, ...orchestratorBudgetOpts() });
      const matches = listed.stdout.split("\n").filter((line) => line === `${law}: test`);
      if (listed.status !== 0 || matches.length !== 1) throw new Error(`admin-live-journey-check expected exactly one ${law} law, selected ${matches.length}`);
      runCargo(["test", "--manifest-path", "Cargo.toml", "-p", owner, "--lib", law, "--", "--exact", "--test-threads=1"], this.root);
    }
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml", "--bin", "os-hub"], this.root);
    await proveAdminLiveJourney(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:admin-journey",0),fixture);
    console.log("admin-live-journey-check: protected loopback bootstrap/relay and real SQLite browser journey passed; no production OIDC claim");
  }
}
