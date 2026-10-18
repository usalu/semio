import { describe, it, expect } from "bun:test";
import { resolveElementState, elementStateHidden, elementStateAttributes, resolveElementFillKind, STYLING_PRESENCE_PALETTES, STYLING_TOKENS } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { contrastRatio01, hexToRgb01, hslToRgb01, oklabDeltaE, oklabOfHsl } from "../🧮️color-math/🟦️.ts";

describe("elementState", () => {
  it("resolveElementState defaults every axis to inert", () => {
    expect(resolveElementState()).toEqual({ state: "normal", status: "idle", hover: false, selected: false });
    expect(resolveElementState({ selected: true })).toEqual({ state: "normal", status: "idle", hover: false, selected: true });
  });

  it("elementStateHidden is true only for state === hidden", () => {
    expect(elementStateHidden({ state: "hidden" })).toBe(true);
    for (const state of ["introducing", "celebrating", "previewed", "normal", "disabled"] as const) {
      expect(elementStateHidden({ state })).toBe(false);
    }
  });

  it("elementStateAttributes omits every axis at default", () => {
    expect(elementStateAttributes(resolveElementState())).toEqual({});
  });

  it("elementStateAttributes returns {} for hidden regardless of other axes", () => {
    expect(elementStateAttributes({ state: "hidden", status: "loading", hover: true, selected: true })).toEqual({});
  });

  it("elementStateAttributes stamps data-ui-state plus data-introduced for introducing", () => {
    expect(elementStateAttributes(resolveElementState({ state: "introducing" }))).toEqual({
      "data-ui-state": "introducing",
      "data-introduced": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state plus data-celebrated for celebrating", () => {
    expect(elementStateAttributes(resolveElementState({ state: "celebrating" }))).toEqual({
      "data-ui-state": "celebrating",
      "data-celebrated": "true",
    });
  });

  it("elementStateAttributes stamps data-ui-state for previewed/disabled without data-introduced", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed" }))).toEqual({ "data-ui-state": "previewed" });
    expect(elementStateAttributes(resolveElementState({ state: "disabled" }))).toEqual({ "data-ui-state": "disabled" });
  });

  it("elementStateAttributes stamps data-ui-status for non-idle status", () => {
    expect(elementStateAttributes(resolveElementState({ status: "loading" }))).toEqual({ "data-ui-status": "loading" });
    expect(elementStateAttributes(resolveElementState({ status: "waiting" }))).toEqual({ "data-ui-status": "waiting" });
    expect(elementStateAttributes(resolveElementState({ status: "finished" }))).toEqual({ "data-ui-status": "finished" });
  });

  it("elementStateAttributes stamps data-ui-hover and data-ui-selected independently", () => {
    expect(elementStateAttributes(resolveElementState({ hover: true }))).toEqual({ "data-ui-hover": "true" });
    expect(elementStateAttributes(resolveElementState({ selected: true }))).toEqual({ "data-ui-selected": "true" });
  });

  it("elementStateAttributes composes all four axes simultaneously", () => {
    expect(elementStateAttributes(resolveElementState({ state: "previewed", status: "waiting", hover: true, selected: true }))).toEqual({
      "data-ui-state": "previewed",
      "data-ui-status": "waiting",
      "data-ui-hover": "true",
      "data-ui-selected": "true",
    });
  });

  it("resolveElementFillKind follows disabled > celebrated > selected > previewed > hovered > neutral precedence", () => {
    expect(resolveElementFillKind(resolveElementState())).toBe("neutral");
    expect(resolveElementFillKind(resolveElementState({ hover: true }))).toBe("hovered");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed" }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ state: "previewed", hover: true }))).toBe("previewed");
    expect(resolveElementFillKind(resolveElementState({ selected: true, hover: true }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ selected: true, state: "previewed" }))).toBe("selected");
    expect(resolveElementFillKind(resolveElementState({ state: "celebrating", selected: true }))).toBe("celebrated");
    expect(resolveElementFillKind(resolveElementState({ state: "disabled", selected: true }))).toBe("disabled");
  });

  it("resolveElementFillKind returns null for hidden", () => {
    expect(resolveElementFillKind({ state: "hidden", status: "idle", hover: true, selected: true })).toBeNull();
  });
});

describe("presence palette", () => {
  it("carries 12 hues plus a light/dark s/l pair for both appearances", () => {
    expect(STYLING_PRESENCE_PALETTES.hues).toHaveLength(12);
    expect(STYLING_PRESENCE_PALETTES.light).toEqual({ s: expect.any(Number), l: expect.any(Number) });
    expect(STYLING_PRESENCE_PALETTES.dark).toEqual({ s: expect.any(Number), l: expect.any(Number) });
  });

  it("every base-cycle swatch clears 3:1 contrast against its appearance's base surface", () => {
    const bases = { light: hexToRgb01(STYLING_TOKENS.light), dark: hexToRgb01(STYLING_TOKENS.dark) } as const;
    for (const appearance of ["light", "dark"] as const) {
      const { s, l } = STYLING_PRESENCE_PALETTES[appearance];
      for (const h of STYLING_PRESENCE_PALETTES.hues) {
        const ratio = contrastRatio01(hslToRgb01(h, s, l), bases[appearance]);
        expect(ratio, `${appearance} h=${h} contrast`).toBeGreaterThanOrEqual(3);
      }
    }
  });

  it("consecutive wheel neighbors clear ΔE >= 0.12 in oklab, both appearances", () => {
    for (const appearance of ["light", "dark"] as const) {
      const { s, l } = STYLING_PRESENCE_PALETTES[appearance];
      const hues = STYLING_PRESENCE_PALETTES.hues;
      const oklabs = hues.map((h) => oklabOfHsl(h, s, l));
      for (let i = 0; i < oklabs.length; i++) {
        const de = oklabDeltaE(oklabs[i]!, oklabs[(i + 1) % oklabs.length]!);
        expect(de, `${appearance} neighbor ${hues[i]} vs ${hues[(i + 1) % hues.length]}`).toBeGreaterThanOrEqual(0.12);
      }
    }
  });
});
