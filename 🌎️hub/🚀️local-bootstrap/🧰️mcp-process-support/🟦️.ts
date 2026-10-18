/** 🧭️ Child-process support for the live hub gates: browser readiness, MCP workspaces, JSON-RPC waits and the admin SPA build.
 * @see ../../📦️packages/🦀️rust/📜️script.ts
 */
import { randomBytes } from "node:crypto";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import { requireMcpBinary } from "@semio-tech/framework-os-mcp";
import { cargoTargetDirectory } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { runCmd, orchestratorBudgetOpts } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { deliverCredentialEnvelopeToChild } from "../../🔐️auth/📤️credential-delivery/🟦️.ts";
import { LOCAL_READINESS_STALL_BOUND_MS, type LocalHubRun } from "../🏃️execution/🟦️.ts";

export async function waitForUiReadiness(origin: string, child: ChildProcess): Promise<void> {
  let observation = "";
  let observedAt = Date.now();
  for (;;) {
    if (child.exitCode !== null || child.signalCode !== null) throw new Error("secure local UI exited before readiness");
    let answer = "no-answer";
    try {
      const response = await fetch(`${origin}/`, { redirect: "error", signal: AbortSignal.timeout(500) });
      if (response.ok && response.url === `${origin}/`) return;
      answer = `http=${response.status} url=${response.url}`;
    } catch (error) {
      answer = error instanceof Error ? `unreachable=${error.message}` : "unreachable";
    }
    const now = Date.now();
    if (answer !== observation) {
      observation = answer;
      observedAt = now;
    } else if (now - observedAt >= LOCAL_READINESS_STALL_BOUND_MS) {
      throw new Error(`secure local UI readiness stalled — nothing changed for ${now - observedAt} ms; last observation: ${answer}`);
    }
    await Bun.sleep(50);
  }
}

export function openExternalBrowser(url: string): void {
  const command = process.platform === "darwin" ? "open" : process.platform === "win32" ? "cmd.exe" : "xdg-open";
  const args = process.platform === "win32" ? ["/d", "/s", "/c", "start", "", url] : [url];
  const opener = spawn(command, args, { shell: false, detached: true, stdio: "ignore" });
  opener.unref();
}

export function nativeWgpuExecutable(repoRoot: string): string {
  return join(cargoTargetDirectory(repoRoot), "debug", process.platform === "win32" ? "semio-wgpu-native.exe" : "semio-wgpu-native");
}

export function mcpExecutable(repoRoot: string): string {
  return requireMcpBinary(repoRoot);
}

const MCP_PROBE_SCHEMA = "os.agent.probe/v1";
const MCP_PROBE_PACK_SCHEMA_HASH = "0302ac7cf70cd2452759542a24931331935527fb9388bb6967cbc0575ccf728e";

async function createMcpProbeWorkspace(run: LocalHubRun, envelope: Record<string, any>): Promise<{ readonly spaceId: string; readonly documentId: string }> {
  const created = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), { kind: "create-space", name: "MCP Socket Grant Probe", spaceKind: "studio", visibility: "private" });
  if (created.status !== 202) throw new Error(`MCP process probe could not create its workspace: ${created.status}`);
  const body = JSON.parse(created.text) as Record<string, any>;
  const event = Array.isArray(body.events) ? body.events.find((candidate: any) => candidate?.body?.kind === "space.created") : undefined;
  const spaceId = event?.body?.spaceId;
  if (typeof spaceId !== "string" || spaceId.length === 0) throw new Error("MCP process probe create-space response lacked its exact identifier");
  const documentId = "mcp-socket-grant-probe";
  const announced = await postLiveDirectoryCommand(run, envelope.capability, liveDirectoryCommandRequestId(), {
    kind: "announce-document",
    descriptor: {
      spaceId,
      documentId,
      artifactKind: "os.agent.probe",
      artifactSchema: MCP_PROBE_SCHEMA,
      owner: { pluginId: "os.mcp", packageId: "os.mcp.probe", version: "1.0.0", packageHash: "22".repeat(32) },
      packSchemaHash: MCP_PROBE_PACK_SCHEMA_HASH,
      bootstrapVersion: 1,
      bootstrapFrontier: { headSeq: 0, commitSeq: 0, epoch: 0 },
      bootstrapSnapshotHash: "33".repeat(32),
    },
  });
  if (announced.status !== 202) throw new Error(`MCP process probe could not announce its document: ${announced.status}`);
  return { spaceId, documentId };
}

export async function startMcpWorkspaceChild(
  repoRoot: string,
  run: LocalHubRun,
  envelope: Record<string, any>,
  environmentSource: NodeJS.ProcessEnv = process.env,
): Promise<{ readonly child: ChildProcess; readonly spaceId: string; readonly documentId: string }> {
  const executable = mcpExecutable(repoRoot);
  if (!existsSync(executable)) throw new Error("owned MCP binary missing after build");
  const { spaceId, documentId } = await createMcpProbeWorkspace(run, envelope);
  const child = await deliverCredentialEnvelopeToChild(executable, ["stdio", "--hub", `http://127.0.0.1:${run.port}`, "--space", spaceId], structuredClone(envelope), "mcp", `http://127.0.0.1:${run.port}`, environmentSource);
  return { child, spaceId, documentId };
}

/** 🧫️ One real GIS Map ledger edit as the native fixture emitted it: the envelopes a replica's store
 * produced for that edit, in causal order, with their operation payloads. */
export type CheckInProcessEnvelopeV1 = {
  readonly mutationId: string;
  readonly dependencies: readonly string[];
  readonly diffSchema: string;
  readonly diff: string;
  readonly inverseSchema: string;
  readonly inverse: string;
};

export type CheckInProcessFixtureV1 = {
  readonly schema: "semio.hub.check-in-process-seed/v1";
  readonly profileId: string;
  readonly generationId: string;
  readonly documentId: string;
  readonly package: { readonly pluginId: string; readonly packageId: string; readonly version: string; readonly componentSha256: string };
  readonly artifact: { readonly kind: "s.gis.gismap"; readonly schema: "gis.map"; readonly packSchemaHash: string };
  readonly surfaceId: string;
  readonly payload: { readonly envelopes: { readonly path: string; readonly count: number; readonly sha256: string } };
};


export async function startMcpBoundWorkspaceChild(repoRoot: string, run: LocalHubRun, envelope: Record<string, any>, spaceId: string, environmentSource: NodeJS.ProcessEnv): Promise<ChildProcess> {
  const executable = requireMcpBinary(repoRoot, environmentSource);
  if (!existsSync(executable)) throw new Error("owned MCP binary missing after build");
  return deliverCredentialEnvelopeToChild(executable, ["stdio", "--hub", `http://127.0.0.1:${run.port}`, "--space", spaceId], structuredClone(envelope), "mcp", `http://127.0.0.1:${run.port}`, environmentSource);
}

export async function waitForDocumentSocketFrame<T>(socket: WebSocket, frames: readonly Record<string, any>[], select: (frame: Record<string, any>) => T | undefined, label: string, after = 0): Promise<T> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    for (let index = after; index < frames.length; index++) {
      const selected = select(frames[index]!);
      if (selected !== undefined) return selected;
    }
    if (socket.readyState >= WebSocket.CLOSING) throw new Error(`document socket closed before ${label}`);
    if (Date.now() >= deadline) throw new Error(`document socket ${label} deadline exceeded`);
    await Bun.sleep(20);
  }
}

export function jsonRpcResponses(chunks: readonly Buffer[]): Record<string, any>[] {
  const text = Buffer.concat(chunks).toString("utf8");
  const lines = text.split("\n");
  if (!text.endsWith("\n")) lines.pop();
  return lines.filter(Boolean).map((line) => JSON.parse(line) as Record<string, any>);
}

export async function waitForJsonRpcResponse(child: ChildProcess, chunks: readonly Buffer[], id: number): Promise<Record<string, any>> {
  const deadline = Date.now() + 10_000;
  for (;;) {
    const response = jsonRpcResponses(chunks).find((candidate) => candidate.id === id);
    if (response) return response;
    if (child.exitCode !== null) throw new Error(`MCP direct child exited before JSON-RPC response ${id}: ${child.exitCode}`);
    if (Date.now() >= deadline) throw new Error(`MCP direct child JSON-RPC response ${id} deadline exceeded`);
    await Bun.sleep(20);
  }
}

/** 🛡️ `os-hub-admin`'s build MUST land before cargo ever runs `main()` for real — `HubState.
 * admin_dir` (§C0 `OS_HUB_ADMIN_DIR`, else the compile-time default) is read at hub STARTUP, not
 * build time, so this is a runtime prerequisite, not a Cargo `build.rs` concern. Zero-touch/
 * cross-platform: `bun repo run os-hub-admin:build` is the same command every OS/devcontainer already
 * runs for every other build target here. */
export function buildAdminSpa(repoRoot: string): void {
  runCmd("bun", ["repo", "run", "os-hub-admin:build"], { cwd: repoRoot, ...orchestratorBudgetOpts() });
}

/** 🆔️ Mints one fresh 32-hex nonzero idempotency correlation for a live process command. */
export function liveDirectoryCommandRequestId(): string {
  return randomBytes(16).toString("hex");
}

/** 🧾️ Posts one sealed `DirectoryCommandRequestV1` and returns the raw status plus response text —
 * the caller decides whether a non-2xx or a redacted receipt is the expected law. */
export async function postLiveDirectoryCommand(run: LocalHubRun, capability: string, requestId: string, command: Record<string, unknown>, body?: string): Promise<{ readonly status: number; readonly text: string }> {
  const response = await fetch(`http://127.0.0.1:${run.port}/directory/commands`, {
    method: "POST",
    headers: { authorization: `Bearer ${capability}`, "content-type": "application/json" },
    body: body ?? JSON.stringify({ schema: "semio.directory.command-request.v1", requestId, command }),
    signal: AbortSignal.timeout(5_000),
  });
  return { status: response.status, text: await response.text() };
}
