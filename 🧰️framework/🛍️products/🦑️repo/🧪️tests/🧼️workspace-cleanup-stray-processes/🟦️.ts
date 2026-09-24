import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { countStrayDevToolZombies, isDevLeftoverRow, planStrayProcessRemovals, strayProcessExecutableName } from "../../🔨️modules/📚️library/🧼️workspace-cleanup/🧟️stray-processes/🟦️.ts";

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
    expect(planStrayProcessRemovals(row.rows, vector.selfPid)).toEqual(row.expected);
  }
});

test("stray dev-tool zombie counter ignores IDE-hosted defunct rows", () => {
  const ideCase = vector.cases.find((row: { id: string }) => row.id === "defunct-under-ide-node-not-stray");
  expect(countStrayDevToolZombies(ideCase.rows)).toBe(0);
  const bunCase = vector.cases.find((row: { id: string }) => row.id === "defunct-zombie-under-bun-parent");
  expect(countStrayDevToolZombies(bunCase.rows)).toBe(1);
});
