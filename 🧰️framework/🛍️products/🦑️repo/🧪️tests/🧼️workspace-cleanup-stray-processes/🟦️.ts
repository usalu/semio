import { expect, test } from "bun:test";
import { mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { countStrayDevToolZombies, dashboardDaemonPid, isDevLeftoverRow, listDevLeftoverRows, planStrayProcessRemovals, strayProcessExecutableName } from "../../🔨️modules/📚️library/🧼️workspace-cleanup/🧟️stray-processes/🟦️.ts";

const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🧼️workspace-cleanup-stray-processes/🔣️.json"), "utf8"));

test("dev leftover command markers catch cursor-agent workers", () => {
  expect(isDevLeftoverRow({
    name: "cursor-agent",
    stat: "S",
    command: "/bin/cursor-agent worker start --worker-dir /Users/dev/Documents/semio",
  })).toBe(true);
});

test("manually opened zsh terminal is never a dev leftover by name alone", () => {
  expect(isDevLeftoverRow({ name: "zsh", stat: "S", command: "-zsh" })).toBe(false);
  expect(isDevLeftoverRow({ name: "caffeinate", stat: "S", command: "caffeinate -dimsu" })).toBe(false);
});

test("stray process executable names match ps command basenames", () => {
  expect(strayProcessExecutableName("/usr/local/bin/bun run dev")).toBe("bun");
  expect(strayProcessExecutableName("<defunct>")).toBe("<defunct>");
  expect(strayProcessExecutableName("/path/to/esbuild --bundle")).toBe("esbuild");
});

test("stray process plan matches fixture oracle", () => {
  for (const row of vector.cases) {
    expect(planStrayProcessRemovals(row.rows, vector.selfPid, row.daemonPids ?? []), row.id).toEqual(row.expected);
  }
});

test("the recorded dashboard daemon pid is read from daemon.pid and ignored when absent or invalid", () => {
  const directory = mkdtempSync(join(tmpdir(), "stray-dashboard-"));
  try {
    expect(dashboardDaemonPid(directory)).toBeUndefined();
    writeFileSync(join(directory, "daemon.pid"), "24408\n");
    expect(dashboardDaemonPid(directory)).toBe(24408);
    writeFileSync(join(directory, "daemon.pid"), "not a pid\n");
    expect(dashboardDaemonPid(directory)).toBeUndefined();
  } finally { rmSync(directory, { recursive: true, force: true }); }
});

test("dashboard processes stay out of the leftover audit", () => {
  const ide = vector.cases.find((row: { id: string }) => row.id === "dashboard-daemon-and-task-tree-protected-by-recorded-pid");
  expect(listDevLeftoverRows(ide.rows, vector.selfPid, ide.daemonPids).map((row) => row.pid)).toEqual([1200]);
});

test("stray dev-tool zombie counter ignores IDE-hosted defunct rows", () => {
  const ideCase = vector.cases.find((row: { id: string }) => row.id === "defunct-under-ide-node-not-stray");
  expect(countStrayDevToolZombies(ideCase.rows)).toBe(0);
  const bunCase = vector.cases.find((row: { id: string }) => row.id === "defunct-zombie-under-bun-parent");
  expect(countStrayDevToolZombies(bunCase.rows)).toBe(1);
});
