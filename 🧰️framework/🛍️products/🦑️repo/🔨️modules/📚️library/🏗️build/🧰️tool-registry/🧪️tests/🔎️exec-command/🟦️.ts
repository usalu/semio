import Ajv from "ajv";
import { Database } from "bun:sqlite";
import signalCases from "./🔣️signals.json" with { type: "json" };
import signalSchema from "./🧬️schema/🔣️signals.json" with { type: "json" };
import { describe, expect, test } from "bun:test";
import { ToolErrorV1, toolForCommandV1 } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { runToolsCommandV1, toolCommandExitStatusV1 } from "../../⌨️command/🟦️.ts";

const context = (lines: string[]) => ({ environment: { ...process.env, SEMIO_LANGUAGE: "en" }, write: (line: string) => { lines.push(line); } });

describe("repo tools exec accepts an executable a registered tool owns", () => {
  test("the registry maps an owned executable to its tool", () => {
    expect(toolForCommandV1("rustc")?.id).toBe("cargo");
    expect(toolForCommandV1("bunx")?.id).toBe("bun");
    expect(toolForCommandV1("python3")?.id).toBe("python");
    expect(toolForCommandV1("not-a-tool")).toBeUndefined();
  });
  test("a tool id still runs", async () => {
    expect(await runToolsCommandV1(["exec", "--light", "node", "--", "--version"], context([]))).toBe(0);
  });
  test("an owned executable runs as that executable of its tool", async () => {
    expect(await runToolsCommandV1(["exec", "--light", "bunx", "--", "--version"], context([]))).toBe(0);
  });
  test("an unknown name is refused with the registered tools listed", async () => {
    const error = await runToolsCommandV1(["exec", "--light", "no-such-tool", "--"], context([])).catch((caught: unknown) => caught);
    expect(error).toBeInstanceOf(ToolErrorV1);
    expect(String((error as Error).message)).toContain("no-such-tool");
  });
});

type SignalCorpusV1 = Readonly<{ version: 1; cases: readonly Readonly<{ name: string; status: number | null; signal: string | null; signals: Readonly<Record<string, number>>; expected: number }>[] }>;

test("signal outcomes agree with the neutral corpus and independent SQLite oracle", () => {
  const validate = new Ajv({ strict: true }).compile<SignalCorpusV1>(signalSchema);
  const corpus: unknown = signalCases;
  if (!validate(corpus)) throw Error("Signal corpus schema required: " + JSON.stringify(validate.errors));
  expect(validate(corpus)).toBe(true);
  const database = new Database(":memory:");
  try {
    const query = database.query<{ code: number }, [number | null, string | null, number | null]>("SELECT COALESCE(?1, CASE WHEN ?2 IS NOT NULL AND ?3 IS NOT NULL THEN 128 + ?3 ELSE 1 END) AS code");
    for (const row of corpus.cases) {
      const signal = row.signal === null ? null : Object.entries(row.signals).find(([name]) => name === row.signal)?.[1] ?? null;
      const oracle = query.get(row.status, row.signal, signal);
      expect(oracle?.code).toBe(row.expected);
      expect(toolCommandExitStatusV1(row.status, row.signal, row.signals)).toBe(row.expected);
    }
  } finally { database.close(); }
});
