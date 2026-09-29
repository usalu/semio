// #region 🔌️Adapters
import { hsl } from "d3-color";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { presenceColor, presenceCssVar, presencePaint, type PresenceAppearance } from "../../🟦️.ts";
// #endregion 🔌️Adapters

// #region 🎨️PresencePaint
const palette = readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../../../🎨️styling/🎨️palette/🎨️.css"), "utf8");

function declared(appearance: PresenceAppearance, slot: number): string {
  const block = appearance === "light" ? /:root \{([^}]*--presence-0:[^}]*)\}/u.exec(palette)?.[1] : /\.dark \{([^}]*--presence-0:[^}]*)\}/u.exec(palette)?.[1];
  const value = new RegExp(`--presence-${slot}: ([^;]+);`, "u").exec(block ?? "")?.[1];
  if (value === undefined) throw new Error(`--presence-${slot} is not declared for ${appearance}`);
  return value;
}

function rgb(css: string): string {
  const [, h, s, l] = /^hsl\(([\d.]+)deg ([\d.]+)% ([\d.]+)%\)$/u.exec(css) ?? [];
  return hsl(Number(h), Number(s) / 100, Number(l) / 100).formatHex();
}

/** 🎨️ The paint of a palette slot is one colour whichever path renders it: the generated `--presence-N` variable in
 * the first cycle, the inline literal past it — both equal {@link presenceColor}, converted by `d3-color`. */
describe("presencePaint", () => {
  for (const appearance of ["light", "dark"] as const) {
    it(`paints the first cycle through the palette variables, which hold presenceColor in ${appearance}`, () => {
      for (let slot = 0; slot < 12; slot += 1) {
        expect(presencePaint(slot, appearance)).toBe(presenceCssVar(slot));
        const { h, s, l } = presenceColor(slot, appearance);
        expect(rgb(declared(appearance, slot))).toBe(hsl(h, s, l).formatHex());
      }
    });

    it(`paints later cycles as presenceColor literals in ${appearance}`, () => {
      for (const slot of [12, 13, 23, 24, 30, 47, 255]) {
        const { h, s, l } = presenceColor(slot, appearance);
        expect(rgb(presencePaint(slot, appearance))).toBe(hsl(h, s, l).formatHex());
      }
    });
  }

  it("paints a session without a slot as slot 0", () => {
    expect(presencePaint(undefined, "light")).toBe(presenceCssVar(0));
  });
});
// #endregion 🎨️PresencePaint
