/** 🧱️ Test builders for the depiction cases: the reference walker evaluated, and a hand-built 1.80 m figure with one rectangle per shape role.
 *
 * @see ../🚶️reference-walker/🔣️.json
 */
import { readFileSync } from "node:fs";
import type { Asset, Shape, ShapeRole } from "../../🧬️schema/🟦️.ts";
import type { EvaluatedState } from "../../🔨️modules/🔗️contracts/🟦️.ts";
import { evaluate } from "../../🔨️modules/🧮️evaluation/🟦️.ts";

/** 🚶️ The reference walker asset. */
export const walkerAsset = (): Asset => JSON.parse(readFileSync(new URL("../🚶️reference-walker/🔣️.json", import.meta.url), "utf8")) as Asset;

/** 🧍️ The reference walker evaluated at a pose. */
export const walkerState = (pose = "stand"): EvaluatedState => evaluate(walkerAsset(), { kind: "pose", pose });

/** 📐️ A rectangle shape ring in world metres. */
const rect = (id: string, role: ShapeRole, lod: 0 | 1 | 2 | 3, x: number, h: number): { shape: Shape; rings: { x: number; y: number }[][]; closed: true } => ({
  shape: { id, bone: walkerAsset().rig.root, role, lod, z: 0, filled: true, tone: 0.5, kind: "rect", x, y: 0, w: 0.25, h },
  rings: [[{ x, y: 0 }, { x: x + 0.25, y: 0 }, { x: x + 0.25, y: h }, { x, y: h }]],
  closed: true,
});

/** 🧍️ The walker state with its shapes replaced by side-by-side rectangles: `figure` (body, 1.80 m), `detail` (lod 1), `texture` (lod 3, role texture), `fine-detail` (lod 3), `glass` (glazing). */
export function rolesState(): EvaluatedState {
  const base = walkerState();
  const shapes = [rect("figure", "body", 0, 0, 1.8), rect("detail", "detail", 1, 0.5, 0.5), rect("texture", "texture", 0, 1, 0.5), rect("fine-detail", "detail", 3, 1.5, 0.5), rect("glass", "glazing", 0, 2, 0.5)];
  return { ...base, shapes: { elevation: shapes, plan: [] }, bounds: { minX: 0, minY: 0, maxX: 2.25, maxY: 1.8 } };
}

/** 🧍️ A single 1.80 m figure and nothing else. */
export function figureState(): EvaluatedState {
  const base = rolesState();
  return { ...base, shapes: { elevation: base.shapes.elevation.slice(0, 1), plan: [] }, bounds: { minX: 0, minY: 0, maxX: 0.25, maxY: 1.8 } };
}
