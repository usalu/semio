import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { join, resolve } from "node:path";
import { ownedKillRecorderV1, ownedRowsV1, readProcessTableV1, terminateOwnedProcessTreeGracefullyV1, verifiedOwnedRowsV1, type ProcessTableRowV1 } from "../../🟦️.ts";

/** 🛡️ Identity law of a stop: a signal goes only to a process whose creation time still matches the snapshot it was found in, a root of another creation time owns nothing, an orphan of a dead root is part of its tree, and a process group is signalled as a group only while every process in it is owned, so stopping one task never reaches an unrelated build.
 * @see ../../🧬️schema/🔣️.json */
const row = (pid: number, parent: number, group: number, startMs: number): ProcessTableRowV1 => ({ pid, parent, group, startMs });
const ownerRoot = resolve(import.meta.dir, "../..");

async function signalsOf(table: readonly ProcessTableRowV1[], fresh: readonly ProcessTableRowV1[], root: number, rootStartMs?: number): Promise<string[]> {
  const sent: string[] = [], original = process.kill;
  let reads = 0;
  process.kill = ((pid: number, signal?: string) => { if (typeof signal === "string") sent.push(`${pid}:${String(signal)}`); return true; }) as typeof process.kill;
  try {
    await terminateOwnedProcessTreeGracefullyV1(root, { rootStartMs, table: () => (reads++ === 0 ? table : fresh), alive: () => false, sleep: async () => {}, graceMilliseconds: 0, platform: "linux" });
  } finally { process.kill = original; }
  return sent;
}

describe("identity of a stop", () => {
  const table = [row(10, 1, 10, 1000), row(11, 10, 10, 2000), row(12, 11, 12, 3000), row(50, 1, 50, 4000)];

  test("the owned tree is the root, its descendants and the orphans that kept the group of the root", () => {
    const orphaned = [row(11, 1, 10, 2000), row(12, 11, 10, 3000), row(50, 1, 50, 4000)];
    expect(ownedRowsV1(10, 1000, () => orphaned)!.map((owned) => owned.pid).sort()).toEqual([11, 12].sort());
    expect(ownedRowsV1(10, 1000, () => table)!.map((owned) => [owned.pid, owned.depth]).sort()).toEqual([[10, 0], [11, 1], [12, 2]]);
  });

  test("a root whose pid now belongs to a process of another creation time owns nothing and no row is invented for a gone root", () => {
    expect(ownedRowsV1(10, 99_000, () => table)).toEqual([]);
    expect(ownedRowsV1(77, 5000, () => table)).toEqual([]);
    expect(ownedRowsV1(77, undefined, () => table)!.map((owned) => owned.pid)).toEqual([77]);
    expect(ownedRowsV1(77, 5000, () => undefined)).toBeUndefined();
  });

  test("a pid reused between the snapshot and the signal is dropped, a gone pid too", () => {
    const owned = ownedRowsV1(10, 1000, () => table)!;
    const fresh = [row(10, 1, 10, 1000), row(11, 10, 10, 999_000), row(50, 1, 50, 4000)];
    expect(verifiedOwnedRowsV1(owned, fresh).map((kept) => kept.pid)).toEqual([10]);
    expect(verifiedOwnedRowsV1(owned, undefined)).toHaveLength(3);
  });

  test("the reused pid is never signalled, the others are, the group of a tree with a stranger is not signalled as a group", async () => {
    const fresh = [row(10, 1, 10, 1000), row(11, 10, 10, 999_000), row(12, 11, 12, 3000), row(50, 1, 50, 4000)];
    const sent = await signalsOf(table, fresh, 10, 1000);
    expect(sent).not.toContain("11:SIGTERM");
    expect(sent).not.toContain("-10:SIGTERM");
    expect(sent).toContain("10:SIGTERM");
    expect(sent).toContain("12:SIGTERM");
    expect(sent.some((entry) => entry.startsWith("50:") || entry.startsWith("-50:"))).toBe(false);
  });

  test("a group made only of owned processes is signalled as a group, deepest group first, and a stranger in it forbids that", async () => {
    const owned = [row(10, 1, 10, 1000), row(11, 10, 10, 2000), row(12, 11, 12, 3000), row(13, 12, 12, 3500)];
    const sent = await signalsOf(owned, owned, 10, 1000);
    expect(sent.filter((entry) => entry.startsWith("-"))).toEqual(["-12:SIGTERM", "-10:SIGTERM"]);
    const stranger = [...owned, row(14, 1, 12, 9000)];
    const kept = await signalsOf(owned, stranger, 10, 1000);
    expect(kept).not.toContain("-12:SIGTERM");
    expect(kept).toContain("-10:SIGTERM");
    expect(kept).toContain("12:SIGTERM");
    expect(kept).not.toContain("14:SIGTERM");
  });

  test("a stop of a root of another creation time sends nothing", async () => {
    expect(await signalsOf(table, table, 10, 99_000)).toEqual([]);
  });

  test("every recorded signal names victim identity, sender identity, label and reason", () => {
    const lines: string[] = [];
    ownedKillRecorderV1("cargo build", () => "cancelled", (line) => lines.push(line))({ pid: 4711, startMs: 1_700_000_000_000 }, "SIGTERM");
    expect(lines).toHaveLength(1);
    expect(lines[0]).toContain("Process 4711 received SIGTERM from owned execution cargo build");
    expect(lines[0]).toContain(`(killing process ${process.pid}`);
    expect(lines[0]).toContain("Victim created at 1700000000000, sender created at ");
    expect(lines[0]).toContain("Opfer erzeugt um 1700000000000, Absender erzeugt um ");
    expect(lines[0]).toContain("cancelled");
  });

  test("the live process table of this host reads as `ps` and `pgrep` see it: this process with a creation time and its own group", () => {
    const rows = readProcessTableV1();
    if (process.platform === "win32") { expect(rows).toBeUndefined(); return; }
    const mine = rows!.find((entry) => entry.pid === process.pid)!;
    expect(mine.startMs).toBeGreaterThan(1_700_000_000_000);
    expect(mine.parent).toBe(process.ppid);
    const group = spawnSync("ps", ["-o", "pgid=", "-p", String(process.pid)], { encoding: "utf8" }).stdout.trim();
    expect(mine.group).toBe(Number(group));
    expect(ownerRoot.length).toBeGreaterThan(0);
    expect(join(ownerRoot, "x").length).toBeGreaterThan(0);
  });
});
