/** ⌨️ Keyboard focus on a bordered control of a shared-border frame is the seam recolour of `@layer shared-border-priority`, never a ring, inset shadow or outline beside the lines.
 *
 * @see ./focus-seam.cases.json */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import Ajv from "ajv/dist/2020.js";
import cases from "./focus-seam.cases.json";
import schema from "./focus-seam.schema.json";

describe("⌨️ focus recolours the seam", () => {
  it("neutral focus-seam schema", () => expect(new Ajv().validate(schema, cases)).toBe(true));
  it.each(cases.files)("$file", ({ file, mustContain, mustNotContain }) => {
    const source = readFileSync(resolve(import.meta.dirname, file), "utf8");
    for (const text of mustContain) expect(source).toContain(text);
    for (const text of mustNotContain) expect(source).not.toContain(text);
  });
});
