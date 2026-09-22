import { randomBytes } from "node:crypto";
import { spawn, spawnSync, type ChildProcess } from "node:child_process";
import { chmodSync, existsSync, mkdtempSync, rmSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join } from "node:path";
import type { Duplex } from "node:stream";
import { cargoTargetDirectory } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { getWorkspaceRoot } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🟦️.ts";
import { GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES } from "../../💡️inference/🧬️schema/🟦️.ts";
import { authenticatedFrame, LOCAL_BOOTSTRAP_SCHEMA, type LocalProfile, verifyAuthenticatedFrame } from "../🛂authentication/🟦️.ts";
import { LOCAL_BOOTSTRAP_DEADLINE_MS, LocalFrameReader, writeLocalFrame } from "../📡️framing/🟦️.ts";

/** ⏳ The span of NO OBSERVABLE PROGRESS that ends a readiness wait. It is not a total budget and
 * never was a good one: a total wall budget charges a booting hub for every millisecond the machine
 * spends on other work, so a launcher on a busy machine abandoned a hub that was still coming up
 * (measured 2026-09-21 by CE2 — two of three 7621 starts, while 17 rustc ran). The number is
 * unchanged from the total deadline it replaces: nothing here is a lengthened timeout. */
export const LOCAL_READINESS_STALL_BOUND_MS = 30_000;

export type LocalHubRunAllocationOperations = Readonly<{
  platform: NodeJS.Platform;
  makeTemporaryDirectory: (prefix: string) => string;
  protectDirectory: (path: string) => void;
  removeDirectory: (path: string) => void;
}>;

export type LocalHubStartOptions = Readonly<{
  port?: number;
  dataDir?: string;
  capture?: boolean;
  adminSubjects?: readonly string[];
  isolatedSecuritySmoke?: boolean;
  binaryPath?: string;
  inferenceCheckpointControl?: boolean;
  runParent?: string;
  allocationOperations?: LocalHubRunAllocationOperations;
}>;

export type LocalHubRun = {
  readonly child: ChildProcess;
  readonly pipe: Duplex;
  readonly reader: LocalFrameReader;
  readonly channelKey: Buffer;
  readonly runId: string;
  readonly port: number;
  readonly runRoot: string;
  /** 🎫️ Whether this run asked the hub to also mint sessions from public credentials
   * (`OS_HUB_CREDENTIAL_SIGN_IN`). A development hub issues through the pipe alone by default; a run
   * that did not ask for public issuance must still see `/readyz` deny it, which is what
   * {@link localHubReadinessAdmitted} compares against. */
  readonly publicSessionIssuance: boolean;
  readonly output: () => string;
  readonly removeRunRoot: () => void;
  readonly inferenceCheckpointPipe?: Duplex;
  readonly inferenceCheckpointReader?: LocalFrameReader;
  inferenceCheckpointEnteredJobId?: string;
  inferenceCheckpointProgress?: Readonly<{ progressCursor: number; completed: number; total: number }>;
  inferenceCheckpointReleased?: boolean;
  finishPromise?: Promise<void>;
};

const nativeAllocationOperations: LocalHubRunAllocationOperations = {
  platform: process.platform,
  makeTemporaryDirectory: (prefix) => mkdtempSync(prefix),
  protectDirectory: (path) => chmodSync(path, 0o700),
  removeDirectory: (path) => rmSync(path, { recursive: true, force: true }),
};

/** 📁 Allocates one private local-Hub root through an injectable, owned filesystem boundary. */
export function allocateLocalHubRunRoot(parent = tmpdir(), operations: LocalHubRunAllocationOperations = nativeAllocationOperations): Readonly<{ path: string; remove: () => void }> {
  const path = operations.makeTemporaryDirectory(join(parent, "semio-hub-run-"));
  if (operations.platform !== "win32") operations.protectDirectory(path);
  return { path, remove: () => operations.removeDirectory(path) };
}

/** 🔌 Reserves and releases one loopback port before a bounded child starts. */
export async function freeLoopbackPort(): Promise<number> {
  const server = createServer();
  await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("local bootstrap launcher could not allocate a loopback port");
  await new Promise<void>((resolveClose, rejectClose) => server.close((error) => (error ? rejectClose(error) : resolveClose())));
  return address.port;
}

/** 🗺️ Resolves the workspace-cached debug Hub executable for native proofs. */
export function hubBinaryPath(repoRoot: string): string {
  return join(cargoTargetDirectory(repoRoot), "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub");
}

/** 🎯 The single Nx target that stages the development executable every launch route reads. */
export const HUB_DEV_BINARY_TARGET = "os-hub:build-dev";

/** 🐘️ The Nx target that stages the PostgreSQL-capable development executable — the SAME hub with
 * `db`'s `postgres` storage/directory drivers linked in, which the default build deliberately leaves
 * out so a zero-touch launch needs no database. */
export const HUB_DEV_POSTGRES_BINARY_TARGET = "os-hub:build-dev-postgres";

/** 🏗️ The injectable staging boundary: one attempt that returns the exit status of the Nx target. */
export type HubDevBinaryStaging = Readonly<{ stage: () => number }>;

const nativeHubDevBinaryStaging: HubDevBinaryStaging = {
  stage: () => spawnSync("bun", ["nx", "run", HUB_DEV_BINARY_TARGET], { cwd: getWorkspaceRoot(), stdio: "inherit", shell: false }).status ?? -1,
};

const nativeHubDevPostgresBinaryStaging: HubDevBinaryStaging = {
  stage: () => spawnSync("bun", ["nx", "run", HUB_DEV_POSTGRES_BINARY_TARGET], { cwd: getWorkspaceRoot(), stdio: "inherit", shell: false }).status ?? -1,
};

/** 📦 Reads the Nx-staged development executable, staging it through its own Nx target when absent instead of
 * failing a launch route with a missing file, and naming that target when the staging itself fails. */
export function hubDevBinaryPath(root: string, staging: HubDevBinaryStaging = nativeHubDevBinaryStaging): string {
  const path = join(root, "dist", "build-dev", process.platform === "win32" ? "os-hub.exe" : "os-hub");
  if (existsSync(path)) return path;
  const status = staging.stage();
  if (status !== 0 || !existsSync(path)) throw new Error(`Missing Nx-staged os-hub dev binary: ${path}; staging it through \`bun nx run ${HUB_DEV_BINARY_TARGET}\` exited with status ${status}`);
  return path;
}

/** 🐘️ The same read for the PostgreSQL-capable executable, staged into its own directory so the two
 * feature sets never overwrite one another (and a running hub is never replaced in place). */
export function hubDevPostgresBinaryPath(root: string, staging: HubDevBinaryStaging = nativeHubDevPostgresBinaryStaging): string {
  const path = join(root, "dist", "build-dev-postgres", process.platform === "win32" ? "os-hub.exe" : "os-hub");
  if (existsSync(path)) return path;
  const status = staging.stage();
  if (status !== 0 || !existsSync(path)) throw new Error(`Missing Nx-staged PostgreSQL os-hub dev binary: ${path}; staging it through \`bun nx run ${HUB_DEV_POSTGRES_BINARY_TARGET}\` exited with status ${status}`);
  return path;
}

/** 🚀 Starts one loopback Hub and completes its authenticated local-bootstrap handshake. */
export async function startLocalHub(repoRoot: string, root: string, profiles: readonly LocalProfile[], options: LocalHubStartOptions = {}): Promise<LocalHubRun> {
  if (profiles.length === 0 || profiles.length > 8) throw new Error("local bootstrap profiles must contain 1..=8 entries");
  const runId = randomBytes(16).toString("hex");
  const channelKey = randomBytes(32);
  const allocation = allocateLocalHubRunRoot(options.runParent, options.allocationOperations);
  const runRoot = allocation.path;
  const port = options.port ?? (await freeLoopbackPort());
  const captured: Buffer[] = [];
  let capturedBytes = 0;
  const capture = (chunk: Buffer): void => {
    const remaining = 1024 * 1024 - capturedBytes;
    if (remaining <= 0) return;
    const retained = Buffer.from(chunk.subarray(0, remaining));
    captured.push(retained);
    capturedBytes += retained.length;
  };
  const env: NodeJS.ProcessEnv = {
    ...process.env,
    OS_HUB_MODE: "development",
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_PORT: String(port),
    OS_HUB_DATA: options.dataDir ?? join(runRoot, "data"),
  };
  if (options.inferenceCheckpointControl) env.OS_HUB_TEST_INFERENCE_CHECKPOINT_FD = "4";
  else delete env.OS_HUB_TEST_INFERENCE_CHECKPOINT_FD;
  delete env.OS_HUB_TRUSTED_CATALOG_BUNDLE;
  delete env.OS_HUB_TRUSTED_CATALOG_PROFILE;
  delete env.OS_HUB_ADMIN_TOKEN;
  delete env.S_USER;
  for (const name of Object.keys(env)) if (/^S_.*TOKEN$/.test(name)) delete env[name];
  if (options.isolatedSecuritySmoke) {
    env.OS_HUB_STORAGE_BACKEND = "fs";
    env.OS_HUB_DIRECTORY_BACKEND = "sqlite";
    delete env.OS_HUB_ADMIN_DIR;
  }
  if (options.adminSubjects?.length) env.OS_HUB_ADMIN_SUBJECTS = options.adminSubjects.join(",");
  else delete env.OS_HUB_ADMIN_SUBJECTS;
  const publicSessionIssuance = env.OS_HUB_CREDENTIAL_SIGN_IN === "true" || env.OS_HUB_CREDENTIAL_SIGN_IN === "1";
  const outputMode: "pipe" | "inherit" = options.capture ? "pipe" : "inherit";
  const child = spawn(options.binaryPath ?? hubBinaryPath(repoRoot), [], {
    cwd: root,
    env,
    shell: false,
    stdio: ["ignore", outputMode, outputMode, "pipe", options.inferenceCheckpointControl ? "pipe" : "ignore"],
  });
  if (options.capture) {
    child.stdout?.on("data", capture);
    child.stderr?.on("data", capture);
  }
  const pipe = child.stdio[3] as Duplex;
  const inferenceCheckpointPipe = options.inferenceCheckpointControl ? (child.stdio[4] as Duplex) : undefined;
  if (!pipe || (options.inferenceCheckpointControl && !inferenceCheckpointPipe)) {
    channelKey.fill(0);
    child.kill();
    await waitForChildExit(child, 2_000).catch(() => undefined);
    allocation.remove();
    throw new Error("local bootstrap inherited endpoint was not created");
  }
  const reader = new LocalFrameReader(pipe);
  const run: LocalHubRun = {
    child,
    pipe,
    reader,
    channelKey,
    runId,
    port,
    runRoot,
    publicSessionIssuance,
    output: () => Buffer.concat(captured).toString("utf8"),
    removeRunRoot: allocation.remove,
    inferenceCheckpointPipe,
    inferenceCheckpointReader: inferenceCheckpointPipe ? new LocalFrameReader(inferenceCheckpointPipe, GIS_INFERENCE_CHECKPOINT_CONTROL_FRAME_MAX_BYTES + 4, "GIS inference checkpoint control") : undefined,
    inferenceCheckpointReleased: inferenceCheckpointPipe ? false : undefined,
  };
  try {
    const initialize = { schema: LOCAL_BOOTSTRAP_SCHEMA, kind: "initialize", runId, channelKey: channelKey.toString("hex"), profiles };
    await writeLocalFrame(pipe, initialize);
    initialize.channelKey = "";
    const now = Date.now();
    const helloExchange = randomBytes(16).toString("hex");
    const hello = authenticatedFrame(channelKey, {
      schema: LOCAL_BOOTSTRAP_SCHEMA,
      kind: "hello",
      runId,
      sequence: 1,
      exchangeId: helloExchange,
      issuedAt: now,
      expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
      launcherNonce: randomBytes(32).toString("hex"),
    });
    await writeLocalFrame(pipe, hello);
    const accepted = await reader.read();
    verifyAuthenticatedFrame(channelKey, accepted);
    if (accepted.schema !== LOCAL_BOOTSTRAP_SCHEMA || accepted.kind !== "hello-accepted" || accepted.runId !== runId || accepted.exchangeId !== helloExchange || accepted.sequence !== 1)
      throw new Error("local bootstrap mutual hello binding mismatch");
    return run;
  } catch (error) {
    let diagnostics = run.output().slice(-2_048).replaceAll(channelKey.toString("hex"), "<channel-key-redacted>");
    for (const profile of profiles) diagnostics = diagnostics.replaceAll(profile.subject, "<profile-subject-redacted>");
    const status = child.exitCode;
    await finishLocalHub(run);
    throw new Error(`local bootstrap handshake failed (child status ${status ?? "running"}): ${error instanceof Error ? error.message : "unknown"}\n${diagnostics}`);
  }
}

/** ✅ Classifies a readiness response against the exact local run and component boundary. */
export function localHubReadinessAdmitted(body: Record<string, any>, status: number, runId: string, bootstrapSecuritySmoke = false, publicSessionIssuance = false): boolean {
  if (body.schema !== "semio.hub.readiness/v1" || body.runId !== runId || body.mode !== "development" || body.bindScope !== "loopback" || body.authentication?.kind !== "local-bootstrap-pipe-v1" || body.authentication?.publicSessionIssuance !== publicSessionIssuance)
    throw new Error("hub readiness binding mismatch");
  const componentsReady = body.directory?.ready === true && body.storage?.ready === true && body.adminAssets?.ready === true;
  const fullyReady = status === 200 && body.status === "ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === true;
  const bootstrapReadyOnly = status === 503 && body.status === "not-ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === false;
  return fullyReady || (bootstrapSecuritySmoke && bootstrapReadyOnly);
}

/** 🔭 Everything one poll can observe about a booting hub, flattened into the exact string whose
 * CHANGE means the hub advanced: whether `/readyz` answered at all, its HTTP status, the readiness
 * status it declared, its closed gates, and how many bytes the hub has written (captured runs only —
 * the hub rate-limits an in-flight `server.catalog.publication` record so a pre-bind catalog load is
 * visible here too). Anything new in this string restarts {@link LOCAL_READINESS_STALL_BOUND_MS}. */
function localHubReadinessObservation(run: LocalHubRun, answer: string): string {
  return `${answer} bytes=${run.output().length}`;
}

/** ⏱️ Polls the bounded readiness endpoint until the exact local run is admitted, bounded by how long
 * the hub goes without showing any sign of advancing rather than by how long the boot takes. */
export async function waitForReadiness(run: LocalHubRun, bootstrapSecuritySmoke = false, stallBoundMs = LOCAL_READINESS_STALL_BOUND_MS): Promise<Record<string, any>> {
  let closedGates = "no /readyz answer was ever received";
  let observation = "";
  let observedAt = Date.now();
  for (;;) {
    if (run.child.exitCode !== null) throw new Error("hub exited before readiness");
    let answer = "no-answer";
    try {
      const response = await fetch(`http://127.0.0.1:${run.port}/readyz`, { signal: AbortSignal.timeout(1000) });
      const body = (await response.json()) as Record<string, any>;
      if (localHubReadinessAdmitted(body, response.status, run.runId, bootstrapSecuritySmoke, run.publicSessionIssuance)) return body;
      const blocked = Array.isArray(body.blockedBy) ? (body.blockedBy as { gate: string; reason: string }[]) : [];
      closedGates = blocked.length ? blocked.map((closed) => `${closed.gate}=${closed.reason}`).join(" ") : `status=${body.status} with no closed gate declared`;
      answer = `http=${response.status} ${closedGates}`;
    } catch (error) {
      if (error instanceof Error && error.message === "hub readiness binding mismatch") throw error;
    }
    const next = localHubReadinessObservation(run, answer);
    const now = Date.now();
    if (next !== observation) {
      observation = next;
      observedAt = now;
    } else if (now - observedAt >= stallBoundMs) {
      throw new Error(`hub readiness stalled — nothing about the hub changed for ${now - observedAt} ms; last observation: ${next}; closed gates: ${closedGates}`);
    }
    await new Promise<void>((resolveDelay) => setTimeout(resolveDelay, 50));
  }
}

/** ⏳ Waits for one already-owned child without taking over its termination policy. */
export async function waitForChildExit(child: ChildProcess, deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS + 2_000): Promise<void> {
  if (child.exitCode !== null) return;
  await Promise.race([new Promise<void>((resolveExit) => child.once("exit", () => resolveExit())), new Promise<void>((_, reject) => setTimeout(() => reject(new Error("hub child exit deadline exceeded")), deadlineMs))]);
}

/** 🧹 Finishes one local Hub exactly once, clears secrets, and removes only its allocated root. */
export async function finishLocalHub(run: LocalHubRun): Promise<void> {
  if (run.finishPromise) return run.finishPromise;
  run.finishPromise = (async () => {
    run.pipe.end();
    run.inferenceCheckpointPipe?.end();
    run.channelKey.fill(0);
    if (run.child.exitCode === null) {
      try {
        await waitForChildExit(run.child, 2_000);
      } catch {
        run.child.kill();
        await waitForChildExit(run.child, 2_000).catch(() => undefined);
      }
    }
    run.removeRunRoot();
  })();
  return run.finishPromise;
}
