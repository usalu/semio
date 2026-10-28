/** 🧪️ Tests for inputs toolchain-activation. */
import {expect, test} from "bun:test";
import {spawnSync} from "node:child_process";
import {join} from "node:path";
import Ajv from "ajv";
import {testPythonV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {toolchainsActivatedByV1} from "../🟨️.mjs";
import corpus from "./🔣️.json" with {type: "json"};
import schema from "./🧬️schema/🔣️.json" with {type: "json"};

/** 🪛️ The provisioning dependencies of a project activate their toolchains: the corpus is strict, the engine and an independent Python oracle agree on every case. */
test("the corpus satisfies its strict schema", () => {
  const ajv = new Ajv({strict: true, allErrors: true});
  expect(ajv.validate(schema, corpus), JSON.stringify(ajv.errors)).toBe(true);
  expect(new Set(corpus.cases.map((row) => row.id)).size).toBe(corpus.cases.length);
});

test("the activation equals the expectation and the Python oracle on every case", () => {
  const oracle = JSON.parse(spawnSync(testPythonV1() ?? "python3", ["-I", join(import.meta.dir, "🐍️oracle/🐍️.py"), join(import.meta.dir, "🔣️.json")], {encoding: "utf8"}).stdout);
  corpus.cases.forEach((row, index) => {
    expect(toolchainsActivatedByV1(row.targets as never, corpus.toolchains as never), row.id).toEqual(row.expect);
    expect(oracle[index], row.id).toEqual(row.expect);
  });
});
