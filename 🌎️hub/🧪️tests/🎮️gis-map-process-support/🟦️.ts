/** 🧭️ GIS map process support: document sockets, MCP clients and checkpoint control shared by the GIS map gates.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { terminateOwnedChildTree } from "../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { createHash, randomBytes } from "node:crypto";
import { type ChildProcess } from "node:child_process";
import { decodeServerFrame, encodeClientFrame } from "@semio-tech/framework-replication";
import { parseDocumentSocketGrantReceiptV1 } from "@semio-tech/framework-os";
import { DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1, type ArtifactFrontier } from "@semio-tech/framework/directory/schema";
import { parseDocumentOpenPlanV1, parseDocumentPlanSocketGrantIntentV1 } from "@semio-tech/framework-os/modules/directory/schema/document-open";
import { artifactFrontierIsGenesisForV1, artifactFrontierIsEditedForV1 } from "@semio-tech/framework/directory/schema";
import { DIRECT_CHILD_BENIGN_ENV_KEY, DIRECT_CHILD_BENIGN_ENV_VALUE } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { writeLocalFrame } from "../../🚀️local-bootstrap/📡️framing/🟦️.ts";
import { type LocalHubRun, waitForChildExit } from "../../🚀️local-bootstrap/🏃️execution/🟦️.ts";
import { GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES } from "../../💡️inference/🧬️schema/🟦️.ts";
import { type CheckInProcessFixtureV1, startMcpBoundWorkspaceChild, waitForDocumentSocketFrame, waitForJsonRpcResponse } from "../../🚀️local-bootstrap/🧰️mcp-process-support/🟦️.ts";

export type GisMapProcessDocumentSocketV1 = {
  readonly socket: WebSocket;
  readonly frames: Record<string, any>[];
  readonly plan: ReturnType<typeof parseDocumentOpenPlanV1>;
  error: unknown;
};

export type GisMapDocumentOpenFixtureV1 = Pick<CheckInProcessFixtureV1, "generationId" | "documentId" | "package" | "artifact" | "surfaceId">;

export type GisMapProcessMcpClientV1 = {
  readonly child: ChildProcess;
  readonly stdout: Buffer[];
  readonly stderr: Buffer[];
  capability: string;
  nextId: number;
  stdoutBytes: number;
  stderrBytes: number;
  overflow: boolean;
};

export type GisMapCheckpointProjectionV1 = Readonly<{
  activeCheckpointId: string;
  descriptorDigestV1: string;
  frontier: ArtifactFrontier;
  packSha256: string;
  packByteLength: number;
  sprSha256: string;
  sprByteLength: number;
  pairSha256: string;
}>;

/** 🔌️ Opens one normal document socket and retains its exact catalog/actor authority for checkpoint observation. */
export async function openGisMapProcessDocumentSocket(
  run: LocalHubRun,
  capability: string,
  spaceId: string,
  fixture: GisMapDocumentOpenFixtureV1,
  clientInstanceId: string,
): Promise<GisMapProcessDocumentSocketV1> {
  const root = `/spaces/${encodeURIComponent(spaceId)}/documents/${encodeURIComponent(fixture.documentId)}`;
  const planResponse = await fetch(`http://127.0.0.1:${run.port}${root}/open-plan`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify({ schema: "semio.hub.document-open-intent/v1", version: 1, scope: { spaceId, documentId: fixture.documentId }, requestedSurfaceId: fixture.surfaceId, clientInstanceId }),
    signal: AbortSignal.timeout(5_000),
  });
  const plan = parseDocumentOpenPlanV1(await planResponse.json().catch(() => undefined), Date.now());
  if (
    !planResponse.ok ||
    plan.scope.spaceId !== spaceId ||
    plan.scope.documentId !== fixture.documentId ||
    plan.catalog.generationId !== fixture.generationId ||
    plan.package.packageId !== fixture.package.packageId ||
    plan.package.componentSha256 !== fixture.package.componentSha256 ||
    plan.package.executionProtocol.appChannelVersion !== DOCUMENT_EXECUTION_PROTOCOL_APP_CHANNEL_VERSION_V1
  ) {
    throw new Error("GIS Map process socket received a substituted execution target");
  }
  const grantResponse = await fetch(`http://127.0.0.1:${run.port}${root}/socket-grants`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: JSON.stringify(parseDocumentPlanSocketGrantIntentV1({ schema: "semio.hub.document-plan-socket-grant-intent/v1", version: 1, planReceipt: plan.receipt })),
    signal: AbortSignal.timeout(5_000),
  });
  const grant = parseDocumentSocketGrantReceiptV1(await grantResponse.json().catch(() => undefined));
  if (!grantResponse.ok) throw new Error(`GIS Map process socket grant failed: ${grantResponse.status}`);
  const scope = `${spaceId}/${fixture.documentId}`;
  const socket = new WebSocket(`ws://127.0.0.1:${run.port}/scopes/${encodeURIComponent(scope)}/document/ws?surface=${encodeURIComponent(fixture.surfaceId)}`, ["semio.session.v1", capability]);
  socket.binaryType = "arraybuffer";
  plan.receipt = "";
  const live: GisMapProcessDocumentSocketV1 = { socket, frames: [], plan, error: undefined };
  socket.onmessage = (event) => {
    try {
      if (live.frames.length >= 64) throw new Error("GIS Map process socket exceeded its frame ceiling");
      live.frames.push(decodeServerFrame(new Uint8Array(event.data as ArrayBuffer)).frame as unknown as Record<string, any>);
    } catch (error) {
      live.error = error;
    }
  };
  socket.onerror = (error) => {
    live.error = error;
  };
  try {
    await new Promise<void>((resolveOpen, rejectOpen) => {
      const finish = (error?: Error): void => {
        clearTimeout(timer); socket.removeEventListener("open", opened); socket.removeEventListener("error", failed); socket.removeEventListener("close", failed);
        if (error) rejectOpen(error); else resolveOpen();
      };
      const opened = (): void => finish();
      const failed = (): void => finish(new Error("GIS Map process socket closed or failed before opening"));
      const timer = setTimeout(() => finish(new Error("GIS Map process socket open deadline exceeded")), 5_000);
      socket.addEventListener("open", opened, { once: true }); socket.addEventListener("error", failed, { once: true }); socket.addEventListener("close", failed, { once: true });
      if (socket.readyState === WebSocket.OPEN) finish();
      else if (socket.readyState >= WebSocket.CLOSING) failed();
    });
    if (socket.protocol !== "semio.session.v1") throw new Error("GIS Map process socket did not negotiate its exact protocol");
    socket.send(
      encodeClientFrame(
        { SocketHelloV1: { wire_version: 1, protocol_version: 1, schema: fixture.artifact.schema, pack_schema_hash: Array.from(Buffer.from(fixture.artifact.packSchemaHash, "hex")), resume_token: null, frontier: null } },
        "command",
      ),
    );
    await waitForDocumentSocketFrame(socket, live.frames, (frame) => ("Welcome" in frame ? frame.Welcome : undefined), `${clientInstanceId} Welcome`);
    await waitForDocumentSocketFrame(socket, live.frames, (frame) => ("Session" in frame && frame.Session.actor === grant.actorId ? frame.Session : undefined), `${clientInstanceId} Session`);
    if (live.error) throw live.error;
    return live;
  } catch (error) {
    try { await finishGisMapProcessDocumentSocket(live); }
    catch (cleanup) { throw new AggregateError([error, cleanup], "GIS Map witness open and retirement failed"); }
    throw error;
  }
}

/** 🔌️ Retires a witness socket before any subsequent MCP or Hub lifetime can start. */
export async function finishGisMapProcessDocumentSocket(live: GisMapProcessDocumentSocketV1 | undefined, budgetMs = 5_000): Promise<void> {
  if (!live) return;
  if (!Number.isSafeInteger(budgetMs) || budgetMs < 1 || budgetMs > 5_000) throw new Error("GIS Map witness retirement budget is invalid");
  live.socket.onmessage = null;
  live.socket.onopen = null;
  live.socket.onerror = null;
  try {
    if (live.socket.readyState === WebSocket.CLOSED) return;
    await new Promise<void>((resolveClose, rejectClose) => {
      const finish = (error?: Error): void => {
        clearTimeout(timer);
        live.socket.removeEventListener("close", closed);
        live.socket.removeEventListener("error", failed);
        if (error) rejectClose(error); else resolveClose();
      };
      const closed = (): void => finish();
      const failed = (): void => finish(new Error("GIS Map witness socket failed during retirement"));
      const timer = setTimeout(() => finish(new Error("GIS Map witness socket retirement deadline exceeded")), budgetMs);
      live.socket.addEventListener("close", closed, { once: true });
      live.socket.addEventListener("error", failed, { once: true });
      try {
        if (live.socket.readyState === WebSocket.CLOSED) finish();
        else if (live.socket.readyState < WebSocket.CLOSING) live.socket.close(1000, "GIS Map witness retired");
      } catch (error) { finish(error instanceof Error ? error : new Error("GIS Map witness close failed")); }
    });
  } finally {
    live.plan.receipt = "";
    live.frames.length = 0;
  }
}

/** 🧰️ Starts one credential-FD MCP child and opens the selected document through its actual ArtifactChannel. */
export async function startGisMapProcessMcpClient(repoRoot: string, run: LocalHubRun, envelope: Record<string, any>, spaceId: string, documentId: string, name: string, binaryPath?: string): Promise<GisMapProcessMcpClientV1> {
  const child = await startMcpBoundWorkspaceChild(repoRoot, run, envelope, spaceId, { ...process.env, ...(binaryPath ? { SEMIO_OS_MCP_BIN: binaryPath } : {}), [DIRECT_CHILD_BENIGN_ENV_KEY]: DIRECT_CHILD_BENIGN_ENV_VALUE });
  const client: GisMapProcessMcpClientV1 = { child, stdout: [], stderr: [], capability: String(envelope.capability), nextId: 1, stdoutBytes: 0, stderrBytes: 0, overflow: false };
  const retain = (target: Buffer[], kind: "stdout" | "stderr", maximum: number, chunk: Buffer): void => {
    const current = kind === "stdout" ? client.stdoutBytes : client.stderrBytes;
    const next = current + chunk.byteLength;
    if (kind === "stdout") client.stdoutBytes = next;
    else client.stderrBytes = next;
    const remaining = maximum - current;
    if (remaining > 0) target.push(Buffer.from(chunk.subarray(0, remaining)));
    if (next > maximum) client.overflow = true;
  };
  child.stdout?.on("data", (chunk: Buffer) => retain(client.stdout, "stdout", 16 * 1024 * 1024, chunk));
  child.stderr?.on("data", (chunk: Buffer) => retain(client.stderr, "stderr", 64 * 1024, chunk));
  try {
    const deadline = Date.now() + 10_000;
    while (!Buffer.concat(client.stderr).toString("utf8").includes("real per-capability ArtifactChannel routing bound")) {
      if (child.exitCode !== null) throw new Error(`${name} MCP child exited before authenticated workspace binding: ${child.exitCode}`);
      if (Date.now() >= deadline) throw new Error(`${name} MCP child workspace binding deadline exceeded`);
      await Bun.sleep(20);
    }
    const call = async (method: string, params: Record<string, unknown>): Promise<Record<string, any>> => {
      const id = client.nextId++;
      child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
      return waitForJsonRpcResponse(child, client.stdout, id);
    };
    const initialized = await call("initialize", { protocolVersion: "2025-11-25", capabilities: {}, clientInfo: { name, version: "1" } });
    if (initialized.result?.protocolVersion !== "2025-11-25") throw new Error(`${name} MCP initialization failed`);
    child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", method: "notifications/initialized" })}\n`);
    const opened = await call("tools/call", { name: "artifact_open", arguments: { artifactId: documentId } });
    if (opened.result?.isError !== false || opened.result?.structuredContent?.artifactId !== documentId) throw new Error(`${name} MCP child did not open the selected GIS Map document`);
    return client;
  } catch (error) {
    child.stdin?.end();
    terminateOwnedChildTree(child);
    await waitForChildExit(child, 5_000).catch(() => {});
    client.capability = "";
    client.stdout.forEach((bytes) => bytes.fill(0));
    client.stderr.forEach((bytes) => bytes.fill(0));
    throw error;
  }
}

async function callGisMapProcessMcp(client: GisMapProcessMcpClientV1, method: string, params: Record<string, unknown>): Promise<Record<string, any>> {
  const id = client.nextId++;
  client.child.stdin?.write(`${JSON.stringify({ jsonrpc: "2.0", id, method, params })}\n`);
  return waitForJsonRpcResponse(client.child, client.stdout, id);
}

const GIS_INFERENCE_CHECKPOINT_CONTROL_SCHEMA = "semio.hub.gis-inference-checkpoint-control/v1";

type GisInferenceCheckpointControlFrameV1 = Readonly<{
  schema: typeof GIS_INFERENCE_CHECKPOINT_CONTROL_SCHEMA;
  version: 1;
  sequence: 1 | 2;
  kind: "progress-persisted" | "release";
  jobId: string;
  progressCursor: number;
  completed: number;
  total: number;
}>;

export function assertGisInferenceCheckpointControlFrame(frame: Record<string, any>, sequence: 1 | 2, kind: "progress-persisted" | "release", jobId: string): void {
  const canonical = JSON.stringify(frame);
  if (
    Object.keys(frame).join(",") !== "schema,version,sequence,kind,jobId,progressCursor,completed,total" ||
    frame.schema !== GIS_INFERENCE_CHECKPOINT_CONTROL_SCHEMA ||
    frame.version !== 1 ||
    frame.sequence !== sequence ||
    frame.kind !== kind ||
    frame.jobId !== jobId ||
    !/^[0-9a-f]{32}$/u.test(frame.jobId) ||
    !Number.isSafeInteger(frame.progressCursor) || frame.progressCursor < 1 || frame.progressCursor > 16 ||
    !Number.isSafeInteger(frame.completed) || frame.completed < 1 ||
    !Number.isSafeInteger(frame.total) || frame.total < 1 || frame.completed > frame.total ||
    Buffer.byteLength(canonical) === 0 ||
    Buffer.byteLength(canonical) > GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES
  ) {
    throw new Error(`GIS inference checkpoint control rejected ${kind} frame`);
  }
}

async function waitForGisInferenceCheckpointControl(run: LocalHubRun, jobId: string): Promise<Readonly<{ progressCursor: number; completed: number; total: number }>> {
  if (!run.inferenceCheckpointReader || run.inferenceCheckpointEnteredJobId || run.inferenceCheckpointReleased !== false) {
    throw new Error("GIS inference checkpoint control has no fresh process-owned reader");
  }
  const frame = await run.inferenceCheckpointReader.read(120_000);
  assertGisInferenceCheckpointControlFrame(frame, 1, "progress-persisted", jobId);
  run.inferenceCheckpointEnteredJobId = jobId;
  const progress = Object.freeze({ progressCursor: frame.progressCursor, completed: frame.completed, total: frame.total });
  run.inferenceCheckpointProgress = progress;
  return progress;
}

async function releaseGisInferenceCheckpointControl(run: LocalHubRun, jobId: string): Promise<void> {
  if (!run.inferenceCheckpointPipe || run.inferenceCheckpointEnteredJobId !== jobId || !run.inferenceCheckpointProgress || run.inferenceCheckpointReleased !== false) {
    throw new Error("GIS inference checkpoint release did not own the exact entered job");
  }
  const frame: GisInferenceCheckpointControlFrameV1 = { schema: GIS_INFERENCE_CHECKPOINT_CONTROL_SCHEMA, version: 1, sequence: 2, kind: "release", jobId, ...run.inferenceCheckpointProgress };
  assertGisInferenceCheckpointControlFrame(frame, 2, "release", jobId);
  await writeLocalFrame(run.inferenceCheckpointPipe, frame);
  run.inferenceCheckpointReleased = true;
  run.inferenceCheckpointPipe.end();
}

/** 📥️ Reads, bounds, hashes, and immediately wipes one MCP-projected canonical pair. */
/** 🧯️ The hub authority's own canonical pair ceiling (`AUTHORITY_MAX_PAIR_BYTES`). */
const GIS_MAP_PROCESS_PAIR_MAX_BYTES = 64 * 1024 * 1024;

export async function readGisMapProcessCheckpoint(client: GisMapProcessMcpClientV1, spaceId: string, documentId: string): Promise<GisMapCheckpointProjectionV1> {
  const uri = `semio://workspace/scopes/${encodeURIComponent(spaceId)}/${encodeURIComponent(documentId)}/checkpoint`;
  const response = await callGisMapProcessMcp(client, "resources/read", { uri });
  const text = response.result?.contents?.[0]?.text;
  const resource = typeof text === "string" ? (JSON.parse(text) as Record<string, any>) : undefined;
  const pack = typeof resource?.untrusted?.content?.packBase64 === "string" ? Buffer.from(resource.untrusted.content.packBase64, "base64") : Buffer.alloc(0);
  const spr = typeof resource?.untrusted?.content?.sprBase64 === "string" ? Buffer.from(resource.untrusted.content.sprBase64, "base64") : Buffer.alloc(0);
  try {
    const scope = { spaceId, documentId };
    const frontier = resource?.frontier as ArtifactFrontier | undefined;
    const frontierKeys = frontier && typeof frontier === "object" && !Array.isArray(frontier) ? Object.keys(frontier).sort() : [];
    if (
      response.error ||
      resource?.schema !== "semio.mcp.canonical-checkpoint-resource/v1" ||
      resource?.scope?.spaceId !== spaceId ||
      resource?.scope?.documentId !== documentId ||
      !/^[0-9a-f]{64}$/u.test(resource?.activeCheckpointId ?? "") ||
      !/^[0-9a-f]{64}$/u.test(resource?.descriptorDigestV1 ?? "") ||
      pack.byteLength === 0 ||
      spr.byteLength === 0 ||
      pack.byteLength !== resource?.pack?.byteLength ||
      spr.byteLength !== resource?.spr?.byteLength ||
      pack.byteLength + spr.byteLength > GIS_MAP_PROCESS_PAIR_MAX_BYTES ||
      createHash("sha256").update(pack).digest("hex") !== resource?.pack?.sha256 ||
      createHash("sha256").update(spr).digest("hex") !== resource?.spr?.sha256 ||
      JSON.stringify(frontierKeys) !== JSON.stringify(["chainHash", "documentId", "headEditId", "headEditOrdinal", "lastCommitSeq"]) ||
      (!artifactFrontierIsGenesisForV1(scope, frontier!) && !artifactFrontierIsEditedForV1(scope, frontier!))
    ) {
      throw new Error("GIS Map MCP child returned an invalid or unbounded canonical checkpoint pair");
    }
    return Object.freeze({
      activeCheckpointId: resource.activeCheckpointId,
      descriptorDigestV1: resource.descriptorDigestV1,
      frontier: structuredClone(frontier!),
      packSha256: resource.pack.sha256,
      packByteLength: pack.byteLength,
      sprSha256: resource.spr.sha256,
      sprByteLength: spr.byteLength,
      pairSha256: createHash("sha256").update(pack).update(spr).digest("hex"),
    });
  } finally {
    pack.fill(0);
    spr.fill(0);
  }
}

export async function finishGisMapProcessMcpClient(client: GisMapProcessMcpClientV1 | undefined): Promise<void> {
  if (!client) return;
  client.child.stdin?.end();
  let timedOut = false;
  await waitForChildExit(client.child, 5_000).catch(() => {
    timedOut = true;
    terminateOwnedChildTree(client.child);
  });
  if (timedOut) await waitForChildExit(client.child, 5_000).catch(() => {});
  const stdout = Buffer.concat(client.stdout);
  const stderr = Buffer.concat(client.stderr);
  try {
    if (timedOut || client.overflow || client.child.exitCode !== 0 || stdout.includes(Buffer.from(client.capability)) || stderr.includes(Buffer.from(client.capability))) {
      throw new Error(`GIS Map MCP child failed its byte-clean terminal fence: exit=${client.child.exitCode} stdout=${client.stdoutBytes} stderr=${client.stderrBytes}`);
    }
  } finally {
    client.capability = "";
    stdout.fill(0);
    stderr.fill(0);
    client.stdout.forEach((bytes) => bytes.fill(0));
    client.stderr.forEach((bytes) => bytes.fill(0));
  }
}

export async function gisMapProcessHttp(run: LocalHubRun, capability: string, method: "GET" | "POST", path: string, body?: Record<string, unknown>): Promise<Readonly<{ status: number; text: string; value: Record<string, any> | undefined }>> {
  const response = await fetch(`http://127.0.0.1:${run.port}${path}`, {
    method,
    headers: { authorization: `Bearer ${capability}`, ...(body ? { "content-type": "application/json" } : {}) },
    ...(body ? { body: JSON.stringify(body) } : {}),
    signal: AbortSignal.timeout(120_000),
  });
  const text = await response.text();
  let value: Record<string, any> | undefined;
  try {
    value = JSON.parse(text);
  } catch {}
  return Object.freeze({ status: response.status, text, value });
}

export async function waitForGisMapProcessInference(run: LocalHubRun, capability: string, path: string, expected: "succeeded" | "cancelled"): Promise<Readonly<{ status: number; text: string; value: Record<string, any> | undefined }>> {
  const deadline = Date.now() + 120_000;
  while (Date.now() < deadline) {
    const page = await gisMapProcessHttp(run, capability, "GET", path);
    if (page.status !== 200) throw new Error(`GIS Map inference event poll failed: ${page.status}`);
    if (page.value?.state === expected) return page;
    if (!["accepted", "running"].includes(page.value?.state)) throw new Error(`GIS Map inference reached unexpected state ${String(page.value?.state)}`);
    await Bun.sleep(5);
  }
  throw new Error(`GIS Map inference did not reach ${expected} before its retained deadline`);
}

/** 🛑️ Cancels one actual paused MCP-owned job and proves it terminates without an offer. */
export async function cancelGisMapProcessMcpOwner(run: LocalHubRun, mcpB: GisMapProcessMcpClientV1, documentId: string): Promise<string> {
  const submittedB = await callGisMapProcessMcp(mcpB, "tools/call", {
    name: "inference_submit",
    arguments: { documentId: documentId, requestId: randomBytes(16).toString("hex"), lifetimeMs: 120_000 },
  });
  const submittedBStructured = submittedB.result?.structuredContent;
  const jobHandleB = submittedBStructured?.jobHandle;
  const jobIdB = submittedBStructured?.receipt?.jobId;
  if (
    submittedB.result?.isError !== false ||
    typeof jobHandleB !== "string" ||
    jobHandleB.length === 0 ||
    !/^[0-9a-f]{32}$/u.test(jobIdB ?? "") ||
    submittedBStructured?.receipt?.state !== "running" ||
    submittedBStructured?.receipt?.proposalState !== "none" ||
    submittedBStructured?.receipt?.proposalHash !== null
  ) {
    throw new Error("GIS Map Author B did not receive one MCP-owned retained Running job");
  }
  const persistedProgress = await waitForGisInferenceCheckpointControl(run, jobIdB);
  const observedB = await callGisMapProcessMcp(mcpB, "tools/call", { name: "inference_events", arguments: { jobHandle: jobHandleB, after: 0 } });
  const observedBPage = observedB.result?.structuredContent?.page;
  const observedProgress = Array.isArray(observedBPage?.progress) ? observedBPage.progress.find((row: any) => row?.cursor === persistedProgress.progressCursor) : undefined;
  if (
    observedB.result?.isError !== false ||
    observedB.result?.structuredContent?.jobHandle !== jobHandleB ||
    observedBPage?.jobId !== jobIdB ||
    observedBPage?.state !== "running" ||
    observedBPage?.cancelRequested !== false ||
    !observedProgress ||
    observedProgress.completed !== persistedProgress.completed ||
    observedProgress.total !== persistedProgress.total ||
    !Number.isSafeInteger(observedProgress.runEpoch) || observedProgress.runEpoch < 1 ||
    observedBPage.nextCursor < persistedProgress.progressCursor ||
    observedBPage.progress.length > 16
  ) {
    throw new Error("GIS Map Author B did not observe the exact bounded persisted progress before cancellation");
  }
  const cancelledB = await callGisMapProcessMcp(mcpB, "tools/call", { name: "inference_cancel", arguments: { jobHandle: jobHandleB } });
  const cancelledBPage = cancelledB.result?.structuredContent?.page;
  if (
    cancelledB.result?.isError !== false ||
    cancelledB.result?.structuredContent?.jobHandle !== jobHandleB ||
    cancelledBPage?.jobId !== jobIdB ||
    cancelledBPage?.cancelRequested !== true
  ) {
    throw new Error("GIS Map Author B's MCP cancellation did not reach the exact paused Hub job");
  }
  await releaseGisInferenceCheckpointControl(run, jobIdB);
  let terminalB: Record<string, any> | undefined;
  const cancellationDeadline = Date.now() + 120_000;
  while (Date.now() < cancellationDeadline) {
    const eventsB = await callGisMapProcessMcp(mcpB, "tools/call", { name: "inference_events", arguments: { jobHandle: jobHandleB, after: 0 } });
    const page = eventsB.result?.structuredContent?.page;
    if (eventsB.result?.isError !== false || eventsB.result?.structuredContent?.jobHandle !== jobHandleB || page?.jobId !== jobIdB) {
      throw new Error("GIS Map Author B's MCP event page lost its retained job identity");
    }
    if (page.state === "cancelled") {
      terminalB = page;
      break;
    }
    if (!['accepted', 'running'].includes(page.state)) throw new Error(`GIS Map Author B's paused job reached unexpected state ${String(page.state)}`);
    await Bun.sleep(5);
  }
  const cancelledKinds = terminalB?.events?.map((event: any) => event?.kind);
  if (
    terminalB?.proposalState !== "cancelled" ||
    terminalB?.cancelRequested !== true ||
    terminalB?.proposalHash !== null ||
    terminalB?.preview !== undefined ||
    !Array.isArray(cancelledKinds) ||
    !cancelledKinds.includes("cancel-requested") ||
    !cancelledKinds.includes("cancelled")
  ) {
    throw new Error("GIS Map Author B's MCP-cancelled job did not terminate without an offer");
  }
  return jobIdB;
}
