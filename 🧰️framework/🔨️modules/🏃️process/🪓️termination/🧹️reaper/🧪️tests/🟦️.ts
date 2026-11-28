import { afterAll, describe, expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { existsSync, readFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import { join, resolve } from "node:path";
import { parseReaperLineV1, reapWithParentV1, REAPER_POLICY_V1, reaperLineTextV1, stopRegisteredV1 } from "../🟦️.ts";

/** 🧹️ Reaper law: a parent that is killed by SIGKILL leaves no registered child behind (not the child, not what it started), a child that was unregistered is left alone, a child that ignores SIGTERM is killed after the declared grace, and the reaping is logged with victim and sender identity. Liveness and descendants are cross-checked with `ps` and `pgrep`, the line grammar with a Python regular expression.
 * @see ../🧬️schema/🔣️.json */
const parentScript = join(resolve(import.meta.dir), "👷️parent/📜️script.ts");

const alive = (pid: number): boolean => {
  try { process.kill(pid, 0); } catch { return false; }
  const state = spawnSync("ps", ["-o", "stat=", "-p", String(pid)], { encoding: "utf8" });
  return state.status === 0 && !state.stdout.trim().startsWith("Z");
};
const children = (pid: number): number[] => spawnSync("pgrep", ["-P", String(pid)], { encoding: "utf8" }).stdout.split("\n").filter(Boolean).map(Number);
const until = async (predicate: () => boolean, milliseconds: number): Promise<boolean> => {
  const deadline = Date.now() + milliseconds;
  while (!predicate()) { if (Date.now() > deadline) return false; await Bun.sleep(50); }
  return true;
};
const leftovers: number[] = [];
afterAll(() => { for (const pid of leftovers) try { process.kill(-pid, "SIGKILL"); } catch {} });

describe("reaper line grammar", () => {
  const lines = ["add 4711 1767225600000", "remove 4711", "stop 4711", "stop 4711 5", "stop", "add 4711", "remove 4711 5", "add 0 5", "add x 5", "kill 4711", "add 4711 1767225600000 extra", ""];
  test("agrees with the Python regular expression and round-trips", () => {
    const python = spawnSync("python3", ["-I", "-c", 'import json,re,sys\nrows=json.loads(sys.argv[1])\nprint(json.dumps([bool(re.fullmatch(r"(add|remove|stop) [0-9]{1,10}( [0-9]{1,16})?",r.strip())) and not (not r.startswith("add") and r.count(" ")>1) and not (r.startswith("add") and r.count(" ")<2) and int(r.split()[1])>=1 for r in rows]))', JSON.stringify(lines)], { encoding: "utf8" });
    expect(python.status, python.stderr).toBe(0);
    expect(lines.map((line) => parseReaperLineV1(line) !== undefined)).toEqual(JSON.parse(python.stdout));
    expect(parseReaperLineV1(reaperLineTextV1({ kind: "add", pid: 4711, spawnedMs: 1767225600000 }))).toEqual({ kind: "add", pid: 4711, spawnedMs: 1767225600000 });
    expect(parseReaperLineV1(reaperLineTextV1({ kind: "remove", pid: 4711 }))).toEqual({ kind: "remove", pid: 4711 });
    expect(parseReaperLineV1(reaperLineTextV1({ kind: "stop", pid: 4711 }))).toEqual({ kind: "stop", pid: 4711 });
  });
});

describe.skipIf(process.platform === "win32")("parent death", () => {
  test("SIGKILL of the parent reaps the registered groups, spares the unregistered one and honours the grace", async () => {
    const parent = spawn(process.execPath, [parentScript], { stdio: ["ignore", "pipe", "inherit"] });
    let text = "";
    parent.stdout!.on("data", (bytes: Buffer) => { text += bytes.toString("utf8"); });
    expect(await until(() => text.includes("\n"), 20_000), "the parent starts its children").toBe(true);
    const { parent: parentPid, pids } = JSON.parse(text.split("\n")[0]!) as { parent: number; pids: Record<"polite" | "stubborn" | "released", number> };
    leftovers.push(...Object.values(pids));
    await Bun.sleep(1500);
    const grandchildren = Object.fromEntries(Object.entries(pids).map(([kind, pid]) => [kind, children(pid)])) as Record<string, number[]>;
    for (const [kind, pid] of Object.entries(pids)) { expect(alive(pid), `${kind} runs`).toBe(true); expect(grandchildren[kind]!.length, `${kind} has a descendant`).toBeGreaterThan(0); }
    const killedAt = Date.now();
    process.kill(parentPid, "SIGKILL");
    const registered = [pids.polite, pids.stubborn, ...grandchildren.polite!, ...grandchildren.stubborn!];
    expect(await until(() => registered.every((pid) => !alive(pid)), REAPER_POLICY_V1.graceMilliseconds + 8000), "no registered process survives the reap interval").toBe(true);
    expect(Date.now() - killedAt, "the polite group went at once, the stubborn one after the grace").toBeLessThan(REAPER_POLICY_V1.graceMilliseconds + 8000);
    expect(alive(pids.released), "an unregistered child is not the sentinel's").toBe(true);
    expect(grandchildren.released!.every(alive)).toBe(true);
    const log = join(tmpdir(), REAPER_POLICY_V1.directoryName, `${parentPid}.log`);
    expect(existsSync(log)).toBe(true);
    const logged = readFileSync(log, "utf8");
    expect(logged).toContain(`Process ${pids.polite} received SIGTERM from owned execution reaper`);
    expect(logged).toContain(`Process ${pids.stubborn} received SIGKILL from owned execution reaper`);
    expect(logged).toContain(`the parent process ${parentPid} ended`);
    expect(logged).toContain("der Elternprozess");
    expect(logged).toContain("Victim created at ");
    expect(logged).not.toContain(`Process ${pids.released} received`);
    rmSync(log, { force: true });
    try { process.kill(-pids.released, "SIGKILL"); } catch {}
  }, 40_000);

  test("a parent that ends normally with a registered child reaps it too", async () => {
    const parent = spawn(process.execPath, [parentScript], { stdio: ["ignore", "pipe", "inherit"] });
    let text = "";
    parent.stdout!.on("data", (bytes: Buffer) => { text += bytes.toString("utf8"); });
    expect(await until(() => text.includes("\n"), 20_000)).toBe(true);
    const { parent: parentPid, pids } = JSON.parse(text.split("\n")[0]!) as { parent: number; pids: Record<string, number> };
    leftovers.push(...Object.values(pids));
    await Bun.sleep(1500);
    process.kill(parentPid, "SIGTERM");
    expect(await until(() => !alive(pids.polite!) && !alive(pids.stubborn!), REAPER_POLICY_V1.graceMilliseconds + 8000)).toBe(true);
    try { process.kill(-pids.released!, "SIGKILL"); } catch {}
    rmSync(join(tmpdir(), REAPER_POLICY_V1.directoryName, `${parentPid}.log`), { force: true });
  }, 40_000);
});

describe.skipIf(process.platform === "win32")("stop of one registered child", () => {
  test("`stop` ends the registered tree now, the parent and the other registered child live on", async () => {
    const options = { stdio: "ignore", detached: true } as const;
    const doomed = spawn("sh", ["-c", "sleep 300 & wait"], options), kept = spawn("sh", ["-c", "sleep 300 & wait"], options);
    leftovers.push(doomed.pid!, kept.pid!);
    const forgetDoomed = reapWithParentV1(doomed.pid), forgetKept = reapWithParentV1(kept.pid);
    expect(await until(() => children(doomed.pid!).length > 0 && children(kept.pid!).length > 0, 10_000)).toBe(true);
    await Bun.sleep(800);
    const grandchild = children(doomed.pid!)[0]!;
    stopRegisteredV1(doomed.pid);
    expect(await until(() => !alive(doomed.pid!) && !alive(grandchild), REAPER_POLICY_V1.graceMilliseconds + 8000), "the registered tree is gone").toBe(true);
    expect([alive(kept.pid!), children(kept.pid!).every(alive)]).toEqual([true, true]);
    forgetDoomed();
    forgetKept();
    try { process.kill(-kept.pid!, "SIGKILL"); } catch {}
  }, 40_000);

  test("a pid that was never registered is not stopped and starts no sentinel", () => {
    const strange = spawn("sleep", ["300"], { stdio: "ignore", detached: true });
    leftovers.push(strange.pid!);
    stopRegisteredV1(strange.pid, { parent: 1 });
    expect(alive(strange.pid!)).toBe(true);
    try { process.kill(-strange.pid!, "SIGKILL"); } catch {}
  });
});
