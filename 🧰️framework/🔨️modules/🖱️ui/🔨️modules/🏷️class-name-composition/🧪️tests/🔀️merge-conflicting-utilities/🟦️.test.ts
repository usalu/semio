import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { pathToFileURL } from "node:url";
import { cn } from "../../🟦️.ts";

const feature = readFileSync(new URL("./🥒️.feature", import.meta.url), "utf8");
const rows = (source: string) => source.split("\n").filter(line => line.trim().startsWith("|")).map(line => line.split("|").slice(1, -1).map(cell => cell.trim())).filter(([input]) => input !== "input");
const oraclePath = process.env.CLASS_COMPOSITION_ORACLE;
const oracle = oraclePath ? (await import(pathToFileURL(oraclePath).href)).extendTailwindMerge({ extend: { classGroups: { "font-size": [{ text: ["tiny", "2xs"] }] } } }) as (input: string) => string : undefined;

describe("class composition feature contract", () => {
  for (const [input, expected] of rows(feature)) test(input!, () => expect(cn(input)).toBe(expected));
  if (oracle) for (const [input, expected] of rows(feature.split("Scenario: Font size and arbitrary text colors remain independent")[1]!.split("@id-modifiers-scope-the-conflict")[0]!)) test(`independent typography oracle: ${input}`, () => {
    expect(oracle(input!)).toBe(expected);
    expect(cn(input)).toBe(oracle(input!));
  });
});
