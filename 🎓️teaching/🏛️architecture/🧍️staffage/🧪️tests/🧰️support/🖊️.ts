/** 🖊️ The DWG oracle of the staffage tests: a written DWG is read back by ezdwg (`./🐍️.ts` `readDwg`, a DWG reader that shares no code with the writer) and compared with the ModelScene it was written from: version, units, layers with colour and lineweight, and every polyline and text on its layer.
 * @see ./🐍️oracles/📐️dwg.py */
import { writeFileSync } from "node:fs";
import { join } from "node:path";
import { expect } from "vitest";
import type { ModelScene } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { withScratch } from "./🧰️.ts";
import { readDwg, type DwgReport } from "./🐍️.ts";

/** 📏️ The standard lineweights in 1/100 mm; a DWG layer stores the nearest one. */
const LINEWEIGHTS = [0, 5, 9, 13, 15, 18, 20, 25, 30, 35, 40, 50, 53, 60, 70, 80, 90, 100, 106, 120, 140, 158, 200, 211] as const;

/** 🔢️ The standard lineweight nearest to `hundredthsMm` (the first one on a tie). */
export const nearestLineweight = (hundredthsMm: number): number => LINEWEIGHTS.reduce((best, weight) => (Math.abs(weight - hundredthsMm) < Math.abs(best - hundredthsMm) ? weight : best));

/** 📂️ Writes `bytes` to a scratch file and reads it with ezdwg. */
export const readDwgBytes = (bytes: Uint8Array): Promise<DwgReport> =>
  withScratch((directory) => {
    const file = join(directory, "drawing.dwg");
    writeFileSync(file, bytes);
    return readDwg(file);
  });

const key = (layer: string | null, parts: readonly (string | number | boolean)[]): string => `${layer}|${parts.join("|")}`;
const rounded = (value: number): number => Math.round(value * 1e9) / 1e9;

/** ✅️ Asserts that what ezdwg read is the scene: AC1024 in metres, every layer with its ACI colour and nearest lineweight, and the same polylines (closed flag and points to 1e-9 m) and texts (string, position, height) on the same layers; nothing else is in model space. */
export function expectSameDrawing(report: DwgReport, scene: ModelScene): void {
  expect(report.version).toBe("AC1024");
  expect(report.units).toBe("meters");
  expect(report.insunits).toBe(6);
  for (const layer of scene.layers) {
    expect(report.layers[layer.name], `layer ${layer.name}`).toBeDefined();
    expect(report.layers[layer.name]!.colorIndex, `${layer.name} colour`).toBe(layer.aci);
    expect(report.layers[layer.name]!.lineweight, `${layer.name} lineweight`).toBe(nearestLineweight(layer.lineweight));
  }
  const expected = scene.layers.flatMap((layer) => [
    ...layer.polylines.map((line) => key(layer.name, ["LWPOLYLINE", line.closed, ...line.points.flatMap((p) => [rounded(p.x), rounded(p.y)])])),
    ...layer.texts.map((text) => key(layer.name, ["TEXT", text.text, rounded(text.at.x), rounded(text.at.y), rounded(text.heightM)])),
  ]);
  const found = report.entities.map((entity) => (entity.type === "LWPOLYLINE" ? key(entity.layer, ["LWPOLYLINE", entity.closed!, ...entity.points!.flatMap(([x, y]) => [rounded(x), rounded(y)])]) : key(entity.layer, ["TEXT", entity.text!, rounded(entity.at![0]), rounded(entity.at![1]), rounded(entity.height!)])));
  expect(report.totalEntities).toBe(expected.length);
  expect(found.sort()).toEqual(expected.sort());
  for (const entity of report.entities) expect(entity.resolvedColorIndex, `${entity.layer} resolves through its layer`).toBe(report.layers[entity.layer!]!.colorIndex);
}
