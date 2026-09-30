/** ✋️ Drawing mutation — `DragLayers` payload mirror: a relative drag of several layers by one world-space offset. */
import { drawingDrawingArtifactGuardArray, drawingDrawingArtifactGuardConstant, drawingDrawingArtifactGuardNumber, drawingDrawingArtifactGuardObject, drawingDrawingArtifactGuardString } from "../../../../../✳️any/🧬️schema/🟦️.ts";

/** 🚫️ Refuses a malformed payload with the JSON path of the offending value. */
const reject = (at: string, why: string): never => {
  throw new Error(`${at}: ${why}`);
};

export interface DragLayers {
  targets: string[];
  dx: number;
  dy: number;
}

/** 🎯️ The addressed layer ids: at least one, none repeated. */
function parseTargets(value: unknown, at: string): string[] {
  const targets = drawingDrawingArtifactGuardArray(value, at, { minItems: 1 }).map((item, index) => drawingDrawingArtifactGuardString(item, `${at}[${index}]`));
  if (new Set(targets).size !== targets.length) reject(at, "targets repeat an id");
  return targets;
}

/** 🚪️ Rejects every key outside the leaf's closed payload. */
function closed(row: Readonly<Record<string, unknown>>, at: string, keys: readonly string[]): void {
  for (const key of Object.keys(row)) if (!keys.includes(key)) reject(`${at}.${key}`, "unknown field");
}

/** ✋️ Parses a tagged `dragLayers` record exactly as the leaf schema admits it. */
export function parseDragLayers(value: unknown, at = "$"): DragLayers {
  const row = drawingDrawingArtifactGuardObject(value, at);
  closed(row, at, ["mutation", "targets", "dx", "dy"]);
  drawingDrawingArtifactGuardConstant(row.mutation, `${at}.mutation`, "dragLayers");
  return { targets: parseTargets(row.targets, `${at}.targets`), dx: drawingDrawingArtifactGuardNumber(row.dx, `${at}.dx`), dy: drawingDrawingArtifactGuardNumber(row.dy, `${at}.dy`) };
}
