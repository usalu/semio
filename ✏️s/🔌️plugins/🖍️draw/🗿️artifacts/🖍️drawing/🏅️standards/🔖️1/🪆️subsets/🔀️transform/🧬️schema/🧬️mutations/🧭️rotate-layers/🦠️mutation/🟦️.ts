/** 🧭️ Drawing mutation — `RotateLayers` payload mirror: a relative rotation of several layers about one world-space pivot. */
import { drawingDrawingArtifactGuardArray, drawingDrawingArtifactGuardConstant, drawingDrawingArtifactGuardNumber, drawingDrawingArtifactGuardObject, drawingDrawingArtifactGuardString } from "../../../../../✳️any/🧬️schema/🟦️.ts";

/** 🚫️ Refuses a malformed payload with the JSON path of the offending value. */
const reject = (at: string, why: string): never => {
  throw new Error(`${at}: ${why}`);
};

export interface RotateLayers {
  targets: string[];
  pivotX: number;
  pivotY: number;
  angle: number;
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

/** 🧭️ Parses a tagged `rotateLayers` record exactly as the leaf schema admits it. */
export function parseRotateLayers(value: unknown, at = "$"): RotateLayers {
  const row = drawingDrawingArtifactGuardObject(value, at);
  closed(row, at, ["mutation", "targets", "pivotX", "pivotY", "angle"]);
  drawingDrawingArtifactGuardConstant(row.mutation, `${at}.mutation`, "rotateLayers");
  return {
    targets: parseTargets(row.targets, `${at}.targets`),
    pivotX: drawingDrawingArtifactGuardNumber(row.pivotX, `${at}.pivotX`),
    pivotY: drawingDrawingArtifactGuardNumber(row.pivotY, `${at}.pivotY`),
    angle: drawingDrawingArtifactGuardNumber(row.angle, `${at}.angle`),
  };
}
