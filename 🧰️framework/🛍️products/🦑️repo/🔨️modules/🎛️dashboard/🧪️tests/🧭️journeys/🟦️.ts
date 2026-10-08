/**
 * 🧭️ Non-interactive journeys (feature: ./🥒️.feature, scenarios tagged @cli): the real `semio` binary and a real
 * workspace daemon over the fixture workspace. Bun's `fetch` is the HTTP oracle of the servers the tasks start.
 *
 * @see ./🧰️support/🟦️.ts
 */
import { afterAll, describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Workspace, get, plain, scenarios, semio, sleep, slug, type Task } from "./🧰️support/🟦️.ts";

const feature = join(import.meta.dir, "🥒️.feature");
const opened: Workspace[] = [];
const workspace = async (name: string): Promise<Workspace> => { const opening = await Workspace.open(name); opened.push(opening); return opening; };
afterAll(async () => { await Promise.all(opened.map((ws) => ws.close())); });

const refused = async (url: string): Promise<boolean> => {
  for (let attempt = 0; attempt < 40; attempt++) {
    const reachable = await get(url).then(() => true, () => false);
    if (!reachable) return true;
    await sleep(250);
  }
  return false;
};

const logOf = async (ws: Workspace, task: string): Promise<string> => plain(await ws.ok(["logs", task]));
const waitFor = async (what: string, read: () => Promise<string>, needle: string | RegExp, within = 20_000): Promise<string> => {
  const deadline = Date.now() + within;
  for (;;) {
    const text = await read();
    if (typeof needle === "string" ? text.includes(needle) : needle.test(text)) return text;
    if (Date.now() >= deadline) throw new Error(`${what}: ${String(needle)} not seen within ${within} ms in:\n${text}`);
    await sleep(250);
  }
};

describe("cli journeys", () => {
  test("A detached server prints its address once it is ready", async () => {
    const ws = await workspace("detached");
    const port = ws.portA;
    const out = await ws.ok(["run", "tool:journey-fixture/serve", "--param", `listen=${port}`, "--detach", "--wait-ready"]);
    const address = `http://127.0.0.1:${port}/health`;
    expect(out).toContain(address);
    const response = await get(address);
    expect(response.status).toBe(200);
    expect(response.body).toContain("journey-ok");
    const task = await ws.task((candidate) => candidate.commandId === "tool:journey-fixture/serve");
    expect(task.status).toBe("running");
    expect(task.readyUrl).toBe(address);
    expect(await waitFor("log", () => logOf(ws, task.session), "journey server listening")).toContain(`http://127.0.0.1:${port}`);
    await ws.ok(["stop", task.session]);
    const after = (await ws.tasks()).find((candidate) => candidate.session === task.session)!;
    expect(after.status).toBe("exited");
    expect(await refused(`http://127.0.0.1:${port}/health`)).toBe(true);
  }, 120_000);

  test("A compound starts its members in order and stops them together", async () => {
    const ws = await workspace("compound");
    const out = await ws.ok(["run", "compound:journey-fixture/pair", "--detach", "--wait-ready"]);
    const [a, b] = [`http://127.0.0.1:${ws.portA}/health`, `http://127.0.0.1:${ws.portB}/health`];
    expect(out).toContain(a);
    expect(out).toContain(b);
    expect(out.indexOf(a)).toBeLessThan(out.indexOf(b));
    for (const address of [a, b]) expect((await get(address)).status).toBe(200);
    const tasks = (await ws.tasks()).filter((task) => task.status === "running");
    expect(tasks).toHaveLength(2);
    expect(tasks[0]!.group).not.toBeNull();
    expect(tasks[0]!.group).toBe(tasks[1]!.group);
    await ws.ok(["stop", tasks[0]!.session, "--group"]);
    const after = await ws.tasks();
    expect(after.filter((task) => task.status === "running")).toHaveLength(0);
    for (const address of [a, b]) expect(await refused(address)).toBe(true);
  }, 120_000);

  test("A command that requires a service starts it first and waits for it", async () => {
    const ws = await workspace("requires");
    const run = await ws.semio(["run", "tool:journey-fixture/probe"], { timeoutMs: 90_000 });
    expect(run.code, run.stderr).toBe(0);
    expect(plain(run.stdout)).toContain("PROBE 200 journey-ok");
    const served = (await ws.tasks()).find((task) => task.commandId === "tool:journey-fixture/serve-a");
    expect(served?.status).toBe("running");
    expect(served?.readyUrl).toBe(`http://127.0.0.1:${ws.portA}/health`);
    const probe = (await ws.tasks()).find((task) => task.commandId === "tool:journey-fixture/probe")!;
    expect(probe.startedMs).toBeGreaterThanOrEqual(served!.startedMs);
    expect(probe.code).toBe(0);
  }, 120_000);

  test("Restarting a task keeps its environment and starts a new process", async () => {
    const ws = await workspace("restart");
    await ws.ok(["run", "tool:journey-fixture/ticker", "--env", "JOURNEY_MARK=alpha", "--detach"]);
    const first = await ws.task((task) => task.commandId === "tool:journey-fixture/ticker" && task.status === "running");
    await waitFor("first output", () => logOf(ws, first.session), "mark=alpha");
    await ws.ok(["restart", first.session]);
    const second = await ws.task((task) => task.commandId === "tool:journey-fixture/ticker" && task.status === "running" && task.pid !== first.pid);
    expect(second.pid).not.toBe(first.pid);
    const text = await waitFor("restarted output", () => logOf(ws, second.session), /TICKER-START pid=\d+ mark=alpha/);
    expect(text).toContain("mark=alpha");
    expect(text).not.toContain("mark=unset");
    await ws.ok(["stop", second.session]);
  }, 120_000);

  test("An attached run exits with the exit code of its task", async () => {
    const ws = await workspace("exit-code");
    const run = await ws.semio(["run", "tool:journey-fixture/fail", "--param", "code=7"], { timeoutMs: 90_000 });
    expect(run.code).toBe(7);
    expect(plain(run.stdout)).toContain("EXITING with 7");
    const task = await ws.task((candidate) => candidate.commandId === "tool:journey-fixture/fail");
    expect(task.status).toBe("exited");
    expect(task.code).toBe(7);
    const defaulted = await ws.semio(["run", "tool:journey-fixture/fail"], { timeoutMs: 90_000 });
    expect(defaulted.code).toBe(3);
    const ok = await ws.semio(["run", "tool:journey-fixture/fail", "--param", "code=0"], { timeoutMs: 90_000 });
    expect(ok.code).toBe(0);
  }, 180_000);

  test("A detached task survives its client and its log replays from the start", async () => {
    const ws = await workspace("replay");
    await ws.ok(["run", "tool:journey-fixture/ticker", "--detach"]);
    const task = await ws.task((candidate) => candidate.commandId === "tool:journey-fixture/ticker" && candidate.status === "running");
    const first = await waitFor("ticks", () => logOf(ws, task.session), "TICK 3");
    expect(first).toContain("TICKER-START");
    const later = await waitFor("more ticks", () => logOf(ws, task.session), "TICK 6");
    expect(later.indexOf("TICKER-START")).toBeLessThan(later.indexOf("TICK 1 "));
    expect(later.length).toBeGreaterThan(first.length);
    expect((await ws.tasks()).find((candidate) => candidate.session === task.session)?.pid).toBe(task.pid);
    await ws.ok(["stop", task.session]);
  }, 120_000);

  test("Every verb refuses what it cannot do with a precise message and exit 2", async () => {
    const ws = await workspace("refusals");
    const unknown = await ws.semio(["run", "tool:journey-fixture/nope", "--dry-run"]);
    expect(unknown.code).toBe(2);
    expect(unknown.stderr).toContain("nope");
    const missing = await ws.semio(["run", "tool:journey-fixture/serve", "--dry-run"]);
    expect(missing.code).toBe(2);
    expect(missing.stderr).toContain("listen");
    const badParameter = await ws.semio(["run", "tool:journey-fixture/burst", "--param", "colour=red", "--dry-run"]);
    expect(badParameter.code).toBe(2);
    expect(badParameter.stderr).toContain("colour");
    const noTask = await ws.semio(["stop", "no-such-task"]);
    expect(noTask.code).toBe(2);
    const noLog = await ws.semio(["logs", "no-such-task"]);
    expect(noLog.code).toBe(2);
    const noArgument = await ws.semio(["run"]);
    expect(noArgument.code).toBe(2);
    expect(noArgument.stderr).toContain("usage");
  }, 120_000);

  test("Two workspaces never share a daemon", async () => {
    const [first, second] = [await workspace("isolated-a"), await workspace("isolated-b")];
    await first.ok(["run", "tool:journey-fixture/ticker", "--detach"]);
    await first.task((task) => task.commandId === "tool:journey-fixture/ticker" && task.status === "running");
    expect(await first.tasks()).toHaveLength(1);
    await second.ok(["run", "tool:journey-fixture/words", "--detach"]);
    const mine = await second.tasks();
    expect(mine).toHaveLength(1);
    expect(mine[0]!.commandId).toBe("tool:journey-fixture/words");
    expect((await first.tasks()).map((task) => task.commandId)).toEqual(["tool:journey-fixture/ticker"]);
    await first.ok(["stop", (await first.tasks())[0]!.session]);
    await second.ok(["stop", (await second.tasks())[0]!.session]);
  }, 120_000);

  test("A resolved launch is stable and independent of the daemon", async () => {
    const ws = await workspace("dry-run");
    const [one, two] = [await ws.ok(["run", "compound:journey-fixture/stack", "--dry-run"]), await ws.ok(["run", "compound:journey-fixture/stack", "--dry-run"])];
    expect(one).toBe(two);
    const launch = JSON.parse(one);
    expect(launch.stop).toBe("together");
    expect(launch.processes.map((process: { commandId: string }) => process.commandId)).toEqual(["tool:journey-fixture/serve-a", "tool:journey-fixture/ticker"]);
    expect(Object.fromEntries(launch.processes[1].env).JOURNEY_MARK).toBe("stacked");
    expect(launch.processes[0].ready.port).toBe(ws.portA);
    expect(await ws.tasks()).toEqual([]);
    const status = await ws.ok(["daemon", "status"]).catch((error: Error) => error.message);
    expect(status).toContain("not running");
  }, 120_000);
});

describe("traceability", () => {
  test("every @cli scenario of the feature has a test of the same title in this file", () => {
    const text = readFileSync(join(import.meta.dir, "🟦️.ts"), "utf8");
    const lines = readFileSync(feature, "utf8").split(/\r?\n/);
    const cli = lines.flatMap((line, index) => (lines[index - 1]?.trim() === "@cli" ? [/Scenario:\s*(.+?)\s*$/.exec(line)?.[1]] : [])).filter((title): title is string => !!title);
    expect(cli.length).toBeGreaterThan(5);
    for (const title of cli) expect(text, `no test titled ${JSON.stringify(title)}`).toContain(`test(${JSON.stringify(title)}`);
  });

  test("every @pty scenario of the feature has a Rust test named after it", () => {
    const rust = readFileSync(join(import.meta.dir, "🦀️.rs"), "utf8");
    const lines = readFileSync(feature, "utf8").split(/\r?\n/);
    const pty = lines.flatMap((line, index) => (lines[index - 1]?.trim() === "@pty" ? [/Scenario:\s*(.+?)\s*$/.exec(line)?.[1]] : [])).filter((title): title is string => !!title);
    expect(pty.length).toBeGreaterThan(3);
    for (const title of pty) expect(rust, `no Rust test for ${JSON.stringify(title)}`).toContain(`fn ${slug(title)}(`);
    expect(scenarios(feature).length).toBe(new Set(scenarios(feature)).size);
  });
});

export type { Task };
export { semio };
