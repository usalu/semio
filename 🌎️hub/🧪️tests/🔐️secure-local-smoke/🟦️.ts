/** 🧭️ The secure local smoke and native document-open gates: credential envelope delivery and the native socket grant actor.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { scriptCompilerAdmissionControlV1 as exactCargoCallerControlV1 } from "@semio-tech/process/compiler-capacity";
import { type CompilerAdmissionControl } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { type ScriptInvocation } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { randomBytes } from "node:crypto";
import { existsSync } from "node:fs";
import { spawn, type ChildProcess } from "node:child_process";
import { decodeClientFrame, encodeServerFrame, type WireFrontierSummary } from "../../../🧰️framework/🔨️modules/📡️replication/🟦️.ts";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 } from "@semio-tech/framework/directory/schema";
import { runCargo, runCmd, runProbe, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { DIRECT_CHILD_BENIGN_ENV_KEY, DIRECT_CHILD_BENIGN_ENV_VALUE, deliverCredentialEnvelopeToChild, deliverMcpCredentialEnvelope, deliverNativeCredentialEnvelope, sealedDirectChildEnvironment } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { proveMcpCredentialSourceOrder, proveNativeCredentialSourceOrder } from "../../🔐️auth/🧪️tests/🧭️credential-source-order/🟦️.ts";
import { issueLocalCredential } from "../../🚀️local-bootstrap/🔐️credential-issuance/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalProfile } from "../../🚀️local-bootstrap/🛂authentication/🟦️.ts";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, writeLocalFrame } from "../../🚀️local-bootstrap/📡️framing/🟦️.ts";
import { finishLocalHub, type LocalHubRun, waitForLocalHubExitV1, withPublishedLocalHubHostV1, startLocalHub, waitForChildExit, waitForReadiness } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { proveDocumentOpenPlanFixture } from "../🗺️open-plan-checks/🟦️.ts";
import { buildAdminSpa, jsonRpcResponses, mcpExecutable, nativeWgpuExecutable, startMcpWorkspaceChild, waitForJsonRpcResponse } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";

async function activeMcpDocumentConnections(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string): Promise<Record<string, any>[]> {
  const rows: Record<string, any>[] = [];
  const cursors = new Set<string>();
  let cursor: string | undefined;
  for (let page = 0; page < 64; page++) {
    const url = new URL(`http://127.0.0.1:${run.port}/admin/api/connections`);
    url.searchParams.set("limit", "100");
    if (cursor) url.searchParams.set("cursor", cursor);
    const response = await fetch(url, { headers: { authorization: `Bearer ${adminEnvelope.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (!response.ok) throw new Error(`MCP process probe could not inspect active connections: ${response.status}`);
    const body = await response.json();
    if (body?.source !== "recorded-sync-sessions" || !Array.isArray(body.rows) || body.rows.length > 100 || (body.nextCursor !== undefined && typeof body.nextCursor !== "string"))
      throw new Error("MCP process probe active connections response was not a bounded snapshot page");
    rows.push(...body.rows);
    if (rows.length > 4_096) throw new Error("MCP process probe active connections exceeded the fixed read ceiling");
    cursor = body.nextCursor;
    if (!cursor) return rows.filter((connection) => connection?.scope?.spaceId === spaceId && connection?.scope?.documentId === documentId);
    if (cursors.has(cursor)) throw new Error("MCP process probe active connection cursor repeated");
    cursors.add(cursor);
  }
  throw new Error("MCP process probe active connection page ceiling exceeded");
}

async function waitForMcpDocumentConnection(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string, priorSyncSessionId?: string): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const connections = await activeMcpDocumentConnections(run, adminEnvelope, spaceId, documentId);
    const fresh = connections.filter((connection) => typeof connection.syncSessionId === "string" && connection.syncSessionId !== priorSyncSessionId && typeof connection.authenticatedUserId === "string" && !("actor" in connection));
    if (fresh.length === 1 && (priorSyncSessionId === undefined || connections.every((connection) => connection.syncSessionId !== priorSyncSessionId))) return fresh[0]!;
    await Bun.sleep(25);
  }
  throw new Error("MCP document SocketGrant/Session connection deadline exceeded");
}

function mcpDocumentGrantSelectorDigests(diagnostics: string, spaceId: string, documentId: string): string[] {
  const path = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(documentId)}/socket-grants`;
  const prefix = `[semio-directory-client] socket-grant-selector-digest ${path} `;
  return diagnostics
    .split("\n")
    .filter((line) => line.startsWith(prefix))
    .map((line) => line.slice(prefix.length))
    .filter((digest) => /^[0-9a-f]{64}$/u.test(digest));
}

async function waitForMcpDocumentGrantSelector(child: ChildProcess, chunks: readonly Buffer[], spaceId: string, documentId: string, count: number): Promise<string[]> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    const digests = mcpDocumentGrantSelectorDigests(Buffer.concat(chunks).toString("utf8"), spaceId, documentId);
    if (digests.length >= count) return digests;
    if (child.exitCode !== null) throw new Error(`MCP direct child exited before document SocketGrant receipt ${count}: ${child.exitCode}`);
    if (Date.now() >= deadline) throw new Error(`MCP document SocketGrant receipt ${count} deadline exceeded`);
    await Bun.sleep(20);
  }
}

async function waitForMcpDocumentHead(run: LocalHubRun, adminEnvelope: Record<string, any>, spaceId: string, documentId: string, minimumHeadSeq: number): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  while (Date.now() < deadline) {
    const response = await fetch(`http://127.0.0.1:${run.port}/admin/api/documents?limit=100&space=${encodeURIComponent(spaceId)}`, { headers: { authorization: `Bearer ${adminEnvelope.capability}` }, signal: AbortSignal.timeout(2_000) });
    if (!response.ok) throw new Error(`MCP process probe could not inspect document state: ${response.status}`);
    const body = await response.json();
    if (!Array.isArray(body?.rows) || body.rows.length > 100) throw new Error("MCP process probe document response was not a bounded page");
    const document = body.rows.find((candidate: Record<string, any>) => candidate?.descriptor?.documentId === documentId);
    if (Number.isSafeInteger(document?.headSeq) && document.headSeq >= minimumHeadSeq) return document;
    await Bun.sleep(25);
  }
  throw new Error(`MCP document command was not durably accepted through head sequence ${minimumHeadSeq}`);
}

async function proveMcpWorkspaceProcess(repoRoot: string, run: LocalHubRun, envelope: Record<string, any>, adminEnvelope: Record<string, any>): Promise<void> {
  const protectedValues = ["poison-user", "poison-session", "poison-token", "poison-origin", "poison-auth", "poison-cookie"];
  const poisonedEnvironment = {
    ...process.env,
    [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE,
    SEMIO_DIRECT_CHILD_PROBE: "1",
    S_USER: protectedValues[0],
    S_SESSION: protectedValues[1],
    NPM_TOKEN: protectedValues[2],
    S_HUB_URL: protectedValues[3],
    AUTHORIZATION: protectedValues[4],
    COOKIE: protectedValues[5],
  };
  const { child, spaceId, documentId } = await startMcpWorkspaceChild(repoRoot, run, envelope, poisonedEnvironment);
  const stdout: Buffer[] = [];
  const stderr: Buffer[] = [];
  let stdoutBytes = 0;
  let stderrBytes = 0;
  child.stdout?.on("data", (chunk: Buffer) => {
    stdoutBytes += chunk.length;
    if (stdoutBytes <= 32_768) stdout.push(Buffer.from(chunk));
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    stderrBytes += chunk.length;
    if (stderrBytes <= 16_384) stderr.push(Buffer.from(chunk));
  });
  try {
    const deadline = Date.now() + 10_000;
    while (!Buffer.concat(stderr).toString("utf8").includes("real per-capability ArtifactChannel routing bound")) {
      if (child.exitCode !== null) throw new Error(`MCP direct child exited before its authenticated workspace bound: ${child.exitCode}`);
      if (Date.now() >= deadline) throw new Error("MCP direct child did not bind its authenticated workspace before deadline");
      await Bun.sleep(20);
    }
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 1, method: "initialize", params: { protocolVersion: "2025-11-25", capabilities: {}, clientInfo: { name: "semio-direct-child-oracle", version: "1" } } })}\n`);
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 2, method: "tools/call", params: { name: "artifact_open", arguments: { artifactId: documentId } } })}\n`);
    const opened = await waitForJsonRpcResponse(child, stdout, 2);
    if (opened.result?.isError !== false || opened.result?.structuredContent?.artifactId !== documentId) throw new Error("MCP artifact_open did not open the authenticated document transport");
    const firstConnection = await waitForMcpDocumentConnection(run, adminEnvelope, spaceId, documentId);
    const firstSelectors = await waitForMcpDocumentGrantSelector(child, stderr, spaceId, documentId, 1);
    const persisted = await waitForMcpDocumentHead(run, adminEnvelope, spaceId, documentId, 1);
    const firstSessionId = firstConnection.syncSessionId as string;
    const firstAuthenticatedUserId = firstConnection.authenticatedUserId as string;
    const closed = await fetch(`http://127.0.0.1:${run.port}/admin/api/intents`, {
      method: "POST",
      headers: { authorization: `Bearer ${adminEnvelope.capability}`, "content-type": "application/json" },
      body: JSON.stringify({
        kind: "kick-connection",
        requestId: `mcp-reconnect-${randomBytes(12).toString("hex")}`,
        syncSessionId: firstSessionId,
        reasonCode: "mcp-process-reconnect-law",
      }),
      signal: AbortSignal.timeout(2_000),
    });
    const closedReceipt = closed.ok ? await closed.json() : undefined;
    if (closed.status !== 200 || closedReceipt?.state !== "succeeded" || closedReceipt?.outcome?.code !== "connection-kick-signalled" || closedReceipt?.outcome?.kickSignalled !== 1) {
      throw new Error(`MCP process probe could not force document reconnect through admin intent: ${closed.status}`);
    }
    const reconnected = await waitForMcpDocumentConnection(run, adminEnvelope, spaceId, documentId, firstSessionId);
    const reconnectedSelectors = await waitForMcpDocumentGrantSelector(child, stderr, spaceId, documentId, 2);
    if (reconnected.authenticatedUserId !== firstAuthenticatedUserId) throw new Error("MCP document reconnect changed its authenticated subject");
    if (reconnected.syncSessionId === firstSessionId) throw new Error("MCP document reconnect reused its sync-session identity");
    if (reconnectedSelectors[1] === firstSelectors[0]) throw new Error("MCP document reconnect reused its one-use SocketGrant selector");
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id: 3, method: "tools/call", params: { name: "artifact_snapshot", arguments: { artifactId: documentId } } })}\n`);
    const snapshotted = await waitForJsonRpcResponse(child, stdout, 3);
    if (snapshotted.result?.isError !== false || snapshotted.result?.structuredContent?.artifactId !== documentId) throw new Error("MCP post-reconnect artifact state-machine operation failed");
    if (snapshotted.result?.structuredContent?.headSeq !== undefined && snapshotted.result.structuredContent.headSeq < persisted.headSeq) throw new Error("MCP post-reconnect snapshot regressed behind the persisted document frontier");
    child.stdin?.end();
    await waitForChildExit(child, 5_000);
    const output = Buffer.concat(stdout);
    const diagnostics = Buffer.concat(stderr).toString("utf8");
    const outputText = output.toString("utf8");
    let responses: Record<string, any>[] = [];
    try {
      responses = jsonRpcResponses(stdout);
    } catch {}
    const initialize = responses.find((response) => response.id === 1);
    if (child.exitCode !== 0 || stdoutBytes !== output.length || stdoutBytes > 32_768 || stderrBytes > 16_384 || responses.length !== 3 || initialize?.jsonrpc !== "2.0" || initialize?.result?.protocolVersion !== "2025-11-25") {
      throw new Error(`MCP direct child byte-clean JSON-RPC law failed: exit=${child.exitCode} stdoutBytes=${stdoutBytes} stderrBytes=${stderrBytes}`);
    }
    if (output.includes(Buffer.from(envelope.capability)) || diagnostics.includes(envelope.capability) || protectedValues.some((value) => outputText.includes(value) || diagnostics.includes(value)))
      throw new Error("MCP direct child leaked protected parent state");
  } finally {
    stdout.forEach((bytes) => bytes.fill(0));
    stderr.forEach((bytes) => bytes.fill(0));
    terminateOwnedChildTree(child);
  }
}

async function proveNativeCredentialEnvelopeDelivery(repoRoot: string, envelope: Record<string, any>, hubOrigin: string): Promise<void> {
  const executable = nativeWgpuExecutable(repoRoot);
  if (!existsSync(executable)) throw new Error("owned WGPU native binary missing after native-build");
  const protectedValues = ["poison-user", "poison-session", "poison-token", "poison-origin", "poison-auth", "poison-cookie"];
  const poisonedEnvironment = {
    ...process.env,
    [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE,
    S_USER: protectedValues[0],
    S_SESSION: protectedValues[1],
    NPM_TOKEN: protectedValues[2],
    S_HUB_URL: protectedValues[3],
    AUTHORIZATION: protectedValues[4],
    COOKIE: protectedValues[5],
  };
  const child = await deliverCredentialEnvelopeToChild(executable, ["--credential-probe"], structuredClone(envelope), "native", hubOrigin, poisonedEnvironment);
  let output = "";
  let diagnostics = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 256) output += chunk.toString("utf8");
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    if (diagnostics.length < 4_096) diagnostics += chunk.toString("utf8");
  });
  await waitForChildExit(child, 5_000);
  if (child.exitCode !== 0 || output !== "native-credential-probe-ok\n" || output.includes(envelope.capability) || diagnostics.includes(envelope.capability) || protectedValues.some((value) => output.includes(value) || diagnostics.includes(value)))
    throw new Error("actual WGPU credential claim/descendant environment seal failed");
  await proveNonFd3CredentialMarkerRejection(executable, "native");
}

async function proveNonFd3CredentialMarkerRejection(executable: string, client: "native" | "mcp"): Promise<void> {
  const poison = `session.v1.${"c".repeat(32)}.${"d".repeat(64)}`;
  const environment = sealedDirectChildEnvironment(process.env);
  environment.S_LOCAL_CREDENTIAL_FD = poison;
  const child = spawn(executable, ["--assert-no-local-credential-state"], { shell: false, env: environment, stdio: ["ignore", "pipe", "pipe"] });
  let output = "";
  let diagnostics = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 1_024) output += chunk.toString("utf8");
  });
  child.stderr?.on("data", (chunk: Buffer) => {
    if (diagnostics.length < 4_096) diagnostics += chunk.toString("utf8");
  });
  await waitForChildExit(child, 5_000);
  if (child.exitCode === 0 || output.includes(poison) || diagnostics.includes(poison)) throw new Error(`${client} entrypoint admitted or leaked a non-fd3 credential marker`);
}

async function proveNativeSocketGrantActor(repoRoot: string): Promise<void> {
  const actor = `hub.v1.${"a".repeat(64)}`;
  const wrongActor = `hub.v1.${"b".repeat(64)}`;
  const capability = `session.v1.${"c".repeat(32)}.${"d".repeat(64)}`;
  let pendingGrants = 0;
  const planReceipts = new Map<string, boolean>();
  const probePackSchemaHash = "0302ac7cf70cd2452759542a24931331935527fb9388bb6967cbc0575ccf728e";
  let planCount = 0;
  let exchangeCount = 0;
  let grantCount = 0;
  let socketCount = 0;
  let helloCount = 0;
  let mutationCount = 0;
  let preSessionCommands = 0;
  const frontier = (ordinal: number): WireFrontierSummary => ({
    document_id: "probe-document",
    head_edit_ordinal: ordinal,
    head_edit_id: ordinal === 0 ? "" : `probe-${ordinal}`,
    last_commit_seq: ordinal,
    chain_hash: new Array(32).fill(ordinal),
  });
  const server = Bun.serve<{ connection: number }>({
    hostname: "127.0.0.1",
    port: 0,
    async fetch(request, control): Promise<Response | undefined> {
      const url = new URL(request.url);
      if (request.method === "POST" && url.pathname === "/spaces/probe-space/documents/probe-document/open-plan") {
        if (request.headers.get("authorization") !== `Bearer ${capability}` || request.headers.get("content-length") === "0") return new Response("", { status: 401 });
        const intent = (await request.json().catch(() => undefined)) as Record<string, any> | undefined;
        if (
          intent?.schema !== "semio.hub.document-open-intent/v1" ||
          intent?.version !== 1 ||
          intent?.scope?.spaceId !== "probe-space" ||
          intent?.scope?.documentId !== "probe-document" ||
          intent?.requestedSurfaceId !== "native.socket-grant.probe.editor" ||
          typeof intent?.clientInstanceId !== "string" ||
          !/^native-document-[0-9a-f]{16}$/u.test(intent.clientInstanceId) ||
          Object.keys(intent).some((key) => !["schema", "version", "scope", "requestedSurfaceId", "clientInstanceId"].includes(key))
        )
          return new Response("", { status: 400 });
        planCount += 1;
        const planReceipt = `open.v1.${Buffer.alloc(32, planCount).toString("base64url")}`;
        planReceipts.set(planReceipt, false);
        const descriptorDigestV1 = "11".repeat(32);
        return Response.json({
          schema: "semio.hub.document-open-plan/v1",
          version: 1,
          receipt: planReceipt,
          expiresAtUnixMs: Date.now() + 20_000,
          scope: { spaceId: "probe-space", documentId: "probe-document" },
          descriptorDigestV1,
          catalog: { generationId: "22".repeat(32) },
          package: {
            pluginId: "socket-grant-probe",
            packageId: "semio:socket-grant-probe",
            version: "1.0.0",
            componentSha256: "33".repeat(32),
            componentBlake3: "44".repeat(32),
            descriptorByteSha256: "55".repeat(32),
            executionProtocol: { appChannelVersion: DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1 },
            componentReceipt: { schema: "semio.plugin.component-receipt/v1", pluginId: "socket-grant-probe", packageId: "semio:socket-grant-probe", profile: "release", target: "wasm32-wasip2", wasmSha256: "33".repeat(32), wasmBytes: 4096, semanticDescriptorSha256: "55".repeat(32), toolchain: "rustc 1.99.0-nightly (9f36de775bc636c8e88c31a173c2bcb6995956a0)", sourceTreeDigest: "3".repeat(64), provenanceSha256: "4".repeat(64) },
          },
          artifact: { kind: "native.socket-grant.probe", schema: "native.socket-grant.probe/v1", packSchemaHash: probePackSchemaHash },
          parentDialect: { artifactKind: "native.socket-grant.probe", standard: "1", subset: "*" },
          surface: {
            surfaceId: "native.socket-grant.probe.editor",
            appId: "native.socket-grant.probe.app",
            windowKindId: "native.socket-grant.probe.window",
            role: "editor",
            rendererTarget: "wgpu",
          },
          browserActor: { kind: "none" },
          grant: { read: true, write: true, observe: true },
          revalidation: { directoryRevision: planCount, membershipGeneration: 1, sessionGeneration: 1 },
        });
      }
      if (request.method === "POST" && url.pathname === "/spaces/probe-space/documents/probe-document/socket-grants") {
        if (request.headers.get("authorization") !== `Bearer ${capability}`) return new Response("", { status: 401 });
        const exchange = (await request.json().catch(() => undefined)) as Record<string, any> | undefined;
        if (
          exchange?.schema !== "semio.hub.document-plan-socket-grant-intent/v1" ||
          exchange?.version !== 1 ||
          typeof exchange?.planReceipt !== "string" ||
          planReceipts.get(exchange.planReceipt) !== false ||
          Object.keys(exchange).some((key) => !["schema", "version", "planReceipt"].includes(key))
        )
          return new Response("", { status: 401 });
        planReceipts.set(exchange.planReceipt, true);
        exchangeCount += 1;
        grantCount += 1;
        pendingGrants += 1;
        return Response.json({ schema: "semio.hub.document-socket-grant/v1", protocol: "semio.session.v1", actorId: actor, expiresAtMs: Date.now() + 10_000 });
      }
      if (request.method === "GET" && url.pathname === "/scopes/probe-space%2Fprobe-document/document/ws") {
        const protocols = (request.headers.get("sec-websocket-protocol") ?? "").split(",").map((value) => value.trim());
        if (protocols.length !== 2 || protocols[0] !== "semio.session.v1" || protocols[1] !== capability || pendingGrants === 0) return new Response("", { status: 401 });
        pendingGrants -= 1;
        socketCount += 1;
        return control.upgrade(request, { data: { connection: socketCount }, headers: { "Sec-WebSocket-Protocol": "semio.session.v1" } }) ? undefined : new Response("", { status: 500 });
      }
      return new Response("", { status: 404 });
    },
    websocket: {
      message(socket, message): void {
        const bytes = typeof message === "string" ? new TextEncoder().encode(message) : new Uint8Array(message);
        const decoded = decodeClientFrame(bytes);
        if (typeof decoded.frame !== "string" && "SocketHelloV1" in decoded.frame) {
          helloCount += 1;
          socket.send(encodeServerFrame({ Welcome: { session_id: `probe-session-${socket.data.connection}`, resume_token: `probe-resume-${socket.data.connection}`, server_frontier: frontier(mutationCount), bootstrap: "None" } }, "command"));
          socket.send(encodeServerFrame({ Session: { actor: socket.data.connection === 1 ? wrongActor : actor, color: socket.data.connection } }, "command"));
          return;
        }
        if (typeof decoded.frame !== "string" && "Commands" in decoded.frame) {
          if (socket.data.connection === 1) {
            preSessionCommands += 1;
            socket.close(1008, "command before matching Session");
            return;
          }
          if (decoded.frame.Commands.envelopes.length !== 1 || decoded.frame.Commands.envelopes[0]?.actor !== actor) {
            socket.close(1008, "actor binding");
            return;
          }
          mutationCount += 1;
          socket.send(encodeServerFrame({ Ack: { batch_id: decoded.frame.Commands.batch_id, stages: ["Received", "Persisted", { Applied: { outcome: "Accepted" } }], frontier: frontier(mutationCount) } }, "command"));
          if (socket.data.connection === 2) setTimeout(() => socket.close(1012, "reconnect"), 25);
        }
      },
    },
  });
  const envelope = { clientClass: "native", sessionId: "probe-session", authorizationGeneration: 1, expiresAt: Date.now() + 15_000, capability };
  let output = "";
  let errorOutput = "";
  try {
    const child = await deliverNativeCredentialEnvelope(nativeWgpuExecutable(repoRoot), ["--socket-grant-probe"], envelope, `http://127.0.0.1:${server.port}`);
    child.stdout?.on("data", (chunk: Buffer) => {
      if (output.length < 512) output += chunk.toString("utf8");
    });
    child.stderr?.on("data", (chunk: Buffer) => {
      if (errorOutput.length < 512) errorOutput += chunk.toString("utf8");
    });
    await waitForChildExit(child, 15_000);
    if (
      child.exitCode !== 0 ||
      output !== "native-socket-grant-probe-ok\n" ||
      planCount !== 3 ||
      exchangeCount !== 3 ||
      grantCount !== 3 ||
      socketCount !== 3 ||
      helloCount !== 3 ||
      mutationCount !== 2 ||
      preSessionCommands !== 0 ||
      [...planReceipts.values()].some((used) => !used) ||
      pendingGrants !== 0
    ) {
      throw new Error(
        `native socket actor law failed: exit=${child.exitCode} plans=${planCount} exchanges=${exchangeCount} grants=${grantCount} sockets=${socketCount} hellos=${helloCount} mutations=${mutationCount} preSessionCommands=${preSessionCommands} stderr=${errorOutput}`,
      );
    }
    if (output.includes(capability) || errorOutput.includes(capability) || [...planReceipts.keys()].some((secret) => output.includes(secret) || errorOutput.includes(secret)))
      throw new Error("native socket actor law leaked protected admission material");
  } finally {
    envelope.capability = "";
    server.stop(true);
  }
}

async function proveCredentialEnvelopeDelivery(envelope: Record<string, any>, clientClass: "native" | "mcp", hubOrigin: string): Promise<void> {
  const consumer = String.raw`
const fs = require("node:fs");
const expectedClass = process.argv.at(-1);
const pipe = fs.createReadStream(null, { fd: 3, autoClose: true });
const chunks = [];
let retained = 0;
const envelope = new Promise((resolve, reject) => {
  pipe.on("data", chunk => { retained += chunk.length; if (retained > 16384) reject(new Error("oversize")); else chunks.push(chunk); });
  pipe.on("error", reject);
  pipe.on("end", () => {
    const framed = Buffer.concat(chunks);
    if (framed.length < 5 || framed.readUInt32BE(0) !== framed.length - 4) return reject(new Error("framing"));
    const value = JSON.parse(framed.subarray(4).toString("utf8"));
    if (value.schema !== "semio.local.consumer-credential/v1" || value.clientClass !== expectedClass || !/^http:\/\/127\.0\.0\.1:\d+$/.test(value.hubOrigin) || !Number.isSafeInteger(value.authorizationGeneration) || value.authorizationGeneration < 1 || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/.test(value.capability)) return reject(new Error("binding"));
    framed.fill(0);
    resolve(undefined);
  });
});
const protocol = expectedClass === "mcp" ? new Promise((resolve, reject) => {
  let text = "";
  process.stdin.setEncoding("utf8");
  process.stdin.on("data", chunk => { text += chunk; if (text.length > 256) reject(new Error("stdio oversize")); });
  process.stdin.on("end", () => text === "{\"jsonrpc\":\"2.0\",\"id\":1}\n" ? resolve(undefined) : reject(new Error("stdio changed")));
}) : Promise.resolve();
Promise.all([envelope, protocol]).then(() => process.stdout.write("credential-delivery-ok")).catch(() => process.exit(2));`;
  const delivery = clientClass === "native" ? deliverNativeCredentialEnvelope : deliverMcpCredentialEnvelope;
  const child = await delivery(process.execPath, ["-e", consumer, clientClass], structuredClone(envelope), hubOrigin);
  let output = "";
  child.stdout?.on("data", (chunk: Buffer) => {
    if (output.length < 256) output += chunk.toString("utf8");
  });
  if (clientClass === "mcp") child.stdin?.end('{"jsonrpc":"2.0","id":1}\n');
  await waitForChildExit(child, 5_000);
  if (child.exitCode !== 0 || output !== "credential-delivery-ok") throw new Error(`${clientClass} credential delivery failed`);
}

type RejectionCase = "wrong-hmac" | "wrong-class" | "wrong-profile" | "expired" | "cross-run" | "timeout" | "eof";

async function proveRejectedBootstrap(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl,rejection:RejectionCase): Promise<void> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const profiles: readonly LocalProfile[] = [{ profileId: "developer", subject: "negative-local-developer", displayName: "Negative Developer", allowedClientClasses: ["native", "mcp"] }];
  const run = await startLocalHub(host, profiles, { capture: true, isolatedSecuritySmoke: true });
  const keyEvidence = run.channelKey.toString("hex");
  try {
    await waitForReadiness(run, true);
    if (rejection === "eof") {
      run.pipe.end();
    } else if (rejection !== "timeout") {
      const now = Date.now();
      const unsigned: Record<string, unknown> = {
        schema: LOCAL_BOOTSTRAP_SCHEMA,
        kind: "issue",
        runId: rejection === "cross-run" ? randomBytes(16).toString("hex") : run.runId,
        sequence: 2,
        exchangeId: randomBytes(16).toString("hex"),
        issuedAt: rejection === "expired" ? now - 2_000 : now,
        expiresAt: rejection === "expired" ? now - 1_000 : now + LOCAL_BOOTSTRAP_DEADLINE_MS,
        profileId: rejection === "wrong-profile" ? "unknown" : "developer",
        deviceInstanceId: "negative-launcher",
        clientClass: rejection === "wrong-class" ? "admin-relay" : "native",
      };
      const frame = authenticatedFrame(run.channelKey, unsigned);
      if (rejection === "wrong-hmac") frame.proof = `${frame.proof === "0".repeat(64) ? "1" : "0"}${String(frame.proof).slice(1)}`;
      await writeLocalFrame(run.pipe, frame);
    }
    await waitForLocalHubExitV1(run).catch((error: unknown) => {
      throw new Error(`${rejection} rejection did not stop the hub: ${error instanceof Error ? error.message : "unknown"}`);
    });
    if (run.child.exitCode === 0) throw new Error(`${rejection} unexpectedly kept the hub usable`);
    const output = run.output();
    if (output.includes(keyEvidence) || output.includes(profiles[0]!.subject)) throw new Error(`${rejection} leaked private bootstrap material`);
  } finally {
    await finishLocalHub(run);
  }
  });
}

async function runSecureLocalSmoke(invocation:ScriptInvocation,phaseControl:CompilerAdmissionControl): Promise<void> {
  return withPublishedLocalHubHostV1(invocation,phaseControl,async(host,context)=>{
  const {workspace:repoRoot,root,dataRoot}=context;
  const profiles: readonly LocalProfile[] = [
    { profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] },
    { profileId: "administrator", subject: "local-administrator-01", displayName: "Local Administrator", allowedClientClasses: ["native"] },
  ];
  const run = await startLocalHub(host, profiles, { capture: true, adminSubjects: ["semio.local.bootstrap/v1:local-administrator-01"], isolatedSecuritySmoke: true });
  const capabilities: string[] = [];
  const keyEvidence = run.channelKey.toString("hex");
  try {
    const nativeEnvelope = await issueLocalCredential(run, "developer", "native", 2);
    const mcpEnvelope = await issueLocalCredential(run, "developer", "mcp", 3);
    const adminEnvelope = await issueLocalCredential(run, "administrator", "native", 4);
    capabilities.push(nativeEnvelope.capability, mcpEnvelope.capability, adminEnvelope.capability);
    let crossClassRejected = false;
    try {
      await deliverMcpCredentialEnvelope(process.execPath, [], structuredClone(nativeEnvelope), `http://127.0.0.1:${run.port}`);
    } catch {
      crossClassRejected = true;
    }
    if (!crossClassRejected) throw new Error("native envelope crossed into MCP delivery");
    await proveNativeCredentialEnvelopeDelivery(repoRoot, nativeEnvelope, `http://127.0.0.1:${run.port}`);
    await proveCredentialEnvelopeDelivery(mcpEnvelope, "mcp", `http://127.0.0.1:${run.port}`);
    await proveNonFd3CredentialMarkerRejection(mcpExecutable(repoRoot), "mcp");
    const readiness = await waitForReadiness(run, true);
    if (readiness.status !== "not-ready" || readiness.authentication.bootstrapReady !== true || readiness.artifactAuthority?.ready !== false) throw new Error("security smoke did not observe truthful partial readiness");
    const readinessJson = JSON.stringify(readiness);
    if (capabilities.some((capability) => readinessJson.includes(capability)) || profiles.some((profile) => readinessJson.includes(profile.subject)) || readinessJson.includes("sessionKind") || readinessJson.includes("authorizationGeneration"))
      throw new Error("readiness leaked private bootstrap material");
    await proveMcpWorkspaceProcess(repoRoot, run, mcpEnvelope, adminEnvelope);
    for (let index = 0; index < capabilities.length; index++) {
      const me = await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers: { authorization: `Bearer ${capabilities[index]}` } });
      if (!me.ok) throw new Error("issued local session did not validate");
      const session = (await me.json()) as Record<string, unknown>;
      const envelope = [nativeEnvelope, mcpEnvelope, adminEnvelope][index]!;
      if (session.sessionKind !== "development-local" || session.authorizationGeneration !== envelope.authorizationGeneration) throw new Error("issued local session metadata did not match its signed envelope");
    }
    const developerAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[0]}` } });
    const administratorAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[2]}` } });
    if (developerAdmin.status !== 401 || !administratorAdmin.ok) throw new Error("local administrator subject policy was not enforced");
    if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions`, { method: "POST" })).status !== 404) throw new Error("public session mint route exists");
    for (const capability of capabilities) {
      const headers = { authorization: `Bearer ${capability}` };
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me/sign-out`, { method: "POST", headers })).status !== 204) throw new Error("local session revoke failed");
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers })).status !== 401) throw new Error("revoked local session remained usable");
    }
    const replayNow = Date.now();
    await writeLocalFrame(
      run.pipe,
      authenticatedFrame(run.channelKey, {
        schema: LOCAL_BOOTSTRAP_SCHEMA,
        kind: "issue",
        runId: run.runId,
        sequence: 5,
        exchangeId: nativeEnvelope.exchangeId,
        issuedAt: replayNow,
        expiresAt: replayNow + LOCAL_BOOTSTRAP_DEADLINE_MS,
        profileId: "developer",
        deviceInstanceId: "native-launcher",
        clientClass: "native",
      }),
    );
    await waitForLocalHubExitV1(run);
    const output = run.output();
    if (capabilities.some((capability) => output.includes(capability)) || output.includes(keyEvidence) || profiles.some((profile) => output.includes(profile.subject))) throw new Error("hub logs leaked private bootstrap material");
  } finally {
    capabilities.fill("");
    await finishLocalHub(run);
  }
  for (const rejection of ["wrong-hmac", "wrong-class", "wrong-profile", "expired", "cross-run", "timeout", "eof"] as const) {
    await proveRejectedBootstrap(invocation,context.control,rejection).catch((error: unknown) => {
      throw new Error(`${rejection} rejection failed: ${error instanceof Error ? error.message : "unknown"}`);
    });
  }
  console.log("secure-local-smoke: truthful partial readiness, native/MCP delivery, admin isolation, issue/validate/revoke, replay/HMAC/class/profile/run/expiry/timeout/EOF rejection, redaction, and absent public mint passed");
  });
}

export class NativeDocumentOpenCheckScript extends BundleScript {
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
    proveNativeCredentialSourceOrder(this.repoRoot);
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-wgpu:check-frame-worker", "--skip-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--skip-cache", "--", "--scale"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runNativeDocumentAdmissionLaws(this.repoRoot);
    await proveNativeSocketGrantActor(this.repoRoot);
    console.log(
      "native-document-open-check: independent D1 oracle, exact scope/package/surface laws, one-use plan receipt exchange, tag7 Session gate, actor stamping, reconnect reissue, cross-space isolation, cancellation, origin denial, and redaction passed",
    );
  }
}

function runNativeDocumentAdmissionLaws(repoRoot: string): void {
  const target = ["test", "-p", "semio-framework-os-kernel", "--lib", "--features", "sync,ureq"];
  const suffixes = [
    "native_document_admission_issues_validates_and_exchanges_exactly_once",
    "hostile_or_cancelled_plan_never_reaches_receipt_exchange",
    "cancellation_after_receipt_exchange_never_reaches_a_document_socket",
    "mismatched_local_plugin_selection_never_exchanges_a_plan_receipt",
    "hostile_hub_binding_cannot_receive_a_credential_bound_document_grant",
    "native_terminal_connection_failure_clears_receipt_actor_before_reissue",
    "hub_document_actor_and_surface_authority_are_isolated_by_full_scope",
  ];
  const env = { ...process.env, RUST_MIN_STACK: "268435456" };
  for (const suffix of suffixes) {
    const listed = runProbe("cargo", [...target, suffix, "--", "--list"], { cwd: repoRoot, env, ...orchestratorBudgetOpts() });
    const matches = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length))
      .filter((name) => name.endsWith(suffix));
    if (listed.status !== 0 || matches.length !== 1) throw new Error(`native document admission gate expected exactly one ${suffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
    runCargo([...target, matches[0]!, "--", "--exact", "--test-threads=1"], repoRoot, env);
  }
  const mcpSuffixes = ["mcp_probe_document_transport_binds_full_scope_and_exact_surface_authority", "authenticated_hub_checkpoint_resource_projects_exact_verified_pair_and_never_crosses_scope"];
  const mcpTarget = ["test", "-p", "semio-framework-os-mcp", "--lib"];
  for (const mcpSuffix of mcpSuffixes) {
    const listed = runProbe("cargo", [...mcpTarget, mcpSuffix, "--", "--list"], { cwd: repoRoot, env, ...orchestratorBudgetOpts() });
    const matches = listed.stdout
      .split("\n")
      .filter((line) => line.endsWith(": test"))
      .map((line) => line.slice(0, -": test".length))
      .filter((name) => name.endsWith(mcpSuffix));
    if (listed.status !== 0 || matches.length !== 1) throw new Error(`MCP document admission gate expected exactly one ${mcpSuffix}, selected ${matches.length}; status=${listed.status}; diagnostic=${listed.stderr.trim().slice(-4_000) || "<none>"}`);
    runCargo([...mcpTarget, matches[0]!, "--", "--exact", "--test-threads=1"], repoRoot, env);
  }
  console.log(`native-document-admission-laws: nativeExact=${suffixes.length} mcpExact=${mcpSuffixes.length} passed`);
}

export class SecureLocalSmokeScript extends BundleScript {
  async run(): Promise<void> {
    await proveDocumentOpenPlanFixture(this.repoRoot);
    proveNativeCredentialSourceOrder(this.repoRoot);
    proveMcpCredentialSourceOrder(this.repoRoot);
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-wgpu:check-frame-worker", "--skip-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/framework-renderer-wgpu:native-build", "--skip-cache", "--", "--scale"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runCmd("bun", ["repo", "run", "@semio-tech/framework-os-mcp-rs:build", "--skip-cache"], { cwd: this.repoRoot, ...orchestratorBudgetOpts() });
    runNativeDocumentAdmissionLaws(this.repoRoot);
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);
    await runSecureLocalSmoke(this.invocation,exactCargoCallerControlV1(this.invocation,"hub:secure-smoke",0));
    await proveNativeSocketGrantActor(this.repoRoot);
    console.log(
      "native-mcp-socket-grant-check: neutral D1 oracle, exact native laws, direct children, early fd3 seals, byte-clean MCP stdio, strict open-plan receipt exchange, v1 grant/tag7, exact package/surface authority, actor stamping, forced reconnect, and fresh plan/grant reissue passed",
    );
  }
}
