//#region 🔖️CatalogSmokeVerify
import { LocalHubHostLedgerV1 } from "../../🚀️local-bootstrap/🎛️host/🟦️.ts";
import type { CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { requireIssuedResourceControlInvocationV1, withRequestedIssuedResourceControlV1, type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {readNativeCallerAuthorityV1} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/📋️native-orchestration/📥️invocation/🟦️.ts";
import {repositoryCargoNetworkV1} from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🟦️.ts";
import { validateAndPublishTrustedStdioGisCandidate } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🟦️.ts";
import { withMaterializedTrustedCatalogBundleV1 } from "../../🏗️bootstrap/🧾️provenance/🚚️bootstrap-publication/🧱️materialized-bundle/🟦️.ts";
import {trustedBootstrapSelectPackages} from "../../🏗️bootstrap/🧾️provenance/🧭️bootstrap-selection/🟦️.ts";
/** 🧩️ Semantic collaboration verification owner. */

import { repoCacheDirectory } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";

import { mkdirSync } from "node:fs";

import { join } from "node:path";

import { generatePluginRegistry } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🔎️discovery/🟦️.ts";
import { COLLAB_E2E_LOCALES, COLLAB_E2E_PREBUILD_BUDGET_MS, COLLAB_E2E_STEP_NAMES, COLLAB_E2E_USER1_EMAIL, COLLAB_E2E_USER1_PASSWORD, COLLAB_E2E_USER2_EMAIL, COLLAB_E2E_USER2_PASSWORD, type CollabLinkRelay, type CollabLocale, collabRecorder, collabStartLinkRelay, type CollabStepOutcome, collabVerdict, repoRoot } from "./⚙️constants-relay-and-recorder/🟦️.ts";
import { collabActivateShellRuntime, type CollabDevServerV1, collabExternalAdminCapability, collabExternalHub, collabOutDir, collabScopesV1, collabScopeV1, type CollabScopeV1, collabStartUserDevServer, finishCollabDevServerV1 } from "./🧭️scope-and-dev-servers/🟦️.ts";
import { collabHubDataDir, type CollabHubHandle, collabProvisionCredentials, collabScanPort, collabSignIn, collabStartHub } from "./🏠️hub-start-and-credentials/🟦️.ts";
import { collabRouteMisses, collabSpaceReopenedAt, collabWaitForApp } from "./🖱️ui-driving-and-row-waits/🟦️.ts";
import { collabCountCommandFrames, collabRecordSentCommands } from "./📡️command-frame-recording/🟦️.ts";
import { collabRunScenario } from "./🎬️scenario-run/🟦️.ts";
import { collabRunRestartStep } from "./🔁️restart-and-convergence/🟦️.ts";
import { collabRunCollaborationBehaviours } from "./🧪️collaboration-behaviours/🟦️.ts";

import {buildComponentClosureV1} from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/🟦️.ts";

import { PLAYWRIGHT_MODULE_SPECIFIER } from "../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/📋️plan/🟦️.ts";

import { withBuiltLocalHubHostV1, withLocalHubHostTaskV1, LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";

import { acceptanceCheckResult, publishAcceptanceCheckResult, withAcceptanceRecord } from "@semio-tech/repo-test/acceptance/orchestration";

//#endregion 🔖️CatalogSmokeVerify
//#region 🔖️CollabE2e
/** 🎯️ The finite host set discovered by the framework plugin registry and the writer component used by this collaboration scenario. */
let collabRequiredPluginIdsCache: readonly string[] | undefined;
function collabRequiredPluginIds(): readonly string[] {
  collabRequiredPluginIdsCache ??= Object.freeze([...new Set([...generatePluginRegistry(repoRoot).filter(entry=>entry.host).map(entry=>entry.pluginId),"writer"])]);
  return collabRequiredPluginIdsCache;
}

/** 🧱️ Produces the actual selected-mode browser closure under the caller's live Original. */
async function collabPrebuildPlugins(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);
 if(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==repoRoot)throw Error("Actual collaboration workspace required");
 await withRequestedIssuedResourceControlV1(phaseControl,COLLAB_E2E_PREBUILD_BUDGET_MS,control=>buildComponentClosureV1(repoRoot,new Set(collabRequiredPluginIds()),{invocation,control,facet:"browser",network:repositoryCargoNetworkV1(invocation,repoRoot)}));
}

/** 🎬️ Orchestrates the full harness: port scan, temp data dirs, hub boot, plugin prebuild, two `s`
 * react dev servers, two independent Playwright browser contexts, the 10-step scenario, and teardown of
 * every spawned process (hub + both dev servers + browser) even on failure. Writes `STEP n: PASS/FAIL`
 * lines plus a final summary, and sets a non-zero exit code if any step failed. */
async function runCollabE2eVerifyOwnedV1(opts: { readonly invocation:ScriptInvocation; readonly phaseControl:CompilerAdmissionControl; readonly hub: string | undefined; readonly locale: CollabLocale; readonly check: string }): Promise<void> {
  requireIssuedResourceControlInvocationV1(opts.phaseControl,opts.invocation);
  if(readNativeCallerAuthorityV1(opts.invocation.capabilities).repositoryRoot!==repoRoot)throw Error("Actual collaboration workspace required");
  const startedAt = new Date();
  const outDir = collabOutDir();
  const external = collabExternalHub(opts.hub);
  const taken = new Set<number>();
  const hubPort = external ? Number(new URL(external.baseUrl).port) : collabScanPort("S_COLLAB_HUB_PORT", taken);
  const user1Port = collabScanPort("S_COLLAB_USER1_PORT", taken);
  const user2Port = collabScanPort("S_COLLAB_USER2_PORT", taken);
  console.log(`[collab-e2e] ports: hub=${hubPort}${external ? " (external)" : ""} user1=${user1Port} user2=${user2Port}`);

  const hubDataDir = external ? "" : await collabHubDataDir();
  const user1DataDir = join(outDir,"user1");mkdirSync(user1DataDir);
  const user2DataDir = join(outDir,"user2");mkdirSync(user2DataDir);

  const passwords: Readonly<Record<string, string>> = external
    ? external.passwords
    : (await collabProvisionCredentials(), { [COLLAB_E2E_USER1_EMAIL]: COLLAB_E2E_USER1_PASSWORD, [COLLAB_E2E_USER2_EMAIL]: COLLAB_E2E_USER2_PASSWORD });

  let hubDaemon: CollabHubHandle | undefined;
  let user1Daemon: CollabDevServerV1 | undefined;
  let user2Daemon: CollabDevServerV1 | undefined;
  let browser: import("playwright").Browser | undefined;
  let relays: readonly [CollabLinkRelay, CollabLinkRelay] | undefined;
  const results: CollabStepOutcome[] = [];
  const record = collabRecorder(results);

  const teardown=async():Promise<void>=>{
   const failures:unknown[]=[];try{await browser?.close();}catch(error){failures.push(error);}for(const relay of relays??[])try{await relay.close();}catch(error){failures.push(error);}for(const daemon of[user1Daemon,user2Daemon])if(daemon)try{await finishCollabDevServerV1(daemon);}catch(error){failures.push(error);}if(hubDaemon)try{await hubDaemon.kill();}catch(error){failures.push(error);}if(failures.length===1)throw failures[0];if(failures.length)throw new AggregateError(failures,"Collaboration browser, relays and child cleanup failed");
  };
  let primary:{error:unknown}|undefined;

  try {
    if (!external) try {
      hubDaemon = await collabStartHub(collabScopeV1().host,hubPort,"boot");
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] hub failed to boot — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= COLLAB_E2E_STEP_NAMES.length; step++) record(step, false, `blocked — hub never became ready: ${message}`);
      throw error;
    }

    try {
      if (process.env.S_COLLAB_SKIP_PREBUILD === "1") {
        console.log("[collab-e2e] S_COLLAB_SKIP_PREBUILD=1 — skipping plugin prebuild");
      } else {
        await collabPrebuildPlugins(opts.invocation,opts.phaseControl);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] plugin prebuild failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= COLLAB_E2E_STEP_NAMES.length; step++) record(step, false, `blocked — plugin prebuild failed: ${message}`);
      throw error;
    }

    const hubBaseUrl = external?.baseUrl ?? `http://127.0.0.1:${hubPort}`;
    try {
      if (process.env.S_COLLAB_SKIP_ACTIVATE === "1") {
        console.log("[collab-e2e] S_COLLAB_SKIP_ACTIVATE=1 — skipping activate-s-react-prototype");
      } else {
        collabActivateShellRuntime();
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] shell activation failed — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= COLLAB_E2E_STEP_NAMES.length; step++) record(step, false, `blocked — shell activation failed: ${message}`);
      throw error;
    }

    try {
      relays = [await collabStartLinkRelay(hubBaseUrl), await collabStartLinkRelay(hubBaseUrl)];
      [user1Daemon, user2Daemon] = await Promise.all([
        collabStartUserDevServer({ port: user1Port, hubUrl: relays[0].url, user: COLLAB_E2E_USER1_EMAIL }),
        collabStartUserDevServer({ port: user2Port, hubUrl: relays[1].url, user: COLLAB_E2E_USER2_EMAIL }),
      ]);
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      console.error(`[collab-e2e] a shell dev server never booted — every scenario step is reported FAIL: ${message}`);
      for (let step = 1; step <= COLLAB_E2E_STEP_NAMES.length; step++) record(step, false, `blocked — shells did not boot: ${message}`);
      throw error;
    }

    // 🎭️ Matches `SetupScript`'s own install location (the shared cache root's `tools/ms-playwright`) —
    // without this, Playwright falls back to the OS default cache (`~/Library/Caches/ms-playwright`),
    // which can hold a different/older browser revision than the one this repo's `playwright` version
    // expects (confirmed during this lane's own iteration: the default cache had `chromium-1223`, this
    // repo's `playwright` wanted `chromium_headless_shell-1234`, which only exists under the repo-scoped path).
    process.env.PLAYWRIGHT_BROWSERS_PATH = process.env.PLAYWRIGHT_BROWSERS_PATH ?? repoCacheDirectory(repoRoot, "tools", "ms-playwright");
    const { chromium }: typeof import("playwright") = await import(PLAYWRIGHT_MODULE_SPECIFIER);
    // 🖥️ `--use-angle=metal` because headless Chromium otherwise falls back to SwiftShader, whose WebGL
    // is slow enough to turn a shell's first paint into a timeout; the explicit viewport keeps the shell
    // out of its narrow/mobile layout, where the presence bar and the toolbar ids STEP 1/3/5 click are
    // collapsed behind an overflow chip.
    browser = await chromium.launch({ headless: true, args: ["--use-angle=metal"] });
    const context1 = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: COLLAB_E2E_LOCALES[opts.locale].browser });
    const context2 = await browser.newContext({ viewport: { width: 1440, height: 900 }, locale: COLLAB_E2E_LOCALES[opts.locale].browser });
    const user1Page = await context1.newPage();
    const user2Page = await context2.newPage();
    const pageErrors: string[] = [];
    /** 🔬️ Full browser-side visibility for whichever page this is attached to: every `console.*` level
     * (not just uncaught exceptions), failed HTTP requests, and the lifecycle + received frames of every
     * WebSocket the page opens (the `/directory/socket/v1` subscription in particular) — printed immediately so
     * they interleave chronologically with the harness's own `STEP n:` lines in the run log, instead of
     * being buffered and dumped out of order at the end. Added per the ticket's w4-h diagnosis: the prior
     * harness only captured `pageerror`, so a silently-caught `console.warn`/`console.error` inside
     * `ShellHost` was invisible even though it was the only place the real failing branch could show up. */
    const attachBrowserDiagnostics = (page: import("playwright").Page, label: string): void => {
      page.on("pageerror", (err) => pageErrors.push(`${label}: ${String(err)}`));
      page.on("console", (msg) => {
        if (msg.text().includes("space index opening failed")) collabSpaceReopenedAt.set(page, Date.now());
        console.log(`[collab-e2e:console] ${label} [${msg.type()}] ${msg.text()}`);
      });
      page.on("requestfailed", (request) => console.log(`[collab-e2e:network] ${label} requestfailed: ${request.method()} ${request.url()} — ${request.failure()?.errorText ?? "unknown"}`));
      page.on("response", (response) => {
        const url = response.url();
        if (url.includes("/directory/event-page/v1?after=0")) collabSpaceReopenedAt.set(page, Date.now());
        if (!url.includes("/auth/") && !url.includes("/directory/")) return;
        const status = response.status();
        const auth = (response.request().headers()["authorization"] ?? "none").split(" ")[0];
        const postData = url.includes("/auth/") ? "<redacted>" : (response.request().postData() ?? "").slice(0, 300);
        console.log(`[collab-e2e:network] ${label} response: ${response.request().method()} ${url} auth=${auth} body=${postData} — ${status}`);
      });
      page.on("websocket", (ws) => {
        console.log(`[collab-e2e:ws] ${label} opened: ${ws.url()}`);
        ws.on("framereceived", (frame) => console.log(`[collab-e2e:ws] ${label} recv: ${(typeof frame.payload === "string" ? frame.payload : "<binary>").slice(0, 800)}`));
        ws.on("close", () => console.log(`[collab-e2e:ws] ${label} closed: ${ws.url()}`));
        ws.on("socketerror", (error) => console.log(`[collab-e2e:ws] ${label} socketerror: ${ws.url()} — ${error}`));
      });
    };
    attachBrowserDiagnostics(user1Page, "user1");
    attachBrowserDiagnostics(user2Page, "user2");
    const user2Commands = collabCountCommandFrames(user2Page);
    const user1Sent = collabRecordSentCommands(user1Page);

    await user1Page.goto(`http://127.0.0.1:${user1Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await user2Page.goto(`http://127.0.0.1:${user2Port}/`, { waitUntil: "domcontentloaded", timeout: 120_000 });
    await collabWaitForApp(user1Page, "s-home-create-space", 120_000);
    await collabWaitForApp(user2Page, "s-home-create-space", 120_000);
    // 🔐️ Two DIFFERENT humans, each signing in from their own browser context against the same hub.
    // Serially, because the hub's sign-in bucket is keyed per address and both contexts share
    // 127.0.0.1 (AU3 gap 12): two simultaneous mints spend the bucket and the second gets a 429.
    await collabSignIn(user1Page, COLLAB_E2E_USER1_EMAIL, passwords[COLLAB_E2E_USER1_EMAIL]!);
    await collabSignIn(user2Page, COLLAB_E2E_USER2_EMAIL, passwords[COLLAB_E2E_USER2_EMAIL]!);
    console.log(`[collab-e2e] both humans hold a verified session authority on ${hubBaseUrl}`);

    const scenario = await collabRunScenario(record, user1Page, user2Page, hubBaseUrl, user2Commands, () => (external ? collabExternalAdminCapability(external) : hubDaemon!.adminCapability), user1Sent, opts.locale);

    if (hubDaemon) hubDaemon = await collabRunRestartStep({ record, hubDaemon, hubPort, hubDataDir, user1: user1Page, user2: user2Page, user2Commands, spaceId: scenario.spaceId, artifactId: scenario.artifactId });
    else for (const step of [9, 10]) record(step, null, `skipped — the external hub ${hubBaseUrl} is not owned by this run, so it is not restarted`);

    await collabRunCollaborationBehaviours({ record, user1: user1Page, user2: user2Page, spaceId: scenario.spaceId, artifactId: scenario.artifactId, relays: relays! });

    const ignorableGpuFragments = ["NoCompatibleDevice"];
    const criticalErrors = pageErrors.filter((message) => !ignorableGpuFragments.some((fragment) => message.includes(fragment)));
    if (criticalErrors.length > 0) console.warn(`[collab-e2e] page errors observed (not a step on their own, informational): ${criticalErrors.join(" | ")}`);
  }catch(error){primary={error};throw error;}finally{try{await teardown();}catch(error){if(primary&&!Object.is(primary.error,error))throw new AggregateError([primary.error,error],"Collaboration body and teardown failed");throw error;}}

  const passed = results.filter((outcome) => outcome.pass === true).length;
  const skipped = results.filter((outcome) => outcome.pass === null).length;
  console.log(`[collab-e2e] summary: ${passed}/${results.length} steps passed, ${skipped} skipped, ${results.length - passed - skipped} failed`);
  console.log(`[collab-e2e] /spaces/<id> loads that did not mount the Space app: ${collabRouteMisses.length}${collabRouteMisses.length > 0 ? ` — ${collabRouteMisses.join(" | ")}` : ""}`);
  for (const outcome of [...results].sort((left, right) => left.step - right.step)) console.log(`  STEP ${outcome.step}: ${collabVerdict(outcome.pass)}: ${outcome.name}`);
  const failing = results.filter((outcome) => outcome.pass === false).map((outcome) => outcome.step).sort((left, right) => left - right);
  const status = results.length === COLLAB_E2E_STEP_NAMES.length && failing.length === 0 ? "pass" : "fail";
  publishAcceptanceCheckResult(
    repoRoot,
    acceptanceCheckResult({
      check: opts.check,
      status,
      startedAt,
      measured: { locale: opts.locale, steps: COLLAB_E2E_STEP_NAMES.length, recorded: results.length, passed, skipped, failed: failing.length, routeMisses: collabRouteMisses.length, externalHub: external !== null },
      summary: {
        en: `${passed}/${COLLAB_E2E_STEP_NAMES.length} collaboration steps pass in ${opts.locale} (${skipped} skipped)${failing.length ? `; failing: STEP ${failing.join(", ")}` : ""}`,
        de: `${passed}/${COLLAB_E2E_STEP_NAMES.length} Zusammenarbeitsschritte bestehen in ${opts.locale} (${skipped} übersprungen)${failing.length ? `; fehlgeschlagen: SCHRITT ${failing.join(", ")}` : ""}`,
      },
      evidence: [outDir],
    }),
  );
  if (status !== "pass") process.exitCode = 1;
}

/** 🏠️ Keeps actual Host, catalog and diagnostics owners around the complete collaboration scenario. */
async function runCollabE2eVerify(opts:Parameters<typeof runCollabE2eVerifyOwnedV1>[0]):Promise<void>{
 requireIssuedResourceControlInvocationV1(opts.phaseControl,opts.invocation);
 for(const key of["S_COLLAB_OUT","S_COLLAB_HUB_DATA","S_COLLAB_HUB_BINARY"])if(process.env[key]!==undefined)throw Error("Collaboration paths belong to the actual retained Host owner");
 return withBuiltLocalHubHostV1(opts.invocation,opts.phaseControl,(host,context)=>withMaterializedTrustedCatalogBundleV1(host,context.control,trustedBootstrapSelectPackages(LOCAL_HUB_DEVELOPMENT_CATALOG_PACKAGES),async(receipt,phase)=>{await validateAndPublishTrustedStdioGisCandidate(host,phase,receipt);return withLocalHubHostTaskV1(host,phase,actual=>{const scope:CollabScopeV1={host,context:actual,servers:new Set(),ledger:new LocalHubHostLedgerV1(),live:true,serverBirths:0};return collabScopesV1.run(scope,async()=>{const failures:unknown[]=[];try{await runCollabE2eVerifyOwnedV1({...opts,phaseControl:actual.control});}catch(error){failures.push(error);}scope.live=false;scope.ledger.stop();try{await scope.ledger.drain();}catch(error){if(!failures.includes(error))failures.push(error);}scope.ledger.close();for(const server of scope.servers)try{await finishCollabDevServerV1(server);}catch(error){if(!failures.includes(error))failures.push(error);}if(failures.length===1)throw failures[0];if(failures.length)throw new AggregateError(failures,"Collaboration body and accepted server cleanup failed");});});}));
}

/** 🚪️ `verify collab [--hub <url>] [--locale en|de]` — the two-human React collaboration journey against the hub `--hub` names
 * (`S_COLLAB_HUB_URL` otherwise; neither: this run boots its own hub), both humans in `--locale`; the goal-gate record is
 * `react-collaboration-e2e-<locale>`, `blocked` when the hub cannot be reached. Credentials only via env. */
export async function runCollabE2eCli(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,segments: readonly string[]): Promise<void> {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);
  if(readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot!==repoRoot)throw Error("Actual collaboration workspace required");
  const flag = (name: string): string | undefined => {
    const value = segments[segments.indexOf(name) + 1];
    return segments.includes(name) && value !== undefined && !value.startsWith("--") ? value : undefined;
  };
  const locale: CollabLocale = flag("--locale") === "de" ? "de" : "en";
  const check = `react-collaboration-e2e-${locale}`;
  await withAcceptanceRecord(repoRoot, check, () => runCollabE2eVerify({ invocation,phaseControl,hub: flag("--hub") ?? process.env.S_COLLAB_HUB_URL, locale, check }), (error) => /ECONNREFUSED|ERR_CONNECTION_REFUSED|Unable to connect|hub never became ready|needs S_COLLAB_USER1_PASSWORD/u.test(String(error instanceof Error ? error.message : error)));
}
//#endregion 🔖️CollabE2e
