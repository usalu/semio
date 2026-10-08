/**
 * 🧰️ Support of the battle tests: the real `semio` binary, an isolated fixture workspace with its own daemon,
 * bounded process pools and plain-socket HTTP, shared by the coverage, journey, load and smoke suites.
 *
 * @see ../../../🧫️fixtures/🧭️journeys/🏗️workspace/📋️project.json
 */
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

export const dashboard = resolve(import.meta.dir, "../../..");
export const repository = resolve(dashboard, "../../../../..");
const executable = process.platform === "win32" ? "semio.exe" : "semio";

/** 🔎️ The binary under test: `SEMIO_TEST_CLI`, else the fleet debug build, else the installed dashboard. */
export function binary(): string {
  const candidates = [process.env.SEMIO_TEST_CLI, join(repository, ".🧬semio/🦑️repo/⚡️cache/cargo/target-fleet-v1/debug", executable)];
  const found = candidates.find((path) => path && existsSync(path));
  if (!found) throw new Error(`no semio binary: build it and set SEMIO_TEST_CLI (looked at ${candidates.join(", ")})`);
  return found;
}

export type Run = { code: number; stdout: string; stderr: string };

/** ▶️ Runs `semio args…` to its end. */
export async function semio(args: string[], options: { cwd: string; env?: Record<string, string>; timeoutMs?: number; stdin?: string }): Promise<Run> {
  const flags = args.slice(0, args.indexOf("--") < 0 ? args.length : args.indexOf("--"));
  if (resolve(options.cwd) === repository && (["run", "stop", "restart", "kill"].includes(args[0]!) || (args[0] === "daemon" && args[1] !== "status")) && !flags.includes("--dry-run") && !options.env?.SEMIO_DASHBOARD_INSTANCE) throw new Error(`refusing to run \`semio ${args.join(" ")}\` on the real workspace without --dry-run before \`--\`: that would start a daemon and real tasks`);
  const child = Bun.spawn([binary(), ...args], { cwd: options.cwd, env: { ...process.env, ...options.env }, stdin: options.stdin === undefined ? "ignore" : new Blob([options.stdin]), stdout: "pipe", stderr: "pipe" });
  const timer = setTimeout(() => child.kill(), options.timeoutMs ?? 120_000);
  const [stdout, stderr, code] = await Promise.all([new Response(child.stdout).text(), new Response(child.stderr).text(), child.exited]);
  clearTimeout(timer);
  return { code, stdout, stderr };
}

/** 🏊️ Runs `work` over `items` with at most `width` in flight; results keep the order of `items`. */
export async function pool<T, R>(items: readonly T[], width: number, work: (item: T, index: number) => Promise<R>): Promise<R[]> {
  const results = new Array<R>(items.length);
  let next = 0;
  await Promise.all(Array.from({ length: Math.min(width, items.length) }, async () => {
    for (let index = next++; index < items.length; index = next++) results[index] = await work(items[index]!, index);
  }));
  return results;
}

export async function freePort(): Promise<number> {
  return await new Promise((done, fail) => {
    const server = createServer();
    server.once("error", fail);
    server.listen(0, "127.0.0.1", () => { const { port } = server.address() as { port: number }; server.close(() => done(port)); });
  });
}

/** 🌐️ Fetches a URL over a plain socket request, so no HTTP client sits between the server and the assertion. */
export async function get(url: string, attempts = 1): Promise<{ status: number; body: string }> {
  let last = "";
  for (let attempt = 0; attempt < attempts; attempt++) {
    try {
      const response = await fetch(url, { signal: AbortSignal.timeout(5_000) });
      return { status: response.status, body: await response.text() };
    } catch (error) { last = String(error); await Bun.sleep(250); }
  }
  throw new Error(`GET ${url} failed: ${last}`);
}

export const sleep = Bun.sleep;

const PORT_A = "47101";
const PORT_B = "47102";

/** 🏗️ A copy of the fixture workspace with free ports and its own daemon; `close` stops that daemon and removes the copy. */
export class Workspace {
  private constructor(readonly root: string, readonly portA: number, readonly portB: number, private readonly runtime: string) {}

  static async open(name: string, options: { bulkTools?: number } = {}): Promise<Workspace> {
    const base = mkdtempSync(join(tmpdir(), `semio-battle-${name}-`));
    const root = join(base, "workspace");
    mkdirSync(join(root, ".git"), { recursive: true });
    const [portA, portB] = [await freePort(), await freePort()];
    const source = join(dashboard, "🧫️fixtures/🧭️journeys/🏗️workspace");
    for (const entry of readdirSync(source)) {
      const text = readFileSync(join(source, entry), "utf8").replaceAll(PORT_A, String(portA)).replaceAll(PORT_B, String(portB));
      writeFileSync(join(root, entry), text);
    }
    if (options.bulkTools) {
      mkdirSync(join(root, "bulk"));
      const tools = Array.from({ length: options.bulkTools }, (_, index) => ({ id: `bulk-${String(index).padStart(6, "0")}`, verb: ["dev", "build", "test", "check"][index % 4], command: ["bun", "--version"] }));
      writeFileSync(join(root, "bulk/📋️project.json"), JSON.stringify({ name: "bulk", metadata: { semio: { dashboard: { tools } } } }));
    }
    const runtime = join(base, "runtime");
    mkdirSync(runtime);
    return new Workspace(root, portA, portB, runtime);
  }

  get env(): Record<string, string> { return { SEMIO_DASHBOARD_RUNTIME_DIR: this.runtime, SEMIO_LOCALE: "en" }; }

  semio(args: string[], options: { env?: Record<string, string>; timeoutMs?: number } = {}): Promise<Run> {
    return semio(args, { cwd: this.root, env: { ...this.env, ...options.env }, timeoutMs: options.timeoutMs });
  }

  /** ✅️ `semio args…` that must exit 0; its stdout. */
  async ok(args: string[], options: { env?: Record<string, string>; timeoutMs?: number } = {}): Promise<string> {
    const run = await this.semio(args, options);
    if (run.code !== 0) throw new Error(`semio ${args.join(" ")} exited ${run.code}\nstdout:\n${run.stdout}\nstderr:\n${run.stderr}`);
    return run.stdout;
  }

  /** 📋️ The tasks the workspace daemon reports. */
  async tasks(): Promise<Task[]> { return JSON.parse(await this.ok(["tasks", "--json"])) as Task[]; }

  async task(match: (task: Task) => boolean, within = 30_000): Promise<Task> {
    const deadline = Date.now() + within;
    for (;;) {
      const found = (await this.tasks()).find(match);
      if (found) return found;
      if (Date.now() >= deadline) throw new Error(`no matching task within ${within} ms: ${JSON.stringify(await this.tasks())}`);
      await sleep(250);
    }
  }

  async close(): Promise<void> {
    await this.semio(["daemon", "stop"], { timeoutMs: 20_000 }).catch(() => undefined);
    await sleep(300);
    rmSync(join(this.root, ".."), { recursive: true, force: true, maxRetries: 5, retryDelay: 200 });
  }
}

export type Task = { place: number; session: string; commandId: string; label: string; status: string; pid: number | null; code: number | null; startedMs: number; endedMs: number | null; readyUrl: string | null; group: string | null; title: string | null };

/** 🧹️ Removes terminal control sequences, so assertions read the text a person reads. */
export function plain(text: string): string {
  return text.replace(/\u001b\[[0-9;?]*[ -/]*[@-~]/g, "").replace(/\u001b\][^\u0007\u001b]*(\u0007|\u001b\\)/g, "").replace(/\r/g, "");
}

/** 📄️ The text of the first `Scenario:` titles of a feature file, in order. */
export function scenarios(featurePath: string): string[] {
  return readFileSync(featurePath, "utf8").split(/\r?\n/).map((line) => /^\s*Scenario(?: Outline)?:\s*(.+?)\s*$/.exec(line)?.[1]).filter((title): title is string => title !== undefined);
}

export const slug = (title: string): string => title.toLowerCase().replace(/[^a-z0-9]+/g, "_").replace(/^_|_$/g, "");

export { cpSync };
