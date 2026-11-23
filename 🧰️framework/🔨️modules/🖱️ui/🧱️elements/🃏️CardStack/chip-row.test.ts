/** 📏️ Title chips and footer chips of a card derive their height from one token, which is also the clearance document content keeps below and above them.
 *
 * @see ./chip-row.cases.json
 * @see ./bands.ts */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import cases from "./chip-row.cases.json";

const read = (file: string): string => readFileSync(resolve(import.meta.dirname, file), "utf8");

describe("📏️ one chip-row height", () => {
  it("declares the token once and its utility", () => {
    const css = read(cases.token.file);
    for (const text of cases.token.declares) expect(css.split(text).length - 1, text).toBe(1);
  });

  it.each(cases.equalHeight)("reads the token in $file", ({ file, mustContain, mustNotContain }) => {
    const source = read(file);
    for (const text of mustContain) expect(source).toContain(text);
    for (const text of mustNotContain) expect(source).not.toContain(text);
  });
});
