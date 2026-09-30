/** 📍️ Drawing mutation — `DragPathPoints` payload mirror: a relative drag of path anchors and handles by one world-space offset. */
import { drawingDrawingArtifactGuardArray, drawingDrawingArtifactGuardConstant, drawingDrawingArtifactGuardInteger, drawingDrawingArtifactGuardMember, drawingDrawingArtifactGuardNumber, drawingDrawingArtifactGuardObject, drawingDrawingArtifactGuardString } from "../../../../../✳️any/🧬️schema/🟦️.ts";

/** 🚫️ Refuses a malformed payload with the JSON path of the offending value. */
const reject = (at: string, why: string): never => {
  throw new Error(`${at}: ${why}`);
};

export const DRAWING_PATH_POINTS = ["anchor", "control1", "control2"] as const;
export type PathPoint = typeof DRAWING_PATH_POINTS[number];

export interface DrawingPathPointTarget {
  layerId: string;
  index: number;
  point: PathPoint;
}

export interface DragPathPoints {
  targets: DrawingPathPointTarget[];
  dx: number;
  dy: number;
}

/** 🎯️ Parses one dragged point: a path layer, a segment index and which of its points. */
export function parseDrawingPathPointTarget(value: unknown, at = "$"): DrawingPathPointTarget {
  const row = drawingDrawingArtifactGuardObject(value, at);
  for (const key of Object.keys(row)) if (!["layerId", "index", "point"].includes(key)) reject(`${at}.${key}`, "unknown field");
  return {
    layerId: drawingDrawingArtifactGuardString(row.layerId, `${at}.layerId`),
    index: drawingDrawingArtifactGuardInteger(row.index, `${at}.index`, { minimum: 0 }),
    point: drawingDrawingArtifactGuardMember(row.point, `${at}.point`, DRAWING_PATH_POINTS),
  };
}

/** 📍️ Parses a tagged `dragPathPoints` record exactly as the leaf schema admits it. */
export function parseDragPathPoints(value: unknown, at = "$"): DragPathPoints {
  const row = drawingDrawingArtifactGuardObject(value, at);
  for (const key of Object.keys(row)) if (!["mutation", "targets", "dx", "dy"].includes(key)) reject(`${at}.${key}`, "unknown field");
  drawingDrawingArtifactGuardConstant(row.mutation, `${at}.mutation`, "dragPathPoints");
  const targets = drawingDrawingArtifactGuardArray(row.targets, `${at}.targets`, { minItems: 1 }).map((item, index) => parseDrawingPathPointTarget(item, `${at}.targets[${index}]`));
  if (new Set(targets.map((target) => `${target.layerId}\u0000${target.index}\u0000${target.point}`)).size !== targets.length) reject(`${at}.targets`, "targets repeat a point");
  return { targets, dx: drawingDrawingArtifactGuardNumber(row.dx, `${at}.dx`), dy: drawingDrawingArtifactGuardNumber(row.dy, `${at}.dy`) };
}
