/** 🪟 The presentation contract keeps no gap ring between frame and content: the body is edge to edge below the chip row and every band comes from the contract. */
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it } from "vitest";

const here = import.meta.dirname;
const presentation = resolve(here, "../../../");
const cases = JSON.parse(readFileSync(resolve(here, "🔣️.json"), "utf8"));
const contract = JSON.parse(readFileSync(resolve(presentation, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(presentation, "🧬️schema.json"), "utf8"));
const interaction = JSON.parse(readFileSync(resolve(presentation, "🖱️interaction/🔣️.json"), "utf8"));
const interactionSchema = JSON.parse(readFileSync(resolve(presentation, "🖱️interaction/🧬️schema.json"), "utf8"));
const pane = readFileSync(resolve(presentation, "🌐️browser/🟦️.tsx"), "utf8");

describe("presentation frame body", () => {
  it("validates the contract against its schema, which no longer names an inset", () => {
    const validate = new Ajv2020({ strict: false }).compile(schema);
    expect(validate(contract), JSON.stringify(validate.errors)).toBe(true);
    for (const key of cases.removedKeys) {
      expect(contract.chrome).not.toHaveProperty(key);
      expect(schema.properties.chrome.properties).not.toHaveProperty(key);
      expect(interaction.control).not.toHaveProperty(key);
      expect(interactionSchema.properties.control.properties).not.toHaveProperty(key);
    }
    const withInset = structuredClone(contract);
    withInset.chrome.inset = 3.2;
    expect(validate(withInset)).toBe(false);
  });

  it("derives the canvas band from the viewport and the bands, and the unobstructed viewport from the focus rect and the chip row", () => {
    const { viewport, chrome, canvas, focus } = contract;
    expect(canvas).toEqual({ x: 0, y: chrome.navbarHeight, width: viewport.width, height: viewport.height - chrome.navbarHeight - chrome.footerHeight });
    expect(focus.rect).toEqual(canvas);
    expect(focus.viewport).toEqual(focus.rect);
    expect(focus.safeClearance).toEqual({ top: chrome.titleHeight, right: 0, bottom: 0, left: 0 });
  });

  it("tiles the overview grid: the course grid rule has no gap and no padding", () => {
    const css = readFileSync(resolve(presentation, "../../../../../../🧰️framework/🛍️products/📚️course/🎯️targets/⚛️react/🎨️styles/🏠️chrome-and-home/🎨️.css"), "utf8");
    const rule = /\.course-home-grid \{([^}]*)\}/u.exec(css)?.[1] ?? "";
    expect(rule).not.toBe("");
    for (const property of cases.gridRuleForbidden) expect(rule).not.toMatch(new RegExp(`(^|[\\s;])${property}\\s*:`, "u"));
    expect(pane).not.toMatch(/--course-home-gap|columnGap|rowGap/u);
  });

  it("lays the body over the complete frame under the chips and reads the band from the contract", () => {
    const body = /data-presentation-body=\{entry\.id\} style=\{\{([^}]*)\}\}/u.exec(pane)?.[1] ?? "";
    for (const [key, value] of Object.entries(cases.bodyStyle)) expect(body).toContain(`${key}:${JSON.stringify(value)}`);
    expect(pane).toContain("config.contract.canvas");
    const band = /const band=\(\)=>\{[^\n]*\n/u.exec(pane)?.[0] ?? "";
    for (const pattern of cases.forbiddenSourcePatterns) expect(`${body}${band}`).not.toMatch(new RegExp(pattern, "u"));
  });
});
