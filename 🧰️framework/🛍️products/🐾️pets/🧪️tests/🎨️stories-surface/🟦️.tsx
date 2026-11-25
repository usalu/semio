// @vitest-environment node
import { readFileSync } from "node:fs";
import { parse } from "postcss";
import { expect, it } from "vitest";
import vectors from "./🔣️.json";

it("keeps gallery seams single and paints the edge chips one level above their content", () => {
  const stylesheet = parse(readFileSync(new URL("../../🎯️targets/⚛️react/📖️stories/🎨️.css", import.meta.url), "utf8"));
  for (const { selector, property, value } of vectors) {
    const values: string[] = [];
    stylesheet.walkRules(selector, (rule) => rule.walkDecls(property, (declaration) => void values.push(declaration.value)));
    expect(values, `${selector} ${property}`).toContain(value);
  }
});
