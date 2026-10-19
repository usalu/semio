import { test, expect } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { join, resolve } from "node:path";
import { mkdtempSync, mkdirSync, lstatSync, readFileSync, rmSync } from "node:fs";
import ts from "typescript";
import { readExactCargoExecutableV1, type ExactCargoTargetDataV1 } from "../../📋️protocol/🟦️.ts";
import { rustCompilerLoaderEnvironment } from "../../📚️loader/🟦️.ts";
import schema from "../../📋️protocol/🧬️schema/🔣️.json";
import fixture from "../../📋️protocol/🧪️tests/🧫️fixtures/📦️executable.json";
import policy from "../../📋️policy/🔣️.json";

const executable = resolve("owned-fixture-executable");
const capture = (kinds: readonly unknown[]) => ({ status: 0, signal: null, stderr: "", stdout: JSON.stringify({ reason: "compiler-artifact", package_id: "registry#fixture@1.0.0", profile: { test: true }, target: { name: "fixture", kind: kinds }, executable }) });
const target = (kind: string): ExactCargoTargetDataV1 => ({ kind, name: "fixture" } as ExactCargoTargetDataV1);

for (const row of fixture.cases) test(`exact Cargo executable descriptor retains actual metadata: ${row.id}`, () => {
  const validate = new Ajv({ strict: true }).compile(schema.definitions.Executable);
  if (!row.accepted) { expect(() => readExactCargoExecutableV1(capture(row.kinds), "fixture", target(row.target))).toThrow(); return; }
  const actual = readExactCargoExecutableV1(capture(row.kinds), "fixture", target(row.target));
  expect(actual).toEqual({ path: executable, kinds: row.kinds as string[] });
  expect(validate(actual)).toBe(true);
  const db = new Database(":memory:");
  try {
    const oracle = db.query("SELECT count(*) > 0 AS runtime FROM json_each(?, '$.target.kind') WHERE value='proc-macro'").get(capture(row.kinds).stdout) as { runtime: number };
    expect(actual.kinds.includes("proc-macro")).toBe(Boolean(oracle.runtime));
    expect(Boolean(oracle.runtime)).toBe(row.runtime);
  } finally { db.close(); }
  expect(Object.isFrozen(actual)).toBe(true);
  expect(Object.isFrozen(actual.kinds)).toBe(true);
});

test("exact Cargo runtime discovery consumes the retained descriptor in the actual runner branch", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Selected exact Cargo diagnostic output required");
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "metadata-")), directory = join(root, "runtime");
  mkdirSync(directory);
  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8"), tree = ts.createSourceFile("exact.ts", source, ts.ScriptTarget.Latest, true);
  let declaration = "", branch = "";
  const visit = (node: ts.Node): void => {
    if (ts.isVariableStatement(node) && node.getText(tree).includes("readExactCargoExecutableV1(")) declaration = node.getText(tree);
    if (ts.isIfStatement(node) && node.expression.getText(tree) === 'kinds.includes("proc-macro")') branch = node.getText(tree);
    ts.forEachChild(node, visit);
  };
  visit(tree);
  expect(declaration).not.toBe(""); expect(branch).not.toBe("");
  const code = ts.transpileModule(`async function selected(){${declaration}${branch};return nativeEnv}`, { compilerOptions: { target: ts.ScriptTarget.ES2022 } }).outputText;
  try {
    for (const row of fixture.cases.filter(row => row.accepted)) {
      const calls: string[] = [], bindings = { built: capture(row.kinds), group: { package: "fixture", target: target(row.target) }, protocol: (read: () => unknown) => read(), readExactCargoExecutableV1, env: { RUSTC: "owned-rustc" }, nativeEnv: {}, policy, process, lstatSync, rustCompilerLoaderEnvironment, fail: (message: string) => { throw Error(message); }, capture: async (stage: string, command: string, args: string[], budget: number) => { calls.push(command); expect(stage).toBe("runtime"); expect(budget).toBe(policy.listMilliseconds); expect(args).toEqual(["--print", "target-libdir"]); return { status: 0, signal: null, stderr: "", stdout: directory + "\n" }; } };
      const actual = await new Function(...Object.keys(bindings), code + ";return selected();")(...Object.values(bindings));
      expect(calls).toEqual(row.runtime ? ["owned-rustc"] : []);
      expect(actual).toEqual(row.runtime ? rustCompilerLoaderEnvironment(process.platform as "win32" | "linux" | "darwin", {}, directory) : {});
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
  console.log("[DEBUG] Cargo metadata kinds and actual procedural-macro loader branch agree with SQLite without compiler allocation");
});
