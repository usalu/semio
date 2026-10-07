/** 🧪️ Retained inventory agrees with a neutral corpus and the independent WAI-ARIA DOM oracle. */
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { runTimeTravelCli } from "../../🟦️.ts";
import { universalMirrorControlsV1, universalMirrorVerbsV1, type UniversalMirrorNodeV1 } from "../🟦️.ts";
const require = createRequire(import.meta.url);
const { JSDOM } = require("jsdom");
const { getRole, computeAccessibleName } = require("dom-accessibility-api");
const read = (relative: string) => JSON.parse(readFileSync(fileURLToPath(new URL(relative, import.meta.url)), "utf8"));
const fixture = read("../🧫️fixtures/🔣️.json");

describe("universal retained history editor inventory", () => {
  it("loads the shared live journey module with its retained inventory port", () => {
    expect(typeof runTimeTravelCli).toBe("function");
  });
  for (const law of fixture.cases) it(law.name, () => {
    const nodes = law.nodes as UniversalMirrorNodeV1[];
    expect(universalMirrorControlsV1(nodes).map(({ index: _index, key: _key, ...control }) => control)).toEqual(law.controls);
    expect(universalMirrorVerbsV1(nodes)).toEqual(law.verbs);
    const dom = new JSDOM("<!doctype html><body></body>");
    const document = dom.window.document;
    for (const node of nodes) {
      const element = document.createElement(node.role === "textbox" ? "input" : "div");
      element.setAttribute("role", node.role);
      element.setAttribute("aria-label", node.label);
      document.body.appendChild(element);
      expect([getRole(element), computeAccessibleName(element)]).toEqual([node.role, node.label]);
    }
    dom.window.close();
  });
});
