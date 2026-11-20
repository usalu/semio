import { describe, expect, it } from "vitest";
import { chromeControlGroupClass, chromeControlGroupShellClass, chromeControlItemBaseClass, chromeControlItemClass } from "./🟦️.ts";

const tokens = (value: string) => value.split(/\s+/);
const heights = /^(h|min-h|max-h)-/;

describe("🧱️ control cells fill their owner", () => {
  it("a cell never imposes a height and stretches over the owner's content box", () => {
    for (const cell of [chromeControlItemBaseClass, chromeControlItemClass]) {
      expect(tokens(cell)).toContain("self-stretch");
      expect(tokens(cell).filter((token) => heights.test(token))).toEqual([]);
    }
  });

  it("only the owner states the height", () => {
    expect(tokens(chromeControlGroupShellClass)).toContain("items-stretch");
    expect(tokens(chromeControlGroupShellClass).filter((token) => heights.test(token))).toEqual([]);
    expect(tokens(chromeControlGroupClass)).toContain("h-medium");
  });

  it("a cell of a bordered owner ends inside the owner's border box", () => {
    const border = 1, medium = 24, content = medium - 2 * border;
    const cell = { top: border, height: content };
    expect(cell.top + cell.height).toBe(medium - border);
  });
});
