/** 🖊️ Stroke widths per role come from the pens of the representation authored for the scale (ISO 128 series); the texture pen is `null` at 1:500, so texture is not drawn there.
 * @see ./🥒️.feature */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { defaultDepictionOptions, depict } from "../../🔨️modules/🎨️depiction/🟦️.ts";
import { rolesState } from "../../🧫️fixtures/🧱️synthetic-figure/🟦️.ts";

const table = (JSON.parse(readFileSync(new URL("../../🧫️fixtures/🖊️line-hierarchy/🔣️.json", import.meta.url), "utf8")) as { scales: Record<string, Record<string, number | null>> }).scales;

describe("line hierarchy", () => {
  for (const [denominator, weights] of Object.entries(table)) {
    it(`uses the pen table at 1:${denominator}`, () => {
      const scene = depict(rolesState(Number(denominator)), defaultDepictionOptions("light-line", "en", Number(denominator)));
      for (const role of ["body", "detail", "texture", "glazing"] as const) {
        const layer = scene.layers.find((l) => l.id === role);
        const widths = (layer?.items ?? []).flatMap((i) => (i.kind === "path" && i.id.startsWith("outline:") && i.stroke ? [i.stroke.widthMm] : []));
        if (weights[role] === null) expect(widths).toEqual([]);
        else if (role === "body" || role === "texture" || role === "glazing" || Number(denominator) <= 200) expect(new Set(widths)).toEqual(new Set([weights[role]]));
      }
    });
  }
  for (const [denominator, weights] of Object.entries(table))
    it(`draws the ground line at ${weights.ground} mm at 1:${denominator}`, () => {
      const ground = depict(rolesState(Number(denominator)), defaultDepictionOptions("light-line", "en", Number(denominator))).layers.find((l) => l.id === "ground")!.items[0]!;
      expect(ground.kind === "path" && ground.stroke?.widthMm).toBe(weights.ground);
    });
});
