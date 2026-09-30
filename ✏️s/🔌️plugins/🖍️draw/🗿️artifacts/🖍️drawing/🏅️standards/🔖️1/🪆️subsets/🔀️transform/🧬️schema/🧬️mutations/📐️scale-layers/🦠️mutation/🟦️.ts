/** 📐️ Drawing mutation — `ScaleLayers` payload mirror: a relative scaling of several layers about one world-space pivot. */
import { drawingDrawingArtifactGuardArray, drawingDrawingArtifactGuardConstant, drawingDrawingArtifactGuardNumber, drawingDrawingArtifactGuardObject, drawingDrawingArtifactGuardString } from "../../../../../✳️any/🧬️schema/🟦️.ts";

/** 🚫️ Refuses a malformed payload with the JSON path of the offending value. */
const reject = (at: string, why: string): never => {
  throw new Error(`${at}: ${why}`);
};

export interface ScaleLayers {
  targets: string[];
  pivotX: number;
  pivotY: number;
  scaleX: number;
  scaleY: number;
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

/** 📐️ A nonzero finite factor — zero would collapse the layers onto the pivot. */
function parseFactor(value: unknown, at: string): number {
  const factor = drawingDrawingArtifactGuardNumber(value, at);
  return factor === 0 ? reject(at, "a scale factor is nonzero") : factor;
}

/** 📐️ Parses a tagged `scaleLayers` record exactly as the leaf schema admits it. */
export function parseScaleLayers(value: unknown, at = "$"): ScaleLayers {
  const row = drawingDrawingArtifactGuardObject(value, at);
  closed(row, at, ["mutation", "targets", "pivotX", "pivotY", "scaleX", "scaleY"]);
  drawingDrawingArtifactGuardConstant(row.mutation, `${at}.mutation`, "scaleLayers");
  return {
    targets: parseTargets(row.targets, `${at}.targets`),
    pivotX: drawingDrawingArtifactGuardNumber(row.pivotX, `${at}.pivotX`),
    pivotY: drawingDrawingArtifactGuardNumber(row.pivotY, `${at}.pivotY`),
    scaleX: parseFactor(row.scaleX, `${at}.scaleX`),
    scaleY: parseFactor(row.scaleY, `${at}.scaleY`),
  };
}
