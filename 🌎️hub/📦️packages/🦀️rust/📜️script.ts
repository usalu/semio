#!/usr/bin/env bun
import { createHmac, randomBytes } from "node:crypto";
import { chmodSync, mkdtempSync, rmSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { spawn, type ChildProcess } from "node:child_process";
import type { Duplex } from "node:stream";
/** 🌎️ `os-hub` router: `bun ./📜️script.ts <setup|build|test|dev>`. */
import { BundleScript, ScriptRouter, OS_HUB_PORT, OS_HUB_PORT_ENV, runBundleScriptMain, runCargo, runCargoTestBudgeted, runCmd, orchestratorBudgetOpts, resolveTestLevel } from "../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

const LOCAL_BOOTSTRAP_SCHEMA = "semio.hub.local-bootstrap/v1";
const LOCAL_BOOTSTRAP_DOMAIN = "semio/hub/local-bootstrap/v1\0";
const LOCAL_BOOTSTRAP_FRAME_MAX = 16 * 1024;
const LOCAL_BOOTSTRAP_DEADLINE_MS = 15_000;
const LOCAL_READINESS_DEADLINE_MS = 30_000;
type LocalClientClass = "native" | "mcp" | "react-relay" | "admin-relay";
type LocalProfile = { readonly profileId: string; readonly subject: string; readonly displayName: string; readonly allowedClientClasses: readonly LocalClientClass[] };

class LocalFrameReader {
  private retained = Buffer.alloc(0);
  private readonly waiters: Array<{ resolve: (value: Record<string, any>) => void; reject: (error: Error) => void; timer: ReturnType<typeof setTimeout> }> = [];

  constructor(private readonly pipe: Duplex) {
    pipe.on("data", (chunk: Buffer) => {
      if (this.retained.length + chunk.length > LOCAL_BOOTSTRAP_FRAME_MAX * 2) return this.fail(new Error("local bootstrap retained input exceeded fixed bound"));
      this.retained = Buffer.concat([this.retained, chunk]);
      this.drain();
    });
    pipe.once("error", () => this.fail(new Error("local bootstrap endpoint failed")));
    pipe.once("end", () => this.fail(new Error("local bootstrap endpoint reached EOF")));
    pipe.once("close", () => this.fail(new Error("local bootstrap endpoint closed")));
  }

  read(deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS): Promise<Record<string, any>> {
    if (!Number.isSafeInteger(deadlineMs) || deadlineMs <= 0) return Promise.reject(new Error("local bootstrap frame deadline invalid"));
    if (this.waiters.length >= 8) return Promise.reject(new Error("local bootstrap outstanding read bound exceeded"));
    return new Promise((resolveRead, rejectRead) => {
      const waiter = {
        resolve: resolveRead,
        reject: rejectRead,
        timer: setTimeout(() => {
          const index = this.waiters.indexOf(waiter);
          if (index >= 0) this.waiters.splice(index, 1);
          rejectRead(new Error("local bootstrap frame deadline exceeded"));
        }, deadlineMs),
      };
      this.waiters.push(waiter);
      this.drain();
    });
  }

  private drain(): void {
    while (this.waiters.length > 0 && this.retained.length >= 4) {
      const length = this.retained.readUInt32BE(0);
      if (length === 0 || length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) return this.fail(new Error("local bootstrap frame exceeded fixed bound"));
      if (this.retained.length < length + 4) return;
      const bytes = this.retained.subarray(4, length + 4);
      this.retained = this.retained.subarray(length + 4);
      const waiter = this.waiters.shift()!;
      clearTimeout(waiter.timer);
      try {
        waiter.resolve(JSON.parse(bytes.toString("utf8")));
      } catch {
        waiter.reject(new Error("local bootstrap frame was not JSON"));
      }
    }
  }

  private fail(error: Error): void {
    for (const waiter of this.waiters.splice(0)) {
      clearTimeout(waiter.timer);
      waiter.reject(error);
    }
    this.retained.fill(0);
    this.retained = Buffer.alloc(0);
  }
}

function hmacProof(channelKey: Buffer, unsigned: object): string {
  const canonical = Buffer.from(JSON.stringify(unsigned));
  if (canonical.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap canonical frame exceeded fixed bound");
  const length = Buffer.alloc(4);
  length.writeUInt32BE(canonical.length);
  const proof = createHmac("sha256", channelKey).update(LOCAL_BOOTSTRAP_DOMAIN).update(length).update(canonical).digest("hex");
  canonical.fill(0);
  return proof;
}

function authenticatedFrame(channelKey: Buffer, unsigned: Record<string, unknown>): Record<string, unknown> {
  return { ...unsigned, proof: hmacProof(channelKey, unsigned) };
}

function writeLocalFrame(pipe: Duplex, value: object): Promise<void> {
  const bytes = Buffer.from(JSON.stringify(value));
  if (bytes.length === 0 || bytes.length + 4 > LOCAL_BOOTSTRAP_FRAME_MAX) throw new Error("local bootstrap frame exceeded fixed bound");
  const frame = Buffer.allocUnsafe(bytes.length + 4);
  frame.writeUInt32BE(bytes.length, 0);
  bytes.copy(frame, 4);
  bytes.fill(0);
  return new Promise((resolveWrite, rejectWrite) => {
    pipe.write(frame, (error?: Error | null) => {
      frame.fill(0);
      if (error) rejectWrite(new Error("local bootstrap write failed"));
      else resolveWrite();
    });
  });
}

function verifyAuthenticatedFrame(channelKey: Buffer, frame: Record<string, unknown>): void {
  const proof = frame.proof;
  if (typeof proof !== "string" || !/^[0-9a-f]{64}$/.test(proof)) throw new Error("local bootstrap response proof invalid");
  const unsigned = { ...frame };
  delete unsigned.proof;
  const expected = hmacProof(channelKey, unsigned);
  const left = Buffer.from(expected, "hex");
  const right = Buffer.from(proof, "hex");
  let difference = 0;
  for (let index = 0; index < left.length; index++) difference |= left[index]! ^ right[index]!;
  left.fill(0);
  right.fill(0);
  if (difference !== 0) throw new Error("local bootstrap response proof invalid");
}

async function freeLoopbackPort(): Promise<number> {
  const server = createServer();
  await new Promise<void>((resolveListen, rejectListen) => server.once("error", rejectListen).listen(0, "127.0.0.1", resolveListen));
  const address = server.address();
  if (!address || typeof address === "string") throw new Error("local bootstrap launcher could not allocate a loopback port");
  await new Promise<void>((resolveClose, rejectClose) => server.close((error) => error ? rejectClose(error) : resolveClose()));
  return address.port;
}

function hubBinaryPath(repoRoot: string): string {
  const target = process.env.CARGO_TARGET_DIR ? resolve(process.env.CARGO_TARGET_DIR) : join(repoRoot, "target");
  return join(target, "debug", process.platform === "win32" ? "os-hub.exe" : "os-hub");
}

type LocalHubRun = {
  readonly child: ChildProcess;
  readonly pipe: Duplex;
  readonly reader: LocalFrameReader;
  readonly channelKey: Buffer;
  readonly runId: string;
  readonly port: number;
  readonly runRoot: string;
  readonly output: () => string;
  finishPromise?: Promise<void>;
};

async function startLocalHub(
  repoRoot: string,
  root: string,
  profiles: readonly LocalProfile[],
  options: { readonly port?: number; readonly dataDir?: string; readonly capture?: boolean; readonly adminSubjects?: readonly string[]; readonly isolatedSecuritySmoke?: boolean } = {},
): Promise<LocalHubRun> {
  if (profiles.length === 0 || profiles.length > 8) throw new Error("local bootstrap profiles must contain 1..=8 entries");
  const runId = randomBytes(16).toString("hex");
  const channelKey = randomBytes(32);
  const runRoot = mkdtempSync(join(tmpdir(), "semio-hub-run-"));
  if (process.platform !== "win32") chmodSync(runRoot, 0o700);
  const port = options.port ?? await freeLoopbackPort();
  const captured: Buffer[] = [];
  let capturedBytes = 0;
  const capture = (chunk: Buffer): void => {
    const remaining = 1024 * 1024 - capturedBytes;
    if (remaining <= 0) return;
    const retained = Buffer.from(chunk.subarray(0, remaining));
    captured.push(retained);
    capturedBytes += retained.length;
  };
  const env: Record<string, string | undefined> = {
    ...process.env,
    OS_HUB_MODE: "development",
    OS_HUB_BIND: "127.0.0.1",
    OS_HUB_PORT: String(port),
    OS_HUB_DATA: options.dataDir ?? join(runRoot, "data"),
  };
  delete env.OS_HUB_ADMIN_TOKEN;
  delete env.S_USER;
  for (const name of Object.keys(env)) if (/^S_.*TOKEN$/.test(name)) delete env[name];
  if (options.isolatedSecuritySmoke) {
    env.OS_HUB_STORAGE_BACKEND = "fs";
    env.OS_HUB_DIRECTORY_BACKEND = "sqlite";
    delete env.OS_HUB_TRUSTED_CATALOG_BUNDLE;
    delete env.OS_HUB_TRUSTED_CATALOG_PROFILE;
    delete env.OS_HUB_ADMIN_DIR;
  }
  if (options.adminSubjects?.length) env.OS_HUB_ADMIN_SUBJECTS = options.adminSubjects.join(",");
  else delete env.OS_HUB_ADMIN_SUBJECTS;
  const outputMode: "pipe" | "inherit" = options.capture ? "pipe" : "inherit";
  const child = spawn(hubBinaryPath(repoRoot), [], { cwd: root, env, shell: false, stdio: ["ignore", outputMode, outputMode, "pipe"] });
  if (options.capture) {
    child.stdout?.on("data", capture);
    child.stderr?.on("data", capture);
  }
  const pipe = child.stdio[3] as Duplex;
  if (!pipe) {
    channelKey.fill(0);
    child.kill();
    await waitForChildExit(child, 2_000).catch(() => undefined);
    rmSync(runRoot, { recursive: true, force: true });
    throw new Error("local bootstrap inherited endpoint was not created");
  }
  const reader = new LocalFrameReader(pipe);
  const run: LocalHubRun = { child, pipe, reader, channelKey, runId, port, runRoot, output: () => Buffer.concat(captured).toString("utf8") };
  try {
    const initialize = {
      schema: LOCAL_BOOTSTRAP_SCHEMA,
      kind: "initialize",
      runId,
      channelKey: channelKey.toString("hex"),
      profiles,
    };
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
    if (accepted.schema !== LOCAL_BOOTSTRAP_SCHEMA || accepted.kind !== "hello-accepted" || accepted.runId !== runId || accepted.exchangeId !== helloExchange || accepted.sequence !== 1) {
      throw new Error("local bootstrap mutual hello binding mismatch");
    }
    return run;
  } catch (error) {
    let diagnostics = run.output().slice(-2_048).replaceAll(channelKey.toString("hex"), "<channel-key-redacted>");
    for (const profile of profiles) diagnostics = diagnostics.replaceAll(profile.subject, "<profile-subject-redacted>");
    const status = child.exitCode;
    await finishLocalHub(run);
    throw new Error(`local bootstrap handshake failed (child status ${status ?? "running"}): ${error instanceof Error ? error.message : "unknown"}\n${diagnostics}`);
  }
}

async function issueLocalCredential(run: LocalHubRun, profileId: string, clientClass: LocalClientClass, sequence = 2, exchangeId = randomBytes(16).toString("hex")): Promise<Record<string, any>> {
  const now = Date.now();
  const issue = authenticatedFrame(run.channelKey, {
    schema: LOCAL_BOOTSTRAP_SCHEMA,
    kind: "issue",
    runId: run.runId,
    sequence,
    exchangeId,
    issuedAt: now,
    expiresAt: now + LOCAL_BOOTSTRAP_DEADLINE_MS,
    profileId,
    deviceInstanceId: `${clientClass}-launcher`,
    clientClass,
  });
  await writeLocalFrame(run.pipe, issue);
  const envelope = await run.reader.read();
  verifyAuthenticatedFrame(run.channelKey, envelope);
  if (envelope.schema !== "semio.hub.local-credential-envelope/v1" || envelope.runId !== run.runId || envelope.exchangeId !== exchangeId || envelope.profileId !== profileId || envelope.clientClass !== clientClass || envelope.sessionKind !== "development-local" || !Number.isSafeInteger(envelope.authorizationGeneration) || envelope.authorizationGeneration < 1) {
    throw new Error("local credential envelope binding mismatch");
  }
  return envelope;
}

async function waitForReadiness(run: LocalHubRun, bootstrapSecuritySmoke = false): Promise<Record<string, any>> {
  const deadline = Date.now() + LOCAL_READINESS_DEADLINE_MS;
  while (Date.now() < deadline) {
    if (run.child.exitCode !== null) throw new Error("hub exited before readiness");
    try {
      const response = await fetch(`http://127.0.0.1:${run.port}/readyz`, { signal: AbortSignal.timeout(1000) });
      const body = await response.json() as Record<string, any>;
      if (body.schema !== "semio.hub.readiness/v1" || body.runId !== run.runId || body.mode !== "development" || body.bindScope !== "loopback" || body.authentication?.kind !== "local-bootstrap-pipe-v1" || body.authentication?.publicSessionIssuance !== false) {
        throw new Error("hub readiness binding mismatch");
      }
      const componentsReady = body.directory?.ready === true && body.storage?.ready === true && body.adminAssets?.ready === true;
      const fullyReady = response.status === 200 && body.status === "ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === true;
      const bootstrapReadyOnly = response.status === 503 && body.status === "not-ready" && body.authentication.bootstrapReady === true && componentsReady && body.artifactAuthority?.ready === false;
      if (fullyReady || (bootstrapSecuritySmoke && bootstrapReadyOnly)) {
        return body;
      }
    } catch (error) {
      if (error instanceof Error && error.message === "hub readiness binding mismatch") throw error;
    }
    await Bun.sleep(50);
  }
  throw new Error("hub readiness deadline exceeded");
}

async function waitForChildExit(child: ChildProcess, deadlineMs = LOCAL_BOOTSTRAP_DEADLINE_MS + 2_000): Promise<void> {
  if (child.exitCode !== null) return;
  await Promise.race([
    new Promise<void>((resolveExit) => child.once("exit", () => resolveExit())),
    Bun.sleep(deadlineMs).then(() => { throw new Error("hub child exit deadline exceeded"); }),
  ]);
}

async function finishLocalHub(run: LocalHubRun): Promise<void> {
  if (run.finishPromise) return run.finishPromise;
  run.finishPromise = (async () => {
    run.pipe.end();
    run.channelKey.fill(0);
    if (run.child.exitCode === null) {
      try {
        await waitForChildExit(run.child, 2_000);
      } catch {
        run.child.kill();
        await waitForChildExit(run.child, 2_000).catch(() => undefined);
      }
    }
    rmSync(run.runRoot, { recursive: true, force: true });
  })();
  return run.finishPromise;
}

async function deliverCredentialEnvelopeToChild(executable: string, args: readonly string[], envelope: Record<string, any>, expectedClass: "native" | "mcp"): Promise<ChildProcess> {
  if (envelope.clientClass !== expectedClass) throw new Error("credential envelope client class mismatch");
  const child = spawn(executable, [...args], { shell: false, stdio: expectedClass === "mcp" ? ["pipe", "pipe", "pipe", "pipe"] : ["ignore", "pipe", "pipe", "pipe"] });
  const pipe = child.stdio[3] as Duplex;
  if (!pipe) {
    child.kill();
    envelope.capability = "";
    throw new Error("one-shot credential endpoint was not created");
  }
  try {
    await writeLocalFrame(pipe, envelope);
    pipe.end();
    return child;
  } catch (error) {
    child.kill();
    throw error;
  } finally {
    envelope.capability = "";
  }
}

export async function deliverNativeCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "native");
}

export async function deliverMcpCredentialEnvelope(executable: string, args: readonly string[], envelope: Record<string, any>): Promise<ChildProcess> {
  return deliverCredentialEnvelopeToChild(executable, args, envelope, "mcp");
}

async function proveCredentialEnvelopeDelivery(envelope: Record<string, any>, clientClass: "native" | "mcp"): Promise<void> {
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
    if (value.schema !== "semio.hub.local-credential-envelope/v1" || value.clientClass !== expectedClass || value.sessionKind !== "development-local" || !Number.isSafeInteger(value.authorizationGeneration) || value.authorizationGeneration < 1 || !/^session\.v1\.[0-9a-f]{32}\.[0-9a-f]{64}$/.test(value.capability)) return reject(new Error("binding"));
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
  const child = await delivery(process.execPath, ["-e", consumer, clientClass], structuredClone(envelope));
  let output = "";
  child.stdout?.on("data", (chunk: Buffer) => { if (output.length < 256) output += chunk.toString("utf8"); });
  if (clientClass === "mcp") child.stdin?.end('{"jsonrpc":"2.0","id":1}\n');
  await waitForChildExit(child, 5_000);
  if (child.exitCode !== 0 || output !== "credential-delivery-ok") throw new Error(`${clientClass} credential delivery failed`);
}

type RejectionCase = "wrong-hmac" | "wrong-class" | "wrong-profile" | "expired" | "cross-run" | "timeout" | "eof";

async function proveRejectedBootstrap(repoRoot: string, root: string, rejection: RejectionCase): Promise<void> {
  const profiles: readonly LocalProfile[] = [{ profileId: "developer", subject: "negative-local-developer", displayName: "Negative Developer", allowedClientClasses: ["native", "mcp"] }];
  const run = await startLocalHub(repoRoot, root, profiles, { capture: true, isolatedSecuritySmoke: true });
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
    await waitForChildExit(run.child).catch((error: unknown) => {
      throw new Error(`${rejection} rejection did not stop the hub: ${error instanceof Error ? error.message : "unknown"}`);
    });
    if (run.child.exitCode === 0) throw new Error(`${rejection} unexpectedly kept the hub usable`);
    const output = run.output();
    if (output.includes(keyEvidence) || output.includes(profiles[0]!.subject)) throw new Error(`${rejection} leaked private bootstrap material`);
  } finally {
    await finishLocalHub(run);
  }
}

async function runSecureLocalSmoke(repoRoot: string, root: string): Promise<void> {
  const profiles: readonly LocalProfile[] = [
    { profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] },
    { profileId: "administrator", subject: "local-administrator-01", displayName: "Local Administrator", allowedClientClasses: ["native"] },
  ];
  const run = await startLocalHub(repoRoot, root, profiles, { capture: true, adminSubjects: ["semio.local.bootstrap/v1:local-administrator-01"], isolatedSecuritySmoke: true });
  const capabilities: string[] = [];
  const keyEvidence = run.channelKey.toString("hex");
  try {
    const nativeEnvelope = await issueLocalCredential(run, "developer", "native", 2);
    const mcpEnvelope = await issueLocalCredential(run, "developer", "mcp", 3);
    const adminEnvelope = await issueLocalCredential(run, "administrator", "native", 4);
    capabilities.push(nativeEnvelope.capability, mcpEnvelope.capability, adminEnvelope.capability);
    let crossClassRejected = false;
    try {
      await deliverMcpCredentialEnvelope(process.execPath, [], structuredClone(nativeEnvelope));
    } catch {
      crossClassRejected = true;
    }
    if (!crossClassRejected) throw new Error("native envelope crossed into MCP delivery");
    await proveCredentialEnvelopeDelivery(nativeEnvelope, "native");
    await proveCredentialEnvelopeDelivery(mcpEnvelope, "mcp");
    const readiness = await waitForReadiness(run, true);
    if (readiness.status !== "not-ready" || readiness.authentication.bootstrapReady !== true || readiness.artifactAuthority?.ready !== false) throw new Error("security smoke did not observe truthful partial readiness");
    const readinessJson = JSON.stringify(readiness);
    if (capabilities.some(capability => readinessJson.includes(capability)) || profiles.some(profile => readinessJson.includes(profile.subject)) || readinessJson.includes("sessionKind") || readinessJson.includes("authorizationGeneration")) throw new Error("readiness leaked private bootstrap material");
    for (let index = 0; index < capabilities.length; index++) {
      const me = await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers: { authorization: `Bearer ${capabilities[index]}` } });
      if (!me.ok) throw new Error("issued local session did not validate");
      const session = await me.json() as Record<string, unknown>;
      const envelope = [nativeEnvelope, mcpEnvelope, adminEnvelope][index]!;
      if (session.sessionKind !== "development-local" || session.authorizationGeneration !== envelope.authorizationGeneration) throw new Error("issued local session metadata did not match its signed envelope");
    }
    const developerAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[0]}` } });
    const administratorAdmin = await fetch(`http://127.0.0.1:${run.port}/admin/api/overview`, { headers: { authorization: `Bearer ${capabilities[2]}` } });
    if (developerAdmin.status !== 401 || !administratorAdmin.ok) throw new Error("local administrator subject policy was not enforced");
    if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions`, { method: "POST" })).status !== 404) throw new Error("public session mint route exists");
    for (const capability of capabilities) {
      const headers = { authorization: `Bearer ${capability}` };
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { method: "DELETE", headers })).status !== 204) throw new Error("local session revoke failed");
      if ((await fetch(`http://127.0.0.1:${run.port}/auth/sessions/me`, { headers })).status !== 401) throw new Error("revoked local session remained usable");
    }
    const replayNow = Date.now();
    await writeLocalFrame(run.pipe, authenticatedFrame(run.channelKey, {
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
    }));
    await waitForChildExit(run.child);
    const output = run.output();
    if (capabilities.some(capability => output.includes(capability)) || output.includes(keyEvidence) || profiles.some(profile => output.includes(profile.subject))) throw new Error("hub logs leaked private bootstrap material");
  } finally {
    capabilities.fill("");
    await finishLocalHub(run);
  }
  for (const rejection of ["wrong-hmac", "wrong-class", "wrong-profile", "expired", "cross-run", "timeout", "eof"] as const) {
    await proveRejectedBootstrap(repoRoot, root, rejection).catch((error: unknown) => {
      throw new Error(`${rejection} rejection failed: ${error instanceof Error ? error.message : "unknown"}`);
    });
  }
  console.log("secure-local-smoke: truthful partial readiness, native/MCP delivery, admin isolation, issue/validate/revoke, replay/HMAC/class/profile/run/expiry/timeout/EOF rejection, redaction, and absent public mint passed");
}

/** 🛡️ `os-hub-admin`'s build MUST land before cargo ever runs `main()` for real — `HubState.
 * admin_dir` (§C0 `OS_HUB_ADMIN_DIR`, else the compile-time default) is read at hub STARTUP, not
 * build time, so this is a runtime prerequisite, not a Cargo `build.rs` concern. Zero-touch/
 * cross-platform: `bun nx run os-hub-admin:build` is the same command every OS/devcontainer already
 * runs for every other nx target here. */
function buildAdminSpa(repoRoot: string): void {
  runCmd("bun", ["nx", "run", "os-hub-admin:build"], { cwd: repoRoot, ...orchestratorBudgetOpts() });
}

class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class BuildScript extends BundleScript {
  run(): void {
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--release", "--manifest-path", "Cargo.toml"], this.root);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    // 🎛️ `--all-features` so a plain `bun ./📜️script.ts test` covers the full old 5-crate baseline
    // (directory core + sqlite/postgres/neo4j backends + the bin's own WS/REST suite) in one run —
    // `postgres`'s own tests still need a live Docker daemon regardless of this flag (pre-existing,
    // not a regression from the merge).
    runCargoTestBudgeted(["semio-hub"], this.repoRoot, ["--all-features", ...rest]);
  }
}

class ArtifactCasCheckScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "--manifest-path", "Cargo.toml", "--all-features", "--lib", "artifact_chunk_cas", ...segments], this.root);
    runCargo(["check", "--manifest-path", "Cargo.toml", "--all-features", "--bin", "os-hub"], this.root);
  }
}

/** 🔗️ `runCargo`'s `env` arg replaces `process.env` wholesale (see `runCmdInternal`'s
 * `opts.env ?? process.env`), so this inherits the full process env and only defaults the port —
 * otherwise the launcher's `OS_HUB_PORT`/`OS_HUB_DATA` (and `PATH`) would be silently dropped. */
class DevScript extends BundleScript {
  async run(): Promise<void> {
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);
    const profiles: readonly LocalProfile[] = [{ profileId: "developer", subject: "local-developer-01", displayName: "Local Developer", allowedClientClasses: ["native", "mcp"] }];
    const run = await startLocalHub(this.repoRoot, this.root, profiles, {
      port: Number(process.env[OS_HUB_PORT_ENV] ?? OS_HUB_PORT),
      dataDir: process.env.OS_HUB_DATA,
    });
    const stop = (): void => { void finishLocalHub(run); };
    process.once("SIGINT", stop);
    process.once("SIGTERM", stop);
    try {
      await waitForReadiness(run);
      console.log(`[INFO] secure local hub ready at http://127.0.0.1:${run.port}`);
      await new Promise<void>((resolveExit, rejectExit) => {
        run.child.once("exit", (code) => code === 0 ? resolveExit() : rejectExit(new Error(`hub child exited with status ${code}`)));
      });
    } finally {
      process.off("SIGINT", stop);
      process.off("SIGTERM", stop);
      await finishLocalHub(run);
    }
  }
}

class SecureLocalSmokeScript extends BundleScript {
  async run(): Promise<void> {
    buildAdminSpa(this.repoRoot);
    runCargo(["build", "--manifest-path", "Cargo.toml"], this.root);
    await runSecureLocalSmoke(this.repoRoot, this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("build", BuildScript).register("test", TestScript).register("artifact-cas-check", ArtifactCasCheckScript).register("dev", DevScript).register("secure-local-smoke", SecureLocalSmokeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
