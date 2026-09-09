// #region 🔬️ParserOracle
/** 🔬️ Holds `🧬️schema/🟦️.ts`'s hand-written `parse<Export>()` entry points against Ajv reading the
 * SAME `🧬️schema/🔣️.json`, over one language-agnostic vector file. The vectors are data, so a Rust or
 * Python leaf can be held against the same corpus; Ajv is the third-party oracle, so "our parser is
 * right" is a measured agreement rather than a belief.
 * @see 🧪️tests/🔬️parser-oracle/🔣️.json for the corpus and 📋️execution-contract.md §A for the rule. */
import { createRequire } from "node:module";
import { readFileSync } from "node:fs";
import { expect, test } from "bun:test";
import * as leaf from "../../🟦️.ts";

type Case = Readonly<{ export: string; id: string; accepted: boolean; instance: unknown }>;

const module_ = JSON.parse(readFileSync(new URL("../../🔣️.json", import.meta.url), "utf8")) as { $id: string; $defs: Record<string, unknown> };
const corpus = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔬️parser-oracle/🔣️.json", import.meta.url), "utf8")) as { version: number; scope: string; cases: readonly Case[] };
const parsers = leaf as unknown as Record<string, (value: unknown) => unknown>;

function vendorKeywords(node: unknown, found = new Set<string>()): Set<string> {
  if (Array.isArray(node)) for (const child of node) vendorKeywords(child, found);
  else if (node && typeof node === "object")
    for (const [key, child] of Object.entries(node)) {
      if (key.startsWith("x-semio-")) found.add(key);
      vendorKeywords(child, found);
    }
  return found;
}

const Ajv = createRequire(import.meta.url)("ajv");
const ajv = new Ajv({ strict: true, allErrors: true });
for (const keyword of vendorKeywords(module_)) ajv.addKeyword({ keyword, metaSchema: true });
ajv.addSchema(module_);

function ajvAccepts(exported: string, instance: unknown): boolean {
  const validate = ajv.getSchema(`${module_.$id}#/$defs/${exported}`);
  if (!validate) throw new Error(`${module_.$id} declares no $defs/${exported}`);
  return validate(instance) === true;
}

function parserAccepts(exported: string, instance: unknown): boolean {
  const parse = parsers[`parse${exported}`];
  if (typeof parse !== "function") throw new Error(`🧬️schema/🟦️.ts exports no parse${exported}()`);
  try {
    parse(instance);
    return true;
  } catch {
    return false;
  }
}

test("🕹️ every $defs export has a same-named TypeScript type and parse entry point", () => {
  const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
  for (const exported of Object.keys(module_.$defs)) {
    expect([exported, new RegExp(`^\\s*export\\s+type\\s+${exported}\\b`, "mu").test(source)]).toEqual([exported, true]);
    expect([exported, typeof parsers[`parse${exported}`]]).toEqual([exported, "function"]);
  }
});

test("🔬️ our parsers accept exactly what Ajv accepts, case by case", () => {
  expect(corpus.cases.length).toBeGreaterThan(0);
  const disagreements = corpus.cases.filter((entry) => ajvAccepts(entry.export, entry.instance) !== parserAccepts(entry.export, entry.instance)).map((entry) => `${entry.export}/${entry.id}`);
  expect(disagreements).toEqual([]);
});

test("🧾️ every case's declared verdict is the one both implementations reach", () => {
  const wrong = corpus.cases.filter((entry) => ajvAccepts(entry.export, entry.instance) !== entry.accepted).map((entry) => `${entry.export}/${entry.id}`);
  expect(wrong).toEqual([]);
});

test("📚️ the corpus covers every named export in both directions", () => {
  const missing = Object.keys(module_.$defs).filter((exported) => !corpus.cases.some((entry) => entry.export === exported && entry.accepted) || !corpus.cases.some((entry) => entry.export === exported && !entry.accepted));
  expect(missing).toEqual([]);
});
// #endregion 🔬️ParserOracle
