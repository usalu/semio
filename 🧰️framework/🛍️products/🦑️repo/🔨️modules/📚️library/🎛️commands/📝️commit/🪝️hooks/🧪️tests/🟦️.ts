/** 🪝️ Tests for commit hooks. */
import {expect, test} from "bun:test";
import {reportHookShellV1, resolveHookShellV1} from "../🟦️.ts";
import fixture from "./🔣️.json" with {type: "json"};

/** 🐚️ Contract: a Windows host is told, in both languages, when the commit hook files cannot rely on a shell that every caller finds; other hosts need nothing because the shebang line is enough. */
test("every Windows host vector finds the shell the way the setup documents", () => {
  for (const row of fixture.cases) {
    const files = new Set(row.files.map((file) => file.toLowerCase()));
    expect(resolveHookShellV1(row.env, "win32", (path) => files.has(path.toLowerCase())), row.name).toEqual(row.expect as never);
  }
  for (const platform of ["linux", "darwin"]) expect(resolveHookShellV1({PATH: "/bin"}, platform, () => false)).toBeUndefined();
});

test("the report is silent for a shell on PATH and on POSIX hosts, an info line for the bundled shell and a warning when none exists", () => {
  const lines: Array<[string, Record<string, unknown>, string | undefined]> = [], say = ((key: string, values: Record<string, unknown>, level?: string) => { lines.push([key, values, level]); }) as never;
  reportHookShellV1(undefined, say);
  reportHookShellV1({state: "path", path: "C:\\x\\sh.exe"}, say);
  expect(lines).toEqual([]);
  reportHookShellV1({state: "bundled", path: "C:\\Git\\usr\\bin\\sh.exe"}, say);
  reportHookShellV1({state: "missing"}, say);
  expect(lines).toEqual([["hooks.shell-bundled", {path: "C:\\Git\\usr\\bin\\sh.exe"}, "info"], ["hooks.shell-missing", {}, "warn"]]);
});

test("both messages exist in English and German", async () => {
  const catalog = (await import("../../📖️texts/🔣️.json", {with: {type: "json"}})).default.messages as Record<string, {en: string; de: string}>;
  for (const key of ["hooks.shell-bundled", "hooks.shell-missing"]) { expect(catalog[key]!.en.length).toBeGreaterThan(20); expect(catalog[key]!.de.length).toBeGreaterThan(20); }
  expect(catalog["hooks.shell-bundled"]!.en).toContain("{path}");
  expect(catalog["hooks.shell-bundled"]!.de).toContain("{path}");
});
