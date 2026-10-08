/**
 * 🧭️ Journey of the command line against an isolated daemon (`SEMIO_DASHBOARD_INSTANCE`): one handle per task
 * everywhere, commands and groups as handles, the ready address as the last line. The daemon runs in the
 * foreground of this test (`daemon serve`), so nothing detached is needed and the developer's daemon is untouched.
 *
 * @see ./🥒️.feature
 */
import { afterAll, beforeAll, expect, test } from "bun:test";
import { existsSync, mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";

const dashboard = resolve(import.meta.dir, "../..");
const repo = resolve(dashboard, "../../../../..");

function binary(): string {
  const name = process.platform === "win32" ? "semio.exe" : "semio";
  const candidates = [process.env.SEMIO_DASHBOARD_BIN, process.env.CARGO_TARGET_DIR && join(process.env.CARGO_TARGET_DIR, "debug", name), join(repo, ".🧬semio/🦑️repo/⚡️cache/cargo/target/debug", name)].filter((path): path is string => Boolean(path));
  const found = candidates.find((path) => existsSync(path));
  if (!found) throw new Error(`no debug \`semio\` binary; build it or set SEMIO_DASHBOARD_BIN. Looked at: ${candidates.join(", ")}`);
  return found;
}

let root = "";
let port = 0;
let daemon: ReturnType<typeof Bun.spawn> | undefined;
const instance = `cli-${process.pid}-${Date.now()}`;
const environment = { ...process.env, SEMIO_DASHBOARD_INSTANCE: instance } as Record<string, string>;

function semio(args: string[]): { code: number; out: string; err: string; lines: string[] } {
  const run = Bun.spawnSync([binary(), ...args, "--root", root], { cwd: root, env: environment, stdout: "pipe", stderr: "pipe" });
  const out = run.stdout.toString();
  return { code: run.exitCode, out, err: run.stderr.toString(), lines: out.split(/\r?\n/u).filter((line) => line !== "") };
}
const handleOf = (line: string): string => line.split("\t")[0]!;

beforeAll(async () => {
  const probe = Bun.serve({ port: 0, fetch: () => new Response("") });
  port = probe.port!;
  probe.stop(true);
  root = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? tmpdir(), "semio-cli-journey-"));
  mkdirSync(root, { recursive: true });
  writeFileSync(join(root, "package.json"), JSON.stringify({ name: "journey" }));
  const server = "const s=Bun.serve({port:Number(process.env.JOURNEY_PORT),fetch:()=>new Response('x')});console.log('listening on http://127.0.0.1:'+s.port);setInterval(()=>{},1000)";
  const tools = [
    { id: "serve", verb: "dev", continuous: true, ready: { port, portEnv: "JOURNEY_PORT", path: "/ok" }, command: ["bun", "-e", server] },
    { id: "hello", command: ["bun", "-e", "console.log('hello from the journey')"] },
  ];
  writeFileSync(join(root, "📋️project.json"), JSON.stringify({ name: "w", targets: {}, metadata: { semio: { dashboard: { tools } } } }));
  daemon = Bun.spawn([binary(), "daemon", "serve", "--root", root], { cwd: root, env: environment, stdout: "ignore", stderr: "ignore" });
  for (let attempt = 0; attempt < 100; attempt++) {
    if (semio(["daemon", "status"]).out.includes("daemon pid")) return;
    await Bun.sleep(100);
  }
  throw new Error("the isolated daemon did not come up");
});

afterAll(async () => {
  semio(["daemon", "stop"]);
  await Promise.race([daemon?.exited, Bun.sleep(5000)]);
  daemon?.kill();
  await Bun.sleep(300);
  rmSync(root, { recursive: true, force: true, maxRetries: 30, retryDelay: 200 });
}, 60000);

test("run, tasks, logs, open and stop all use the one handle a task is printed with, or its command or group", () => {
  const url = `http://127.0.0.1:${port}/ok`;
  const first = semio(["run", "tool:w/serve", "--detach", "--wait-ready"]);
  expect(first.code, first.err).toBe(0);
  expect(first.lines.at(-1), "the ready address is the last line, alone").toBe(url);
  const handle = handleOf(first.lines[0]!);
  expect(handle).toMatch(/^group-.+\.0$/u);
  expect(first.lines.filter((line) => line.includes("\t")).map(handleOf)).toEqual([handle]);

  const listed = semio(["tasks"]);
  expect(listed.lines.filter((line) => handleOf(line) === handle), "tasks prints the same handle").toHaveLength(1);
  const json = JSON.parse(semio(["tasks", "--json"]).out) as { session: string; status: string; readyUrl: string | null; commandId: string }[];
  expect(json.find((task) => task.session === handle)).toMatchObject({ status: "running", readyUrl: url, commandId: "tool:w/serve" });
  expect(json.some((task) => "place" in task), "no unstable position is printed").toBe(false);

  const again = semio(["run", "tool:w/serve", "--detach", "--wait-ready"]);
  expect(handleOf(again.lines[0]!), "a running task is reused under the same handle").toBe(handle);
  expect(again.lines.at(-1)).toBe(url);

  for (const needle of [handle, "tool:w/serve", handle.replace(/\.0$/u, "")]) {
    const logs = semio(["logs", needle]);
    expect(logs.code, `${needle}: ${logs.err}`).toBe(0);
    expect(logs.out, needle).toContain("listening on");
    const open = semio(["open", needle, "--print"]);
    expect({ needle, out: open.out.trim() }).toEqual({ needle, out: url });
  }
  expect(semio(["logs", "1"]).code, "a bare number is no handle").toBe(2);
  expect(semio(["logs", "1"]).err).toContain("no task matches");

  const stopped = semio(["stop", "tool:w/serve"]);
  expect(stopped.code, stopped.err).toBe(0);
  expect(handleOf(stopped.lines[0]!)).toBe(handle);
  expect(stopped.lines[0]).toContain("exited");
  const nothing = semio(["stop", "tool:w/serve"]);
  expect(nothing.code).toBe(2);
  expect(nothing.err).toContain("has no live task to stop");

  const second = semio(["run", "tool:w/serve", "--detach", "--wait-ready"]);
  const next = handleOf(second.lines[0]!);
  expect(next).not.toBe(handle);
  expect(semio(["logs", handle]).out, "the ended session keeps its log under its handle").toContain("listening on");
  const byHandle = semio(["stop", next]);
  expect(byHandle.code, byHandle.err).toBe(0);
  expect(byHandle.lines[0]).toContain("exited");

  const third = semio(["run", "tool:w/serve", "--detach", "--wait-ready"]);
  const group = handleOf(third.lines[0]!).replace(/\.0$/u, "");
  const byGroup = semio(["stop", group]);
  expect(byGroup.code, byGroup.err).toBe(0);
  expect(byGroup.lines.map((line) => line.split("\t")[1])).toEqual(["exited"]);
}, 240000);

test("a finished task is run attached to its exit code and stays addressable by command", () => {
  const attached = semio(["run", "tool:w/hello"]);
  expect(attached.code, attached.err).toBe(0);
  expect(attached.out).toContain("hello from the journey");
  const logs = semio(["logs", "tool:w/hello"]);
  expect(logs.out).toContain("hello from the journey");
  const open = semio(["open", "tool:w/hello", "--print"]);
  expect(open.code).toBe(2);
  expect(open.err).toContain("has no ready address");
  expect(semio(["restart", "tool:w/hello"]).code, "an ended task may be restarted by its command").toBe(0);
  expect(semio(["tasks"]).lines.filter((line) => line.includes("hello")).length).toBeGreaterThan(0);
}, 120000);
