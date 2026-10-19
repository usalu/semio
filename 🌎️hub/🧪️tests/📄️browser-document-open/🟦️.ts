/** 🧭️ The browser document-open fixture and runtime gate.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { decodeClientFrame, encodeServerFrame } from "@semio-tech/framework-replication";
import { decodeBackboneWorkerResponse, encodeBackboneWorkerRequest } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { runCmd, orchestratorBudgetOpts } from "@semio-tech/process/execution";
import { viteConfigLoader } from "@semio-tech/repo-lib/process";
import { BundleScript } from "@semio-tech/process/routing";
import { hubSchemaExport } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { freeLoopbackPort } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { documentOpenMutation } from "../🗺️open-plan-checks/🟦️.ts";

export type BrowserDocumentOpenFixture = {
  readonly nowMs: number;
  readonly intent: Record<string, any>;
  readonly installedTarget: Record<string, any>;
  readonly plan: Record<string, any>;
  readonly socketGrant: Record<string, any>;
  readonly expected: {
    readonly httpPaths: readonly [string, string];
    readonly webSocketPath: string;
    readonly protocol: string;
    readonly helloSchema: string;
    readonly helloPackSchemaHashByte: number;
    readonly responseMaxBytes: number;
    readonly rustWorkerBypassDenied: true;
    readonly scopeIsolation: {
      readonly left: { readonly spaceId: string; readonly documentId: string };
      readonly right: { readonly spaceId: string; readonly documentId: string };
      readonly leftKey: string;
      readonly rightKey: string;
      readonly localKey: string;
    };
    readonly forbiddenSocketFragments: readonly string[];
  };
  readonly hostile: readonly { readonly name: string; readonly stage: string; readonly replacePath?: string; readonly value?: unknown; readonly expected: string }[];
};

export function browserDocumentOpenFixture(repoRoot: string): BrowserDocumentOpenFixture {
  const root = join(repoRoot, "🧰️framework/🛍️products/💻️os/🧪️tests/🧫️fixtures/📇️directory");
  const fixture = JSON.parse(readFileSync(join(root, "🌐️browser-document-open-v1.json"), "utf8")) as BrowserDocumentOpenFixture;
  const validatePlan = hubSchemaExport(repoRoot, "schema://os.directory/BrowserDocumentOpenTransportPlan");
  for (const name of ["parent-standard-control", "parent-subset-trim"]) {
    const vector = fixture.hostile.find((row) => row.name === name)!;
    if (validatePlan(documentOpenMutation(fixture.plan, vector.replacePath!, vector.value))) throw new Error(`browser document-open schema admitted ${name}`);
  }
  return fixture;
}

/** ⚖️ Independent full-field lease relation. It is hand-written here (no production import) and
 * compares every plan-projected lease field to the installed execution target, including both
 * component digests, byte lengths, catalog generation, checkpoint and revalidation. The renderer
 * target is whatever the installation declares: a non-`react` target is a lease-path decision, not
 * a fixture constant. */
function browserDocumentOpenAuthority(plan: Record<string, any>, fixture: BrowserDocumentOpenFixture): boolean {
  const intent = fixture.intent;
  const installed = fixture.installedTarget;
  const sameJson = (left: unknown, right: unknown): boolean => JSON.stringify(left) === JSON.stringify(right);
  return (
    installed.schema === "semio.os.document-execution-target-lease/v1" &&
    installed.version === 1 &&
    plan.scope?.spaceId === intent.scope?.spaceId &&
    plan.scope?.documentId === intent.scope?.documentId &&
    sameJson(plan.scope, installed.scope) &&
    plan.descriptorDigestV1 === installed.descriptorDigestV1 &&
    sameJson(plan.catalog, installed.catalog) &&
    sameJson(plan.package, installed.package) &&
    installed.component?.sha256 === installed.package?.componentSha256 &&
    installed.component?.blake3 === installed.package?.componentBlake3 &&
    installed.descriptor?.sha256 === installed.package?.descriptorByteSha256 &&
    Number.isSafeInteger(installed.component?.byteLength) &&
    installed.component.byteLength >= 1 &&
    installed.component.byteLength <= DOCUMENT_EXECUTION_TARGET_COMPONENT_MAX_BYTES &&
    Number.isSafeInteger(installed.descriptor?.byteLength) &&
    installed.descriptor.byteLength >= 1 &&
    installed.descriptor.byteLength <= 4 * 1024 * 1024 &&
    sameJson(plan.artifact, installed.artifact) &&
    plan.artifact?.schema === fixture.expected.helloSchema &&
    plan.artifact?.packSchemaHash === fixture.expected.helloPackSchemaHashByte.toString(16).padStart(2, "0").repeat(32) &&
    sameJson(plan.parentDialect, installed.parentDialect) &&
    plan.parentDialect?.artifactKind === plan.artifact?.kind &&
    sameJson(plan.surface, installed.surface) &&
    sameJson(plan.browserActor?.kind === "none" ? plan.browserActor : { ...plan.browserActor, byteLength: installed.browserActor?.byteLength }, installed.browserActor) &&
    plan.surface?.surfaceId === intent.requestedSurfaceId &&
    sameJson(plan.grant, installed.grant) &&
    plan.grant?.write === (plan.surface?.role === "editor") &&
    sameJson(plan.checkpoint, installed.checkpoint) &&
    sameJson(plan.revalidation, installed.revalidation) &&
    plan.expiresAtUnixMs > fixture.nowMs &&
    plan.expiresAtUnixMs - fixture.nowMs <= 30_000
  );
}

export async function proveBrowserDocumentOpenFixture(repoRoot: string): Promise<BrowserDocumentOpenFixture> {
  const fixture = browserDocumentOpenFixture(repoRoot);
  const scope = fixture.intent.scope;
  const root = `/spaces/${encodeURIComponent(scope.spaceId)}/documents/${encodeURIComponent(scope.documentId)}`;
  const httpPaths = [`${root}/open-plan`, `${root}/socket-grants`];
  const webSocketPath = `/scopes/${encodeURIComponent(`${scope.spaceId}/${scope.documentId}`)}/document/ws?surface=${encodeURIComponent(fixture.intent.requestedSurfaceId)}`;
  if (JSON.stringify(httpPaths) !== JSON.stringify(fixture.expected.httpPaths) || webSocketPath !== fixture.expected.webSocketPath) throw new Error("browser document-open encoded path oracle mismatch");
  if (!browserDocumentOpenAuthority(fixture.plan, fixture)) throw new Error("browser document-open plan authority oracle mismatch");
  const runtimeKey = (scope: { readonly spaceId: string; readonly documentId: string }): string => `v1:${Buffer.byteLength(scope.spaceId, "utf8")}:${Buffer.byteLength(scope.documentId, "utf8")}:${scope.spaceId}${scope.documentId}`;
  const isolation = fixture.expected.scopeIsolation;
  const localKey = `local:v1:${Buffer.byteLength(isolation.left.documentId, "utf8")}:${isolation.left.documentId}`;
  if (runtimeKey(isolation.left) !== isolation.leftKey || runtimeKey(isolation.right) !== isolation.rightKey || localKey !== isolation.localKey || isolation.leftKey === isolation.rightKey || isolation.leftKey === isolation.localKey)
    throw new Error("browser document-open scope-key oracle mismatch");
  const exchange = { schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: fixture.plan.receipt };
  if (Object.keys(exchange).join(",") !== "schema,version,planReceipt" || fixture.socketGrant.expiresAtMs > fixture.plan.expiresAtUnixMs)
    throw new Error("browser document-open receipt exchange oracle mismatch");
  const hello = { SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: fixture.plan.artifact.schema, pack_schema_hash: new Array(32).fill(fixture.expected.helloPackSchemaHashByte), resume_token: null, frontier: null } };
  const publicTransport = JSON.stringify({ webSocketPath, protocol: fixture.expected.protocol, hello });
  for (const fragment of fixture.expected.forbiddenSocketFragments) if (publicTransport.includes(fragment)) throw new Error(`browser document-open public transport leaked ${fragment}`);
  if (fixture.expected.rustWorkerBypassDenied !== true) throw new Error("browser document-open execution ownership oracle drift");
  let hostile = 0;
  for (const vector of fixture.hostile) {
    if (vector.expected === "invalid-plan") {
      const candidate = documentOpenMutation(fixture.plan, vector.replacePath!, vector.value);
      if (browserDocumentOpenAuthority(candidate, fixture)) throw new Error(`browser document-open authority oracle admitted ${vector.name}`);
    } else if (vector.expected === "response-too-large") {
      if (vector.value !== fixture.expected.responseMaxBytes + 1) throw new Error("browser document-open max-plus-one oracle drift");
    } else if (vector.expected === "cancelled-before-exchange") {
      if (vector.stage !== "exchange" || httpPaths[0] === httpPaths[1]) throw new Error("browser document-open cancellation sequence oracle drift");
    } else if (vector.expected === "credential-free") {
      if (fixture.expected.forbiddenSocketFragments.some((fragment) => webSocketPath.includes(fragment))) throw new Error("browser document-open socket URL is credential-bearing");
    } else if (vector.expected === "withheld-activation") {
      if (vector.stage !== "session" || vector.value === fixture.socketGrant.actorId) throw new Error("browser document-open mismatched Session activation oracle drift");
    } else {
      throw new Error(`browser document-open unknown hostile outcome ${vector.expected}`);
    }
    hostile += 1;
  }
  console.log(
    `browser-document-open-oracle: ajv=1 paths=3 installed-target=1 scope-keys=2 authority=1 exchange=1 websocket=1 rust-worker-bypass=denied hostile=${hostile} bound=${fixture.expected.responseMaxBytes} redaction=${fixture.expected.forbiddenSocketFragments.length} passed`,
  );
  return fixture;
}

async function proveBrowserDocumentOpenRuntime(repoRoot: string, fixture: BrowserDocumentOpenFixture): Promise<void> {
  const current = structuredClone(fixture);
  current.plan.expiresAtUnixMs = Date.now() + 30_000;
  current.socketGrant.expiresAtMs = Date.now() + 25_000;
  const effects = { open: 0, exchange: 0, socket: 0, hello: 0 };
  let documentSocket: { send(data: Uint8Array): void } | undefined;
  const capability = `session.v1.${"a".repeat(32)}.${"b".repeat(64)}`;
  const authorityServer = Bun.serve<{ admitted: boolean }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, server): Promise<Response | undefined> {
      const url = new URL(request.url);
      if (request.method === "POST" && [current.expected.httpPaths[0], current.expected.httpPaths[1]].includes(url.pathname)) {
        if (request.headers.get("authorization") !== `Bearer ${capability}` || request.headers.get("content-type") !== "application/json" || url.search !== "") return new Response("", { status: 401 });
        const bytes = new Uint8Array(await request.arrayBuffer());
        try {
          if (bytes.byteLength === 0 || bytes.byteLength > 8 * 1024) return new Response("", { status: 413 });
          const value = JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(bytes));
          if (url.pathname === current.expected.httpPaths[0]) {
            const intent = value as Record<string, any>;
            const clientInstanceBytes = typeof intent.clientInstanceId === "string" ? Buffer.byteLength(intent.clientInstanceId, "utf8") : 0;
            if (
              JSON.stringify(Object.keys(intent).sort()) !== JSON.stringify(["clientInstanceId", "requestedSurfaceId", "schema", "scope", "version"]) ||
              intent.schema !== current.intent.schema ||
              intent.version !== current.intent.version ||
              JSON.stringify(intent.scope) !== JSON.stringify(current.intent.scope) ||
              intent.requestedSurfaceId !== current.intent.requestedSurfaceId ||
              clientInstanceBytes === 0 ||
              clientInstanceBytes > 128 ||
              /[\u0000-\u001f\u007f]/u.test(intent.clientInstanceId)
            )
              return new Response("", { status: 400 });
            effects.open += 1;
            return Response.json(current.plan, { headers: { "cache-control": "no-store" } });
          }
          if (JSON.stringify(value) !== JSON.stringify({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: current.plan.receipt })) return new Response("", { status: 400 });
          effects.exchange += 1;
          return Response.json(current.socketGrant, { headers: { "cache-control": "no-store" } });
        } finally {
          bytes.fill(0);
        }
      }
      if (request.method === "GET" && `${url.pathname}${url.search}` === current.expected.webSocketPath) {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        if (protocols.length !== 2 || protocols[0] !== current.expected.protocol || protocols[1] !== capability || request.headers.has("authorization")) return new Response("", { status: 401 });
        effects.socket += 1;
        return server.upgrade(request, { data: { admitted: true }, headers: { "Sec-WebSocket-Protocol": current.expected.protocol } }) ? undefined : new Response("", { status: 500 });
      }
      return new Response("", { status: 404 });
    },
    websocket: {
      message(socket, message): void {
        if (!socket.data.admitted || effects.hello !== 0) return;
        const bytes = typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message);
        const decoded = decodeClientFrame(bytes);
        if (typeof decoded.frame === "string" || !("SocketHelloV1" in decoded.frame)) return;
        const hello = decoded.frame.SocketHelloV1;
        if (hello.schema !== current.expected.helloSchema || hello.pack_schema_hash.length !== 32 || hello.pack_schema_hash.some((byte) => byte !== current.expected.helloPackSchemaHashByte) || hello.resume_token !== null || hello.frontier !== null)
          return;
        effects.hello += 1;
        documentSocket = socket;
        const frontier = { document_id: current.intent.scope.documentId, head_edit_ordinal: 0, head_edit_id: "", last_commit_seq: 0, chain_hash: new Array(32).fill(0) };
        socket.send(encodeServerFrame({ Welcome: { session_id: "browser-open-session", resume_token: "browser-open-resume", server_frontier: frontier, bootstrap: "None" } }, "command"));
      },
    },
  });
  const hubOrigin = `http://127.0.0.1:${authorityServer.port}`;
  const openWire = Array.from(
    encodeBackboneWorkerRequest({
      kind: "open",
      actorIdentityGrant: { maximumItems: 1024, maximumCopyBytes: 65536, maximumCapacityBytes: 65536, maximumReleaseBytes: 65536, maximumDepth: 64 },
      documentId: current.intent.scope.documentId,
      schema: current.expected.helloSchema,
      bindings: [{ kind: "hub", baseUrl: hubOrigin, spaceId: current.intent.scope.spaceId, dataClass: "persistedShared", installedTarget: current.installedTarget as unknown as import("@semio-tech/framework-os").DocumentExecutionTargetLeaseFieldsV1 }],
      actor: "browser-untrusted-actor",
      packSchemaHash: new Array(32).fill(current.expected.helloPackSchemaHashByte),
    }),
  );
  let viteServer: { close(): Promise<void>; listen(): Promise<unknown> } | undefined;
  let browser: Awaited<ReturnType<(typeof import("playwright"))["chromium"]["launch"]>> | undefined;
  const browserDiagnostics: string[] = [];
  const priorViteEnvironment = {
    S_OS_PORT: process.env.S_OS_PORT,
    S_HUB_URL: process.env.S_HUB_URL,
    S_LOCAL_RELAY_URL: process.env.S_LOCAL_RELAY_URL,
    S_LOCAL_RELAY_SECRET: process.env.S_LOCAL_RELAY_SECRET,
    SEMIO_PLUGIN: process.env.SEMIO_PLUGIN,
    SEMIO_RENDERER: process.env.SEMIO_RENDERER,
  };
  try {
    const uiPort = await freeLoopbackPort();
    const uiOrigin = `http://127.0.0.1:${uiPort}`;
    process.env.S_OS_PORT = String(uiPort);
    process.env.S_HUB_URL = hubOrigin;
    delete process.env.S_LOCAL_RELAY_URL;
    delete process.env.S_LOCAL_RELAY_SECRET;
    process.env.SEMIO_PLUGIN = "s";
    process.env.SEMIO_RENDERER = "react";
    const { createServer: createViteServer } = await import("vite");
    viteServer = await createViteServer({
      configFile: join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts"),
      configLoader: viteConfigLoader(),
      server: { host: "127.0.0.1", port: uiPort, strictPort: true },
      clearScreen: false,
    });
    await viteServer.listen();
    const { chromium } = await import("playwright");
    browser = await chromium.launch({ headless: true });
    const page = await browser.newPage();
    page.on("console", (message) => browserDiagnostics.push(`console:${message.type()}:${message.text()}`));
    page.on("pageerror", (error) => browserDiagnostics.push(`pageerror:${error.message}`));
    page.on("requestfailed", (request) => browserDiagnostics.push(`requestfailed:${request.url()}:${request.failure()?.errorText ?? "unknown"}`));
    page.on("response", (response) => {
      if (response.url().includes("/_semio/hub/")) browserDiagnostics.push(`response:${response.status()}:${response.url()}`);
    });
    await page.goto(uiOrigin, { waitUntil: "commit", timeout: 10_000 });
    await page.setContent("<!doctype html><meta charset=utf-8>");
    current.plan.expiresAtUnixMs = Date.now() + 30_000;
    current.socketGrant.expiresAtMs = Date.now() + 25_000;
    await page.evaluate(
      ({ workerUrl, capability, openWire }) => {
        const state: { messages: unknown[]; errors: string[]; started: boolean; worker?: Worker } = ((globalThis as any).__semio = { messages: [], errors: [], started: false });
        history.replaceState(history.state, "", `${location.pathname}${location.search}`);
        const worker = new Worker(workerUrl, { type: "module" });
        const channel = new MessageChannel();
        worker.postMessage({ kind: "semio-hub-session-port", port: channel.port2 }, [channel.port2]);
        channel.port1.onmessage = (event) => {
          if (event.data?.kind !== "initialized" || state.started) return;
          if (event.data.ok !== true) {
            state.errors.push("broker bootstrap rejected");
            return;
          }
          state.started = true;
          worker.postMessage({ wire: new Uint8Array(openWire) });
        };
        channel.port1.start();
        channel.port1.postMessage({ kind: "initialize", capability });
        worker.onerror = (event) => state.errors.push(String(event.message ?? "worker error"));
        worker.onmessage = (event) => {
          state.messages.push(event.data?.wire ? Array.from(event.data.wire) : event.data);
        };
        state.worker = worker;
      },
      { workerUrl: `/@fs${join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts")}`, capability, openWire },
    );
    await page.waitForFunction(() => (globalThis as any).__semio?.errors?.length > 0 || (globalThis as any).__semio?.messages?.length > 1, undefined, { timeout: 10_000 });
    const deadline = Date.now() + 10_000;
    let browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
    let activation = browserState.messages.flatMap((wire: number[]) => {
      try {
        const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
        return message.kind === "socket-actor" ? [message] : [];
      } catch {
        return [];
      }
    });
    while (effects.hello !== 1 && Date.now() < deadline) {
      await Bun.sleep(10);
      browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
      activation = browserState.messages.flatMap((wire: number[]) => {
        try {
          const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
          return message.kind === "socket-actor" ? [message] : [];
        } catch {
          return [];
        }
      });
    }
    if (effects.hello !== 1 || activation.length !== 0 || !documentSocket) throw new Error(`browser document-open activated before authenticated Session effects=${JSON.stringify(effects)} activation=${activation.length}`);
    documentSocket.send(encodeServerFrame({ Session: { actor: current.socketGrant.actorId, color: 7 } }, "command"));
    while (activation.length !== 1 && Date.now() < deadline) {
      await Bun.sleep(10);
      browserState = await page.evaluate(() => ({ hash: location.hash, errors: (globalThis as any).__semio?.errors ?? [], messages: (globalThis as any).__semio?.messages ?? [] }));
      activation = browserState.messages.flatMap((wire: number[]) => {
        try {
          const message = decodeBackboneWorkerResponse(Uint8Array.from(wire));
          return message.kind === "socket-actor" ? [message] : [];
        } catch {
          return [];
        }
      });
    }
    if (
      effects.open !== 1 ||
      effects.exchange !== 1 ||
      effects.socket !== 1 ||
      effects.hello !== 1 ||
      activation.length !== 1 ||
      activation[0]!.actorId !== current.socketGrant.actorId ||
      browserState.hash !== "" ||
      browserState.errors.length !== 0 ||
      browserState.messages.length < 2
    )
      throw new Error(`browser document-open runtime mismatch effects=${JSON.stringify(effects)} state=${JSON.stringify(browserState)} diagnostics=${browserDiagnostics.slice(-8).join("|")}`);
    console.log("browser-document-open-runtime: chromium-worker=1 authenticated-open=1 receipt-exchange=1 credential-free-websocket=1 authoritative-tag7=1 pre-session-activation=0 matched-session-activation=1 fragment-cleared=1 passed");
  } finally {
    await browser?.close().catch(() => undefined);
    await viteServer?.close().catch(() => undefined);
    await authorityServer.stop(true);
    for (const [key, value] of Object.entries(priorViteEnvironment)) {
      if (value === undefined) delete process.env[key];
      else process.env[key] = value;
    }
  }
}

export class BrowserDocumentOpenCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixture = await proveBrowserDocumentOpenFixture(this.repoRoot);
    await proveBrowserDocumentOpenRuntime(this.repoRoot, fixture);
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os:test", "--skip-cache", "--", "quick", "--run", "-t", "browser document open"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["./📜️script.ts", "open-plan-server-check"], { cwd: this.root, ...orchestratorBudgetOpts() });
    console.log("browser-document-open-check: neutral oracle, browser Worker D1 issue/exchange/WebSocket runtime, Session-gated activation, hostile bounds/cancellation/redaction, and current server laws passed");
  }
}
