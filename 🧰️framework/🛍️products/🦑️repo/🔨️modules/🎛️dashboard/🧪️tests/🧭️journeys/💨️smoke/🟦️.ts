/**
 * 💨️ Real-workspace smoke (feature: ./🥒️.feature): real Nx commands of this monorepo through a dashboard daemon of
 * their own. The developer's daemon must never be started, stopped or contacted, so the suite runs only when the
 * binary isolates daemons by `SEMIO_DASHBOARD_INSTANCE` (a name mixed into the workspace key and the cache
 * directory); the suite starts only the named instance and fails if an instance-less client reaches the same daemon.
 */
import { afterAll, describe, expect, test } from "bun:test";
import { get, plain, repository, semio, sleep } from "../🧰️support/🟦️.ts";

const instance = process.env.SEMIO_SMOKE_INSTANCE ?? "v1-smoke";
const env = { SEMIO_DASHBOARD_INSTANCE: instance, SEMIO_LOCALE: "en" };
const run = (args: string[], timeoutMs = 120_000) => semio(args, { cwd: repository, env, timeoutMs });

/** 🔬️ Starts the named instance and proves that it is not the daemon an instance-less client reaches. */
async function isolation(): Promise<string | undefined> {
  const started = await run(["daemon", "start"], 120_000);
  if (started.code !== 0) return `the instance daemon did not start: ${started.stderr}${started.stdout}`;
  const pid = (text: string): string | undefined => /daemon pid (\d+)/.exec(text)?.[1];
  const mine = pid((await run(["daemon", "status"], 30_000)).stdout);
  const shared = await semio(["daemon", "status"], { cwd: repository, env: { SEMIO_LOCALE: "en" }, timeoutMs: 30_000 });
  if (!mine) return "the instance daemon reports no pid";
  return pid(shared.stdout) === mine ? `the instance ${instance} answers as the daemon of the workspace itself (pid ${mine}): no isolation` : undefined;
}

afterAll(async () => { await run(["daemon", "stop"], 60_000).catch(() => undefined); await sleep(500); });

const refused = async (url: string): Promise<boolean> => {
  for (let attempt = 0; attempt < 60; attempt++) {
    if (!(await get(url).then(() => true, () => false))) return true;
    await sleep(500);
  }
  return false;
};

describe("real workspace smoke", () => {
  test("A real native build completes with a retained task log", async () => {
    expect(await isolation()).toBeUndefined();
    const command = "@semio-tech/ui-rs:build";
    const resolved = await run(["run", command, "--param", "cache=skip-local", "--dry-run"]);
    expect(resolved.code, resolved.stderr).toBe(0);
    expect(JSON.parse(resolved.stdout).processes[0].args).toContain("--skip-nx-cache");
    const built = await run(["run", command, "--param", "cache=skip-local"], 1_200_000);
    const output = plain(built.stdout);
    expect(built.code, `${built.stderr}\n${output.slice(-3000)}`).toBe(0);
    expect(output).toMatch(/Successfully ran target build/);
    const tasks = JSON.parse((await run(["tasks", "--json"])).stdout) as { session: string; commandId: string; status: string; code: number | null; startedMs: number }[];
    const task = tasks.filter((task) => task.commandId === command).sort((a, b) => b.startedMs - a.startedMs)[0]!;
    expect(task).toBeDefined();
    expect(task.status).toBe("exited");
    expect(task.code).toBe(0);
    const logs = await run(["logs", task.session]);
    expect(logs.code, logs.stderr).toBe(0);
    expect(plain(logs.stdout)).toMatch(/Successfully ran target build/);
    console.log(`[DEBUG] real native Nx build ${task.session} completed through ${instance}, exited 0, and retained its build log`);
  }, 1_300_000);

  test("A finite real Nx command runs to completion with exit code 0", async () => {
    expect(await isolation()).toBeUndefined();
    const result = await run(["run", "@semio-tech/repo-dashboard-rs:test", "--param", "dependencies", "--", "execution"], 1_200_000);
    const text = plain(result.stdout);
    expect(result.code, `${result.stderr}\n${text.slice(-3000)}`).toBe(0);
    expect(text).toMatch(/\b\d+ pass\b/);
    expect(text).not.toMatch(/\b[1-9]\d* fail\b/);
    console.log(`[DEBUG] real dashboard Nx test completed through instance ${instance}: ${text.slice(-1600)}`);
  }, 1_300_000);

  test("A real development server starts detached, answers and stops", async () => {
    expect(await isolation()).toBeUndefined();
    const busy = await get("http://127.0.0.1:6061/").then(() => true, () => false);
    if (busy) { console.log("[smoke] port 6061 is in use (a developer's quiz is running): scenario skipped"); return; }
    const started = await run(["run", "@teaching/architecture-quiz:dev", "--detach", "--wait-ready", "--timeout", "900"], 1_000_000);
    expect(started.code, `${started.stderr}\n${started.stdout}`).toBe(0);
    const address = /http:\/\/(?:127\.0\.0\.1|localhost):6061\S*/.exec(started.stdout)?.[0];
    expect(address, started.stdout).toBeDefined();
    const early = await get(address!).then(() => false, () => true);
    expect((await get(address!, 240)).status).toBe(200);
    const mine = async () => (JSON.parse((await run(["tasks", "--json"])).stdout) as { session: string; commandId: string; status: string; readyUrl: string | null; startedMs: number }[]).filter((task) => task.commandId === "@teaching/architecture-quiz:dev").sort((a, b) => b.startedMs - a.startedMs)[0]!;
    const task = await mine();
    expect(task.status).toBe("running");
    expect(task.readyUrl).toBe(address);
    const stopped = await run(["stop", task.session], 60_000);
    expect(stopped.code, stopped.stderr).toBe(0);
    expect((await mine()).status).toBe("exited");
    expect(await refused("http://127.0.0.1:6061/")).toBe(true);
    expect(early, `--wait-ready returned ${address} before the server accepted a connection`).toBe(false);
    console.log(`[DEBUG] real development server ${task.session} answered HTTP 200 at ${address}, stopped, and released port 6061`);
  }, 1_100_000);
});
