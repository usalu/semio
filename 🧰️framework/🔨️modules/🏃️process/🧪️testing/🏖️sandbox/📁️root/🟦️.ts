import { randomBytes } from "node:crypto";
import { existsSync, mkdirSync, readFileSync, rmSync, rmdirSync, utimesSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { detachDiskImagesUnderV1 } from "../🖼️images/🟦️.ts";
import { readTestSandboxProtocolV1, testSandboxParentV1, type TestSandboxProtocolV1 } from "../📐️layout/🟦️.ts";
import { releaseTestPortV1, reserveTestPortV1 } from "../🔌️ports/🟦️.ts";
import { summarizeGuardReportV1 } from "../🛡️guard/🟦️.ts";
import { reclaimAbandonedTestSandboxesV1 } from "../♻️reclaim/🟦️.ts";
import { agentSessionAddressV1, currentAgentSessionV1 } from "../../../🪪️agent-sessions/🟦️.ts";
import { runtimeTestApiV1 } from "../../../../../📦️packages/🟦️typescript/⚙️runtime/🚀️process/🟦️.ts";

/** 🏖️ Names the guard mode a sandbox hands to its children. */
export type TestGuardModeV1 = "off" | "report" | "enforce";

/** 🆕️ Describes how one sandbox is created. */
export type TestSandboxOptionsV1 = Readonly<{ route: string; parent?: string; checkout?: string; guard?: TestGuardModeV1; heartbeat?: boolean; reclaim?: boolean; session?: Readonly<{ id: string; worktree: string; resources: Readonly<{ tests: Readonly<{ root: string }> }> }> }>;

/** ▶️ One per-run test environment: a private temp root, artifact, image and cache directories, a port lease set and the environment children inherit. */
export type TestSandboxV1 = Readonly<{
  id: string; root: string; artifacts: string; tmp: string; images: string; cache: string; shared: string;
  environment: Readonly<Record<string, string>>;
  reservePort(signal?: AbortSignal): Promise<number>;
  dispose(): void;
}>;

const live = new Map<string, TestSandboxV1>();
let exitHook = false, processSandbox: TestSandboxV1 | undefined;

function disposeAll(): void { for (const sandbox of [...live.values()]) sandbox.dispose(); }

/** 🏁️ Registers a callback for the end of the current test file, because some test runners end their process without running exit handlers. */
function atTestEnd(callback: () => void): void {
  try { runtimeTestApiV1()?.afterAll(callback); } catch {}
}

/** 📁️ Creates a unique sandbox under the root of the current agent session below the short host-wide parent (the session decides the root, so concurrent agents never share a directory and a gone session takes its sandboxes with it); it is detached, released and removed on disposal and when the process exits, and abandoned siblings of dead processes are reclaimed first. */
export function createTestSandboxV1(options: TestSandboxOptionsV1, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): TestSandboxV1 {
  if (!/^[a-z0-9][a-z0-9-]{0,127}$/u.test(options.route)) throw Error("Invalid test sandbox route");
  const parent = testSandboxParentV1(options.parent, declared), names = declared.environment, session = options.session ?? currentAgentSessionV1(), home = join(parent, session.resources.tests.root);
  mkdirSync(home, { recursive: true, mode: 0o700 });
  if (options.reclaim !== false) { try { reclaimAbandonedTestSandboxesV1({ parent: options.parent }, declared); } catch {} }
  let id = "", root = "";
  for (let attempt = 0; attempt < 16; attempt++) {
    id = randomBytes(4).toString("hex"); root = join(home, id);
    try { mkdirSync(root, { mode: 0o700 }); break; } catch (error) {
      const code = (error as NodeJS.ErrnoException).code;
      if (code === "ENOENT" && attempt < 15) { mkdirSync(home, { recursive: true, mode: 0o700 }); continue; }
      if (code !== "EEXIST" || attempt === 15) throw error;
    }
  }
  if (root.length > declared.layout.maximumRootLength) { rmSync(root, { recursive: true, force: true }); throw Error("Short test sandbox root required"); }
  for (const name of declared.layout.directories) mkdirSync(join(root, name), { recursive: true, mode: 0o700 });
  const artifacts = join(root, "artifacts", options.route), owner = join(root, declared.layout.owner), guard = options.guard ?? declared.guard.defaultMode as TestGuardModeV1;
  mkdirSync(artifacts, { recursive: true, mode: 0o700 });
  writeFileSync(owner, JSON.stringify({ version: 1, pid: process.pid, route: options.route, startedAt: Date.now(), checkout: options.checkout ?? null, session: agentSessionAddressV1(session) }));
  const tmp = join(root, "tmp"), environment: Record<string, string> = {
    [names.sandbox]: root, [names.artifacts]: artifacts, [names.images]: join(root, "images"), [names.cache]: join(root, "cache"),
    [names.guard]: guard, [names.report]: join(root, "guard.jsonl"), TMPDIR: tmp, TMP: tmp, TEMP: tmp,
  };
  if (options.checkout) environment[names.checkout] = resolve(options.checkout);
  const ports = new Set<number>();
  let timer: ReturnType<typeof setInterval> | undefined, disposed = false;
  if (options.heartbeat !== false) {
    timer = setInterval(() => { try { const now = Date.now() / 1000; utimesSync(owner, now, now); } catch {} }, declared.layout.heartbeatMilliseconds);
    timer.unref();
  }
  const sandbox: TestSandboxV1 = Object.freeze({
    id, root, artifacts, tmp, images: join(root, "images"), cache: join(root, "cache"), shared: parent,
    environment: Object.freeze(environment),
    async reservePort(signal?: AbortSignal): Promise<number> {
      const port = await reserveTestPortV1(parent, id, signal, declared);
      ports.add(port);
      return port;
    },
    dispose(): void {
      if (disposed) return;
      disposed = true;
      if (timer) clearInterval(timer);
      live.delete(id);
      try { for (const line of summarizeGuardReportV1(readFileSync(join(root, "guard.jsonl"), "utf8"))) console.error(line); } catch {}
      try { for (const port of ports) releaseTestPortV1(parent, port, id, declared); } catch {}
      try { detachDiskImagesUnderV1(root); rmSync(root, { recursive: true, force: true, maxRetries: 3 }); } catch (error) { console.error(`[leak] test sandbox ${root} not released: ${String(error)}`); }
      try { rmdirSync(home); } catch {}
    },
  });
  live.set(id, sandbox);
  atTestEnd(() => sandbox.dispose());
  if (!exitHook) { exitHook = true; process.on("exit", disposeAll); }
  return sandbox;
}

/** 🚨️ Runs a body inside a fresh sandbox and disposes it afterwards, also when the body fails or is cancelled. */
export async function withTestSandboxV1<T>(options: TestSandboxOptionsV1, body: (sandbox: TestSandboxV1) => Promise<T> | T): Promise<T> {
  const sandbox = createTestSandboxV1(options);
  try { return await body(sandbox); } finally { sandbox.dispose(); }
}

/** 🧬️ Tells whether an environment already belongs to a live sandbox, so nested runs share it instead of nesting another. */
export function adoptedTestSandboxRootV1(env: NodeJS.ProcessEnv = process.env, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): string | undefined {
  const root = env[declared.environment.sandbox];
  return root && existsSync(join(root, declared.layout.owner)) ? root : undefined;
}

/** ↩️ Returns the environment a child of this process must inherit: the adopted sandbox environment of a nested run, else a new per-run sandbox bound to the caller until it disposes. */
export function testSandboxEnvironmentV1(base: NodeJS.ProcessEnv, options: TestSandboxOptionsV1, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): Readonly<{ environment: NodeJS.ProcessEnv; release(): void }> {
  const adopted = adoptedTestSandboxRootV1(base, declared);
  if (adopted) {
    const environment: NodeJS.ProcessEnv = { ...base }, key = declared.environment.artifacts;
    if (!environment[key]) { const artifacts = join(adopted, "artifacts", options.route); mkdirSync(artifacts, { recursive: true }); environment[key] = artifacts; }
    return { environment, release() {} };
  }
  const sandbox = createTestSandboxV1(options, declared);
  return { environment: { ...base, ...sandbox.environment }, release: () => sandbox.dispose() };
}

/** 🧪️ Returns the scratch directory of this process: the caller-owned artifact directory when configured, else the artifact directory of one process-wide sandbox. */
export function testScratchRootV1(route = "scratch", env: NodeJS.ProcessEnv = process.env, declared: TestSandboxProtocolV1 = readTestSandboxProtocolV1()): string {
  const configured = env[declared.environment.artifacts]?.trim();
  if (configured) { mkdirSync(configured, { recursive: true }); return resolve(configured); }
  if (!processSandbox || !live.has(processSandbox.id)) processSandbox = createTestSandboxV1({ route });
  return processSandbox.artifacts;
}

/** 🔎️ Lists the live sandboxes of this process, which tests use to prove nothing leaks. */
export function liveTestSandboxesV1(): readonly TestSandboxV1[] { return [...live.values()]; }
