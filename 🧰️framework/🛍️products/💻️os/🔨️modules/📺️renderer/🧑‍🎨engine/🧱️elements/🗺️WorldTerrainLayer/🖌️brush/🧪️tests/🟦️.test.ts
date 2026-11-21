import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../../../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📐️edit-elevation/🧪️tests/🔣️.json" with { type: "json" };
import { MAX_TERRAIN_STROKE_SAMPLES, TerrainBrushStroke, type TerrainBrushSample } from "../👆️stroke.ts";

const coordinate = { type: "integer", minimum: 0, maximum: 32 };
const pathValidator = new Ajv().compile({
  type: "array", minItems: 1, maxItems: MAX_TERRAIN_STROKE_SAMPLES,
  items: { type: "array", minItems: 2, maxItems: 2, items: coordinate },
});

test("one pointer yields one bounded stroke or cancels it", () => {
  for (const row of fixture.strokeGestures) {
    const gesture = new TerrainBrushStroke();
    let releases = 0;
    const sample = (value: number[]) => value as unknown as TerrainBrushSample;
    expect(gesture.begin(7, row.radius, sample(row.down), () => ++releases)).toBe(true);
    expect(gesture.begin(8, row.radius, sample(row.down), () => ++releases)).toBe(false);
    gesture.move(8, [32, 32]);
    for (const move of row.moves) gesture.move(7, move ? sample(move) : undefined);
    if ("cancel" in row && row.cancel) gesture.cancel(7);
    const path = gesture.finish(7, sample(row.up));
    expect(path ?? null, row.id).toEqual(row.anchors);
    if (path) expect(pathValidator(path), row.id).toBe(true);
    expect(gesture.finish(7, sample(row.up))).toBeUndefined();
    expect(releases).toBe(1);
  }
});

test("capture loss discards only the original pointer session", () => {
  const gesture = new TerrainBrushStroke();
  const deferred: Array<() => void> = [];
  expect(gesture.begin(7, 2, [4, 4], () => undefined)).toBe(true);
  gesture.captureLost(7, cancel => deferred.push(cancel));
  expect(gesture.finish(7, [4, 4])).toEqual([[4, 4]]);
  expect(gesture.begin(7, 2, [6, 6], () => undefined)).toBe(true);
  deferred.shift()?.();
  expect(gesture.owns(7)).toBe(true);
  gesture.captureLost(7, cancel => deferred.push(cancel));
  deferred.shift()?.();
  expect(gesture.finish(7, [6, 6])).toBeUndefined();
});

test("independent brush arithmetic matches the neutral stroke elevations", () => {
  for (const row of fixture.strokeEdits) {
    const { operation, radius, value, stroke } = row.command;
    expect(pathValidator(stroke)).toBe(true);
    const coordinates = Array.from({ length: fixture.size }, (_, index) => index);
    const original = coordinates.map(() => coordinates.map(() => fixture.height));
    for (const [x, y, height] of fixture.seed) original[y]![x] = height;
    let current = original;
    const stamps = [stroke[0]!];
    for (let index = 1; index < stroke.length; index++) {
      const from = stroke[index - 1]!;
      const to = stroke[index]!;
      const delta = to.map((value, axis) => value - from[axis]!);
      const steps = Math.ceil(Math.max(...delta.map(Math.abs)) / Math.max(1, Math.floor((radius + 1) / 2)));
      for (let step = 1; step <= steps; step++) stamps.push(delta.map((value, axis) => Math.round(from[axis]! + value * step / steps)));
    }
    for (const [centerX, centerY] of stamps) {
      const previous = current;
      current = coordinates.map(y => coordinates.map(x => {
        const height = previous[y]![x]!;
        const normalized = Math.max(0, 1 - ((x - centerX!) ** 2 + (y - centerY!) ** 2) / (radius + 1) ** 2);
        let delta = value * normalized;
        if (operation === "smooth") {
          const neighbors = previous.slice(Math.max(0, y - 1), y + 2).flatMap(row => row.slice(Math.max(0, x - 1), x + 2));
          const average = neighbors.reduce((total, height) => total + height, 0) / neighbors.length;
          delta = (average - height) * normalized * value / 100;
        }
        return height + Math.sign(delta) * Math.round(Math.abs(delta));
      }));
    }
    const baseline = original.flat();
    const changed = current.flat().filter((height, index) => height !== baseline[index]).length;
    expect(changed).toBe(row.changed);
    for (const [x, y, height] of row.samples) expect(current[y]![x]).toBe(height);
  }
  for (const index of [2, 3, 4, 6]) expect(pathValidator(fixture.refusedStrokes[index]!.stroke)).toBe(false);
  const overflow = fixture.overflowStroke[0]!;
  expect(pathValidator(overflow.command.stroke)).toBe(true);
  expect(overflow.seed[2]! + overflow.command.value).toBeGreaterThan(32767);
});
