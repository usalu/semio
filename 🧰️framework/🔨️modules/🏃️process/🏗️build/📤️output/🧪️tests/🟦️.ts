import { expect, test } from "bun:test";
import { spawn, spawnSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { testPythonV1 } from "../../../🧪️testing/🧰️toolchain/🟦️.ts";
import { machineOutputRequestedV1 } from "../🟦️.ts";
import protocol from "../🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };
import corpus from "./🔣️.json" with { type: "json" };
import corpusSchema from "./🧬️schema/🔣️.json" with { type: "json" };

type Child = (typeof corpus.child)[number];
type Run = Readonly<{ stdout: string; stderr: string; status: number | null }>;

const fixture = fileURLToPath(new URL("./🎭️fixture/🟦️.ts", import.meta.url)), oracleFile = fileURLToPath(new URL("./🐍️oracle.py", import.meta.url));
const lines = (text: string): string[] => text.split("\n").filter((line) => line !== "");
const run = (row: Child): Promise<Run> => new Promise((accept, reject) => {
  const child = spawn(process.execPath, [fixture, row.mode, ...row.args], { env: { ...process.env, ...row.environment }, stdio: ["ignore", "pipe", "pipe"] });
  let stdout = "", stderr = "";
  child.stdout.setEncoding("utf8").on("data", (text: string) => { stdout += text; });
  child.stderr.setEncoding("utf8").on("data", (text: string) => { stderr += text; });
  child.once("error", reject);
  child.once("close", (status) => accept({ stdout, stderr, status }));
});
const oracle = (requested: readonly (readonly string[])[], documents: readonly string[]): Readonly<{ requested: boolean[]; documents: boolean[] }> => {
  const answer = spawnSync(testPythonV1() ?? "python3", ["-I", oracleFile], { encoding: "utf8", input: JSON.stringify({ requested, documents }), maxBuffer: 1 << 24 });
  if (answer.status !== 0) throw new Error(`oracle failed: ${answer.stderr}`);
  return JSON.parse(answer.stdout);
};

test("the contract and the corpus satisfy the strict independent validator", () => {
  const ajv = new Ajv({ strict: true, allErrors: true });
  expect(ajv.validate(schema, protocol), JSON.stringify(ajv.errors)).toBe(true);
  expect(ajv.validate(corpusSchema, corpus), JSON.stringify(ajv.errors)).toBe(true);
});

test("which argument lists ask for machine output agrees with the independent oracle", () => {
  const answer = oracle(corpus.requested.map((row) => row.args), []);
  corpus.requested.forEach((row, index) => {
    expect(machineOutputRequestedV1(row.args), row.name).toBe(row.expected);
    expect(answer.requested[index], row.name).toBe(row.expected);
  });
});

for (const row of corpus.child) {
  test(`child process: ${row.name}`, async () => {
    const result = await run(row);
    expect(result.status, result.stderr).toBe(0);
    expect(lines(result.stdout)).toEqual(row.stdout);
    expect(lines(result.stderr)).toEqual(row.stderr);
    const machine = machineOutputRequestedV1(row.args), resultLines = lines(result.stdout).filter((line) => line.startsWith("{"));
    if (machine) expect(oracle([], [result.stdout]).documents[0], "the whole result stream parses as JSON").toBe(true);
    else expect(oracle([], [resultLines.join("\n")]).documents[0]).toBe(true);
  });
}
