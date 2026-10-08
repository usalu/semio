/**
 * 🏋️ Load of the control plane (feature: ./🥒️.feature, scenarios tagged @cli): the documented 128 retained sessions,
 * the search over fifty thousand commands and a ten megabyte output burst, with the real binary and daemon.
 * The @pty scenarios live in ./🦀️.rs.
 */
import { afterAll, describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { Workspace, plain, pool, slug, sleep, type Task } from "../🧰️support/🟦️.ts";

const opened: Workspace[] = [];
afterAll(async () => { await Promise.all(opened.map((ws) => ws.close())); });

/** 🔢️ Processes whose command line carries the idle program, so leaks after a daemon stop are visible. */
async function idleProcesses(): Promise<number> {
  const marker = "setInterval(() => {}, 3600000)";
  const command = process.platform === "win32" ? ["powershell", "-NoProfile", "-Command", `@(Get-CimInstance Win32_Process | Where-Object { $_.CommandLine -like '*${marker}*' -and $_.Name -eq 'bun.exe' }).Count`] : ["sh", "-c", `pgrep -fc -- '${marker}' || true`];
  const child = Bun.spawn(command, { stdout: "pipe", stderr: "ignore" });
  const text = await new Response(child.stdout).text();
  await child.exited;
  return Number(text.trim()) || 0;
}

describe("cli load", () => {
  test("One hundred and twenty-eight sessions run side by side and the next one is handled", async () => {
    const ws = await Workspace.open("sessions");
    opened.push(ws);
    const baseline = await idleProcesses();
    const started = Date.now();
    const numbers = Array.from({ length: 128 }, (_, index) => index);
    const failures: string[] = [];
    await ws.ok(["run", "tool:journey-fixture/idle", "--param", "slot=0", "--detach"]);
    await pool(numbers.slice(1), 8, async (index) => {
      const run = await ws.semio(["run", "tool:journey-fixture/idle", "--param", `slot=${index}`, "--detach"], { timeoutMs: 120_000 });
      if (run.code !== 0) failures.push(`session ${index}: exit ${run.code} ${run.stderr.trim()}`);
    });
    console.log(`[load] 128 sessions started in ${Date.now() - started} ms`);
    expect(failures, failures.slice(0, 10).join("\n")).toEqual([]);
    const listed = Date.now();
    const tasks = await ws.tasks();
    const answered = Date.now() - listed;
    expect(tasks.filter((task) => task.status === "running")).toHaveLength(128);
    expect(answered, "listing 128 tasks took too long").toBeLessThan(2_000);
    expect(new Set(tasks.map((task) => task.session)).size).toBe(128);
    expect(new Set(tasks.map((task) => task.pid)).size).toBe(128);
    const extra = await ws.semio(["run", "tool:journey-fixture/idle", "--param", "slot=128", "--detach"], { timeoutMs: 120_000 });
    const after: Task[] = await ws.tasks();
    if (extra.code === 0) expect(after.filter((task) => task.status === "running").length, "a 129th running session was admitted").toBeLessThanOrEqual(128);
    else expect(`${extra.stderr}${extra.stdout}`, "the refusal does not name the limit").toMatch(/128|limit|too many/i);
    expect((await ws.semio(["daemon", "status"])).code).toBe(0);
    await ws.ok(["daemon", "stop"]);
    for (let attempt = 0; attempt < 40 && (await idleProcesses()) > baseline; attempt++) await sleep(500);
    expect(await idleProcesses(), "idle processes survived the daemon stop").toBeLessThanOrEqual(baseline);
  }, 900_000);

  test("Searching fifty thousand commands answers within the latency budget", async () => {
    const ws = await Workspace.open("search", { bulkTools: 50_000 });
    opened.push(ws);
    const budget = Number(process.env.SEMIO_SEARCH_BUDGET_MS ?? 5_000);
    const first = Date.now();
    const check = await ws.ok(["commands", "--check"], { timeoutMs: 300_000 });
    console.log(`[load] check of ${/(\d+) commands/.exec(check)?.[1]} commands in ${Date.now() - first} ms`);
    expect(check).toMatch(/5\d{4} commands, 0 problems/);
    const timings: Record<string, number> = {};
    for (const [label, words, hits] of [["word", ["bulk", "build"], 12_500], ["rare prefix", ["bulk-049999"], 1], ["nothing", ["zzzz-nothing"], 0]] as const) {
      const began = Date.now();
      const out = await ws.ok(["commands", ...words, "--json"]);
      timings[label] = Date.now() - began;
      const found = JSON.parse(out) as { id: string }[];
      expect(found.length, label).toBeGreaterThanOrEqual(hits === 12_500 ? 12_500 : hits);
      if (hits === 1) expect(found[0]!.id).toBe("tool:bulk/bulk-049999");
      expect(timings[label], `${label} search took ${timings[label]} ms`).toBeLessThan(budget);
    }
    console.log(`[load] search timings ${JSON.stringify(timings)} ms (budget ${budget})`);
  }, 900_000);

  test("A ten megabyte output burst reaches an attached client without a disconnect", async () => {
    const ws = await Workspace.open("burst");
    opened.push(ws);
    const bytes = 10 * 1024 * 1024;
    const began = Date.now();
    const burst = ws.semio(["run", "tool:journey-fixture/burst", "--param", `bytes=${bytes}`], { timeoutMs: 300_000 });
    await sleep(2_500);
    const answers: number[] = [];
    for (let attempt = 0; attempt < 5; attempt++) {
      const asked = Date.now();
      const listing = await ws.semio(["tasks", "--json"], { timeoutMs: 30_000 });
      if (listing.code === 0) answers.push(Date.now() - asked);
      await sleep(500);
    }
    const run = await burst;
    console.log(`[load] ${(run.stdout.length / 1048576).toFixed(1)} MiB through an attached client in ${Date.now() - began} ms; listing answered in ${JSON.stringify(answers)} ms`);
    expect(run.code, run.stderr).toBe(0);
    expect(run.stdout.length).toBeGreaterThanOrEqual(bytes);
    const done = /BURST-DONE (\d+)\s*$/.exec(plain(run.stdout));
    expect(done, "the done line is missing from the end of the stream").not.toBeNull();
    expect(Number(done![1])).toBeGreaterThanOrEqual(bytes);
    expect(answers.length, "the daemon stopped answering other clients during the burst").toBeGreaterThanOrEqual(1);
    for (const answer of answers) expect(answer).toBeLessThan(10_000);
    const task = (await ws.tasks()).find((candidate) => candidate.commandId === "tool:journey-fixture/burst")!;
    expect(task.status).toBe("exited");
    expect(task.code).toBe(0);
    const log = plain(await ws.ok(["logs", task.session], { timeoutMs: 120_000 }));
    expect(log).toMatch(/BURST-DONE \d+/);
  }, 900_000);
});

describe("traceability", () => {
  test("every @cli scenario of the feature has a test of the same title in this file and every @pty scenario a Rust test", () => {
    const lines = readFileSync(join(import.meta.dir, "🥒️.feature"), "utf8").split(/\r?\n/);
    const titled = (tag: string): string[] => lines.flatMap((line, index) => (lines[index - 1]?.trim() === tag ? [/Scenario:\s*(.+?)\s*$/.exec(line)?.[1]] : [])).filter((title): title is string => !!title);
    const source = readFileSync(join(import.meta.dir, "🟦️.ts"), "utf8");
    const rust = readFileSync(join(import.meta.dir, "🦀️.rs"), "utf8");
    for (const title of titled("@cli")) expect(source, title).toContain(`test(${JSON.stringify(title)}`);
    for (const title of titled("@pty")) expect(rust, title).toContain(`fn ${slug(title)}(`);
    expect(titled("@cli").length + titled("@pty").length).toBe(6);
  });
});
