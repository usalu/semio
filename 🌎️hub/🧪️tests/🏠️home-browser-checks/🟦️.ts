/** 🧭️ Directory Home browser process and scoped presence browser serve gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { readNativeCallerAuthorityV1 } from "@semio-tech/repo-lib/artifacts";
import { type CompilerAdmissionControl } from "@semio-tech/process/compiler-capacity";
import { requireIssuedResourceControlInvocationV1, type ScriptInvocation } from "@semio-tech/process/routing";
import { buildBudgetMs } from "@semio-tech/process/budget";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, statSync } from "node:fs";
import { isAbsolute, join, relative } from "node:path";
import { repoCacheDirectory, runCmd, runProbe } from "@semio-tech/process/execution";
import { decodeClientFrame, decodePresencePeer, encodePresencePeer, encodeServerFrame, type ArtifactPresencePeer } from "@semio-tech/framework-replication";
import { builtPluginModulesRootIn } from "@semio-tech/framework-os-dev/activation";
import { COMPONENT_RECEIPT_FILE, readComponentReceiptV1 } from "@semio-tech/framework-plugin/describe/component-receipt";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { freeLoopbackPort, hubBinaryPath } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { browserDocumentOpenFixture, type BrowserDocumentOpenFixture } from "../📄️browser-document-open/🟦️.ts";
import type { LiveDirectoryEventPageV1 } from "../📣️directory-live-support/🟦️.ts";
import { proveDirectoryEventPageV1Process } from "../📰️event-page-checks/🟦️.ts";
import { type LocalBrowserRelay, startLocalBrowserRelay } from "../../🚀️local-relay/🛰️servers/🟦️.ts";

type DirectoryHomeBrowserProcessFixture = {
  readonly schema: "semio.hub.directory-home-browser-process-fixture/v1";
  readonly limits: { readonly journeyMs: number; readonly stepMs: number; readonly responseBytes: 65536; readonly appliedEvents: number };
  readonly spaceGuest: { readonly target: "wasm32-wasip2"; readonly package: "semio-hub-space"; readonly nativeFeature: "sync"; readonly forbiddenPackages: readonly ["ring", "cc", "tokio"] };
  readonly profiles: Readonly<Record<"a" | "b", { readonly profileId: string; readonly subject: string; readonly displayName: string }>>;
  readonly home: { readonly pluginId: "space"; readonly appId: "s.space.home@1/*#editor"; readonly actionId: "applyDirectoryEventPage"; readonly moduleDirectory: "🪐️space" };
  readonly pages: Readonly<
    Record<string, { readonly epoch: number; readonly binding: string; readonly generation: number; readonly after: number; readonly through: number; readonly hasMore: boolean; readonly receipt: string; readonly eventIds: readonly string[] }>
  >;
  readonly traces: readonly { readonly name: string; readonly steps: readonly Record<string, any>[]; readonly expected: Record<string, any> }[];
  readonly hostile: readonly { readonly name: string; readonly mutation: string }[];
};

function directoryHomeBrowserProcessModel(fixture: DirectoryHomeBrowserProcessFixture): number {
  type State = { epoch: number; frontier: number; phase: "fetching" | "awaiting-ack" | "live" | "closed"; pending?: DirectoryHomeBrowserProcessFixture["pages"][string]; applied: string[]; dials: number[]; retries: number[]; denials: number };
  const exactPage = (left: DirectoryHomeBrowserProcessFixture["pages"][string] | undefined, right: DirectoryHomeBrowserProcessFixture["pages"][string]): boolean =>
    !!left && left.epoch === right.epoch && left.binding === right.binding && left.generation === right.generation && left.after === right.after && left.through === right.through && left.receipt === right.receipt;
  const step = (state: State, operation: Record<string, any>): void => {
    if (operation.kind === "rebootstrap") {
      state.epoch = operation.epoch;
      state.frontier = 0;
      state.phase = "fetching";
      state.pending = undefined;
      state.applied = [];
      return;
    }
    if (operation.kind === "close") {
      state.phase = "closed";
      state.pending = undefined;
      return;
    }
    if (operation.kind === "wake") {
      if (state.phase === "live") state.phase = "fetching";
      else state.denials += 1;
      return;
    }
    if (operation.kind === "dial") {
      if (state.phase === "live" && operation.since === state.frontier) state.dials.push(operation.since);
      else state.denials += 1;
      return;
    }
    const page = fixture.pages[operation.page];
    if (!page) throw new Error(`directory Home process trace references unknown page ${operation.page}`);
    if (operation.kind === "present") {
      if (state.phase === "fetching" && page.epoch === state.epoch && page.after === state.frontier && page.through >= state.frontier) {
        state.pending = page;
        state.phase = "awaiting-ack";
      } else state.denials += 1;
      return;
    }
    if (operation.kind === "reject") {
      if (state.phase === "awaiting-ack" && exactPage(state.pending, page)) {
        state.pending = undefined;
        state.phase = "fetching";
        state.retries.push(state.frontier);
      } else state.denials += 1;
      return;
    }
    if (operation.kind !== "ack" || state.phase !== "awaiting-ack" || !exactPage(state.pending, page)) {
      state.denials += 1;
      return;
    }
    if (page.eventIds.some((id) => state.applied.includes(id)) || state.applied.length + page.eventIds.length > fixture.limits.appliedEvents) {
      state.denials += 1;
      return;
    }
    state.applied.push(...page.eventIds);
    state.frontier = page.through;
    state.pending = undefined;
    state.phase = page.hasMore ? "fetching" : "live";
  };
  for (const trace of fixture.traces) {
    const state: State = { epoch: 1, frontier: 0, phase: "fetching", applied: [], dials: [], retries: [], denials: 0 };
    for (const operation of trace.steps) step(state, operation);
    const observed = { epoch: state.epoch, frontier: state.frontier, phase: state.phase, appliedEventIds: state.applied, dials: state.dials, retries: state.retries, denials: state.denials };
    if (JSON.stringify(observed) !== JSON.stringify(trace.expected)) throw new Error(`directory Home process model differs for ${trace.name}: ${JSON.stringify(observed)}`);
  }
  const initial = fixture.pages.initial!;
  const mutations: Record<string, (page: typeof initial) => typeof initial> = {
    epoch: (page) => ({ ...page, epoch: page.epoch + 1 }),
    binding: (page) => ({ ...page, binding: "f".repeat(64) }),
    generation: (page) => ({ ...page, generation: page.generation + 1 }),
    through: (page) => ({ ...page, through: page.through + 1 }),
    receipt: (page) => ({ ...page, receipt: "f".repeat(64) }),
  };
  for (const hostile of fixture.hostile) {
    const state: State = { epoch: 1, frontier: 0, phase: "fetching", applied: [], dials: [], retries: [], denials: 0 };
    if (mutations[hostile.mutation]) {
      step(state, { kind: "present", page: "initial" });
      const changed = mutations[hostile.mutation]!(initial);
      if (state.phase !== "awaiting-ack" || exactPage(state.pending, changed)) throw new Error(`directory Home process hostile ${hostile.name} was not distinct`);
      state.denials += 1;
    } else if (hostile.mutation === "duplicate-event") {
      state.pending = { ...initial, eventIds: [initial.eventIds[0]!, initial.eventIds[0]!] };
      state.phase = "awaiting-ack";
      if (new Set(state.pending.eventIds).size === state.pending.eventIds.length) throw new Error("directory Home duplicate hostile was not duplicated");
      state.denials += 1;
    } else if (hostile.mutation === "socket-before-ack") {
      step(state, { kind: "present", page: "initial" });
      step(state, { kind: "dial", since: 0 });
    } else if (hostile.mutation === "late-after-close") {
      step(state, { kind: "present", page: "initial" });
      step(state, { kind: "close" });
      step(state, { kind: "ack", page: "initial" });
    } else throw new Error(`directory Home process model has unknown hostile ${hostile.mutation}`);
    if (state.frontier !== 0 || state.applied.length !== 0 || state.denials !== 1) throw new Error(`directory Home process hostile ${hostile.name} advanced authority`);
  }
  return fixture.traces.length + fixture.hostile.length;
}

async function proveDirectoryHomeBrowserProcessSource(repoRoot: string): Promise<DirectoryHomeBrowserProcessFixture> {
  const fixtureRoot = join(repoRoot, "🌎️hub/📇️directory/🧪️tests/🧫️fixtures/🌐️directory-home-browser-process-v1");
  const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as DirectoryHomeBrowserProcessFixture;
  if (fixture.schema !== "semio.hub.directory-home-browser-process-fixture/v1") throw new Error("directory Home browser process fixture schema drift");
  if (Object.keys(fixture).sort().join(",") !== "home,hostile,limits,pages,profiles,schema,spaceGuest,traces") throw new Error("directory Home browser process fixture envelope drift");
  if (fixture.limits.responseBytes !== 65536 || fixture.limits.appliedEvents < 8 || fixture.limits.appliedEvents > 512) throw new Error("directory Home browser process bound drift");
  if (fixture.spaceGuest.target !== "wasm32-wasip2" || fixture.spaceGuest.package !== "semio-hub-space" || fixture.spaceGuest.nativeFeature !== "sync" || fixture.spaceGuest.forbiddenPackages.join(",") !== "ring,cc,tokio")
    throw new Error("directory Home browser process guest boundary drift");
  if (fixture.home.pluginId !== "space" || fixture.home.appId !== "s.space.home@1/*#editor" || fixture.home.actionId !== "applyDirectoryEventPage" || fixture.home.moduleDirectory !== "🪐️space") throw new Error("directory Home browser process app binding drift");
  const homeProfile = hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryProfileV1");
  if (!homeProfile(fixture.profiles.a) || !homeProfile(fixture.profiles.b) || fixture.profiles.a.profileId === fixture.profiles.b.profileId) throw new Error("directory Home browser process profiles are not two distinct owned directory profiles");
  const homePage = hubSchemaExport(repoRoot, "schema://hub.directory/DirectoryEventPageCursorV1");
  if (Object.keys(fixture.pages).length < 4) throw new Error("directory Home browser process page inventory drift");
  for (const [name, page] of Object.entries(fixture.pages)) if (!homePage(page)) throw new Error(`directory Home browser process page is not the owned event-page cursor contract: ${name}`);
  if (fixture.traces.length < 8 || fixture.traces.length > 16 || new Set(fixture.traces.map((trace) => trace.name)).size !== fixture.traces.length) throw new Error("directory Home browser process trace inventory drift");
  const homeMutations = ["epoch", "binding", "generation", "through", "receipt", "duplicate-event", "socket-before-ack", "late-after-close"];
  if (fixture.hostile.length < 8 || fixture.hostile.length > 24 || fixture.hostile.some((row) => !homeMutations.includes(row.mutation))) throw new Error("directory Home browser process hostile inventory drift");
  if (new Set(fixture.hostile.map((row) => row.mutation)).size !== homeMutations.length) throw new Error("directory Home browser process hostile mutations are not exhaustive");
  const modelChecks = directoryHomeBrowserProcessModel(fixture);
  const guestTree = runProbe("cargo", ["tree", "--manifest-path", join(repoRoot, "🌎️hub/Cargo.toml"), "-e", "features", "-p", fixture.spaceGuest.package, "--target", fixture.spaceGuest.target], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  if (guestTree.status !== 0) throw new Error(`directory Home Space guest graph oracle failed: ${guestTree.stderr}`);
  for (const forbidden of fixture.spaceGuest.forbiddenPackages) {
    if (new RegExp(`(?:^|\\s)${forbidden} v`, "mu").test(guestTree.stdout)) throw new Error(`directory Home Space guest graph retained forbidden package ${forbidden}`);
  }
  if (guestTree.stdout.includes(`semio-framework-os-kernel feature "${fixture.spaceGuest.nativeFeature}"`)) throw new Error(`directory Home Space guest graph retained native kernel feature ${fixture.spaceGuest.nativeFeature}`);
  const rustc = runProbe("rustc", ["-vV"], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  const hostTarget = rustc.status === 0 ? rustc.stdout.match(/^host: (.+)$/mu)?.[1] : undefined;
  if (!hostTarget) throw new Error(`directory Home Space native target oracle failed: ${rustc.stderr}`);
  const nativeTree = runProbe("cargo", ["tree", "--manifest-path", join(repoRoot, "🌎️hub/Cargo.toml"), "-e", "features", "-p", fixture.spaceGuest.package, "--target", hostTarget, "-i", "semio-framework-os-kernel"], { cwd: repoRoot, budgetMs: fixture.limits.stepMs });
  if (nativeTree.status !== 0 || !nativeTree.stdout.includes(`semio-framework-os-kernel feature "${fixture.spaceGuest.nativeFeature}"`)) throw new Error("directory Home Space native graph lost the kernel sync transport");
  const worker = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"));
  const owner = readFileSync(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx"), "utf8");
  const shell = readModuleTreeSource(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx"));
  const sourceClosed = (workerSource: string, ownerSource: string, shellSource: string): boolean => {
    const acknowledge = workerSource.indexOf("owner.machine.acknowledge(ack)");
    const openLive = workerSource.indexOf("openDirectoryBootstrapLive(owner, transition.since)");
    const identity = ownerSource.indexOf('directoryActionInvocation(owner, "setClient"');
    const opened = ownerSource.indexOf('kind: "directory-bootstrap-open"');
    const receipt = ownerSource.indexOf("parseDirectoryProjectionReceiptV1(response.output)");
    const ack = ownerSource.indexOf('kind: "directory-bootstrap-ack"');
    return (
      workerSource.includes("class DirectoryEventPageBootstrapV1") &&
      workerSource.includes("streamAcknowledged(since") &&
      acknowledge >= 0 &&
      openLive > acknowledge &&
      workerSource.includes("if (directoryBootstrap !== owner || owner.abort.signal.aborted) return") &&
      workerSource.includes("owner.machine.wake(rebootstrap)") &&
      ownerSource.includes("await owner.plugin.handleAction") &&
      identity >= 0 &&
      opened > identity &&
      ownerSource.includes("invocationTerminal(response)") &&
      ownerSource.includes("if (owner.ownsInstance)") &&
      ownerSource.includes("await beforeAcknowledge?.(owner)") &&
      receipt >= 0 &&
      ack > receipt &&
      ownerSource.includes("owner.abort.signal.aborted") &&
      ownerSource.includes('kind: "directory-bootstrap-reject"') &&
      shellSource.includes("openDirectoryHomeOwnerV1") &&
      shellSource.includes("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }") &&
      shellSource.includes("identity: { userId: identity.userId, displayName: identity.displayName }") &&
      shellSource.includes("directoryHomeOpeningRef.current.catch") &&
      shellSource.includes("await refreshDirectoryHomeRef.current(active)") &&
      shellSource.includes("applyDirectoryEventPageBootstrapV1") &&
      shellSource.includes("closeDirectoryHomeOwnerV1") &&
      !shellSource.includes('kind: "directory-open", baseUrl: resolved.hubBaseUrl')
    );
  };
  if (!sourceClosed(worker, owner, shell)) throw new Error("directory Home browser process source lost ACK-owned frontier/cancellation wiring");
  const hostiles = [
    [worker.replace("owner.machine.acknowledge(ack)", "owner.machine.wake(false)"), owner, shell],
    [worker.replaceAll("if (directoryBootstrap !== owner || owner.abort.signal.aborted) return", "if (false) return"), owner, shell],
    [worker, owner.replace("parseDirectoryProjectionReceiptV1(response.output)", "page as any"), shell],
    [worker, owner.replaceAll("owner.abort.signal.aborted", "false"), shell],
    [worker, owner.replace('directoryActionInvocation(owner, "setClient"', 'directoryActionInvocation(owner, "applyDirectoryEventPage"'), shell],
    [worker, owner, shell.replace("instance: { instanceId: visibleSession.instanceId, viewState: visibleSession.viewState }", "instance: undefined")],
  ];
  hostiles.forEach((candidate, index) => {
    if (sourceClosed(candidate[0]!, candidate[1]!, candidate[2]!)) throw new Error(`directory Home browser process source oracle admitted removed fence ${index}`);
  });
  console.log(`directory-home-browser-process-oracle: ajv=1 model=${modelChecks} source=5 hostile-source=${hostiles.length} guest-graph=${fixture.spaceGuest.forbiddenPackages.length} native-feature=1 passed`);
  return fixture;
}

async function proveDirectoryHomeBrowserControllerRuntime(repoRoot: string, fixture: DirectoryHomeBrowserProcessFixture): Promise<void> {
  let runtimeServer: { stop(closeActiveConnections?: boolean): void | Promise<void> } | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const abort = AbortSignal.timeout(fixture.limits.journeyMs);
  try {
    process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repoRoot, "tools", "ms-playwright");
    const controllerPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx");
    const bundle = await Bun.build({ entrypoints: [controllerPath], target: "browser", format: "esm", sourcemap: "none" });
    if (!bundle.success || bundle.outputs.length !== 1) throw new Error(`directory Home browser controller bundle failed: ${bundle.logs.map(String).join("; ") || "unexpected output closure"}`);
    const controller = await bundle.outputs[0]!.text();
    const port = await freeLoopbackPort();
    runtimeServer = Bun.serve({
      hostname: "127.0.0.1",
      port,
      fetch(request): Response {
        const path = new URL(request.url).pathname;
        if (path === "/controller.js") return new Response(controller, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
        if (path === "/__directory-home-controller") return new Response("<!doctype html><meta charset=utf-8><title>Directory Home Controller</title>", { headers: { "content-type": "text/html; charset=utf-8" } });
        return new Response("not found", { status: 404 });
      },
    });
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => browserDiagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => browserDiagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => browserDiagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    const probeUrl = `http://127.0.0.1:${port}/__directory-home-controller`;
    await page.goto(probeUrl, { waitUntil: "domcontentloaded", timeout: fixture.limits.stepMs });
    const moduleUrl = "/controller.js";
    const deadline = new Promise<never>((_, reject) => {
      const fail = () => reject(new Error("directory Home browser controller deadline exceeded"));
      if (abort.aborted) fail();
      else abort.addEventListener("abort", fail, { once: true });
    });
    const result = await Promise.race([
      page.evaluate(
        async ({ moduleUrl, home, sourcePage }) => {
          const api = await import(moduleUrl);
          const records: string[] = [];
          const receipt = {
            schema: "semio.space.home.directory-projection-receipt.v1",
            sessionBindingSha256: sourcePage.binding,
            authorizationGeneration: sourcePage.generation,
            throughSeqInclusive: sourcePage.through,
            receiptSha256: sourcePage.receipt,
          };
          const terminal = (output: unknown) => ({ output, mutations: [], inverseGroup: { invocationId: "browser", mutations: [], inverseMutations: [] } });
          let release: (() => void) | undefined;
          const delayed = new Promise<void>((resolve) => {
            release = resolve;
          });
          const plugin = {
            pluginId: home.pluginId,
            createApp: async () => {
              records.push("create");
              return 41;
            },
            destroyApp: async () => {
              records.push("destroy");
            },
            handleAction: async (_instanceId: number, invocation: string) => {
              const parsed = JSON.parse(invocation);
              if (parsed.address.actionId === "setClient") {
                records.push(`identity:${parsed.arguments.clientId}:${parsed.arguments.clientName}`);
                return terminal(null);
              }
              records.push("page");
              return terminal(receipt);
            },
          };
          const app = { id: home.appId, controllerId: home.appId, modes: [{ id: "explore" }], defaultModeId: "explore", windowKinds: [{ id: "main", actions: [{ id: home.actionId }, { id: "setClient" }] }] };
          const posts: any[] = [];
          const owner = await api.openDirectoryHomeOwnerV1({
            plugin,
            app,
            identity: { userId: "user-a", displayName: "Directory Browser A" },
            instance: { instanceId: 41, viewState: { activeModeId: "explore" } },
            baseUrl: "http://127.0.0.1:6070",
            bootstrapEpoch: 1,
            locale: "en",
            terminology: "native",
            beforeBootstrap: async () => {
              records.push("refresh-open");
            },
            post: (message: any) => posts.push(message),
          });
          const page = {
            kind: "directory-event-page",
            bootstrapEpoch: 1,
            canonicalJson: JSON.stringify({ schema: "semio.directory.event-page.v1", events: sourcePage.eventIds }),
            sessionBindingSha256: sourcePage.binding,
            authorizationGeneration: sourcePage.generation,
            afterSeqExclusive: sourcePage.after,
            throughSeqInclusive: sourcePage.through,
            hasMore: sourcePage.hasMore,
            receiptSha256: sourcePage.receipt,
          };
          const applied = await api.applyDirectoryEventPageBootstrapV1(
            owner,
            page,
            (message: any) => posts.push(message),
            async () => {
              records.push("refresh-ack");
            },
          );
          await api.closeDirectoryHomeOwnerV1(owner, (message: any) => posts.push(message));
          const latePosts: any[] = [];
          const latePlugin = {
            ...plugin,
            handleAction: async (_instanceId: number, invocation: string) => {
              const parsed = JSON.parse(invocation);
              if (parsed.address.actionId === "setClient") return terminal(null);
              await delayed;
              return terminal(receipt);
            },
          };
          const lateOwner = await api.openDirectoryHomeOwnerV1({
            plugin: latePlugin,
            app,
            identity: { userId: "user-b", displayName: "Directory Browser B" },
            instance: { instanceId: 42, viewState: { activeModeId: "explore" } },
            baseUrl: "http://127.0.0.1:6070",
            bootstrapEpoch: 2,
            locale: "de",
            terminology: "native",
            post: (message: any) => latePosts.push(message),
          });
          const late = api.applyDirectoryEventPageBootstrapV1(lateOwner, { ...page, bootstrapEpoch: 2 }, (message: any) => latePosts.push(message));
          await api.closeDirectoryHomeOwnerV1(lateOwner, (message: any) => latePosts.push(message));
          release!();
          const cancelled = await late;
          return { records, posts, applied, latePosts, cancelled };
        },
        { moduleUrl, home: fixture.home, sourcePage: fixture.pages.initial! },
      ),
      deadline,
    ]);
    const ack = result.posts.find((message):message is Record<string,unknown> => message!==null&&typeof message==="object"&&"kind" in message&&message.kind === "directory-bootstrap-ack");
    if (
      JSON.stringify(result.records) !== JSON.stringify(["identity:user-a:Directory Browser A", "refresh-open", "page", "refresh-ack"]) ||
      !ack ||
      ack.receiptSha256 !== fixture.pages.initial!.receipt ||
      ack.throughSeqInclusive !== fixture.pages.initial!.through ||
      result.applied.state.kind !== "idle"
    )
      throw new Error(`directory Home browser controller positive journey differed: ${JSON.stringify(result)}`);
    if (result.latePosts.some((message: Record<string, any>) => message.kind === "directory-bootstrap-ack") || result.cancelled.state.code !== "directory-bootstrap.cancelled")
      throw new Error("directory Home browser controller accepted a late terminal after close");
    console.log("directory-home-browser-runtime: Chromium actual same-visible-instance Hub identity, pre-open/pre-ACK refresh, ACK and late-cancel laws passed");
  } catch (error) {
    const diagnostics = browserDiagnostics.slice(-16).join("\n");
    throw new Error(`${error instanceof Error ? error.message : "directory Home browser controller failed"}${diagnostics ? `\nbrowser diagnostics:\n${diagnostics}` : ""}`);
  } finally {
    await browser?.close().catch(() => {});
    await runtimeServer?.stop(true);
  }
}

function assertDirectoryHomeBrowserComponentAttestation(repoRoot: string, fixture: DirectoryHomeBrowserProcessFixture, mode:"prototype"|"release"): void {
  const moduleRoot = join(builtPluginModulesRootIn(repoRoot,mode),fixture.home.moduleDirectory);
  const manifest = JSON.parse(readFileSync(join(moduleRoot, "🔣️.json"), "utf8")) as Record<string, any>;
  const descriptorSha256 = createHash("sha256").update(readFileSync(join(moduleRoot, "🛂️.descriptor.semio"))).digest("hex");
  const app = manifest.manifest?.apps?.find((candidate: Record<string, any>) => candidate.id === fixture.home.appId);
  const failures = [];
  if (!app) failures.push(`missing-app=${fixture.home.appId}`);
  else {
    for (const actionId of ["setClient", fixture.home.actionId]) {
      if (!app.windowKinds?.some((window: Record<string, any>) => window.actions?.some((action: Record<string, any>) => action.id === actionId))) failures.push(`missing-action=${actionId}`);
    }
  }
  try {
    const receipt = readComponentReceiptV1(join(moduleRoot, COMPONENT_RECEIPT_FILE));
    if (receipt.pluginId !== "space" || receipt.semanticDescriptorSha256 !== descriptorSha256 || "hashes" in manifest) failures.push(`receipt names ${receipt.pluginId} with descriptor ${receipt.semanticDescriptorSha256}, module descriptor ${descriptorSha256}`);
  } catch (error) {
    failures.push(`receipt ${(error as Error).message}`);
  }
  if (failures.length > 0) throw new Error(`directory Home browser process blocked before discovery: Space component attestation rejected: ${failures.join("; ")}`);
}

async function proveDirectoryHomeBrowserStaticWasmProcessRuntime(
  repoRoot: string,
  fixture: DirectoryHomeBrowserProcessFixture,
  source: Readonly<{ page: LiveDirectoryEventPageV1; userId: string; displayName: string; baseUrl: string }>,
  mode:"prototype"|"release",
): Promise<void> {
  const bundleModule = async (entrypoint: string): Promise<string> => {
    const bundle = await Bun.build({ entrypoints: [entrypoint], target: "browser", format: "esm", sourcemap: "none", define: { "import.meta.vitest": "undefined" } });
    if (!bundle.success || bundle.outputs.length !== 1) throw new Error(`directory Home real browser bundle failed: ${bundle.logs.map(String).join("; ") || "unexpected output closure"}`);
    return bundle.outputs[0]!.text();
  };
  const [controller, runtime] = await Promise.all([
    bundleModule(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/📇️directory-bootstrap/🟦️.tsx")),
    bundleModule(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔌️PluginRuntime/🟦️.tsx")),
  ]);
  const pluginRoot=builtPluginModulesRootIn(repoRoot,mode);
  const mime = (path: string): string => (path.endsWith(".wasm") ? "application/wasm" : path.endsWith(".json") ? "application/json; charset=utf-8" : "text/javascript; charset=utf-8");
  const physicalPluginPath = (urlPath: string): string | undefined => {
    const prefix = "/🔌️plugin-modules/";
    const shardPrefix = "/plugin-modules/_shard/";
    const relativePath = urlPath.startsWith(prefix) ? urlPath.slice(prefix.length) : urlPath.startsWith(shardPrefix) ? join("🧵️shard", urlPath.slice(shardPrefix.length)) : undefined;
    if (!relativePath || relativePath.split("/").includes("..")) return undefined;
    const candidate = join(pluginRoot, relativePath);
    const escaped = relative(pluginRoot, candidate);
    return escaped.startsWith("..") || isAbsolute(escaped) || !existsSync(candidate) || !statSync(candidate).isFile() ? undefined : candidate;
  };
  const port = await freeLoopbackPort();
  const server = Bun.serve({
    hostname: "127.0.0.1",
    port,
    fetch(request): Response {
      const path = decodeURIComponent(new URL(request.url).pathname);
      if (path === "/controller.js") return new Response(controller, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
      if (path === "/plugin-runtime.js") return new Response(runtime, { headers: { "content-type": "text/javascript; charset=utf-8", "cache-control": "no-store" } });
      if (path === "/__directory-home-process") return new Response("<!doctype html><meta charset=utf-8><title>Directory Home Process</title>", { headers: { "content-type": "text/html; charset=utf-8" } });
      const physical = physicalPluginPath(path);
      return physical ? new Response(readFileSync(physical), { headers: { "content-type": mime(physical), "cache-control": "no-store" } }) : new Response("not found", { status: 404 });
    },
  });
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const diagnostics: string[] = [];
  try {
    process.env.PLAYWRIGHT_BROWSERS_PATH ??= repoCacheDirectory(repoRoot, "tools", "ms-playwright");
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => diagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => diagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => diagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    await page.goto(`http://127.0.0.1:${port}/__directory-home-process`, { waitUntil: "domcontentloaded", timeout: fixture.limits.stepMs });
    const abort = AbortSignal.timeout(fixture.limits.journeyMs);
    const deadline = new Promise<never>((_, reject) => {
      const fail = () => reject(new Error("directory Home real browser process deadline exceeded"));
      if (abort.aborted) fail();
      else abort.addEventListener("abort", fail, { once: true });
    });
    const result = await Promise.race([
      page.evaluate(
        async ({ home, source }) => {
          const record=(value:unknown):value is Record<string,unknown>=>value!==null&&typeof value==="object"&&!Array.isArray(value);
          const call=(module:unknown,name:string,args:unknown[]):unknown=>{
            if(!record(module)||typeof module[name]!=="function")throw Error(`Directory browser module method missing: ${name}`);
            return Reflect.apply(module[name],module,args);
          };
          const controllerUrl="/controller.js",runtimeUrl="/plugin-runtime.js";
          const controller:unknown=await import(controllerUrl);
          const runtime:unknown=await import(runtimeUrl);
          call(runtime,"setPluginRuntimeActor",[`user:${source.userId}#directory-home-process`]);
          const plugin:unknown=await call(runtime,"loadPluginModule",[home.pluginId,`/🔌️plugin-modules/${home.moduleDirectory}/🌉️bridge.js`,AbortSignal.timeout(10_000)]);
          if(!record(plugin)||!record(plugin.manifest)||!Array.isArray(plugin.manifest.apps))throw Error("Directory browser plugin manifest unavailable");
          const app=plugin.manifest.apps.find((candidate:unknown)=>record(candidate)&&candidate.id===home.appId);
          if(!record(app)||!Array.isArray(app.windowKinds))throw Error("Directory browser app unavailable after discovery");
          const posts:unknown[]=[];
          let owner:unknown;
          let opened=false;
          try {
            owner=await call(controller,"openDirectoryHomeOwnerV1",[{
              plugin,app,identity:{userId:source.userId,displayName:source.displayName},baseUrl:source.baseUrl,
              bootstrapEpoch:1,locale:"en",terminology:"native",post:(message:unknown)=>posts.push(message),
            }]);
            opened=true;
            const applied:unknown=await call(controller,"applyDirectoryEventPageBootstrapV1",[owner,{
              kind:"directory-event-page",bootstrapEpoch:1,canonicalJson:JSON.stringify(source.page),
              sessionBindingSha256:source.page.sessionBindingSha256,authorizationGeneration:source.page.authorizationGeneration,
              afterSeqExclusive:source.page.afterSeqExclusive,throughSeqInclusive:source.page.throughSeqInclusive,
              hasMore:source.page.hasMore,receiptSha256:source.page.receiptSha256,
            },(message:unknown)=>posts.push(message)]);
            if(!record(applied)||!record(applied.state)||typeof applied.state.kind!=="string")throw Error("Directory browser apply result unavailable");
            return {pluginId:plugin.manifest.pluginId,appId:app.id,action:app.windowKinds.some((window:unknown)=>record(window)&&Array.isArray(window.actions)&&window.actions.some((action:unknown)=>record(action)&&action.id===home.actionId)),posts,appliedKind:applied.state.kind};
          } finally {
            if(opened)await call(controller,"closeDirectoryHomeOwnerV1",[owner,(message:unknown)=>posts.push(message)]);
            call(plugin,"dispose",[]);
          }
        },
        { home: fixture.home, source },
      ),
      deadline,
    ]);
    const ack = result.posts.find((message):message is Record<string,unknown> => message!==null&&typeof message==="object"&&"kind" in message&&message.kind === "directory-bootstrap-ack");
    const expected = source.page;
    if (
      result.pluginId !== fixture.home.pluginId ||
      result.appId !== fixture.home.appId ||
      !result.action ||
      result.appliedKind !== "idle" ||
      !ack ||
      ack.sessionBindingSha256 !== expected.sessionBindingSha256 ||
      ack.authorizationGeneration !== expected.authorizationGeneration ||
      ack.throughSeqInclusive !== expected.throughSeqInclusive ||
      ack.receiptSha256 !== expected.receiptSha256
    ) {
      throw new Error(`directory Home real browser terminal differed: ${JSON.stringify(result)}`);
    }
    console.log(`directory-home-browser-process-runtime: real-hub-page=1 static-dev-space-wasm=1 verified-activation=0 Chromium=1 through=${expected.throughSeqInclusive} receipt=${expected.receiptSha256}`);
  } catch (error) {
    const detail = diagnostics.slice(-24).join("\n");
    throw new Error(`${error instanceof Error ? error.message : "directory Home real browser process failed"}${detail ? `\nbrowser diagnostics:\n${detail}` : ""}`);
  } finally {
    await browser?.close().catch(() => {});
    await server.stop(true);
  }
}

async function proveDirectoryHomeBrowserProcessJourney(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,fixture:DirectoryHomeBrowserProcessFixture,mode:"prototype"|"release"):Promise<void>{
 requireIssuedResourceControlInvocationV1(phaseControl,invocation);const repoRoot=readNativeCallerAuthorityV1(invocation.capabilities).repositoryRoot,root=join(repoRoot,"🌎️hub/📦️packages/🦀️rust");
  const binary = hubBinaryPath(repoRoot);
  if (!existsSync(binary)) throw new Error(`directory Home browser process blocked before discovery: os-hub binary absent at ${binary}`);
  const source = await proveDirectoryEventPageV1Process(invocation,phaseControl);
  assertDirectoryHomeBrowserComponentAttestation(repoRoot,fixture,mode);
  await proveDirectoryHomeBrowserStaticWasmProcessRuntime(repoRoot,fixture,source,mode);
}

type ScopedPresenceBrowserAuthorityCase = Readonly<{
  id: "a" | "b";
  scope: { readonly spaceId: string; readonly documentId: string };
  surfaceId: string;
  plan: Record<string, any>;
  installedTarget: Record<string, any>;
  socketGrant: Record<string, any>;
  openPath: string;
  grantPath: string;
  socketPath: string;
}>;

function scopedPresenceBrowserCases(fixture: BrowserDocumentOpenFixture): readonly [ScopedPresenceBrowserAuthorityCase, ScopedPresenceBrowserAuthorityCase] {
  const make = (id: "a" | "b", spaceId: string, surfaceId: string, actorByte: string): ScopedPresenceBrowserAuthorityCase => {
    const scope = { spaceId, documentId: fixture.intent.scope.documentId };
    const plan = structuredClone(fixture.plan) as Record<string, any>;
    const installedTarget = structuredClone(fixture.installedTarget) as Record<string, any>;
    const socketGrant = structuredClone(fixture.socketGrant) as Record<string, any>;
    plan.scope = scope;
    plan.surface.surfaceId = surfaceId;
    plan.surface.role = id === "a" ? "editor" : "viewer";
    plan.grant.write = id === "a";
    plan.expiresAtUnixMs = Date.now() + 30_000;
    installedTarget.scope = scope;
    installedTarget.surface.surfaceId = surfaceId;
    installedTarget.surface.role = id === "a" ? "editor" : "viewer";
    installedTarget.grant.write = id === "a";
    socketGrant.actorId = `hub.v1.${actorByte.repeat(64)}`;
    socketGrant.grant = `socket.v1.${actorByte.repeat(32)}.${actorByte.repeat(64)}`;
    socketGrant.expiresAtMs = Date.now() + 25_000;
    const root = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(scope.documentId)}`;
    return {
      id,
      scope,
      surfaceId,
      plan,
      installedTarget,
      socketGrant,
      openPath: `${root}/open-plan`,
      grantPath: `${root}/socket-grants`,
      socketPath: `/scopes/${encodeURIComponent(`${spaceId}/${scope.documentId}`)}/document/ws?surface=${encodeURIComponent(surfaceId)}`,
    };
  };
  return [make("a", fixture.expected.scopeIsolation.left.spaceId, "surface.gis.editor", "3"), make("b", fixture.expected.scopeIsolation.right.spaceId, "surface.gis.viewer", "4")];
}

/** 👥️ Runs the real browser Worker behind a mounted React Shell probe for interactive Chromium acceptance. */
async function serveScopedPresenceBrowserRuntime(repoRoot: string): Promise<void> {
  const fixture = browserDocumentOpenFixture(repoRoot);
  const cases = scopedPresenceBrowserCases(fixture);
  const capability = `session.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  const effects = {
    a: { open: 0, exchange: 0, socket: 0, hello: 0, heartbeats: 0, closes: 0 },
    b: { open: 0, exchange: 0, socket: 0, hello: 0, heartbeats: 0, closes: 0 },
  };
  const authority = Bun.serve<{ id: "a" | "b" }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, server): Promise<Response | undefined> {
      const url = new URL(request.url);
      const scopeCase = cases.find((row) => url.pathname === row.openPath || url.pathname === row.grantPath || `${url.pathname}${url.search}` === row.socketPath);
      if (!scopeCase) return new Response("", { status: 404 });
      if (`${url.pathname}${url.search}` === scopeCase.socketPath && request.method === "GET") {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        if (protocols[0] !== "semio.socket.v1" || protocols[1] !== scopeCase.socketGrant.grant || request.headers.has("authorization")) return new Response("", { status: 401 });
        effects[scopeCase.id].socket += 1;
        return server.upgrade(request, { data: { id: scopeCase.id }, headers: { "Sec-WebSocket-Protocol": "semio.socket.v1" } }) ? undefined : new Response("", { status: 500 });
      }
      if (request.method !== "POST" || request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("", { status: 401 });
      const raw = new Uint8Array(await request.arrayBuffer());
      try {
        if (raw.byteLength === 0 || raw.byteLength > 8 * 1024) return new Response("", { status: 413 });
        const input = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(raw)) as Record<string, any>;
        if (url.pathname === scopeCase.openPath) {
          if (input.schema !== fixture.intent.schema || input.version !== 1 || JSON.stringify(input.scope) !== JSON.stringify(scopeCase.scope) || input.requestedSurfaceId !== scopeCase.surfaceId) return new Response("", { status: 400 });
          effects[scopeCase.id].open += 1;
          return Response.json(scopeCase.plan, { headers: { "cache-control": "no-store" } });
        }
        if (url.pathname === scopeCase.grantPath) {
          if (input.schema !== "semio.hub.document-plan-socket-grant-intent/v1" || input.version !== 1 || input.planReceipt !== scopeCase.plan.receipt) return new Response("", { status: 400 });
          effects[scopeCase.id].exchange += 1;
          return Response.json(scopeCase.socketGrant, { headers: { "cache-control": "no-store" } });
        }
        return new Response("", { status: 404 });
      } finally {
        raw.fill(0);
      }
    },
    websocket: {
      message(socket, message): void {
        const scopeCase = cases.find((row) => row.id === socket.data.id)!;
        const decoded = decodeClientFrame(typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message));
        if (typeof decoded.frame === "string") return;
        if ("SocketHelloV1" in decoded.frame) {
          effects[scopeCase.id].hello += 1;
          const frontier = { document_id: scopeCase.scope.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0) };
          socket.send(encodeServerFrame({ Welcome: { session_id: `presence-${scopeCase.id}`, resume_token: `presence-${scopeCase.id}-resume`, server_frontier: frontier, bootstrap: "None" } }, "command"));
          socket.send(encodeServerFrame({ Session: { actor: scopeCase.socketGrant.actorId, color: scopeCase.id === "a" ? 2 : 5 } }, "command"));
          return;
        }
        if (!("Presence" in decoded.frame)) return;
        effects[scopeCase.id].heartbeats += 1;
        const incoming = decodePresencePeer(new Uint8Array(decoded.frame.Presence.peer), [0]);
        const correct: ArtifactPresencePeer = {
          ...incoming,
          actor: scopeCase.socketGrant.actorId,
          userId: scopeCase.id === "a" ? "user-a" : "user-b",
          label: scopeCase.id === "a" ? "Ada" : "Berta",
          role: scopeCase.id === "a" ? "owner" : "viewer",
          connectedAtMs: scopeCase.id === "a" ? 101 : 202,
          color: scopeCase.id === "a" ? 2 : 5,
          surface: scopeCase.surfaceId,
          views: [],
        };
        const hostile: ArtifactPresencePeer = {
          ...correct,
          actor: `intruder-${scopeCase.id}`,
          userId: `intruder-${scopeCase.id}`,
          label: "Wrong Surface",
          surface: scopeCase.id === "a" ? cases[1].surfaceId : cases[0].surfaceId,
        };
        socket.send(encodeServerFrame({ Presence: { peers: [Array.from(encodePresencePeer(correct)), Array.from(encodePresencePeer(hostile))] } }, "preview"));
      },
      close(socket): void {
        effects[socket.data.id].closes += 1;
      },
    },
  });
  const hubOrigin = `http://127.0.0.1:${authority.port}`;
  const prior = {
    S_OS_PORT: process.env.S_OS_PORT,
    S_HUB_URL: process.env.S_HUB_URL,
    S_LOCAL_RELAY_URL: process.env.S_LOCAL_RELAY_URL,
    S_LOCAL_RELAY_SECRET: process.env.S_LOCAL_RELAY_SECRET,
    SEMIO_PLUGIN: process.env.SEMIO_PLUGIN,
    SEMIO_RENDERER: process.env.SEMIO_RENDERER,
  };
  let relay: LocalBrowserRelay | undefined;
  let vite: { close(): Promise<void>; listen(): Promise<unknown>; transformRequest(url: string): Promise<unknown> } | undefined;
  try {
    const uiPort = await freeLoopbackPort();
    const uiOrigin = `http://127.0.0.1:${uiPort}`;
    relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability });
    process.env.S_OS_PORT = String(uiPort);
    process.env.S_HUB_URL = hubOrigin;
    process.env.S_LOCAL_RELAY_URL = relay.url;
    process.env.S_LOCAL_RELAY_SECRET = relay.secret.toString("hex");
    process.env.SEMIO_PLUGIN = "s";
    process.env.SEMIO_RENDERER = "react";
    const componentPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🧬️contracts/👥️presence-scope/🌐️browser/🟦️.tsx");
    const workerPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts");
    const shellEntryPath = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🟦️.ts");
    const browserConfig = {
      hubOrigin,
      workerUrl: `/@fs${workerPath}`,
      cases: cases.map((row) => ({ id: row.id, scope: row.scope, schema: fixture.expected.helloSchema, surfaceId: row.surfaceId, installedTarget: row.installedTarget })),
    };
    const refreshBrowserConfig = async (): Promise<Record<string, unknown>> => {
      if (!relay) throw new Error("scoped presence relay is unavailable");
      const now = Date.now();
      for (const row of cases) {
        row.plan.expiresAtUnixMs = now + 30_000;
        row.socketGrant.expiresAtMs = now + 25_000;
      }
      const binding = { port: Number(new URL(relay.url).port), secret: Buffer.from(relay.secret) };
      await relay.stop();
      relay = startLocalBrowserRelay(hubOrigin, uiOrigin, { schema: "semio.hub.local-credential-envelope/v1", clientClass: "react-relay", capability }, { binding });
      const currentProof = relay.takeBrowserBootstrapProof();
      const proofHex = currentProof.toString("hex");
      currentProof.fill(0);
      return { proof: proofHex, ...browserConfig };
    };
    const html = `<!doctype html><html><head><meta charset="utf-8"><title>Scoped presence browser shell</title><script type="module">import { injectIntoGlobalHook } from "/@react-refresh"; injectIntoGlobalHook(window); window.$RefreshReg$ = () => {}; window.$RefreshSig$ = () => (type) => type;</script><script type="module" src="/@vite/client"></script></head><body><div id="root"></div><script type="module">const root = document.querySelector("#root"); try { const module = await import(${JSON.stringify(`/@fs${componentPath}`)}); const response = await fetch("/__scoped-presence/config"); if (!response.ok) throw new Error(\`config status \${response.status}\`); module.mountScopedPresenceBrowserShellV1(root, await response.json()); } catch (error) { root.textContent = \`scoped-presence mount failed: \${error instanceof Error ? error.message : String(error)}\`; console.error(root.textContent); }</script></body></html>`;
    const { createServer: createViteServer } = await import("vite");
    vite = await createViteServer({
      configFile: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"),
      server: { host: "127.0.0.1", port: uiPort, strictPort: true },
      clearScreen: false,
      plugins: [
        {
          name: "semio-scoped-presence-browser-runtime",
          configureServer(server) {
            server.middlewares.use(async (request, response, next) => {
              if (request.url === "/__scoped-presence/config") {
                response.setHeader("content-type", "application/json; charset=utf-8");
                response.end(JSON.stringify(await refreshBrowserConfig()));
                return;
              }
              if (request.url === "/__scoped-presence/effects") {
                response.setHeader("content-type", "application/json; charset=utf-8");
                response.end(JSON.stringify(effects));
                return;
              }
              if (request.url === "/__scoped-presence") {
                response.setHeader("content-type", "text/html; charset=utf-8");
                response.end(html);
                return;
              }
              next();
            });
          },
        },
      ],
    });
    await vite.listen();
    if ((await vite.transformRequest(`/@fs${componentPath}`)) === null || (await vite.transformRequest(`/@fs${workerPath}`)) === null || (await vite.transformRequest(`/@fs${shellEntryPath}`)) === null)
      throw new Error("scoped presence browser or production shell module did not transform");
    console.log(`scoped-presence-browser-serve: url=${uiOrigin}/__scoped-presence effects=${uiOrigin}/__scoped-presence/effects shell=${uiOrigin}/`);
    await new Promise<void>((resolveStop) => {
      const stop = (): void => resolveStop();
      process.once("SIGINT", stop);
      process.once("SIGTERM", stop);
    });
  } finally {
    await vite?.close().catch(() => {});
    await relay?.stop().catch(() => {});
    await authority.stop(true);
    for (const [key, value] of Object.entries(prior)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
  }
}

export class ScopedPresenceBrowserServeScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length !== 0) throw new Error("scoped-presence-browser-serve accepts no arguments");
    await serveScopedPresenceBrowserRuntime(this.repoRoot);
  }
}

export class DirectoryHomeBrowserProcessCheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const phase = segments[0] ?? "source";
    if (segments.length > 1 || !["source", "runtime", "process"].includes(phase)) throw new Error("directory-home-browser-process-check accepts source, runtime, or process");
    const fixture = await proveDirectoryHomeBrowserProcessSource(this.repoRoot);
    if (phase === "runtime" || phase === "process") await proveDirectoryHomeBrowserControllerRuntime(this.repoRoot, fixture);
    if (phase === "process") {
      runCmd("cargo", ["build", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], { cwd: this.root, budgetMs: buildBudgetMs() });
      await proveDirectoryHomeBrowserProcessJourney(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:home-browser",0),fixture,readNativeCallerAuthorityV1(this.invocation.capabilities).original.buildMode);
    }
    console.log(`directory-home-browser-process-check: phase=${phase} traces=${fixture.traces.length} hostile=${fixture.hostile.length}`);
  }
}
