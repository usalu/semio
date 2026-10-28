import { describe, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { join } from "node:path";
import messages from "../../../🧬️schema/🗣️messages.json" with { type: "json" };
import { validateJsonSchemaSubset } from "../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import fixture from "./🔣️.json" with { type: "json" };
import { resolveRuntimeNativeLibcV1, runtimeNativeLibcCandidatesV1, runtimeNativeLibcV1 } from "../🟦️.ts";
const category = (error: unknown): string => {
  const text = String((error as Error).message);
  return /binding exists for the platform/u.test(text) ? "unsupported-platform" : /names are declared for the architecture/u.test(text) ? "unsupported-architecture" : /opened none of the C library names/u.test(text) ? "missing" : "other";
};
const resolveRow = (row: (typeof fixture.distributions)[number]): string => {
  try { return resolveRuntimeNativeLibcV1(row.platform, row.arch, name => row.loadable.includes(name)); } catch (error) { return category(error); }
};
describe("portable C library resolution", () => {
  test("the corpus is schema-valid and every distribution resolves as declared", () => {
    expect(validateJsonSchemaSubset(schema.definitions.Corpus, fixture, schema)).toEqual([]);
    for (const row of fixture.distributions) expect(resolveRow(row), row.id).toBe(row.expected);
  });
  test("an independent python implementation of the table agrees on every distribution", () => {
    const run = spawnSync("/usr/bin/python3", ["-I", join(import.meta.dir, "🐍️.py"), join(import.meta.dir, "🔣️.json"), join(import.meta.dir, "../🧬️schema/🔣️.json")], { encoding: "utf8" });
    expect(run.status, run.stderr).toBe(0);
    const model = JSON.parse(run.stdout) as Record<string, string>;
    for (const row of fixture.distributions) expect(resolveRow(row), row.id).toBe(model[row.id]!);
  });
  test("the first choice is the glibc soname and no name is a distribution path", () => {
    for (const arch of Object.keys(schema.definitions.Protocol.const.linux)) {
      const names = runtimeNativeLibcCandidatesV1("linux", arch);
      expect(names[0]).toBe("libc.so.6");
      for (const name of names) expect(name.includes("/")).toBe(false);
    }
  });
  test("refusals are bilingual and carry the tried names", () => {
    for (const id of ["nativeLibcPlatform", "nativeLibcArchitecture", "nativeLibcMissing"]) {
      const row = (messages as Record<string, { en: string; de: string }>)[id]!;
      expect(row.en.length).toBeGreaterThan(10);
      expect(row.de.length).toBeGreaterThan(10);
      expect([...row.en.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort()).toEqual([...row.de.matchAll(/\{(\w+)\}/gu)].map(m => m[1]).sort());
    }
    expect(() => resolveRuntimeNativeLibcV1("linux", "x64", () => false)).toThrow("libc.so.6, libc.musl-x86_64.so.1");
  });
  test.skipIf(process.platform !== "darwin" && process.platform !== "linux")("the real host resolves a name that an independent python ctypes load also opens", () => {
    const name = runtimeNativeLibcV1();
    expect(runtimeNativeLibcV1()).toBe(name);
    const run = spawnSync("/usr/bin/python3", ["-I", "-c", "import ctypes,sys;ctypes.CDLL(sys.argv[1]);print('ok')", name], { encoding: "utf8" });
    expect(run.stdout.trim(), run.stderr).toBe("ok");
  });
});
